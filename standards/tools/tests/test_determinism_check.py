#!/usr/bin/env python3
"""Regressionstests W2-MECH-01 (SCR-0131) — Determinism-Checker v1.1.0.

Design nach Work-Order (Owner-Vorgabe 06.10.2026):

  Test A — Self-Exclusion: Fixture-Repo mit dem Checker selbst unter tools/
           plus einer echten Verletzung in src/.
           Erwartung: KEIN Finding aus tools/ (auch nicht vom Checker selbst),
           ABER das echte Finding aus src/ (File/Zeile/Pattern).
  Test B — String-Literale/Docstrings: Vorkommen nur in String-Literal,
           Docstring oder Kommentar. Erwartung: kein Finding.
  Test C — True Positive: echter Aufruf ausserhalb von String/Docstring.
           Erwartung: Finding mit Datei, Zeile und Pattern-Beschreibung.

Ohne Test C waere der Fix wertlos: 'alle roten Ergebnisse verschwunden'
ist ein Nachweis der Stille, nicht der Korrektheit (Fail-closed-Prinzip).

Lauf: python3 tools/tests/test_determinism_check.py  (exit 0 nur bei A+B+C gruen)
"""

import importlib.util
import os
import shutil
import tempfile
import unittest

CHECKER = os.path.realpath(
    os.path.join(os.path.dirname(os.path.realpath(__file__)), "..", "determinism_check.py")
)
_spec = importlib.util.spec_from_file_location("det_check", CHECKER)
det = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(det)


def scan(tmp, lang):
    return det.scan_sources(tmp, lang)


class TestA_SelfExclusion(unittest.TestCase):
    """Kriterium 2: eigener tools/-Baum wird nicht gescannt."""

    def test_checker_sub_tree_produziert_keine_findings(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.makedirs(os.path.join(tmp, "tools"))
            shutil.copy(CHECKER, os.path.join(tmp, "tools", "determinism_check.py"))
            os.makedirs(os.path.join(tmp, "src"))
            with open(os.path.join(tmp, "src", "lib.rs"), "w") as f:
                f.write("let t = SystemTime::now();\n")
            findings = scan(tmp, "rust")
            self.assertFalse(
                any("/tools/" in f for f in findings),
                f"Self-Exclusion verletzt — Findings aus tools/: {findings}",
            )
            self.assertTrue(
                any("lib.rs" in f for f in findings),
                "True Positive aus src/ fehlt — Scanner zu stumpf (Test C-Verletzung)",
            )
            self.assertEqual(len(findings), 1, f"Erwartet genau 1 Finding: {findings}")


class TestB_StringLiterals(unittest.TestCase):
    """Kriterium 3: Docstrings und String-Literale sind ausgeschlossen."""

    def test_rust_string_und_kommentar_und_doc(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.makedirs(os.path.join(tmp, "src"))
            with open(os.path.join(tmp, "src", "lit.rs"), "w") as f:
                f.write(
                    "//! Docs: rand::thread_rng() wird verboten\n"
                    "/// SystemTime::now() darf nicht aufgerufen werden\n"
                    'let x = "SystemTime::now()";\n'
                    'let s = String::from("rand::"); // Kommentar: thread_rng\n'
                    "/* SystemTime::now() im Blockkommentar */\n"
                )
            self.assertEqual(scan(tmp, "rust"), [], "String-/Kommentar-FP erkannt")

    def test_python_docstring_string_kommentar(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.makedirs(os.path.join(tmp, "src"))
            with open(os.path.join(tmp, "src", "mod.py"), "w") as f:
                f.write(
                    '"""Generate a new random wallet."""\n'
                    'DOC = "uuid4"\n'
                    "# random\n"
                    'X = "Text mit time.time() und uuid4 innen"\n'
                )
            self.assertEqual(scan(tmp, "python"), [], "Docstring-/String-FP erkannt")


class TestC_TruePositive(unittest.TestCase):
    """Kriterium 4: echte Treffer werden weiterhin erkannt."""

    def test_rust_echter_aufruf(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.makedirs(os.path.join(tmp, "src"))
            with open(os.path.join(tmp, "src", "real.rs"), "w") as f:
                f.write("fn now() -> u64 { SystemTime::now() }\n")
            findings = scan(tmp, "rust")
            self.assertEqual(len(findings), 1)
            f0 = findings[0]
            self.assertIn("real.rs:1", f0, f"Datei/Zeile fehlt: {f0}")
            self.assertIn("Wall-Clock im Quellcode", f0, f"Pattern fehlt: {f0}")

    def test_python_echter_aufruf(self):
        with tempfile.TemporaryDirectory() as tmp:
            os.makedirs(os.path.join(tmp, "src"))
            with open(os.path.join(tmp, "src", "real.py"), "w") as f:
                f.write("import time\nprint(time.time())\n")
            findings = scan(tmp, "python")
            self.assertEqual(len(findings), 1)
            self.assertIn("real.py:2", findings[0])
            self.assertIn("Wall-Clock", findings[0])


class TestD_Saeule2Unveraendert(unittest.TestCase):
    """Nebenbeweis: Fail-closed-Saeule 2 darf durch die Haertung nicht erodieren."""

    def test_skip_tests_exit(self):
        with tempfile.TemporaryDirectory() as tmp:
            r = det.run_tests("exit 3", tmp)
            self.assertEqual(r[0], 3, "Test-Kommando muss 1:1 durchgereicht werden")


if __name__ == "__main__":
    unittest.main(verbosity=2)
