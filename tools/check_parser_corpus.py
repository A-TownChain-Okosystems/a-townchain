#!/usr/bin/env python3
# Copyright (c) 2026 Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0
"""Prueft den Parser-Korpus gegen einen Dekoder (Standard: der Referenzdekoder).

Drei Pruefungen, alle fail-closed:

1. Die Korpusdatei entspricht der Erzeugung (keine Handaenderung, kein Drift).
2. Jede Zeile traegt ihre Sollwerte und eine Konstruktion im Feld `hinweis`.
3. Der Dekoder liefert je Zeile `(decision, code, offset)` wie im Korpus.

Was dieses Werkzeug **nicht** zeigt: Konformitaet. REQ-SCE-084 verlangt zwei
unabhaengige Dekoder auf demselben Korpus. Hier laeuft genau einer; die VM ist heute
kein konformer zweiter Dekoder (REQ-SCE-088).

Aufruf:
    python3 tools/check_parser_corpus.py
    python3 tools/check_parser_corpus.py -v
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(Path(__file__).resolve().parent))

from atc_reference_decoder import decode_hex  # noqa: E402
from build_parser_corpus import CORPUS, build, serialise  # noqa: E402


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("-v", "--verbose", action="store_true")
    args = parser.parse_args(argv)

    problems: list[str] = []

    if not CORPUS.exists():
        print(f"FAIL: {CORPUS.relative_to(ROOT)} fehlt")
        return 1

    regenerated = serialise(build())
    if CORPUS.read_text(encoding="utf-8") != regenerated:
        problems.append("Korpusdatei weicht von der Erzeugung ab")

    corpus = json.loads(CORPUS.read_text(encoding="utf-8"))
    entries = corpus["eintraege"]
    if len(entries) < 30:
        problems.append(f"nur {len(entries)} Eingaben, REQ-SCE-084 verlangt mindestens 30")

    for entry in entries:
        if not entry.get("hinweis"):
            problems.append(f"{entry['id']}: Konstruktion fehlt")
        if entry["entscheidung"] not in ("accept", "reject"):
            problems.append(f"{entry['id']}: Entscheidung '{entry['entscheidung']}' ungueltig")
        if entry["entscheidung"] == "accept" and (entry["code"] or entry["offset"] is not None):
            problems.append(f"{entry['id']}: angenommene Zeile traegt einen Fehler")

    deviations = 0
    per_class: dict[str, int] = {}
    for entry in entries:
        per_class[entry["klasse"]] = per_class.get(entry["klasse"], 0) + 1
        result = decode_hex(entry["eingabe_hex"])
        expected = (entry["entscheidung"], entry["code"], entry["offset"])
        actual = (result.decision, result.code, result.offset)
        if expected != actual:
            deviations += 1
            problems.append(
                f"{entry['id']}: erwartet {expected}, erhalten {actual} "
                f"({entry['hinweis']})"
            )
        elif args.verbose:
            print(f"  {entry['id']:<28} {actual[0]:<6} {str(actual[1]):<7} {actual[2]}")

    for klasse in sorted(per_class):
        print(f"  {klasse}: {per_class[klasse]} Eingaben")
    print(f"  Summe: {len(entries)} Eingaben, {deviations} Abweichungen")
    print(
        "  Hinweis: ein Dekoder belegt keine Konformitaet; "
        "REQ-SCE-084 verlangt zwei unabhaengige Dekoder."
    )

    if problems:
        for line in problems:
            print(f"FAIL: {line}")
        return 1
    print("PASS: Korpus unveraendert, alle Zeilen vom Referenzdekoder bestaetigt")
    return 0


if __name__ == "__main__":
    sys.exit(main())
