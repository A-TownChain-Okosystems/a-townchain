#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Prueft die Goldvektoren aus Abschnitt 18.2 des SCE-Kandidaten gegen SHA-256.

Das Werkzeug liest die Vektoren aus dem Dokument selbst und rechnet sie mit einem
reinen SHA-256 nach. Damit ist jeder Sollwert ohne Projektcode pruefbar, und eine
Aenderung der Bytes ohne Aenderung des Sollwerts faellt sofort auf.

Eine Zeile mit "wie GV-n" ist eine Bezugszeile: sie behauptet Gleichheit mit einem
frueheren Vektor und wird nicht gehasht, sondern verglichen.

Aufruf:
    python3 tools/check_stage1_vectors.py          # pruefen
    python3 tools/check_stage1_vectors.py -v       # jede Zeile ausgeben
"""

from __future__ import annotations

import argparse
import hashlib
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DOC = ROOT / "standards/atc/candidates/ATC-SMART-CONTRACT-EXECUTION-001-CANDIDATE.md"
VECTORS = ROOT / "standards/atc/conformance/stage1-vectors.json"

ROW = re.compile(r"^\|\s*(GV-\d+)\s*\|([^|]*)\|([^|]*)\|\s*`([0-9a-f]{64})`")
TOKEN = re.compile(r"`([0-9a-fA-F]+)`|(GV-\d+)|(P2-Strom)")
ALIAS = re.compile(r"wie\s+(GV-\d+)")
P1_STREAM = re.compile(r"\*\*Programm P1\*\*.*?Strom `([0-9a-f]+)`", re.S)
P2_STREAM = re.compile(r"\*\*Programm P2\*\*.*?Strom `([0-9a-f]+)`", re.S)


def parse_input(cell: str, digests: dict[str, str], p2: bytes) -> list[bytes] | str:
    """Bytes-Segmente in Dokumentreihenfolge, oder eine Bezugs-ID ('GV-n')."""
    alias = ALIAS.search(cell)
    if alias and not re.search(r"`[0-9a-fA-F]`", cell):
        return alias.group(1)
    segments: list[bytes] = []
    for m in TOKEN.finditer(cell):
        hexpart, ref, p2mark = m.group(1), m.group(2), m.group(3)
        if hexpart:
            if len(hexpart) % 2:
                raise SystemExit(f"ungerade Hexlaenge: {hexpart}")
            segments.append(bytes.fromhex(hexpart))
        elif ref:
            if ref not in digests:
                raise SystemExit(f"{ref} ist noch nicht aufgeloest")
            segments.append(bytes.fromhex(digests[ref]))
        else:
            segments.append(p2)
    return segments


def main() -> int:
    ap = argparse.ArgumentParser(description="Goldvektoren Stufe 1 pruefen")
    ap.add_argument("-v", "--verbose", action="store_true")
    ap.add_argument("--write", action="store_true", help="Vektordatei aus dem Dokument erzeugen")
    args = ap.parse_args()

    text = DOC.read_text(encoding="utf-8")
    sec = text.split("### 18.2 Goldvektoren")[1].split("### 18.3")[0]
    p1m, p2m = P1_STREAM.search(sec), P2_STREAM.search(sec)
    if not p1m or not p2m:
        print("FEHLER Stromdefinition P1/P2 nicht gefunden")
        return 1
    p2 = bytes.fromhex(p2m.group(1))

    digests: dict[str, str] = {}
    failures = 0
    count = 0
    for line in sec.splitlines():
        m = ROW.match(line)
        if not m:
            continue
        gid, _label, cell, expected = m.groups()
        parsed = parse_input(cell, digests, p2)
        count += 1
        if isinstance(parsed, str):
            ref = parsed
            if digests.get(ref) != expected:
                failures += 1
                print(f"FEHLER {gid}: behauptet Gleichheit mit {ref}, Sollwerte weichen ab")
            elif args.verbose:
                print(f"OK     {gid}: gleich {ref}")
        else:
            data = b"".join(parsed)
            actual = hashlib.sha256(data).hexdigest()
            if actual != expected:
                failures += 1
                print(f"FEHLER {gid}: erwartet {expected}, berechnet {actual} ({len(data)} Byte)")
            elif args.verbose:
                print(f"OK     {gid}: {actual} ({len(data)} Byte)")
        digests[gid] = expected

    neg = re.findall(r"^\|\s*(NV-\d)\s*\|[^|]*\|\s*`([0-9a-f]{64})`", sec, re.M)
    p1 = p1m.group(1)
    for nid, digest in neg:
        if nid == "NV-1":
            data = bytes.fromhex("4154432d41525449464143542d563100") + bytes.fromhex(p1)
        else:
            data = (
                bytes.fromhex("4154432d53545245414d2d563100")
                + bytes.fromhex("41544342020102000000")
                + bytes.fromhex(p1)
            )
        actual = hashlib.sha256(data).hexdigest()
        if actual != digest:
            failures += 1
            print(f"FEHLER {nid}: erwartet {digest}, berechnet {actual}")
        elif args.verbose:
            print(f"OK     {nid}: {actual}")

    for nid, digest in neg:
        if digest in (digests.get("GV-1"), digests.get("GV-2")):
            failures += 1
            print(f"FEHLER {nid}: Wert kollidiert mit einer regulaeren Identitaet")

    if failures:
        print(f"FEHLER {failures} Abweichung(en)")
        return 1

    if args.write:
        import json

        payload = {
            "quelle": str(DOC.relative_to(ROOT)),
            "abschnitt": "18.2",
            "standard": "ATC-SMART-CONTRACT-EXECUTION-001",
            "fassung": "1.6.0-draft",
            "hinweis": "Erzeugt aus dem Dokument; nicht von Hand aendern.",
            "stroeme": {"P1": p1, "P2": p2m.group(1)},
            "goldvektoren": [
                {"id": gid, "sollwert": dig}
                for gid, dig in sorted(digests.items())
            ],
            "negativvektoren": [
                {"id": nid, "sollwert": dig, "regel": "muss von GV-1 und GV-2 abweichen"}
                for nid, dig in neg
            ],
        }
        VECTORS.parent.mkdir(parents=True, exist_ok=True)
        VECTORS.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print(f"Vektordatei geschrieben: {VECTORS.relative_to(ROOT)}")

    print(f"PASS: {count} Goldvektoren und {len(neg)} Negativvektoren stimmen")
    return 0


if __name__ == "__main__":
    sys.exit(main())
