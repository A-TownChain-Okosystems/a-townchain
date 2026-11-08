---
standard:
  id: ATC-STD-V2S-013
  title: "Validation Standard Standard"
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

# ATC-STD-V2S-013 — Validation Standard Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45; Range
> ATC-STD-V2S-000..026). Dockt direkt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-013 regelt die fachliche Validierung im Vision-to-Software Lifecycle. Während die Verifikation (ATC-STD-V2S-012) die technische Spezifikationstreue prüft, bestätigt die Validierung, dass das Gesamtsystem seinen vorgesehenen Zweck im realen Einsatzkontext erfüllt («Have we built the right system?»). Der Standard legt die Anforderungen an Akzeptanzszenarien, End-User-Validation und das Quality Gate V2S-G6 fest.

## Scope

**Gilt für:** Alle ATC-Projekte, Repositories, Protokolle, Smart Contracts, KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten gemäß dem Anwendungsbereich von ATC-STD-V2S-000.
**Nicht-Gilt:** Rein dokumentarische Artefakte ohne Software-Ziel oder eigenständigen Quellcode (z.B. reine Spezifikationsdokumente, Dokumentations-Wikis — diese folgen ATC-STD-000 bzw. ATC-STD-DESC-001).

## §1 Gegenstand & Verortung

Dieser Standard ordnet die Phase **Validation** verbindlich in den Vision-to-Software Lifecycle ein (Validierungsphase / Akzeptanzprüfung).
Er stellt das Bindeglied zwischen den vorangehenden Entwicklungs- und Testschritten und den nachgelagerten Qualitäts- bzw. Veröffentlichungsphasen dar.

Die wesentliche Aufgabe besteht darin, das Erreichen des Meilensteins erfüllt das System den vorgesehenen Zweck; V2S-G6; Akzeptanzszenarien systematisch und nachvollziehbar sicherzustellen. Eine saubere Abgrenzung zu den benachbarten Lifecycle-Stationen garantiert, dass keine Prozessschritte übersprungen werden und die festgelegten Qualitätsanforderungen lückenlos nachgewiesen werden.

## §2 Kernregeln

1. **KR-1:** Für jede aus der Vision abgeleitete fachliche Anforderung MÜSSEN verbindliche Akzeptanzszenarien (Acceptance Scenarios) mit klaren Gut- und Schlechtfällen definiert sein.
2. **KR-2:** Die Validierung MUSS in einer realistischen, produktionsnahen Staging-Umgebung unter Verwendung repräsentativer Nutzungsszenarien durchgeführt werden.
3. **KR-3:** Die Akzeptanzprüfungen MÜSSEN von den fachlich Verantwortlichen (Owner, Product Manager oder autorisierte Akzeptanz-Agenten) nachvollziehbar abgenommen und unterzeichnet werden.
4. **KR-4:** Nicht-erfüllte Akzeptanzkriterien oder fachliche Mängel MÜSSEN als Validierungs-Findings erfasst und kategorisiert werden.
5. **KR-5:** Das Gesamtergebnis der Validierung MUSS in einem formalen Validation Record (VAL-Record) fälschungssicher dokumentiert werden.
6. **KR-6:** Das Quality Gate V2S-G6 DARF NICHT erteilt werden, wenn Kern-Szenarien der fachlichen Nutzung fehlschlagen oder P0/P1-Akzeptanzblocker vorliegen.

## §3 Artefakte & Nachweispflichten

Für die ordnungsgemäße Durchführung der Phase **Validation** MÜSSEN die folgenden Artefakte vollständig erstellt, im Repository abgelegt und der Qualitätskontrolle vorgelegt werden:

- **Validation Plan:** Beschreibung der Validierungsstrategie, der Zielumgebung und der Beteiligten.
- **Acceptance Test Report / Scenarios:** Dokumentierte Durchführung aller Akzeptanzszenarien inklusive Ergebnissen.
- **Validation Record (VAL-Record):** Signiertes Dokument der fachlichen Freigabe des Systems.
- **Gate-Record V2S-G6:** Formeller Nachweis über das Erreichen von Gate V2S-G6.

Alle genannten Artefakte MÜSSEN versionsgeführt und gegen unbefugte nachträgliche Modifikationen geschützt werden.

## §4 Quality-Gate-Bezug

Die Phase **Validation** gipfelt im bzw. bezieht sich auf das Quality Gate **V2S-G6** (*Erfüllt das System den vorgesehenen Zweck?*).

1. **Gate-Record Pflicht:** Für jedes Durchlaufen des Quality Gates MUSS ein vollständiger Gate-Record erstellt werden. Dieser MUSS mindestens die folgenden Felder enthalten:
   - **Prüfer:** Name/ID der prüfenden Person oder des autonomen Prüf-Agenten.
   - **Datum:** ISO-8601-Zeitstempel der Prüfung.
   - **Ergebnis:** PASSED, FAILED oder BLOCKED.
   - **Nachweise:** Eindeutige Links/Hashes zu den in §3 geforderten Prüfartefakten.
2. **Blockierungsmodell:** Offene Fehler der Schweregrade P0 oder P1 (nach ATC-STD-BUG-001) blockieren das Erteilen der Gate-Freigabe absolut. Das Passieren des Gates mit offenen P0/P1-Mängeln ist strengstens VERBOTEN (MUST NOT).
3. **Maturity-Level:** Das Erreichen dieses Gates ist Voraussetzung für das Aufrücken auf die nächste Reifegradstufe im Lifecycle-State (ATC-STD-V2S-000 §14).

## §5 Schnittstellen

- **Upstream / Downstream:** Upstream: ATC-STD-V2S-012 (Verification Standard), ATC-STD-V2S-000 (Master) · Downstream: ATC-STD-V2S-014 (Security Standard) · Andock-Standards: ATC-STD-202, ATC-STD-BUG-001.
- **Master-Standard:** ATC-STD-V2S-000 dient als oberster normativer Rahmen für alle Phasen-Prozesse.
- **Schnittstellenüberwachung:** Schnittstellenänderungen zu Nachbarphasen MÜSSEN über Änderungsanträge (SCR-0042 / ATC-STD-CHANGE-001) koordiniert werden.

## Metriken & Akzeptanzkriterien

Die Einhaltung der Vorgaben dieses Standards wird anhand der folgenden quantitativen Metriken überprüft:

- **M1:** Erfüllungsquote aller definierten fachlichen Akzeptanzszenarien = 100%.
- **M2:** Erfolgreiche Unterzeichnung des Validation Records durch den Owner / Fachexperten.
- **M3:** 0 offene fachliche Akzeptanzblocker der Prioritäten P0 und P1.

## Compliance & Verifikation

Die Compliance mit diesem Standard wird im Rahmen automatisierter Repository-Audits (atc-repo-audit) sowie durch den Standard-Validator (`tools/atc-std-validator/atc_std_validator.py`) anhand der Prüfkriterien S-01 bis S-25 überprüft:

- **Normverstöße:** Abweichungen von MUSS-Kriterien werden als Findings der Klasse F-NNN erfasst und nach ATC-STD-BUG-001 klassifiziert.
- **Prüfzyklus:** Die Compliance-Prüfung erfolgt kontinuierlich in der CI/CD-Pipeline bei jedem Merge-Request sowie im Rahmen von Perioden-Audits.

## Security Considerations

Bei der Validierung in Staging-Umgebungen MUSS sichergestellt sein, dass verfälschte oder syntetisierte Datensätze verwendet werden. Ist die Nutzung von Realdaten unvermeidbar, MÜSSEN diese Anonymisierungsprozeduren durchlaufen haben.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-V2S-012, ATC-STD-202, ATC-STD-BUG-001  
**INFORMATIVE:** SCR-0042, ATC-STD-V2S-000 (V2S Lifecycle Master)
