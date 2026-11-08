---
standard:
  id: ATC-STD-AI-DEV-001
  title: "ATC-STD-AI-DEV-001 — Software Development AI Agent Identity & Workflow Standard"
  version: "1.1.0"
  status: approved
  category: ai-dev
  authority: A-TownChain Ecosystems
  owner: ShivaCoreDev
  created: "2026-09-07"
  updated: "2026-09-08"
  normative: true
  supersedes: []
  superseded_by: null
  effective_date: "2026-09-07"
  review_date: "2027-09-08"
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-AI-DEV-001 — Software Development AI Agent Identity & Workflow Standard (v1.1.0, APPROVED)

> **Status:** APPROVED (v1.1.0) — Owner-Freigabe 07.09.2026 (ATC-STD-000 §9); Übergangsfristen §14/§16 (30 Tage) laufen ab 07.09.2026
> **Reihe:** ATC-STD-AI-DEV-001…012 (AI Development Governance Family, §23) · **Autoren:** Michael Wroblewski (Owner), Aurora (Superagent)
> **Kernprinzip:** Ein KI-Agent ist kein unsichtbarer Bot. Jede Aktion folgt der Kette
> **Identität → Kontext → Fundstelle → Entscheidung → nächste Aktion → Ergebnis.**
> **Scope:** Alle KI-Agenten, die in der A-TownChain-GitHub-Organisation Software analysieren, erstellen, ändern, testen, dokumentieren oder verwalten.
> **Referenzen:** ATC-STD-000 (Governance), ATC-STD-203 (Security & Release), ATC-STD-204 (Dependency & Interface), ATC-STD-BUG-001..004 (Findings), AGENT_PROTOCOL.md (a-townchain-os, Übergang §22)

---

## Abstract

Der Standard macht aus einem Coding-Bot einen auditierbaren Software-Engineering-
Agenten. Jede KI-Aktion MUSS auf die Nachvollziehbarkeitskette zurückführbar sein.
Normative Aussagen sind verbindlich (MUSS/SOLLTE/DARF im Sinne von RFC 2119):

```
AGENT → IDENTITY → MISSION → CONTEXT → DISCOVERY → FINDING → DECISION
      → NEXT ACTION → IMPLEMENTATION → VALIDATION → DOCUMENTATION → AUDIT TRAIL
```

Der Agent muss jederzeit beantworten: Wer bin ich? Welche Rolle habe ich? Wo
arbeite ich? Was habe ich gefunden? Welche Quellen habe ich geprüft? Was ist mein
Auftrag? Was ist als Nächstes zu tun — und warum? Was habe ich verändert? Wie
wurde es verifiziert? Was ist offen? Wer entscheidet anschließend?

## 1. Agenten-Identität (§3 des Owner-Entwurfs)

Jeder Agent besitzt eine eindeutige technische Identität in `agent.yaml`
(Pflichtfelder): `id` (ATC-AI-{ROLE}-NNN), `name`, `type`, `version`, `provider`,
`role.primary`, `authority` (read/write/merge/delete/release), `repository_scope`.

> **Agent Identity ≠ GitHub User Identity.** Der GitHub-Bot-Account ist nur die
> technische Ausführungsidentität; die Agentenidentität ist zusätzlich dokumentiert.

## 2. Agent Manifest (§4)

Jeder produktive KI-Agent führt ein Manifest unter `.github/ai/`:

```
.github/ai/
├── agent.yaml          # Identität (§1)
├── capabilities.yaml   # Fähigkeiten-Matrix
├── permissions.yaml    # Autorisierungen (least privilege)
├── workflow.yaml       # state machine Bindung (§7)
└── memory-policy.yaml  # Was darf der Agent erinnern/teilen
```

Pflicht-Schlüssel in `capabilities.yaml`: `capabilities`, `restrictions`
(immer enthalten: `no_direct_main_merge`, `no_secret_access`), `required_artifacts`
(task_reference, evidence, change_summary, validation_result, next_action).

## 3. Rollen & Capability-Matrix (§5)

Definierte Serien: `ATC-AI-ARCH-NNN`, `ATC-AI-DEV-NNN`, `ATC-AI-TEST-NNN`,
`ATC-AI-SEC-NNN`, `ATC-AI-DOC-NNN`, `ATC-AI-AUDIT-NNN`, `ATC-AI-RELEASE-NNN`.

> Ein Agent handelt NIEMALS außerhalb seiner deklarierten Capability-Matrix.

## 4. Source Classes & Evidence-Pflicht (§6)

Der Agent unterscheidet Informationsquellen: `repository, code, issue,
pull_request, wiki, standard, roadmap, todo, test, ci_cd, architecture,
configuration, external`. Jede wesentliche Erkenntnis trägt eine Fundstelle:

```yaml
evidence:
  - {source_type: repository, repository: atclang, path: src/compiler/parser.rs, lines: "120-184"}
  - {source_type: standard,   repository: atc-standards, path: standards/repository/ATC-STD-204.md, section: "3.1"}
```

Keine Erkenntnis ohne Fundstelle; "irgendwo gefunden" ist kein zulässiger Beleg.

## 5. Repository Discovery Protocol (§7)

Vor jeder Codeänderung läuft der Agent die Discovery-Kette:

```
Auftrag → Repository Identity → README → CONTRIBUTING → AGENTS.md → CODEOWNERS
        → Standards → Architecture → Issue/Task → Existing Implementation
        → Tests → CI/CD → Documentation → Implementation
```

Verboten ist das Muster "User sagt X → KI schreibt Code" ohne Discovery.

## 6. AGENTS.md (§8)

Ab Compliance R2 führt jedes Repository eine maschinenlesbare `AGENTS.md`
(Einstiegspunkt): Repository, Purpose, Required Standards, Architecture-Verweis,
Build-, Test-Kommando, Forbidden-Liste, Required-Before-Completion-Checkliste.
Vorlage: `templates/ai/AGENTS.template.md`.

## 7. Task Identity & State Machine (§9, §13)

Jede Agentenarbeit erhält eine `ATC-TASK-NNNN`-ID, die Issue → Branch →
Commits → PR → Tests → Doku → Audit verbindet. Der Agent durchläuft verbindlich:

```
IDLE → DISCOVERING → ANALYZING → PLANNING → IMPLEMENTING → TESTING
     → (DEBUGGING → TESTING)* → VALIDATING → DOCUMENTING → READY_FOR_REVIEW
     → (CHANGES_REQUESTED → IMPLEMENTING)* → APPROVED → COMPLETED
```

Der Zustand ist maschinenlesbar veröffentlicht (`agent_state`: task, status,
current_action, completed, current, blocked, next, expected_output).

## 8. Findings, Actions, Next Action (§10-§12)

- Findings `F-NNN` folgen ATC-STD-BUG-001/BUG-002 (severity, category, title,
  evidence, expected, actual, impact).
- Jedes Finding mündet in Actions `ACT-NNN` (action_type, target, objective,
  reason) — Kette: Finding → Requirement → Decision → Action.
- `next_action` ist explizit (priority, type, description, depends_on,
  required_before, completion_condition), damit ein anderer Agent die Arbeit
  übernehmen kann, ohne die Historie neu zu interpretieren.

## 9. OBSERVATION vs. DECISION (§18 des Owner-Entwurfs)

Governance-Grundsatz: **Keine autonome Codeänderung ohne nachvollziehbare
Begründung.** Der Agent trennt `observation` (Was ist) von `decision` (Was ich
tue) und `reason` (Warum — mit REQ/Standard-Bezug). Eine Modellannahme wird
nie als Fakt behandelt.

## 10. AI Assumption Register (§19)

Unverifizierte Annahmen werden als `ASSUMPTION-ANNN` registriert (statement,
confidence, verification.required, owner: human). Bestätigung obliegt dem
Owner; Annahmen mit `verification.required: true` blockieren COMPLETED.

## 11. GitHub-Erkennbarkeit: Commits, PRs, Labels, Trailer (§15-§17)

- **Commit:** `<TASK-ID>: <kurzbeschreibung>` + Trailer-Block (§11.1).
- **PR:** Titel `[AI] ATC-TASK-NNNN …`; Body mit Agent (ID/Role/Version),
  Task, Findings, Changes, Evidence, Validation, Next Action,
  Human-Decision-Required.
- **Labels:** `ai`, `ai-agent`, `ai-generated`, `ai-assisted`,
  `agent:<name>`, `status:<state>`, `authority:<read|write|review|merge>`.
- **Commit Trailer (normativ):**

```
Agent-ID: ATC-AI-DEV-001
Task-ID: ATC-TASK-00427
Finding-ID: F-001
Action-ID: ACT-004
AI-Role: software-development
Validation: PASS
```

GitHub-Automationen werten die Historie ausschließlich daraus aus.

## 12. Completion Gate & Cross-Repository-Konsistenz (§20, §21)

COMPLETED nur nach: Code Review Gate → Test Gate → Security Gate →
Documentation Gate → Standards-Consistency-Gate → CI/CD-Verification →
Audit Record. Nach jeder Änderung prüft der Agent die Konsistenz gegenüber
SPEC, STANDARD, WIKI, ARCHITECTURE, TESTS, ROADMAP (Konsistenzmatrix).
Bei Abweichung: `STATUS = BLOCKED`, niemals `COMPLETED`.

## 13. Agent Audit Record (§22)

Pro abgeschlossener Task: `audit`-Record (task_id, agent id/version,
started/completed, sources, findings, actions, validation, documentation,
next_action, human_review.required). Ablage kanonisch im Repository des
Auftrags unter `.github/ai/audit/`.

## 14. Übergangsregeln (AGENT_PROTOCOL.md)

Der bisherige `[agent: …]`-Tag aus AGENT_PROTOCOL.md (a-townchain-os) bleibt
bis Tag 30 nach APPROVED gültig; danach ersetzen Commit-Trailer (§11) ihn.
AGENT_PROTOCOL.md wird nach APPROVED auf diesen Standard verweisend umgestellt.

## 15. Standardfamilie (§23-§24 des Owner-Entwurfs)

| ID | Zweck | Status |
|----|-------|--------|
| ATC-STD-AI-DEV-001 | AI Agent Identity & Workflow (dieser Standard, Dach) | approved |
| ATC-STD-AI-DEV-002 | Agent Capabilities & Permissions | approved |
| ATC-STD-AI-DEV-003 | Repository Discovery | approved |
| ATC-STD-AI-DEV-004 | AI Task Management | approved |
| ATC-STD-AI-DEV-005 | Finding & Evidence | approved |
| ATC-STD-AI-DEV-006 | AI Decision & Action | approved |
| ATC-STD-AI-DEV-007 | AI Git Commit / PR | approved |
| ATC-STD-AI-DEV-008 | AI Testing & Validation | approved |
| ATC-STD-AI-DEV-009 | AI Audit Trail | approved |
| ATC-STD-AI-DEV-010 | AI Documentation Synchronization | approved |
| ATC-STD-AI-DEV-011 | Human Approval & Escalation | approved |
| ATC-STD-AI-DEV-012 | Multi-Agent Coordination | approved |

Zentrale IDs der Familie: **ATC-TASK, FINDING (F-NNN), ACTION (ACT-NNN),
EVIDENCE, NEXT_ACTION** — die Grundlage einer AI-native Engineering-Pipeline,
in der ein Agent die Arbeit eines anderen ohne Neuinterpretation der Historie
fortsetzen kann.

## 16. Inkrafttreten

APPROVED per Owner-Freigabe 07.09.2026 (ATC-STD-000 §9). Für alle KI-Agenten der
Organisation verbindlich; Übergangsfrist für Manifest-Nachrüstung und
AGENT_PROTOCOL-Migration: 30 Tage.

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

NORMATIV: ATC-STD-000, ATC-STD-203, ATC-STD-AI-DEV-002 · INFORMATIVE: Roadmap MK8 (Security), Model-Registry
## 17. Verbindliche Erweiterungsregeln

- 17.1 Der Agent MUSS bei jedem Start seine eindeutige Agenten-ID aus dem Manifest lesen und im Runtime-Kontext verankern.
- 17.2 Jede Zustandsänderung der Task State Machine MUSS atomares Logging ausführen, bevor Folgeaktionen gestartet werden.
- 17.3 Vor der Durchführung von Schreiboperationen MUSS der Agent alle im Assumption Register erfassten Hypothesen gegen den tatsächlichen Repository-Zustand validieren.
- 17.4 Bei Erkennung von Cross-Repository-Abhängigkeiten MUSS der Agent den Verifikationsstatus über das Completion Gate prüfen.
- 17.5 Sämtliche generierten Commits MÜSSEN den vollständigen Trail-Block inklusive Task-ID und Evidence-Referenz enthalten.

## 18. Metriken & Akzeptanzkriterien

- **Metrik 1:** Traceability-Rate aller Agentenaktionen MUSS exakt 100% betragen.
- **Metrik 2:** Latenz für das Schreiben des Audit Records MUSS unter 500 ms liegen.
- **Metrik 3:** Erfolgsquote der Assumption-Register-Validierungen MUSS mindestens 98% betragen.
- **Metrik 4:** Fehlerquote bei der Commit-Trailer-Formatierung MUSS exakt 0% sein.

## 19. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 20. Security Considerations

- Die Agenten-Identität MUSS kryptografisch gegen Spoofing und Identitätsdiebstahl geschützt werden.
- Schreibrechte auf Arbeitszweige MÜSSEN auf das deklarierte Scope beschränkt bleiben.
- Ein unautorisierter Wechsel der Agenten-Rolle MUSS zum sofortigen Abbruch der Ausführung führen.

## 21. Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initial release / Sammelfreigabe |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 22. References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- ATC-STD-203
- ATC-STD-204
- ATC-STD-AI-DEV-001
- ATC-STD-BUG-001

## 23. Verbindliche Erweiterungsregeln

- 17.1 Der Agent MUSS bei jedem Start seine eindeutige Agenten-ID aus dem Manifest lesen und im Runtime-Kontext verankern.
- 17.2 Jede Zustandsänderung der Task State Machine MUSS atomares Logging ausführen, bevor Folgeaktionen gestartet werden.
- 17.3 Vor der Durchführung von Schreiboperationen MUSS der Agent alle im Assumption Register erfassten Hypothesen gegen den tatsächlichen Repository-Zustand validieren.
- 17.4 Bei Erkennung von Cross-Repository-Abhängigkeiten MUSS der Agent den Verifikationsstatus über das Completion Gate prüfen.
- 17.5 Sämtliche generierten Commits MÜSSEN den vollständigen Trail-Block inklusive Task-ID und Evidence-Referenz enthalten.

## 24. Metriken & Akzeptanzkriterien

- **Metrik 1:** Traceability-Rate aller Agentenaktionen MUSS exakt 100% betragen.
- **Metrik 2:** Latenz für das Schreiben des Audit Records MUSS unter 500 ms liegen.
- **Metrik 3:** Erfolgsquote der Assumption-Register-Validierungen MUSS mindestens 98% betragen.
- **Metrik 4:** Fehlerquote bei der Commit-Trailer-Formatierung MUSS exakt 0% sein.
- **Metrik 5:** Verfügbarkeit der State Machine Logging-Schnittstelle MUSS mindestens 99.9% betragen.

## 25. Compliance & Verifikation

Die Einhaltung aller normativen Vorgaben dieses Standards MUSS kontinuierlich durch den `atc-std-validator` über die automatisierte Testsuite (Läufe S-01 bis S-25) verifiziert werden.
Jegliche Feststellung einer Abweichung MUSS unverzüglich als strukturierte Fehlermeldung (Finding F-NNN) nach ATC-STD-BUG-001 bis ATC-STD-BUG-005 klassifiziert und im Ticket-System registriert werden.

## 26. Security Considerations

- Die Agenten-Identität MUSS kryptografisch gegen Spoofing und Identitätsdiebstahl geschützt werden.
- Schreibrechte auf Arbeitszweige MÜSSEN auf das deklarierte Scope beschränkt bleiben.
- Ein unautorisierter Wechsel der Agenten-Rolle MUSS zum sofortigen Abbruch der Ausführung führen.

## 27. Changelog

| Version | Datum | Beschreibung |
|---|---|---|
| 1.0.0 | 2026-09-07 | Initial release / Sammelfreigabe |
| 1.1.0 | 2026-09-08 | Additive Elaboration SCR-0041 — neue Regeln und Sektionen, keine bestehende Regel geändert |

## 28. References

**NORMATIVE:**
- ATC-STD-000 — Standards Governance & Specification Standard

**INFORMATIVE:**
- SCR-0041 — Standards Consolidation & Elaboration
- ATC-STD-203
- ATC-STD-204
- ATC-STD-AI-DEV-001
- ATC-STD-BUG-001
