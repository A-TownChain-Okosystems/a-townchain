---
standard:
  id: ATC-AAS-020
  title: "ATC-AAS-020 — Agent Failure Standard"
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

# ATC-AAS-020 — Agent Failure Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-020 (Agent Failure Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Fehlerzustände

```
FAILED             — Ausführung fehlgeschlagen
BLOCKED            — extern/philosophisch blockiert (Finding, Konflikt, Frist)
NEEDS-HUMAN        — Owner-Entscheidung zwingend (AI-DEV-011 §2)
ROLLBACK-REQUIRED  — Änderung hat System instabil gemacht (AAS-009 Rollback)
```

## 2. Schleifenvermeidung (Pflicht)

```yaml
max_attempts: 3
after_3_failures:
  - create_finding      # F-NNN mit Evidenz (AI-DEV-005)
  - stop_task           # Status BLOCKED, keine Wiederholung
  - request_human_review
```

Keine Endlosschleifen: Jeder Wiederholungsversuch ist im Task-Record
protokolliert; nach `max_attempts` ist Autonomie für diese Task beendet.

## 3. ROLLBACK-REQUIRED

Sofortige Eskalation (S1), Ausführung des dokumentierten Rollbacks
(AAS-009 §1 ROLLBACK) nach Owner- oder Rollback-Standard-Freigabe,
Finding mit Ursachenanalyse (AI-DEV-006: observation/decision/reason).

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

1. Tritt ein unbehebbarer Fehler auf, MUSS der Agent sofort den Zustand `ROLLBACK-REQUIRED` oder `FAILED` einnehmen.
2. Wiederholte Fehlversuche MÜSSEN durch einen Circuit Breaker nach maximal 3 Versuchen unterbrochen werden.
3. Bei Auslösung eines Rollbacks MÜSSEN alle seit dem letzten stabilen Zustand getätigten Änderungen rückgängig gemacht werden.
4. Fehlerursachen MÜSSEN in einer strukturierten Root-Cause-Analyse (RCA) protokolliert werden.
5. Nach Eintritt eines kritischen Fehlers MUSS eine sofortige Benachrichtigung an das Überwachungssystem erfolgen.
6. Rollback-Prozeduren MÜSSEN automatisiert getestet und im Notfallplan hinterlegt sein.

## 5. Metriken & Akzeptanzkriterien

- 100% erfolgreiche Rollback-Ausführungen MÜSSEN im Fehlerfall garantiert werden.
- 0 unendliche Wiederholungsschleifen MÜSSEN bei fehlgeschlagenen Aktionen vorkommen.
- Circuit-Breaker-Auslösezeit MUSS unter 100ms liegen.
- RCA-Berichte MÜSSEN innerhalb von 5 Minuten generiert sein.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch Validator-Läufe (S-01 bis S-25) überprüft werden. Abweichungen werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert. Fehlerzustände MÜSSEN dokumentiert werden.

## 7. Security Considerations

Automatische Rollbacks verhindern, dass das System in einem inkonsistenten oder unsicheren Zwischenzustand verbleibt. Fehlgeschlagene Rollbacks MÜSSEN den Notfall-Killswitch aktivieren.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-005, AI-DEV-006, AI-DEV-011, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
