---
standard:
  id: ATC-ENT-001
  title: "ATC-ENT-001 — Enterprise Governance Standard"
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

# ATC-ENT-001 — Enterprise Governance Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft
> **Block:** ATC-ENT (Enterprise Standards Layer) · **Priorität:** P0 · **Position:** zwischen ATC-STD-000 (Verfassung) und den technischen Familien (AI-DEV, AAS, 201-204, BUG, NET, ZKP, 100/300)

## Abstract

ATC-ENT-001 (Enterprise Governance Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## Scope

**Scope:** Gilt für: Organisationsebene von A-TownChain Ökosystems — oberhalb der technischen Familien, unterhalb der Verfassung ATC-STD-000.
Nicht-Gilt: normative Standardtexte (Änderungen via SCR, ATC-STD-000 §19-33); technische Domäneninhalte (NET/ZKP/AI-DEV).

## 1. Zweck des Enterprise-Layers

Einheitliche Regeln dafür, wie das Unternehmen organisiert, entscheidet,
entwickelt, dokumentiert, prüft, verändert und auditiert wird — getrennt
von technischen Standards. ATC-ENT legt fest:

Wer darf was entscheiden? · Wer trägt Verantwortung? · Wie werden Änderungen
genehmigt? · Wie werden Risiken behandelt? · Wie wird Wissen verwaltet? ·
Wie wird auditiert? · Wie werden Menschen UND KI-Agenten organisatorisch
eingebunden?

## 2. Standard-Hierarchie (verbindlich)

```
ATC-STD-000 (Verfassung, Meta-Ebene)
    └── ATC-ENT (Enterprise-Layer, dieser Block)
            ├── ATC-GOV  — in ATC-STD-000 §14.1 verankert
            ├── ATC-AI   — ATC-STD-AI-DEV-001..012 + ATC-AAS-001..025
            ├── ATC-DEV  — ATC-STD-100, 300, 201-204, BUG-001..004
            ├── ATC-SEC  — ATC-STD-203 (+ ENT-011 Risiko-Registry)
            ├── ATC-ARCH — ATC-STD-100 (Language/Stack) + ARCH-Entscheidungen (DEC-NNNN)
            ├── ATC-QA   — ENT-015 (DoD) + AI-DEV-008
            ├── ATC-OPS  — offen (DevOps/Betrieb, eigene Familie später)
            ├── ATC-BLOCK— ATC-STD-NET-001..008, ZKP-001..010
            ├── ATC-DATA — offen (Datenstandards, später)
            └── ATC-DOC  — Wiki/Dokumentation (BUG-004, AI-DEV-010)
```

Technische Standards dürfen ENT-Regeln nie konterkarieren; Konflikt →
ATC-AAS-013 (Konflikt-Leiter: Standard > Architecture > …).

## 3. Dokumentenstandard (Metadaten-Pflicht)

Jedes Unternehmensdokument trägt:

```yaml
document:
  id: ATC-ENT-001          # Namensraum gem. §37/Schema entStandardId
  title: "Enterprise Governance Standard"
  version: "1.1.0"
  status: draft|candidate|approved|stable|deprecated|archived
  owner: ROLE-CEO           # Rollen: ATC-ENT-002
  approver: ROLE-CTO
  created: 2026-09-07
  updated: 2026-09-07
  review_cycle: annual       # Pflicht: jährung Review-Termin
```

Dokumente ohne gültige Identität haben keinen normativen Status.

## 4. Geltungsbereich

ATC-ENT bindet alle Organisationseinheiten (ENT-008), Rollen (ENT-002),
Entscheidungen (ENT-003), Repositories (ENT-009) — menschliche wie
KI-Agenten-Mitglieder gleichermaßen (§7 Einbindung über AAS/AI-DEV).

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

NORMATIV: ATC-STD-000, ATC-STD-280ff (Security-Familie), ATC-ENT-002 · INFORMATIVE: Registry-Kategorie enterprise
## 5. Spezifische Enterprise-Governance-Regeln

1. Jedes Unternehmensdokument MUSS vor seiner Inkraftsetzung von der in den Metadaten festgelegten Approver-Rolle (z. B. ROLE-CTO) freigegeben und digital signiert werden.
2. Bei Strukturänderungen oder Anpassungen von Hierarchieebenen MUSS eine Auswirkungsanalyse gemäß ATC-ENT-010 durchgeführt werden.
3. Sämtliche Abweichungen von der definierten Standard-Hierarchie MUSS im zentralen Risiko-Register gemäß ATC-ENT-011 als Risiko erfasst und nachverfolgt werden.
4. Alle Enterprise-Standards MUSS jährlich (annual Review-Cycle) auf Aktualität und Konformität mit ATC-STD-000 geprüft werden.
5. Änderungen an den Metadaten eines Dokuments MUSS über den Change-Management-Prozess nach ATC-ENT-010 dokumentiert und freigegeben werden.

## 6. Metriken & Akzeptanzkriterien

1. **Dokumenten-Metadaten-Abdeckung:** 100% aller Unternehmensdokumente MUSS ein vollständiges und valides YAML-Metadaten-Header-Schema aufweisen.
2. **Review-Frequenz:** 100% aller Enterprise-Standards MUSS innerhalb des vorgegebenen Review-Intervalls (jährlich) überprüft und aktualisiert werden.
3. **Konflikt-Auflösungszeit:** 100% aller gemeldeten Standard-Konflikte MUSS innerhalb von 5 Arbeitstagen gemäß ATC-AAS-013 entschieden und aufgelöst werden.
4. **Compliance-Quote:** 0 ungelöste P0-Compliance-Findings (F-NNN gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005) bei automatisierten Validator-Läufen.

## 7. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in  erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 8. Security Considerations

Unternehmensdokumente und Governance-Regeln stellen die operative Grundlage dar. Jegliche unautorisierte Modifikation MUSS durch Zugriffsrechte und CI-Pipelines unterbunden werden. Kritische Governance-Änderungen MUSS zusätzlich eine Sicherheitsbewertung nach ATC-STD-203 durchlaufen.

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
- **ATC-AAS-013**: Konflikt-Eskalation und Standards-Rangfolge
- **ATC-STD-203**: Security Considerations & Scanning Standard
- **ATC-STD-BUG-001**: Bug & Finding Management Standard
- **ATC-STD-BUG-005**: Compliance & Validation Standard
- **ATC-ENT-002**: Rollen & Verantwortlichkeiten Standard
- **ATC-ENT-003**: Entscheidungsmanagement Standard
- **ATC-ENT-008**: Organisationsstruktur Standard
- **ATC-ENT-009**: Repository Governance Standard
- **ATC-ENT-010**: Enterprise Change Management Standard
- **ATC-ENT-011**: Risiko-Management Standard
- **ATC-ENT-015**: Qualitätsmanagement & Definition of Done Standard

## 5. Spezifische Enterprise-Governance-Regeln

1. Jedes Unternehmensdokument MUSS vor seiner Inkraftsetzung von der in den Metadaten festgelegten Approver-Rolle (z. B. ROLE-CTO) freigegeben und digital signiert werden.
2. Bei Strukturänderungen oder Anpassungen von Hierarchieebenen MUSS eine Auswirkungsanalyse gemäß ATC-ENT-010 durchgeführt werden.
3. Sämtliche Abweichungen von der definierten Standard-Hierarchie MUSS im zentralen Risiko-Register gemäß ATC-ENT-011 als Risiko erfasst und nachverfolgt werden.
4. Alle Enterprise-Standards MUSS jährlich (annual Review-Cycle) auf Aktualität und Konformität mit ATC-STD-000 geprüft werden.
5. Änderungen an den Metadaten eines Dokuments MUSS über den Change-Management-Prozess nach ATC-ENT-010 dokumentiert und freigegeben werden.

## 6. Metriken & Akzeptanzkriterien

1. **Dokumenten-Metadaten-Abdeckung:** 100% aller Unternehmensdokumente MUSS ein vollständiges und valides YAML-Metadaten-Header-Schema aufweisen.
2. **Review-Frequenz:** 100% aller Enterprise-Standards MUSS innerhalb des vorgegebenen Review-Intervalls (jährlich) überprüft und aktualisiert werden.
3. **Konflikt-Auflösungszeit:** 100% aller gemeldeten Standard-Konflikte MUSS innerhalb von 5 Arbeitstagen gemäß ATC-AAS-013 entschieden und aufgelöst werden.
4. **Compliance-Quote:** 0 ungelöste P0-Compliance-Findings (F-NNN gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005) bei automatisierten Validator-Läufen.

## 7. Compliance & Verifikation

Die Einhaltung aller Vorgaben dieses Standards MUSS durch automatisierte Validator-Läufe (S-01 bis S-25) kontinuierlich geprüft werden. Etwaige Abweichungen oder Nicht-Konformitäten MUSS unverzüglich als Findings (F-NNN) gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005 in `registry/findings.yaml` erfasst und kategorisiert werden. Abweichungen von Sicherheits- oder Governance-Invarianten MUSS priorisiert vor einem Release behoben werden.

## 8. Security Considerations

Unternehmensdokumente und Governance-Regeln stellen die operative Grundlage dar. Jegliche unautorisierte Modifikation MUSS durch Zugriffsrechte und CI-Pipelines unterbunden werden. Kritische Governance-Änderungen MUSS zusätzlich eine Sicherheitsbewertung nach ATC-STD-203 durchlaufen.

## 9. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.1.0 | 2026-09-07 | Initialversion (Owner-Sammelfreigabe) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 10. References

### Normative Referenzen
- **ATC-STD-000**: Verfassung der A-TownChain Ökosystems

### Informative Referenzen
- **SCR-0041**: Owner-Auftrag zur Batch-Elaboration dünner Standards
- **ATC-AAS-013**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-002**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-003**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-008**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-009**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-010**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-011**: Referenzierter Standard im ATC-Regelwerk
- **ATC-ENT-015**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-100**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-203**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-AI-DEV-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-001**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-BUG-005**: Referenzierter Standard im ATC-Regelwerk
- **ATC-STD-NET-001**: Referenzierter Standard im ATC-Regelwerk
