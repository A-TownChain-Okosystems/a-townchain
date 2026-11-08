---
document_id: ATC-DOC-CONTRIB-001
title: Contributing — atc-standards
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-14
standard: ATC-STD-MD-001
---

# Contributing to atc-standards

Beiträge zum Governance-Repository laufen ausschließlich über den Standards-Prozess (**ATC-STD-000 v1.3.0**).

## Wie beitragen

- **Neuer Standard:** Registry-First — Eintrag in `registry/standards.yaml` (ID nach ATC-STD-000 §37, Kategorie nach `categories.yaml`), Standard-Datei nach §9-Struktur, Validierung mit `validate_all.py`, dann Owner-§9-Freigabe.
- **Änderung an APPROVED-Standards:** Nur via SCR (ATC-STD-000 §19–33); Immutabilität per §30.
- **Findings:** Nach ATC-STD-BUG-001 registrieren (Severity nach BUG-002), Remediation mit AUD-Record (AI-DEV-009).
- **Commits:** Conventional Commits nach AI-DEV-007 v1.0.1 (feat/fix/docs/security/build/ci), Agent-Kennzeichen als Trailer.

## Vor jedem Push

```bash
python3 tools/atc-std-validator/validate_all.py
python3 tools/atc-repo-audit/atc_repo_audit.py . --level R3
python3 tools/atc-std-validator/check_agent_manifest.py
python3 tools/atc-readme-validator/check_readme.py .
python3 tools/atc-md-validator/check_md.py .
```

## AI Agent Instructions

Repository development is governed by the current APPROVED ATC-STD-000 and the standards referenced by the Registry SSOT. Version/status claims in generated views must be derived from `registry/standards.yaml`, never hard-coded.

### Required Workflow

1. Inspect STATUS.md · 2. Read applicable standards · 3. Inspect ARCHITECTURE.md · 4. Identify current task · 5. Implement change · 6. Run tests/gates · 7. Update documentation · 8. Update CHANGELOG · 9. Verify repository consistency (all gates green).
