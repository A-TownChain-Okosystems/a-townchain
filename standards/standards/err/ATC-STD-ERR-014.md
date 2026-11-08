---
standard:
  id: ATC-STD-ERR-014
  title: "Recurrence Monitoring"
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

# ATC-STD-ERR-014 — Recurrence Monitoring (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P1 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Wiederauftreten ueberwachen: Ein geschlossener Fehler, der erneut auftritt, ist ein
Governance-Finding (Praevention versagt oder wurde nicht gebaut).

## Kernregeln
- Je Pattern: Recurrence-Zaehler und Rate; erneutes Auftreten eines CLOSED-Fehlers
  wird als neuer Fehler MIT Verweis auf den alten Record und Pattern erfasst.
- Wiederholung nach Praeventions-Bau = P0-Befund „Praeventionsversagen"
  (ERR-009-Massnahme pruefen und verschaerfen).
- Metriken je Wartungszyklus (REPO-MAINT-001 §16): neue Fehler je Pattern,
  Recurrence-Rate, Pruef-Abdeckung.

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-140 | Recurrence je Pattern wird gezaehlt und im Maintenance-Report gefuehrt |
| REQ-ER-141 | Wiederholung nach Praeventions-Bau = P0-Befund Praeventionsversagen |

## Implementierungsstatus
SPECIFIED — Umsetzung: Feld im Knowledge Record + Maintenance-Report-Metrik.

## Security Considerations

Recurrence-Monitoring ueberwacht Praeventionsversagen; Security-Recurrence = sofortiges P0 (BUG-001).

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-ERR-009/-010/-013, ATC-STD-REPO-MAINT-001 §16 · INFORMATIVE: SCR-0044
