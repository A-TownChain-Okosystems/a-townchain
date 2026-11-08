---
standard:
  id: ATC-STD-V2S-011
  title: "Testing Standard Standard"
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

# ATC-STD-V2S-011 — Testing Standard Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45; Range
> ATC-STD-V2S-000..026). Dockt direkt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-011 definiert die Anforderungen an die Teststrategie, Testplanung und Testdurchführung über alle Testebenen (Unit-, Integrations-, System- und End-to-End-Tests) im Vision-to-Software Lifecycle. Er verankert das Qualitätsprinzip automatisierter Tests, legt Metriken zur Testabdeckung fest und regelt die Kopplung an das Fehlerklassifizierungssystem ATC-STD-BUG-001 sowie das Quality Gate V2S-G5.

## Scope

**Gilt für:** Alle ATC-Projekte, Repositories, Protokolle, Smart Contracts, KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten gemäß dem Anwendungsbereich von ATC-STD-V2S-000.
**Nicht-Gilt:** Rein dokumentarische Artefakte ohne Software-Ziel oder eigenständigen Quellcode (z.B. reine Spezifikationsdokumente, Dokumentations-Wikis — diese folgen ATC-STD-000 bzw. ATC-STD-DESC-001).

## §1 Gegenstand & Verortung

Dieser Standard ordnet die Phase **Testing** verbindlich in den Vision-to-Software Lifecycle ein (Station 10 (TESTING)).
Er stellt das Bindeglied zwischen den vorangehenden Entwicklungs- und Testschritten und den nachgelagerten Qualitäts- bzw. Veröffentlichungsphasen dar.

Die wesentliche Aufgabe besteht darin, das Erreichen des Meilensteins Teststrategie und -durchführung; Unit/Integration/System/E2E; V2S-G5; Kopplung ATC-STD-BUG-001 systematisch und nachvollziehbar sicherzustellen. Eine saubere Abgrenzung zu den benachbarten Lifecycle-Stationen garantiert, dass keine Prozessschritte übersprungen werden und die festgelegten Qualitätsanforderungen lückenlos nachgewiesen werden.

## §2 Kernregeln

1. **KR-1:** Für jedes Softwareprojekt MUSS eine mehrstufige Teststrategie dokumentiert sein, welche Unit-, Integrations-, System- und End-to-End-Testebenen verbindlich abdeckt.
2. **KR-2:** Sämtliche Testsuiten MÜSSEN vollautomatisiert in der CI/CD-Pipeline ausführbar sein und bei jedem Pull Request sowie Haupt-Build gestartet werden.
3. **KR-3:** Gefundene Testabweichungen und Regressionen MÜSSEN unverzüglich als Findings nach ATC-STD-BUG-001 klassifiziert und den Schweregraden S0 bis S4 bzw. Prioritäten P0 bis P3 zugeordnet werden.
4. **KR-4:** Es MUSS eine automatisierte Code-Coverage-Messung durchgeführt werden, bei der eine Gesamtabdeckung von mindestens 80% auf Zeilenebene nachgewiesen wird.
5. **KR-5:** Alle Testergebnisse MÜSSEN in einem standardisierten, maschinenlesbaren Test-Report (z.B. JUnit XML oder JSON) dauerhaft archiviert werden.
6. **KR-6:** Das Quality Gate V2S-G5 MUSS verweigert werden, solange mindestens ein P0- oder P1-Testfehler ungelöst im Issue-Tracker verbleibt.

## §3 Artefakte & Nachweispflichten

Für die ordnungsgemäße Durchführung der Phase **Testing** MÜSSEN die folgenden Artefakte vollständig erstellt, im Repository abgelegt und der Qualitätskontrolle vorgelegt werden:

- **Test-Strategie & Test-Plan:** Dokumentierte Definition der Testebenen, Testumgebungen und Testdatenstrategie.
- **Automatisierte Test-Suites:** Quellcode der Unit-, Integrations-, System- und E2E-Tests.
- **Test-Execution-Report:** Maschinenlesbares Protokoll aller ausgeführen Tests inklusive Pass/Fail-Status.
- **Coverage-Report:** Nachweis der Code-Abdeckung auf Zeilen- und Zweigebene.
- **Gate-Record V2S-G5:** Formeller Prüfbefund für den Abschluss der technischen Testphase.

Alle genannten Artefakte MÜSSEN versionsgeführt und gegen unbefugte nachträgliche Modifikationen geschützt werden.

## §4 Quality-Gate-Bezug

Die Phase **Testing** gipfelt im bzw. bezieht sich auf das Quality Gate **V2S-G5** (*Sind die technischen Tests erfolgreich?*).

1. **Gate-Record Pflicht:** Für jedes Durchlaufen des Quality Gates MUSS ein vollständiger Gate-Record erstellt werden. Dieser MUSS mindestens die folgenden Felder enthalten:
   - **Prüfer:** Name/ID der prüfenden Person oder des autonomen Prüf-Agenten.
   - **Datum:** ISO-8601-Zeitstempel der Prüfung.
   - **Ergebnis:** PASSED, FAILED oder BLOCKED.
   - **Nachweise:** Eindeutige Links/Hashes zu den in §3 geforderten Prüfartefakten.
2. **Blockierungsmodell:** Offene Fehler der Schweregrade P0 oder P1 (nach ATC-STD-BUG-001) blockieren das Erteilen der Gate-Freigabe absolut. Das Passieren des Gates mit offenen P0/P1-Mängeln ist strengstens VERBOTEN (MUST NOT).
3. **Maturity-Level:** Das Erreichen dieses Gates ist Voraussetzung für das Aufrücken auf die nächste Reifegradstufe im Lifecycle-State (ATC-STD-V2S-000 §14).

## §5 Schnittstellen

- **Upstream / Downstream:** Upstream: ATC-STD-V2S-010 (Integration Standard) · Downstream: ATC-STD-V2S-012 (Verification Standard), ATC-STD-V2S-013 (Validation Standard) · Andock-Standards: ATC-STD-BUG-001 (Fehlerbehandlung).
- **Master-Standard:** ATC-STD-V2S-000 dient als oberster normativer Rahmen für alle Phasen-Prozesse.
- **Schnittstellenüberwachung:** Schnittstellenänderungen zu Nachbarphasen MÜSSEN über Änderungsanträge (SCR-0042 / ATC-STD-CHANGE-001) koordiniert werden.

## Metriken & Akzeptanzkriterien

Die Einhaltung der Vorgaben dieses Standards wird anhand der folgenden quantitativen Metriken überprüft:

- **M1:** Code-Coverage auf Zeilenebene >= 80% über das Gesamtsystem.
- **M2:** Pass-Rate der automatisierten Testsuite = 100% für P0/P1-relevante Testfälle.
- **M3:** Ausführungsdauer der gesamten automatisierten Regressions-Testsuite < 30 Minuten.

## Compliance & Verifikation

Die Compliance mit diesem Standard wird im Rahmen automatisierter Repository-Audits (atc-repo-audit) sowie durch den Standard-Validator (`tools/atc-std-validator/atc_std_validator.py`) anhand der Prüfkriterien S-01 bis S-25 überprüft:

- **Normverstöße:** Abweichungen von MUSS-Kriterien werden als Findings der Klasse F-NNN erfasst und nach ATC-STD-BUG-001 klassifiziert.
- **Prüfzyklus:** Die Compliance-Prüfung erfolgt kontinuierlich in der CI/CD-Pipeline bei jedem Merge-Request sowie im Rahmen von Perioden-Audits.

## Security Considerations

Testumgebungen und Testdaten MÜSSEN frei von produktiven personenbezogenen Daten oder echten Secrets sein. Für Tests erforderliche Zugangsdaten MÜSSEN über dedizierte Test-Vaults bereitgestellt werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-V2S-010, ATC-STD-BUG-001  
**INFORMATIVE:** SCR-0042, ATC-STD-V2S-000 (V2S Lifecycle Master)
