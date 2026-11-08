---
standard:
  id: ATC-ENT-009
  title: "ATC-ENT-009 — Repository Governance Standard"
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

# ATC-ENT-009 — Repository Governance Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-009 (Repository Governance Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Unternehmens-Repo-Registry (Pflichtfelder)

```yaml
repository:
  id: REPO-001
  name: atc-core
  domain: blockchain              # ENT-008-Einheit
  owner: ROLE-ARCH                # fachlich
  technical_owner: ROLE-DEV
  security_owner: ROLE-CISO
  classification: INTERNAL        # PUBLIC|INTERNAL|RESTRICTED
  canonical: true                 # kanonische Ablage (keine Fork-Divergenz)
  standards: [ATC-STD-000, ATC-STD-201, ATC-STD-203, ATC-AAS-025]
  documentation: {wiki: required, readme: required, architecture: required}
```

## 2. Regeln

- Ein Repository ohne Registry-Eintrag hat keinen Unternehmensstatus;
  neue Repos über ENT-010 + §37 ID-Allokation (REPO-NNN fortlaufend).
- `canonical: true` markiert die SSOT-Ablage; Sync-Ziele (Wiki, Docs)
  sind abgeleitet (BUG-004, AI-DEV-010).
- Dokumentations-Pflichten sind Merge-Gate-relevant (Consistency-Gate,
  ENT-012 §2): fehlende Pflicht-Doku blockiert Release.
- Überschneidungsfreie Zuständigkeit: ein Repo = eine Organisationseinheit
  = ein zuständiger Agent (1-Agent-per-Repo, AI-DEV-012 §1).

## 3. Rollout

Nach APPROVED: `registry/org-units.yaml` + `registry/repositories.yaml`
als ableitende Sichten aus der existierenden 26-Repo-Struktur generieren;
Zuordnung mit bestehender AgentAssignment-DB verzahnen.

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
## 4. Erweiterte Repository-Governance-Regeln

1. Jedes Repository der Organisation MUSS in registry/repositories.yaml mit allen geforderten Pflichtfeldern registriert sein.
2. Repositories mit dem Flag canonical: true MUSS als Single Source of Truth (SSOT) behandelt werden; abgeleitete Klon- oder Sync-Ziele DÜRFEN keine abweichenden Primärdaten halten.
3. Jedem Repository MUSS genau ein primär verantwortlicher KI-Agent (1-Agent-per-Repo gemäß AI-DEV-012 §1) zugewiesen werden.
4. Commits in Repositories ohne gültigen Registry-Eintrag MUSS von den CI-Pipelines automatisch blockiert werden.
5. Vor jedem Release MUSS alle in der Registry geforderten Dokumentationselemente (README, Wiki, Architektur) vollständig vorliegen (ENT-012).

## 5. Metriken & Akzeptanzkriterien

1. **Registry-Abdeckung:** 100% aller produktiven und internen Repositories MUSS in registry/repositories.yaml eingetragen sein.
2. **1-Agent-per-Repo-Quote:** 100% der Repositories MUSS eine eindeutige Zuweisung eines verantwortlichen Agenten aufweisen.
3. **SSOT-Konformität:** 0 unautorisierte Divergenzen zwischen kanonischen Repositories und abgeleiteten Sichten.
4. **Dokumentations-Compliance:** 100% der Repositories MUSS die im Standard definierten Pflicht-Dokumente vorweisen.

## 6. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in `registry/findings.yaml` erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 7. Security Considerations

Unregistrierte oder fehlerhaft klassifizierte Repositories stellen ein signifikantes Sicherheitsrisiko dar. Repositories mit Klassifizierung RESTRICTED oder INTERNAL MUSS strengen Zugriffskontrollen unterliegen. Das Einspielen von Code MUSS über signierte Commits und geprüfte Merge-Gates erfolgen.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (Owner-Sammelfreigabe) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

### Normative Referenzen
- **ATC-STD-000**: Verfassung der A-TownChain Ökosystems

### Informative Referenzen
- **SCR-0041**: Owner-Auftrag zur Batch-Elaboration dünner Standards
- **ATC-AAS-025**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-008**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-010**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-012**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-201**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-203**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-AI-DEV-010**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-AI-DEV-012**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-004**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-005**: Referenzierter Standard im ATC-Regelwerk
