---
standard:
  id: ATC-AAS-008
  title: "ATC-AAS-008 — Agent Workflow Standard"
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

# ATC-AAS-008 — Agent Workflow Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-008 (Agent Workflow Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Makro-Lifecycle (Pflichtreihenfolge)

```
DISCOVER → UNDERSTAND → PLAN → IMPLEMENT → TEST → AUDIT → DOCUMENT →
REVIEW → COMMIT → PULL REQUEST → HUMAN APPROVAL → MERGE
```

## 2. Mapping auf die State Machine (AI-DEV-001 §7)

DISCOVER/UNDERSTAND = DISCOVERING · PLAN/IMPLEMENT = IMPLEMENTING ·
TEST/VALIDIERUNG = TESTING/VALIDATING · AUDIT = AUDITING ·
COMMIT/PR/HUMAN APPROVAL/MERGE = Git-Gates aus AI-DEV-007 (§1-§6).
Rücksprünge: TEST→IMPLEMENT (FAIL), REVIEW→IMPLEMENT (CHANGES_REQUESTED),
jede Stufe → BLOCKED (ATC-AAS-020).

## 3. Stufenwechsel-Dokumentation

Jeder Stufenübergang wird im Task-Record dokumentiert (`history`-Eintrag
mit Ziel-Stufe, Zeitpunkt, Auslöser, AI-DEV-004 §2). Sprünge über Stufen
hinweg (z.B. IMPLEMENT → COMMIT ohne TEST) sind unzulässig; Rücksprünge
sind als solche zu kennzeichnen (Grund: Finding/FAIL/CHANGES_REQUESTED).

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

1. Der Makro-Lifecycle MUSS ausnahmslos in der vorgegebenen Phasenfolge durchlaufen werden; ein Phasensprung ist unzulässig.
2. Jeder Übergang zwischen Workflow-Stufen MUSS mit Zeitstempel und Grund im Audit-Trail protokolliert werden.
3. Vor dem Eintritt in die Phase `EXECUTION` MÜSSEN alle Vorbedingungen der Analyse- und Planungsphase erfüllt sein.
4. Tritt während eines Phasenübergangs ein Fehler auf, MUSS der Workflow kontrolliert in den Zustand `FAILED` schalten.
5. Workflows MÜSSEN mit einer maximalen Ausführungszeit (Timeout) versehen sein und bei Überschreiten beendet werden.
6. Manuelle Eingriffe in laufende Workflows MÜSSEN über autorisierte Befehle erfolgen und im Stufenprotokoll vermerkt werden.

## 5. Metriken & Akzeptanzkriterien

- 100% aller Workflow-Ausführungen MÜSSEN der strikten Phasenabfolge entsprechen.
- 0 unprotokollierte Stufenwechsel MÜSSEN in der State Machine zugelassen werden.
- Rollback-Einleitung bei Workflow-Fehlern MUSS in unter 1 Sekunde erfolgen.
- Workflow-Timeouts MÜSSEN nach Ablauf exakt greifen.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch Validator-Läufe (S-01 bis S-25) verifiziert werden. Verstöße werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 protokolliert. Workflow-Fehler MÜSSEN behoben werden.

## 7. Security Considerations

Strikte Workflow-Grenzen verhindern das unkontrollierte Ausführen von Code ohne vorherige Planungs- und Prüfschritte. Ungültige Zustandsübergänge MÜSSEN den Prozess stoppen.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-001, AI-DEV-004, AI-DEV-007, ATC-AAS-020, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
