---
standard:
  id: ATC-STD-REPO-DISCOVERY-007
  title: "Dependency Discovery"
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

# ATC-STD-REPO-DISCOVERY-007 — Dependency Discovery (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33
> (08.09.2026, SCR-0046). Familie FAM-49 (Repository Content Discovery,
> Kategorie repo-discovery). **Prioritaet:** P1 · **Grundregel der Familie:**
> Kein neuer Repository-Inhalt darf unbewertet bleiben.

## Zweck
Neue Abhaengigkeiten (Libraries, Packages, Images) werden erkannt und in
Review-Prozesse gezwungen: NEW DEPENDENCY → Dependency Review (Lizenz, Version,
Security, Pflegezustand). Kopplung: ATC-STD-204 (Dependency Governance) und
Dependabot-Alerts; Dependency-Changed ist zulaessiger Change Type (RD-002).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-RD-070 | Manifest-/Lock-Aenderungen werden als DEPENDENCY_CHANGED klassifiziert |
| REQ-RD-071 | Je neue Dependency ein Dependency Review (204-Kriterien) |
| REQ-RD-072 | Neuein fuehren nicht zu unaufgeloesten UNKNOWN-Zustaenden (ERR-007-Analogie) |

## Implementierungsstatus
SPECIFIED — Dependabot-Alerts existieren org-weit; Manifest-Diff-Hook im
scan.py vorgesehen (requirements.txt, Cargo.toml, package.json, go.mod).

## Security Considerations

Dependency-Reviews pruefen Lizenz + bekannte CVEs; neue Dependencies werden nie ohne Review uebernommen (ATC-STD-204).

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0046) |

## References
NORMATIV: ATC-STD-204, RD-002, ERR-007 · INFORMATIVE: SCR-0046
