---
standard:
  id: ATC-STD-REPO-DISCOVERY-009
  title: "Documentation Gap Detection"
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

# ATC-STD-REPO-DISCOVERY-009 — Documentation Gap Detection (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Jede relevante Aenderung MUSS auf Dokumentationsbedarf geprueft werden:
Code geaendert → Doku pruefen (ERR-006); neue API → API-Doku; neue Funktion →
Doku/Beispiele; Nutzer-sichtbare Aenderung → CHANGELOG. Fehlende Doku wird
nicht nur gemeldet, sondern als TODO-Folgeaufgabe erzeugt (Abschluss-Blocker
nach REPO-MAINT-001 §12).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-090 | Doku-Bedarfspruefung je relevanter Aenderung (Doku-Kategorien) |
| REQ-RD-091 | Doku-Luecken erzeugen TODO-Eintraege mit Verweis auf Change/Scan-ID |
| REQ-RD-092 | Doku-Luecke bei P1-Aenderungen blockiert den Merge-Abschluss |

## Implementierungsstatus
SPECIFIED — Regelwerk in scan.py-Pruefliste vorgesehen (README/ARCHITECTURE/
CHANGELOG-Beruehrung je Change Type).

## Security Considerations

Doku-Gap-Checks verhindern, dass Sicherheitsmechanismen undichtig oder unzureichend dokumentiert werden; keine Zugangsdaten in TODOs.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: ERR-006, MD-001, README-001, REPO-MAINT-001 §9/§12 · INFORMATIVE: SCR-0046
