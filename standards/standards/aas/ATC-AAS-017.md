---
standard:
  id: ATC-AAS-017
  title: "ATC-AAS-017 — Agent Human Approval Standard"
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

# ATC-AAS-017 — Agent Human Approval Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft

## Abstract

ATC-AAS-017 (Agent Human Approval Standard) — Owner-Sammelfreigabe 07.09.2026 20:20 UTC+2 (alles freigeben): CANDIDATE -> APPROVED, normativ in Kraft; dokumentiert in approval/APPROVAL-DECISION-2026-09-07-AAS-ENT-AUDIT.md.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119).

## 1. Autonom zulässig (innerhalb Scope + Capability + Permission)

Code analysieren · Tests ausführen · Dokumentation verbessern ·
Branch erstellen · PR vorbereiten.

## 2. Human Approval erforderlich (Pflichtliste, nicht abschließend)

Merge · Mainnet Deployment · Consensus-Änderung · Tokenomics-Änderung ·
Kryptographie-Änderung · Security Policy · Rechteänderung ·
Infrastrukturänderung.

## 3. Verfahren

Unverändert AI-DEV-011: `human_review: {required, requested}` je Task und
PR; Genehmigungsformen Mandat/Freigabe-Vermerk/SCR; Agenten sind niemals
Approver (ATC-STD-000 §14.1). Die Pflichtliste §2 ist mit AI-DEV-011 §2
Eskalationsstufen a-d verzahnt: jede Liste-Aktion löst Eskalation aus.

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

1. Aktionen aus der Pflichtliste für Human Approval MÜSSEN bis zur expliziten menschlichen Freigabe pausieren.
2. Freigabeanfragen MÜSSEN den vollständigen Kontext, Auswirkungsanalysen und den betroffenen Code enthalten.
3. Ein Human Approval MUSS kryptographisch oder über ein auditierbares Governance-System erteilt werden.
4. Stillschweigende Zustimmung oder Timeouts MÜSSEN als Ablehnung gewertet werden.
5. Erteilte Freigaben MÜSSEN innerhalb eines Gültigkeitsfensters von 24 Stunden umgesetzt werden.
6. Abgewiesene Freigabeanfragen MÜSSEN unverzüglich im Audit-Trail vermerkt werden und stoppen den Vorgang.

## 5. Metriken & Akzeptanzkriterien

- 100% aller genehmigungspflichtigen Operationen MÜSSEN ein dokumentiertes Human Approval aufweisen.
- 0 unautorisierte autonome Ausführungen MÜSSEN bei risikobehafteten Aktionen vorkommen.
- Gültigkeitsdauer erteilter Approvals MUSS strikt überwacht werden (0 Ausführungen nach Ablauf).
- Freigabe-Logs MÜSSEN zu 100% fälschungssicher hinterlegt werden.

## 6. Compliance & Verifikation

Die Einhaltung dieses Standards MUSS über Validator-Läufe (S-01 bis S-25) überprüft werden. Abweichungen führen zu Findings F-001 bis F-005 gemäß ATC-STD-BUG-001 bis ATC-STD-BUG-005. Nicht genehmigte Aktionen MÜSSEN storniert werden.

## 7. Security Considerations

Human Approval schützt vor unbeabsichtigten oder schädlichen Auswirkungen kritischer Agenten-Operationen. Umgehungen des Approval-Prozesses MÜSSEN zur Deaktivierung des Agenten führen.

## 8. Changelog

| Version | Datum | Beschreibung |
| :--- | :--- | :--- |
| 1.0.0 | 2026-09-07 | Initialversion (APPROVED) |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 9. References

- **NORMATIVE:** ATC-STD-000
- **INFORMATIVE:** SCR-0041, AI-DEV-011, ATC-STD-000, ATC-STD-BUG-001, ATC-STD-BUG-002, ATC-STD-BUG-003, ATC-STD-BUG-004, ATC-STD-BUG-005
