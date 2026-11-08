---
standard:
  id: ATC-STD-REPO-DISCOVERY-003
  title: "Standard Candidate Detection"
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

# ATC-STD-REPO-DISCOVERY-003 — Standard Candidate Detection (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Hoechste Prioritaet: implizit neu entstandene Standards erkennen. Prueffrage:
**Enthaelt der neue Inhalt Regeln, Anforderungen, verbindliche Prozesse oder
wiederverwendbare Vorgaben?** Wenn ja → STANDARD CANDIDATE (z.B. „Every service
must provide a health endpoint" → ATC-STD-SERVICE-HEALTH-001).

## Kandidaten-Lifecycle (kein Auto-Approve!)
DISCOVERED → CANDIDATE → REVIEW → APPROVED → ACTIVE; daneben REJECTED (kein
Standard) und MERGED (in bestehenden Standard integriert). Verbindlichkeit
entsteht NUR via ATC-STD-000 §9/§37 Registry-First.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-030 | Regel-Signale in neuem Inhalt werden geprueft (MUST/muss/verbindlich/Pflicht-Muster) |
| REQ-RD-031 | Kandidaten durchlaufen den 6-Status-Lifecycle; kein Auto-Approve |
| REQ-RD-032 | Kandidaten-Registry maschinenlesbar (candidates.json) |
| REQ-RD-033 | REVIEW-Pflicht mit fachlichem Pruefer dokumentiert |

## Implementierungsstatus
IMPLEMENTED (erster Zyklus) — Evidence: scan.py-Regel-Signal-Scan;
candidates.json im ersten Scan-Report.

## Security Considerations

Kandidaten-Extrakte enthalten Regel-Signale, keine sensiblen Konfigurationswerte; REVIEW-Prozess verhindert versehentliche Standardisierung von Zugangsdaten.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: ATC-STD-000 (§37 Registry-First), RD-001/-004 · INFORMATIVE: SCR-0046
