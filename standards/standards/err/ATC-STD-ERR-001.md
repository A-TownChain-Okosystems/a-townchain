---
standard:
  id: ATC-STD-ERR-001
  title: "Error Discovery"
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

# ATC-STD-ERR-001 — Error Discovery (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Fehler eindeutig erfassen. ERR-001 definiert die Erfassungs-Pflicht; das
Record-Format und Severity-Modell kommen normativ aus ATC-STD-BUG-001 (keine
Re-Spezifikation).

## Kernregeln
- Jeder gefundene Fehler ERHAELT eine error_id `ATC-ERR-NNNN` (fortlaufend, Registry-First).
- Pflichtfelder: Fundstelle (Repo/Datei/Zeile), Zeitpunkt, Beobachter (Agent/Mensch),
  Schwere (BUG-001 S0–S4), Erstbeschreibung.
- Der Record wird im Bug-Documentation-Format (BUG-002) gefuehrt; error_id verknuepft
  den Knowledge Record (ERR-013).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-010 | Jeder Fehler erhaelt error_id ATC-ERR-NNNN + BUG-001-Record |
| REQ-ER-011 | Fundstelle/Zeitpunkt/Beobachter/Schwere sind Pflichtfelder |

## Implementierungsstatus
SPECIFIED — Umsetzung: error_id-Serie im Docs-Hub (docs/error-records/INDEX.md).

## Security Considerations

Discovery-Records enthalten keine Zugangsdaten; Schwere-Angaben folgen BUG-001 (S0-S4). Fundstellen in Logs/Reports nur mit $ENV-Platzhaltern.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-ERR-000/-013 · INFORMATIVE: SCR-0044
