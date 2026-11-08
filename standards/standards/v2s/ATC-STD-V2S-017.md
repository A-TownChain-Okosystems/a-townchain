---
standard:
  id: ATC-STD-V2S-017
  title: "Release Standard Standard"
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

# ATC-STD-V2S-017 — Release Standard Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45; Range
> ATC-STD-V2S-000..026). Dockt direkt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-017 regelt die finale Veröffentlichung und Freigabe von Software-Projekten im Vision-to-Software Lifecycle. Er beschreibt das Passing des Quality Gates V2S-G8, fordert die Übergabe der 9 Pflichtartefakte einschließlich eines automatisierten Rollback-Plans, garantiert die zeitliche und technische Reproduzierbarkeit des Builds und verzahnt das Release mit ATC-STD-203 (GATE-001..010) sowie ATC-STD-VERSION-001.

## Scope

**Gilt für:** Alle ATC-Projekte, Repositories, Protokolle, Smart Contracts, KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten gemäß dem Anwendungsbereich von ATC-STD-V2S-000.
**Nicht-Gilt:** Rein dokumentarische Artefakte ohne Software-Ziel oder eigenständigen Quellcode (z.B. reine Spezifikationsdokumente, Dokumentations-Wikis — diese folgen ATC-STD-000 bzw. ATC-STD-DESC-001).

## §1 Gegenstand & Verortung

Dieser Standard ordnet die Phase **Release** verbindlich in den Vision-to-Software Lifecycle ein (Station 13 (RELEASE)).
Er stellt das Bindeglied zwischen den vorangehenden Entwicklungs- und Testschritten und den nachgelagerten Qualitäts- bzw. Veröffentlichungsphasen dar.

Die wesentliche Aufgabe besteht darin, das Erreichen des Meilensteins Release; V2S-G8; 9 Pflichtartefakte inkl. Rollback-Plan; Reproduzierbarkeit; Kopplung ATC-STD-203/GATE-001..010, ATC-STD-VERSION-001 systematisch und nachvollziehbar sicherzustellen. Eine saubere Abgrenzung zu den benachbarten Lifecycle-Stationen garantiert, dass keine Prozessschritte übersprungen werden und die festgelegten Qualitätsanforderungen lückenlos nachgewiesen werden.

## §2 Kernregeln

1. **KR-1:** Ein offizielles Release DARF NUR aus einem erfolgreich genehmigten Release Candidate (ATC-STD-V2S-016) hervorgehen; direkte Releases aus Unstable-Branches sind VERBOTEN.
2. **KR-2:** Jedes Release MUSS exakt die 9 Pflichtartefakte gemäß ATC-STD-V2S-000 §12 enthalten: (1) Release Artifact, (2) Version Tag, (3) SHA-256 Checksum Manifest, (4) CHANGELOG.md Update, (5) Release Notes, (6) Configuration Specification, (7) Deployment Instructions, (8) Rollback Plan, (9) Monitoring & Incident Plan.
3. **KR-3:** Der Build-Prozess MUSS vollständig deterministisch und reproduzierbar sein (Reproducible Build verification).
4. **KR-4:** Jedes Release MUSS einen getesteten, automatisierten Rollback-Plan beinhalten, der im Fehlerfall die Wiederherstellung des vorherigen stabilen Zustands (vN-1) garantiert.
5. **KR-5:** Das Quality Gate V2S-G8 MUSS bestanden und im Gate-Record dokumentiert sein (unter Berücksichtigung der Release-Gates GATE-001..010 gemäß ATC-STD-203).
6. **KR-6:** Mit Veröffentlichung des Releases MUSS der Lifecycle-State (ATC-STD-V2S-000 §16) auf release: approved aktualisiert und die Reifegradstufe M7 (RELEASED) gesetzt werden.

## §3 Artefakte & Nachweispflichten

Für die ordnungsgemäße Durchführung der Phase **Release** MÜSSEN die folgenden Artefakte vollständig erstellt, im Repository abgelegt und der Qualitätskontrolle vorgelegt werden:

- **1. Release Artifact:** Kompiliertes Binary, Container-Image oder Quellcode-Paket.
- **2. Version Tag:** Unveränderlicher Git Tag nach SemVer (z.B. v1.0.0).
- **3. SHA-256 Checksum Manifest:** Signiertes Dokument aller Prüfsummen.
- **4. CHANGELOG.md Update:** Aktualisiertes Verzeichnis aller Änderungen.
- **5. Release Notes:** Human-readable Beschreibung von Features und Fixes.
- **6. Configuration Spec:** Spezifikation aller Umgebungsvariablen und Parameter.
- **7. Deployment Instructions:** Schritt-für-Schritt-Anleitung für die Installation.
- **8. Rollback Plan:** Getestete Prozedur zur Rückabwicklung fehlerhafter Deploys.
- **9. Monitoring & Incident Plan:** Metriken, Alerting-Regeln und Notfallkontakte.
- **Gate-Record V2S-G8:** Formelles Qualitätszertifikat für das Gesamtrelease.

Alle genannten Artefakte MÜSSEN versionsgeführt und gegen unbefugte nachträgliche Modifikationen geschützt werden.

## §4 Quality-Gate-Bezug

Die Phase **Release** gipfelt im bzw. bezieht sich auf das Quality Gate **V2S-G8** (*Ist das Release vollständig und reproduzierbar?*).

1. **Gate-Record Pflicht:** Für jedes Durchlaufen des Quality Gates MUSS ein vollständiger Gate-Record erstellt werden. Dieser MUSS mindestens die folgenden Felder enthalten:
   - **Prüfer:** Name/ID der prüfenden Person oder des autonomen Prüf-Agenten.
   - **Datum:** ISO-8601-Zeitstempel der Prüfung.
   - **Ergebnis:** PASSED, FAILED oder BLOCKED.
   - **Nachweise:** Eindeutige Links/Hashes zu den in §3 geforderten Prüfartefakten.
2. **Blockierungsmodell:** Offene Fehler der Schweregrade P0 oder P1 (nach ATC-STD-BUG-001) blockieren das Erteilen der Gate-Freigabe absolut. Das Passieren des Gates mit offenen P0/P1-Mängeln ist strengstens VERBOTEN (MUST NOT).
3. **Maturity-Level:** Das Erreichen dieses Gates ist Voraussetzung für das Aufrücken auf die nächste Reifegradstufe im Lifecycle-State (ATC-STD-V2S-000 §14).

## §5 Schnittstellen

- **Upstream / Downstream:** Upstream: ATC-STD-V2S-016 (Release Candidate Standard) · Downstream: Station 14 (Deployment), Station 15 (Operations) · Andock-Standards: ATC-STD-203 (Release Management / GATE-001..010), ATC-STD-VERSION-001, ATC-STD-UPDATE-001.
- **Master-Standard:** ATC-STD-V2S-000 dient als oberster normativer Rahmen für alle Phasen-Prozesse.
- **Schnittstellenüberwachung:** Schnittstellenänderungen zu Nachbarphasen MÜSSEN über Änderungsanträge (SCR-0042 / ATC-STD-CHANGE-001) koordiniert werden.

## Metriken & Akzeptanzkriterien

Die Einhaltung der Vorgaben dieses Standards wird anhand der folgenden quantitativen Metriken überprüft:

- **M1:** Vollständigkeitsgrad der 9 Pflichtartefakte = 100%.
- **M2:** Verifizierte Reproduzierbarkeit des Builds (Byte-Identität bei erneutem Build).
- **M3:** Erfolgreicher Testlauf der Rollback-Prozedur in der Staging-Umgebung.

## Compliance & Verifikation

Die Compliance mit diesem Standard wird im Rahmen automatisierter Repository-Audits (atc-repo-audit) sowie durch den Standard-Validator (`tools/atc-std-validator/atc_std_validator.py`) anhand der Prüfkriterien S-01 bis S-25 überprüft:

- **Normverstöße:** Abweichungen von MUSS-Kriterien werden als Findings der Klasse F-NNN erfasst und nach ATC-STD-BUG-001 klassifiziert.
- **Prüfzyklus:** Die Compliance-Prüfung erfolgt kontinuierlich in der CI/CD-Pipeline bei jedem Merge-Request sowie im Rahmen von Perioden-Audits.

## Security Considerations

Releases MÜSSEN kryptografisch signiert werden (GPG / Cosign). Nicht-signierte Artefakte DÜRFEN NICHT in Produktions-Registrys geladen oder auf Produktivknoten ausgeführt werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-203, ATC-STD-VERSION-001, ATC-STD-UPDATE-001  
**INFORMATIVE:** SCR-0042, ATC-STD-V2S-000 (V2S Lifecycle Master)
