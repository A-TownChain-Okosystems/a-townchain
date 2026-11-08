---
standard:
  id: ATC-STD-V2S-010
  title: "Integration Standard Standard"
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

# ATC-STD-V2S-010 — Integration Standard Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45; Range
> ATC-STD-V2S-000..026). Dockt direkt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-010 regelt die Zusammenführung von Softwarekomponenten, Modulen, Services und Subsystemen zu einem lauffähigen Gesamtsystem im Vision-to-Software Lifecycle. Er beschreibt den Übergang von der isolierten Modulentwicklung zur Systemintegration, definiert die Anforderungen an Schnittstellenverträge und bildet die normative Grundlage für das Quality Gate V2S-G4.

## Scope

**Gilt für:** Alle ATC-Projekte, Repositories, Protokolle, Smart Contracts, KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten gemäß dem Anwendungsbereich von ATC-STD-V2S-000.
**Nicht-Gilt:** Rein dokumentarische Artefakte ohne Software-Ziel oder eigenständigen Quellcode (z.B. reine Spezifikationsdokumente, Dokumentations-Wikis — diese folgen ATC-STD-000 bzw. ATC-STD-DESC-001).

## §1 Gegenstand & Verortung

Dieser Standard ordnet die Phase **Integration** verbindlich in den Vision-to-Software Lifecycle ein (Station 9 (INTEGRATION)).
Er stellt das Bindeglied zwischen den vorangehenden Entwicklungs- und Testschritten und den nachgelagerten Qualitäts- bzw. Veröffentlichungsphasen dar.

Die wesentliche Aufgabe besteht darin, das Erreichen des Meilensteins Komponenten zusammenführen; V2S-G4 systematisch und nachvollziehbar sicherzustellen. Eine saubere Abgrenzung zu den benachbarten Lifecycle-Stationen garantiert, dass keine Prozessschritte übersprungen werden und die festgelegten Qualitätsanforderungen lückenlos nachgewiesen werden.

## §2 Kernregeln

1. **KR-1:** Alle zur Integration anstehenden Komponenten MÜSSEN vor Beginn der Zusammenführung erfolgreich entwickelt und durch Unit-Tests auf Modulebene verifiziert sein.
2. **KR-2:** Die Schnittstellen und Datenverträge zwischen integrierenden Komponenten MÜSSEN den in den Architecture Decision Records (ADR, ATC-STD-201) festgelegten Spezifikationen entsprechen.
3. **KR-3:** Für jede Integration von Komponenten MUSS ein automatisierter Integrations-Build in der CI/CD-Pipeline ausgeführt werden, der ohne Fehler oder Warnungen abschließt.
4. **KR-4:** Tritt bei der Zusammenführung ein Schnittstellenkonflikt oder ein Kompatibilitätsbruch auf, MUSS die Integration unverzüglich gestoppt und ein Finding (F-NNN nach ATC-STD-BUG-001) erfasst werden.
5. **KR-5:** Es MUSS ein maschinenlesbarer Integration-Test-Report erstellt werden, der alle Schnittstellenaufrufe und Datenflüsse zwischen den Modulen protokolliert.
6. **KR-6:** Das Quality Gate V2S-G4 DARF NICHT erteilt werden, wenn offene P0- oder P1-Integrationsfehler oder ungelöste Schnittstellenabweichungen vorliegen.

## §3 Artefakte & Nachweispflichten

Für die ordnungsgemäße Durchführung der Phase **Integration** MÜSSEN die folgenden Artefakte vollständig erstellt, im Repository abgelegt und der Qualitätskontrolle vorgelegt werden:

- **Integration-Build-Log:** Automatisierter Nachweis der fehlerfreien Zusammenstellung aller Komponenten aus der CI/CD-Pipeline.
- **Interface-Compatibility-Matrix:** Dokumentierte Zuordnung der Modulschnittstellen und deren Kompatibilitätsstatus.
- **Integration-Test-Report:** Protokoll aller automatisierten Integrations- und Vertragstests.
- **Gate-Record V2S-G4:** Formeller Prüfbefund mit Prüfer, Datum, Testnachweisen und Ergebnis (PASSED/FAILED).

Alle genannten Artefakte MÜSSEN versionsgeführt und gegen unbefugte nachträgliche Modifikationen geschützt werden.

## §4 Quality-Gate-Bezug

Die Phase **Integration** gipfelt im bzw. bezieht sich auf das Quality Gate **V2S-G4** (*Funktionieren die Komponenten zusammen?*).

1. **Gate-Record Pflicht:** Für jedes Durchlaufen des Quality Gates MUSS ein vollständiger Gate-Record erstellt werden. Dieser MUSS mindestens die folgenden Felder enthalten:
   - **Prüfer:** Name/ID der prüfenden Person oder des autonomen Prüf-Agenten.
   - **Datum:** ISO-8601-Zeitstempel der Prüfung.
   - **Ergebnis:** PASSED, FAILED oder BLOCKED.
   - **Nachweise:** Eindeutige Links/Hashes zu den in §3 geforderten Prüfartefakten.
2. **Blockierungsmodell:** Offene Fehler der Schweregrade P0 oder P1 (nach ATC-STD-BUG-001) blockieren das Erteilen der Gate-Freigabe absolut. Das Passieren des Gates mit offenen P0/P1-Mängeln ist strengstens VERBOTEN (MUST NOT).
3. **Maturity-Level:** Das Erreichen dieses Gates ist Voraussetzung für das Aufrücken auf die nächste Reifegradstufe im Lifecycle-State (ATC-STD-V2S-000 §14).

## §5 Schnittstellen

- **Upstream / Downstream:** Upstream: ATC-STD-V2S-000 (Lifecycle Master), ATC-STD-201 (Architektur/ADR), ATC-STD-202 (Spezifikation) · Downstream: ATC-STD-V2S-011 (Testing Standard) · Andock-Standards: ATC-STD-BUG-001 (Fehlerbehandlung).
- **Master-Standard:** ATC-STD-V2S-000 dient als oberster normativer Rahmen für alle Phasen-Prozesse.
- **Schnittstellenüberwachung:** Schnittstellenänderungen zu Nachbarphasen MÜSSEN über Änderungsanträge (SCR-0042 / ATC-STD-CHANGE-001) koordiniert werden.

## Metriken & Akzeptanzkriterien

Die Einhaltung der Vorgaben dieses Standards wird anhand der folgenden quantitativen Metriken überprüft:

- **M1:** 100% Erfolgsquote automatisierter Integrations-Builds in der CI/CD-Pipeline.
- **M2:** 0 offene Integrationsfehler der Schweregrade P0 und P1.
- **M3:** Abdeckung der definierten Modulschnittstellen durch Vertragstests >= 95%.

## Compliance & Verifikation

Die Compliance mit diesem Standard wird im Rahmen automatisierter Repository-Audits (atc-repo-audit) sowie durch den Standard-Validator (`tools/atc-std-validator/atc_std_validator.py`) anhand der Prüfkriterien S-01 bis S-25 überprüft:

- **Normverstöße:** Abweichungen von MUSS-Kriterien werden als Findings der Klasse F-NNN erfasst und nach ATC-STD-BUG-001 klassifiziert.
- **Prüfzyklus:** Die Compliance-Prüfung erfolgt kontinuierlich in der CI/CD-Pipeline bei jedem Merge-Request sowie im Rahmen von Perioden-Audits.

## Security Considerations

Sicherheitsrelevante Schnittstellen (z.B. Authentifizierung, Datenverschlüsselung bei Inter-Modul-Kommunikation) MÜSSEN während der Integration separat auf Richtigkeit und Einhaltung von Sicherheitsrichtlinien geprüft werden. Unverschlüsselte Datenpfade zwischen Subsystemen sind MUSS-Verstöße.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-201, ATC-STD-202, ATC-STD-BUG-001  
**INFORMATIVE:** SCR-0042, ATC-STD-V2S-000 (V2S Lifecycle Master)
