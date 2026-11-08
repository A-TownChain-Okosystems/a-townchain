---
standard:
  id: ATC-STD-ERR-003
  title: "Root Cause Analysis & Pattern Extraction"
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

# ATC-STD-ERR-003 — Root Cause Analysis & Pattern Extraction (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0044). Familie FAM-47 (Error Propagation & Prevention, Range ATC-STD-ERR-000..999).
> **Prioritaet:** P0 · **Master:** ATC-STD-ERR-000 · **Abgrenzung:** BUG-001..005
> decken den Fehler-PROZESS ab; ERR deckt die systemische PROPAGATION ab.

## Zweck
Die eigentliche Ursache bestimmen. RCA-Methodik (4 Ebenen, Fault Tree, Evidence,
Timeline) kommt normativ aus ATC-STD-BUG-005 — ERR-003 ERGANZT die Pflicht zur
Muster-Extraktion: aus der Ursache MUSS ein wiederverwendbares Fehlermuster
abgeleitet werden.

## Kernregeln
- RCA nach BUG-005 ist Pflicht fuer P0/P1 (ERR-000 §3).
- Zusatzpflicht: root_cause MUSS ein `error_pattern` liefern (beschreibt die
  FehlerKLASSE, nicht den Einzelfall) — z.B. „Versionskompatibilitaetsangabe ohne
  Pruefschritt" statt „1.4.0 falsch gelistet".
- Das Muster speist die ERR-010-Pattern-Library und definiert die Scan-Query fuer
  ERR-004 (Muster-Suche, nicht Literal-Suche).

## REQ-Matrix

| REQ | Verpflichtung (MUST) |
|---|---|
| REQ-ER-030 | RCA gemaess BUG-005 fuer P0/P1-Fehler |
| REQ-ER-031 | error_pattern-Beschreibung ist Pflichtfeld (Klasse, nicht Einzelfall) |

## Implementierungsstatus
SPECIFIED — Umsetzung: error_pattern-Feld im BUG-005-Analyse-Template.

## Security Considerations

Root-Cause-Analysen duerfen keine Klartext-Zugangsdaten enthalten; Sicherheitsrelevante Ursachen (BUG-005-Klasse 18) erfordern Security-Scan im Post-Fix-Audit (ERR-012).

## Changelog
| 1.0.0 | 2026-09-08 | Initiale Fassung (SCR-0044) |

## References
NORMATIV: ATC-STD-BUG-005, ATC-STD-ERR-000/-004/-010 · INFORMATIVE: SCR-0044
