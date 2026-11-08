---
standard:
  id: ATC-STD-V2S-009
  title: "Development Standard"
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

# ATC-STD-V2S-009 — Development Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-009 regelt die Durchführung der Implementierung und Quellcodierung im
ATC-Ökosystem. Er schreibt die Rückführung jeder Entwicklungsaufgabe auf Spezifikationen
und Anforderungen vor, erzwingt den Status READY vor Arbeitsbeginn und fordert die
strikte Einhaltung der Zustandskette READY → PLANNED → IN_PROGRESS → IMPLEMENTED →
CODE_REVIEW → CI → TESTED → INTEGRATED. Er stellt klar, dass ein bloßer Commit nicht
mit einem fertigen Feature gleichzusetzen ist (CODE ≠ DONE).

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/DARF NICHT — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** die gesamte Quellcode-Implementierung, Task-Verwaltung, Pull-Request-Abwicklung
und Code-Review-Prozesse in allen ATC-Softwarekomponenten.
**Nicht-Gilt:** rein administrative Aufgabenerfassung ohne Quellcode- oder Build-Änderung.

## §1 Gegenstand & Verortung

Dieser Standard definiert Station 8 (IMPLEMENTATION) des Lifecycles gemäß ATC-STD-V2S-000 §1.
Station 8 setzt die Entwürfe aus Station 7 (DESIGN) in ausführbaren Quellcode um. Sie bildet
den Kern der Entwicklungsaktivität und leitet direkt an Station 9 (INTEGRATION) und
Station 10 (TESTING) weiter.

## §2 Kernregeln

1. **KR-1:** Entwicklungsaufgaben MÜSSEN der strikten Zustandskette READY → PLANNED → IN_PROGRESS → IMPLEMENTED → CODE_REVIEW → CI → TESTED → INTEGRATED folgen (MUST — Bezug Master §7).
2. **KR-2:** Nur Work Items, die explizit den Status READY aufweisen, DÜRFEN in den Status IN_PROGRESS überführt werden (MUST — Bezug Master §7).
3. **KR-3:** Jede Implementierungsaufgabe MUSS direkt auf eine gültige Spezifikation (ATC-STD-V2S-006) und Anforderung (ATC-STD-V2S-004) zurückgeführt werden können.
4. **KR-4:** Ein Commit oder Code-Checkin allein bedeutet NICHT, dass ein Feature fertiggestellt ist (MUST — Grundsatz CODE ≠ DONE nach Master §7 & §8).
5. **KR-5:** Das Code Review MUSS durch eine vom Autor unabhängige Instanz (Mensch oder zugelassener KI-Agent) durchgeführt und nachweisbar dokumentiert werden.
6. **KR-6:** Code ohne zugehörige automatisierte Tests oder mit fehlschlagenden CI-Prüfungen DARF NICHT in den Status INTEGRATED oder den Hauptzweig übernommen werden (MUST NOT).

## §3 Artefakte & Nachweispflichten

In Station 8 (IMPLEMENTATION) entstehen folgende obligatorische Artefakte:

- **Implementation Task Record:** Task-Beschreibung (`.atc/tasks/TASK-NNN.yaml` oder Issue-Record) mit Referenz auf SPEC-ID und REQ-ID.
- **Source Code Commits / Pull Requests:** Versionskontrollierte Codeänderungen mit strukturierter Commit-Message.
- **Code Review Protocol:** Nachweis des absolvierten Reviews inklusive Freigabevermerk.
- **CI Build & Test Logs:** Ausführungsberichte der automatisierten Continuous Integration Pipeline.

## §4 Quality-Gate-Bezug

Dieser Standard regelt den Phasenabschluss für das Quality Gate **V2S-G3** ("Ist die Implementierung abgeschlossen?"):

- **Prüffrage:** Ist der Code vollständig implementiert, reviewed, CI-geprüft und rückverfolgbar?
- **Gate-Record-Pflicht:** Nachweis aller Reviews, Tests und Task-Zustände im Gate-Record.
- **Reifegrad:** Der erfolgreiche Abschluss hebt den Reifegrad auf **M4 (IMPLEMENTED)** gemäß ATC-STD-V2S-000 §14.

## §5 Schnittstellen

- **Upstream Phase:** Station 7 (DESIGN, ATC-STD-V2S-008) und Station 6 (ARCHITECTURE, ATC-STD-V2S-007).
- **Downstream Phase:** Station 9 (INTEGRATION) und Station 10 (TESTING, ATC-STD-BUG-001).
- **Andock-Standards:**
  - ATC-STD-V2S-000 (Lifecycle Master)
  - ATC-STD-001 (Coding Standards)
  - ATC-STD-BUG-001 (Bug Management)
  - ATC-STD-VERSION-001 (Versionierungsstandards)

## Metriken & Akzeptanzkriterien

- **M1:** 100% aller gestarteten Aufgaben befanden sich vor Arbeitsbeginn im Status READY.
- **M2:** 100% Code Review Abdeckungsrate für alle Pull Requests im Hauptzweig.
- **M3:** 100% Traceability von Task zu SPEC-ID und REQ-ID.
- **M4:** 0 Integrationen von Code ohne bestandene CI-Tests.

## Compliance & Verifikation

Die Compliance wird durch automatische CI/CD-Branch-Protection-Rules, Peer-Reviews und
den `atc_std_validator.py` durchgesetzt. Umgehung von Reviews oder nicht-autorisierte
Zustandssprünge erzeugen Findings (F-NNN) gemäß ATC-STD-BUG-001.

## Security Considerations

Der Entwicklungsprozess MUSS Secure Coding Guidelines berücksichtigen. Statische
Code-Analysen (SAST) und Secret-Scanner MÜSSEN in die CI-Pipeline eingebunden sein, um
Sicherheitslücken vor der Integration zu verhindern.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000; **INFORMATIVE:** SCR-0042, ATC-STD-V2S-006, ATC-STD-V2S-008, ATC-STD-BUG-001
