#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Erzeugt den Parser-Korpus C-1 bis C-8 aus Abschnitt 19.5 des SCE-Kandidaten.

Jeder Eintrag traegt seine Konstruktion im Feld `hinweis` und seinen Sollwert in
`entscheidung`/`code`/`offset`. Die Sollwerte werden **aus der Konstruktion**
abgeleitet, nicht vom Dekoder berechnet: sonst pruefte der Korpus sich selbst und
REQ-SCE-084 waere wertlos.

Die drei Container aus C-1 werden gegen die veroeffentlichten Sollwerte aus
`stage1-vectors.json` geprueft, damit der Korpus nicht unbemerkt von Abschnitt 18.2
abweicht.

Aufruf:
    python3 tools/build_parser_corpus.py           # Korpusdatei schreiben
    python3 tools/build_parser_corpus.py --check   # nur pruefen, Exit 1 bei Abweichung
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VECTORS = ROOT / "standards/atc/conformance/stage1-vectors.json"
CORPUS = ROOT / "standards/atc/conformance/stage1-parser-corpus.json"
DOC = ROOT / "standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md"

DOMAIN_ARTIFACT = b"ATC-ARTIFACT-V1\x00"
TAIL_LEN = 32
HEAD_LEN = 10

# Abschnitt 18.2: Kopf plus Strom, ohne Tail.
P1 = bytes.fromhex("01010000000000000010")  # Push(1), Halt
P2 = bytes.fromhex("010700000000000000012a00000000000000020b0000000010")  # Push, Push, Add, Store(0), Halt
GV1_BODY = bytes.fromhex("4154434502010200000001010000000000000010")  # ATCE, 0x02, Gen 1, 2 Ops
GV3_BODY = bytes.fromhex("4154434502020200000001010000000000000010")  # ATCE, 0x02, Gen 2, 2 Ops
GV7_BODY = bytes.fromhex("41544345020105000000") + P2  # ATCE, 0x02, Gen 1, 5 Ops
GV5_BODY = bytes.fromhex("4154434201010200000001010000000000000010")  # ATCB, 0x01, Gen 1


def digest(body: bytes) -> bytes:
    return hashlib.sha256(DOMAIN_ARTIFACT + body).digest()


def seal(body: bytes) -> bytes:
    """Container mit korrekt berechnetem Tail."""
    return body + digest(body)


def header(generation: int = 1, count: int = 0, magic: bytes = b"ATCE", version: int = 2) -> bytes:
    return magic + bytes([version, generation]) + count.to_bytes(4, "little")


def with_count(body: bytes, count: int) -> bytes:
    return body[:6] + count.to_bytes(4, "little") + body[10:]


def with_byte(body: bytes, offset: int, value: int) -> bytes:
    raw = bytearray(body)
    raw[offset] = value
    return bytes(raw)


def build() -> dict:
    published = json.loads(VECTORS.read_text(encoding="utf-8"))
    soll = {v["id"]: bytes.fromhex(v["sollwert"]) for v in published["goldvektoren"]}

    # Der Korpus darf nicht von Abschnitt 18.2 abweichen.
    for gid, body in (("GV-1", GV1_BODY), ("GV-3", GV3_BODY), ("GV-7", GV7_BODY)):
        if digest(body) != soll[gid]:
            raise SystemExit(
                f"{gid}: berechneter Digest weicht von stage1-vectors.json ab "
                f"({digest(body).hex()} != {soll[gid].hex()})"
            )
    # GV-5 ist die alte Fassung: der Digest ist reines SHA-256 ohne Domaenenpraefix.
    if hashlib.sha256(GV5_BODY).digest() != soll["GV-5"]:
        raise SystemExit("GV-5: berechneter Digest weicht ab")

    entries: list[dict] = []

    def add(
        cid: str,
        klasse: str,
        hinweis: str,
        data: bytes,
        decision: str,
        code: str | None = None,
        offset: int | None = None,
        instruktionen: int | None = None,
    ) -> None:
        entries.append(
            {
                "id": cid,
                "klasse": klasse,
                "hinweis": hinweis,
                "laenge": len(data),
                "eingabe_hex": data.hex(),
                "sha256": hashlib.sha256(data).hexdigest(),
                "entscheidung": decision,
                "code": code,
                "offset": offset,
                "instruktionen": instruktionen,
            }
        )

    # --- C-1: vollstaendige Container aus den Goldvektoren --------------------
    add(
        "C-1-GV1",
        "C-1",
        "GV-1: Kopf + P1 + Tail = Sollwert; zwei Instruktionen, Generation 1",
        GV1_BODY + soll["GV-1"],
        "accept",
        instruktionen=2,
    )
    add(
        "C-1-GV3",
        "C-1",
        "GV-3: derselbe Strom, Generation 2; der Digest ist ein anderer",
        GV3_BODY + soll["GV-3"],
        "accept",
        instruktionen=2,
    )
    add(
        "C-1-GV7",
        "C-1",
        "GV-7: P2 mit fuenf Instruktionen, Generation 1",
        GV7_BODY + soll["GV-7"],
        "accept",
        instruktionen=5,
    )

    # --- C-2: Magic und Generation falsch ------------------------------------
    add(
        "C-2-MAGIC",
        "C-2",
        "Magic ATCE -> ATCX an Offset 0; Tail bleibt stehen, Stufe 2 gewinnt",
        with_byte(GV1_BODY, 0, 0x58) + soll["GV-1"],
        "reject",
        "E-007",
        0,
    )
    add(
        "C-2-GEN00",
        "C-2",
        "Generation 0x00 an Offset 5; weder 0x01 noch 0x02",
        with_byte(GV1_BODY, 5, 0x00) + soll["GV-1"],
        "reject",
        "E-009",
        5,
    )
    add(
        "C-2-GEN03",
        "C-2",
        "Generation 0x03 an Offset 5",
        with_byte(GV1_BODY, 5, 0x03) + soll["GV-1"],
        "reject",
        "E-009",
        5,
    )

    # --- C-3: Altcontainer ATCB + 0x01 --------------------------------------
    add(
        "C-3-GV5",
        "C-3",
        "GV-5 als vollstaendiger Container: Magic ATCB, Fassung 0x01; Magic steht vor Version",
        GV5_BODY + soll["GV-5"],
        "reject",
        "E-007",
        0,
    )
    add(
        "C-3-MINIMAL",
        "C-3",
        "kleinster Altcontainer: ATCB + 0x01, Generation 1, ein Halt, Tail aus 32 Nullbytes",
        bytes.fromhex("41544342010101000000") + b"\x10" + bytes(32),
        "reject",
        "E-007",
        0,
    )

    # --- C-4: ATCE mit fremder Fassung --------------------------------------
    for idx, version in enumerate((0x01, 0x03, 0xFF)):
        add(
            f"C-4-V{version:02X}",
            "C-4",
            f"Magic ATCE, Fassung 0x{version:02X} an Offset 4; keine Umdeutung, kein Fallback",
            with_byte(GV1_BODY, 4, version) + soll["GV-1"],
            "reject",
            "E-003",
            4,
        )

    # --- C-5: Stromfehler ---------------------------------------------------
    # GV-1: Push(1) belegt Byte 10..18, Halt steht an 19. tail_start = L - 32.
    # k=1 schneidet nur das Halt weg: die Push endet bei 19, tail_start ist 19,
    #   also endet der Strom an einer Instruktionsgrenze -> E-005 bei 19.
    # k>=2 schneidet in den 8-Byte-Operanden: tail_start = 20-k, benoetigt wird
    #   Byte bis 18 -> E-002 bei 20-k.
    for k in range(1, 10):
        code, offset = ("E-005", 19) if k == 1 else ("E-002", 20 - k)
        note = (
            "Strom um 1 Byte gekuerzt: nur Halt fehlt, Grenze ist sauber -> E-005 bei 19"
            if k == 1
            else f"Strom um {k} Byte gekuerzt: Operand der Push reicht nicht bis {20 - k} -> E-002"
        )
        add(f"C-5-GV1-K{k}", "C-5", note, GV1_BODY[: 20 - k] + soll["GV-1"], "reject", code, offset)

    # GV-7: Push(7) 10..18, Push(42) 19..27, Add 28, Store(0) 29..33, Halt 34;
    # tail_start = 35 bei unveraenderter Laenge.
    add(
        "C-5-GV7-K1",
        "C-5",
        "GV-7 um 1 Byte gekuerzt: Halt fehlt, Store endet bei 34, Grenze sauber -> E-005 bei 34",
        GV7_BODY[:34] + soll["GV-7"],
        "reject",
        "E-005",
        34,
    )
    add(
        "C-5-GV7-K2",
        "C-5",
        "GV-7 um 2 Byte gekuerzt: Store braucht Bytes bis 33, tail_start ist 33 -> E-002 bei 33",
        GV7_BODY[:33] + soll["GV-7"],
        "reject",
        "E-002",
        33,
    )
    add(
        "C-5-GV7-K8",
        "C-5",
        "GV-7 um 8 Byte gekuerzt: die zweite Push braucht Byte bis 27, tail_start ist 27 -> E-002",
        GV7_BODY[:27] + soll["GV-7"],
        "reject",
        "E-002",
        27,
    )
    add(
        "C-5-OPCODE",
        "C-5",
        "Halt an 19 durch 0x14 ersetzt; 0x14 existiert in keiner Generation -> E-010 bei 19",
        with_byte(GV1_BODY, 19, 0x14) + soll["GV-1"],
        "reject",
        "E-010",
        19,
    )
    add(
        "C-5-GENMISMATCH",
        "C-5",
        "Generation 1 mit Opcode 0x11 (Assert, ab Generation 2) an 10 -> E-011",
        seal(header(generation=1, count=1) + b"\x11"),
        "reject",
        "E-011",
        10,
    )
    add(
        "C-5-NV4-LEER",
        "C-5",
        "NV-4: Anzahl 0 und leerer Strom, korrekter Tail -> E-004 bei 10",
        seal(header(generation=1, count=0)),
        "reject",
        "E-004",
        10,
    )
    add(
        "C-5-NV5-OPERAND",
        "C-5",
        "NV-5: Strom 01 ohne 8-Byte-Operand, tail_start 11 -> E-002 bei 11",
        seal(header(generation=1, count=1) + b"\x01"),
        "reject",
        "E-002",
        11,
    )
    add(
        "C-5-OPCODE-NEU-VERSIEGELT",
        "C-5",
        "wie C-5-OPCODE, aber Tail neu berechnet: der Strukturcode bleibt frueher als E-006",
        seal(with_byte(GV1_BODY, 19, 0x14)),
        "reject",
        "E-010",
        19,
    )

    # --- C-6: eingefuegte Bytes zwischen Strom und Tail ---------------------
    for k in (1, 2, 3):
        add(
            f"C-6-K{k}",
            "C-6",
            f"{k} Byte zwischen Strom und Tail eingefuegt: tail_start = {20 + k}, "
            f"nach 2 Instruktionen stehen Bytes ab 20 -> E-008 bei 20",
            GV1_BODY + bytes([0xAA]) * k + soll["GV-1"],
            "reject",
            "E-008",
            20,
        )
    add(
        "C-6-K1-NEU-VERSIEGELT",
        "C-6",
        "wie C-6-K1, aber Tail ueber den verlaengerten Rumpf: E-008 bleibt vor E-006",
        seal(GV1_BODY + b"\xAA"),
        "reject",
        "E-008",
        20,
    )

    # --- C-7: Tail im Inhalt veraendert, Laenge unveraendert ----------------
    tail1 = soll["GV-1"]
    add(
        "C-7-ROT1",
        "C-7",
        "Tail um ein Byte rotiert, Laenge 52 unveraendert -> E-006 bei 20",
        GV1_BODY + tail1[1:] + tail1[:1],
        "reject",
        "E-006",
        20,
    )
    flipped = bytearray(tail1)
    flipped[0] ^= 0xFF
    add(
        "C-7-FLIP",
        "C-7",
        "erstes Tailbyte invertiert, Laenge unveraendert -> E-006 bei 20",
        GV1_BODY + bytes(flipped),
        "reject",
        "E-006",
        20,
    )

    # --- C-8: deklarierte Anzahl falsch ------------------------------------
    add(
        "C-8-GV1-ZU-KLEIN",
        "C-8",
        "Anzahl 2 -> 1: nach der Push endet der Strom bei 19, Tail beginnt bei 20 -> E-008 bei 19",
        with_count(GV1_BODY, 1) + soll["GV-1"],
        "reject",
        "E-008",
        19,
    )
    add(
        "C-8-GV7-ZU-KLEIN",
        "C-8",
        "Anzahl 5 -> 3: nach Push, Push, Add endet der Strom bei 29, Tail beginnt bei 35 -> E-008",
        with_count(GV7_BODY, 3) + soll["GV-7"],
        "reject",
        "E-008",
        29,
    )
    add(
        "C-8-GV1-ZU-GROSS",
        "C-8",
        "Anzahl 2 -> 3: der Strom ist bei 20 erschoepft, das ist eine Instruktionsgrenze -> E-005 bei 20",
        with_count(GV1_BODY, 3) + soll["GV-1"],
        "reject",
        "E-005",
        20,
    )
    add(
        "C-8-GV7-ZU-GROSS",
        "C-8",
        "Anzahl 5 -> 6: Strom erschoepft bei 35 an einer Instruktionsgrenze -> E-005 bei 35",
        with_count(GV7_BODY, 6) + soll["GV-7"],
        "reject",
        "E-005",
        35,
    )
    add(
        "C-8-GV1-MAX",
        "C-8",
        "Anzahl 0xFFFFFFFF: ein Schritt, kein E-012, Grenze bei 20 -> E-005 bei 20",
        with_count(GV1_BODY, 0xFFFFFFFF) + soll["GV-1"],
        "reject",
        "E-005",
        20,
    )
    add(
        "C-8-GV1-ZU-KLEIN-NEU-VERSIEGELT",
        "C-8",
        "wie C-8-GV1-ZU-KLEIN, aber Tail neu berechnet: E-008 bleibt vor E-006",
        seal(with_count(GV1_BODY, 1)),
        "reject",
        "E-008",
        19,
    )

    return {
        "standard": "ATC-SMART-CONTRACT-EXECUTION-001",
        "abschnitt": "19.5",
        "quelle": "standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md",
        "erzeugt_von": "tools/build_parser_corpus.py",
        "hinweis": (
            "Sollwerte aus der Konstruktion abgeleitet, nicht vom Dekoder berechnet. "
            "Ein Vergleich der Zeilen mit tools/atc_reference_decoder.py ist kein "
            "Konformitaetsnachweis (REQ-SCE-084 verlangt zwei unabhaengige Dekoder)."
        ),
        "klassen": {
            "C-1": "vollstaendige Container aus GV-1, GV-3, GV-7",
            "C-2": "Magic falsch, Generation falsch",
            "C-3": "Altcontainer ATCB + 0x01 (GV-5)",
            "C-4": "ATCE mit fremder Fassung",
            "C-5": "Stromfehler: Operand, Opcode, Generation, leerer Strom",
            "C-6": "eingefuegte Bytes zwischen Strom und Tail",
            "C-7": "Tail im Inhalt veraendert",
            "C-8": "deklarierte Anzahl zu klein und zu gross",
        },
        "anzahl": len(entries),
        "eintraege": entries,
    }


def serialise(corpus: dict) -> str:
    return json.dumps(corpus, indent=2, ensure_ascii=False, sort_keys=False) + "\n"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="nur pruefen")
    args = parser.parse_args(argv)

    corpus = build()
    text = serialise(corpus)
    per_class: dict[str, int] = {}
    for entry in corpus["eintraege"]:
        per_class[entry["klasse"]] = per_class.get(entry["klasse"], 0) + 1

    if args.check:
        if not CORPUS.exists():
            print(f"FAIL: {CORPUS.relative_to(ROOT)} fehlt")
            return 1
        if CORPUS.read_text(encoding="utf-8") != text:
            print("FAIL: Korpusdatei weicht von der Erzeugung ab (von Hand geaendert?)")
            return 1
        print(f"PASS: Korpus unveraendert, {corpus['anzahl']} Eingaben in 8 Klassen")
        return 0

    CORPUS.write_text(text, encoding="utf-8")
    print(f"Korpus geschrieben: {CORPUS.relative_to(ROOT)} ({corpus['anzahl']} Eingaben)")
    for klasse in sorted(per_class):
        print(f"  {klasse}: {per_class[klasse]}")
    if corpus["anzahl"] < 30:
        print("FAIL: weniger als 30 Eingaben (REQ-SCE-084)")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
