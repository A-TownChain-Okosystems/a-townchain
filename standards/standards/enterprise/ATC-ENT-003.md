---
standard:
  id: ATC-ENT-003
  title: "ATC-ENT-003 — Entscheidungsmanagement Standard"
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

# ATC-ENT-003 — Entscheidungsmanagement Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-003 (Entscheidungsmanagement Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Kernregel

**Jede kritische Entscheidung muss einen Verantwortlichen, einen Status,
eine Begründung und eine nachvollziehbare Historie besitzen.** Keine
kritische Architekturentscheidung darf „irgendwo im Chat verschwinden".

## 2. Decision Record (DEC-NNNN)

```
DEC-0001
├── Problem
├── Optionen
├── Bewertung
├── Entscheidung
├── Verantwortlicher (ROLE-XXX)
├── Datum
├── Auswirkungen
├── Abhängigkeiten (DEC-/SCR-/Standard-Referenzen)
└── Review-Date
```

## 3. Status-Lifecycle

PROPOSED → UNDER_REVIEW → APPROVED | REJECTED → SUPERSEDED → ARCHIVED.

## 4. Verhältnis zu bestehenden Instrumenten

- Freigaben normativer Standards: unverändert ATC-STD-000 §9 mit
  approval/APPROVAL-DECISION-*.md (diese SIND DEC-Records in
  Kanon-Form; künftig zusätzlich DEC-ID vergeben).
- Bestands-Entscheidungen AD-016..AD-046 (Owner-Mandate, AGENT_MANIFEST)
  bleiben gültig und werden als DEC-Records anerkannt (grandfathered,
  keine Rückmigration erzwungen).
- SCR-XXXX (Standard-Änderungen): formale Änderungsspur bleibt SCR;
  DEC deckt Organisations-/Produkt-/Architektur-Entscheidungen ab.
- Priorisierungskonflikte zwischen Entscheidungen → AAS-013.

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
## 5. Verbindliche Entscheidungsregeln

1. Alle kritischen Architekturentscheidungen und organisatorischen Beschlüsse MUSS in Form eines DEC-Records (DEC-NNNN) dokumentiert werden.
2. Ein DEC-Record MUSS vor der Freigabe eine vollständige Bewertung von mindestens zwei Alternativen sowie eine Begründung der gewählten Option enthalten.
3. DEC-Records MUSS von einer autorisierten menschlichen Rolle (z. B. ROLE-CEO, ROLE-CTO oder ROLE-ARCH) freigegeben werden; KI-Agenten MUSS als reine Autoren fungieren.
4. Statusübergänge im DEC-Lifecycle MUSS mit Zeitstempel und Verantwortlichem im Audit-Log gemäß ATC-ENT-014 erfasst werden.
5. Bei Entscheidungen mit Sicherheitsauswirkungen MUSS eine Sicherheitsüberprüfung nach ATC-STD-203 erfolgen.

## 6. Metriken & Akzeptanzkriterien

1. **DEC-Vollständigkeit:** 100% aller kritischen Architekturentscheidungen MUSS als valider DEC-Record vorliegen.
2. **Freigabe-Konformität:** 100% aller freigegebenen DEC-Records MUSS von einer berechtigten menschlichen Rolle abgezeichnet sein.
3. **Rückverfolgbarkeit:** 100% aller DEC-Records MUSS Verweise auf betroffene Standards und Tickets aufweisen.
4. **Review-Einhaltung:** Abgelaufene DEC-Records MUSS innerhalb von 14 Tagen überprüft und im Status aktualisiert werden.

## 7. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in `registry/findings.yaml` erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 8. Security Considerations

Invalide oder unberechtigte Entscheidungen können die Systemsicherheit gefährden. Änderungen an DEC-Records MUSS immutabel im Repository erfasst werden. Freigaben MUSS kryptografisch oder durch fälschungssichere Audit-Trail-Verfahren geschützt werden.

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
- **ATC-AAS-013**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-014**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-203**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-005**: Referenzierter Standard im ATC-Regelwerk
