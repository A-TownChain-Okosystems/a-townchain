---
standard:
  id: ATC-ENT-008
  title: "ATC-ENT-008 — Organisationsstruktur Standard"
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

# ATC-ENT-008 — Organisationsstruktur Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-008 (Organisationsstruktur Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Organisationshierarchie (verbindlich)

```
A-TownChain Ökosystems
├── Executive / Governance
├── Architecture
├── Blockchain (L1, Consensus, ZKP, Token, Interoperability)
├── Software Engineering (Backend, Frontend, Infrastructure, QA)
├── AI Engineering (AI Agents, Model Infrastructure, Agent Governance, AI Security)
├── Security
├── DevOps / Infrastructure
├── Game / GameFi
├── Wallet / DeFi
├── Marketplace
├── Documentation / Wiki
└── Compliance / Audit
```

## 2. Pflichtattribute je Organisationseinheit (Registry `registry/org-units.yaml`)

```yaml
org_unit:
  id: UNIT-BLOCKCHAIN
  name: Blockchain
  owner: ROLE-ARCH          # ENT-002-Rolle
  deputy: ROLE-DEV
  responsibility: "..."
  permissions: [...]        # abgeleitet aus ENT-004
  kpis: [...]               # ENT-013
  repositories: [atc-blockchain, atc-node, atc-zkp]   # ENT-009
  standards: [ATC-STD-NET-003, ATC-STD-ZKP-001]       # required
  documentation: {...}       # Wiki/Architektur-Pflichten
  audit_requirements: {...}  # ENT-014
```

## 3. Regeln

- Jedes Repository gehört zu genau einer Organisationseinheit (Mapping in
  ENT-009 Repo-Registry); jede Einheit hat Owner UND Deputy.
- 26 aktive Repos: Zuordnung bestehender 5-Produktlinien-Struktur
  (Governance, Docs, Core, Protocol Services, User Products, Integration)
  auf die obigen Einheiten; Doppelposten dokumentieren (ENT-002 §4).
- Änderungen nur über ENT-010 mit Registry-Update und Notion-/Wiki-Sync
  (AI-DEV-010).

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
## 4. Ergänzende Organisationsregeln

1. Jede Organisationseinheit MUSS in registry/org-units.yaml mit einem zugewiesenen Owner (ROLE-XXX) und einem Deputy eingetragen sein.
2. Repositories MUSS überschneidungsfrei genau einer Organisationseinheit zugeordnet werden; Mehrfachzuordnungen sind unzulässig.
3. Organisationseinheiten MUSS mindestens einmal jährlich auf ihre operative Wirksamkeit und korrekte Verlinkung zu Standards geprüft werden.
4. Änderungen an der Organisationsstruktur MUSS über das Enterprise Change Management gemäß ATC-ENT-010 beantragt und freigegeben werden.
5. Jede Organisationseinheit MUSS spezifische KPIs definieren und deren Erhebung nach ATC-ENT-013 sicherstellen.

## 5. Metriken & Akzeptanzkriterien

1. **Org-Unit-Vollständigkeit:** 100% aller aktiven Organisationseinheiten MUSS in registry/org-units.yaml vollständig konfiguriert sein.
2. **Repo-Zuordnungsquote:** 100% der Unternehmens-Repositories MUSS genau einer Org-Einheit zugewiesen sein.
3. **Owner/Deputy-Besetzung:** 100% der Einheiten MUSS sowohl einen aktiven Owner als auch einen benannten Deputy aufweisen.
4. **Change-Konformität:** 0 nicht genehmigte Strukturänderungen in der Org-Registry.

## 6. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in `registry/findings.yaml` erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 7. Security Considerations

Klare Organisationsstrukturen verhindern ungeklärte Zuständigkeiten und Sicherheitslücken. Zugriffsrechte auf Repositories MUSS streng an die Zugehörigkeit zur Organisationseinheit gebunden sein. Änderungen an den Org-Unit-Definitionen MUSS schreibgeschützt und auditierbar sein.

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
- **ATC-ENT-002**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-004**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-009**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-010**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-013**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-014**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-AI-DEV-010**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-005**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-NET-003**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-ZKP-001**: Referenzierter Standard im ATC-Regelwerk
