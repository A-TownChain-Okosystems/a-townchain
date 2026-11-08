---
standard:
  id: ATC-STD-V2S-023
  title: "Improvement Standard"
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

# ATC-STD-V2S-023 — Improvement Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-023 regelt den Prozess der kontinuierlichen Verbesserung (Continuous Improvement)
im ATC Vision-to-Software Lifecycle. Der Standard spezifiziert den geschlossenen Regelkreis
aus Master §13 (`FEEDBACK → CHANGE REQUEST → IMPACT ANALYSIS → NEW REQUIREMENT → NEXT VERSION`)
und stellt sicher, dass Erkenntnisse strukturiert in neue Anforderungen und Folgereleases einfließen.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** Alle kontinuierlichen Weiterentwicklungen, Optimierungen und Refactorings
bestehender ATC-Softwareprodukte, Smart Contracts und KI-Systeme.
**Nicht-Gilt:** Erstmalige Neuentwicklungen vor Version v1.0.0 (diese starten bei V2S-001).

## §1 Gegenstand & Verortung

Improvement ist Station 18 im V2S-Lifecycle (ATC-STD-V2S-000 §1). Es transformiert das in
V2S-022 gesammelte Feedback über eine fundierte Impact-Analyse in neue, operationalisierbare
Anforderungen und leitet damit geordnet die Planung der nächsten Version (V2S-024 / Version
Upgrade Standard) ein.

Die verbindliche Prozesskette lautet:
`FEEDBACK → CHANGE REQUEST → IMPACT ANALYSIS → NEW REQUIREMENT → NEXT VERSION`

- **FEEDBACK:** Konsolidierte Daten aus den 7 Quellen (V2S-022).
- **CHANGE REQUEST:** Formaler Änderungsantrag (ATC-STD-UPDATE-001).
- **IMPACT ANALYSIS:** Bewertung von Architektur, Security, Abwärtskompatibilität & Aufwand.
- **NEW REQUIREMENT:** Neue Anforderungen mit eindeutiger ID & Acceptance Criteria.
- **NEXT VERSION:** Einplanung in die Roadmap für den nächsten V2S-Durchlauf.

## §2 Kernregeln

1. **KR-1 (Verbindliche Verbesserungsschleife):** Jede Produktverbesserung MUSS exakt die Kette `FEEDBACK → CHANGE REQUEST → IMPACT ANALYSIS → NEW REQUIREMENT → NEXT VERSION` durchlaufen. Abkürzungen ohne Impact-Analyse sind VERBOTEN (MUST NOT).
2. **KR-2 (Verpflichtende Impact-Analyse):** Vor der Ableitung neuer Anforderungen MUSS eine schriftliche Impact-Analyse durchgeführt werden, die Auswirkungen auf Sicherheit, Performance, Protokoll-Kompatibilität und bestehende Schnittstellen bewertet.
3. **KR-3 (Ableitung neuer Requirements):** Aus jedem akzeptierten Change Request MÜSSEN formale Anforderungen mit eindeutiger ID und operationalisierbaren Acceptance Criteria gemäß ATC-STD-V2S-000 §2 erstellt werden.
4. **KR-4 (Backlog-Priorisierung nach P0-P3):** Neue Anforderungen MÜSSEN im Backlog gemäß dem P0-P3 Blockierungsmodell (ATC-STD-V2S-000 §10) sowie nach geschäftlichem Nutzwert eingestuft und eingeplant werden.
5. **KR-5 (Geschlossene Traceability-Kette):** Der Pfad vom auslösenden Feedback über den Change Request bis zum neuen Requirement und dessen Code-Implementierung MUSS lückenlos in der Traceability-Matrix nachweisbar sein.
6. **KR-6 (Maturity-Level M9 Qualifikation):** Ein Produkt erreicht und behält den Reifegrad `M9` (MATURE) gemäß ATC-STD-V2S-000 §14 nur, wenn die Improvement-Schleife nachweislich aktiv und kontinuierlich durchlaufen wird.

## §3 Artefakte & Nachweispflichten

- **Impact Analysis Report:** Dokumentierte Bewertung der Risiken, Abhängigkeiten und Architektur-Konsequenzen eines Change Requests.
- **Requirements Update Document:** Neue oder überarbeitete Spezifikations-Artefakte mit Test-IDs.
- **Priorisiertes Product Backlog:** Maschinenlesbares Backlog mit P0-P3 Zuordnungen.
- **Closed-Loop Traceability Record:** Nachweis der Verknüpfung Feedback → CR → Requirement → Build.

## §4 Lifecycle-State-Bezug

Der Standard pflegt und aktualisiert den maschinenlesbaren Lifecycle-State
(`.atc/lifecycle-state.yaml`) gemäß ATC-STD-V2S-000 §16:

- Sets `lifecycle.improvement_status`: `active` | `evaluating` | `idle`
- Sets `lifecycle.maturity`: `M9` (MATURE) bei aktiver Verbesserungsschleife
- Tracks `lifecycle.active_change_requests_count`: Anzahl in Bearbeitung befindlicher CRs

## §5 Schnittstellen

- **Upstream:** ATC-STD-V2S-022 (Feedback Standard).
- **Downstream:** ATC-STD-V2S-024 (Version Upgrade Standard), ATC-STD-V2S-001 bis V2S-004 (Neuer Spezifikationsdurchlauf).
- **Andock-Standards:** ATC-STD-UPDATE-001 (Change Management), ATC-STD-202 (Requirements Engineering).

## Metriken & Akzeptanzkriterien

- **M1 (Impact Analysis Coverage):** 100 % aller zugelassenen Change Requests besitzen einen freigegebenen Impact Analysis Report.
- **M2 (Requirement Traceability Rate):** 100 % der abgeleiteten neuen Requirements sind auf eine konkrete CR- und Feedback-ID zurückführbar.
- **M3 (Closed-Loop Cycle Time):** Die durchschnittliche Durchlaufzeit von akzeptiertem Feedback bis zur Requirement-Spezifikation beträgt < 30 Tage.
- **M4 (Maturity M9 Verification):** 100 % der als M9 eingestuften Repositories weisen mindestens einen aktiven Improvement-Durchlauf pro Halbjahr nach.

## Compliance & Verifikation

Die Einhaltung wird durch atc-repo-audit, Traceability-Matrix-Validierung und Governance-
Reviews verifiziert (S-01..S-25). Änderungen ohne Impact-Analyse oder nicht rückverfolgbare
Requirements erzeugen Findings (F-001..F-999) nach ATC-STD-BUG-001.

## Security Considerations

Verbesserungsprozesse bergen das Risiko des "Architectural Drift" und unbeabsichtigter
Sicherheitslücken. Die Impact-Analyse MUSS zwingend eine erneute Sicherheitsbewertung
(Security Re-Assessment) umfassen, um festzustellen, ob bestehende Threat Models oder
Kryptographie-Annahmen berührt werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-UPDATE-001, ATC-STD-202
**INFORMATIVE:** SCR-0042
