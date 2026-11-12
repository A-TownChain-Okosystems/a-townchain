#!/usr/bin/env python3
"""Prueft die Admission-Codeliste gegen Spezifikation und Implementierung.

Eine Wahrheit je Code: dieser Vergleich bindet
  * die normative Tabelle in ATC-SEC-EXEC-001 (Markdown),
  * die SSOT-Datei standards/atc/conformance/admission-codes.json,
  * die Implementierung (AdmCode in components/exec-admission/src/lib.rs).

Aktive Codes muessen an allen drei Stellen uebereinstimmen (Code, Reihenfolge).
Vorgeschlagene Codes duerfen in der Spezifikation und der SSOT stehen, aber
NICHT in der Implementierung: das haelt die Sperre fuer ADM-012 sichtbar.

Aufruf: python3 tools/check_admission_codes.py
"""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = ROOT / "standards/atc/candidates/ATC-SEC-EXEC-001-CANDIDATE.md"
SSOT = ROOT / "standards/atc/conformance/admission-codes.json"
RUST = ROOT / "components/exec-admission/src/lib.rs"


def lade_ssot(pfad: Path | None = None) -> dict:
    """Liest die SSOT-Datei. Der Pfad wird bei jedem Aufruf aus dem Modul
    gelesen, damit Tests auf eine Kopie zeigen koennen."""
    return json.loads((pfad or SSOT).read_text(encoding="utf-8"))


def spec_tabelle(pfad: Path | None = None) -> list[tuple[int, str]]:
    """Liest die Pruefreihenfolge aus Abschnitt 4 (| # | Pruefung | Code |)."""
    text = (pfad or SPEC).read_text(encoding="utf-8")
    start = text.index("## 4. Prüfreihenfolge")
    ende = text.index("### 4a.")
    zeilen: list[tuple[int, str]] = []
    for zeile in text[start:ende].splitlines():
        treffer = re.match(r"^\|\s*(\d+)\s*\|[^|]*\|\s*`(ADM-\d{3})`\s*\|", zeile)
        if treffer:
            zeilen.append((int(treffer.group(1)), treffer.group(2)))
    return zeilen


def rust_code_map(pfad: Path | None = None) -> dict[str, str]:
    """Liest `AdmCode::Variante => "ADM-0xx"` aus der Implementierung."""
    text = (pfad or RUST).read_text(encoding="utf-8")
    return {
        variante: code
        for variante, code in re.findall(
            r"AdmCode::(\w+)\s*=>\s*\"(ADM-\d{3})\"", text
        )
    }


def rust_reihenfolge(pfad: Path | None = None) -> list[str]:
    """Liest die Schrittnummern aus `rank()` in Deklarationsreihenfolge."""
    text = (pfad or RUST).read_text(encoding="utf-8")
    block = text[text.index("AdmCode::AuthorityUnavailable => 1") :]
    block = block[: block.index("}")]
    return [code for code in re.findall(r"AdmCode::(\w+) =>", block)]


def pruefe() -> list[str]:
    fehler: list[str] = []
    ssot = lade_ssot()
    codes = ssot["codes"]

    aktiv = [c for c in codes if c["status"] == "active"]
    vorgeschlagen = [c for c in codes if c["status"] == "proposed"]

    # 1 — SSOT ist eindeutig
    alle = [c["code"] for c in codes]
    if len(alle) != len(set(alle)):
        fehler.append("SSOT: doppelte Codes")
    nummern = sorted(int(c["code"].split("-")[1]) for c in codes)
    if nummern != list(range(1, len(nummern) + 1)):
        fehler.append(f"SSOT: Nummern nicht lueckenlos fortlaufend: {nummern}")

    # 2 — Spezifikation und SSOT stimmen in der Reihenfolge ueberein
    tabelle = spec_tabelle()
    if len(tabelle) != len(aktiv):
        fehler.append(
            f"Spezifikation: {len(tabelle)} Pruefschritte, SSOT: {len(aktiv)} aktive Codes"
        )
    for (schritt, code), eintrag in zip(tabelle, sorted(aktiv, key=lambda c: c["order"])):
        if code != eintrag["code"] or schritt != eintrag["order"]:
            fehler.append(
                f"Reihenfolge: Spezifikation Schritt {schritt}={code}, "
                f"SSOT {eintrag['order']}={eintrag['code']}"
            )
    for code in [c["code"] for c in vorgeschlagen]:
        if code not in SPEC.read_text(encoding="utf-8"):
            fehler.append(f"Spezifikation: vorgeschlagener Code {code} nicht dokumentiert")

    # 3 — Implementierung deckt genau die aktiven Codes
    implementiert = rust_code_map()
    if set(implementiert.values()) != {c["code"] for c in aktiv}:
        fehlt = {c["code"] for c in aktiv} - set(implementiert.values())
        extra = set(implementiert.values()) - {c["code"] for c in aktiv}
        if fehlt:
            fehler.append(f"Implementierung: fehlende Codes {sorted(fehlt)}")
        if extra:
            fehler.append(f"Implementierung: nicht registrierte Codes {sorted(extra)}")
    for eintrag in aktiv:
        if implementiert.get(eintrag["rust_variant"]) != eintrag["code"]:
            fehler.append(
                f"Implementierung: {eintrag['rust_variant']} != {eintrag['code']}"
            )

    # 4 — Sperre: vorgeschlagene Codes sind NICHT implementiert
    for eintrag in vorgeschlagen:
        if eintrag["code"] in implementiert.values():
            fehler.append(
                f"Sperre verletzt: {eintrag['code']} ist implementiert, obwohl "
                f"status={eintrag['status']}"
            )
        if eintrag["rust_variant"]:
            fehler.append(
                f"SSOT: vorgeschlagener Code {eintrag['code']} nennt eine Rust-Variante"
            )

    # 5 — Rangfolge der Implementierung entspricht der Spezifikationsreihenfolge
    reihenfolge = rust_reihenfolge()
    erwartet = [c["rust_variant"] for c in sorted(aktiv, key=lambda c: c["order"])]
    if reihenfolge != erwartet:
        fehler.append(f"rank(): {reihenfolge} != Spezifikationsreihenfolge {erwartet}")

    return fehler


def main() -> int:
    fehler = pruefe()
    if fehler:
        for eintrag in fehler:
            print(f"FAIL: {eintrag}")
        return 1
    ssot = lade_ssot()
    aktiv = sum(1 for c in ssot["codes"] if c["status"] == "active")
    offen = sum(1 for c in ssot["codes"] if c["status"] == "proposed")
    print(
        f"PASS: {aktiv} aktive Codes in Spezifikation, SSOT und Implementierung deckungsgleich; "
        f"{offen} vorgeschlagen (nicht implementiert)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
