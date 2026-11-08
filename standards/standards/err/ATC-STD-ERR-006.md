---
standard:
  id: ATC-STD-ERR-006
  title: "Documentation Consistency Check"
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

# ATC-STD-ERR-006 — Documentation Consistency Check (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P1 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Ein Softwarefehler kann gleichzeitig ein Dokumentationsfehler sein — und umgekehrt.

## Kernregeln
- **Code geaendert → Doku pruefen.** **Doku-Fehler gefunden → Implementierung pruefen.**
- Beispiel: Doku sagt „API unterstuetzt Feature X", Code: Feature X existiert nicht —
  dann MUSS geprueft werden: API falsch dokumentiert? Implementierung fehlt?
  Anderes Repository betroffen? Beispiele falsch? Tests falsch? Architektur-Doku
  veraltet? Widerspruechliche Standards?
- Bidirektionale Pruefung ist Teil des Propagation-Scans (ERR-004) und des
  Maintenance-Zyklus (REPO-MAINT-001 §9).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-060 | Code-Aenderung ohne Doku-Pruefung darf nicht abgeschlossen werden |
| REQ-ER-061 | Doku-Abweichung loest Implementierungs-Pruefung (und ggf. Cross-Repo-Scan) aus |

## Implementierungsstatus
SPECIFIED — Umsetzung: Doku-Check als Maintenance-Report-Feld (REPO-MAINT §16).

## Security Considerations

Doku-Checks pruefen auch auf versehentlich dokumentierte Zugangsdaten (Security-Doku-Regel ATC-STD-203).

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-ERR-000/-004, ATC-STD-MD-001, ATC-STD-README-001,
ATC-STD-REPO-MAINT-001 §9 · INFORMATIVE: SCR-0044
