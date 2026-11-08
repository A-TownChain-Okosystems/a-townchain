---
standard:
  id: ATC-STD-V2S-012
  title: "Verification Standard Standard"
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

# ATC-STD-V2S-012 — Verification Standard Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45; Range
> ATC-STD-V2S-000..026). Dockt direkt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-012 legt die Regeln für die technische Verifikation im Vision-to-Software Lifecycle fest. Verifikation prüft objektiv, ob das erstellte Software-System die technischen Spezifikationen, Architekturanforderungen und Schnittstellenkontrakte korrekterweise erfüllt («Have we built the system right?»). Der Standard grenzt Verifikation präzise von der fachlichen Validierung (ATC-STD-V2S-013) ab und definiert die Verifikationsnachweiskette.

## Scope

**Gilt für:** Alle ATC-Projekte, Repositories, Protokolle, Smart Contracts, KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten gemäß dem Anwendungsbereich von ATC-STD-V2S-000.
**Nicht-Gilt:** Rein dokumentarische Artefakte ohne Software-Ziel oder eigenständigen Quellcode (z.B. reine Spezifikationsdokumente, Dokumentations-Wikis — diese folgen ATC-STD-000 bzw. ATC-STD-DESC-001).

## §1 Gegenstand & Verortung

Dieser Standard ordnet die Phase **Verification** verbindlich in den Vision-to-Software Lifecycle ein (Verifikationsphase (Prüfung gegen Spezifikation)).
Er stellt das Bindeglied zwischen den vorangehenden Entwicklungs- und Testschritten und den nachgelagerten Qualitäts- bzw. Veröffentlichungsphasen dar.

Die wesentliche Aufgabe besteht darin, das Erreichen des Meilensteins technische Verifikation: erfüllt das System die Spezifikation; Abgrenzung zu 013 Validation systematisch und nachvollziehbar sicherzustellen. Eine saubere Abgrenzung zu den benachbarten Lifecycle-Stationen garantiert, dass keine Prozessschritte übersprungen werden und die festgelegten Qualitätsanforderungen lückenlos nachgewiesen werden.

## §2 Kernregeln

1. **KR-1:** Jedes in der Spezifikation (SPEC/REQ) definierte technische Kriterium MUSS durch mindestens ein objektiv prüfbares Verifikationsverfahren nachgewiesen werden.
2. **KR-2:** Die Verifikation MUSS sowohl die funktionalen Spezifikationen als auch alle nicht-funktionalen Qualitätsmerkmale (Performance, Stabilität, Ressourcenverbrauch) abdecken.
3. **KR-3:** Sämtliche Verifikationsnachweise MÜSSEN in der zentralen Traceability-Matrix (ATC-STD-V2S-000 §11) direkt mit den zugehörigen Spezifikationselementen verknüpft werden.
4. **KR-4:** Abweichungen zwischen der technischen Soll-Spezifikation und dem Ist-Verhalten der Software MÜSSEN als Verifikationsfehler (F-NNN) erfasst werden.
5. **KR-5:** Für jedes Verifikationsverfahren MUSS ein formales Verifikationsprotokoll (Verification Record, VR-Record) von der zuständigen Qualitätsinstanz ausgestellt werden.
6. **KR-6:** Eine Software DARF NICHT zur fachlichen Validierung (ATC-STD-V2S-013) freigegeben werden, solange nicht alle technischen Verifikationskriterien der Stufen P0 und P1 erfüllt sind.

## §3 Artefakte & Nachweispflichten

Für die ordnungsgemäße Durchführung der Phase **Verification** MÜSSEN die folgenden Artefakte vollständig erstellt, im Repository abgelegt und der Qualitätskontrolle vorgelegt werden:

- **Verification Cross-Reference Index (VCRI):** Abgleichmatrix aller Spezifikationspunkte mit den zugehörigen Verifikationsnachweisen.
- **Verification Record (VR-Record):** Signiertes Protokoll der technischen Verifikationsergebnisse.
- **Traceability-Matrix-Aktualisierung:** Nachweis der 100%igen Verknüpfung von SPEC zu Verifikationsergebnis.
- **Non-Conformance Reports (NCR):** Berichte über festgestellte Abweichungen von den technischen Vorgaben.

Alle genannten Artefakte MÜSSEN versionsgeführt und gegen unbefugte nachträgliche Modifikationen geschützt werden.

## §4 Quality-Gate-Bezug

Die Phase **Verification** gipfelt im bzw. bezieht sich auf das Quality Gate **Prüfstufe vor V2S-G6 (Spezifikationstreue)** (*Erfüllt die Implementierung nachweisbar alle technischen Spezifikationen?*).

1. **Gate-Record Pflicht:** Für jedes Durchlaufen des Quality Gates MUSS ein vollständiger Gate-Record erstellt werden. Dieser MUSS mindestens die folgenden Felder enthalten:
   - **Prüfer:** Name/ID der prüfenden Person oder des autonomen Prüf-Agenten.
   - **Datum:** ISO-8601-Zeitstempel der Prüfung.
   - **Ergebnis:** PASSED, FAILED oder BLOCKED.
   - **Nachweise:** Eindeutige Links/Hashes zu den in §3 geforderten Prüfartefakten.
2. **Blockierungsmodell:** Offene Fehler der Schweregrade P0 oder P1 (nach ATC-STD-BUG-001) blockieren das Erteilen der Gate-Freigabe absolut. Das Passieren des Gates mit offenen P0/P1-Mängeln ist strengstens VERBOTEN (MUST NOT).
3. **Maturity-Level:** Das Erreichen dieses Gates ist Voraussetzung für das Aufrücken auf die nächste Reifegradstufe im Lifecycle-State (ATC-STD-V2S-000 §14).

## §5 Schnittstellen

- **Upstream / Downstream:** Upstream: ATC-STD-V2S-011 (Testing Standard), ATC-STD-202 (Anforderungen) · Downstream: ATC-STD-V2S-013 (Validation Standard) · Andock-Standards: ATC-STD-BUG-001, ATC-STD-201 (ADR).
- **Master-Standard:** ATC-STD-V2S-000 dient als oberster normativer Rahmen für alle Phasen-Prozesse.
- **Schnittstellenüberwachung:** Schnittstellenänderungen zu Nachbarphasen MÜSSEN über Änderungsanträge (SCR-0042 / ATC-STD-CHANGE-001) koordiniert werden.

## Metriken & Akzeptanzkriterien

Die Einhaltung der Vorgaben dieses Standards wird anhand der folgenden quantitativen Metriken überprüft:

- **M1:** Verifikationsabdeckung aller Spezifikationselemente = 100%.
- **M2:** 0 offene Verifikationsabweichungen der Prioritäten P0 und P1.
- **M3:** Automatisierungsgrad der technischen Verifikationsprüfungen >= 85%.

## Compliance & Verifikation

Die Compliance mit diesem Standard wird im Rahmen automatisierter Repository-Audits (atc-repo-audit) sowie durch den Standard-Validator (`tools/atc-std-validator/atc_std_validator.py`) anhand der Prüfkriterien S-01 bis S-25 überprüft:

- **Normverstöße:** Abweichungen von MUSS-Kriterien werden als Findings der Klasse F-NNN erfasst und nach ATC-STD-BUG-001 klassifiziert.
- **Prüfzyklus:** Die Compliance-Prüfung erfolgt kontinuierlich in der CI/CD-Pipeline bei jedem Merge-Request sowie im Rahmen von Perioden-Audits.

## Security Considerations

Sicherheitsspezifische Parameter (wie Verschlüsselungsstärken, Schlüssellängen, Timeout-Verhalten) MÜSSEN im Rahmen der Verifikation exakt gegen die Vorgaben aus den Sicherheitsstandards (ATC-STD-314) geprüft werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000, ATC-STD-V2S-011, ATC-STD-202, ATC-STD-BUG-001  
**INFORMATIVE:** SCR-0042, ATC-STD-V2S-000 (V2S Lifecycle Master)
