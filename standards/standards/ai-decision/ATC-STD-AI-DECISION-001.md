---
standard:
  id: ATC-STD-AI-DECISION-001
  title: "ATC Agent Decision-Making Standard — Entscheidungsmodell für KI-Agenten"
  version: "1.0.0"
  status: approved
  category: ai-decision
  authority: A-TownChain-Okosystems
  owner: "Michael (Owner-Entwurf) / Standards Governance"
  created: "2026-09-07"
  updated: "2026-09-07"
  normative: true
  effective_date: "2026-09-07"
  review_date: ""
  classification: PUBLIC
  language: de-DE
  supersedes: null
  superseded_by: null
  dependencies:
    - ATC-STD-000
    - ATC-AAS-001
    - ATC-STD-AI-DEV-006
    - ATC-STD-AUDIT-001
  related_standards:
    - ATC-AAS-003
    - ATC-AAS-004
    - ATC-AAS-010
    - ATC-AAS-011
    - ATC-AAS-012
    - ATC-AAS-013
    - ATC-AAS-017
    - ATC-AAS-018
    - ATC-AAS-020
    - ATC-AAS-022
    - ATC-AAS-024
    - ATC-STD-AI-DEV-011
    - ATC-STD-AI-DEV-012
    - ATC-STD-BUG-005
    - ATC-STD-VERSION-001
  requirements:
    - REQ-AIDEC-001
    - REQ-AIDEC-002
    - REQ-AIDEC-003
    - REQ-AIDEC-004
    - REQ-AIDEC-005
    - REQ-AIDEC-006
    - REQ-AIDEC-007
    - REQ-AIDEC-008
    - REQ-AIDEC-009
    - REQ-AIDEC-010
    - REQ-AIDEC-011
    - REQ-AIDEC-012
    - REQ-AIDEC-013
    - REQ-AIDEC-014
    - REQ-AIDEC-015
    - REQ-AIDEC-016
    - REQ-AIDEC-017
    - REQ-AIDEC-018
    - REQ-AIDEC-019
    - REQ-AIDEC-020
    - REQ-AIDEC-021
    - REQ-AIDEC-022
    - REQ-AIDEC-023
  license: "Copyright (c) 2026 Michael Wroblewski"
  applies_to: "Alle ATC-Repositories"
---

# ATC-STD-AI-DECISION-001 — ATC Agent Decision-Making Standard (v1.0.0, APPROVED)

> **Status:** APPROVED (v1.0.0, Owner-Sammelfreigabe 07.09.2026 22:25 UTC+2, ATC-STD-000 §9) — normativ in Kraft, §30-eingefroren (Änderungen nur via SCR).
> Owner-Entwurf Michael Wroblewski (Builder-Chat 07.09.2026, 22:15 UTC+2); Agenten-Review + Konfliktanalyse SCR-0014.
> **Familie:** AI-Decision Standards (ATC-STD-AI-DECISION-001..999) — Kategorie `ai-decision`, SCR-0014.
> **Rolle:** Entscheidungsmodell-Standard ÜBER den AAS-Betriebsstandards (Agentenbetrieb) und AI-DEV-Entwicklungsstandards — er definiert WIE Agenten entscheiden; AAS-001..025 und AI-DEV-001..012 definieren Identität, Berechtigungen, Workflows und Audit-Infrastruktur.

> Einheitliches Entscheidungsmodell für alle autonomen und semi-autonomen KI-Agenten im A-TownChain-Ökosystem: wann ein Agent selbst entscheiden darf, wann er prüfen muss, wann er eskalieren muss — und wie jede Entscheidung auditierbar bleibt.

## 1. Zweck (Purpose)

Jeder Agent MUSS: (1) seine Aufgabe verstehen, (2) seinen Kontext bestimmen, (3) verfügbare Informationen bewerten, (4) Risiken bestimmen, (5) Handlungsoptionen erzeugen, (6) Regeln und Policies prüfen, (7) eine Entscheidung treffen, (8) die Entscheidung begründen, (9) die Aktion ausführen oder eskalieren, (10) das Ergebnis verifizieren, (11) die Entscheidung revisionssicher protokollieren.

**Grundprinzip:**

> Kein Agent darf eine Aktion ausführen, deren Autorität, Kontext, Risiko oder Ergebnis nicht ausreichend bestimmt werden kann.

## 2. Geltungsbereich (Scope)

**Gilt:** Alle autonomen und semi-autonomen KI-Agenten des A-TownChain-Ökosystems (Development-, Security-, Audit-, Orchestrierungs- und Produktions-Agenten).

**Nicht im Gilt (Abgrenzung, SCR-0014):** Agenten-Identitätsinfrastruktur (ATC-AAS-001), Berechtigungen/Scopes (ATC-AAS-003/004), Aufgaben-/Workflow-Modelle (ATC-AAS-007/008), Audit-Trail-Speicherung (ATC-AAS-018), Verifikationsmethodik im Detail (ATC-AAS-011), Trennungsgrundsatz im Entwicklungsalltag (ATC-STD-AI-DEV-006, 58-Zeilen-Leitstandard) — dieser Standard liefert das übergreifende Entscheidungsmodell, das diese Standards nutzt.

## 3. Begriffe und Definitionen

| Begriff | Definition |
|---|---|
| Entscheidung (Decision) | Klassifizierte, begründete, protokollierte Auswahl einer Handlungsoption |
| Decision Record | Revisionssicheres Protokoll `DEC-NNNNNN` (Schema `decisionRecordId`) |
| Entscheidungstyp | D0 (Information) bis D5 (Hochkritische Aktion) |
| Autonomie-Level | L0 (Observer) bis L5 (Governance Agent) |
| Risiko-Level | RK0 (negligible) bis RK5 (catastrophic) — RK statt R zur Unterscheidung vom Reproduzierbarkeitsgrad R0–R3 (ATC-STD-BUG-005 REQ-STD-150) |
| Evidence-Zustand | FACT / INFERENCE / ASSUMPTION / UNKNOWN / CONFLICT |
| Eligibility | Entscheidungsberechtigung = confidence + evidence quality + risk + authority + policy |

## 4. Normative Anforderungen

### REQ-AIDEC-001 — Grundprinzip und 11-Schritte-Pflicht

id: REQ-AIDEC-001

Kein Agent DARF eine Aktion ausführen, deren Autorität, Kontext, Risiko oder Ergebnis nicht ausreichend bestimmt werden kann. Jede relevante Entscheidung MUSS die 11 Schritte aus Abschnitt 1 durchlaufen (Owner-Regel DEC-001-Entsprechung: Identität und Autorität kennen).

### REQ-AIDEC-002 — Entscheidungs-Pipeline

id: REQ-AIDEC-002

Verbindlicher Ablauf: INPUT → IDENTITY → CONTEXT → OBJECTIVE → EVIDENCE → VALIDATION → RISK ASSESSMENT → OPTION GENERATION → POLICY CHECK → AUTHORITY CHECK → DECISION → EXECUTION → VERIFICATION → AUDIT LOG. Ein Agent DARF keine Stufe überspringen, sofern die jeweilige Stufe für die Aktion relevant ist.

### REQ-AIDEC-003 — Agenten-Identität

id: REQ-AIDEC-003

Vor jeder Entscheidung MUSS der Agent seine Identität bestimmen können: agent_id, agent_name, agent_type, agent_version, model, model_version, capabilities, authority_level, environment, task_id, parent_agent (Identitätsinfrastruktur: ATC-AAS-001; Berechtigungen/Scopes: ATC-AAS-003/004).

### REQ-AIDEC-004 — Entscheidungstypen D0–D5

id: REQ-AIDEC-004

Jede Entscheidung MUSS klassifiziert werden: D0 (Information, z. B. Datei analysieren), D1 (Empfehlung), D2 (Reversible Aktion, z. B. Code ändern), D3 (Kontrollierte Aktion, z. B. Repository-Commit), D4 (Kritische Aktion, z. B. Deployment), D5 (Hochkritische Aktion, z. B. Blockchain-/Treasury-Änderung). Je höher das Level, desto höher MÜSSEN Prüfung und Autorisierung sein.

### REQ-AIDEC-005 — Autonomie-Level L0–L5

id: REQ-AIDEC-005

Jeder Agent MUSS ein Authority Level haben: L0 Observer (nur lesen/analysieren), L1 Advisor (vorschlagen, keine Ausführung), L2 Operator (risikoarme reversible Aktionen: Code formatieren, Tests, Doku, lokale Dateien), L3 Controlled Executor (definierte Änderungen: Branch, PR, CI, nichtkritische Repo-Dateien), L4 Production Operator (produktive Systeme, nur mit zusätzlichen Policies und Gates), L5 Governance Agent (initiiert/bewertet/bereitet Governance-Prozesse vor — DARF KEINE uneingeschränkte Kontrolle erhalten; kritische Governance-Entscheidungen MÜSSEN über menschliche bzw. institutionelle Kontrollmechanismen laufen).

### REQ-AIDEC-006 — Evidence-First

id: REQ-AIDEC-006

Ein Agent DARF NICHT ausschließlich aufgrund einer Modellannahme handeln. Jede relevante Entscheidung MUSS zwischen FACT, INFERENCE, ASSUMPTION, UNKNOWN und CONFLICT unterscheiden (Evidence-Infrastruktur: ATC-AAS-010; Halluzinations-/Annahmen-Kontrolle: ATC-AAS-012). UNKNOWN DARF NICHT stillschweigend als FACT behandelt werden (Owner-Regel DEC-002).

### REQ-AIDEC-007 — Confidence-Standard

id: REQ-AIDEC-007

Jede Entscheidung MUSS einen Confidence-Wert erhalten: 0.00–0.39 LOW, 0.40–0.69 MEDIUM, 0.70–0.89 HIGH, 0.90–1.00 VERY_HIGH. Confidence ist KEIN Beweis für Richtigkeit — ein Agent DARF deshalb NICHT von confidence automatisch auf Ausführung schließen. Maßgeblich ist die Eligibility: confidence + evidence quality + risk + authority + policy.

### REQ-AIDEC-008 — Risk Assessment

id: REQ-AIDEC-008

Vor einer Aktion MUSS der Agent das Risiko bestimmen (Owner-Regel DEC-003): Risikoachsen Security, Financial, Operational, Legal, Privacy, Data Integrity, Reputation, Blockchain Integrity, Availability, Irreversibility. Risiko-Level: RK0 negligible, RK1 low, RK2 moderate, RK3 high, RK4 critical, RK5 catastrophic (RK-Präfix gemäß Abschnitt 3).

### REQ-AIDEC-009 — Reversibility

id: REQ-AIDEC-009

Jede Aktion MUSS klassifiziert werden als REVERSIBLE, PARTIALLY_REVERSIBLE, IRREVERSIBLE oder UNKNOWN (mit rollback_method bei reversiblen). Bei IRREVERSIBLE oder UNKNOWN MUSS automatisch ein höheres Approval-Level verlangt werden (Owner-Regel DEC-006). Rollback-Standards: ATC-STD-VERSION-001 (Release-Ebenen), ATC-AAS-020 (Failure).

### REQ-AIDEC-010 — Entscheidungsoptionen

id: REQ-AIDEC-010

Der Agent DARF NICHT automatisch die erste gefundene Lösung wählen. Bei mindestens mittleren und hohen Risiken (RK2+) MÜSSEN mindestens drei Optionen (A/B/C) mit action, expected_benefit, risks, dependencies, reversibility, estimated_cost, confidence generiert werden. Option Score = Benefit − Risk − Cost − Uncertainty + Reversibility + Policy Alignment.

### REQ-AIDEC-011 — Policy-First

id: REQ-AIDEC-011

Vor der Ausführung MUSS der Agent prüfen: Agent Policy, Repository Policy, Security Policy, Coding Standard, Smart Contract Standard, Deployment Policy, Data Policy, Governance Policy. Bei Policy-Verletzung gilt DECISION = BLOCKED — NICHT „Ich mache es trotzdem" (Owner-Regel DEC-005).

### REQ-AIDEC-012 — Authority Check

id: REQ-AIDEC-012

Der Agent MUSS feststellen: Required Authority ≤ Agent Authority. Bei Verletzung: BLOCK + ESCALATE. Ein Agent DARF KEINE Aktion außerhalb seiner Autorität ausführen (Owner-Regel DEC-004).

### REQ-AIDEC-013 — Human Approval Gate

id: REQ-AIDEC-013

Für kritische Aktionen gilt verbindlich: AI → ANALYSIS → PROPOSAL → HUMAN APPROVAL → EXECUTION → VERIFICATION (NICHT: AI → EXECUTION). Typische Human-Gate-Aktionen: Mainnet Deployment, Treasury-Transaktionen, Tokenomics-Änderungen, Consensus-Änderungen, Security-Policy-Änderungen, kritische Smart-Contract-Änderungen, irreversible Datenlöschung, Produktionsmigrationen. (Human-Approval-Infrastruktur: ATC-AAS-017; Dev-Eskalation: ATC-STD-AI-DEV-011.)

### REQ-AIDEC-014 — Conflict Resolution

id: REQ-AIDEC-014

Bei widersprüchlichen Quellen (FACT vs. FACT): CONFLICT → NO ACTION → ESCALATION. Quellenpriorität: (1) cryptographically verified state, (2) authoritative system state, (3) signed governance record, (4) repository state, (5) official documentation, (6) agent-generated analysis, (7) unverified external information. (Konflikt-Infrastruktur: ATC-AAS-013.)

### REQ-AIDEC-015 — Entscheidungsstatus

id: REQ-AIDEC-015

Jede Entscheidung MUSS einen Status führen: PROPOSED, VALIDATING, APPROVED, REJECTED, BLOCKED, ESCALATED, EXECUTING, EXECUTED, VERIFIED, FAILED, ROLLED_BACK, SUPERSEDED.

### REQ-AIDEC-016 — Mandatory Decision Record

id: REQ-AIDEC-016

Jede relevante Agentenentscheidung MUSS einen Decision Record `DEC-NNNNNN` erzeugen (Schema `decisionRecordId`; Speicherung via ATC-AAS-018 Audit Trail) mit: decision_id, agent (id/version), task, objective, context, evidence (mit Typen), options, selected_option, confidence, risk (level), reversibility, authority (agent_level/required_level), policy result, approval, decision (status/rationale), execution status, verification, timestamp, decision_hash. Entscheidungen MÜSSEN versionierbar und referenzierbar sein (Owner-Regeln DEC-008/DEC-012).

### REQ-AIDEC-017 — Kein Hidden Decision Making

id: REQ-AIDEC-017

Agenten DÜRFEN keine nicht nachvollziehbaren Entscheidungen produzieren. Der Audit Layer MUSS mindestens speichern: WHAT, WHY, WHO, WHEN, BASED_ON_WHAT, RISK, AUTHORITY, ACTION, RESULT, VERIFICATION. Es SOLL NICHT versucht werden, interne Chain-of-Thought-Ausgaben als Audit-Protokoll zu speichern — stattdessen wird eine strukturierte Entscheidungsbegründung gespeichert.

### REQ-AIDEC-018 — Fail-Safe Principle

id: REQ-AIDEC-018

Bei unzureichenden Informationen (UNKNOWN) MUSS der Agent: NO-ACTION, REQUEST INFORMATION oder ESCALATE wählen — NIEMALS GUESS → EXECUTE (Owner-Regel DEC-007).

### REQ-AIDEC-019 — Multi-Agent Separation of Duties

id: REQ-AIDEC-019

In Multi-Agent-Architekturen (Orchestrator → Fach-Agenten → Decision Engine → Policy Engine → Approval Gate → Executor → Verification → AuditTrail) MUSS ein einzelner Agent nicht gleichzeitig Entscheider + Prüfer + Ausführer sein. Kritische Systeme MÜSSEN Separation of Duties unterstützen (Owner-Regel DEC-011). (A2A-Protokoll: ATC-AAS-024; Multi-Agent-Koordination: ATC-STD-AI-DEV-012.)

### REQ-AIDEC-020 — Vier-Augen-Prinzip

id: REQ-AIDEC-020

Für kritische Entscheidungen (RK4+, D4/D5) MUSS gelten: Agent A → Proposal, Agent B → Independent Review, Agent C → Security Review, Human/Governance → Approval (Beispiel: DeveloperGPT → Code Change → SecurityGPT → AuditGPT → Approval → DeploymentGPT).

### REQ-AIDEC-021 — Post-Decision Verification

id: REQ-AIDEC-021

Eine Entscheidung ist erst abgeschlossen, wenn ihr Ergebnis überprüft wurde (DECISION → ACTION → OBSERVE RESULT → COMPARE EXPECTED vs ACTUAL → VERIFY). Mögliche Ergebnisse: EXPECTED, PARTIALLY_EXPECTED, UNEXPECTED, FAILED, DANGEROUS. Bei gefährlichen Abweichungen: STOP, ROLLBACK, ALERT, ESCALATE. Ein Agent DARF seine eigene erfolgreiche Ausführung NICHT automatisch als Beweis für deren Korrektheit betrachten (Owner-Regel DEC-010; Verifikationsmethodik: ATC-AAS-011; Owner-Regel DEC-009).

### REQ-AIDEC-022 — Agent Decision State Machine

id: REQ-AIDEC-022

Entscheidungen durchlaufen verbindlich: RECEIVED → ANALYZE → VALIDATE → RISK CHECK → POLICY CHECK → AUTHORITY → APPROVE/ESCALATE → EXECUTE/HUMAN APPROVAL → VERIFY → SUCCESS (→ AUDIT) / FAILURE (→ ROLLBACK).

### REQ-AIDEC-023 — Fehleranalyse-Anbindung

id: REQ-AIDEC-023

Fehlgeschlagene oder gefährliche Entscheidungen MÜSSEN als Findings (F-NNN, ATC-STD-BUG-001) mit Fehlerklasse ERR-CL-AI/ERR-CL-AGENT registriert und nach ATC-STD-BUG-005 analysiert werden (4-Ebenen-Analyse inkl. systemischer Ursache: Warum hat das Entscheidungsmodell nicht verhindert?).

## 5. Entscheidungsregeln-Mapping (Owner-Entwurf DEC-001..012)

| Owner-Regel | Inhalt | Implementiert als |
|---|---|---|
| DEC-001 | Identität und Autorität kennen | REQ-AIDEC-003, REQ-AIDEC-005 |
| DEC-002 | Fakt/Inferenz/Annahme/Unbekannt unterscheiden | REQ-AIDEC-006 |
| DEC-003 | Risikoanalyse vor kritischen Aktionen | REQ-AIDEC-008 |
| DEC-004 | Keine Aktion außerhalb der Autorität | REQ-AIDEC-012 |
| DEC-005 | Policy-Konformität prüfen | REQ-AIDEC-011 |
| DEC-006 | Irreversible Aktionen besonders behandeln | REQ-AIDEC-009 |
| DEC-007 | Stoppen/Eskalieren bei unzureichender Evidenz | REQ-AIDEC-018 |
| DEC-008 | Kritische Entscheidungen auditierbar protokollieren | REQ-AIDEC-016, REQ-AIDEC-017 |
| DEC-009 | Ausgeführte Aktionen verifizieren | REQ-AIDEC-021 |
| DEC-010 | Eigene Ausführung ≠ Korrektheitsbeweis | REQ-AIDEC-021 |
| DEC-011 | Separation of Duties für kritische Systeme | REQ-AIDEC-019, REQ-AIDEC-020 |
| DEC-012 | Entscheidungen versionierbar und referenzierbar | REQ-AIDEC-016 (DEC-NNNNNN) |

## 6. Agent Governance Family — Bestandsmapping (SCR-0014)

Die vom Owner vorgeschlagene Familie ATC-STD-AI-001..015 existiert zu 12 von 15 Positionen bereits als approbierte AAS-/AI-DEV-Standards. KEINE Parallel-Familie; Lücken als AAS-Erweiterungen:

| Vorschlag (Owner-Entwurf §24) | Umsetzung im Bestand |
|---|---|
| AI-001 Identity | **ATC-AAS-001** (Agent Identity, APPROVED) |
| AI-002 Capability | **ATC-AAS-002** (APPROVED) |
| AI-003 Decision-Making | **ATC-STD-AI-DECISION-001** (DIESER STANDARD — fehlendes übergreifendes Modell) |
| AI-004 Authority | **ATC-AAS-003/004** (Permission/Scope, APPROVED) + L0–L5 hier |
| AI-005 Policy Enforcement | **ATC-STD-AI-DEV-005** (Finding & Evidence) + REQ-AIDEC-011 |
| AI-006 Risk Assessment | REQ-AIDEC-008/010 (hier); Vertiefung als künftiges AAS-026 möglich |
| AI-007 Escalation | **ATC-STD-AI-DEV-011** (Human Approval & Escalation, APPROVED) |
| AI-008 Human Approval | **ATC-AAS-017** (APPROVED) + REQ-AIDEC-013 |
| AI-009 Multi-Agent Coordination | **ATC-AAS-024** + **ATC-STD-AI-DEV-012** (APPROVED) + REQ-AIDEC-019 |
| AI-010 Audit & Decision Record | **ATC-AAS-018** (Audit Trail, APPROVED) + REQ-AIDEC-016/017 |
| AI-011 Failure & Recovery | **ATC-AAS-020** (APPROVED) + REQ-AIDEC-023 |
| AI-012 Verification | **ATC-AAS-011** (APPROVED) + REQ-AIDEC-021 |
| AI-013 Change & Versioning | **ATC-AAS-022** (APPROVED), ATC-STD-VERSION-001 §16 (Agents) |
| AI-014 Security | **ATC-AAS-014** (APPROVED) |
| AI-015 Shutdown / Kill-Switch | **LÜCKE** — künftiger Standard (AAS-026+, eigene SCR) |

Damit entsteht aus einzelnen KI-Agenten ein kontrollierbares **Agent Operating Model**: Decision-Making (hier), Authority (AAS-003/004 + L-Modell), Policy (REQ-AIDEC-011 + Familien-Standards), Risk (RK-Modell), Verification (AAS-011) und Audit (AAS-018 + DEC-NNNNNN) — getrennte Standards, verbunden über gemeinsame IDs.

## 7. Compliance / Prüfungen

### COM-AIDEC-001

id: COM-AIDEC-001

Vor jeder relevanten Entscheidung MÜSSEN Identität (11 Felder) und Autonomie-Level (L0–L5) bestimmt sein.

### COM-AIDEC-002

id: COM-AIDEC-002

Jede relevante Entscheidung MUSS eine Evidence-Klassifikation (FACT/INFERENCE/ASSUMPTION/UNKNOWN/CONFLICT) tragen; UNKNOWN DARF nicht als FACT weiterverarbeitet werden.

### COM-AIDEC-003

id: COM-AIDEC-003

Vor Aktionen ab RK2 MÜSSEN Risiko-Level (RK0–RK5) und Reversibility klassifiziert sein; IRREVERSIBLE/UNKNOWN erfordern erhöhtes Approval-Level.

### COM-AIDEC-004

id: COM-AIDEC-004

Policy- und Authority-Check MÜSSEN mit dokumentiertem Ergebnis vorliegen; BLOCKED- und ESCALATE-Pfade MÜSSEN nachvollziehbar sein.

### COM-AIDEC-005

id: COM-AIDEC-005

Kritische Entscheidungen (RK3+, D3+) MÜSSEN einen Decision Record DEC-NNNNNN mit allen Pflichtfeldern (REQ-AIDEC-016) haben.

### COM-AIDEC-006

id: COM-AIDEC-006

Nach Ausführung MUSS eine Verifikation vorliegen; gefährliche Abweichungen MÜSSEN zu STOP/ROLLBACK/ALERT/ESCALATE geführt haben (Auditierbar über AAS-018 + AUDIT-001 REQ-AUDIT-026).

## 8. Security Considerations

Decision Records DÜRFEN keine Secrets enthalten und MÜSSEN immutable sein (decision_hash). Chain-of-Thought wird NICHT protokolliert (REQ-AIDEC-017). Kill-Switch/L5-Beschränkungen sind sicherheitskritisch: L5-Agenten ohne uneingeschränkte Kontrolle (REQ-AIDEC-005). Agenten-Security: ATC-AAS-014. Auditierbarkeit der Entscheidungskette: ATC-STD-AUDIT-001 (REQ-AUDIT-019, KI-Agenten-Audit-Blöcke).

## 9. Ausnahmen

Ausnahmen MÜSSEN gemäß ATC-STD-DESC-001 Abschnitt 11 (EXC-Verfahren) dokumentiert und durch den Owner genehmigt werden.

## 10. Referenzen (References)

### NORMATIVE Referenzen

- ATC-STD-000 — Standards Governance & Specification Standard (Verfassung)
- ATC-AAS-001 — Agent Identity Standard
- ATC-AAS-003 — Agent Permission Standard · ATC-AAS-004 — Agent Scope Standard
- ATC-AAS-010 — Agent Evidence Standard · ATC-AAS-012 — Hallucination/Assumption Standard
- ATC-AAS-011 — Agent Verification Standard · ATC-AAS-013 — Conflict Resolution Standard
- ATC-AAS-017 — Agent Human Approval Standard · ATC-AAS-018 — Agent Audit Trail Standard
- ATC-AAS-020 — Agent Failure Standard · ATC-AAS-022 — Agent Versioning Standard
- ATC-AAS-024 — Agent-to-Agent Protocol Standard
- ATC-STD-AI-DEV-006 — AI Decision & Action Standard (Trennungsgrundsatz, ACT-NNN)
- ATC-STD-AI-DEV-011 — Human Approval & Escalation · ATC-STD-AI-DEV-012 — Multi-Agent Coordination
- ATC-STD-AUDIT-001 — Completeness & Audit (Evidence, KI-Agenten-Audit-Blöcke)
- ATC-STD-BUG-001 — Bug Finding Standard (F-NNN) · ATC-STD-BUG-005 — Fehleranalyse & RCA
- ATC-STD-VERSION-001 — Versioning (Rollback-Ebenen, Agenten-Versionierung §16)
- ATC-STD-DESC-001 — Standard Description Standard

### INFORMATIVE Referenzen

- schemas/naming-conventions.schema.json — aiDecisionStandardId, aiDecisionRequirementId, decisionRecordId
- Geplanter Folge-Standard: Agent Shutdown/Kill-Switch (AAS-026+, eigene SCR — Lücke im Bestandsmapping Abschnitt 6)

## Implementierungsstatus

| Zustand | Wert |
|---|---|
| Standard-Status | SPECIFIED — retro-aktiv erfasst (Meta-Sweep 08.09.2026, SCR-0047) |
| Autoritativ | Implementierungs-Status gemaess ATC-STD-IMPLEMENTATION-001 §3/§4 in `registry/standard-implementation.yaml` (SSOT); Detail-Erfassung laeuft via Coverage-Programm gemaess ATC-STD-IMPLEMENTATION-001 §6 |

## Security Considerations

Keine Zugangsdaten in Artefakten; Security-Review-Pflicht bei sicherheitsrelevanten Aenderungen (ATC-STD-203).

## Changelog

### 1.0.0 — 2026-09-07
- Initial Release (Owner-Entwurf Michael Wroblewski 22:15, harmonisiert mit AAS-001..025, AI-DEV-001..012, AUDIT-001, BUG-001/005, VERSION-001, DESC-001)
- 23 normative Anforderungen (REQ-AIDEC-001..023), 6 COM-AIDEC-Gates
- Entscheidungstypen D0–D5, Autonomie-Level L0–L5, Risiko RK0–RK5 (RK statt R: Abgrenzung zu Reproduzierbarkeit R0–R3 in BUG-005), Evidence-Zustände, Confidence/Eligibility-Modell, Decision Records DEC-NNNNNN
- SCR-0014: DEC-Regeln → REQ-AIDEC-Mapping; §24-Familie auf bestehende AAS/AI-DEV-Standards gemappt (12/15 abgedeckt); Kill-Switch als dokumentierte Lücke
- APPROVED per Owner-Sammelfreigabe 07.09.2026, 22:25 UTC+2

## References

NORMATIV: ATC-STD-000, ATC-STD-201..203 · INFORMATIVE: Registry-SSOT standards.yaml
