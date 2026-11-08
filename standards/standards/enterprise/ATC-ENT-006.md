---
standard:
  id: ATC-ENT-006
  title: "ATC-ENT-006 — Interessenkonflikte Standard"
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

# ATC-ENT-006 — Interessenkonflikte Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-ENT-006 (Interessenkonflikte Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Konfliktklassen

Rollen-Kumulation (Autor ≠ Approver für dieselbe Entscheidung), wirtschaftliche
Interessen, emotionale/verantwortliche Doppelpflichten, Agenten-Zielkonflikte
(Task-Ziel vs. Governance-Regel).

## 2. Kernregeln

- Wer eine Entscheidung vorbereitet (Autor/Agent), ist nicht deren Approver
  (Verfassung §14.1; Agenten nie Approver).
- Bei erkanntem Konflikt: Offenlegung im DEC-Record vor der Entscheidung;
  Entscheider entscheidet über Abgabe der Entscheidung an nächsthöhere Rolle.
- Agenten-Zielkonflikte: Task-Ziel verliert gegen Governance — Eskalation
  nach AI-DEV-011 §2, Dokumentation als Finding.

## 3. Dokumentation

Konflikt-Fälle fließen in den Audit-Trail (ENT-014, Feld `conflict_flag`)
und den KPI-Report (ENT-013, Human Escalation Rate).

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
## 4. Verbindliche Regeln zur Vermeidung von Interessenkonflikten

1. Keine Rolle und kein KI-Agent darf die eigenen Arbeitsergebnisse selbst genehmigen oder freigeben (Vier-Augen-Prinzip).
2. Identifizierte Interessenkonflikte MUSS unverzüglich im jeweiligen DEC-Record nach ATC-ENT-003 offengelegt und mit dem conflict_flag markiert werden.
3. Wenn ein KI-Agent einen Zielkonflikt zwischen einer Aufgabenstellung und den Governance-Regeln feststellt, MUSS der Agent die Ausführung abbrechen und nach AI-DEV-011 §2 eskalieren.
4. Entscheidungen, die unter einem nicht offengelegten Interessenkonflikt getroffen wurden, MUSS als ungültig deklariert und erneut geprüft werden.
5. Alle Interessenkonflikt-Meldungen MUSS im monatlichen KPI-Report gemäß ATC-ENT-013 erfasst werden.

## 5. Metriken & Akzeptanzkriterien

1. **Vier-Augen-Prinzip-Quote:** 100% aller Freigaben MUSS durch eine von der Autorenschaft unabhängige Rolle erfolgen.
2. **Offenlegungsrate:** 100% aller identifizierten Konflikte MUSS im Audit-Trail mit conflict_flag versehen sein.
3. **Eskalations-Einhaltung:** 100% der Agenten-Zielkonflikte MUSS ordnungsgemäß nach AI-DEV-011 §2 eskaliert werden.
4. **Unabhängige Auditierung:** 0 nicht untersuchte Interessenkonflikt-Meldungen bei Stichproben-Audits.

## 6. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in `registry/findings.yaml` erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 7. Security Considerations

Unentdeckte Interessenkonflikte gefährden die Integrität des Gesamtsystems. Rollen- und Rechte-Mischungen MUSS systematisch unterbunden werden. Bei Agenten-Systemen MUSS verhindert werden, dass Optimierungsziele Governance-Schranken umgehen.

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
- **ATC-ENT-003**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-013**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-014**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-AI-DEV-011**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-005**: Referenzierter Standard im ATC-Regelwerk
