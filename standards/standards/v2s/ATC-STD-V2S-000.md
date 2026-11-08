---
standard:
  id: ATC-STD-V2S-000
  title: "Vision-to-Software Lifecycle Master Standard"
  version: "1.0.0"
  status: approved
  category: v2s
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-08"
  updated: "2026-09-08"
  normative: true
  applies_to: "Alle ATC-Projekte, Repositories, Protokolle, Smart Contracts, KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten"
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-08"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC-STD-V2S-000 — Vision-to-Software Lifecycle Master (v1.0.0, CANDIDATE)

> **Status:** APPROVED (v1.0.0) — normativ per Owner-Entwurf-Mandat §33 (08.09.2026,
> SCR-0042). Master-Standard der Familie Vision-to-Software (FAM-45, Range
> ATC-STD-V2S-000..026). Die 26 Phasen-Standards V2S-001..026 docken an diesen
> Master an; dieser Standard definiert das verbindliche Rahmenwerk.
> **Familie:** ATC Vision-to-Software Standards (V2S, FAM-45).

## Abstract

ATC-STD-V2S-000 ist der Master-Standard des ATC Vision-to-Software Lifecycle: der
verbindliche Prozess zur Überführung einer Idee oder Vision in eine fertig
entwickelte, getestete, dokumentierte, veröffentlichte und betriebbare Software.
Er definiert den kompletten Lifecycle (20 Stationen), die Traceability-Kette
Vision→Requirement→Specification→Architecture→Design→Code→Test→Release,
Definition of Ready und Done, zehn Quality Gates (V2S-G0..V2S-G9), das
P0-P3-Blockierungsmodell, das Reifegradmodell M0-M9, die zentrale
Traceability-Matrix als Pflichtartefakt und den maschinenlesbaren
Lifecycle-State. Praktisch jeder andere ATC-Standard dockt an diese Familie an:
Repository, Coding, Smart Contracts, KI-Agenten, Audits, Versionierung,
Testing, Releases, Roadmaps.

Schlüsselwörter: MUSS/MUSS NICHT, SOLLTE, DARF/KANN — RFC 2119 (MUST/SHOULD/MAY)
gemäß ATC-STD-000 §10.

## Scope

**Gilt für:** alle ATC-Projekte, Repositories, Protokolle, Smart Contracts,
KI-Systeme, Apps, Services, Spiele und Betriebssystem-Komponenten.
**Nicht-Gilt:** rein dokumentarische Artefakte ohne Software-Ziel (Wiki-Artikel,
Standards-Dokumente selbst — diese folgen ATC-STD-000/DESC-001).

## §1 Der verbindliche Lifecycle (20 Stationen)

VISION → IDEA → BUSINESS/SYSTEM GOAL → REQUIREMENTS → SPECIFICATION →
ARCHITECTURE → DESIGN → IMPLEMENTATION → INTEGRATION → TESTING →
SECURITY/AUDIT → RELEASE CANDIDATE → RELEASE → DEPLOYMENT → OPERATIONS →
MONITORING → FEEDBACK → IMPROVEMENT → NEXT VERSION.

1. **KR-1:** Kein Schritt DARF implizit übersprungen werden (MUST NOT).
2. **KR-2:** Ist ein Schritt für ein Projekt nicht erforderlich, MUSS das dokumentiert und begründet werden (Skip-Record im Projekt-Lifecycle-State).
3. **KR-3:** Jedes Projekt MUSS einen Lifecycle-State nach §19 führen, der jede Station ausweist.
4. **KR-4:** Der Zustand jeder Station MUSS einer der Werte sein: pending, in_progress, complete, skipped (mit Begründung), blocked.

## §2 Vision → Requirement (Verbot des Direktübersetzens)

Die Vision DARF NICHT direkt in Code übersetzt werden (MUST NOT). Zwischen
Vision und Code steht zwingend die Kette:

VISION → MISSION → OBJECTIVES → CAPABILITIES → REQUIREMENTS

1. **KR-5:** Aus der Vision MUSS mindestens eine messbare Capability abgeleitet werden, bevor Requirements entstehen.
2. **KR-6:** Jedes Requirement MUSS mindestens ein prügbares Acceptance Criterion haben.
3. **KR-7:** Acceptance Criteria MÜSSEN als Tests operationalisierbar sein (Referenz auf Test-IDs).

Beispiel: Vision „Dezentrale KI-Plattform" → Capability „Lokale KI-Ausführung" →
Requirement REQ-V2S-001 „Das System MUSS ein lokales LLM ausführen können" →
AC-001 Modell ladbar, AC-002 Prompt verarbeitet, AC-003 Antwort zurückgegeben,
AC-004 Fehler protokolliert.

## §3 Requirement → Specification

1. **KR-8:** Jede Anforderung MUSS eine eindeutige ID nach ATC-STD-000 §36 (REQ-<DOM>-NNN) haben.
2. **KR-9:** Die Spezifikation MUSS beantworten: Was? Warum? Schnittstellen? Daten? Abhängigkeiten? Grenzen? Fehlerfälle? Sicherheitsanforderungen? Erfolgsmessung?
3. **KR-10:** Requirement-Status Übergänge: IDEA → PROPOSED → SPECIFIED → READY (nur vorwärts, keine Sprünge).

## §4 Specification → Architecture (ADR-Pflicht)

SPECIFICATION → ARCHITECTURE → COMPONENTS → INTERFACES → DATA FLOW →
DEPLOYMENT MODEL.

1. **KR-11:** Vor Implementierung MUSS eine Architekturentscheidung als ADR (Architecture Decision Record) existieren — mit Decision, Reason, Alternatives, Status.
2. **KR-12:** Der ADR-Status MUSS ACCEPTED sein, bevor Implementierungsaufgaben beginnen.
3. **KR-13:** Zentrale ADRs werden im DECISIONS_REGISTER (Hub) geführt; repo-lokale ADRs verweisen dorthin (Zwei-Ebenen-Modell, ATC-STD-201).

## §5 Architecture → Implementation (Traceability-Kette)

REQ → SPEC → ARCH → DESIGN → TASK → CODE → TEST.

1. **KR-14:** Jede Implementierungsaufgabe MUSS auf eine Spezifikation zurückführen können (MUST — Traceability-Pflicht).
2. **KR-15:** Die Traceability-Kette MUSS in der zentralen Traceability-Matrix (§11) dokumentiert sein.
3. **KR-16:** Code ohne Rückverfolgbarkeit auf ein Requirement gilt als nicht konform (Finding F-NNN nach ATC-STD-BUG-001).

## §6 Definition of Ready (DoR)

Ein Work Item DARF NICHT entwickelt werden, wenn es nicht READY ist (MUST NOT).
Ready erfordert mindestens: eindeutige ID, Beschreibung, Ziel, Priorität,
Anforderungen, Acceptance Criteria, technische Abhängigkeiten, bekannte
Risiken, zuständiges Repository, zuständiges Modul, Teststrategie.

## §7 Development Lifecycle

READY → PLANNED → IN_PROGRESS → IMPLEMENTED → CODE_REVIEW → CI → TESTED →
INTEGRATED.

1. **KR-17:** Ein Commit allein bedeutet NICHT, dass ein Feature fertig ist (MUST: DER Zustand INTEGRATED erfordert CI + Tests + Review).
2. **KR-18:** Code Review MUSS durch eine andere Instanz als den Autor erfolgen (Agenten-Review zulässig, dokumentiert).
3. **KR-19:** Nur READY-Items DÜRFEN zu IN_PROGRESS werden.

## §8 Definition of Done (DoD)

Ein Feature ist erst DONE, wenn ALLES erfüllt ist: Code implementiert, Code
Review abgeschlossen, Unit Tests vorhanden, Integration Tests vorhanden
(sofern erforderlich), Acceptance Criteria erfüllt, CI erfolgreich, Security
Checks erfolgreich, Dokumentation aktualisiert, CHANGELOG aktualisiert,
Versionierung geprüft, Abhängigkeiten geprüft, keine offenen P0/P1-Fehler,
Traceability vollständig.

**CODE ≠ DONE.** Es gilt: CODE + TEST + REVIEW + SECURITY + DOCUMENTATION +
TRACEABILITY + ACCEPTANCE = DONE (MUST).

## §9 Quality Gates V2S-G0..G9

| Gate | Prüffrage | Pflicht |
|---|---|---|
| V2S-G0 | Ist die Vision eindeutig? | MUSS bestanden vor IDEA-Verfolgung |
| V2S-G1 | Sind die Anforderungen vollständig? | MUSS vor Spezifikation |
| V2S-G2 | Ist die Architektur freigegeben? | MUSS vor Implementierung (ADR ACCEPTED) |
| V2S-G3 | Ist die Implementierung abgeschlossen? | MUSS vor Integration |
| V2S-G4 | Funktionieren die Komponenten zusammen? | MUSS vor Testing |
| V2S-G5 | Sind die technischen Tests erfolgreich? | MUSS vor Validation |
| V2S-G6 | Erfüllt das System den vorgesehenen Zweck? | MUSS vor Security-Freigabe |
| V2S-G7 | Sind Sicherheitsrisiken bewertet? | MUSS vor Release Candidate |
| V2S-G8 | Ist das Release vollständig und reproduzierbar? | MUSS vor Release |
| V2S-G9 | Kann das System sicher betrieben werden? | MUSS vor Deployment |

1. **KR-20:** Jedes Gate MUSS einen dokumentierten Gate-Record haben (Prüfer, Datum, Ergebnis, Nachweise).
2. **KR-21:** Ein fehlgeschlagenes Gate blockiert den Übergang zur nächsten Station (MUST — keine Gate-Überholung).
3. **KR-22:** Release-Gates (GATE-001..010, ATC-STD-203) und Netzwerk-Promotion (GATE-011..013, ATC-STD-NET-004) bleiben unberührt und DOKUMENTIEREN V2S-Gates in ihren Abläufen.

## §10 P0-P3-Blockierungsmodell

| Stufe | Konsequenz |
|---|---|
| P0 | Release absolut blockiert (Datenverlust, kritische Sicherheitslücke) |
| P1 | Release normalerweise blockiert (Kernfunktion funktioniert nicht) |
| P2 | Release nur mit dokumentierter Ausnahme (fehlende Dokumentation) |
| P3 | Verbesserung / Backlog (UX-Verbesserung) |

1. **KR-23:** Jeder offene Fehler MUSS einer P-Stufe zugeordnet sein; die Zuordnung erfolgt aus der Finding-Schwere (ATC-STD-BUG-001, S0-S4): S0→P0, S1→P0/P1, S2→P1, S3→P2, S4→P3.
2. **KR-24:** P2-Ausnahmen MÜSSEN mit Begründung und Auflage dokumentiert werden; P0-Ausnahmen sind VERBOTEN (MUST NOT).

## §11 Zentrale Traceability Matrix (Pflichtartefakt)

Jedes größere ATC-Projekt MUSS eine Traceability-Matrix führen (Markdown
oder maschinenlesbar) mit den Spalten: Requirement | Specification |
Architecture (ADR) | Code (Modul) | Test | Release.

| REQ | SPEC | ADR | MOD | TEST | Version |
|---|---|---|---|---|---|
| REQ-V2S-002 | SPEC-001 | ADR-001 | MOD-001 | TEST-001 | v1.0 |

Damit MUSS ein Audit jederzeit beantworten können: „Welche Anforderung wird
durch welchen Code implementiert und durch welchen Test nachgewiesen?"

## §12 Release Candidate und Release→Production

1. **KR-25:** Vor jedem Release entsteht ein Release Candidate mit eindeutigem Build: `VERSION X.Y.Z-rc.N`, Build-Nummer (JJJJMMTT.NNN), Commit-Hash (MUST — exakte Nachvollziehbarkeit).
2. **KR-26:** Ein Production-Release MUSS enthalten: Release Artifact, Version, Checksum, Changelog, Release Notes, Configuration, Deployment Instructions, Rollback Plan, Monitoring, Incident Plan.
3. **KR-27:** Jeder Production-Release MUSS einen Rollback-Plan haben (MUST — Deployment: CURRENT → DEPLOY → HEALTH CHECK → SUCCESS: aktiv | FAILURE: ROLLBACK auf vorn-N).
4. **KR-28:** RC-Durchlauf: VERSION → RELEASE CANDIDATE → FULL TEST → SECURITY CHECK → AUDIT → APPROVAL → RELEASE (kein Schritt darf entfallen).

## §13 Betrieb und Feedback (geschlossener Regelkreis)

1. **KR-29:** Production MUSS betrieben werden mit: Monitoring, Logging, Metrics, Alerting, Backup, Recovery, Incident Management, Security Monitoring, Performance Monitoring, Capacity Monitoring, Version Management.
2. **KR-30:** Produktions-Erkenntnisse (User Feedback, System Metrics, Incidents, Security Findings, Performance, neue Requirements, Markt/Ökosystem) MÜSSEN über den Change-Request-Prozess (ATC-STD-UPDATE-001) in die nächste Version fließen: FEEDBACK → CHANGE REQUEST → IMPACT ANALYSIS → NEW REQUIREMENT → NEXT VERSION.
3. **KR-31:** Damit entsteht der geschlossene Regelkreis: VISION → BUILD → RELEASE → OPERATE → LEARN → IMPROVE → (zurück zu VISION/next Version).

## §14 Reifegradmodell M0-M9

| Level | Bezeichnung | Zustand |
|---|---|---|
| M0 | IDEA | Idee |
| M1 | DEFINED | Vision/Ziel definiert |
| M2 | SPECIFIED | Requirements/Spezifikation |
| M3 | ARCHITECTED | Architektur vorhanden |
| M4 | IMPLEMENTED | Software implementiert |
| M5 | VERIFIED | Tests erfolgreich |
| M6 | VALIDATED | Zweck erfüllt |
| M7 | RELEASED | offiziell veröffentlicht |
| M8 | OPERATING | produktiv betrieben |
| M9 | MATURE | kontinuierlich verbessert |

1. **KR-32:** Der Maturity-Level MUSS im Lifecycle-State geführt werden; eine Software mit M4 ist NICHT automatisch produktionsreif (M7/M8 erfordern V2S-G7..G9).
2. **KR-33:** M-Level-Übergänge MÜSSEN den Gates zugeordnet sein: M1←G0, M2←G1, M3←G2, M5←G5, M6←G6, M7←G8, M8←G9.

## §15 Familienstruktur (Andockpunkte)

Die 26 Phasen-Standards V2S-001..026 (FAM-45) implementieren je Station die
Details; dieser Master ist normativ für Übergänge, Gates und Pflichtartefakte.
Andocken: Requirements ↔ ATC-STD-202, Testing ↔ ATC-STD-BUG-001, Security ↔
ATC-STD-314/NET-007, Release ↔ ATC-STD-203, Updates ↔ ATC-STD-UPDATE-001,
Versionierung ↔ ATC-STD-VERSION-001, Audits ↔ ATC-STD-AUDIT-Familie.

## §16 Maschinenlesbarer Lifecycle-State

Jedes Projekt MUSS einen Lifecycle-State führen (z.B. als
`.atc/lifecycle-state.yaml`), validierbar gegen
`schemas/software-lifecycle.schema.json`:

```yaml
lifecycle:
  vision: approved        # pending | in_progress | complete | skipped | blocked
  requirements: approved
  specification: approved
  architecture: approved
  implementation: complete
  integration: complete
  testing: passed
  security: passed
  audit: passed
  release: approved
  deployment: complete
  operations: active
  maturity: M8            # M0..M9
```

1. **KR-34:** Der Lifecycle-State MUSS von CI/CD, Repository-Audits (atc-repo-audit) und KI-Agenten automatisch auswertbar sein (MUST — maschinenlesbares Format, Schema-konform).
2. **KR-35:** Der State MUSS bei jedem Gate-Übergang aktualisiert und committet werden.

## §17 Oberste ATC-Governance-Regel (V2S-Grundsatz)

> Keine ATC-Software gilt als fertig, nur weil der Code funktioniert. Sie gilt
> erst als fertig, wenn Vision, Anforderungen, Spezifikation, Architektur,
> Implementierung, Tests, Sicherheit, Dokumentation, Audit, Release und
> Betrieb nachweisbar abgeschlossen sind.

Diese Regel ist normativ (MUST) und geht bei Konflikten mit Convenience-
Regeln vor; Verstöße werden als Findings (F-NNN) geführt.

## Compliance

Geprüft per Validator-Lauf (S-01..S-25), Lifecycle-State-Schema-Validierung und
Review-Chain nach ATC-STD-000; Verstöße gegen MUST-Kriterien = Finding (F-NNN)
nach ATC-STD-BUG-001..004.

## Security Considerations

V2S-G7 (Security) ist hartes Release-Gate; Sicherheitsprüfungen MÜSSEN vor
jedem Release Candidate abgeschlossen sein. Notfalländerungen folgen
ATC-STD-UPDATE-001 (Emergency) und MÜSSEN nachträglich durch die Chain.

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Changelog

| Version | Datum | Änderung |
|---|---|---|
| 1.0.0 | 2026-09-08 | Initiale Fassung — Owner-Entwurf kanonisiert, Familie FAM-45 errichtet (SCR-0042) |

## References

**NORMATIVE:** ATC-STD-000 (Verfassung, §36 IDs), ATC-STD-201..204, ATC-STD-BUG-001..005,
ATC-STD-NET-001..008, ATC-STD-UPDATE-001, ATC-STD-VERSION-001, ATC-STD-314,
schemas/software-lifecycle.schema.json · **INFORMATIVE:** SCR-0042, Owner-Entwurf
„ATC Vision-to-Software Standards" (08.09.2026), registry/framework.yaml (FAM-45)
