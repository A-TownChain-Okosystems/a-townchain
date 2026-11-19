---
document_id: ATC-PR-TRIAGE-001
title: "PR-Triage — Bestand und Einzelentscheidung"
status: GENERATED
normative: false
generator: backlog/tools/pr_triage.py
---

# PR-Triage

> Nicht-normativ, generiert. Keine Aktion an GitHub ausgefuehrt: keine Merges,
> keine Schliessungen. Die Entscheidung ist ein Vorschlag, keine Vollmacht.

## Bestand

| Kennzahl | Wert |
|---|---|
| Offene PRs | 244 |
| Repositories mit offenen PRs | 24 |
| KEEP | 0 |
| UPDATE | 19 |
| MERGE_CANDIDATE | 6 |
| CLOSE | 0 |
| BLOCKED | 194 |
| DUPLICATE | 25 |

## Die zehn im Backlog benannten PRs

| PR | Repository | Entscheidung | Befund |
|---|---|---|---|
| #116 | a-townchain | BLOCKED | 3 fehlgeschlagene Checks |
| #118 | a-townchain | BLOCKED | 1 fehlgeschlagene Checks |
| #243 | a-townchain-ecosystem | BLOCKED | Merge-Konflikt mit der Basis |
| #247 | a-townchain-ecosystem | BLOCKED | 19 fehlgeschlagene Checks |
| #248 | a-townchain-ecosystem | BLOCKED | 15 fehlgeschlagene Checks |
| #121 | atc-standards | BLOCKED | Merge-Konflikt mit der Basis |
| #123 | atc-standards | BLOCKED | 3 fehlgeschlagene Checks |
| #16 | atc-vm | BLOCKED | SSOT-Freeze: kanonischer Ort ist a-townchain/components/vm |
| #40 | atclang | UPDATE | Draft, nicht abnahmebereit |
| #117 | globus-os | BLOCKED | 1 fehlgeschlagene Checks |

## Mergekandidaten

| PR | Repository | Checks | Titel |
|---|---|---|---|
| #101 | a-townchain-ecosystem | 42 gruen | ci: bind determinism gate to exact execution SHA |
| #56 | a-townchain-os-docs | 6 gruen | chore(deps-dev): bump jsdom from 29.1.1 to 31.0.1 in /aistud |
| #33 | atc-engineering | 8 gruen | fix(ci): verify exact head SHA and publish binary evidence f |
| #15 | atc-sdk | 16 gruen | chore(deps): update click requirement from >=8.5.0 to >=8.5. |
| #46 | atc-standards | 21 gruen | SCR-0129: Standalone First, Ecosystem Second architecture co |
| #30 | genesis-engine | 16 gruen | docs: refresh Genesis Engine status evidence notice |

## Groesste Dateikonflikte

| Repository | PRs | Gemeinsame Dateien |
|---|---|---|
| a-townchain | #116 / #118 | 68 |
| globus-os | #32 / #120 | 18 |
| globus-os | #117 / #120 | 11 |
| globus-os | #23 / #32 | 10 |
| a-townchain | #38 / #90 | 9 |
| globus-os | #23 / #120 | 9 |
| a-townchain | #114 / #116 | 8 |
| a-townchain | #114 / #118 | 8 |
| aurora-ai | #24 / #48 | 8 |
| globus-os | #13 / #82 | 8 |

## Regel

Jede Entscheidung ist ein Vorschlag mit Begruendung. Ein Merge oder Close
erfordert eine bewusste Handlung am jeweiligen PR.
