#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""validate_maintenance_readiness.py - Maintenance Readiness Gate (ATC-STD-MAINT-000 Par.7.2).

Implementiert die normative Kernregel der MAINT-Familie als maschinenpruefbares
Release-Gate (Welle 1, SCR-0123):

  'A system MUST NOT be released to a lifecycle state requiring operational
   support unless its required maintenance capabilities are implemented,
   validated, documented, and evidenced.'

Pruefungen (fail-closed):
  1. SCHEMA: Jeder Readiness-Record in registry/maintenance/readiness/*.yaml
     validiert gegen schemas/maintenance/maintenance-readiness.schema.json
     (JSON Schema 2020-12).
  2. PFLICHT-ABDECKUNG: Komponenten mit Live-Releases (Org-Audit 14.09.:
     a-townchain-os v2.0.0, atc-standards v1.1.0) MUESSEN einen Record haben.
  3. SEMANTIK (MAINT-000 Par.7.2): result=PASS ist nur zulaessig, wenn ALLE 10
     Required-Felder state=complete sind; fuer critical_maintenance (M2/M3)
     muessen zusaetzlich alle 5 Felder complete sein. Ein 'PASS' mit
     unvollstaendigen Feldern ist ein Fake-PASS => GATE FAIL (hard).

Ehrlichkeit vor Whitewashing: Ein Record mit result=FAIL ist ein LEGITIMER
BLOCK-Zustand (naechstes Release blockiert, Luecken sichtbar) und laesst das
Gate NICHT scheitern - exakt wie in MAINT-000 Par.7.2 definiert (FAIL = BLOCK,
nicht FAIL = Pipeline-Fehler).

Exit-Codes: 0 = Gate PASS (Records ehrlich + vollstaendig abgedeckt),
            1 = Gate FAIL (Schema-Verstoss, fehlender Pflicht-Record oder Fake-PASS).
"""

import glob
import json
import os
import sys

try:
    import yaml
    from jsonschema import Draft202012Validator
except ImportError:
    print("FEHLER: pyyaml + jsonschema erforderlich (pip install pyyaml jsonschema)")
    sys.exit(1)

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
READY_DIR = os.path.join(ROOT, "registry", "maintenance", "readiness")
SCHEMA = os.path.join(ROOT, "schemas", "maintenance", "maintenance-readiness.schema.json")

# Pflicht-Abdeckung: Komponenten mit Live-Releases MUESSEN einen Readiness-Record
# haben (Org-Audit 14.09. 2026, Live-API-Evidence: Releases 2/30 Repos).
REQUIRED_COMPONENTS = ["a-townchain-os", "atc-standards"]

REQUIRED_FIELDS = [
    "maintenance_owner",
    "maintenance_documentation",
    "dependency_inventory",
    "security_process",
    "test_suite",
    "rollback_strategy",
    "compatibility_strategy",
    "monitoring",
    "evidence_collection",
    "lifecycle_status",
]
CRITICAL_FIELDS = [
    "independent_validation",
    "security_review",
    "rollback_test",
    "incident_record",
    "audit_evidence",
]


def main() -> int:
    errors = []
    warnings = []

    schema = json.load(open(SCHEMA, encoding="utf-8"))
    validator = Draft202012Validator(schema)

    records = sorted(glob.glob(os.path.join(READY_DIR, "*.yaml")))
    seen = set()

    if not records:
        print("MAINT-Readiness: keine Records vorhanden.")
        print("MAINT-Readiness: %d Record(s) in registry/maintenance/readiness/" % len(records))

    for path in records:
        comp = os.path.splitext(os.path.basename(path))[0]
        seen.add(comp)
        try:
            doc = yaml.safe_load(open(path, encoding="utf-8"))
        except yaml.YAMLError as e:
            errors.append("%s: YAML unlesbar: %s" % (comp, e))
            continue

        # 1) Schema
        for err in sorted(validator.iter_errors(doc or {}), key=lambda e: list(e.path)):
            loc = "/".join(str(x) for x in err.path) or "<root>"
            errors.append("%s: SCHEMA-Verstoss (%s): %s" % (comp, loc, err.message))

        if not isinstance(doc, dict):
            continue
        mr = doc.get("maintenance_readiness") or {}
        result = doc.get("result")

        # 3) Semantik: Fake-PASS erkennen
        incomplete = [f for f in REQUIRED_FIELDS if (mr.get(f) or {}).get("state") != "complete"]
        cm = doc.get("critical_maintenance") or {}
        cm_incomplete = (
            [f for f in CRITICAL_FIELDS if (cm.get(f) or {}).get("state") != "complete"]
            if cm
            else []
        )

        if result == "PASS" and incomplete:
            errors.append(
                "%s: FAKE-PASS - result=PASS, aber unvollstaendig: %s "
                "(MAINT-000 Par.7.2: PASS erfordert alle 10 Felder complete)"
                % (comp, ", ".join(incomplete))
            )
        if result == "PASS" and cm and cm_incomplete:
            errors.append(
                "%s: FAKE-PASS - critical_maintenance unvollstaendig: %s"
                % (comp, ", ".join(cm_incomplete))
            )

        # Legitimer BLOCK-Zustand: ehrlich dokumentiert
        if result == "FAIL":
            warnings.append(
                "%s: RELEASE BLOCKED (ehrlicher BLOCK-Zustand) - unvollstaendig: %s"
                % (comp, ", ".join(incomplete + cm_incomplete) or "?")
            )
        if result not in ("PASS", "FAIL"):
            errors.append("%s: result fehlt oder ungueltig: %r" % (comp, result))

    # 2) Pflicht-Abdeckung
    for c in REQUIRED_COMPONENTS:
        if c not in seen:
            errors.append("Pflicht-Record fehlt: %s.yaml (Komponente hat Live-Release)" % c)

    # --- Report ---
    for w in warnings:
        print("  BLOCK: %s" % w)
    for e in errors:
        print("  FAIL:  %s" % e)
    if errors:
        print("RESULT: MAINTENANCE READINESS GATE FAIL (%d Verstoesse)" % len(errors))
        return 1
    print("RESULT: MAINTENANCE READINESS GATE PASS")
    print("  (PASS = Records schema-konform + Pflicht-Komponenten abgedeckt;")
    print("   BLOCK-Zustaende sind ehrliche Release-Sperren, keine Gate-Fehler)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
