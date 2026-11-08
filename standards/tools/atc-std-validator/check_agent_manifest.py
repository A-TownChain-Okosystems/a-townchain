#!/usr/bin/env python3
"""Agent-Manifest-Enforcement (ATC-AAS-025 §2/§3, Vollmandat F-018).

Prueft, dass der Agent vollstaendig an die Registry gebunden ist:
  A1  .github/ai/agent.yaml required_standards == ALLE Registry-Standards
  A2  AGENT_MANIFEST.md enthaelt das Voll-Compliance-Mandat (Verbindlich-
      keitsklausel + dynamische Bindung + Umsetzungspflicht)
  A3  AGENTS.md referenziert das Mandat (Vollstaendigkeit statt Auszug)
  A4  AUD-Records vorhanden (AI-DEV-009) — mind. eine Audit-Datei
Exit-Code 0 = PASS (Gate), 1 = FAIL.
CI-ungebunden lauffaehig: python3 tools/atc-std-validator/check_agent_manifest.py
"""

import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
REGISTRY = os.path.join(ROOT, "registry", "standards.yaml")
MANIFEST = os.path.join(ROOT, ".github", "ai", "agent.yaml")
AGENT_MD = os.path.join(ROOT, "AGENT_MANIFEST.md")
AGENTS_MD = os.path.join(ROOT, "AGENTS.md")
AUDIT_DIR = os.path.join(ROOT, ".github", "ai", "audit")


def main():
    fails = []
    # yaml-frei (SCR-0053): Laeuft auf jedem CI-Runner ohne PyYAML-Abhaengigkeit
    reg_text = open(REGISTRY, encoding="utf-8").read()
    ids = set(re.findall(r"id:\s*(ATC-[A-Z0-9-]+)", reg_text))

    # A1: Repo-Manifest referenziert ALLE Registry-Standards
    man_text = open(MANIFEST, encoding="utf-8").read()
    required = set(re.findall(r"^-\s*(ATC-[A-Z0-9-]+)\s*$", man_text, re.M))
    fehlt = sorted(ids - required)
    zuviel = sorted(required - ids)
    if fehlt:
        fails.append(
            "A1: required_standards fehlen %d Registry-Standards: %s%s"
            % (len(fehlt), ", ".join(fehlt[:5]), " …" if len(fehlt) > 5 else "")
        )
    if zuviel:
        fails.append("A1: required_standards enthalten unbekannte IDs: " + ", ".join(zuviel[:5]))

    # A2: Voll-Compliance-Mandat in AGENT_MANIFEST.md
    am = open(AGENT_MD, encoding="utf-8").read()
    if not re.search(r"Standard-Compliance-Mandat", am):
        fails.append("A2: AGENT_MANIFEST.md ohne Standard-Compliance-Mandat")
    for k in ("MUSS sämtliche Standards", "Dynamische Bindung", "Umsetzungspflicht"):
        if k not in am:
            fails.append("A2: Mandat-Klausel fehlt: " + k)

    # A3: AGENTS.md verweist auf Vollmandat (nicht 'Auszug')
    ag = open(AGENTS_MD, encoding="utf-8").read()
    if re.search(r"Auszug", ag):
        fails.append("A3: AGENTS.md nennt Standards nur als 'Auszug' — Vollmandat erforderlich")
    if "Standard-Compliance-Mandat" not in ag and "ALLE" not in ag:
        fails.append("A3: AGENTS.md ohne Mandat-Verweis")

    # A4: Audit-Records (AI-DEV-009)
    if not os.path.isdir(AUDIT_DIR) or not [
        f for f in os.listdir(AUDIT_DIR) if f.endswith(".yaml")
    ]:
        fails.append("A4: Keine AUD-Records in .github/ai/audit/ (AI-DEV-009)")

    print("== Agent-Manifest-Enforcement (Vollmandat: %d Standards) ==" % len(ids))
    if fails:
        for f in fails:
            print("  [FAIL] " + f)
        print("RESULT: FAIL — Agent ist nicht vollstaendig an die Registry gebunden")
        return 1
    print("  [PASS] A1: agent.yaml referenziert alle %d Registry-Standards" % len(ids))
    print("  [PASS] A2: Voll-Compliance-Mandat in AGENT_MANIFEST.md")
    print("  [PASS] A3: AGENTS.md verweist auf Vollmandat")
    print("  [PASS] A4: AUD-Records vorhanden")
    print("RESULT: GATE PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
