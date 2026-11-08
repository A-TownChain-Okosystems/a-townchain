---
standard:
  id: ATC-STD-ERR-008
  title: "Regression Test Requirement"
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

# ATC-STD-ERR-008 — Regression Test Requirement (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Jeder reproduzierbare Fehler MUSS nach Moeglichkeit einen Test erhalten.

## Kernregeln
Kette: Bug → Fix → Regression-Test → CI → Future Protection. Der Test MUSS
moeglichst VOR dem Fix fehlschlagen und NACH dem Fix erfolgreich sein. Damit ist
bewiesen: (1) Test reproduziert den Fehler, (2) Fix behebt ihn, (3) Test verhindert
Wiederauftreten. Der Test laeuft in CI (Build/Test-Gate, REPO-MAINT-001 §13);
nicht reproduzierbare Fehler bekommen einen dokumentierten Begründungs-Eintrag.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-080 | Regressionstest vor Fix rot / nach Fix gruen (soweit reproduzierbar) |
| REQ-ER-081 | Test in CI aufgenommen; nicht-reproduzierbare Faelle mit Begruendung dokumentiert |

## Implementierungsstatus
PARTIAL — Evidence existiert: atc-contracts-Suite fand GovernanceContract-Bug
(59/59 gruen, Issue #99); GovernanceContract.name()-Fix mit Test abgedeckt.

## Security Considerations

Regressionstests fuer Security-Fehler sind selbst Security-relevant: Testdaten nie mit echten Zugangsdaten; Fixture-Prinzip.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-ERR-000/-009/-011, ATC-STD-BUG-003 (Fix Lifecycle),
ATC-STD-REPO-MAINT-001 §13 (Build/Test-Gate) · INFORMATIVE: SCR-0044, Issue #99
