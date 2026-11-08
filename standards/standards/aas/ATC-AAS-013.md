---
standard:
  id: ATC-AAS-013
  title: "ATC-AAS-013 — Agent Conflict Resolution Standard"
  version: "1.1.0"
  status: approved
  category: aas
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-AAS-013 — Agent Conflict Resolution Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-013 (Agent Conflict Resolution Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Konflikt-Leiter (Welche Quelle gewinnt)

```
Standard > Architecture > Specification > Requirement > Issue > Implementation
```

(konsistent mit der Wahrheits-Priorität, ATC-AAS-006 §2)

## 2. Verfahren

1. Konflikt erkennen (zwei Quellen widersprechen sich in einer Aussage).
2. Konflikt DARF NICHT stillschweigend gelöst werden.
3. Finding F-NNN anlegen (AI-DEV-005; Quelle A vs. B, Fundstellen) — bei
   Standard-Widerspruch zusätzlich SCR-XXXX (ATC-STD-000 §19-§33).
4. Eskalation an Owner, wenn das Finding eine normative Ebene betrifft
   (AI-DEV-011 §2).
5. Arbeit an betroffener Task → BLOCKED bis Konflikt entschieden.

## 3. Short-Circuit

Bei rein inner-tasklichen Widersprüchen (Implementation vs. Issue-Text)
ohne normative Wirkung: dokumentierte Entscheidung nach AI-DEV-006
(observation/decision/reason) ausreichend — kein Finding nötig.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Agenten-spezifisch: Agent-Identitaet via AGENT_MANIFEST verifizierbar; Permissions nach Least-Privilege; Delegationen dokumentiert und widerrufbar; Zugangsdaten ausschliesslich als $ENV-Platzhalter (ATC-STD-203); Agent-Kommunikation authentifiziert, nie anonym.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203, ATC-AAS-001 · INFORMATIVE: AGENT_MANIFEST.md v3.1.7, Registry-Kategorie aas
## 4. Ergänzende Verbindliche Betriebs- und Governance-Regeln

1. Bei widersprüchlichen Vorgaben MUSS die Auflösung streng anhand der vorgegebenen Konflikt-Leiter erfolgen.
2. Kann ein Konflikt nicht automatisch gelöst werden, MUSS der Agent sofort den Short-Circuit-Mechanismus auslösen.
3. Es MUSS ein Konflikt-Protokoll mit allen beteiligten Quellen und der getroffenen Auflösungsentscheidung erstellt werden.
4. Agenten DÜRFEN KEINE eigenmächtigen Kompromisse eingehen, die einer höherrangigen Vorgabe widersprechen.
5. Wiederkehrende Konflikte MÜSSEN zur Klärung an das Governance-Board gemeldet werden.
6. Konflikt-Entscheidungen MÜSSEN unverzüglich an abhängige Sub-Systeme weitergeleitet werden.

## 5. Metriken & Akzeptanzkriterien

- 100% aller erkannten Regelkonflikte MÜSSEN regelkonform nach der Konflikt-Leiter aufgelöst werden.
- Reaktionszeit für die Auslösung des Short-Circuit MUSS unter 1 Sekunde liegen.
- 0 ungelöste Konfigurationskonflikte MÜSSEN bei der Ausführung verbleiben.
- Protokollierung von Konflikten MUSS zu 100% lückenlos erfolgen.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch Validator-Läufe (S-01 bis S-25) kontrolliert werden. Mängel werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 gemeldet. Konfliktfehler MÜSSEN korrigiert werden.

## 7. Security Considerations

Die eindeutige Priorisierung verhindert Sicherheitslücken, die durch widersprüchliche Sicherheitsrichtlinien entstehen könnten. Höherrangige Sicherheitsregeln MÜSSEN stets gewinnen.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-005, AI-DEV-006, AI-DEV-011, ATC-AAS-006, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
