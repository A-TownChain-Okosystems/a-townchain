# Owner-Workflow-Patch (konsolidiert) — naming-governance.yml

> **Status:** Vom Agenten nicht anwendbar (**GH013**: OAuth-App ohne `workflow`-Scope).
> **Anwendung:** GitHub-UI → `.github/workflows/naming-governance.yml` → Inhalt ersetzen → Commit "fix(ci): SCR-0054/SG-02 — pip install, permissions, Validator-Verdrahtung".
> **Wirkt:** E-5-Regressionstest (T2) grün · Naming-Governance-CI grün · SG-02/F-051/F-049 RESOLVED · AuditGPT-Empfehlung (Validator-Wiring) umgesetzt.

```yaml
name: ATC Naming & Governance (ATC-STD-000 s7.11)

on:
  push:
  pull_request:

permissions:
  contents: read        # SG-02 (AUD-2026-0004, SecurityGPT): Least-Privilege

jobs:
  validate:
    runs-on: ubuntu-latest
    permissions:
      contents: read
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: "3.11"
      - name: Dependencies installieren (SCR-0054 / Issue #1 / CI-002/CI-003)
        run: pip install -r requirements.txt
      - name: Standards-Validierung (S-01...S-16 je Standard, S-17 Duplicate Detection, S-18/S-18a/S-20/S-21/E-5)
        run: python3 tools/atc-std-validator/validate_all.py
      - name: Repository-Audit (ATC-STD-201ff, Level R3)
        run: python3 tools/atc-repo-audit/atc_repo_audit.py . --level R3
      - name: README-Gate (ATC-STD-README-001, 13 Gates — F-049-Verdrahtung)
        run: python3 tools/atc-readme-validator/check_readme.py .
      - name: Markdown-Gate (ATC-STD-MD-001, MD-01..10 — F-049-Verdrahtung)
        run: python3 tools/atc-md-validator/check_md.py .
      - name: Agent-Manifest-Gate (ATC-AAS-025, A1-A4 — F-049-Verdrahtung)
        run: python3 tools/atc-std-validator/check_agent_manifest.py
      - name: Contract-Registry-Gate (ATC-STD-SC-002/SC-019 — F-049-Verdrahtung)
        run: python3 tools/atc-sc-validator/check_contracts.py
```

**Hinweise:**
1. `permissions: contents: read` auf Job- UND Workflow-Ebene (SecurityGPT SG-02): Pull-Requests aus Forks könnten sonst Schreibrechte erben.
2. `pip install -r requirements.txt` (SCR-0054): löst E-5/T2 und schaltet S-18/S-20 vom geprüften Fallback-Modus auf Vollprüfung (PyYAML).
3. Die vier neuen Gate-Steps verdrahten die existierenden, bisher entkoppelten Validatoren (AuditGPT D-01/D-02, F-049) — AUTOMATED-Klassifikation wird dadurch real: 9 Standards CI-erzwungen.
