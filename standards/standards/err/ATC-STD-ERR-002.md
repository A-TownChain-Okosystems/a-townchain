---
standard:
  id: ATC-STD-ERR-002
  title: "Error Classification"
  version: "1.0.0"
  status: approved
  category: err
  authority: A-TownChain Ecosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Gesamtes A-TownChain-Oekosystem: Repositories, Software, Smart Contracts, Doku, Standards, APIs, KI-Agenten, Infrastruktur, Prozesse"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-ERR-002 — Error Classification (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Fehler kategorisieren. ERR-002 ordnet jeden Fehler in Kategorien und Fehlerklassen ein;
die 18 Fehlerklassen und das Schwere-Modell kommen normativ aus BUG-001/BUG-005.

## Kernregeln
- Kategorie-Flags (mehrfach moeglich): software, documentation, configuration,
  standards, dependency, security, infrastructure.
- Fehlerklasse gemaess BUG-005 (18 Klassen) + Pattern-Match gegen ERR-010-Library.
- Die Klassifikation bestimmt das Pflichtset nach ERR-000 §3 (P0–P3).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-020 | Kategorie + BUG-005-Fehlerklasse je Fehler dokumentiert |
| REQ-ER-021 | Klassifikation bestimmt Pflichtset (ERR-000 §3) |

## Implementierungsstatus
SPECIFIED — Umsetzung: Klassifikationsfelder im BUG-002-Record.

## Security Considerations

Klassifikation darf keine sensiblen Daten offenlegen (Kategorie security nur verweisend, ohne Payload-Daten).

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-BUG-001, ATC-STD-BUG-005, ATC-STD-ERR-000/-010 · INFORMATIVE: SCR-0044
