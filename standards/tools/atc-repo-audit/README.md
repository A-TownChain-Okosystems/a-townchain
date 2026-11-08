# atc-repo-audit (v0.1.0)

Automatischer ATC Repository Auditor — Validator fuer ATC-STD-201 v1.0.0
(AD-031). Prueft V-01…V-16 der Compliance-Matrix, erzeugt den Health Score und
entscheidet GATE: PASS / NO-GO.

```bash
python3 tools/atc-repo-audit/atc_repo_audit.py <repo>            # Level aus .atc/repository.yaml
python3 tools/atc-repo-audit/atc_repo_audit.py <repo> --level R4  # Level-Override
```

Exit-Code 0 = PASS, 1 = NO-GO (CI-tauglich). Nur Python-stdlib, keine Abhaengigkeiten.
Semantik: MUST-FAIL = GATE: NO-GO; SHOULD-Verstoss = WARN (Score-Abzug); Score >= 85
und 0 FAILs = PASS (ATC-STD-203 §15).
