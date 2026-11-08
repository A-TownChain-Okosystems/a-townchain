---
standard:
  id: ATC-STD-ERR-004
  title: "Error Propagation Scan"
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

# ATC-STD-ERR-004 — Error Propagation Scan (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
**Der zentrale Standard der Familie:** Nach einem Fehler E MUSS eine
Fehler-Replikationspruefung durchgefuehrt werden — im Fund-Repository entlang des
Fehlermusters.

## Kernregeln
Nicht: Fehler gefunden → behoben → fertig. Sondern die verbindliche Kette:

```
Fehler gefunden → klassifizieren (ERR-002) → Ursache bestimmen (ERR-003) →
Fehlermuster extrahieren → gesamtes System durchsuchen → andere Vorkommen
identifizieren → Abhaengigkeiten pruefen (ERR-007) → Doku pruefen (ERR-006) →
Tests erstellen/erweitern (ERR-008) → Fehler beheben → Regressionstest →
Praevention (ERR-009) → erneuter Systemscan (ERR-012) → Audit-Abschluss
```

**Muster-Suche statt Literal-Suche (zwingend):** Es DARF nicht nur nach dem konkreten
Fehlertext gesucht werden. Beispiel — Fehler „Version 1.4.0 wird als kompatibel mit
API 2.0 angegeben": gesucht werden MUSS das Muster in allen Dimensionen —
Versionsnummern, Kompatibilitaetsangaben, API-Versionen, Dependency-Versionen,
README, CHANGELOG, Doku, CI/CD, Tests, Build-Konfiguration, Docker, Package Manager,
Code, Schemas, Interfaces. Aus „Finde denselben Fehler" wird
„Finde alle Instanzen desselben Fehlermusters".

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-040 | Propagation-Scan je relevantem Fehler vor Fix-Abschluss |
| REQ-ER-041 | Suche nach Muster in allen Dimensionen (mind. die 14 genannten), nie nur Literal |
| REQ-ER-042 | Scan-Doku: Dimensionen, Treffer (occurrences_found), Fundstellen |

## Implementierungsstatus
SPECIFIED — Umsetzung: Scan-Block im BUG-002-Record; Validator prueft Scan-Nachweis.

## Security Considerations

Propagation-Scans laufen nur mit read-only Zugriff; Scan-Logs maskieren Zugangsdaten ($ENV-Platzhalter, Regel ATC-STD-203).

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-ERR-000/-002/-003/-006/-007/-008/-012, ATC-STD-BUG-002 ·
INFORMATIVE: SCR-0044
