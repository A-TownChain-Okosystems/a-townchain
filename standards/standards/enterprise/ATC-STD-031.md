---
standard:
  id: ATC-STD-031
  title: "Repository Health Score Standard"
  version: "1.0.0"
  status: approved
  category: enterprise
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-11"
  updated: "2026-09-11"
  normative: true
  effective_date: null
  applies_to: "Alle ATC-Repositories und die Organisation gesamt"
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-STD-018
----

# ATC-STD-031 — Repository Health Score Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0) — Owner-Entwurf Automated Assurance 11.09.2026
> (SCR-0095; Owner-Referenz s. Kopf-Mapping). Bis zur §9-Freigabe nicht wirksam.
> **Scope:** ATC-STD-031 · maschinenberechneter Gesundheitszustand je Repository · **Governance:** ATC-STD-000
> **ID-Mapping:** Owner-Referenz "ATC-STD-029" — Slot belegt (Threat Modeling,
> SCR-0094), §37 ergab 031.

## Abstract

ATC-STD-031 definiert den maschinenberechneten Repository Health Score über
neun Dimensionen mit verbindlichen Bändern. Kernregel: Der Score DARF
Security-Gates NICHT überstimmen — ein Repository mit Score 98 und einer
ungepatchten kritischen RCE bleibt blockiert.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF — RFC 2119 gemäß ATC-STD-000 §10.

## Scope

Gilt für alle governed Repositories. Score ist Management-Sicht; Release-
Entscheidung trifft ausschließlich ATC-GATE-SEC-001.

## §1 Dimensionen (REQ-STD-001, MUST)

Der Health Score MUSS aus neun Dimensionen berechnet werden: Security,
Reliability, Technology Currency, Code Quality, Documentation, Testing,
Supply Chain, Governance, Maintainability. Dimensionen MÜSSEN aus
Assurance-Scanner-Daten (ATC-STD-030 §1) gespeist werden.

## §2 Bänder (REQ-STD-002, MUST)

| Band | Score |
|---|---|
| EXCELLENT | 95–100 |
| HEALTHY | 85–94 |
| ACCEPTABLE | 70–84 |
| DEGRADED | 50–69 |
| CRITICAL | < 50 |

## §3 Gate-Vorrang (REQ-STD-003, MUST)

Der Health Score MUSS NICHT Security-Gates ersetzen oder überstimmen:
offene P0/P1-Security-Findings blockieren unabhängig vom Score
(ATC-STD-018 §8-Prinzip, ATC-GATE-SEC-001 §2). Score-Verbesserung DARF
NICHT als Kompensation für offene Findings verwendet werden.

## REQ-Matrix (normative Anforderungen)

- id: REQ-STD-001 — Dimensionen: 9 Dimensionen MÜSSEN aus Scanner-Daten berechnet werden.
- id: REQ-STD-002 — Bänder: 5 Bänder MÜSSEN einheitlich verwendet werden.
- id: REQ-STD-003 — Gate-Vorrang: Score MUSS NICHT Gates überstimmen; offene P0/P1 blockieren.

## Compliance

Prüfung: Score-Berechnung je Review-Zyklus (ATC-STD-030 §3), Ablage in den
Statusfeldern (ATC-STD-018 §11) bzw. Org-Audit-Bericht.

## Security Considerations

- Score-Gaming (gezielte Dimensionsoptimierung bei offenen Findings) MUSS
  über Gate-Vorrang (§3) ausgeschlossen sein.
- Score IST Momentaufnahme: Zeitstempel pflichtig.

## Implementierungsstatus

**Status: SPECIFIED** — Berechnung mit Assurance-Engine-Rollout. SSOT:
registry/standard-implementation.yaml.

## Changelog

- v1.0.0 (2026-09-11): Owner-Entwurf Automated Assurance (SCR-0095) — 9
  Dimensionen, 5 Bänder, Gate-Vorrang-Regel. CANDIDATE.

## References

- ATC-STD-030 — Scanner (Datenquelle) · ATC-STD-018 §8 — Score-Prinzip
- ATC-GATE-SEC-001 — Release-Entscheidung
