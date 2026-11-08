---
standard:
  id: ATC-STD-V2S-014
  title: "Security Standard Standard"
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

# ATC-STD-V2S-014 — Security Standard Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45; Range
> ATC-STD-V2S-000..026). Dockt direkt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-014 bestimmt die verbindlichen Sicherheitsprüfungen und Bedrohungsanalysen im Vision-to-Software Lifecycle. Er regelt die Erstellung von Threat-Modellen, die Durchführung statischer und dynamischer Sicherheitsanalysen (SAST/DAST), die Absicherung von Netzelementen gemäß ATC-STD-NET-007 und die Einhaltung der Sicherheitsrichtlinien aus ATC-STD-314 als Voraussetzung für das Quality Gate V2S-G7.

## Scope

**Gilt für:** Alle ATC-Projekte, Repositories, Protokolle, Smart Contracts, KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten gemäß dem Anwendungsbereich von ATC-STD-V2S-000.
**Nicht-Gilt:** Rein dokumentarische Artefakte ohne Software-Ziel oder eigenständigen Quellcode (z.B. reine Spezifikationsdokumente, Dokumentations-Wikis — diese folgen ATC-STD-000 bzw. ATC-STD-DESC-001).

## §1 Gegenstand & Verortung

Dieser Standard ordnet die Phase **Security** verbindlich in den Vision-to-Software Lifecycle ein (Station 11 (SECURITY/AUDIT)).
Er stellt das Bindeglied zwischen den vorangehenden Entwicklungs- und Testschritten und den nachgelagerten Qualitäts- bzw. Veröffentlichungsphasen dar.

Die wesentliche Aufgabe besteht darin, das Erreichen des Meilensteins Sicherheitsprüfung; V2S-G7; Kopplung ATC-STD-314, ATC-STD-NET-007, Threat-Modelle systematisch und nachvollziehbar sicherzustellen. Eine saubere Abgrenzung zu den benachbarten Lifecycle-Stationen garantiert, dass keine Prozessschritte übersprungen werden und die festgelegten Qualitätsanforderungen lückenlos nachgewiesen werden.

## §2 Kernregeln

1. **KR-1:** Für jedes Software-System MUSS vor dem Release Candidate ein dokumentiertes Threat Model (nach STRIDE oder PASTA) erstellt und bei wesentlichen Architekturänderungen aktualisiert werden.
2. **KR-2:** Automatische Statische (SAST) und Dynamische (DAST) Sicherheits-Analysen MÜSSEN nach ATC-STD-314 in der CI/CD-Pipeline integriert sein und fehlerfrei durchlaufen.
3. **KR-3:** Alle Netzwerk-Schnittstellen, P2P-Protokolle und RPC-Endpunkte MÜSSEN den Härtungsvorgaben von ATC-STD-NET-007 entsprechen.
4. **KR-4:** Verwendete Drittanbieter-Bibliotheken und Abhängigkeiten MÜSSEN automatisiert auf bekannte Schwachstellen (CVE) gescannt werden; bekannte kritische oder hohe Schwachstellen (Critical/High) sind VERBOTEN.
5. **KR-5:** Das Ergebnis aller Sicherheitsprüfungen MUSS in einem formalen Security Audit Report (SEC-Record) festgehalten werden.
6. **KR-6:** Das Quality Gate V2S-G7 MUSS verweigert werden, solange ungelöste Sicherheitsrisiken der Stufen P0 oder P1 oder offene High/Critical CVEs vorliegen.

## §3 Artefakte & Nachweispflichten

Für die ordnungsgemäße Durchführung der Phase **Security** MÜSSEN die folgenden Artefakte vollständig erstellt, im Repository abgelegt und der Qualitätskontrolle vorgelegt werden:

- **Threat Model Dokumentation:** Analyse von Angriffsflächen, Bedrohungsszenarien und Gegenmaßnahmen.
- **SAST/DAST Scan Reports:** Automatisierte Auswertungen der Quellcode- und Laufzeitsicherheitsprüfungen.
- **Dependency Vulnerability Audit:** Prüfprotokoll über Drittkomponenten und bekannte CVEs.
- **Security Audit Record (SEC-Record):** Signiertes Sicherheitsgutachten der Governance-Instanz.
- **Gate-Record V2S-G7:** Formeller Freigabebefund für das Security Gate.

Alle genannten Artefakte MÜSSEN versionsgeführt und gegen unbefugte nachträgliche Modifikationen geschützt werden.

## §4 Quality-Gate-Bezug

Die Phase **Security** gipfelt im bzw. bezieht sich auf das Quality Gate **V2S-G7** (*Sind Sicherheitsrisiken bewertet?*).

1. **Gate-Record Pflicht:** Für jedes Durchlaufen des Quality Gates MUSS ein vollständiger Gate-Record erstellt werden. Dieser MUSS mindestens die folgenden Felder enthalten:
   - **Prüfer:** Name/ID der prüfenden Person oder des autonomen Prüf-Agenten.
   - **Datum:** ISO-8601-Zeitstempel der Prüfung.
   - **Ergebnis:** PASSED, FAILED oder BLOCKED.
   - **Nachweise:** Eindeutige Links/Hashes zu den in §3 geforderten Prüfartefakten.
2. **Blockierungsmodell:** Offene Fehler der Schweregrade P0 oder P1 (nach ATC-STD-BUG-001) blockieren das Erteilen der Gate-Freigabe absolut. Das Passieren des Gates mit offenen P0/P1-Mängeln ist strengstens VERBOTEN (MUST NOT).
3. **Maturity-Level:** Das Erreichen dieses Gates ist Voraussetzung für das Aufrücken auf die nächste Reifegradstufe im Lifecycle-State (ATC-STD-V2S-000 §14).

## §5 Schnittstellen

- **Upstream / Downstream:** Upstream: ATC-STD-V2S-013 (Validation Standard) · Downstream: ATC-STD-V2S-015 (Software Audit Standard), ATC-STD-V2S-016 (Release Candidate Standard) · Andock-Standards: ATC-STD-314 (Security Controls), ATC-STD-NET-007 (Network Security), ATC-STD-BUG-001.
- **Master-Standard:** ATC-STD-V2S-000 dient als oberster normativer Rahmen für alle Phasen-Prozesse.
- **Schnittstellenüberwachung:** Schnittstellenänderungen zu Nachbarphasen MÜSSEN über Änderungsanträge (SCR-0042 / ATC-STD-CHANGE-001) koordiniert werden.

## Metriken & Akzeptanzkriterien

Die Einhaltung der Vorgaben dieses Standards wird anhand der folgenden quantitativen Metriken überprüft:

- **M1:** 0 ungelöste Sicherheitslücken der Schweregrade Critical und High (CVE / SAST / DAST).
- **M2:** 100%ige Abdeckung der im Threat Model identifizierten Bedrohungen durch Gegenmaßnahmen.
- **M3:** Automated Security Scan Pass-Rate = 100% in der CI/CD-Pipeline.

## Compliance & Verifikation

Die Compliance mit diesem Standard wird im Rahmen automatisierter Repository-Audits (atc-repo-audit) sowie durch den Standard-Validator (`tools/atc-std-validator/atc_std_validator.py`) anhand der Prüfkriterien S-01 bis S-25 überprüft:

- **Normverstöße:** Abweichungen von MUSS-Kriterien werden als Findings der Klasse F-NNN erfasst und nach ATC-STD-BUG-001 klassifiziert.
- **Prüfzyklus:** Die Compliance-Prüfung erfolgt kontinuierlich in der CI/CD-Pipeline bei jedem Merge-Request sowie im Rahmen von Perioden-Audits.

## Security Considerations

Sicherheit ist ein zentrales Qualitätsmerkmal. Ein Ignorieren oder temporäres Aussetzen von Sicherheitskontrollen über Waiver ist bei P0-Risiken strengstens VERBOTEN (MUST NOT).

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-314, ATC-STD-NET-007, ATC-STD-BUG-001  
**INFORMATIVE:** SCR-0042, ATC-STD-V2S-000 (V2S Lifecycle Master)
