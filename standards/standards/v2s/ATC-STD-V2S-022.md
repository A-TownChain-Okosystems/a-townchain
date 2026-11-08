---
standard:
  id: ATC-STD-V2S-022
  title: "Feedback Standard"
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

# ATC-STD-V2S-022 — Feedback Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-022 regelt die systematische Erfassung, Strukturierung und Bewertung von Nutzer-
und Systemfeedback im ATC Vision-to-Software Lifecycle. Der Standard spezifiziert die 7
verbindlichen Feedback-Quellen aus Master §13 und steuert deren Transformation in formale
Change Requests (CR) zur kontinuierlichen Produktverbesserung.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** Alle Rückmeldungen, Telemetrieauswertungen, Incident-Erkenntnisse und Sicherheits-
Audits zu produktiven ATC-Softwareprodukten, Smart Contracts und KI-Systemen.
**Nicht-Gilt:** Unstrukturierte Gerüchte oder inoffizielle Diskussionen außerhalb der autorisierten Kanäle.

## §1 Gegenstand & Verortung

Feedback ist Station 17 im V2S-Lifecycle (ATC-STD-V2S-000 §1). Es sammelt Erkenntnisse aus
dem Betrieb (V2S-019), dem Monitoring (V2S-020) und Vorfällen (V2S-021) und bereitet diese
für die kontinuierliche Verbesserung (V2S-023 / Improvement Standard) auf.

Die 7 verbindlichen Feedback-Quellen gemäß ATC-STD-V2S-000 §13 sind:
1. **User Feedback:** Nutzer-Rückmeldungen, Support-Tickets, Community-Foren.
2. **System Metrics:** Metrik- und Telemetriedaten zur Nutzung und Fehlerraten.
3. **Incidents & Postmortems:** Erkenntnisse und Action Items aus Vorfällen (V2S-021).
4. **Security Findings:** Befunde aus Penetrationstests, Code-Audits und Vulnerability Scans.
5. **Performance Analytics:** Kapazitäts- und Performance-Analysen aus dem Betrieb.
6. **New Requirements:** Neue geschäftliche, funktionale oder Markt-Anforderungen.
7. **Regulatory & Compliance:** Änderungen bei rechtlichen Vorgaben und Governance-Standards.

## §2 Kernregeln

1. **KR-1 (7-Quellen-Erfassung):** Das Feedback-System MUSS alle 7 Feedback-Quellen nach ATC-STD-V2S-000 §13 kontinuierlich beobachten und strukturierte Eingaben verarbeiten.
2. **KR-2 (Zentrales Feedback-Register):** Jedes empfangene Feedback MUSS mit einer eindeutigen Feedback-ID in einem maschinenlesbaren Feedback-Register (z.B. `.atc/feedback-register.yaml`) protokolliert werden.
3. **KR-3 (Bewertung & Triage SLA):** Eingegangenes Feedback MUSS innerhalb von 5 Werktagen hinsichtlich Relevanz, Machbarkeit und Auswirkung getriagt werden.
4. **KR-4 (Change Request Pflicht):** Akzeptiertes Feedback MUSS zwingend in einen formalen Change Request (CR) gemäß ATC-STD-UPDATE-001 / ATC-STD-CHANGE-001 überführt werden.
5. **KR-5 (Closed-Loop Rückkopplung):** Bei externem User-Feedback MUSS das System den Feedback-Geber über den Status (eingegangen, geprüft, in Überarbeitung, abgelehnt) informieren.
6. **KR-6 (Datenschutz & Sanitization):** Alle Feedback-Daten MÜSSEN vor der Speicherung automatisch auf PII (Personenbezogene Daten) geprüft, sanitisiert und anonymisiert werden.

## §3 Artefakte & Nachweispflichten

- **Feedback-Register:** Maschinenlesbare Übersicht aller Feedbacks mit Quelle, Status, Priorität und CR-Referenz.
- **Feedback-Triage-Protokoll:** Dokumentation der Bewertungsentscheidungen inklusive Begründung bei Ablehnung.
- **Change Request Proposal:** Formaler Änderungsantrag gemäß ATC-STD-UPDATE-001.

## §4 Lifecycle-State-Bezug

Der Standard pflegt und aktualisiert den maschinenlesbaren Lifecycle-State
(`.atc/lifecycle-state.yaml`) gemäß ATC-STD-V2S-000 §16:

- Sets `lifecycle.feedback_loop`: `active` | `paused`
- Tracks `lifecycle.pending_feedback_count`: Anzahl offener Feedbacks
- Tracks `lifecycle.converted_change_requests_count`: Anzahl ersteller CRs

## §5 Schnittstellen

- **Upstream:** ATC-STD-V2S-019 (Operations), ATC-STD-V2S-020 (Monitoring), ATC-STD-V2S-021 (Incident).
- **Downstream:** ATC-STD-V2S-023 (Improvement Standard), ATC-STD-V2S-024 (Version Upgrade Standard).
- **Andock-Standards:** ATC-STD-UPDATE-001 (Change Management), ATC-STD-CHANGE-001 (Governance Changes).

## Metriken & Akzeptanzkriterien

- **M1 (7-Quellen Abdeckung):** 100 % der 7 Feedback-Quellen besitzen definierte automatische oder manuelle Erfassungskanäle.
- **M2 (Triage SLA Compliance):** ≥ 95 % aller Feedback-Eingänge werden innerhalb von 5 Werktagen getriagt.
- **M3 (CR Conversion Rate):** ≥ 80 % aller als "hoch" eingestuften Feedbacks werden in Change Requests konvertiert.
- **M4 (Traceability Rate):** 100 % der erzeugten Change Requests verweisen auf die auslösende Feedback-ID.

## Compliance & Verifikation

Die Einhaltung wird durch atc-repo-audit, Konsistenzprüfungen des Feedback-Registers
und Stichproben-Reviews verifiziert (S-01..S-25). Nicht bewertetes Feedback oder fehlende
CR-Überführungen erzeugen Findings (F-001..F-999) nach ATC-STD-BUG-001.

## Security Considerations

Feedback-Eingabefelder (z.B. User Support Formulare oder API-Endpoints) sind potenzielle
Vektoren für Prompt Injection, Cross-Site Scripting (XSS) und SQL Injection. Sämtliche
Feedback-Eingaben MÜSSEN streng sanitisiert und vor der Weiterverarbeitung in KI- oder
Datenbank-Systemen isoliert werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-UPDATE-001, ATC-STD-CHANGE-001
**INFORMATIVE:** SCR-0042
