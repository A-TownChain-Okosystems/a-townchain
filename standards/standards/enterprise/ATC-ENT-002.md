---
standard:
  id: ATC-ENT-002
  title: "ATC-ENT-002 — Rollen & Verantwortlichkeiten Standard"
  version: "1.1.0"
  status: approved
  category: enterprise
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

# ATC-ENT-002 — Rollen & Verantwortlichkeiten Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-002 (Rollen & Verantwortlichkeiten Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Rollen-Definition (Pflichtstruktur)

```yaml
role:
  id: ROLE-DEV
  name: "..."
  purpose: "..."
  responsibilities: [...]
  authority: [...]
  limitations: [...]
  required_approvals: [...]
  audit_required: true
```

## 2. Kanonische Rollen (Initialbestand)

ROLE-CEO · ROLE-CTO · ROLE-CISO · ROLE-ARCH · ROLE-DEV · ROLE-QA ·
ROLE-SEC · ROLE-DEVOPS · ROLE-AI-ENGINEER · ROLE-AI-AGENT · ROLE-AUDITOR.

Mapping zur Verfassung: Owner = ROLE-CEO/CTO-Ebene; Approver ausschließlich
menschliche Rollen (ATC-STD-000 §14.1: Agenten sind NIE Approver); Agenten
tragen ALWAYS `audit_required: true`.

## 3. KI-Agenten als Unternehmensmitglieder

Agenten sind kontrollierte technische Entitäten innerhalb der
Unternehmens-Governance — identifiziert über ATC-AAS-001 (agent_id),
berechtigt über AAS-002/003, eingebunden als ROLE-AI-AGENT mit denselben
Pflichten wie menschliche Rollen: Verantwortlicher, Status, Begründung,
Historie (ENT-003 Kernregel), Audit-Pflicht (ENT-014). KEINE separaten
parallelen Standards (statt eines eigenen „ATC-AI-001" gelten
AI-DEV-001..012 + AAS-001..025 unverändert).

## 4. Regeln

- Neue Rollen nur über ENT-010 (Change) mit Registry-Eintrag; IDs nach
  Schema `roleId` (^ROLE-[A-Z][A-Z0-9-]*$), fortlaufend (§37).
- Eine Person/ein Agent kann mehrere Rollen tragen — Ausübung je Rolle
  getrennt dokumentieren; Approver-Rolle nie durch Agenten (§14.1).
- Rollen-Reviews im review_cycle (ENT-001 §3): annual.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Enterprise-Transaktionen: Buchungs- und Audit-Trails unveraenderbar (append-only); Betrugsschutz-Mechanismen bei Zahlungswegen; keine Zugangsdaten in Geschaeftsdaten; Transaktions-Integritaet vor und nach Konsens-Teilnahme gewaehrleistet.

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-280ff (Security-Familie), ATC-ENT-001 · INFORMATIVE: Registry-Kategorie enterprise
## 5. Verbindliche Rollen- und Verantwortlichkeitsregeln

1. Neue Rollen MUSS über den Change-Prozess gemäß ATC-ENT-010 beantragt und in der zentralen Rollen-Registry erfasst werden.
2. Jede Rolle MUSS eindeutig definierte Zuständigkeiten (responsibilities), Befugnisse (authority) und Einschränkungen (limitations) enthalten.
3. KI-Agenten, die als ROLE-AI-AGENT agieren, MUSS zwingend an eine eindeutige agent_id gemäß ATC-AAS-001 gebunden sein.
4. Bei Rollenwechseln oder Abberufungen MUSS die zugewiesenen Berechtigungen unverzüglich entzogen und im Audit-Log gemäß ATC-ENT-014 dokumentiert werden.
5. Rollen-Kumulationen, die zu einem Interessenkonflikt führen, MUSS offengelegt und gemäß ATC-ENT-006 behandelt werden.

## 6. Metriken & Akzeptanzkriterien

1. **Rollen-Registry-Abdeckung:** 100% aller aktiven Rollen MUSS in der Rollen-Registry vollständig dokumentiert sein.
2. **Agenten-Zuordnungsquote:** 100% aller agierenden KI-Agenten MUSS einer gültigen agent_id nach ATC-AAS-001 und der Rolle ROLE-AI-AGENT zugewiesen sein.
3. **Approver-Invariante:** 0 Verstöße gegen das Verbot von Agenten als Approver (ATC-STD-000 §14.1).
4. **Audit-Vollständigkeit:** 100% aller von KI-Agenten ausgeführten Aktionen MUSS mit audit_required: true protokolliert werden.

## 7. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in `registry/findings.yaml` erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 8. Security Considerations

Fehlkonfigurierte oder verwaiste Rollen stellen ein erhebliches Sicherheitsrisiko dar. Rollenrechte MUSS nach dem Prinzip der geringsten Rechtevergabe (Least Privilege) vergeben werden. KI-Agenten MUSS strikt auf ihren zugewiesenen Funktionsbereich beschränkt bleiben.

## 9. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (Owner-Sammelfreigabe) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 10. References

### Normative Referenzen
- **ATC-STD-000**: Verfassung der A-TownChain Ökosystems

### Informative Referenzen
- **SCR-0041**: Owner-Auftrag zur Batch-Elaboration dünner Standards
- **ATC-AAS-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-003**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-006**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-010**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-014**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-005**: Referenzierter Standard im ATC-Regelwerk
