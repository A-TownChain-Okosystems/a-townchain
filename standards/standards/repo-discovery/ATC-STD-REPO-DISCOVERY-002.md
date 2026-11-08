---
standard:
  id: ATC-STD-REPO-DISCOVERY-002
  title: "Change Detection"
  version: "1.0.0"
  status: approved
  category: repo-discovery
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Repositories; Agenten, CI, Audits, Wartungszyklen"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-REPO-DISCOVERY-002 — Change Detection (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Jede inhaltliche Veraenderung erhaelt einen Change Type — nichts bleibt unklassifiziert.

## Change Types
NEW · MODIFIED · DELETED · RENAMED · MOVED · GENERATED · DEPENDENCY_CHANGED ·
INTERFACE_CHANGED · CONFIGURATION_CHANGED · SECURITY_RELEVANT.

## Pflichtpruefung nach jedem relevanten Merge

```
MERGE → DISCOVERY SCAN → NEW CONTENT?
  ├─ NO  → PASS
  └─ YES → CLASSIFY → IMPACT ANALYSIS (RD-006) → STANDARDS CHECK (RD-003)
           → DOCUMENTATION CHECK (RD-009) → TEST CHECK → SECURITY CHECK (RD-008)
           → REPORT (RD-010)
```

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-020 | Jede Aenderung erhaelt einen der 10 Change Types |
| REQ-RD-021 | Merge-Pflichtpruefung gemaess Kette; NO-Content = dokumentiertes PASS |
| REQ-RD-022 | Diff gegen Baseline (nicht nur HEAD-Stand) |
| REQ-RD-023 | GENERATED-Inhalte werden erkannt (keine Pseudo-Discovery von Generaten) |

## Implementierungsstatus
IMPLEMENTED (erster Zyklus) — Evidence: scan.py-Diff (NEW/MODIFIED/DELETED).

## Security Considerations

Change-Reports enthalten Pfade und Typen, niemals Dateiinhalte mit potenziellen Zugangsdaten.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: RD-001, BUG-004 (Merge-Gate), REPO-MAINT-001 §12 · INFORMATIVE: SCR-0046
