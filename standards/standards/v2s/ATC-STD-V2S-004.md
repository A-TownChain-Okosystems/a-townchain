---
standard:
  id: ATC-STD-V2S-004
  title: "Requirements Standard"
  version: "1.0.0"
  status: candidate
  category: v2s
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Projekte gemäß ATC-STD-V2S-000 Scope"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-V2S-004 — Requirements Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-004 regelt die Erfassung, Klassifizierung, Eindeutigkeit und Validierung
von Anforderungen (Requirements) in allen ATC-Softwareprojekten. Er stellt sicher,
dass Anforderungen mit normierten IDs nach ATC-STD-000 §36 formuliert werden,
prüfbare Acceptance Criteria besitzen und die Bedingungen der Definition of Ready (DoR)
gemäß ATC-STD-V2S-000 §6 erfüllen, bevor sie in die Spezifikation übernommen werden.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/DARF NICHT — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** alle funktionalen (FR) und nicht-funktionalen (NFR) Anforderungen für
ATC-Softwarekomponenten, Smart Contracts, APIs, Protokolle und KI-Modelle.
**Nicht-Gilt:** allgemeine Wünsche ohne konkreten Systembezug oder nicht-evaluierte
Ideenskizzen.

## §1 Gegenstand & Verortung

Dieser Standard ordnet sich in Station 4 (REQUIREMENTS) des 20-Stationen-Lifecycles
gemäß ATC-STD-V2S-000 §1 ein. Station 4 überführt die in Station 3 (BUSINESS/SYSTEM GOAL)
definierten Capabilities in präzise formulierte Anforderungen. Sie bildet die direkte
Grundlage für Station 5 (SPECIFICATION) und leitet die Querschnitts-Traceability
(ATC-STD-V2S-005) ein.

## §2 Kernregeln

1. **KR-1:** Jede Anforderung MUSS eine eindeutige ID nach der Konvention `REQ-<DOM>-NNN` gemäß ATC-STD-000 §36 aufweisen (MUST — Bezug Master §3).
2. **KR-2:** Jedes Requirement MUSS mindestens ein eindeutiges, prüfbares Acceptance Criterion (AC) besitzen (MUST — Bezug Master §2).
3. **KR-3:** Acceptance Criteria MÜSSEN so gestaltet sein, dass sie als automatisierte oder manuelle Testfälle operationalisierbar sind.
4. **KR-4:** Ein Requirement MUSS alle Kriterien der Definition of Ready (DoR) nach ATC-STD-V2S-000 §6 erfüllen, um den Status READY zu erhalten.
5. **KR-5:** Anforderungen MÜSSEN explizit in funktionale Anforderungen (FR) und nicht-funktionale Anforderungen (NFR) kategorisiert sein.
6. **KR-6:** Unvollständige, widersprüchliche oder nicht prüfbare Requirements DÜRFEN NICHT in den Status READY versetzt oder spezifiziert werden (MUST NOT).

## §3 Artefakte & Nachweispflichten

Für die Station 4 (REQUIREMENTS) entstehen folgende obligatorische Artefakte:

- **Requirements Document / Catalog:** Dokumentation (`docs/REQUIREMENTS.md` oder `.atc/requirements.yaml`) mit allen REQ-IDs.
- **Acceptance Criteria Catalog:** Liste aller ACs inklusive Zuordnung zu den jeweiligen REQ-IDs und Testansätzen.
- **DoR Verification Record:** Nachweis der bestandenen DoR-Prüfung für jedes einzelne Requirement.
- **Requirement Status Log:** Protokollierung der Statusübergänge: IDEA → PROPOSED → SPECIFIED → READY (gemäß Master §3).

## §4 Quality-Gate-Bezug

Dieser Standard bildet das Fundament für das Quality Gate **V2S-G1** ("Sind die Anforderungen vollständig?"):

- **Prüffrage:** Sind alle Anforderungen mit REQ-IDs versehen, durch ACs abgedeckt und DoR-konform?
- **Bedingung:** Gate V2S-G1 MUSS bestanden sein, bevor die technische Spezifikation (Station 5) freigegeben wird.
- **Gate-Record-Pflicht:** Ausführliches Prüfprotokoll inklusive Prüfer, Datum, Checklistenergebnis und Nachweisen.
- **Reifegrad:** Der erfolgreiche Abschluss erreicht Reifegrad **M2 (SPECIFIED)** gemäß ATC-STD-V2S-000 §14.

## §5 Schnittstellen

- **Upstream Phase:** Station 3 (BUSINESS/SYSTEM GOAL, ATC-STD-V2S-003).
- **Downstream Phase:** Station 5 (SPECIFICATION, ATC-STD-V2S-006) und Station 6 (ARCHITECTURE, ATC-STD-V2S-007).
- **Andock-Standards:**
  - ATC-STD-V2S-000 (Lifecycle Master)
  - ATC-STD-V2S-005 (Traceability Standard)
  - ATC-STD-202 (Requirements Engineering)
  - ATC-STD-BUG-001 (Finding-Klassifizierung)

## Metriken & Akzeptanzkriterien

- **M1:** 100% aller Anforderungen besitzen eine konforme REQ-ID nach ATC-STD-000 §36.
- **M2:** 100% Abdeckung aller Requirements mit mindestens einem Acceptance Criterion.
- **M3:** DoR-Pass-Rate beträgt 100% vor Phasenübergang zu V2S-G1.
- **M4:** 0 unvollständige oder widersprüchliche Requirements im Status READY.

## Compliance & Verifikation

Die Einhaltung wird durch automatisierte Scans des `atc_std_validator.py` sowie
DoR-Reviews verifiziert. Mängel wie fehlende REQ-IDs, unvollständige Acceptance
Criteria oder übergangene DoR-Prüfungen erzeugen Findings (F-NNN) gemäß ATC-STD-BUG-001.

## Security Considerations

Sicherheitsrelevante Anforderungen MÜSSEN als nicht-funktionale Sicherheitsanforderungen
(NFR-SEC) klassifiziert und mit erhöhter Priorität (P0/P1 nach Master §10) geführt werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000; **INFORMATIVE:** SCR-0042, ATC-STD-202, ATC-STD-V2S-003, ATC-STD-V2S-005
