---
standard:
  id: ATC-STD-300
  title: "ATC-STD-300 — Development & Project Management Standard"
  version: "1.0.0"
  status: approved
  category: development
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  mandate: "Owner-Entwurf 07.09.2026 (Michael Wroblewski) — DRAFT->Review per Owner-Vorgabe; ID korrigiert 204->300 (300er-Block, Owner-Rueckfrage)"
  supersedes: null
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-300 — Development & Project Management Standard (v1.0.0, APPROVED)
> **Status:** APPROVED (Owner-Sammelfreigabe 07.09.2026, ATC-STD-000 §9) — normativ in Kraft| **Datum:** 07.09.2026 | **Autor:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Standard-ID:** ATC-STD-300 | **Scope:** Alle A-TownChain-Repositories und Entwicklungsprojekte
> **Referenzen:** ATC-STD-000 (Governance), ATC-STD-201/-202/-203 (Repository-Serie), AD-023 (Quality-Driven), AD-026 (Bauhierarchie)
> **Anwendungsregel:** NORMATIV ab APPROVED — definiert die verbindliche Entwicklungs-Traceability-Schicht (DTC) des gesamten Oekosystems.

---

## Abstract (Purpose)

ATC-STD-300 definiert das verbindliche Managementmodell fuer Wiki, Roadmap,
Requirements, TODOs, Sprints, Issues, Findings, Change Requests, Decisions,
Code/PRs, Tests, Gates und Releases. Ziel ist eine vollstaendige und pruefbare
Traceability vom strategischen Ziel bis zur ausgelieferten Software.

## 1. Zweck

Dieser Standard definiert das verbindliche Managementmodell fuer alle
Entwicklungs- und Governanceartefakte des A-TownChain-Oekosystems und die
verbindliche Verkettung dieser Artefakte (Development Traceability Chain, DTC).

## 2. Scope — Gilt / Gilt nicht

**Gilt:** Alle A-TownChain-Repositories (23 aktive Repos, AD-016/AD-024),
alle Entwicklungsprojekte, alle Agenten (AGENT_MANIFEST.md) und alle
Artefakte der in §5 definierten Klassen.

**Gilt nicht:** Externe Abhaengigkeiten Dritter (ausser deren Einbindung
wird projektitern referenziert); historische Archivbestaende
(docs/archive/, Vault) — diese werden nur per Migration (§22) erfasst.

## 3. REQ-Matrix (normative Anforderungen)

| ID | Anforderung | Verweis |
|----|-------------|---------|
| id: REQ-STD-301 | Jedes Artefakt MUSS in der DTC-Kette (§4) rueckfuehrbar sein. | §4 |
| id: REQ-STD-302 | Das Wiki MUSS den verifizierten Ist-Zustand darstellen; ungepruefte Zukunftsbehauptungen als Ist-Zustand sind unzulaessig. | §6 |
| id: REQ-STD-303 | Die Wiki-Struktur (§6) ist verbindlich; Abweichungen erfordern SCR. | §6 |
| id: REQ-STD-304 | Roadmaps MUESSEN objective, targets und exit_criteria definieren; Status gemaess §7. | §7 |
| id: REQ-STD-305 | Jedes Requirement MUSS testbar sein und auf Standard + Roadmap referenzieren. | §8 |
| id: REQ-STD-306 | Ein TODO beschreibt genau eine Arbeitseinheit mit Lifecycle gemaess §9. | §9 |
| id: REQ-STD-307 | Jeder Sprint MUSS ein Sprint Goal haben; jedes Sprint-TODO MUSS dazu beitragen. | §10 |
| id: REQ-STD-308 | Issue und Finding MUESSEN getrennt gefuehrt werden (ISSUE vs. F). | §11 |
| id: REQ-STD-309 | Wesentliche Architektur-/Anforderungsaenderungen MUESSEN ueber einen Change Request laufen. | §12 |
| id: REQ-STD-310 | Architekturentscheidungen MUESSEN dauerhaft als Decision Record dokumentiert sein. | §13 |
| id: REQ-STD-311 | Jeder relevante PR MUSS mindestens TODO, Requirement und Issue/Finding/CR referenzieren. | §14 |
| id: REQ-STD-312 | Tests MUESSEN Requirement, Implementation und Evidence verknuepfen. | §15 |
| id: REQ-STD-313 | DONE ist kein Gate; Gates entscheiden die Weitergabe des Standes. | §16 |
| id: REQ-STD-314 | Releases MUESSEN vorwaerts und rueckwaerts vollstaendig nachvollziehbar sein. | §17 |
| id: REQ-STD-315 | Die Registry (§18) ist die maschinenlesbare Verknuepfungsebene; kein Eintrag = kein Artefakt. | §18 |
| id: REQ-STD-316 | Die Ebenen CODE/WIKI/ROADMAP/TODO/SPRINT/STANDARD/REGISTRY DUERFEN ihre Funktion nicht gegenseitig uebernehmen. | §19 |
| id: REQ-STD-317 | Die Konsistenzpruefung (§20) MUSS automatisiert in CI laufen. | §20 |
| id: REQ-STD-318 | Jede Implementierung MUSS auf ein freigegebenes Requirement oder eine ausdruecklich freigegebene Wartungsaufgabe zurueckfuehrbar sein. Jeder Release MUSS auf Implementierung, Verifikationsnachweise und Release-Gates zurueckfuehrbar sein. | §21 |
| id: REQ-STD-319 | Das Wiki MUSS den verifizierten aktuellen Zustand darstellen und DARF NICHT als Ersatz fuer Roadmap-, Aufgaben-, Requirement- oder Aenderungsmanagement verwendet werden. | §21 |
| id: REQ-STD-320 | Die Checker-Regeln (§20) sind normativ: FAIL/WARNING-Bewertungen sind verbindlich. | §20 |

## 4. Normative Entwicklungsstruktur (DTC)

Die primäre Kette lautet:

```
STANDARD → REQUIREMENT → ROADMAP → SPRINT → TODO → IMPLEMENTATION
        → TEST → REVIEW → GATE → RELEASE → WIKI
```

Diese Kette bildet die **ATC Development Traceability Chain (DTC)**.
Jedes Artefakt MUSS entlang dieser Kette rueckfuehrbar sein (REQ-STD-301).

## 5. Artefaktdefinitionen

| Artefakt | ID | Zweck |
|----------|----|----|
| Standard | ATC-STD-NNN | Verbindliche Regeln |
| Requirement | REQ-STD-NNN | Nachweisbare Anforderung |
| Roadmap | RM-NNNN | Strategisches Ziel |
| Sprint | SPR-YYYY-NNN | Zeitlich begrenztes Arbeitsziel |
| TODO | TODO-NNNNNN | Konkrete Aufgabe |
| Issue | ISSUE-NNNNNN | Problem / Arbeitspunkt |
| Finding | F-NNN | Audit-/Pruefergebnis |
| Change Request | CR-NNNNNN | Beantragte Aenderung |
| Decision | DEC-NNNNNN | Architektur-/Governanceentscheidung |
| Pull Request | PR-NNNNNN | Codeaenderung |
| Test | TEST-NNNNNN | Verifikation |
| Gate | GATE-NNNNNN | Freigabepruefung |
| Release | REL-X.Y.Z | Ausgelieferter Stand |

## 6. Wiki-Standard

Das Wiki stellt den kanonisch dokumentierten **Ist-Zustand** dar.

```
wiki/
├── architecture/  ├── standards/      ├── requirements/
├── protocols/     ├── specifications/ ├── apis/
├── modules/       ├── security/       ├── testing/
├── deployment/    ├── operations/     ├── development/
├── governance/    ├── decisions/      └── changelog/
```

**Regel (REQ-STD-302):** Das Wiki DARF keine ungeprueften
Zukunftsbehauptungen als Ist-Zustand darstellen.

```text
❌ "ATC besitzt bereits X."   (wenn X nicht implementiert ist)

Stattdessen:
Status: planned
Roadmap: RM-XXXX
Requirement: REQ-STD-XXXX
```

## 7. Roadmap-Standard

Roadmaps definieren strategische Ziele:

```yaml
roadmap_id: RM-0003
name: ATC Testnet
status: planned
objective: Public ATC Testnet
targets: [consensus, validator, wallet, explorer, monitoring]
exit_criteria: [consensus_stable, security_gate_pass,
                reproducible_build, documentation_complete]
```

**Roadmap-Status:** DRAFT · PLANNED · ACTIVE · PAUSED · COMPLETED · CANCELLED

## 8. Requirement-Standard

Requirements sind die pruefbare Bruecke zwischen Governance und Umsetzung:

```yaml
requirement_id: REQ-STD-342
title: Validator Health Monitoring
type: functional
priority: P1
source:
  standard: ATC-STD-204
roadmap: [RM-0003]
acceptance_criteria: [validator_heartbeat, missed_block_detection,
                      alert_generation, automated_tests, documentation]
```

Ein Requirement MUSS grundsätzlich testbar sein (REQ-STD-305).
(Beispiel-IDs: REQ-STD-342 ist ein exemplarischer Requirement-Eintrag im
300er-Nummernkreis dieses Standards.)

## 9. TODO-Standard

Ein TODO beschreibt genau eine klar abgegrenzte Arbeitseinheit:

```yaml
todo_id: TODO-000421
title: Implement validator health monitoring
status: READY
priority: P1
requirement: [REQ-STD-342]
roadmap: [RM-0003]
sprint: [SPR-2026-037]
repository: [validator]
component: [monitoring]
acceptance_criteria: [heartbeat, missed_block_detection,
                      alerting, tests, documentation]
```

**TODO-Lifecycle:** BACKLOG → READY → IN_PROGRESS → IN_REVIEW → DONE
Alternativ: IN_PROGRESS → BLOCKED / CANCELLED.

## 10. Sprint-Standard

Ein Sprint besitzt immer ein **Sprint Goal**:

```yaml
sprint_id: SPR-2026-037
goal: Validator monitoring baseline
start: 2026-09-07
end: 2026-09-20
roadmap: [RM-0003]
todos: [TODO-000421, TODO-000422, TODO-000423]
deliverables: [monitoring_service, dashboard, alerting]
exit_criteria: [tests_pass, review_pass, security_pass,
                documentation_updated]
```

**Sprint-Regel (REQ-STD-307):** Ein Sprint ist kein Sammelbecken fuer
beliebige TODOs. Jedes Sprint-TODO MUSS zum Sprint Goal beitragen.

## 11. Issue & Finding

Diese beiden Begriffe MUESSEN getrennt bleiben (REQ-STD-308):

- **Issue** — Entwicklungs-/Produktproblem: `ISSUE-000421`
- **Finding** — Festgestelltes Ergebnis eines Audits, Reviews oder einer Pruefung: `F-017`

Beispielkette:

```
F-017 → CR-000031 → TODO-000512 → PR-000872 → TEST-000921 → GATE-000314
```

## 12. Change-Request-Standard

Aenderungen an bestehenden Anforderungen, Standards oder
Architekturentscheidungen werden ueber einen Change Request gesteuert:

```yaml
change_request_id: CR-000031
title: Change validator monitoring architecture
status: REVIEW
reason: scalability
affected: [ATC-STD-300, REQ-STD-342]
impact: [validator, monitoring, infrastructure]
decision: pending
```

Keine wesentliche Architekturanderung DARF als normales TODO
„hineingeschmuggelt" werden (REQ-STD-309).

## 13. Decision-Record-Standard

Architekturentscheidungen werden dauerhaft dokumentiert:

```yaml
decision_id: DEC-000017
title: Canonical validator monitoring backend
status: ACCEPTED
context: "Multiple monitoring implementations exist."
decision: "Use ATC Monitoring Core as canonical implementation."
consequences:
  positive: [single_source, easier_audit, lower_duplication]
  negative: [migration_required]
```

Damit wird verhindert, dass dieselbe Architekturfrage Monate spaeter
erneut entschieden wird (REQ-STD-310).

## 14. Pull-Request-Traceability

Jeder relevante PR MUSS mindestens referenzieren: TODO, Requirement,
Issue/Finding/Change Request (REQ-STD-311):

```yaml
PR-00872:
  implements: [TODO-000421]
  satisfies: [REQ-STD-342]
  resolves: [ISSUE-000311]
```

## 15. Test-Traceability

Tests MUESSEN zeigen, was sie verifizieren (REQ-STD-312):

```yaml
test_id: TEST-00421
requirement: [REQ-STD-342]
implementation: [PR-00872]
result: PASS
evidence: [CI, integration_test, monitoring_test]
```

Dadurch entsteht: Requirement → Implementation → Test → Evidence.

## 16. Gate-System

**DONE bedeutet lediglich:** „Die definierte Arbeit wurde abgeschlossen."

Ein Gate beantwortet dagegen: **„Darf dieser Stand weiter?"**

Gates: GATE-DEV · GATE-TEST · GATE-SECURITY · GATE-REVIEW ·
GATE-RELEASE · GATE-MAINNET

```yaml
gate_id: GATE-000314
target:
  release: REL-1.2.0
checks:
  tests: PASS
  security: PASS
  review: PASS
  documentation: PASS
  reproducibility: PASS
decision: APPROVED
```

## 17. Release-Standard

Ein Release MUSS rueckwaerts vollstaendig nachvollziehbar sein:

```
REL-1.2.0
  ├── GATE-000314
  ├── PR-00872, PR-00873
  ├── TODO-000421, TODO-000422
  ├── REQ-STD-342
  └── RM-0003
```

Umgekehrt MUSS auch gelten: TODO → PR → Release. Damit kann
festgestellt werden, in welchem Release eine Aufgabe tatsaechlich
ausgeliefert wurde (REQ-STD-314).

## 18. Registry

Die Registry ist die zentrale maschinenlesbare Verknuepfung
(REQ-STD-315 — kein Eintrag = kein Artefakt):

```
registry/
├── standards.yaml        ├── requirements.yaml   ├── roadmap.yaml
├── sprints.yaml         ├── todos.yaml          ├── issues.yaml
├── findings.yaml        ├── change-requests.yaml├── decisions.yaml
├── repositories.yaml     ├── dependencies.yaml   ├── tests.yaml
├── gates.yaml            └── releases.yaml
```

## 19. Single Source of Truth

Besonders wichtig ist die Trennung (REQ-STD-316):

| Ebene | Funktion |
|-------|----------|
| CODE | technische Realitaet |
| WIKI | dokumentierte Realitaet |
| ROADMAP | strategische Zukunft |
| TODO | offene Arbeit |
| SPRINT | aktuelle Umsetzung |
| STANDARD | verbindliche Regeln |
| REGISTRY | maschinenlesbare Verknuepfung |

Keine dieser Ebenen DARF die Funktion einer anderen uebernehmen.

## 20. Automatisierbare Konsistenzpruefung

Der **ATC Development Consistency Checker** MUSS automatisiert in CI
laufen (REQ-STD-317). Er prueft normativ (REQ-STD-320):

| Pruefung | Bewertung |
|----------|-----------|
| TODO ohne Requirement | FAIL |
| TODO ohne Roadmap-Ziel | FAIL |
| TODO ohne Sprint | WARNING |
| Sprint ohne Goal | FAIL |
| PR ohne TODO | WARNING/FAIL |
| Release ohne Gate | FAIL |
| Requirement ohne Test | FAIL |
| Wiki behauptet nicht existierenden Code | FAIL |
| Standard ohne Registry-Eintrag | FAIL |
| Release ohne Changelog | FAIL |

Ergebnisformat:

```
┌───────────────────────────────┐
│ ATC DEVELOPMENT CONSISTENCY   │
├───────────────────────────────┤
│ Standards/Requirements/Roadmap│
│ TODO-/Sprint-/Code-Trace      │
│ Tests/Gates/Wiki              │
├───────────────────────────────┤
│ RESULT          APPROVED      │
└───────────────────────────────┘
```

## 21. Governance-Kernregel (normativ, bilingual)

> **EN:** Every implementation MUST be traceable to an approved requirement
> or explicitly approved maintenance task. Every release MUST be traceable
> to its implementation, verification evidence, and applicable release
> gates. The Wiki MUST represent the verified current state and MUST NOT
> be used as a substitute for roadmap, task, requirement, or change
> management.

> **DE:** Jede Implementierung MUSS auf ein freigegebenes Requirement oder
> eine ausdruecklich freigegebene Wartungsaufgabe zurueckfuehrbar sein.
> Jeder Release MUSS auf Implementierung, Verifikationsnachweise und die
> zutreffenden Release-Gates zurueckfuehrbar sein. Das Wiki MUSS den
> verifizierten aktuellen Zustand darstellen und DARF NICHT als Ersatz fuer
> Roadmap-, Aufgaben-, Requirement- oder Aenderungsmanagement verwendet
> werden.

## 22. Migration & Kompatibilitaet

Bestehende Strukturen werden wie folgt abgebildet (uebergangsweise
parallel bis zur vollstaendigen Registry-Migration):

| Bestand | Ziel-Form | Bemerkung |
|---------|-----------|-----------|
| AD-001…AD-041 (DECISIONS_REGISTER) | DEC-000001…DEC-000041 | 1:1-Index; kanonisch bleibt DECISIONS_REGISTER bis registry/decisions.yaml existiert |
| findings.yaml (F-001…F-010) | F-NNN | bereits konform |
| GitHub Issues | ISSUE-NNNNNN | Nummernkreis je Repo; Mapping via registry/issues.yaml |
| ATCLang Gates G0-G19 (AD-022) | GATE-NNNNNN | Gate-Metadaten wandern nach registry/gates.yaml |
| K-/M-Sprints | SPR-YYYY-NNN | historische Sprints bleiben referenzierbar |
| MK1-MK12 / Roadmap v2.0 | RM-NNNN | Mapping bei Anlage von registry/roadmap.yaml |

**Uebergangsregel:** Solange eine Registry-Datei (§18) noch nicht existiert,
ist die jeweils fuehrende Dokumentstruktur (DECISIONS_REGISTER, GitHub
Issues, SPRINT_ROADMAP) die provisorische Quelle; doppelte Pflege ist
zu vermeiden, Migration hat Vorrang.

## 23. Security Considerations

- Der Consistency-Checker verarbeitet Metadaten aus allen Repos; er DARF
  keine Secrets lesen oder ausgeben (Sicherheitsregel: Token maskieren).
- Gate-Entscheidungen (insbesondere GATE-SECURITY) MUESSEN unabhaengig
  von der implementierenden Partei dokumentiert werden.
- Registry-Dateien unterliegen derselben Branch-Protection und
  Secret-Scanning-Policy wie der Rest des Governance Root
  (ATC-STD-203, SCR-0003).
- Releases ohne bestandenes GATE-SECURITY sind unzulaessig
  (AD-023: G18 Security Audit vor jedem Freeze).

## 24. Compliance

**Verfahren:** Die Einhaltung dieses Standards wird geprueft durch:

1. **ATC Development Consistency Checker** (§20) — automatisiert in CI,
   Registry-getrieben analog tools/atc-std-validator/validate_all.py.
2. **ATC Repository Audit** (ATC-STD-201ff) — Struktur-Compliance je Repo.
3. **Realitaets-Audits** (AD-023-Praxis): Wiki-Aussagen werden gegen den
   tatsaechlichen Code-Zustand geprueft; Abweichungen erzeugen Findings
   (F-NNN) nach ATC-STD-BUG-001.

Bis zur Implementierung des Checkers gilt dieser Standard als
**anwendbar mit UEbergangsfrist**: die manuelle Pruefung der
REQ-STD-302/-018/-019 (Wiki-Wahrheit) erfolgt durch Realitaets-Audits.

## 25. Changelog

| Version | Datum | Aenderung |
|---------|-------|-----------|
| 1.0.0 | 2026-09-07 | Initiale Fassung (Owner-Entwurf Michael Wroblewski, DRAFT→Review). ID-Korrektur 204→300 (Owner-Entscheid: 300er-Block development). Review-Chain + REQ-Matrix durch Aurora ergaenzt. Status: candidate. |

## 26. References

**NORMATIVE**
- ATC-STD-000 — Standards Governance & Specification Standard (Verfassung)
- ATC-STD-201 — Repository Structure Standard
- ATC-STD-202 — Repository Naming & Classification Standard
- ATC-STD-203 — Repository Security & Release Standard
- RFC 2119 / RFC 8174 — Key words for use in RFCs (MUST/SHOULD/MAY)

**INFORMATIVE**
- AD-023 — Quality-Driven Development (kein Launch-Termin)
- AD-026 — Bauhierarchie L0-L7
- AD-040/AD-041 — Bug- & Netz-Standard-Serien (Prozess-Praezedenz)
- AGENT_MANIFEST.md v3.1.1 — Agenten-Onboarding (Root aller Repos)

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

| 1.0.0 | 2026-09-07 | Initiale Fassung; Meta-Sektionen retro-aktiv ergaenzt (SCR-0049) |

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203 · INFORMATIVE: Registry-SSOT standards.yaml
