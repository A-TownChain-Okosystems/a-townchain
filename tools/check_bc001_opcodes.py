#!/usr/bin/env python3
"""Prueft die ATC-BC-001-Opcode-Tabelle gegen Spezifikation, SSOT und Implementierungen.

Eine Wahrheit je Opcode. Dieser Vergleich bindet
  * die normative Tabelle in ai/atclang/specs/bytecode/SPEC.md (Abschnitt 3),
  * die generierte SSOT-Datei standards/atc/conformance/bc001-opcodes.json,
  * den kanonischen Kodierer und Dekodierer in
    ai/atclang/crates/atc-core/src/{bytecode,artifact}.rs.

Die SSOT-Datei wird aus der Spezifikation erzeugt, nie von Hand gepflegt:

    python3 tools/check_bc001_opcodes.py --write     # Datei neu schreiben
    python3 tools/check_bc001_opcodes.py             # pruefen (Exit 1 bei Drift)

Zusaetzlich wacht das Skript ueber ATC-BC-002: die VM-Tabelle
(components/vm/src/artifact.rs) muss unveraendert bleiben. E2-C-1 aendert
keine Bytewerte, keine Zulaessigkeitsregel und kein Laufzeitverhalten.

Was hier NICHT passiert: keine Interpretation von Operandenbreiten, keine
Normalisierung von Namen. Bytewert, Name und Operandentext werden woertlich
aus der Spezifikation uebernommen. Unbekannte Operandenformen sind ein Fehler,
kein stilles Durchwinken.
"""

from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "ai/atclang/specs/bytecode/SPEC.md"
SSOT = ROOT / "standards/atc/conformance/bc001-opcodes.json"
ATC_CORE_BYTECODE = ROOT / "ai/atclang/crates/atc-core/src/bytecode.rs"
ATC_CORE_ARTIFACT = ROOT / "ai/atclang/crates/atc-core/src/artifact.rs"
VM_ARTIFACT = ROOT / "components/vm/src/artifact.rs"

SPEC_ID = "ATC-BC-001"
SPEC_SECTION = "## 3. Opcode-Katalog"
SCHEMA_VERSION = "1.0.0"
ERWARTETE_ANZAHL = 19

# Erlaubte Operandenformen. Neue Form heisst: Spezifikation geaendert, also
# bewusste Entscheidung mit Doku-Update, nicht stilles Durchlaufen.
ERLAUBTE_OPERANDEN = {
    "-",
    "i64",
    "i16",
    "u16",
    "function:u16, argc:u16",
}

# ATC-BC-002 ist in E2-C-1 ausdruecklich unveraendert zu halten. Diese Bindung
# ist der Waechter: sie schlaegt fehl, sobald jemand die VM-Tabelle anfasst.
BC002_SOLL = {
    0x01: "Push",
    0x02: "Add",
    0x03: "Sub",
    0x04: "Mul",
    0x05: "Div",
    0x06: "Dup",
    0x07: "Swap",
    0x08: "Eq",
    0x09: "Lt",
    0x0A: "Load",
    0x0B: "Store",
    0x0C: "Caller",
    0x0D: "Jump",
    0x0E: "JumpIfZero",
    0x0F: "JumpIfNotZero",
    0x10: "Halt",
    0x11: "Assert",
    0x12: "Revert",
    0x13: "RevertIf",
}


def _text(pfad: Path | None = None) -> str:
    return (pfad or SPEC).read_text(encoding="utf-8")


def spec_sha256(pfad: Path | None = None) -> str:
    return hashlib.sha256((pfad or SPEC).read_bytes()).hexdigest()


def spec_tabelle(pfad: Path | None = None) -> list[dict]:
    """Liest Abschnitt 3 (`| 0x01 | ConstI64 | i64 |`), in Spezifikationsreihenfolge."""
    text = _text(pfad)
    start = text.index(SPEC_SECTION)
    ende = text.index("\n## ", start + len(SPEC_SECTION))
    zeilen: list[dict] = []
    for zeile in text[start:ende].splitlines():
        treffer = re.match(
            r"^\|\s*`0x([0-9A-Fa-f]{2})`\s*\|\s*([A-Za-z][A-Za-z0-9]*)\s*\|\s*(.+?)\s*\|\s*$",
            zeile,
        )
        if treffer:
            zeilen.append(
                {
                    "opcode": "0x" + treffer.group(1).upper(),
                    "name": treffer.group(2),
                    "operand": treffer.group(3),
                }
            )
    return zeilen


def spec_metadaten(pfad: Path | None = None) -> dict:
    """Liest Kennung, Version, Container und Endianness aus der Spezifikation."""
    text = _text(pfad)
    kopf = text[text.index(SPEC_SECTION) :]
    anzahl = re.search(r"normativ,\s*(\d+)\s*Opcodes", kopf)
    endian = re.search(r"\*\*Endianness:\*\*[^\n]*?(big|little)-endian", text)
    magic = re.search(r"Magic\s*\|\s*4\s*\|\s*ASCII\s*`([A-Z]{4})`", text)
    version = re.search(r"Format-Version\s*\|\s*2\s*\|\s*u16 big-endian[^\n]*?`(\d+)`", text)
    return {
        "spec_id": SPEC_ID,
        "spec_version": (version.group(1) if version else ""),
        "endianness": (endian.group(1) if endian else ""),
        "container": (magic.group(1) if magic else ""),
        "declared_opcode_count": int(anzahl.group(1)) if anzahl else 0,
    }


def _int_opcode(opcode: str) -> int:
    return int(opcode, 16)


def _spec_pfad_text() -> str:
    try:
        return str(SPEC.relative_to(ROOT))
    except ValueError:
        return str(SPEC)


def render(zeilen: list[dict], meta: dict, sha: str) -> str:
    """Deterministische Serialisierung: feste Schluesselreihenfolge, 2 Leerzeichen, LF."""
    daten = {
        "schema_version": SCHEMA_VERSION,
        "source": {
            "standard": meta["spec_id"],
            "spec_path": _spec_pfad_text(),
            "spec_section": SPEC_SECTION.lstrip("# "),
            "spec_sha256": sha,
            "container": meta["container"],
            "format_version": meta["spec_version"],
            "endianness": meta["endianness"],
        },
        "opcode_count": len(zeilen),
        "opcodes": [
            {"opcode": z["opcode"], "name": z["name"], "operand": z["operand"]} for z in zeilen
        ],
    }
    return json.dumps(daten, indent=2, ensure_ascii=False, sort_keys=False) + "\n"


def erzeuge(pfad: Path | None = None) -> str:
    """Baut den SSOT-Inhalt allein aus der Spezifikation."""
    return render(spec_tabelle(pfad), spec_metadaten(pfad), spec_sha256(pfad))


def lade_ssot(pfad: Path | None = None) -> dict:
    return json.loads((pfad or SSOT).read_text(encoding="utf-8"))


def _funktionskoerper(text: str, marke: str, ende_marke: str) -> str:
    start = text.index(marke)
    return text[start : text.index(ende_marke, start)]


def rust_kodierer(pfad: Path | None = None) -> dict[str, int]:
    """Name -> Bytewert aus `encode_canonical` in atc-core/src/bytecode.rs."""
    text = (pfad or ATC_CORE_BYTECODE).read_text(encoding="utf-8")
    koerper = _funktionskoerper(text, "for ins in &self.instructions {", "Ok(out)")
    return {
        name: int(byte, 16)
        for name, byte in re.findall(
            r"Instruction::(\w+)[^\n]*?\n?\s*out\.push\(0x([0-9A-F]{2})\)", koerper
        )
    }


def rust_dekoder(pfad: Path | None = None) -> dict[int, str]:
    """Bytewert -> Name aus `decode_bytecode` in atc-core/src/artifact.rs."""
    text = (pfad or ATC_CORE_ARTIFACT).read_text(encoding="utf-8")
    koerper = _funktionskoerper(text, "fn decode_bytecode", "fn read_u16")
    return {
        int(byte, 16): name
        for byte, name in re.findall(r"0x([0-9A-F]{2}) => Instruction::(\w+)", koerper)
    }


def vm_bc002(pfad: Path | None = None) -> dict[int, str]:
    """Bytewert -> Op-Name aus `dekodiere_op` in components/vm/src/artifact.rs."""
    text = (pfad or VM_ARTIFACT).read_text(encoding="utf-8")
    koerper = _funktionskoerper(text, "pub(crate) fn dekodiere_op", "\n}")
    ergebnis: dict[int, str] = {}
    teile = re.split(r"\n\s+0x([0-9A-F]{2})\s*=>", koerper)
    # teile: [praefix, code1, koerper1, code2, koerper2, ...]
    for i in range(1, len(teile), 2):
        code = int(teile[i], 16)
        treffer = re.search(r"Op::(\w+)", teile[i + 1])
        if treffer:
            ergebnis[code] = treffer.group(1)
    return ergebnis


def pruefe() -> list[str]:
    fehler: list[str] = []
    zeilen = spec_tabelle()
    meta = spec_metadaten()
    sha = spec_sha256()

    # 1 — Spezifikation selbst: Vollstaendigkeit, Eindeutigkeit, Formen
    if len(zeilen) != ERWARTETE_ANZAHL:
        fehler.append(f"Spezifikation: {len(zeilen)} Opcodes statt {ERWARTETE_ANZAHL}")
    if meta["declared_opcode_count"] != len(zeilen):
        fehler.append(
            "Spezifikation: Abschnitt nennt "
            f"{meta['declared_opcode_count']} Opcodes, Tabelle hat {len(zeilen)}"
        )
    codes = [_int_opcode(z["opcode"]) for z in zeilen]
    if len(codes) != len(set(codes)):
        fehler.append("Spezifikation: doppelte Bytewerte")
    namen = [z["name"] for z in zeilen]
    if len(namen) != len(set(namen)):
        fehler.append("Spezifikation: doppelte Opcode-Namen")
    for z in zeilen:
        if z["operand"] not in ERLAUBTE_OPERANDEN:
            fehler.append(
                f"Spezifikation: unbekannte Operandenform {z['operand']!r} bei {z['name']}"
            )
    if meta["endianness"] != "big":
        fehler.append(f"Spezifikation: Endianness {meta['endianness']!r} statt 'big'")
    if meta["container"] != "ATCB":
        fehler.append(f"Spezifikation: Container {meta['container']!r} statt 'ATCB'")

    # 2 — SSOT-Datei: Drift gegen die Spezifikation, Determinismus
    soll = erzeuge()
    if soll != erzeuge():
        fehler.append("SSOT: Erzeugung ist nicht deterministisch")
    if not SSOT.exists():
        fehler.append(f"SSOT: {SSOT.relative_to(ROOT)} fehlt (--write ausfuehren)")
        return fehler
    ist = SSOT.read_text(encoding="utf-8")
    if ist != soll:
        fehler.append(
            "SSOT: Datei weicht von der Spezifikation ab "
            "(python3 tools/check_bc001_opcodes.py --write)"
        )
    try:
        daten = lade_ssot()
    except json.JSONDecodeError as ausnahme:
        fehler.append(f"SSOT: kein gueltiges JSON ({ausnahme})")
        return fehler

    # 3 — SSOT inhaltlich: keine zusaetzlichen, keine fehlenden Opcodes
    ssot_zeilen = daten.get("opcodes", [])
    if daten.get("opcode_count") != len(ssot_zeilen):
        fehler.append("SSOT: opcode_count passt nicht zur Liste")
    if len(ssot_zeilen) != ERWARTETE_ANZAHL:
        fehler.append(f"SSOT: {len(ssot_zeilen)} Opcodes statt {ERWARTETE_ANZAHL}")
    ssot_codes = [_int_opcode(z["opcode"]) for z in ssot_zeilen]
    if sorted(ssot_codes) != sorted(codes):
        fehler.append("SSOT: Bytewerte weichen von der Spezifikation ab")
    if sorted(z["name"] for z in ssot_zeilen) != sorted(namen):
        fehler.append("SSOT: Namen weichen von der Spezifikation ab")
    if [z["opcode"] for z in ssot_zeilen] != [z["opcode"] for z in zeilen]:
        fehler.append("SSOT: Reihenfolge weicht von der Spezifikation ab")
    if daten.get("source", {}).get("spec_sha256") != sha:
        fehler.append("SSOT: gebundene Spezifikations-SHA-256 ist veraltet")

    # 4 — Implementierung: kanonischer Kodierer und Dekodierer
    soll_kodierer = {z["name"]: _int_opcode(z["opcode"]) for z in zeilen}
    kodierer = rust_kodierer()
    if kodierer != soll_kodierer:
        fehlt = sorted(set(soll_kodierer) - set(kodierer))
        extra = sorted(set(kodierer) - set(soll_kodierer))
        falsch = sorted(
            n for n in set(kodierer) & set(soll_kodierer) if kodierer[n] != soll_kodierer[n]
        )
        fehler.append(
            "atc-core encode_canonical weicht ab: "
            f"fehlt={fehlt} extra={extra} falscher Bytewert={falsch}"
        )

    soll_dekoder = {_int_opcode(z["opcode"]): z["name"] for z in zeilen}
    dekoder = rust_dekoder()
    if dekoder != soll_dekoder:
        fehler.append("atc-core decode_bytecode weicht von der Spezifikation ab")

    # 5 — ATC-BC-002 bleibt unveraendert (E2-C-1-Grenze)
    bc002 = vm_bc002()
    if bc002 != BC002_SOLL:
        fehler.append(
            "ATC-BC-002: VM-Tabelle weicht vom Stand vor E2-C-1 ab "
            f"({len(bc002)} Eintraege, erwartet {len(BC002_SOLL)})"
        )

    return fehler


def main(argv: list[str]) -> int:
    if "--write" in argv:
        SSOT.parent.mkdir(parents=True, exist_ok=True)
        SSOT.write_text(erzeuge(), encoding="utf-8")
        print(f"geschrieben: {SSOT.relative_to(ROOT)}")
        return 0
    fehler = pruefe()
    if fehler:
        for eintrag in fehler:
            print(f"FEHLER: {eintrag}")
        return 1
    print(
        f"OK: {ERWARTETE_ANZAHL} Opcodes, SSOT und Implementierungen stimmen ueberein; "
        f"spec_sha256={spec_sha256()}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
