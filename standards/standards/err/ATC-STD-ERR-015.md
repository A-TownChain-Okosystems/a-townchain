---
standard:
  id: ATC-STD-ERR-015
  title: "Error Prevention Gate"
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

# ATC-STD-ERR-015 — Error Prevention Gate (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Fuer kritische Fehlerklassen wird der Fehler kuenftig technisch unmoeglich oder
automatisch blockierbar: ein Gate in der Merge-Kette.

## Kernregeln — Gate-Kette (bis Merge)

```
Developer → Commit → Pre-Commit Checks → Static Analysis → Unit Tests →
Integration Tests → Repository Standards Check (Governance-CI) →
Documentation Check → Security Check → CI → Merge
```

Wird derselbe Fehler erneut eingefuehrt: **FAIL → Merge blockiert → Fehlerursache
angezeigt → Korrektur erforderlich.** Gates existieren teilweise schon
(Governance-CI 26/26, Standards Validator S-01..S25, Naming-Gate); ERR-015 verpflichtet
je kritischer Fehlerklasse den zugehoerigen Gate (Massnahmen-Matrix ERR-009).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-150 | Je kritischer Fehlerklasse ein Gate in der Merge-Kette dokumentiert und errichtet |
| REQ-ER-151 | Re-Einfuehrung desselben Fehlers blockiert das Merge mit Ursachen-Anzeige |

## Implementierungsstatus
PARTIAL — Evidence: Governance-CI 26/26, Standards Validator, Branch-Protection
(atc-standards), Build/Test-CI in atc-contracts vorbereitet (Owner-Aktion ci.yml,
Issue #99); Version-Gate ueber Issue #96 geplant.

## Security Considerations

Prevention-Gates duerfen keine Zugangsdaten in Gate-Konfigurationen enthalten; Security-Gates folgen ATC-STD-203 (Secret-Scanning, Push-Protection).

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-ERR-000/-008/-009, ATC-STD-BUG-004 (Merge Gate), ATC-STD-203,
ATC-STD-REPO-MAINT-001 §11 · INFORMATIVE: SCR-0044, Issues #95/#96/#99
