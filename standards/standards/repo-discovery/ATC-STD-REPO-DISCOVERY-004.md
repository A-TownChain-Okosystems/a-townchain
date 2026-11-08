---
standard:
  id: ATC-STD-REPO-DISCOVERY-004
  title: "Duplicate Detection"
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

# ATC-STD-REPO-DISCOVERY-004 — Duplicate Detection (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Neue Inhalte gegen bestehende Standards pruefen — Ergebnis-Triade:
DUPLICATE (bereits abgedeckt) / EXTENSION (bestehenden Standard erweitern) /
NEW STANDARD (wirklich neu). Verhindert doppelte, widerspruechliche und parallele
Standards, veraltete Regeln und mehrere Standards fuer dasselbe Problem.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-040 | Je Kandidat: Suche gegen Standards-Registry vor Neueintrag |
| REQ-RD-041 | Ergebnis-Triade dokumentiert (duplicate/extension/new) |
| REQ-RD-042 | Duplikate werden gemergt (MERGED), nie doppelt eingetragen |

## Implementierungsstatus
SPECIFIED — Mechanik etabliert (S-17 Duplicate Detection der Validator-Suite +
Registry-First §37); Kopplung an Kandidaten-Pipeline folgt mit Scan-Integration.

## Security Considerations

Duplikat-Suche laeuft gegen die Registry (read-only); keine Dateisystem-Inhalte an Dritte.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: ATC-STD-000 (§37/§7.8 S-17), RD-003 · INFORMATIVE: SCR-0046
