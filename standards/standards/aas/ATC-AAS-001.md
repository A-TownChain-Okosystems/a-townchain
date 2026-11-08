---
standard:
  id: ATC-AAS-001
  title: "ATC-AAS-001 — Agent Identity Standard"
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

# ATC-AAS-001 — Agent Identity Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Block:** ATC-AAS (AI Agent Standards) · **Priorität:** P0 · **Erweitert:** ATC-STD-AI-DEV-001 §2 (Agent-Manifest)

## Abstract

ATC-AAS-001 (Agent Identity Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Identitäts-Pflichtstruktur

Jeder KI-Agent ist eindeutig identifizierbar. Kanonische Ablage:
`.github/ai/agent.yaml` (Ablageort gemäß AI-DEV-001 §6).

```yaml
agent_id: ATC-AI-DEV-001          # Namensraum AI-DEV-001 §2: ATC-AI-{ROLE}-NNN
agent_name: ATC Development Agent
agent_type: software-development
agent_version: 1.0.0              # Versionierung nach ATC-AAS-022
owner: A-TownChain-Okosystems
repository_scope: []              # konkretisiert durch ATC-AAS-004
capabilities: []                  # konkrete Kategorien: ATC-AAS-002
permissions: {}                   # Operationen: ATC-AAS-003
environment: {runtime, network, sandbox}   # konkrete Kategorien: ATC-AAS-014
status: ACTIVE|RETIRED
```

## 2. Regeln

- Agenten ohne gültiges Identity-Manifest gelten als nicht vorhanden
  (kein Zugang, keine Commits, keine Tasks).
- `agent_id` folgt AI-DEV-001 §2 und naming-conventions (`aiAgentId`).
- Ein Agent ist immer genau EINEM Identity-Manifest zugeordnet; Session-
  Kontext darf die Identität nicht ändern.

## 3. Verhältnis zu AI-DEV-001

AAS-001 konkretisiert die abstrakte Manifest-Pflicht aus AI-DEV-001 §2 um
die Felder `environment` und `status`; kein Widerspruch — AI-DEV-001 bleibt
Dach-Norm der Identität.

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

NORMATIV: ATC-STD-000, ATC-STD-201..203, ATC-AAS-002 · INFORMATIVE: AGENT_MANIFEST.md v3.1.7, Registry-Kategorie aas
## 4. Ergänzende Verbindliche Betriebs- und Governance-Regeln

1. Das Identity-Manifest (`.github/ai/agent.yaml`) MUSS bei jedem Agenten-Start vor der Ausführung von Aktionen auf Schema-Konformität verifiziert werden.
2. Eine Änderung der `agent_id` während einer aktiven Sitzung MUSS vom Laufzeitsystem zwingend blockiert werden; die Identität ist unveränderlich.
3. Identitäts-Zertifikate und Verifizierungsschlüssel MÜSSEN im Abstand von maximal 90 Tagen erneuert und rotiert werden; abgelaufene Schlüssel MÜSSEN ungültig werden.
4. Temporäre Sub-Agenten MÜSSEN ihren übergeordneten Hauptagenten als Parent-ID im Manifest deklarieren und sich gegenüber dem Gesamtsystem ausweisen.
5. Agenten MÜSSEN jede Identitätsanfrage unverzüglich mit der signierten `agent_id` und dem zugehörigen Manifest-Hash beantworten.
6. Bei Verlust oder Kompromittierung des Identitätsnachweises MUSS der betroffene Agent unverzüglich deaktiviert und der Status auf RETIRED gesetzt werden.

## 5. Metriken & Akzeptanzkriterien

- 100% der aktiven KI-Agenten MÜSSEN über ein valides, nicht abgelaufenes Identity-Manifest verfügen.
- Die Verifikation der Agenten-Identität MUSS in unter 50 Millisekunden abgeschlossen sein.
- Fehlgeschlagene Identitäts-Prüfungen MÜSSEN zu 100% ein Audit-Event auslösen und die Sitzung stoppen.
- Abgewiesene Identifikationsversuche MÜSSEN innerhalb von 1 Sekunde im Sicherheits-Monitor registriert werden.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich verifiziert werden. Festgestellte Abweichungen werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert. Bei kritischen Abweichungen MUSS die Agentenausführung unverzüglich unterbrochen werden.

## 7. Security Considerations

Identitätsnachweise und Schlüsselmaterial MÜSSEN vor unbefugtem Lese- und Schreibzugriff geschützt aufbewahrt werden. Impersonation oder Identitätsdiebstahl durch andere Prozesse MUSS durch kryptographische Bindung verhindert werden. Sämtliche Identitätsänderungen MÜSSEN revisionssicher nachvollziehbar sein.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-001, ATC-AAS-002, ATC-AAS-003, ATC-AAS-004, ATC-AAS-014, ATC-AAS-022, ATC-ENT-002, ATC-STD-000, ATC-STD-AI-DEV-001, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
