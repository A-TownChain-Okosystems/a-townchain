---
standard:
  id: ATC-AAS-003
  title: "ATC-AAS-003 — Agent Permission Standard"
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

# ATC-AAS-003 — Agent Permission Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-003 (Agent Permission Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. Berechtigungsstufen

READ · WRITE · EXECUTE · MERGE · DEPLOY · ADMIN. Jede Stufe ist separat
zu erteilen; keine implizite Stufenhierarchie (WRITE beinhaltet NICHT EXECUTE).

## 2. Operationsmatrix (Beispiel Entwicklungsagent)

READ ✓ · WRITE ✓ · TEST ✓ · COMMIT ✓ · PUSH ✓ · PR ✓ ·
MERGE ✗ · DEPLOY ✗ · ADMIN ✗

→ Der Agent entwickelt selbstständig, kann aber keinen Mainnet-Code deployen
und nicht eigenständig mergen (Merge-Gate: AI-DEV-007 §6).

## 3. Eskalation

Merge/Deploy/ADMIN nur per dokumentierter Owner-Freigabe mit Befristung
(AI-DEV-002 §2); nach Ablauf automatische Rückstufung.

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

1. Berechtigungen MÜSSEN nach dem Prinzip der geringsten Rechte (Least Privilege) streng bedarfsgerecht vergeben werden.
2. Eine Weitergabe, Vererbung oder Delegation erteilter Berechtigungen an andere Agenten MUSS strikt verboten werden.
3. Temporäre Rechte-Eskalationen MÜSSEN automatisch nach Ablauf des definierten Zeitfensters (maximal 4 Stunden) widerrufen werden.
4. Jede Berechtigungsprüfung MUSS vor der Ausführung der Zieloperation atomar und idempotent durchlaufen.
5. Ein Entzug von Berechtigungen MUSS unverzüglich und ohne Verzögerung für alle laufenden Agenten-Sitzungen wirksam werden.
6. Sämtliche Berechtigungsentscheidungen MÜSSEN auditierbar protokolliert und gegen Modifikationen geschützt werden.

## 5. Metriken & Akzeptanzkriterien

- 100% aller privilegierten Operationen MÜSSEN eine gültige Berechtigung vorweisen.
- 0 verbleibende abgelaufene temporäre Rechte-Eskalationen MÜSSEN im System existieren.
- Berechtigungsprüfungen MÜSSEN eine Erfolgsquote von 100% präziser Auswertung aufweisen.
- Rechte-Widerrufe MÜSSEN in unter 100ms systemweit in Kraft treten.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontrolliert werden. Abweichungen führen zu Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005. Unbefugte Privilegienerweiterungen MÜSSEN sofort eskaliert werden.

## 7. Security Considerations

Unberechtigte Rechteausweitung stellt ein kritisches Sicherheitsrisiko dar. Das System MUSS sicherstellen, dass Berechtigungsentscheidungen serverseitig und fälschungssicher evaluiert werden. Bei Privilegienverlust MUSS die Session beendet werden.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-002, AI-DEV-007, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
