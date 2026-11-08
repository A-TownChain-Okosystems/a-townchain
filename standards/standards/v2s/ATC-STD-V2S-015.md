---
standard:
  id: ATC-STD-V2S-015
  title: "Software Audit Standard Standard"
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

# ATC-STD-V2S-015 — Software Audit Standard Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45; Range
> ATC-STD-V2S-000..026). Dockt direkt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-015 etabliert die unabhängige Vollständigkeits- und Compliance-Prüfung im Vision-to-Software Lifecycle. Er regelt die neutralen Audit-Verfahren durch menschliche Auditoren oder autonome Audit-Agenten, die systematische Überprüfung aller V2S-Phasenartefakte, die Erstellung formeller AUD-Records in Kopplung mit der ATC-STD-AUDIT-Familie (ATC-STD-AUDIT-001) und stellt das Veto-Recht bei Compliance-Verstößen sicher.

## Scope

**Gilt für:** Alle ATC-Projekte, Repositories, Protokolle, Smart Contracts, KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten gemäß dem Anwendungsbereich von ATC-STD-V2S-000.
**Nicht-Gilt:** Rein dokumentarische Artefakte ohne Software-Ziel oder eigenständigen Quellcode (z.B. reine Spezifikationsdokumente, Dokumentations-Wikis — diese folgen ATC-STD-000 bzw. ATC-STD-DESC-001).

## §1 Gegenstand & Verortung

Dieser Standard ordnet die Phase **Software Audit** verbindlich in den Vision-to-Software Lifecycle ein (Software Audit Phase / Unabhängige Prüfung).
Er stellt das Bindeglied zwischen den vorangehenden Entwicklungs- und Testschritten und den nachgelagerten Qualitäts- bzw. Veröffentlichungsphasen dar.

Die wesentliche Aufgabe besteht darin, das Erreichen des Meilensteins unabhängige Vollständigkeitsprüfung; AUD-Records; Kopplung ATC-STD-AUDIT-Familie systematisch und nachvollziehbar sicherzustellen. Eine saubere Abgrenzung zu den benachbarten Lifecycle-Stationen garantiert, dass keine Prozessschritte übersprungen werden und die festgelegten Qualitätsanforderungen lückenlos nachgewiesen werden.

## §2 Kernregeln

1. **KR-1:** Das Software Audit MUSS von einer vom Entwicklungsteam unabhängigen Instanz (Human Auditor oder zertifizierter Audit-Agent nach ATC-STD-AUDIT-001) durchgeführt werden.
2. **KR-2:** Der Auditor MUSS die Lückenlosigkeit der Traceability-Kette (ATC-STD-V2S-000 §11) sowie das Vorhandensein aller Gate-Records V2S-G0 bis V2S-G7 überprüfen.
3. **KR-3:** Sämtliche Feststellungen und Normabweichungen MÜSSEN als Findings (F-NNN) klassifiziert und im Audit Finding Register erfasst werden.
4. **KR-4:** Das Auditergebnis MUSS in einem standardisierten, fälschungssicheren AUD-Record (z.B. AUD-YYYY-NNNN.md) fassbar dokumentiert und im Repository hinterlegt werden.
5. **KR-5:** Bei festgestellten Lizenz-, Governance- oder schwerwiegenden Qualitätsverstößen MUSS der Auditor ein unanfechtbares Audit-Veto einlegen.
6. **KR-6:** Kein Release Candidate DARF zur finalen Release-Freigabe (ATC-STD-V2S-017) vorrücken, ohne dass ein gültiger AUD-Record mit dem Status APPROVED vorliegt.

## §3 Artefakte & Nachweispflichten

Für die ordnungsgemäße Durchführung der Phase **Software Audit** MÜSSEN die folgenden Artefakte vollständig erstellt, im Repository abgelegt und der Qualitätskontrolle vorgelegt werden:

- **Audit-Plan & Audit-Checkliste:** Definierter Prüfkatalog basierend auf den Standards S-01 bis S-25.
- **Audit Finding Register:** Verzeichnis aller identifizierten Findings (F-NNN) mit Zuordnung zu S0..S4.
- **AUD-Record (z.B. AUD-2026-0001.md):** Offizielles Audit-Zertifikat mit Prüfurteil, Prüfer-Signatur und Zeitstempel.

Alle genannten Artefakte MÜSSEN versionsgeführt und gegen unbefugte nachträgliche Modifikationen geschützt werden.

## §4 Quality-Gate-Bezug

Die Phase **Software Audit** gipfelt im bzw. bezieht sich auf das Quality Gate **Prüfstufe vor V2S-G8 (Audit-Approval)** (*Ist das Gesamtsystem unabhängig auf Vollständigkeit und Regelkonformität geprüft?*).

1. **Gate-Record Pflicht:** Für jedes Durchlaufen des Quality Gates MUSS ein vollständiger Gate-Record erstellt werden. Dieser MUSS mindestens die folgenden Felder enthalten:
   - **Prüfer:** Name/ID der prüfenden Person oder des autonomen Prüf-Agenten.
   - **Datum:** ISO-8601-Zeitstempel der Prüfung.
   - **Ergebnis:** PASSED, FAILED oder BLOCKED.
   - **Nachweise:** Eindeutige Links/Hashes zu den in §3 geforderten Prüfartefakten.
2. **Blockierungsmodell:** Offene Fehler der Schweregrade P0 oder P1 (nach ATC-STD-BUG-001) blockieren das Erteilen der Gate-Freigabe absolut. Das Passieren des Gates mit offenen P0/P1-Mängeln ist strengstens VERBOTEN (MUST NOT).
3. **Maturity-Level:** Das Erreichen dieses Gates ist Voraussetzung für das Aufrücken auf die nächste Reifegradstufe im Lifecycle-State (ATC-STD-V2S-000 §14).

## §5 Schnittstellen

- **Upstream / Downstream:** Upstream: ATC-STD-V2S-014 (Security Standard) · Downstream: ATC-STD-V2S-016 (Release Candidate Standard) · Andock-Standards: ATC-STD-AUDIT-001, ATC-STD-000, ATC-STD-BUG-001.
- **Master-Standard:** ATC-STD-V2S-000 dient als oberster normativer Rahmen für alle Phasen-Prozesse.
- **Schnittstellenüberwachung:** Schnittstellenänderungen zu Nachbarphasen MÜSSEN über Änderungsanträge (SCR-0042 / ATC-STD-CHANGE-001) koordiniert werden.

## Metriken & Akzeptanzkriterien

Die Einhaltung der Vorgaben dieses Standards wird anhand der folgenden quantitativen Metriken überprüft:

- **M1:** 100% Prüfabdeckung der anwendbaren ATC-Compliance-Kriterien.
- **M2:** 0 offene Audit-Findings der Schweregrade S0 und S1.
- **M3:** Vorhandensein eines vollständig signierten AUD-Records im Repository.

## Compliance & Verifikation

Die Compliance mit diesem Standard wird im Rahmen automatisierter Repository-Audits (atc-repo-audit) sowie durch den Standard-Validator (`tools/atc-std-validator/atc_std_validator.py`) anhand der Prüfkriterien S-01 bis S-25 überprüft:

- **Normverstöße:** Abweichungen von MUSS-Kriterien werden als Findings der Klasse F-NNN erfasst und nach ATC-STD-BUG-001 klassifiziert.
- **Prüfzyklus:** Die Compliance-Prüfung erfolgt kontinuierlich in der CI/CD-Pipeline bei jedem Merge-Request sowie im Rahmen von Perioden-Audits.

## Security Considerations

Der Software-Audit umfasst die Integritätsprüfung aller Build-Artefakte und Quellcode-Tags, um Manipulationen oder unautorisierte Quellcode-Injektionen vor der Freigabe auszuschließen.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-AUDIT-001, ATC-STD-BUG-001  
**INFORMATIVE:** SCR-0042, ATC-STD-V2S-000 (V2S Lifecycle Master)
