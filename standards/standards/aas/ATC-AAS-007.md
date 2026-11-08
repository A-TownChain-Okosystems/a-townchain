---
standard:
  id: ATC-AAS-007
  title: "ATC-AAS-007 — Agent Task Standard"
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

# ATC-AAS-007 — Agent Task Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-007 (Agent Task Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Feldstruktur (Zusatzfelder zu AI-DEV-004 §1)

```yaml
task_id: ATC-TASK-00427
title: ...
objective: ...                      # messbares Ziel
repository: ...                     # Ziel-Repo (muss im Scope, ATC-AAS-004)
requirements: [REQ-...]             # Verweis auf Anforderungsquellen
dependencies: [ATC-TASK-NNNN]
allowed_files: []                   # Verfeinerung von ATC-AAS-004
forbidden_files: []
acceptance_criteria: []             # prüfbare Kriterien
test_requirements: {}                # Mindest-Validierung, ATC-AAS-011
documentation_requirements: {}      # Sync-Pflicht, AI-DEV-010
status: <AI-DEV-004 §2 Lifecycle>
```

## 2. Regeln

- Task-ID-Format und Allokation unverändert AI-DEV-004 §3 (fortlaufend,
  nie wiederverwendet, ATC-STD-000 §37).
- Datei-Arbeitsbereich einer Task ist die Schnittmenge aus Scope
  (AAS-004) und allowed_files; forbidden_files hat Vorrang.
- COMPLETED nur bei erfüllten acceptance_criteria UND grüner Validierung
  UND Audit-Record (AI-DEV-009).

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

1. Tasks MÜSSEN alle geforderten Pflichtfelder aus AI-DEV-004 §1 sowie die spezifischen Zusatzfelder vollständig aufweisen.
2. Vor dem Beginn der Arbeit an einem Task MUSS der Agent den Task-Status im System auf `IN_PROGRESS` setzen.
3. Unvollständige, mehrdeutige oder widersprüchliche Task-Spezifikationen MÜSSEN vor der Bearbeitung abgelehnt werden.
4. Mehrfachbearbeitungen desselben Tasks durch verschiedene Agenten MÜSSEN durch Locking koordiniert werden.
5. Nach Abschluss MUSS ein strukturierter Task-Abschlussbericht mit allen erzeugten Artefakten erstellt werden.
6. Task-Abbrüche MÜSSEN mit einer klaren Fehlerursache im System dokumentiert und im Audit-Trail hinterlegt werden.

## 4. Metriken & Akzeptanzkriterien

- 100% aller bearbeiteten Tasks MÜSSEN das vollständige Pflichtfeldschema erfüllen.
- 0 abgebrochene Tasks ohne dokumentierte Statusänderung MÜSSEN akzeptiert werden.
- Task-Statusaktualisierungen MÜSSEN in < 100ms im System reflektiert werden.
- Locking-Konflikte MÜSSEN in unter 50ms aufgelöst werden.

## 5. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) überwacht werden. Fehler führen zu Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005. Mangelhafte Tasks MÜSSEN überarbeitet werden.

## 6. Security Considerations

Das Manipulieren von Task-Parametern MUSS verhindert werden. Task-Payloads MÜSSEN vor der Ausführung auf böswillige Befehle geprüft werden. Nicht autorisierte Task-Änderungen MÜSSEN verworfen werden.

## 7. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 8. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-004, AI-DEV-009, AI-DEV-010, ATC-AAS-004, ATC-AAS-011, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
