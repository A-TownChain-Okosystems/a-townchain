# FLEET_GATE_POLICY

Version: 1.0.0
Status: APPROVED (Owner-Direktive, 2026-10-06)
Entscheidung: AD-051 (a-townchain-os-docs/docs/DECISIONS_REGISTER.md)

## Entscheidung

Der organisation-weite Fleet-Audit ist ein **globales Main/Schedule-Health-Gate** und kein PR-Merge-Gate.

| Ebene | Gates | Verhalten |
|---|---|---|
| PR | engineering-ci (verify/Audit-Gate), Markdown Lint, Engineering Self-Test | fail-closed, blockieren den PR |
| Main/Schedule | Fleet Audit (Push auf main, woechentlich Mo 02:17 UTC, Dispatch) | fail-closed, markiert Flotten-Trust-Status, blockiert Releases |

Das fail-closed-Prinzip bleibt auf beiden Ebenen unveraendert. Die Aenderung ist eine Scope-Korrektur, keine Aufweichung: eine Fleet-weite Invariante gehoert nicht auf einen einzelnen PR, dessen Scope die Flotte nicht ist.

## Umsetzungsordnung (anti-retrofitting), vollstaendig vor Neubewertung PR 7

1. SEC-AUDIT-001 (atc-ide) triagiert: FALSE POSITIVE — UI-Passwort-Platzhalter, kein Credential-Material; Fix atc-ide 16f034a; Scanner-Schaerfe unveraendert (Schwelle 20, echte ghp_/github_pat_/AKIA-Token feuern weiterhin).
2. 19 Fleet-Findings aus Run 153 registriert: F-141 bis F-159 (atc-standards 96d5f12), je Owner und Status OPEN — vor der Gate-Verschiebung.
3. Diese Policy auf main dokumentiert — vor der Workflow-Aenderung und vor der Neubewertung von PR 7.
4. fleet-audit.yml: Trigger von push/pull_request beschraenkt auf push branches: [main] + Schedule + Dispatch.
5. PR 7 (Head 7fd11b3) wird ausschliesslich gegen diese bereits geltende Semantik neu bewertet.

## Coverage und Taxonomie

- Verbindliche Statusformulierung: `FLEET_PARTIAL_VERIFIED: 32/33; private repository unavailable under GITHUB_TOKEN` (Run 153, 2026-10-06).
- demo-repository ist UNAVAILABLE (Discovery-Luecke unter GITHUB_TOKEN), F-059 (Owner-Zweckentscheidung) zugeordnet — nicht RESIDUAL, nicht FAILED.
- Volle Discovery-Abdeckung erfordert ein least-privilege Read-PAT als Repo-Secret (Owner-Aktion). Kein admin:org, kein Token-Broadening.
- Per-Repo-Klassifizierung verbindlich: PASS / FINDING / UNAVAILABLE / RESIDUAL.

## Evidence

- Run 153: atc-engineering Actions-Run 37473775619 (Head 7fd11b3), Artifact fleet-audit-results, SUMMARY 13 PASS / 19 FINDINGS / 1 UNAVAILABLE.
- Findings: atc-standards registry/findings.yaml F-141 bis F-159.
