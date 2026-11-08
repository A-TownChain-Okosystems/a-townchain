#!/usr/bin/env python3
"""S-19 Mutationstest-Suite: Header-Drift MUSS zuverlaessig erkannt werden.

Erzeugt SYNTHETISCHE Fixtures mit definiertem Zustand (v1.0.0, CANDIDATE)
und prueft, dass der Validator jeden Drift-Fall korrekt klassifiziert
(FAIL) und keine False Positives produziert (PASS).
Fixtures sind bewusst nicht von Live-Dateien abhaengig (Status der
Live-Dateien aendert sich mit Freigaben; siehe F-012).
CI-ungebunden lauffaehig: python3 tests/test_s19_mutation.py
Exit-Code 0 = alle Faelle bestanden.
"""

import os
import re
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
VALIDATOR = os.path.join(HERE, "..", "atc_std_validator.py")


def run_validator(text):
    with tempfile.NamedTemporaryFile("w", suffix=".md", delete=False, encoding="utf-8") as f:
        f.write(text)
        path = f.name
    try:
        out = subprocess.run(
            [sys.executable, VALIDATOR, path], capture_output=True, text=True
        ).stdout
        m = re.search(r"\[(PASS|FAIL|WARN)\] S-19: (.*)", out)
        return (m.group(1), m.group(2)) if m else ("?", "keine S-19-Ausgabe: " + out[:120])
    finally:
        os.unlink(path)


BASE = (
    '---\nstandard:\n  id: ATC-STD-BUG-001\n  version: "1.0.0"\n'
    "  status: candidate\n---\n\n"
    "# ATC-STD-BUG-001 — Bug Finding Standard (v1.0.0, CANDIDATE)\n\n"
    "> **Version:** 1.0.0\n> **Status:** CANDIDATE\n\n## Abstract\n\nTest.\n"
)

STD202_LIKE = (
    '---\nstandard:\n  id: ATC-STD-202\n  version: "1.1.0"\n'
    "  status: approved\n---\n\n"
    "# ATC-STD-202 — Naming Standard (v1.1.0, APPROVED)\n\n"
    "> **Status:** APPROVED (v1.1.0)\n\n## Abstract\n\nTest.\n"
)


def main():
    faelle = [
        (
            "M1: Titel-Version manipuliert",
            BASE.replace(
                "# ATC-STD-BUG-001 — Bug Finding Standard (v1.0.0, CANDIDATE)",
                "# ATC-STD-BUG-001 — Bug Finding Standard (v7.7.7, CANDIDATE)",
                1,
            ),
            "FAIL",
        ),
        (
            "M2: Titel-Status NORMATIV statt candidate",
            BASE.replace("(v1.0.0, CANDIDATE)", "(v1.0.0, NORMATIV)", 1),
            "FAIL",
        ),
        (
            "M3: Kopf-Statuszeile manipuliert",
            BASE.replace("> **Status:** CANDIDATE", "> **Status:** APPROVED", 1),
            "FAIL",
        ),
        (
            "M4: Frontmatter-Status abweichend",
            BASE.replace("status: candidate", "status: approved", 1),
            "FAIL",
        ),
        (
            "M5: Kopf-Statuszeile geloescht",
            re.sub(r"^> \*\*Status:\*\*.*\n", "", BASE, count=1, flags=re.M),
            "WARN",
        ),
        ("M6: Fixturedokument unveraendert", BASE, "PASS"),
        ("M7: Statuszeilen-Version konsistent", STD202_LIKE, "PASS"),
        (
            "M8: Statuszeilen-Version manipuliert",
            STD202_LIKE.replace("APPROVED (v1.1.0)", "APPROVED (v9.9.9)", 1),
            "FAIL",
        ),
        (
            "M9: Beispielblock tief im Dokument (kein False Positive)",
            BASE + "\n\n> **Version:** 0.0.1 (nur Beispiel, weit unterhalb des Kopfs)\n",
            "PASS",
        ),
        ("M10: klassisches Frontmatter, konsistenter Kopf", BASE, "PASS"),
        (
            "M11: klassisches Frontmatter, Kopf-Drift",
            BASE.replace("> **Version:** 1.0.0", "> **Version:** 2.0.0", 1),
            "FAIL",
        ),
        (
            "M12: klassisches Frontmatter, Titel-Status-Drift",
            BASE.replace("(v1.0.0, CANDIDATE)", "(v1.0.0, APPROVED)", 1),
            "FAIL",
        ),
    ]

    fehl = 0
    print(f"S-19 Mutationstest-Suite: {len(faelle)} Faelle (synthetische Fixtures)\n" + "=" * 70)
    for label, text, expect in faelle:
        got, msg = run_validator(text)
        ok = got == expect
        print(f"[{'OK     ' if ok else 'FEHLER '}] {label:46s} erwartet={expect:5s} ist={got}")
        if not ok:
            print(f"          -> {msg}")
            fehl += 1
    print("=" * 70)
    print(f"Ergebnis: {len(faelle) - fehl}/{len(faelle)} bestanden")
    sys.exit(1 if fehl else 0)


if __name__ == "__main__":
    main()
