# ROADMAP — atc-standards

## Q3/2026 (abgeschlossen)

1. ✅ **ATC-STD-000:** v1.3.0 APPROVED; historische v1.2.0-Artefakte bleiben als Audit-/Approval-Historie erhalten und sind nicht der aktuelle normative Stand.
2. ✅ **AI-DEV-Familie 001–012:** APPROVED.
3. ✅ **ATC-AAS-Block 001–025:** APPROVED.
4. ✅ **ATC-ENT-Layer 001–015:** APPROVED.
5. 🔄 **Registry/Implementation Truth:** aktueller Registry-State wird ausschließlich aus `registry/standards.yaml` und dem generierten State-Block abgeleitet.
6. 🔄 **Self-Compliance:** CI- und Cross-Registry-Gates werden gegen den aktuellen Registry-State ausgeführt.

## Bis 07.10.2026 (30-Tage-Fristen)

0. **ATC-STD-README-001:** Rollout der README-Konformität auf die im aktuellen Registry-/Profile-State erfassten Repositories; keine hardcodierte Organisationszahl.
0b. **ATC-STD-MD-001:** MD-Konformitäts-Rollout auf die im aktuellen Registry-/Profile-State erfassten Repositories.
0c. **ATC-STD-SC-001..020:** SC-Gate-Rollout auf die Contract-Repositories; Registry-Befüllung und Conformance-Evidence.

1. **SCR-0007 (F-017):** REQ-ID-Rollout AI-DEV/AAS/ENT + §9-Reststruktur — Owner-Entscheidung ausstehend.
2. **Interface-Test-Suiten IFC-0001..0010:** seed → active (ATC-STD-204 §9).
3. **Repo-Manifeste:** `.github/ai/agent.yaml` in den betroffenen R2+-Repositories.
4. **Commit-Trailer-Rollout:** statt Inline-[agent:]-Tag.

## Danach (Q4/2026)

- ENT-Ableitungen: org-units.yaml, repositories.yaml, risks.yaml.
- Erste DEC-Records und RISK-Registry-Seed.
- KPI-Reporting anbinden.
- Generator-Hardening: README/STATUS/AGENT_MANIFEST dürfen keine statischen Registry-Zahlen oder veralteten Governance-Versionen enthalten.

## SSOT-Regel

Organisations-, Registry- und Standardzahlen dürfen in dieser Roadmap nicht als dauerhaft aktuelle Fakten hardcodiert werden. Für den aktuellen Bestand gilt ausschließlich der generierte State-Block in `README.md` auf Basis von `registry/standards.yaml`.
