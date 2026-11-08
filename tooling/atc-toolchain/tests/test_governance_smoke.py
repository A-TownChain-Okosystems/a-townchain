"""Governance-Smoke-Test fuer atc-toolchain (R1).

Prueft die Governance-Metadaten des eigenen Repos (Spiegel der
Audit-Regeln V-04/V-02 aus atc-standards). Kein Produkt-Claim:
Die Werkzeug-Implementierung (CLI/Build/Audit-Runner) folgt und
bringt eigene Tests mit (SCR-0080: IMPLEMENTED != VERIFIED).
"""
import os
import re
import unittest

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def read(rel):
    with open(os.path.join(ROOT, rel), encoding="utf-8") as f:
        return f.read()


class GovernanceSmoke(unittest.TestCase):

    def test_repository_yaml_pflichtfelder(self):
        meta = read(".atc/repository.yaml")
        for key in ("standard", "name", "classification", "maturity",
                    "organization", "primary", "criticality"):
            self.assertRegex(meta, r"(?m)^\s*%s\s*:" % key)

    def test_repository_yaml_r_level(self):
        m = re.search(r"maturity:\s*(R[0-4])", read(".atc/repository.yaml"))
        self.assertIsNotNone(m)
        self.assertEqual(m.group(1), "R1")

    def test_evidence_ehrlichkeit(self):
        ev = read(".atc/evidence/evidence.yaml")
        self.assertIn("schema_version", ev)
        self.assertRegex(ev, r"r_level:\s*R1")
        self.assertIn("CLAIMED != PASS", ev)

    def test_readme_kernabschnitte(self):
        rm = read("README.md")
        for s in ("urpose", "cope", "rchitect", "nstall", "icens"):
            self.assertRegex(rm, s)

    def test_lizenz_ist_apache(self):
        self.assertIn("Apache License", read("LICENSE"))


if __name__ == "__main__":
    unittest.main()
