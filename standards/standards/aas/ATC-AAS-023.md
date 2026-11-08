---
standard:
  id: ATC-AAS-023
  title: "ATC-AAS-023 — Agent Role Standard"
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

# ATC-AAS-023 — Agent Role Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-023 (Agent Role Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## 1. Rollenmodell (Least Privilege je Rolle)

```
ORCHESTRATOR (ShivaCore-Systemebene)
    ├── DEV-AGENT     — entwickelt (Scope: src/, tests/)
    ├── TEST-AGENT    — testet (Scope: tests/, CI)
    ├── SEC-AGENT     — prüft Sicherheit (read-all, write: Findings)
    ├── DOC-AGENT     — dokumentiert (Scope: docs/, wiki)
    └── AUDIT-AGENT   — auditiert (read-all, write: AUD/SYNC-Records)
            ↓
      HUMAN REVIEW (Merge/Deploy-Gates, AAS-017)
```

## 2. Regeln

- Jede Rolle ist ein eigener Agent mit eigenem Identity-Manifest (AAS-001),
  eigenen Capabilities (AAS-002) und Permissions (AAS-003) — niemals ein
  einziger Super-Agent mit All-Macht.
- 1-Agent-per-Repo (AI-DEV-012 §1) bleibt bestehen: pro Repo arbeitet zu
  jedem Zeitpunkt genau ein Agent; Orchestrierung verteilt Aufgaben, keine
  parallelen Schreibzugriffe.
- Orchestrierung kommuniziert über das A2A-Protokoll (AAS-024).

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

1. Jeder Agent MUSS genau einer vordefinierten Rolle (z.B. Developer, Auditor, Reviewer) zugewiesen sein.
2. Eine Kumulierung widersprüchlicher Rollen auf einen Agenten MUSS vermieden werden (Function Separation).
3. Rollenwechsel während des Betriebs MÜSSEN durch Re-Authentifizierung und Neu-Initialisierung abgesichert werden.
4. Jede Rolle MUSS eine minimalistische Rechtematrix definieren, die regelmäßig überprüft werden MUSS.
5. Die Erstellung benutzerdefinierter Rollen MUSS durch das Governance-Board genehmigt werden.
6. Rollenzuweisungen MÜSSEN im zentralen Manifest hinterlegt und schreibgeschützt verwaltet werden.

## 4. Metriken & Akzeptanzkriterien

- 100% Zuordnung von Agenten zu klar definierten Rollen MUSS nachgewiesen werden.
- 0 Rollen-Konflikte oder Mehrfachzuweisungen MÜSSEN ohne Gewaltentrennung zugelassen werden.
- Rollenvalidierung MUSS in < 10ms erfolgen.
- Rollenberechtigungen MÜSSEN zu 100% konform zur Matrix sein.

## 5. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS über Validator-Läufe (S-01 bis S-25) verifiziert werden. Abweichungen führen zu Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005. Rollenabweichungen MÜSSEN gemeldet werden.

## 6. Security Considerations

Klare Rollentrennung schützt das System vor Missbrauch durch Verhinderung von Interessenkonflikten in automatisierten Abläufen. Unbefugte Rollenübernahmen MÜSSEN verhindert werden.

## 7. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 8. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-012, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005
