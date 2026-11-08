---
standard:
  id: ATC-STD-ERR-007
  title: "Dependency Impact Analysis"
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

# ATC-STD-ERR-007 — Dependency Impact Analysis (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Jeder Fehler MUSS auf abhaengige Systeme untersucht werden. Kette: Library A →
Protocol B → Service C → API D → Frontend E — B bis E duerfen nicht ungeprueft
bleiben, wenn in A ein Fehler gefunden wurde.

## Kernregeln
Pruefstatus je abhaengigem System: **DIRECT** (betroffen), **INDIRECT**
(weitergereicht), **NOT AFFECTED** (geprueft, sauber), **UNKNOWN** (nicht pruefbar).
**UNKNOWN DARF NICHT als „kein Problem" interpretiert werden** — UNKNOWN erfordert
Follow-up und blockiert den Abschluss (ERR-000 REQ-ER-008).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-070 | Je abhaengigem System ein Status DIRECT/INDIRECT/NOT AFFECTED/UNKNOWN |
| REQ-ER-071 | UNKNOWN blockiert Closure bis Follow-up; UNKNOWN != OK |

## Implementierungsstatus
SPECIFIED — Umsetzung: Impact-Tabelle im Knowledge Record; Dependency-Graph aus
ATC-STD-202 (Repository-Rollen/-Domaenen).

## Security Considerations

Impact-Status UNKNOWN blockiert Abschluss; Security-Komponenten im Impact erhalten automatisch Security-Review-Pflicht (ERR-008).

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-ERR-000, ATC-STD-204 (Dependency), ATC-STD-202 (Rollen) ·
INFORMATIVE: SCR-0044
