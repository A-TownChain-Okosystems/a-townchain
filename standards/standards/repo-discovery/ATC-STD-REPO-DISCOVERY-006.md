---
standard:
  id: ATC-STD-REPO-DISCOVERY-006
  title: "Impact Analysis"
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

# ATC-STD-REPO-DISCOVERY-006 — Impact Analysis (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Neue Inhalte werden auf Auswirkungen geprueft: NEW MODULE → Architecture-Update?
NEW API → API-Doku? NEW FUNCTION → Testabdeckung? NEW AUTH MODULE → Security
Review? NEW DEVELOPMENT RULE → Standard-Kandidat (RD-003)? USER-VISIBLE CHANGE →
CHANGELOG? Je Pruefung ein Urteil REQUIRED/NOT_REQUIRED mit Begruendung.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-060 | Impact-Analyse je neuem Inhalt ueber die 6 Wirkungsbereiche |
| REQ-RD-061 | Je Bereich ein dokumentiertes Urteil (REQUIRED + Massnahme oder NOT_REQUIRED + Begruendung) |
| REQ-RD-062 | REQUIRED-Massnahmen werden als TODO/Folgeaufgabe erzeugt (RD-010 §15) |
| REQ-RD-063 | Architektur- und Schnittstellen-Aenderungen lösen Cross-Repo-Pruefung (RD-005) aus |

## Implementierungsstatus
SPECIFIED — Format im scan.py-Report (impact-report.json) vorgesehen;
Ausbaustufe: heuristische Ableitung der 6 Bereiche aus Change Type + Pfad.

## Security Considerations

Impact-Analysen markieren Security-Relevanz automatisch (RD-008-Kopplung); keine sensiblen Daten in impact-report.json.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: RD-002/-003/-009, ERR-006/-007, REPO-MAINT-001 §12 · INFORMATIVE: SCR-0046
