---
standard:
  id: ATC-AAS-018
  title: "ATC-AAS-018 — Agent Audit Trail Standard"
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

# ATC-AAS-018 — Agent Audit Trail Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-018 (Agent Audit Trail Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Aktions-Protokollierung

Jede Agenten-Aktion (nicht nur Task-Abschluss) wird protokolliert:

```
timestamp · agent_id · task_id · repository · action · command ·
files_changed · result · risk_level · human_approval
```

## 2. Zweistufiges Modell

- **Aktionsprotokoll** (feingranular, je Operation): maschinenlesbar, in
  `.github/ai/audit/` als Event-Stream je Task
  (`.github/ai/audit/ATC-TASK-NNNN.events.yaml`), append-only.
- **Abschluss-Record AUD-NNN** (AI-DEV-009 §1, unverändert): Zusammenfassung
  bei COMPLETED/CANCELLED/BLOCKED/HANDOVER.

## 3. Regeln

- Beide Ebenen append-only; Korrektur nur per corrects:-Folgercord.
- risk_level: S0-S4 (ATC-STD-BUG-002); S0/S1-Aktionen triggieren
  Human-Approval-Pflicht (AAS-017 §2).
- Anschluss an LogChain/AuditTrail/DefenderGPT-Konzepte erfolgt über die
  Produkt-Repositories (atc-indexer, aurora-ai); dieser Standard definiert
  nur das Protokollformat.

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

1. Sämtliche Aktionen des Agenten MÜSSEN fälschungssicher im zweistufigen Audit-Log protokolliert werden.
2. Audit-Einträge MÜSSEN den Zeitstempel (UTC), die `agent_id`, den Kontext sowie den Aktions-Hash enthalten.
3. Die Integrität des Audit-Trails MUSS durch fortlaufende Hash-Ketten gesichert werden.
4. Audit-Logs MÜSSEN mindestens 365 Tage revisionssicher aufbewahrt werden.
5. Schreibfehler im Audit-System MÜSSEN zum sofortigen Stopp weiterer Agenten-Aktionen führen.
6. Audit-Logs MÜSSEN regelmäßig auf Vollständigkeit und unerlaubte Modifikationen gescannt werden.

## 5. Metriken & Akzeptanzkriterien

- 100% Lückenlosigkeit im Audit-Trail MUSS durch das System nachgewiesen werden (0 fehlende Log-Einträge).
- 100% valide Hash-Ketten MÜSSEN im Audit-Protokoll vorhanden sein.
- Latenz beim Schreiben von Audit-Logs MUSS unter 15ms liegen.
- Log-Schreibfehler MÜSSEN in < 1 Sekunde zum Ausführungsstopp führen.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch Validator-Läufe (S-01 bis S-25) verifiziert werden. Fehler werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 gemeldet. Log-Lücken MÜSSEN sofort geschlossen werden.

## 7. Security Considerations

Revisionssichere Audit-Trails ermöglichen die lückenlose Nachvollziehbarkeit aller Aktionen im Sicherheits- und Incident-Fall. Manipulationen am Audit-Trail MÜSSEN einen Alarm auslösen.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-009, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
