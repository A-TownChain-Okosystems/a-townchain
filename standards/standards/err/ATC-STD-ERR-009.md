---
standard:
  id: ATC-STD-ERR-009
  title: "Preventive Control"
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

# ATC-STD-ERR-009 — Preventive Control (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Ein Regressionstest allein reicht nicht immer: Je Fehlerklasse MUSS eine zusaetzliche
Schutzmassnahme entstehen, die die Wiederholung strukturell verhindert.

## Kernregeln — Massnahmen-Matrix (Fehlerklasse → Praevention)

| Fehlerklasse | Praevention |
|---|---|
| falsche Version | automatischer Version Check |
| falsche Dependency | Dependency Validation (ATC-STD-204, Dependabot) |
| fehlende Datei | Repository Completeness Check (REPO-AUDIT-002) |
| falsches Schema | Schema Validator |
| API-Inkompatibilitaet | Compatibility Test |
| falsche Dokumentation | Documentation CI (Doc-Abgleich) |
| Security Bug | Security Regression Test |
| falsche Konfiguration | Config Validator |
| fehlende Migration | Migration Gate |
| falsche Standards-ID | Standards Validator (S-Checks) |

Die Matrix ist offen: neue Fehlerklassen erweitern sie via SCR. Fuer P0-Fehler ist
die Praevention Pflicht (ERR-000 §3); Ausnahmen nur mit Begruendung im Record.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-090 | Je Fehlerklasse (P0/P1) eine Praeventionsmassnahme aus der Matrix oder begruendet |

## Implementierungsstatus
SPECIFIED — Evidence-Anker: Version Check (Issue #96), Standards Validator
(S-01..S-25), Dependabot-Rollout — jeweils schon im Aufbau.

## Security Considerations

Praeventionsmassnahmen fuer Security-Fehlerklassen erfordern Security Regression Tests (Massnahmen-Matrix); Validator-Erweiterungen laufen unter CI ohne erweiterte Rechte.

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-ERR-000/-008/-010/-015, ATC-STD-BUG-005 (Corrective/Preventive) ·
INFORMATIVE: SCR-0044, Issues #95/#96
