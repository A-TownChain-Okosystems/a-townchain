# GitHub-Enforcement-Ableitung (ATC-GOV-001 Kap. 12, SCR-0097)

> Governance MUSS technisch erzwingen: Policy → Standard → Control → Evidence.

## ATC-GITHUB-001..010 (Ziel-Rulesets, org-weit)

| ID | Ruleset | Phase |
|---|---|---|
| ATC-GITHUB-001 | Protected Main (PR required, Reviews je Profil G1-G4) | 1 — aktiv teils (Branch-Protection atc-standards/.github) |
| ATC-GITHUB-002 | Signed Commits | 3 — erst nach Release-Key (AUD-001 F-006) |
| ATC-GITHUB-003 | Mandatory PR Review (review_level je Profil) | 1 — Rollout-Owner-Entscheidung |
| ATC-GITHUB-004 | CODEOWNERS Approval | 2 |
| ATC-GITHUB-005 | Required CI (Status Checks) | 1 — wo CI existiert |
| ATC-GITHUB-006 | Security Checks (Secret-Scanning, Code-Scanning) | 2 — CodeQL nur wo scannbarer Code |
| ATC-GITHUB-007 | No Force Push | 1 |
| ATC-GITHUB-008 | No Unauthorized Tag Modification (immutable Release-Tags) | 1 — aktiv auf atc-standards (Ruleset v1.1.0) |
| ATC-GITHUB-009 | Release Protection (Tags nur via Release-Prozess) | 2 |
| ATC-GITHUB-010 | Repository Creation Standard | 2 |

## Evidence je Ruleset

Ruleset-Konfiguration (API-Snapshot) + CI-Results + PR-Approvals + Audit-Log —
Ablage evidence/audits/github-rulesets/.

## Rollout

Phase 1 (sofort möglich): 001/003/005/007/008 auf alle governed Repos —
Owner-Freigabe nötig (org-weite Wirkung, Ein-Personen-Übergang: Reviews via
zweiten Account oder dokumentierte Exception). Phase 2: 004/006/009/010.
Phase 3: 002 nach GPG-Release-Key. Umsetzung via GitHub Admin API; Snapshot-
Audit danach (gov_drift-ähnlich).
