---
standard:
  id: ATC-ENT-005
  title: "ATC-ENT-005 — Unternehmensrichtlinien Standard"
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

# ATC-ENT-005 — Unternehmensrichtlinien Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-005 (Unternehmensrichtlinien Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Richtlinien-Hierarchie

1. ATC-STD-000 (Verfassung) — nicht überlagerbar
2. ATC-ENT-Richtlinien ( Policies, ENT-Block)
3. Domänen-Standards (AI-DEV, AAS, 201-204, …)
4. Repo-interne Richtlinien (CONTRIBUTING, AGENTS.md)

Niedrigere Ebene darf höhere Ebene präzisieren, nie aufweichen.

## 2. Richtlinien-Registry

Jede Richtlinie ist ein Dokument nach ENT-001 §3 (Metadaten) mit
`policy: {id: POL-NNNN, scope, enforcement, owner}` — enforcement:
automatisiert (CI/Workflow) bevorzugt, sonst Audit-Stichprobe (ENT-014).

## 3. Mindest-Policies (Initialbestand)

- Security-Policy (Secrets: AAS-014; Scanning: ATC-STD-203)
- Data-Policy (Datenklassifikation, Offenlegung)
- AI-Policy (Agenten-Einsatz nach AAS/AI-DEV; False-Claim-Verbot: AAS-012)
- Release-Policy (NET-003/004 Mainnet/Release-Gates)

## 4. Änderungen

Nur über ENT-010 (Change) mit Impact auf betroffene Standards;
Richtlinien-Änderungen lösen Review-Pflicht abhängiger Dokumente aus
(review_cycle, ENT-001 §3).

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
## 5. Verbindliche Richtlinien-Governance-Regeln

1. Neue Unternehmensrichtlinien MUSS in der zentralen Richtlinien-Registry mit einer eindeutigen ID (POL-NNNN) erfasst werden.
2. Jede Richtlinie MUSS mindestens eine bevorzugt automatisierte CI-Durchsetzungsmethode oder eine strukturierte Audit-Vorschrift gemäß ATC-ENT-014 definieren.
3. Unternehmensrichtlinien MUSS frei von Widersprüchen zur Verfassung ATC-STD-000 und zum Sicherheitsstandard ATC-STD-203 formuliert werden.
4. Bei Änderungen an einer Richtlinie MUSS alle davon abgeleiteten Dokumente und Workflows innerhalb von 10 Arbeitstagen überprüft werden.
5. Sämtliche Richtlinien-Verstöße MUSS im zentralen Risiko-Register nach ATC-ENT-011 bewertet und protokolliert werden.

## 6. Metriken & Akzeptanzkriterien

1. **Registry-Eintragungsquote:** 100% aller aktiven Richtlinien MUSS in der Richtlinien-Registry eingetragen sein.
2. **CI-Enforcement-Rate:** Mindestens 80% der Richtlinien-Vorgaben MUSS über automatisierte CI/CD-Pipelines geprüft werden.
3. **Hierarchie-Konformität:** 0 Konflikte zwischen Richtlinien und höherrangigen Standards (ATC-STD-000).
4. **Aktualisierungsquote:** 100% aller Richtlinien MUSS innerhalb des vorgegebenen Review-Zyklus aktualisiert werden.

## 7. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in `registry/findings.yaml` erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 8. Security Considerations

Richtlinien bilden die regulatorische Leitplanke für den gesamten Betrieb. Ungesicherte oder veraltete Richtlinien können zu Sicherheitslücken führen. Der Schreibzugriff auf die Richtlinien-Registry MUSS strikt auf berechtigte Governance-Rollen beschränkt werden.

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
- **ATC-AAS-012**: Referenzierter Standard im ATC-Regelwerk
- **ATC-AAS-014**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-010**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-011**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-014**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-203**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-005**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-NET-003**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-NET-004**: Referenzierter Standard im ATC-Regelwerk
