---
title: "Error Records — Knowledge Base (ATC-STD-ERR-013)"
summary: "Dauerhafte Fehler-Wissensbasis der ATC-STD-ERR-Familie (SCR-0044): Error Knowledge Records ATC-ERR-NNNN mit Propagation-Scan, Root Cause, Regression-Test und Prävention."
----

# Error Records — Knowledge Base (ATC-STD-ERR-013)

Jeder wichtige Fehler (P0/P1) erhält hier einen Error Knowledge Record
(`ATC-ERR-NNNN.yaml`) gemäß ATC-STD-ERR-013 (REQ-ER-130/131).
Fehlerlebenszyklus: DETECTED → … → VERIFIED → CLOSED (ERR-000 §2) —
Sprünge von DETECTED nach CLOSED sind verboten.

## Pattern Library

[ATC Error Pattern Library (PATTERNS.yaml)](PATTERNS.yaml) — Startkatalog
ATC-ERR-PATTERN-001..008 (SCR-0044) + Erweiterungen über Fehlerfälle.

## Records

| error_id | Titel | Schwere | Pattern | Repos geprüft | Treffer | Status |
|---|---|---|---|---|---|---|
| [ATC-ERR-0001](ATC-ERR-0001.yaml) | GovernanceContract implementierte abstrakte BaseContract.name() nicht | P1 | 009 | 4 | 1 | CLOSED (08.09.2026) |
| [ATC-ERR-0002](ATC-ERR-0002.yaml) | Undeklarierte CI-Dependency: Workflows ohne Dependency-Installation | P1 | 010 | 26 | 27 | OPEN (08.09.2026, SCR-0054) |

## Metriken (ERR-014, je Wartungszyklus)

- Neue Fehler je Pattern: PATTERN-005 (Ursprung): 1 · PATTERN-009 (Wirkung): 1
- Recurrence-Rate: 0/1 (kein Wiederauftreten nach Fix)
- Präventionsversagen: 0
