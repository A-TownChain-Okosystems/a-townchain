---
standard:
  id: ATC-STD-V2S-016
  title: "Release Candidate Standard Standard"
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

# ATC-STD-V2S-016 — Release Candidate Standard Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45; Range
> ATC-STD-V2S-000..026). Dockt direkt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-016 regelt das Erstellen, Verwalten und die Freigabekette von Release Candidates (RC) im Vision-to-Software Lifecycle. Er legt die Anforderungen an die eindeutige Versionierung (`vX.Y.Z-rc.N`), die Build-Nummerierung (`JJJJMMTT.NNN`), die Git-Commit-Verknüpfung und den strikten Durchlauf der RC-Qualitätskette (FULL TEST → SECURITY → AUDIT → APPROVAL) fest.

## Scope

**Gilt für:** Alle ATC-Projekte, Repositories, Protokolle, Smart Contracts, KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten gemäß dem Anwendungsbereich von ATC-STD-V2S-000.
**Nicht-Gilt:** Rein dokumentarische Artefakte ohne Software-Ziel oder eigenständigen Quellcode (z.B. reine Spezifikationsdokumente, Dokumentations-Wikis — diese folgen ATC-STD-000 bzw. ATC-STD-DESC-001).

## §1 Gegenstand & Verortung

Dieser Standard ordnet die Phase **Release Candidate** verbindlich in den Vision-to-Software Lifecycle ein (Station 12 (RELEASE CANDIDATE)).
Er stellt das Bindeglied zwischen den vorangehenden Entwicklungs- und Testschritten und den nachgelagerten Qualitäts- bzw. Veröffentlichungsphasen dar.

Die wesentliche Aufgabe besteht darin, das Erreichen des Meilensteins RC mit eindeutigem Build: vX.Y.Z-rc.N, Build-Nummer JJJJMMTT.NNN, Commit-Hash; RC-Kette FULL TEST→SECURITY→AUDIT→APPROVAL systematisch und nachvollziehbar sicherzustellen. Eine saubere Abgrenzung zu den benachbarten Lifecycle-Stationen garantiert, dass keine Prozessschritte übersprungen werden und die festgelegten Qualitätsanforderungen lückenlos nachgewiesen werden.

## §2 Kernregeln

1. **KR-1:** Jeder Release Candidate MUSS eine eindeutige Version nach SemVer mit RC-Suffix führen: vX.Y.Z-rc.N (gemäß ATC-STD-VERSION-001).
2. **KR-2:** Jedes RC-Build-Artefakt MUSS mit einer kanonischen Build-Nummer im Format JJJJMMTT.NNN und dem exakten Git-Commit-Hash unlösbar verknüpft sein.
3. **KR-3:** Der RC MUSS zwingend die vollständige RC-Kette durchlaufen: FULL TEST → SECURITY CHECK → AUDIT VERIFICATION → APPROVAL.
4. **KR-4:** Mit der Deklaration eines Release Candidate MUSS für das betroffene Repository ein Code Freeze ausgerufen werden; Quellcodeänderungen sind während des Freeze VERBOTEN.
5. **KR-5:** Tritt während der RC-Prüfung ein Fehler auf, MUSS der aktuelle RC verworfen werden; ein Fix erfordert das Erzeugen eines neuen RC (z.B. -rc.N+1).
6. **KR-6:** Die finale Freigabe des RC MUSS durch ein formelles APPROVAL_DECISION-Dokument signiert werden, bevor der Übergang zum Release (ATC-STD-V2S-017) erfolgt.

## §3 Artefakte & Nachweispflichten

Für die ordnungsgemäße Durchführung der Phase **Release Candidate** MÜSSEN die folgenden Artefakte vollständig erstellt, im Repository abgelegt und der Qualitätskontrolle vorgelegt werden:

- **RC Build Manifest:** Maschinenlesbare Deklaration aus Version, Build-Nummer, Commit-Hash und Modulliste.
- **Checksum Manifest:** SHA-256-Kryptografische Prüfsummen aller RC-Binaries und Artefakte.
- **Code Freeze Protocol:** Dokumentierter Zeitraum und Status des Entwicklungs-Stopps.
- **Approval Decision Record:** Formelles Freigabedokument (z.B. APPROVAL-DECISION-2026-09-08-001.md).

Alle genannten Artefakte MÜSSEN versionsgeführt und gegen unbefugte nachträgliche Modifikationen geschützt werden.

## §4 Quality-Gate-Bezug

Die Phase **Release Candidate** gipfelt im bzw. bezieht sich auf das Quality Gate **Prüfkette vor V2S-G8** (*Ist das Candidate-Build vollständig erprobt und freigabefähig?*).

1. **Gate-Record Pflicht:** Für jedes Durchlaufen des Quality Gates MUSS ein vollständiger Gate-Record erstellt werden. Dieser MUSS mindestens die folgenden Felder enthalten:
   - **Prüfer:** Name/ID der prüfenden Person oder des autonomen Prüf-Agenten.
   - **Datum:** ISO-8601-Zeitstempel der Prüfung.
   - **Ergebnis:** PASSED, FAILED oder BLOCKED.
   - **Nachweise:** Eindeutige Links/Hashes zu den in §3 geforderten Prüfartefakten.
2. **Blockierungsmodell:** Offene Fehler der Schweregrade P0 oder P1 (nach ATC-STD-BUG-001) blockieren das Erteilen der Gate-Freigabe absolut. Das Passieren des Gates mit offenen P0/P1-Mängeln ist strengstens VERBOTEN (MUST NOT).
3. **Maturity-Level:** Das Erreichen dieses Gates ist Voraussetzung für das Aufrücken auf die nächste Reifegradstufe im Lifecycle-State (ATC-STD-V2S-000 §14).

## §5 Schnittstellen

- **Upstream / Downstream:** Upstream: ATC-STD-V2S-014 (Security Standard), ATC-STD-V2S-015 (Software Audit Standard) · Downstream: ATC-STD-V2S-017 (Release Standard) · Andock-Standards: ATC-STD-VERSION-001, ATC-STD-203 (Release Management).
- **Master-Standard:** ATC-STD-V2S-000 dient als oberster normativer Rahmen für alle Phasen-Prozesse.
- **Schnittstellenüberwachung:** Schnittstellenänderungen zu Nachbarphasen MÜSSEN über Änderungsanträge (SCR-0042 / ATC-STD-CHANGE-001) koordiniert werden.

## Metriken & Akzeptanzkriterien

Die Einhaltung der Vorgaben dieses Standards wird anhand der folgenden quantitativen Metriken überprüft:

- **M1:** 100% Einhaltung des RC-Identitätsmusters (SemVer-RC + Build-Nummer + Commit-Hash).
- **M2:** 0 unautorisierte Commits während der aktiven Code-Freeze-Phase.
- **M3:** Lückenloser Durchlauf aller 4 Stufen der RC-Kette vor dem Release-Antrag.

## Compliance & Verifikation

Die Compliance mit diesem Standard wird im Rahmen automatisierter Repository-Audits (atc-repo-audit) sowie durch den Standard-Validator (`tools/atc-std-validator/atc_std_validator.py`) anhand der Prüfkriterien S-01 bis S-25 überprüft:

- **Normverstöße:** Abweichungen von MUSS-Kriterien werden als Findings der Klasse F-NNN erfasst und nach ATC-STD-BUG-001 klassifiziert.
- **Prüfzyklus:** Die Compliance-Prüfung erfolgt kontinuierlich in der CI/CD-Pipeline bei jedem Merge-Request sowie im Rahmen von Perioden-Audits.

## Security Considerations

Die Kryptografischen Prüfsummen (SHA-256) aller RC-Artefakte MÜSSEN unmittelbar nach dem Build erzeugt und digital signiert werden, um spätere Verfälschungen der Testobjekte zu verhindern.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-VERSION-001, ATC-STD-203, ATC-STD-BUG-001  
**INFORMATIVE:** SCR-0042, ATC-STD-V2S-000 (V2S Lifecycle Master)
