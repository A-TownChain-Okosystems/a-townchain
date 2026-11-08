---
standard:
  id: ATC-ENT-015
  title: "ATC-ENT-015 — Qualitätsmanagement & Definition of Done Standard"
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

# ATC-ENT-015 — Qualitätsmanagement & Definition of Done Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-015 (Qualitätsmanagement & Definition of Done Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Definition of Done (Unternehmens-DoD)

```
CODE        ✓ implementiert (Review nach AI-DEV-007)
TEST        ✓ Unit-Tests · ✓ Integration-Tests (L1-L4 je AAS-011)
SECURITY    ✓ Security Scan · ✓ Dependency Scan (ATC-STD-203)
DOKU        ✓ Wiki ✓ README ✓ Architektur aktualisiert (ENT-012)
GOVERNANCE  ✓ Change Request/DEC ✓ Approval (ENT-003/010)
RELEASE     ✓ Version ✓ Changelog ✓ Audit-Record (ENT-014)
```

## 2. Regeln

- Nicht erfüllter DoD-Punkt = nicht Done: kein COMPLETED (AI-DEV-004),
  kein Release (Consistency Gate ENT-012 §2).
- DoD gilt für menschliche und Agenten-Beiträge gleichermaßen; Agenten
  liefern für jeden Haken Evidenz (AAS-010), Menschen deklarieren mit
  Verweis (Commit/PR/Record).
- Abweichungen (z.B. reine Doku-Änderung ohne Tests) sind im Task-Record
  als solche deklariert (AI-DEV-008 §1).

## 3. Review

DoD-Konformität wird stichprobenhaft auditiert (ROLE-AUDITOR, ENT-014 §4);
Missachtung = Finding + KPI-Wirkung (ENT-013 §1 Rework/Regression Rate).

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
## 4. Erweiterte DoD-Regeln

1. Eine Aufgabe oder ein Feature gilt erst dann als Done, wenn sämtliche in der DoD-Matrix definierten Kriterien nachweisbar erfüllt sind.
2. Für jeden Haken in der DoD MUSS der Bearbeiter (Mensch oder KI-Agent) konkrete Evidenzen (Commit-SHAs, Testberichte, Audit-IDs) erbringen.
3. Ausnahmen von einzelnen DoD-Punkten MUSS im Task-Record ausführlich begründet und vom Technischen Owner abgezeichnet werden.
4. Nicht erfüllte DoD-Kriterien MUSS die automatische Code-Merge-Freigabe im CI/CD-System unverzüglich blockieren.
5. Regelmäßige Stichprobenprüfungen der DoD-Einhaltung MUSS durch den ROLE-AUDITOR durchgeführt und dokumentiert werden.

## 5. Metriken & Akzeptanzkriterien

1. **DoD-Erfüllungsquote:** 100% aller gemergten PRs MUSS alle anwendbaren DoD-Kriterien erfüllen.
2. **Evidenz-Vollständigkeit:** 100% der als Done markierten Tasks MUSS prüfbare Evidenzlinks enthalten.
3. **Ausnahme-Quote:** Maximal 5% aller Tasks DÜRFEN eine genehmigte DoD-Ausnahme aufweisen.
4. **Regression-Rate:** Absenkung der durch DoD-Mängel verursachten Reworks auf unter 2%.

## 6. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in `registry/findings.yaml` erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 7. Security Considerations

Die Definition of Done stellt sicher, dass kein ungarantierter oder unsicherer Code in die Produktion gelangt. Security-Scans und Dependency-Checks MUSS zwingender Bestandteil der DoD sein. Ein Umgehen der DoD hebelt das gesamte Qualitäts- und Sicherheitsregime aus.

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
- **ATC-AAS-010**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-012**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-013**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-014**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-203**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-AI-DEV-004**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-AI-DEV-007**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-AI-DEV-008**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-005**: Referenzierter Standard im ATC-Regelwerk
