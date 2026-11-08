# SCR-0052 — ci.yml: Registry-driven Vollvalidierung (Owner-Aktion wegen GH013)

## Status
**BEREIT — wartet auf Owner-Anwendung** (kongruent zu SCR-0051). Der OAuth-Push
der Workflow-Datei wird per GH013 abgelehnt (kein workflow-Scope).

## Befund (externes Audit 08.09., P1-002)
`.github/workflows/ci.yml` validiert im Schritt „ATC Standards Validation"
nur 4 explizit hardcodierte Standards (000, 201, 202, 203) — die README-Aussage
„431/431 COMPLIANT" wird durch DIESEN Workflow nicht bewiesen. (Anmerkung:
naming-governance.yml führt bereits validate_all.py = alle Standards aus;
die Luecke ist ci.yml-spezifisch, aber ein echter Governance-Wart.)

## Patch (exakt)
Schritt in ci.yml ersetzen:

```yaml
      - name: ATC Standards Validation (registry-driven, ATC-STD-000 Compliance)
        run: python3 tools/atc-std-validator/validate_all.py
```

## Wirkung
- CI validiert registry-basiert ALLE registrierten Standards (0 Orphans,
  0 Missing, 0 Duplicates, 0 invalid) — die Registry wird im Workflow
  tatsächlich zum SSOT.
- Hardcodierte Dateilisten entfallen; neue Standards sind automatisch validiert.
