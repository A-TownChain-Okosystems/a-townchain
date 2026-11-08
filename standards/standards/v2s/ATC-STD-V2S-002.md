---
standard:
  id: ATC-STD-V2S-002
  title: "Idea Intake Standard"
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

# ATC-STD-V2S-002 — Idea Intake Standard (v1.0.0, CANDIDATE)

> **Status:** CANDIDATE (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Phasen-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Dockt an Master-Standard ATC-STD-V2S-000 an.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-002 regelt den Prozess der Erfassung, Bewertung und Strukturierung von
neuen Ideen, Lösungsansätzen und Feature-Vorschlägen (Idea Intake) im ATC-Ökosystem.
Er stellt sicher, dass eingehende Impulse aus der Vision oder dem laufenden Betrieb
systematisch erfasst, kategorisiert und auf ihre Passfähigkeit zur Gesamtstrategie
geprüft werden, bevor Ressourcen für die Detaillierung gebunden werden.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/DARF NICHT — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** alle eingehenden Produktideen, Feature-Vorschläge, Innovationsansätze
und Change-Impulse für ATC-Projekte.
**Nicht-Gilt:** Fehlermeldungen und konkrete Software-Bugs (diese folgen direkt dem
Bug-Prozess nach ATC-STD-BUG-001) sowie reine Wartungsaufgaben ohne Feature-Gehalt.

## §1 Gegenstand & Verortung

Dieser Standard definiert Station 2 (IDEA) des Lifecycles nach ATC-STD-V2S-000 §1.
Station 2 nimmt Impulse auf, die entweder direkt aus der Vision (Station 1) hervorgehen
oder aus dem Betrieb und Feedback-Loop (Station 17/18) stammen. Sie grenzt sich von
Station 1 (VISION) dadurch ab, dass sie konkrete Funktionalitäten oder Lösungsansätze
adressiert. Von Station 3 (BUSINESS/SYSTEM GOAL) und Station 4 (REQUIREMENTS) grenzt
sie sich dadurch ab, dass eine Idee noch nicht detailliert spezifiziert oder messbar
quantifiziert sein muss, sondern im Erst-Assessment kategorisiert wird.

## §2 Kernregeln

1. **KR-1:** Jede neue Idee MUSS in einem strukturierten Idea-Record erfasst werden, bevor eine fachliche Bearbeitung beginnt (MUST).
2. **KR-2:** Ein Idea-Record MUSS mindestens Problemstellung, vorgeschlagenen Lösungsansatz, Zielgruppe und erwarteten Nutzen beschreiben (MUST).
3. **KR-3:** Jede Idee MUSS bezüglich ihrer Auswirkung und Komplexität in das Fast-Track- oder Full-Track-Verfahren klassifiziert werden.
4. **KR-4:** Eine Idee DARF NICHT ohne dokumentiertes Intake-Assessment direkt in den Status eines verbindlichen Requirements erhoben werden (MUST NOT).
5. **KR-5:** Verworfene oder zurückgestellte Ideen MÜSSEN mit einer nachvollziehbaren Begründung archiviert werden, um Doppelarbeiten zu vermeiden.
6. **KR-6:** Die Konformität der Idee mit den Leitlinien der Vision (ATC-STD-V2S-001) MUSS vor der Freigabe zur Zieldefinition geprüft werden.

## §3 Artefakte & Nachweispflichten

In Station 2 (IDEA) entstehen folgende obligatorische Artefakte:

- **Idea-Record:** Maschinell oder textuell lesbarer Record (`.atc/ideas/IDEA-NNN.yaml` oder `docs/ideas/IDEA-NNN.md`).
- **Intake-Assessment-Report:** Bewertungsergebnis bezüglich Machbarkeit, Visions-Konformität und strategischer Priorität.
- **Classification Tag:** Zuordnung zu Fast-Track (kleine Erweiterungen) oder Full-Track (komplexe Neuentwicklungen).
- **Decision Log:** Protokoll über die Annahme, Ablehnung oder Zurückstellung der Idee.

## §4 Quality-Gate-Bezug

Der Standard unterstützt den Übergang von der Vision zur Zieldefinition und bereitet
das Quality Gate **V2S-G0** sowie die Vorprüfungen zu **V2S-G1** vor:

- **Prüfkriterium:** Ist die Idee valide, mit der Vision vereinbar und ordnungsgemäß klassifiziert?
- **Gate-Record-Pflicht:** Dokumentation des Intake-Ergebnisses im Prüfprotokoll.
- **Reifegrad:** Ideen im aktiven Intake verbleiben auf Reifegrad **M0 (IDEA)** gemäß ATC-STD-V2S-000 §14, bis die Zieldefinition (M1) erreicht wird.

## §5 Schnittstellen

- **Upstream Phase:** Station 1 (VISION, ATC-STD-V2S-001) sowie Station 17/18 (FEEDBACK / IMPROVEMENT, ATC-STD-UPDATE-001).
- **Downstream Phase:** Station 3 (BUSINESS/SYSTEM GOAL, ATC-STD-V2S-003) und Station 4 (REQUIREMENTS, ATC-STD-V2S-004).
- **Andock-Standards:**
  - ATC-STD-V2S-000 (Lifecycle Master)
  - ATC-STD-V2S-001 (Vision Standard)
  - ATC-STD-BUG-001 (Abgrenzung zu Bug-Reporting)
  - ATC-STD-UPDATE-001 (Einspeisung von Change Requests)

## Metriken & Akzeptanzkriterien

- **M1:** 100% aller bearbeiteten Ideen verfügen über einen vollständigen Idea-Record.
- **M2:** Durchlaufzeit des Intake-Assessments liegt im Zielkorridor (z.B. < 5 Werktage).
- **M3:** 0 nicht-klassifizierte Ideen im aktiven Entwicklungsportfolio.
- **M4:** Transparente Archivierungsquote verworfener Ideen mit 100% Begründungsdichte.

## Compliance & Verifikation

Die Einhaltung wird durch die automatische Verifikation des `atc_std_validator.py`
sowie durch Stichprobenkontrollen im Rahmen der Governance-Audits sichergestellt.
Verstöße gegen die Intake-Pflicht (z.B. ungeprüfte Injektion von Features) führen
zu Findings (F-NNN) gemäß ATC-STD-BUG-001.

## Security Considerations

Bei der Erfassung von Ideen MUSS darauf geachtet werden, dass keine vertraulichen
Sicherheitsarchitekturen oder ungeprüften Drittanbieter-Komponenten ohne Sicherheitsbewertung
in den Ideenspeicher gelangen. Ideen mit Security-Auswirkungen MÜSSEN frühzeitig
gekennzeichnet werden.

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — SCR-0042 |

## References

**NORMATIVE:** ATC-STD-V2S-000, ATC-STD-000; **INFORMATIVE:** SCR-0042, ATC-STD-V2S-001, ATC-STD-BUG-001, ATC-STD-UPDATE-001
