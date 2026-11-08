---
standard:
  id: ATC-ENT-010
  title: "ATC-ENT-010 — Enterprise Change Management Standard"
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

# ATC-ENT-010 — Enterprise Change Management Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-010 (Enterprise Change Management Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Change-Pipeline (Pflichtreihenfolge)

```
CHANGE REQUEST → IMPACT ANALYSIS → TECHNICAL REVIEW → SECURITY REVIEW →
APPROVAL → IMPLEMENTATION → TEST → AUDIT → DOCUMENTATION UPDATE → RELEASE
```

Keine Ausnahmeschritte; Parallelisierung nur zwischen den Reviews.

## 2. Änderungsklassen

- Standards: unverändert SCR (ATC-STD-000 §19-33) — SCR bleibt die
  formale Änderungsspur für normative Texte.
- Organisation (Rollen, Einheiten, Zuständigkeiten): ENT-010 mit
  DEC-Record (ENT-003) + Registry-Update.
- Architektur/Produkt: ENT-010 mit Impact auf Dependency-Graph
  (ATC-STD-204) + betroffene Interface-Tests (IFC-NNNN).
- Richtlinien: ENT-010 §-Pipeline mit POL-Update (ENT-005).

## 3. Impact-Analyse (Pflichtinhalte)

Betroffene Repos (ENT-009), Standards, Interfaces (ATC-STD-204),
Risiken (ENT-011), Abhängigkeiten, Migrations-/Rollback-Plan
(Rollback-Pflicht nach AAS-009 §1).

## 4. Notfall-Änderung (Emergency)

Wie Verfassung §32: nachgelagerte Dokumentation binnen 72h, volle
Pipeline rückwirkend; Owner-Benachrichtigung sofort. Kein Stillstand
kritischer Systeme erzwingbar.

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
## 5. Verbindliche Change-Management-Regeln

1. Sämtliche organisatorischen, technischen und strukturellen Änderungen MUSS die definierte Change-Pipeline lückenlos durchlaufen.
2. Vor der Genehmigung einer Änderung MUSS eine schriftliche Impact-Analyse gemäß §3 erstellt und im Change-Request hinterlegt werden.
3. Sicherheitsrelevante Änderungen MUSS zwingend ein Security Review nach ATC-STD-203 erfolgreich bestehen.
4. Emergency-Änderungen MUSS unverzüglich an den Owner gemeldet und innerhalb von 72 Stunden vollständig nachdokumentiert werden.
5. Jede genehmigte Änderung MUSS einen dokumentierten Migrations- und Rollback-Plan aufweisen (AAS-009 §1).

## 6. Metriken & Akzeptanzkriterien

1. **Pipeline-Konformität:** 100% aller produktiven Änderungen MUSS die Change-Pipeline ohne ungeplante Abkürzungen durchlaufen.
2. **Impact-Analyse-Abdeckung:** 100% der Change Requests MUSS eine vollständige Impact-Analyse aufweisen.
3. **Emergency-Nachdokumentationsrate:** 100% aller Notfall-Änderungen MUSS innerhalb der 72h-Frist nachdokumentiert sein.
4. **Rollback-Fähigkeit:** 100% der freigegebenen Changes MUSS über einen getesteten Rollback-Plan verfügen.

## 7. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in `registry/findings.yaml` erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 8. Security Considerations

Ungeprüfte Änderungen sind die häufigste Ursache für Sicherheitsvorfälle. Jede Änderung MUSS vor der Freigabe auf unbeabsichtigte Nebeneffekte und Schwachstellen gescannt werden. Berechtigungen zum Durchführen von Änderungen MUSS rollenbasiert geschützt sein.

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
- **ATC-AAS-009**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-003**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-005**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-009**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-011**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-203**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-204**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-005**: Referenzierter Standard im ATC-Regelwerk
