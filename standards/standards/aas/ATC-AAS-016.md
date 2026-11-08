---
standard:
  id: ATC-AAS-016
  title: "ATC-AAS-016 — Agent PR Standard"
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

# ATC-AAS-016 — Agent PR Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-016 (Agent PR Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: alle KI-Agenten der A-TownChain-Organisation und deren Betrieb in sämtlichen R2+-Repositories (Identität bis Repository-Manifest).
Nicht-Gilt: menschliche Rollen (ATC-ENT-002); Mainnet-Betrieb (ATC-STD-NET).

## 1. KI-Kennzeichnung (Pflichtblock am PR-Anfang)

```yaml
created_by:
  type: ai_agent
  agent_id: ATC-AI-DEV-001
  agent_version: 1.2.0
```

## 2. Pflichtabschnitte (Erweiterung der 8 Abschnitte aus AI-DEV-007 §4)

Ziel · Änderungen · betroffene Dateien · Tests (Evidenzblock AAS-010) ·
Risiken · Breaking Changes · Dokumentationsstatus · Security-Auswirkungen ·
Rollback-Informationen (AAS-009 §1).

## 3. Merge-Gate (unverändert)

AI-DEV-007 §6: Struktur + grüner CI-Run + Task-Status READY_FOR_REVIEW +
Human-Approval, falls gefordert. `Human Decision Required: Yes` zwingt zur
dokumentierten Owner-Entscheidung (AI-DEV-011 §3).

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

1. Jeder Pull Request eines KI-Agenten MUSS den normierten KI-Kennzeichnungsblock am PR-Anfang enthalten.
2. Alle geforderten Pflichtabschnitte MÜSSEN im PR-Body vollständig ausgefüllt vorliegen.
3. PRs ohne bestandenes Merge-Gate MÜSSEN im Status `DRAFT` verbleiben und DÜRFEN NICHT gemergt werden.
4. Änderungen an PR-Beschreibungen MÜSSEN auditierbar in der Revisionshistorie nachvollziehbar bleiben.
5. Der Agent MUSS nach Erstellen des PRs automatisierte CI-Checks abwarten und Fehlschläge analysieren.
6. Automatisch generierte PR-Inhalte MÜSSEN vor der Einreichung auf Syntax und Formatierung geprüft werden.

## 5. Metriken & Akzeptanzkriterien

- 100% der Agenten-PRs MÜSSEN die geforderte KI-Kennzeichnung tragen.
- 0 PR-Merges ohne Erfüllung aller 8 Pflichtabschnitte MÜSSEN stattfinden.
- PR-Validierungsdauer MUSS unter 1 Minute liegen.
- Draft-Status MUSS bei nicht bestandenen Checks zu 100% beibehalten werden.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS durch Validator-Läufe (S-01 bis S-25) verifiziert werden. Mängel werden als Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 erfasst. PR-Verstöße MÜSSEN behoben werden.

## 7. Security Considerations

Die PR-Kennzeichnung und Pflichtabschnitte stellen die menschliche Überprüfbarkeit von automatisierten Codeänderungen sicher. Manipulierte PR-Beschreibungen MÜSSEN gemeldet werden.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-001, AI-DEV-007, AI-DEV-011, ATC-ENT-002, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005, ATC-STD-NET
