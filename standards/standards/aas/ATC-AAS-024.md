---
standard:
  id: ATC-AAS-024
  title: "ATC-AAS-024 — Agent-to-Agent Protocol Standard"
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

# ATC-AAS-024 — Agent-to-Agent Protocol Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-024 (Agent-to-Agent Protocol Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Nachrichtentypen

AGENT→TASK · AGENT→FINDING · AGENT→RESULT · AGENT→REQUEST ·
AGENT→HANDOFF · AGENT→BLOCKER · AGENT→REVIEW.

## 2. Nachrichtenstruktur (Pflichtfelder)

```yaml
message_id: A2A-000001         # fortlaufend, nie wiederverwendet (§37)
sender_agent: ATC-AI-DEV-001
receiver_agent: ATC-AI-TEST-001
task_id: ATC-TASK-00427
timestamp: "<ISO-8601>"
message_type: AGENT->RESULT
payload: {...}
evidence: [F-NNN / AUD-NNN / CI-Run]
priority: P0|P1|P2
```

## 3. Regeln

- Nachrichten sind persistente Artefakte (`.github/ai/messages/` je Repo,
  append-only) — kein flüchtiger Chat-Kanal.
- Jede Nachricht referenziert Evidenz (AAS-010) und Task (AI-DEV-004).
- Antwortpflicht: BLOCKER- und REQUEST-Nachrichten beantwortet der
  Empfänger mit RESULT oder BLOCKED — niemals Schweigen (Timeout = Eskalation
  an Owner, AI-DEV-011 §2).

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

1. Nachrichten zwischen Agenten MÜSSEN das definierte Protokollformat mit allen Pflichtfeldern einhalten.
2. Jede Nachricht MUSS eine eindeutige Message-ID, Sender-ID, Empfänger-ID und eine digitale Signatur enthalten.
3. Ungültige oder unvollständige Protokollnachrichten MÜSSEN vom Empfänger verworfen werden.
4. Die Kommunikation zwischen Agenten MUSS verschlüsselt und integritätsgeschützt erfolgen.
5. Nachrichten-Timeouts MÜSSEN definiert werden, um Deadlocks bei der Kommunikation zu verhindern.
6. Nachrichtenströme MÜSSEN auf Ratenbegrenzungen (Rate Limiting) überwacht werden.

## 5. Metriken & Akzeptanzkriterien

- 100% Protokoll-Konformität MUSS bei Inter-Agenten-Nachrichten durchgesetzt werden.
- Latenz der Nachrichtenverarbeitung SOLLTE unter 100ms liegen.
- 0 unverschlüsselte Nachrichten MÜSSEN im Netzwerkverkehr akzeptiert werden.
- Signaturfehler bei Nachrichten MÜSSEN zu 100% verworfen werden.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch Validator-Läufe (S-01 bis S-25) verifiziert werden. Mängel werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 erfasst. Protokollverstöße MÜSSEN korrigiert werden.

## 7. Security Considerations

Authentifizierte und verschlüsselte Protokolle verhindern Man-in-the-Middle-Angriffe und Spoofing zwischen Agenten. Ungesicherte Nachrichten MÜSSEN das System veranlassen, die Verbindung zu trennen.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-001, AI-DEV-004, AI-DEV-011, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
