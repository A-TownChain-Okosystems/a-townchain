---
standard:
  id: ATC-AAS-019
  title: "ATC-AAS-019 — Agent Handoff Standard"
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

# ATC-AAS-019 — Agent Handoff Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-019 (Agent Handoff Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Handoff-Record (Pflichtstruktur)

```yaml
handoff:
  task_id: ATC-TASK-00427
  current_state: IN_PROGRESS
  completed: [ACT-001, ACT-002]
  remaining: [ACT-003, ACT-005]
  blockers: [F-009]
  decisions: [{observation, decision, reason}]   # AI-DEV-006
  modified_files: [src/parser.rs, tests/parser.rs]
  tests: {result: PASS, ci_run: "#1234"}          # AAS-010
  known_issues: []
  next_action: {id: ACT-003, priority: P1, depends_on: []}
```

## 2. Regeln

- Handoff nur bei übergabefähigem Task (AI-DEV-004 §5: Record vollständig,
  next_action aktuell, Annahmen registriert, history lückenlos).
- Übernehmender Agent startet DISCOVERING (AI-DEV-003), vermerkt
  `handover_from` (AI-DEV-012 §3) und bestätigt Übernahme im Task-Record.
- Handoff ist persistent (Artefakt-basiert, AI-DEV-012 §2) — keine
  Session-abhängige Übergabe.

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
## 3. Ergänzende Verbindliche Betriebs- und Governance-Regeln

1. Bei der Übergabe einer Aufgabe an einen anderen Agenten oder Menschen MUSS ein vollständiger Handoff-Record erstellt werden.
2. Der Handoff-Record MUSS den bisherigen Fortgang, offene Punkte, Blockaden und den aktuellen Kontext enthalten.
3. Der empfangende Agent MUSS den Handoff-Record validieren und die Übernahme im Audit-Trail bestätigen.
4. Unvollständige Handoff-Records MÜSSEN vom Empfänger zurückgewiesen werden.
5. Während des Handoff-Prozesses MÜSSEN schreibende Aktionen auf den betroffenen Ressourcen pausiert werden.
6. Handoff-Protokolle MÜSSEN in einem zentralen Register zur Einsichtnahme abgelegt werden.

## 4. Metriken & Akzeptanzkriterien

- 100% aller Task-Übergaben MÜSSEN über einen validen Handoff-Record verfügen.
- 0 Informationsverluste oder doppelte Task-Ausführungen MÜSSEN bei Übergaben auftreten.
- Übergabedauer MUSS unter 2 Sekunden liegen.
- Bestätigungen durch den Empfänger MÜSSEN in < 500ms vorliegen.

## 5. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS über Validator-Läufe (S-01 bis S-25) verifiziert werden. Verstöße werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 dokumentiert. Fehlerhafte Handoffs MÜSSEN korrigiert werden.

## 6. Security Considerations

Strukturierte Übergaben verhindern Kontextverlust und unbeabsichtigte Mehrfachausführungen kritischer Operationen. Nicht verifizierte Handoffs MÜSSEN blockiert werden.

## 7. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 8. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-003, AI-DEV-004, AI-DEV-006, AI-DEV-012, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
