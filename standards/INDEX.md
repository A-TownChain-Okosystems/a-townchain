---
document:
  id: ATC-STD-INDEX-001
  type: master-index
  version: "1.0.0"
  status: GENERATED
  normative: false
  owner: "A-TownChain Okosystems (Michael Wroblewski)"
  generated: "2026-10-07"
  generator: "tools/index/gen_index.py (SCR-0038)"
  sources: "registry/standards.yaml + registry/framework.yaml + registry/categories.yaml + registry/taxonomy.yaml + registry/versions.yaml + registry/protocol-registry.yaml + registry/findings.yaml"
  license: "Copyright (c) 2026 Michael Wroblewski"
----

# ATC Standards Index — Master Index (ATC-STD-INDEX-001, v1.0.0)

> **Nicht-normativ · generiert.** Diese Datei ist der zentrale Einstiegspunkt in
> das ATC-Standards-System — sie enthält KEINE eigenen Fachwahrheiten. Sie wird
> vollständig aus den SSOT-Registern generiert (2026-10-07, SCR-0038); manuelle
> Änderungen sind verboten (Regeneration: `python3 tools/index/gen_index.py`).
> **SSOT-Kaskade bei Konflikten:** Governance (ATC-STD-000) → Standard →
> Registry → INDEX → Implementierung.

## 1. Zweck

Der Index beantwortet: welche Standards existieren, wo sie liegen, zu welcher
Familie sie gehören, welchen Status/welche Version sie haben — und verweist auf
Abhängigkeiten, offene Punkte und Ersatz-Beziehungen. Die normative Wahrheit
eines Standards liegt ausschließlich in seiner Standarddatei; die Registry
(registry/standards.yaml) ist das SSOT des Bestands.

## 2. Die Register (SSOT-Ebene) — alle maschinenlesbar

| Registry | Inhalt | Bindender Standard |
|---|---|---|
| registry/agents.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/architecture.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/categories.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/code-quality-matrix.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/compliance_state.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/dependencies.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/findings.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/framework.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/improvements.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/interfaces.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/lifecycle.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/maintenance-conformance.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/maintenance-requirements.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/milestones.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/networks.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/protocol-conformance.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/protocol-registry.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/protocol-security.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/releases.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/repo-audit-checks.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/repositories.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/requirements.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/security.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/standard-implementation.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/standards.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/taxonomy.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/teams.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/technology-registry.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |
| registry/versions.yaml | Bestand/Status der zugehörigen Domäne | — siehe registry/standards.yaml je Eintrag |

Kernregister: **standards.yaml** (Bestand, 532 Standards) · **versions.yaml**
(Versionierung je Standard) · **framework.yaml** (Katalog: 76 Familien,
659 Slots) · **categories.yaml**
(Kategorien) · **taxonomy.yaml** (Domain/Familie/Kategorie) · **protocol-registry.yaml**
(26 Protokollfamilien, Status {'draft': 11, 'planned': 15}) ·
**findings.yaml** (Findings: 90 OPEN / 70 RESOLVED von 160).

## 3. Standardfamilien (Katalog, 76 Familien)

| FAM | Familie | Slots | BELEGT | VERWEIST |
|---|---|---|---|---|
| FAM-01 | Enterprise & Governance | 20 | 12 | 8 |
| FAM-02 | Standards-Governance | 14 | 3 | 11 |
| FAM-03 | Repository Standards | 15 | 7 | 8 |
| FAM-04 | Dokumentationsstandards | 16 | 8 | 8 |
| FAM-05 | Software Development | 16 | 12 | 4 |
| FAM-06 | Git & Version Control | 14 | 6 | 8 |
| FAM-07 | Bug & Fehler-Management | 12 | 2 | 10 |
| FAM-08 | Testing & Quality Assurance | 14 | 9 | 5 |
| FAM-09 | CI/CD & DevOps | 15 | 9 | 6 |
| FAM-10 | Release Readiness (RR-Gates) | 8 | 0 | 8 |
| FAM-11 | Blockchain Standards | 19 | 18 | 1 |
| FAM-12 | Token Standards | 12 | 11 | 1 |
| FAM-13 | Smart Contracts | 11 | 0 | 11 |
| FAM-14 | Interoperability | 11 | 10 | 1 |
| FAM-15 | ZKP / Privacy | 8 | 0 | 8 |
| FAM-16 | Oracle & External Data | 7 | 7 | 0 |
| FAM-17 | Identity & Reputation | 7 | 6 | 1 |
| FAM-18 | Cybersecurity | 14 | 11 | 3 |
| FAM-19 | AI-Agent Standards | 16 | 0 | 16 |
| FAM-20 | Agent Operating (KI-Softwareentwicklungsagent) | 1 | 1 | 0 |
| FAM-21 | Mining Standards | 11 | 11 | 0 |
| FAM-22 | Wallet Standards | 9 | 9 | 0 |
| FAM-23 | DeFi Standards | 10 | 10 | 0 |
| FAM-24 | NFT / Marketplace | 10 | 10 | 0 |
| FAM-25 | GameFi / Shivamon | 15 | 15 | 0 |
| FAM-26 | API Standards | 11 | 10 | 1 |
| FAM-27 | Datenstandards | 10 | 8 | 2 |
| FAM-28 | Observability | 9 | 8 | 1 |
| FAM-29 | Incident & Recovery | 10 | 8 | 2 |
| FAM-30 | Release & Update Standards | 10 | 0 | 10 |
| FAM-31 | Projektmanagement | 10 | 7 | 3 |
| FAM-32 | Requirements Engineering | 7 | 4 | 3 |
| FAM-33 | UI/UX | 8 | 8 | 0 |
| FAM-34 | Mobile / Desktop / OS | 8 | 8 | 0 |
| FAM-35 | ATCLang | 10 | 6 | 4 |
| FAM-36 | AuditTrail / LogChain | 7 | 4 | 3 |
| FAM-37 | Supply Chain & Dependencies | 8 | 6 | 2 |
| FAM-38 | Open Source & Lizenzierung | 7 | 7 | 0 |
| FAM-39 | Business / Economics | 7 | 7 | 0 |
| FAM-40 | Master-Audit | 1 | 1 | 0 |
| FAM-41 | Repository Audit | 3 | 3 | 0 |
| FAM-42 | Protocol Standards | 3 | 3 | 0 |
| FAM-43 | Standards Governance Core | 4 | 4 | 0 |
| FAM-44 | ATC License System | 9 | 9 | 0 |
| FAM-45 | Vision-to-Software (V2S) | 27 | 27 | 0 |
| FAM-46 | Repository Maintenance (REPO-MAINT) | 1 | 1 | 0 |
| FAM-47 | Error Propagation & Prevention (ERR) | 16 | 16 | 0 |
| FAM-48 | Implementation Tracking (IMPLEMENTATION) | 1 | 1 | 0 |
| FAM-49 | Repository Content Discovery (REPO-DISCOVERY) | 10 | 10 | 0 |
| FAM-50 | CI/CD-Standards (CICD) | 1 | 1 | 0 |
| FAM-51 | Technology Uniqueness & Differentiation (TUD) | 1 | 1 | 0 |
| FAM-52 | Maintenance (MAINT) | 25 | 25 | 0 |
| FAM-53 | Bug & Fehler-Reports (BUG) | 5 | 5 | 0 |
| FAM-54 | Zero-Knowledge Proofs (ZKP) | 10 | 10 | 0 |
| FAM-55 | AI-Assisted Development (AI-DEV) | 12 | 12 | 0 |
| FAM-56 | AI-Agent Standards (AAS) | 25 | 25 | 0 |
| FAM-57 | Enterprise Standards (ATC-ENT) | 23 | 23 | 0 |
| FAM-58 | README Governance (README) | 1 | 1 | 0 |
| FAM-59 | Markdown & Metadata (MD) | 1 | 1 | 0 |
| FAM-60 | Smart Contracts (SC) | 20 | 20 | 0 |
| FAM-61 | Network Standards (NET) | 8 | 8 | 0 |
| FAM-62 | Description Standards (DESC) | 1 | 1 | 0 |
| FAM-63 | Versioning Standards (VERSION) | 1 | 1 | 0 |
| FAM-64 | Audit Standards (AUDIT) | 1 | 1 | 0 |
| FAM-65 | AI Decision Records (AI-DECISION) | 1 | 1 | 0 |
| FAM-66 | Update Standards (UPDATE) | 1 | 1 | 0 |
| FAM-67 | Framework Standards (FRAMEWORK) | 1 | 1 | 0 |
| FAM-68 | Milestone Standards (MILESTONE) | 1 | 1 | 0 |
| FAM-69 | Compatibility Standards (COMPAT) | 1 | 1 | 0 |
| FAM-70 | Improvement Standards (IMPROVEMENT) | 1 | 1 | 0 |
| FAM-71 | AI Governance (ATC-AI-GOV) | 11 | 11 | 0 |
| FAM-72 | Engineering Standards (ENG) | 1 | 1 | 0 |
| FAM-73 | Legal Standards (LEGAL) | 1 | 1 | 0 |
| FAM-74 | Governance Constitution (GOV) | 1 | 1 | 0 |
| FAM-75 | System Architecture (ARCH) | 1 | 1 | 0 |
| FAM-76 | Aurora Swarm Network (AURORA-SWARM) | 1 | 1 | 0 |

Statusverteilung der 532 Registry-Standards: {'approved': 505, 'candidate': 27}.
Alle 532 sind APPROVED und normativ (§30-eingefroren); Details je Standard
in registry/standards.yaml und registry/versions.yaml.

## 4. Master-Registry-Tabelle (532 Standards)

Sortiert nach ID; Version = aktuelle Registry-Version; Status = Registry-Status.

| Standard-ID | Titel | Kategorie | Version | Status | Datei |
|---|---|---|---|---|---|
| ATC-AAS-001 | Agent Identity Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-001.md |
| ATC-AAS-002 | Agent Capability Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-002.md |
| ATC-AAS-003 | Agent Permission Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-003.md |
| ATC-AAS-004 | Agent Scope Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-004.md |
| ATC-AAS-005 | Agent Discovery Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-005.md |
| ATC-AAS-006 | Agent Context Standard (P1) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-006.md |
| ATC-AAS-007 | Agent Task Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-007.md |
| ATC-AAS-008 | Agent Workflow Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-008.md |
| ATC-AAS-009 | Agent Change Standard (P1) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-009.md |
| ATC-AAS-010 | Agent Evidence Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-010.md |
| ATC-AAS-011 | Agent Verification Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-011.md |
| ATC-AAS-012 | Agent Hallucination / Assumption Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-012.md |
| ATC-AAS-013 | Agent Conflict Resolution Standard (P1) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-013.md |
| ATC-AAS-014 | Agent Security Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-014.md |
| ATC-AAS-015 | Agent Git Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-015.md |
| ATC-AAS-016 | Agent PR Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-016.md |
| ATC-AAS-017 | Agent Human Approval Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-017.md |
| ATC-AAS-018 | Agent Audit Trail Standard (P0) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-018.md |
| ATC-AAS-019 | Agent Handoff Standard (P1) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-019.md |
| ATC-AAS-020 | Agent Failure Standard (P1) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-020.md |
| ATC-AAS-021 | Agent Quality Standard (P2) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-021.md |
| ATC-AAS-022 | Agent Versioning Standard (P1) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-022.md |
| ATC-AAS-023 | Agent Role Standard (P2) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-023.md |
| ATC-AAS-024 | Agent-to-Agent Protocol Standard (P1) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-024.md |
| ATC-AAS-025 | Agent Repository Manifest Standard (P1) | aas | 1.1.0 | approved | standards/aas/ATC-AAS-025.md |
| ATC-AI-GOV-001 | ATC Agent Governance Framework v1.0 — Produktionsarchitektur (18-stufige Governance-Kette, 4 Kontrollprinzipien: Fail Closed / Evidence First / Persistent Findings / No Self-Certification, Severity-Modell mit Repository-Status-Berechnung, Zuständigkeits-Trennung, 7-Phasen-Roadmap) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-001.md |
| ATC-AI-GOV-ACCESS-001 | ATC Agent Governance — AI Access Role Model (Rollenprofile ATC-AUDITOR/ATC-DEVELOPER/ATC-CI, Omni-Verbot, Stufe-3-Reserve, Token-Hygiene, Access Review) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-ACCESS-001.md |
| ATC-AI-GOV-AGENTS-001 | ATC Agent Governance — Organisationsweite Arbeitsregeln (Discovery, Hierarchie, Registry-Binding, Session-Mandat, Readiness) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-AGENTS-001.md |
| ATC-AI-GOV-AUDIT-001 | ATC Agent Governance — Auditverfahren (AGOV-FULL/DELTA/GATE, Snapshot, Readiness, Post-Change-Audit) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-AUDIT-001.md |
| ATC-AI-GOV-CAPABILITY-001 | ATC Agent Governance — Capability & Authorization Model (explizite Berechtigungen, Autorisierungskette, Owner-Gates fuer P0-Capabilities, Entzug/Suspendierung) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-CAPABILITY-001.md |
| ATC-AI-GOV-CHANGE-001 | ATC Agent Governance — Governance Change Management (SCR-Pflicht, SemVer, Vorher/Nachher, Rollback) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-CHANGE-001.md |
| ATC-AI-GOV-CHECK-001 | ATC Agent Governance — Automatisierte Compliance Checks (AGOV-CHECK-001..020 versioniert, PASS/FAIL/WARN/N/A) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-CHECK-001.md |
| ATC-AI-GOV-FINDING-001 | ATC Agent Governance — Persistent Findings (ATC-FINDING-YYYY-NNNNNN, Lifecycle bis VERIFIED, Verifikations-Wahrheit, deterministische Repository-Status-Aggregation, Registry-Kopplung registry_ref) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-FINDING-001.md |
| ATC-AI-GOV-HANDOFF-001 | ATC Agent Governance — Agent-to-Agent Übergabe (10 Pflichtfelder, Kontinuität, Auditierbarkeit) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-HANDOFF-001.md |
| ATC-AI-GOV-INCIDENT-001 | ATC Agent Governance — Incident & Fehlerbehandlung (6 Klassen, Lifecycle, RCA-Pflicht, Postmortem) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-INCIDENT-001.md |
| ATC-AI-GOV-MANIFEST-001 | ATC Agent Governance — Agent Identity & Scope (Manifest, Registry, Capabilities, Status-Modell) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-MANIFEST-001.md |
| ATC-AI-GOV-POLICY-001 | ATC Agent Governance — Maschinenlesbare Policies (ATC-POL-001..010, MUST/SHOULD/MAY, Verdikt-Modell) | ai-gov | 1.0.0 | approved | standards/ai-gov/ATC-AI-GOV-POLICY-001.md |
| ATC-ARCH-001 | KAI-OS System Architecture Standard | architecture | 1.0.0 | approved | standards/architecture/ATC-ARCH-001.md |
| ATC-AURORA-SWARM-001 | Aurora Swarm Network (ASN) — Neuronales KI-Schwarmnetzwerk: Neuron-Modell, Synapse-Protokoll, Schwarm-Kognition, Evidence-getriebene Verifikation und Authority-Integration | aurora-swarm | 0.1.0 | candidate | standards/aurora-swarm/ATC-AURORA-SWARM-001.md |
| ATC-ENT-001 | Enterprise Governance Standard (P0) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-001.md |
| ATC-ENT-002 | Rollen & Verantwortlichkeiten Standard (P0) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-002.md |
| ATC-ENT-003 | Entscheidungsmanagement Standard (P0) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-003.md |
| ATC-ENT-004 | Delegation & Berechtigungen Standard (P0) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-004.md |
| ATC-ENT-005 | Unternehmensrichtlinien Standard (P1) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-005.md |
| ATC-ENT-006 | Interessenkonflikte Standard (P2) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-006.md |
| ATC-ENT-007 | Eskalationsmanagement Standard (P1) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-007.md |
| ATC-ENT-008 | Organisationsstruktur Standard (P1) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-008.md |
| ATC-ENT-009 | Repository Governance Standard (P1) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-009.md |
| ATC-ENT-010 | Enterprise Change Management Standard (P0) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-010.md |
| ATC-ENT-011 | Risiko-Management Standard (P0) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-011.md |
| ATC-ENT-012 | Wissensmanagement & Consistency-Gate Standard (P1) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-012.md |
| ATC-ENT-013 | KPI & Performance Standard (P2) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-013.md |
| ATC-ENT-014 | Audit & Nachvollziehbarkeit Standard (P1) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-014.md |
| ATC-ENT-015 | Qualitätsmanagement & Definition of Done Standard (P2) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-ENT-015.md |
| ATC-GATE-SEC-001 | Continuous Security & Technology Assurance Gate (14 Gate-Zeilen, RELEASE-READY-Regel, 5-Stufen-Security-Status, Evidence Store, Org-Audit-Zielarchitektur) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-GATE-SEC-001.md |
| ATC-GOV-001 | A-TownChain Governance Constitution (Level-0-Verfassung: 5-Ebenen-Hierarchie, 8 Authorities ohne God Mode, Decision Rights Matrix, Quoren 2/3-of-5/4-of-7 mit Timelock, Emergency/Exception/COI/Appeals/Succession, AI-Agent-Pflichtkette, 8 Governance Gates, GitHub-Enforcement, G0-G4-Profile, Governance Score mit harten Deckeln, Amendment-Prozess) | governance | 1.0.0 | approved | governance/constitution/ATC-GOV-001.md |
| ATC-ORG-BASELINE-001 | ATC Organization Engineering Baseline (5 Schutzschichten, Repo-Tiers T0-T4 mit Ableitungsregel, Merge-Gate je Tier, P0-P3-Roadmap mit ehrlichem IST-Stand) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-ORG-BASELINE-001.md |
| ATC-STD-000 | Standards Governance & Specification Standard (Verfassung; v1.3.0: Domaenen-Familien-Verweis §7.7) | governance | 1.3.0 | approved | governance/ATC-STD-000.md |
| ATC-STD-001 | Enterprise Governance Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-001.md |
| ATC-STD-002 | Standards Family ID & Artifact Identifier Architecture (10 Domaenen-Familien 000-900 dauerhaft reserviert, No-Recycling, Artefakt-Klassen REQ/TC/GATE/ADR/CTRL/SPEC/SCHEMA, Traceability-Kette Familie->Standard->REQ->Impl->Test->Gate->Evidence) | governance | 1.0.0 | approved | governance/ATC-STD-002.md |
| ATC-STD-003 | Governance Determinism & Source-of-Truth Matrix Standard (SSOT-Matrix je Informationsklasse, State-ID/Ein-Zahl-Regel mit Registry-SHA, README-Vollregeneration/CURRENT-STATE-only, .github-Referenzpflicht, Autoritaetskaskade Registry>Evidence>Doku, ATC-STD-000-Bootstrap mit Sunset, Slot-Prinzip, APPROVED!=IMPLEMENTED, Agent-Autoritaetsgrenzen 2-of-N, State-Snapshot-Pflicht) | governance | 1.0.0 | approved | standards/governance/ATC-STD-003.md |
| ATC-STD-006 | Policy Management Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-006.md |
| ATC-STD-008 | Compliance Management Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-008.md |
| ATC-STD-012 | Incident Governance Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-012.md |
| ATC-STD-013 | Escalation Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-013.md |
| ATC-STD-015 | Records Management Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-015.md |
| ATC-STD-016 | Repository Artifact & File Inventory Standard (generiertes Inventar-SSOT, Manifest-Schema, Generator-Pflicht, Drift-Check) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-016.md |
| ATC-STD-017 | Obsolete & Orphaned Artifact Management Standard (9 Klassen, Entscheidungslogik, Mindestzuordnung, duale Verwaist-Bestimmung, 6-Kriterien-Loeschschutz, CI-Gate 2 Stufen, Audit-Signal-Nicht-Loeschkriterium) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-017.md |
| ATC-STD-018 | Technology Currency, Vulnerability & Security Assurance Standard (Technologie-Inventar, EOL-Verbot+ADR-Ausnahme, CVE/GHSA/OSV-SLAs, Attack-Klassen inkl. Blockchain, Security Baseline, CI-Security-Gate mit Merge-Block, Technology Review, TCS-Score, No-Evidence-No-Claim, Lifecycle, Statusfelder, Org-Assurance-Gate) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-018.md |
| ATC-STD-019 | Dependency & Supply Chain Security Standard (Lockfiles, SBOM, Pinning, Signing/Provenance, Reproducible Builds, Registry-Sicherheit, Build-Sicherheit, Supply-Chain-Gate) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-019.md |
| ATC-STD-020 | Security Incident & Vulnerability Response Standard (Erkennung->Eskalation->Containment->Patch->Verifikation->Disclosure->PIR, SEV-1..4 mit SLA, Human-Gate SEV-1/2+Disclosure, ATC-VULN-Record-Schema v1.1.0) | enterprise | 1.1.0 | approved | standards/enterprise/ATC-STD-020.md |
| ATC-STD-021 | Continuous Security Monitoring Standard (11-Umfeld, 7-Zustandsmodell MONITORED..VERIFIED, Kontinuitaets- und Eskalationspflicht) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-021.md |
| ATC-STD-022 | Security Patch Management Standard (SLA Critical<=24h/High<=72h/Medium<=14d/Low<=30d, Active-Exploit-Notfallpfad, Patch-Verifikationspflicht) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-022.md |
| ATC-STD-023 | Technology Lifecycle Management Standard (8-Status-Zyklus PROPOSED..REMOVED, 9-Kriterien-Adoption-Gate, 5 Entscheidungen, Registry-Kopplung) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-023.md |
| ATC-STD-024 | Known Bug Management Standard (9-Status-Bug-Lifecycle, Bug-Record-Schema, Root-Cause-Pflicht bis Preventive Control, ERR-Kopplung) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-024.md |
| ATC-STD-025 | Standards Review Standard | governance | 1.2.0 | approved | standards/governance/ATC-STD-025.md |
| ATC-STD-026 | Security Regression Prevention Standard (Fix-zu-Test-Pflicht: jede behobene Vulnerability wird permanent Testfall, Qualitaets-/Nachweispflicht) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-026.md |
| ATC-STD-027 | Standards Deprecation Standard | governance | 1.2.0 | approved | standards/governance/ATC-STD-027.md |
| ATC-STD-028 | Attack Surface Management Standard (Expositionsvektoren-Inventarpflicht, 5 Vektor-Zustaende EXPOSED..JUSTIFIED, Unknown Attack Surface = Finding) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-028.md |
| ATC-STD-029 | Threat Modeling Standard (8-Glied-Analysekette Asset..Residual Risk, Pflicht fuer 18 kritische Komponenten, Evidence-Pflicht) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-029.md |
| ATC-STD-030 | Automated Repository Health & Compliance Standard (10-Scanner-Dimensionen, P0-P3-Finding-Klassen mit 10 Kategorien und ID-Format, Continuous-Assurance-Prinzip, Feedback-Loop, Standards-Evolution-Engine) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-030.md |
| ATC-STD-031 | Repository Health Score Standard (9 Dimensionen, 5 Bänder EXCELLENT..CRITICAL, Gate-Vorrang: Score ueberstimmt Security-Gates nie) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-031.md |
| ATC-STD-032 | Automated Update Management Standard (5-Kategorie-Release-Klassifizierung, 5-Stufen-Update-Pipeline, automatisierte Update-PRs, Major nur mit Kompatibilitätsprüfung) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-032.md |
| ATC-STD-033 | Vulnerability Intelligence Standard (10 Quellklassen CVE..Bug-Bounty, 5-Stufen-Betroffenheitskette Technology->Repo->Version->Deployment->Risk, VULN-Record-Pflicht) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-033.md |
| ATC-STD-034 | Exploitability Assessment Standard (7 Bewertungsfaktoren Existence..Network-Reachability, kein Abwertungs-Fehlschluss, Re-Assessment-Pflicht) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-034.md |
| ATC-STD-035 | Zero-Day Response Standard (9-Schritt-Kette Confirm..Recover, 8 Containment-Maßnahmen, erhöhte Monitoring-Pflicht bis Patch) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-035.md |
| ATC-STD-036 | Emergency Security Change Standard (7-Schritt-Notfallpfad mit Security Authorization, Minimal-Safe-Change, Ex-Post-Review, volle Rückverfolgbarkeit) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-036.md |
| ATC-STD-037 | Security Configuration Baseline Standard (15 Konfigurationsbereiche OS..Monitoring, maschinenlesbare Baseline-Datei, Scanner-Anbindung) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-037.md |
| ATC-STD-038 | Secrets & Credential Security Standard (Source-Control-Verbot P0, Leak-Kette Detection->Revocation->Rotation->Verification, Löschen genügt nie) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-038.md |
| ATC-STD-039 | Cryptographic Agility Standard (austauschbare Crypto-Interfaces Current/Alternative/Future, 4 Wechselkriterien inkl. Quantum Threat, Eigenbau-Krypto-Verbot) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-039.md |
| ATC-STD-040 | Repository Architecture Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-040.md |
| ATC-STD-041 | Reproducible & Verifiable Builds Standard (Build-Kette Pinned Deps->Reproducible Build->Hash->Signature->SBOM, Determinismus, Hash-Verifikation) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-041.md |
| ATC-STD-042 | Repository Ownership Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-042.md |
| ATC-STD-043 | Artifact Integrity Standard (9-Feld-Artefakt-Record id/hash/build/sbom/provenance/signature, Provenance-Kette zu Reproducible Builds, Ablage evidence/releases) | enterprise | 1.0.0 | approved | standards/enterprise/ATC-STD-043.md |
| ATC-STD-045 | Repository Metadata Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-045.md |
| ATC-STD-048 | Repository Branching Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-048.md |
| ATC-STD-050 | Repository Archiving Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-050.md |
| ATC-STD-051 | Repository Deprecation Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-051.md |
| ATC-STD-052 | Repository Health Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-052.md |
| ATC-STD-062 | Wiki Standard | md | 1.2.0 | approved | standards/md/ATC-STD-062.md |
| ATC-STD-063 | API Documentation Standard | md | 1.2.0 | approved | standards/md/ATC-STD-063.md |
| ATC-STD-064 | Architecture Documentation Standard | md | 1.2.0 | approved | standards/md/ATC-STD-064.md |
| ATC-STD-066 | Whitepaper Standard | md | 1.2.0 | approved | standards/md/ATC-STD-066.md |
| ATC-STD-067 | Technical Paper Standard | md | 1.2.0 | approved | standards/md/ATC-STD-067.md |
| ATC-STD-069 | Release Notes Standard | md | 1.2.0 | approved | standards/md/ATC-STD-069.md |
| ATC-STD-070 | TODO Standard | md | 1.2.0 | approved | standards/md/ATC-STD-070.md |
| ATC-STD-072 | Sprint Documentation Standard | md | 1.2.0 | approved | standards/md/ATC-STD-072.md |
| ATC-STD-081 | Coding Standard | development | 1.2.0 | approved | standards/development/ATC-STD-081.md |
| ATC-STD-082 | Code Style Standard | development | 1.2.0 | approved | standards/development/ATC-STD-082.md |
| ATC-STD-085 | Modularisation Standard | development | 1.2.0 | approved | standards/development/ATC-STD-085.md |
| ATC-STD-087 | Error Handling Standard | development | 1.2.0 | approved | standards/development/ATC-STD-087.md |
| ATC-STD-088 | Logging Standard | development | 1.2.0 | approved | standards/development/ATC-STD-088.md |
| ATC-STD-089 | Configuration Management Standard | development | 1.2.0 | approved | standards/development/ATC-STD-089.md |
| ATC-STD-090 | Environment Management Standard | development | 1.2.0 | approved | standards/development/ATC-STD-090.md |
| ATC-STD-091 | Testing Standard | development | 1.2.0 | approved | standards/development/ATC-STD-091.md |
| ATC-STD-093 | Refactoring Standard | development | 1.2.0 | approved | standards/development/ATC-STD-093.md |
| ATC-STD-094 | Technical Debt Standard | development | 1.2.0 | approved | standards/development/ATC-STD-094.md |
| ATC-STD-095 | Build Standard | development | 1.2.0 | approved | standards/development/ATC-STD-095.md |
| ATC-STD-100 | Language & Technology Stack Standard | architecture | 2.0.0 | approved | standards/architecture/ATC-STD-100.md |
| ATC-STD-101 | Branch Naming Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-101.md |
| ATC-STD-103 | Commit Message Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-103.md |
| ATC-STD-104 | Pull Request Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-104.md |
| ATC-STD-105 | Merge Policy Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-105.md |
| ATC-STD-107 | Tagging Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-107.md |
| ATC-STD-114 | Git Standard | repository | 1.0.0 | approved | standards/repository/ATC-STD-114.md |
| ATC-STD-129 | Incident Correlation Standard | bug | 1.2.0 | approved | standards/bug/ATC-STD-129.md |
| ATC-STD-130 | Post-Incident Review Standard | bug | 1.2.0 | approved | standards/bug/ATC-STD-130.md |
| ATC-STD-140 | QA Framework Standard | development | 1.2.0 | approved | standards/development/ATC-STD-140.md |
| ATC-STD-141 | Unit Testing Standard | development | 1.2.0 | approved | standards/development/ATC-STD-141.md |
| ATC-STD-142 | Integration Testing Standard | development | 1.2.0 | approved | standards/development/ATC-STD-142.md |
| ATC-STD-143 | System Testing Standard | development | 1.2.0 | approved | standards/development/ATC-STD-143.md |
| ATC-STD-144 | End-to-End Testing Standard | development | 1.2.0 | approved | standards/development/ATC-STD-144.md |
| ATC-STD-146 | Performance Testing Standard | development | 1.2.0 | approved | standards/development/ATC-STD-146.md |
| ATC-STD-147 | Load Testing Standard | development | 1.2.0 | approved | standards/development/ATC-STD-147.md |
| ATC-STD-148 | Stress Testing Standard | development | 1.2.0 | approved | standards/development/ATC-STD-148.md |
| ATC-STD-149 | Security Testing Standard | development | 1.2.0 | approved | standards/development/ATC-STD-149.md |
| ATC-STD-160 | CI/CD Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-160.md |
| ATC-STD-161 | Automated Build Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-161.md |
| ATC-STD-163 | Artifact Management Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-163.md |
| ATC-STD-164 | Deployment Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-164.md |
| ATC-STD-169 | Infrastructure as Code Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-169.md |
| ATC-STD-170 | Container Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-170.md |
| ATC-STD-171 | Kubernetes Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-171.md |
| ATC-STD-172 | Terraform Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-172.md |
| ATC-STD-174 | Disaster Recovery Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-174.md |
| ATC-STD-180 | Blockchain Architecture Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-180.md |
| ATC-STD-181 | Node Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-181.md |
| ATC-STD-182 | Consensus Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-182.md |
| ATC-STD-183 | PoW Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-183.md |
| ATC-STD-184 | PoS Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-184.md |
| ATC-STD-185 | PoH Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-185.md |
| ATC-STD-186 | Validator Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-186.md |
| ATC-STD-187 | Staking Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-187.md |
| ATC-STD-188 | Delegation Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-188.md |
| ATC-STD-189 | Slashing Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-189.md |
| ATC-STD-190 | Block Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-190.md |
| ATC-STD-191 | Transaction Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-191.md |
| ATC-STD-192 | Mempool Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-192.md |
| ATC-STD-193 | Fee Market Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-193.md |
| ATC-STD-194 | MEV Mitigation Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-194.md |
| ATC-STD-195 | Fork Handling Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-195.md |
| ATC-STD-196 | Chain Recovery Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-196.md |
| ATC-STD-197 | Genesis Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-197.md |
| ATC-STD-200 | ATC Token Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-200.md |
| ATC-STD-201 | Repository Structure Standard | repository | 1.0.1 | approved | standards/repository/ATC-STD-201.md |
| ATC-STD-202 | Repository Naming & Classification Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-202.md |
| ATC-STD-203 | Repository Security & Release Standard | repository | 1.0.1 | approved | standards/repository/ATC-STD-203.md |
| ATC-STD-204 | Dependency & Interface Standard | repository | 1.0.0 | approved | standards/repository/ATC-STD-204.md |
| ATC-STD-205 | Minting Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-205.md |
| ATC-STD-206 | Staking Rewards Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-206.md |
| ATC-STD-207 | Treasury Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-207.md |
| ATC-STD-208 | Governance Token Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-208.md |
| ATC-STD-209 | Token Allocation Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-209.md |
| ATC-STD-210 | Vesting Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-210.md |
| ATC-STD-212 | Token Supply Standard | blockchain | 1.0.0 | approved | standards/blockchain/ATC-STD-212.md |
| ATC-STD-213 | Emission Standard | blockchain | 1.0.0 | approved | standards/blockchain/ATC-STD-213.md |
| ATC-STD-214 | Burning Standard | blockchain | 1.0.0 | approved | standards/blockchain/ATC-STD-214.md |
| ATC-STD-215 | Tokenomics Standard | blockchain | 1.0.0 | approved | standards/blockchain/ATC-STD-215.md |
| ATC-STD-220 | Repository File Admission & Placement Standard (Master Subfamilie 220-229: Entscheidungsmodell 10 Spalten, Admission-Status ADMITTED/CONDITIONAL/QUARANTINED/REJECTED, Taxonomie 20 Klassen, File Entitlements A/B/T/D/G/S/C/I/P/O, autoritative Quelle, Generated-Artifact-Policy, CHECK-FILE-001 Checker, Ausnahmeprozess mit Expiry; Grundsatz Presence is not permission) | repository | 1.0.0 | approved | standards/repository/ATC-STD-220.md |
| ATC-STD-240 | Interoperability Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-240.md |
| ATC-STD-241 | Bridge Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-241.md |
| ATC-STD-242 | Bridge Security Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-242.md |
| ATC-STD-243 | Cross-Chain Messaging Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-243.md |
| ATC-STD-244 | IBC Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-244.md |
| ATC-STD-245 | EVM Compatibility Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-245.md |
| ATC-STD-246 | Ethereum Integration Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-246.md |
| ATC-STD-247 | Solana Integration Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-247.md |
| ATC-STD-248 | Polygon Integration Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-248.md |
| ATC-STD-249 | BSC Integration Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-249.md |
| ATC-STD-270 | Oracle Architecture Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-270.md |
| ATC-STD-271 | External Data Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-271.md |
| ATC-STD-272 | Data Verification Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-272.md |
| ATC-STD-273 | Oracle Consensus Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-273.md |
| ATC-STD-274 | Price Feeds Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-274.md |
| ATC-STD-275 | Oracle Failure Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-275.md |
| ATC-STD-276 | External API Binding Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-276.md |
| ATC-STD-280 | Identity Standard | security | 1.2.0 | approved | standards/security/ATC-STD-280.md |
| ATC-STD-282 | Wallet Identity Standard | security | 1.2.0 | approved | standards/security/ATC-STD-282.md |
| ATC-STD-283 | Reputation Standard | security | 1.2.0 | approved | standards/security/ATC-STD-283.md |
| ATC-STD-284 | Sybil Resistance Standard | security | 1.2.0 | approved | standards/security/ATC-STD-284.md |
| ATC-STD-285 | Identity Recovery Standard | security | 1.2.0 | approved | standards/security/ATC-STD-285.md |
| ATC-STD-286 | Credential Verification Standard | security | 1.2.0 | approved | standards/security/ATC-STD-286.md |
| ATC-STD-300 | Development & Project Management Standard | development | 1.0.0 | approved | standards/development/ATC-STD-300.md |
| ATC-STD-301 | Secure Development Standard | security | 1.2.0 | approved | standards/security/ATC-STD-301.md |
| ATC-STD-302 | Authentication Standard | security | 1.2.0 | approved | standards/security/ATC-STD-302.md |
| ATC-STD-303 | Authorisation Standard | security | 1.2.0 | approved | standards/security/ATC-STD-303.md |
| ATC-STD-304 | Key Management Standard | security | 1.2.0 | approved | standards/security/ATC-STD-304.md |
| ATC-STD-306 | Encryption Standard | security | 1.2.0 | approved | standards/security/ATC-STD-306.md |
| ATC-STD-307 | Network Security Standard | security | 1.2.0 | approved | standards/security/ATC-STD-307.md |
| ATC-STD-308 | Endpoint Security Standard | security | 1.2.0 | approved | standards/security/ATC-STD-308.md |
| ATC-STD-309 | Supply Chain Security Standard | security | 1.2.0 | approved | standards/security/ATC-STD-309.md |
| ATC-STD-311 | Vulnerability Management Standard | security | 1.2.0 | approved | standards/security/ATC-STD-311.md |
| ATC-STD-312 | Penetration Testing Standard | security | 1.2.0 | approved | standards/security/ATC-STD-312.md |
| ATC-STD-314 | Cybersecurity Framework Standard | security | 1.0.0 | approved | standards/security/ATC-STD-314.md |
| ATC-STD-340 | Mining Architecture Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-340.md |
| ATC-STD-341 | Miner Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-341.md |
| ATC-STD-342 | Mining Manager Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-342.md |
| ATC-STD-343 | CPU Mining Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-343.md |
| ATC-STD-344 | GPU Mining Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-344.md |
| ATC-STD-345 | Mobile Mining Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-345.md |
| ATC-STD-346 | Mining Algorithms Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-346.md |
| ATC-STD-347 | Miner Plugins Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-347.md |
| ATC-STD-348 | Mining Rewards Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-348.md |
| ATC-STD-349 | Mining Security Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-349.md |
| ATC-STD-350 | Miner Monitoring Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-350.md |
| ATC-STD-360 | Wallet Architecture Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-360.md |
| ATC-STD-361 | Address Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-361.md |
| ATC-STD-362 | Key Management Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-362.md |
| ATC-STD-363 | Transaction Signing Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-363.md |
| ATC-STD-364 | Wallet Recovery Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-364.md |
| ATC-STD-365 | Hardware Wallet Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-365.md |
| ATC-STD-366 | Wallet Security Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-366.md |
| ATC-STD-367 | Wallet UI Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-367.md |
| ATC-STD-368 | Burning Wallet Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-368.md |
| ATC-STD-380 | DeFi Architecture Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-380.md |
| ATC-STD-381 | DEX Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-381.md |
| ATC-STD-382 | Liquidity Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-382.md |
| ATC-STD-383 | Liquidity Pools Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-383.md |
| ATC-STD-384 | Staking Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-384.md |
| ATC-STD-385 | Lending Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-385.md |
| ATC-STD-386 | Yield Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-386.md |
| ATC-STD-387 | Oracle Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-387.md |
| ATC-STD-388 | DeFi Risk Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-388.md |
| ATC-STD-389 | DeFi Security Standard | blockchain | 1.2.0 | approved | standards/blockchain/ATC-STD-389.md |
| ATC-STD-400 | NFT Architecture Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-400.md |
| ATC-STD-401 | NFT Metadata Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-401.md |
| ATC-STD-402 | NFT Minting Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-402.md |
| ATC-STD-403 | NFT Ownership Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-403.md |
| ATC-STD-404 | NFT Transfer Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-404.md |
| ATC-STD-405 | NFT Marketplace Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-405.md |
| ATC-STD-406 | NFT Royalties Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-406.md |
| ATC-STD-407 | NFT Storage Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-407.md |
| ATC-STD-408 | NFT Verification Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-408.md |
| ATC-STD-409 | NFT Security Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-409.md |
| ATC-STD-420 | GameFi Architecture Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-420.md |
| ATC-STD-421 | Shivamon Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-421.md |
| ATC-STD-422 | Game Economy Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-422.md |
| ATC-STD-423 | PvE Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-423.md |
| ATC-STD-424 | PvP Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-424.md |
| ATC-STD-425 | Guild Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-425.md |
| ATC-STD-426 | Alliance Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-426.md |
| ATC-STD-427 | Territory Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-427.md |
| ATC-STD-428 | Crafting Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-428.md |
| ATC-STD-429 | Mods Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-429.md |
| ATC-STD-430 | DNA Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-430.md |
| ATC-STD-431 | Evolution Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-431.md |
| ATC-STD-432 | Genesis Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-432.md |
| ATC-STD-433 | Game Assets Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-433.md |
| ATC-STD-434 | Game Blockchain Integration Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-434.md |
| ATC-STD-440 | API Architecture Standard | development | 1.2.0 | approved | standards/development/ATC-STD-440.md |
| ATC-STD-441 | REST API Standard | development | 1.2.0 | approved | standards/development/ATC-STD-441.md |
| ATC-STD-442 | GraphQL Standard | development | 1.2.0 | approved | standards/development/ATC-STD-442.md |
| ATC-STD-443 | RPC Standard | development | 1.2.0 | approved | standards/development/ATC-STD-443.md |
| ATC-STD-444 | WebSocket Standard | development | 1.2.0 | approved | standards/development/ATC-STD-444.md |
| ATC-STD-445 | Authentication Standard | development | 1.2.0 | approved | standards/development/ATC-STD-445.md |
| ATC-STD-447 | Rate Limiting Standard | development | 1.2.0 | approved | standards/development/ATC-STD-447.md |
| ATC-STD-448 | API Errors Standard | development | 1.2.0 | approved | standards/development/ATC-STD-448.md |
| ATC-STD-449 | API Security Standard | development | 1.2.0 | approved | standards/development/ATC-STD-449.md |
| ATC-STD-450 | API Documentation Standard | development | 1.2.0 | approved | standards/development/ATC-STD-450.md |
| ATC-STD-460 | Data Architecture Standard | development | 1.2.0 | approved | standards/development/ATC-STD-460.md |
| ATC-STD-461 | Data Model Standard | development | 1.2.0 | approved | standards/development/ATC-STD-461.md |
| ATC-STD-462 | Data Validation Standard | development | 1.2.0 | approved | standards/development/ATC-STD-462.md |
| ATC-STD-463 | Data Integrity Standard | development | 1.2.0 | approved | standards/development/ATC-STD-463.md |
| ATC-STD-466 | Database Standard | development | 1.2.0 | approved | standards/development/ATC-STD-466.md |
| ATC-STD-467 | Firestore Standard | development | 1.2.0 | approved | standards/development/ATC-STD-467.md |
| ATC-STD-468 | Backup Standard | development | 1.2.0 | approved | standards/development/ATC-STD-468.md |
| ATC-STD-469 | Data Recovery Standard | development | 1.2.0 | approved | standards/development/ATC-STD-469.md |
| ATC-STD-480 | Observability Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-480.md |
| ATC-STD-481 | Logging Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-481.md |
| ATC-STD-482 | Metrics Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-482.md |
| ATC-STD-483 | Tracing Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-483.md |
| ATC-STD-484 | Monitoring Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-484.md |
| ATC-STD-485 | Alerting Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-485.md |
| ATC-STD-486 | Health Checks Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-486.md |
| ATC-STD-488 | Performance Monitoring Standard | infrastructure | 1.2.0 | approved | standards/infrastructure/ATC-STD-488.md |
| ATC-STD-500 | Incident Management Standard | security | 1.2.0 | approved | standards/security/ATC-STD-500.md |
| ATC-STD-501 | Incident Classification Standard | security | 1.2.0 | approved | standards/security/ATC-STD-501.md |
| ATC-STD-502 | Incident Response Standard | security | 1.2.0 | approved | standards/security/ATC-STD-502.md |
| ATC-STD-504 | Disaster Recovery Standard | security | 1.2.0 | approved | standards/security/ATC-STD-504.md |
| ATC-STD-505 | Fault Recovery Standard | security | 1.2.0 | approved | standards/security/ATC-STD-505.md |
| ATC-STD-507 | Failover Standard | security | 1.2.0 | approved | standards/security/ATC-STD-507.md |
| ATC-STD-508 | Backup Recovery Standard | security | 1.2.0 | approved | standards/security/ATC-STD-508.md |
| ATC-STD-509 | Business/System Continuity Standard | security | 1.2.0 | approved | standards/security/ATC-STD-509.md |
| ATC-STD-540 | Project Management Standard | development | 1.2.0 | approved | standards/development/ATC-STD-540.md |
| ATC-STD-541 | Epic Standard | development | 1.2.0 | approved | standards/development/ATC-STD-541.md |
| ATC-STD-542 | Feature Standard | development | 1.2.0 | approved | standards/development/ATC-STD-542.md |
| ATC-STD-543 | Requirement Standard | development | 1.2.0 | approved | standards/development/ATC-STD-543.md |
| ATC-STD-544 | Task Standard | development | 1.2.0 | approved | standards/development/ATC-STD-544.md |
| ATC-STD-545 | TODO Standard | development | 1.2.0 | approved | standards/development/ATC-STD-545.md |
| ATC-STD-546 | Sprint Standard | development | 1.2.0 | approved | standards/development/ATC-STD-546.md |
| ATC-STD-560 | Requirements Standard | development | 1.2.0 | approved | standards/development/ATC-STD-560.md |
| ATC-STD-561 | Functional Requirements Standard | development | 1.2.0 | approved | standards/development/ATC-STD-561.md |
| ATC-STD-562 | Non-Functional Requirements Standard | development | 1.2.0 | approved | standards/development/ATC-STD-562.md |
| ATC-STD-566 | Requirement Validation Standard | development | 1.2.0 | approved | standards/development/ATC-STD-566.md |
| ATC-STD-580 | UI/UX Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-580.md |
| ATC-STD-581 | Design System Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-581.md |
| ATC-STD-582 | Component Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-582.md |
| ATC-STD-583 | Accessibility Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-583.md |
| ATC-STD-584 | Responsive Design Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-584.md |
| ATC-STD-585 | UX Consistency Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-585.md |
| ATC-STD-586 | Error UI Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-586.md |
| ATC-STD-587 | Admin UI Standard | applications | 1.2.0 | approved | standards/applications/ATC-STD-587.md |
| ATC-STD-600 | Chain Identity & Network Identification | blockchain | 1.3.0 | approved | standards/os/ATC-STD-600.md |
| ATC-STD-601 | Mobile Standard | os | 1.2.0 | approved | standards/os/ATC-STD-601.md |
| ATC-STD-602 | Desktop Standard | os | 1.2.0 | approved | standards/os/ATC-STD-602.md |
| ATC-STD-603 | Globus OS Standard | os | 1.2.0 | approved | standards/os/ATC-STD-603.md |
| ATC-STD-604 | Shiva OS Standard | os | 1.2.0 | approved | standards/os/ATC-STD-604.md |
| ATC-STD-605 | Aurora OS Standard | os | 1.2.0 | approved | standards/os/ATC-STD-605.md |
| ATC-STD-606 | Device Integration Standard | os | 1.2.0 | approved | standards/os/ATC-STD-606.md |
| ATC-STD-607 | Offline Mode Standard | os | 1.2.0 | approved | standards/os/ATC-STD-607.md |
| ATC-STD-620 | ATCLang Architecture Standard | development | 1.2.0 | approved | standards/development/ATC-STD-620.md |
| ATC-STD-623 | Compiler Standard | development | 1.2.0 | approved | standards/development/ATC-STD-623.md |
| ATC-STD-624 | VM Standard | development | 1.2.0 | approved | standards/development/ATC-STD-624.md |
| ATC-STD-625 | REPL Standard | development | 1.2.0 | approved | standards/development/ATC-STD-625.md |
| ATC-STD-626 | Package System Standard | development | 1.2.0 | approved | standards/development/ATC-STD-626.md |
| ATC-STD-627 | Standard Library Standard | development | 1.2.0 | approved | standards/development/ATC-STD-627.md |
| ATC-STD-640 | AuditTrail Standard | audit | 1.2.0 | approved | standards/audit/ATC-STD-640.md |
| ATC-STD-641 | LogChain Standard | audit | 1.2.0 | approved | standards/audit/ATC-STD-641.md |
| ATC-STD-643 | Event Recording Standard | audit | 1.2.0 | approved | standards/audit/ATC-STD-643.md |
| ATC-STD-646 | Audit Retention Standard | audit | 1.2.0 | approved | standards/audit/ATC-STD-646.md |
| ATC-STD-660 | Software Supply Chain Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-660.md |
| ATC-STD-662 | Dependency Pinning Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-662.md |
| ATC-STD-664 | SBOM Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-664.md |
| ATC-STD-665 | Third-Party Software Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-665.md |
| ATC-STD-666 | License Compliance Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-666.md |
| ATC-STD-667 | Artifact Integrity Standard | repository | 1.2.0 | approved | standards/repository/ATC-STD-667.md |
| ATC-STD-680 | Open Source Standard | governance | 1.2.0 | approved | standards/governance/ATC-STD-680.md |
| ATC-STD-681 | License Standard | governance | 1.2.0 | approved | standards/governance/ATC-STD-681.md |
| ATC-STD-682 | Contributor Standard | governance | 1.2.0 | approved | standards/governance/ATC-STD-682.md |
| ATC-STD-683 | CLA Standard | governance | 1.2.0 | approved | standards/governance/ATC-STD-683.md |
| ATC-STD-684 | Code of Conduct Standard | governance | 1.2.0 | approved | standards/governance/ATC-STD-684.md |
| ATC-STD-685 | Security Disclosure Standard | governance | 1.2.0 | approved | standards/governance/ATC-STD-685.md |
| ATC-STD-686 | Vulnerability Disclosure Standard | governance | 1.2.0 | approved | standards/governance/ATC-STD-686.md |
| ATC-STD-700 | Business Model Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-700.md |
| ATC-STD-701 | Treasury Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-701.md |
| ATC-STD-702 | Revenue Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-702.md |
| ATC-STD-703 | Cost Management Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-703.md |
| ATC-STD-704 | Incentive Alignment Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-704.md |
| ATC-STD-705 | Economic Security Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-705.md |
| ATC-STD-706 | Sustainability Standard | enterprise | 1.2.0 | approved | standards/enterprise/ATC-STD-706.md |
| ATC-STD-999 | ATC Master-Audit — Enterprise Completeness & Consistency Audit: 16-Stufen-System-Audit-Kette, 13 Change-Nachweis-Fragen, Register-Abdeckung (11 Register), MAUD-Records, Orchestrierung von S-01..S-22/REPO-AUDIT/RR-G01..G08 | master-audit | 1.0.0 | approved | standards/master-audit/ATC-STD-999.md |
| ATC-STD-AI-DECISION-001 | ATC Agent Decision-Making Standard — Entscheidungsmodell für KI-Agenten: Pipeline, D0-D5, L0-L5, Evidence-First, RK-Risiko, Decision Records DEC-NNNNNN, Separation of Duties | ai-decision | 1.0.0 | approved | standards/ai-decision/ATC-STD-AI-DECISION-001.md |
| ATC-STD-AI-DEV-001 | Software Development AI Agent Identity & Workflow Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-001.md |
| ATC-STD-AI-DEV-002 | Agent Capabilities & Permissions Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-002.md |
| ATC-STD-AI-DEV-003 | Repository Discovery Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-003.md |
| ATC-STD-AI-DEV-004 | AI Task Management Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-004.md |
| ATC-STD-AI-DEV-005 | Finding & Evidence Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-005.md |
| ATC-STD-AI-DEV-006 | AI Decision & Action Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-006.md |
| ATC-STD-AI-DEV-007 | AI Git Commit & Pull Request Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-007.md |
| ATC-STD-AI-DEV-008 | AI Testing & Validation Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-008.md |
| ATC-STD-AI-DEV-009 | AI Audit Trail Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-009.md |
| ATC-STD-AI-DEV-010 | AI Documentation Synchronization Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-010.md |
| ATC-STD-AI-DEV-011 | Human Approval & Escalation Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-011.md |
| ATC-STD-AI-DEV-012 | Multi-Agent Coordination Standard | ai-dev | 1.1.0 | approved | standards/ai/ATC-STD-AI-DEV-012.md |
| ATC-STD-AOS-001 | ATC Agent Operating Standard — Verbindliches Session-Mandat für alle KI-Agenten: 14 Session-Fragen, Session-Lifecycle, maschinenlesbarer Session-Record, Audit-Nachweis | agent-operating | 1.0.0 | approved | standards/agent-operating/ATC-STD-AOS-001.md |
| ATC-STD-AUDIT-001 | ATC Completeness & Audit Standard — Vollständigkeitsprüfung & Audit: Kontrollschicht über allen Standards (20 Domänen, Traceability, Cross-System Integrity, Release-Gates) | audit | 1.0.0 | approved | standards/audit/ATC-STD-AUDIT-001.md |
| ATC-STD-BUG-001 | Bug Finding Standard | bug | 1.0.0 | approved | standards/bug/ATC-STD-BUG-001.md |
| ATC-STD-BUG-002 | Bug Documentation Standard | bug | 1.1.0 | approved | standards/bug/ATC-STD-BUG-002.md |
| ATC-STD-BUG-003 | Bug Fix Lifecycle Standard | bug | 1.0.0 | approved | standards/bug/ATC-STD-BUG-003.md |
| ATC-STD-BUG-004 | Repository Synchronization & Merge Gate | bug | 1.0.0 | approved | standards/bug/ATC-STD-BUG-004.md |
| ATC-STD-BUG-005 | Fehleranalyse- und Root-Cause-Analysis-Standard — Analyse- und QMS-Schicht der Bug-Familie: 4-Ebenen-RCA, Fault Tree, Evidence, Timeline, Impact, Metrics, Closure Gate, Corrective/Preventive | bug | 1.0.0 | approved | standards/bug/ATC-STD-BUG-005.md |
| ATC-STD-CHANGE-001 | ATC Change Control Dachnorm — Eine Änderung, ein Kanal, eine Gate-Landkarte: konsolidierte Zuordnung von ATC-STD-000 §19–33 (SCR), VERSION-001, UPDATE-001 und COMPAT-001 zur verbindlichen Entscheidungsmatrix mit RACI, Notfallpfad und den 13 Change-Nachweis-Fragen als Prüfraster | governance-core | 1.1.0 | approved | standards/governance-core/ATC-STD-CHANGE-001.md |
| ATC-STD-CI-001 | Reproducible CI Dependencies — Deklaration, Installation, Reproduzierbarkeit, Fresh-Runner-Faehigkeit und Fehlerklassifikation von CI-Dependencies | cicd | 1.0.0 | approved | standards/cicd/ATC-STD-CI-001.md |
| ATC-STD-COMPAT-001 | ATC Major Version Compatibility & Recovery Standard — Verbindliche Kompatibilitätsprüfung, -Wiederherstellung und -Migration nach MAJOR-Updates | compat | 1.0.0 | approved | standards/compat/ATC-STD-COMPAT-001.md |
| ATC-STD-DESC-001 | Standard Description Standard — Standard zur Beschreibung von Standards | desc | 1.0.0 | approved | standards/desc/ATC-STD-DESC-001.md |
| ATC-STD-ENG-001 | Software Engineering & Code Quality Standard (Determinism First-Class, Security by Design, CI-Gates, Code Quality Matrix) | eng | 1.0.0 | approved | standards/eng/ATC-STD-ENG-001.md |
| ATC-STD-ERR-000 | No Local Fix Without System Verification (Error Master) | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-000.md |
| ATC-STD-ERR-001 | Error Discovery | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-001.md |
| ATC-STD-ERR-002 | Error Classification | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-002.md |
| ATC-STD-ERR-003 | Root Cause Analysis & Pattern Extraction | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-003.md |
| ATC-STD-ERR-004 | Error Propagation Scan | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-004.md |
| ATC-STD-ERR-005 | Cross-Repository Error Scan | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-005.md |
| ATC-STD-ERR-006 | Documentation Consistency Check | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-006.md |
| ATC-STD-ERR-007 | Dependency Impact Analysis | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-007.md |
| ATC-STD-ERR-008 | Regression Test Requirement | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-008.md |
| ATC-STD-ERR-009 | Preventive Control | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-009.md |
| ATC-STD-ERR-010 | Error Pattern Detection | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-010.md |
| ATC-STD-ERR-011 | Fix Verification | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-011.md |
| ATC-STD-ERR-012 | Post-Fix Audit | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-012.md |
| ATC-STD-ERR-013 | Knowledge Capture | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-013.md |
| ATC-STD-ERR-014 | Recurrence Monitoring | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-014.md |
| ATC-STD-ERR-015 | Error Prevention Gate | err | 1.0.0 | approved | standards/err/ATC-STD-ERR-015.md |
| ATC-STD-FRAMEWORK-001 | ATC Enterprise Standards Framework — Master-Dokument (ATC-STANDARDS-MASTER): Zusammenführung aller Standards, Katalog, Kollisionsauflösung, einheitliche Status-/Change-/Traceability-Modelle, Register-Architektur | framework | 1.0.7 | approved | standards/framework/ATC-STD-FRAMEWORK-001.md |
| ATC-STD-IMPLEMENTATION-001 | Standard Implementation Matrix | implementation | 1.0.0 | approved | standards/implementation/ATC-STD-IMPLEMENTATION-001.md |
| ATC-STD-IMPROVEMENT-001 | ATC Improvement Standard — Systemverbesserungsstandard: Continuous Improvement Management System (Zyklus, 14 Quellen, 10 Klassen, ATC-IMP-Board, Root-Cause, Regression Prevention, Automatisierungsleiter, DoD, org-weite Anwendung) | improvement | 1.0.0 | approved | standards/improvement/ATC-STD-IMPROVEMENT-001.md |
| ATC-STD-LEGAL-002 | Impressum & Anbieterkennzeichnung Standard | legal | 1.0.0 | approved | standards/legal/ATC-STD-LEGAL-002.md |
| ATC-STD-LICENSE-001 | License Governance Standard | license | 1.1.0 | approved | standards/license/ATC-STD-LICENSE-001.md |
| ATC-STD-LICENSE-002 | License Specification Standard | license | 1.1.0 | approved | standards/license/ATC-STD-LICENSE-002.md |
| ATC-STD-LICENSE-003 | License Registry Standard | license | 1.1.0 | approved | standards/license/ATC-STD-LICENSE-003.md |
| ATC-STD-LICENSE-004 | License Manifest Standard | license | 1.0.0 | approved | standards/license/ATC-STD-LICENSE-004.md |
| ATC-STD-LICENSE-005 | Third-Party License Management Standard | license | 1.1.0 | approved | standards/license/ATC-STD-LICENSE-005.md |
| ATC-STD-LICENSE-006 | License Compliance Standard | license | 1.0.0 | approved | standards/license/ATC-STD-LICENSE-006.md |
| ATC-STD-LICENSE-007 | License Audit Standard | license | 1.1.0 | approved | standards/license/ATC-STD-LICENSE-007.md |
| ATC-STD-LICENSE-008 | Trademark Separation Standard | license | 1.1.0 | approved | standards/license/ATC-STD-LICENSE-008.md |
| ATC-STD-LICENSE-009 | License Versioning Standard | license | 1.1.0 | approved | standards/license/ATC-STD-LICENSE-009.md |
| ATC-STD-MAINT-000 | Maintenance Governance & Specification Standard (Parent Standard: Readiness Gate, RACI, M0-M3, Evidence, Machine-Readable Conformance) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-000.md |
| ATC-STD-MAINT-001 | Maintenance Classification Standard (M0-Routine, M1-Operational, M2-Security, M3-Critical verbindlich) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-001.md |
| ATC-STD-MAINT-002 | Maintenance Lifecycle Standard (DETECT..CLOSE, 13 Phasen, GSEPF-Mapping) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-002.md |
| ATC-STD-MAINT-003 | Code Maintenance Standard (Refactoring, Technical Debt, Deprecation) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-003.md |
| ATC-STD-MAINT-004 | Dependency Maintenance Standard (Dependencies, Lockfiles, SBOM, Updates, CVE-Reaktion) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-004.md |
| ATC-STD-MAINT-005 | Security Maintenance Standard (CVEs, Hardening, Secrets, Keys, Sandbox, SoD) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-005.md |
| ATC-STD-MAINT-006 | Infrastructure Maintenance Standard (CI/CD, Runner, Build-/Release-Infrastruktur) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-006.md |
| ATC-STD-MAINT-007 | OS Maintenance Standard (Kernel, Treiber, Runtime, Systemdienste, TCB-Grenzen) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-007.md |
| ATC-STD-MAINT-008 | Blockchain Maintenance Standard (Node, P2P, State, Storage, Consensus) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-008.md |
| ATC-STD-MAINT-009 | VM & Runtime Maintenance Standard (ATC-VM, ABI/API, 7-Stufen-Pruefkette) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-009.md |
| ATC-STD-MAINT-010 | AI Maintenance Standard (Modelle, Registry, Inference, Evaluation) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-010.md |
| ATC-STD-MAINT-011 | Repository Maintenance Standard (Bindung an REPO-MAINT-001-SSOT) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-011.md |
| ATC-STD-MAINT-012 | Standards Maintenance Standard (Standards, Registry, Schemas, Conformance) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-012.md |
| ATC-STD-MAINT-013 | Documentation Maintenance Standard (README, Wiki, API, Architektur) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-013.md |
| ATC-STD-MAINT-014 | Performance Maintenance Standard (CPU, RAM, I/O, Netzwerk, Latenz) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-014.md |
| ATC-STD-MAINT-015 | Reliability Maintenance Standard (Monitoring, Backup, Recovery, Regression) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-015.md |
| ATC-STD-MAINT-016 | Compatibility Maintenance Standard (Bindung an COMPAT-001-SSOT) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-016.md |
| ATC-STD-MAINT-017 | Upgrade & Migration Standard (Version Upgrades, Migrationen, Schemaaenderungen) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-017.md |
| ATC-STD-MAINT-018 | Rollback & Recovery Standard (Rollback, Restore, Failover, A/B-Prinzip) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-018.md |
| ATC-STD-MAINT-019 | Maintenance Evidence Standard (maschinenlesbares Record-Schema, No Evidence No Trust) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-019.md |
| ATC-STD-MAINT-020 | Maintenance Automation Standard (Scanner, Maintenance Engine, M0 Auto/M1 Review/M2-M3 Escalate) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-020.md |
| ATC-STD-MAINT-021 | Emergency Maintenance Standard (M3-Emergency-Pfad, Incident Records, Post-Mortem-Evidence) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-021.md |
| ATC-STD-MAINT-022 | End-of-Life & Retirement Standard (Deprecation-Sunset-Retirement, Consumer-Koordination, Retirement-Evidence) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-022.md |
| ATC-STD-MAINT-023 | Vendor & Supply-Chain Maintenance Standard (Vendor-Abhaengigkeiten, Provenance/SBOM, Third-Party-Risiken, Ersatz-Strategien) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-023.md |
| ATC-STD-MAINT-024 | Cross-Ecosystem Maintenance Standard (Multi-Repo-/Schicht-Koordination, konsolidierte Evidence, Schichtreihenfolge) | maint | 1.0.0 | approved | standards/maintenance/ATC-STD-MAINT-024.md |
| ATC-STD-MD-001 | ATC Markdown & Documentation Standard | md | 1.0.0 | approved | standards/md/ATC-STD-MD-001.md |
| ATC-STD-MILESTONE-001 | ATC Milestone Standard — Verbindliche Meilenstein-Governance: Zustandsnachweis, Lebenszyklus, Acceptance Gates, Evidence Packs, maschinenlesbare Registry | milestone | 1.0.0 | approved | standards/milestone/ATC-STD-MILESTONE-001.md |
| ATC-STD-NET-001 | Devnet Standard | net | 1.0.0 | approved | standards/net/ATC-STD-NET-001.md |
| ATC-STD-NET-002 | Testnet Standard | net | 1.0.0 | approved | standards/net/ATC-STD-NET-002.md |
| ATC-STD-NET-003 | Mainnet Standard | net | 1.0.0 | approved | standards/net/ATC-STD-NET-003.md |
| ATC-STD-NET-004 | Network Promotion Standard | net | 1.0.0 | approved | standards/net/ATC-STD-NET-004.md |
| ATC-STD-NET-005 | Network Genesis Standard | net | 1.1.0 | approved | standards/net/ATC-STD-NET-005.md |
| ATC-STD-NET-006 | Network Upgrade Standard | net | 1.1.0 | approved | standards/net/ATC-STD-NET-006.md |
| ATC-STD-NET-007 | Network Security Standard | net | 1.1.0 | approved | standards/net/ATC-STD-NET-007.md |
| ATC-STD-NET-008 | Network Recovery Standard | net | 1.1.0 | approved | standards/net/ATC-STD-NET-008.md |
| ATC-STD-NET-009 | Network Release Maturity & Versioning Standard (NETWORK x MATURITY, Reifestufen-Matrix Alpha/Beta/Stable, 6 Release Gates GATE-DEV/TEST/MAINNET, Versionslogik 0.x-Prerelease/1.0.0+, Release State SSOT registry/networks.yaml) | net | 1.0.0 | approved | standards/net/ATC-STD-NET-009.md |
| ATC-STD-PROTOCOL-001 | ATC Protocol Standards — Dachstandard oberhalb der Einzelprotokolle: einheitliche Regeln für Identität, Versionierung, Nachrichten, Sicherheit, Fehler, Kompatibilität, Governance und Auditing aller ATC-Protokolle (ATC-PROTO-*) | protocol | 1.0.0 | approved | standards/protocol/ATC-STD-PROTOCOL-001.md |
| ATC-STD-PROTOCOL-002 | ATC Protocol Conformance- & Interoperabilitäts-Test-Standard — CONF-Pläne je Familie, 10 Pflicht-Testkategorien, Stufen CONF-BRONZE/SILBER/GOLD, Conformance-Registry, active-Gate-Verschärfung | protocol | 1.0.0 | approved | standards/protocol/ATC-STD-PROTOCOL-002.md |
| ATC-STD-PROTOCOL-003 | ATC Protocol Threat-Model- & Security-Audit-Standard — 12 Pflicht-Angriffe je Familie, Ehrlichkeitsregel, Security-Registry, Audit-Kadenz, Crypto-HAL-Disziplin | protocol | 1.0.0 | approved | standards/protocol/ATC-STD-PROTOCOL-003.md |
| ATC-STD-README-001 | README Standard | readme | 1.0.0 | approved | standards/readme/ATC-STD-README-001.md |
| ATC-STD-REGISTRY-001 | ATC Registry Management Standard — Verbindliche SSOT-Verwaltung aller ATC-Registries: Inventar mit Zuständigkeiten, Single-Source-of-Truth-Prinzip, Generatoren statt Handarbeit, Cross-Registry-Konsistenz über Validator-Gates, Prozess für neue Registries, Manipulationsschutz | governance-core | 1.1.0 | approved | standards/governance-core/ATC-STD-REGISTRY-001.md |
| ATC-STD-REPO-AUDIT-001 | ATC Repository Audit Standard — Verbindlicher, reproduzierbarer Repository Health Check: 16 Prüfbereiche, Prüfmatrix, SOLL/IST, Gap Analysis, Findings, Health Score A-E | repo-audit | 1.0.0 | approved | standards/repo-audit/ATC-STD-REPO-AUDIT-001.md |
| ATC-STD-REPO-AUDIT-002 | ATC Repository Audit Checklisten- & Health-Score-Standard — Konkrete automatisierbare Checks (CHECK-001, CHECK-002, …) und standardisierter Repository Health Score | repo-audit | 1.0.0 | approved | standards/repo-audit/ATC-STD-REPO-AUDIT-002.md |
| ATC-STD-REPO-AUDIT-003 | Automatisierter ATC Repository Auditor — verbindliche Spezifikation des KI-/Automatisierungsagenten für reproduzierbare Repository-Audits: Mandat, 17-Schritte-Pipeline, Read-Only-Pflicht, Gates, AUD-Report-Erzeugung | repo-audit | 1.0.0 | approved | standards/repo-audit/ATC-STD-REPO-AUDIT-003.md |
| ATC-STD-REPO-DISCOVERY-001 | Content Discovery | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-001.md |
| ATC-STD-REPO-DISCOVERY-002 | Change Detection | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-002.md |
| ATC-STD-REPO-DISCOVERY-003 | Standard Candidate Detection | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-003.md |
| ATC-STD-REPO-DISCOVERY-004 | Duplicate Detection | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-004.md |
| ATC-STD-REPO-DISCOVERY-005 | Cross-Repository Discovery | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-005.md |
| ATC-STD-REPO-DISCOVERY-006 | Impact Analysis | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-006.md |
| ATC-STD-REPO-DISCOVERY-007 | Dependency Discovery | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-007.md |
| ATC-STD-REPO-DISCOVERY-008 | Security-Relevant Content Detection | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-008.md |
| ATC-STD-REPO-DISCOVERY-009 | Documentation Gap Detection | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-009.md |
| ATC-STD-REPO-DISCOVERY-010 | Discovery Audit & Reporting | repo-discovery | 1.0.0 | approved | standards/repo-discovery/ATC-STD-REPO-DISCOVERY-010.md |
| ATC-STD-REPO-MAINT-001 | Repository Maintenance & Lifecycle Standard | repo-maint | 1.0.0 | approved | standards/repo-maint/ATC-STD-REPO-MAINT-001.md |
| ATC-STD-SC-001 | Smart Contract General Standard | sc | 1.0.0 | approved | standards/sc/ATC-STD-SC-001.md |
| ATC-STD-SC-002 | Smart Contract Identity Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-002.md |
| ATC-STD-SC-003 | Smart Contract Security Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-003.md |
| ATC-STD-SC-004 | Smart Contract Testing Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-004.md |
| ATC-STD-SC-005 | Smart Contract Audit Standard | sc | 1.0.0 | approved | standards/sc/ATC-STD-SC-005.md |
| ATC-STD-SC-006 | Smart Contract Deployment Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-006.md |
| ATC-STD-SC-007 | Smart Contract Upgrade Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-007.md |
| ATC-STD-SC-008 | Smart Contract Event Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-008.md |
| ATC-STD-SC-009 | Smart Contract Access Control Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-009.md |
| ATC-STD-SC-010 | Smart Contract Treasury Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-010.md |
| ATC-STD-SC-011 | Token Contract Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-011.md |
| ATC-STD-SC-012 | NFT Contract Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-012.md |
| ATC-STD-SC-013 | DeFi Contract Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-013.md |
| ATC-STD-SC-014 | Governance Contract Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-014.md |
| ATC-STD-SC-015 | Bridge Contract Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-015.md |
| ATC-STD-SC-016 | Oracle Contract Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-016.md |
| ATC-STD-SC-017 | GameFi Contract Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-017.md |
| ATC-STD-SC-018 | Mining Contract Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-018.md |
| ATC-STD-SC-019 | Contract Registry Standard | sc | 1.0.0 | approved | standards/sc/ATC-STD-SC-019.md |
| ATC-STD-SC-020 | AI-Assisted Smart Contract Development Standard | sc | 1.1.0 | approved | standards/sc/ATC-STD-SC-020.md |
| ATC-STD-STDDEV-001 | ATC Standards Development Standard — Verbindlicher Lebenszyklus für Standards: Erstellung im Hausformat, Review, §9-Freigabe, §30-Einfrierung, Wartung (MINOR/MAJOR), Review-Zyklen, Deprecation und Retirement | governance-core | 1.0.0 | approved | standards/governance-core/ATC-STD-STDDEV-001.md |
| ATC-STD-TAXONOMY-001 | ATC Standards Taxonomy & Family Creation Standard — Meta-Governance: vierstufige Taxonomie (Domain→Familie→Kategorie→Standard), kontrollierte Familien-/Kategorie-Erstellung, Lifecycle, automatische ID-Vergabe, TAX-CHECK-001..018 | taxonomy | 1.0.0 | approved | standards/taxonomy/ATC-STD-TAXONOMY-001.md |
| ATC-STD-TUD-001 | Technology Uniqueness & Differentiation Standard | governance | 1.0.0 | approved | standards/technology-uniqueness/ATC-STD-TUD-001.md |
| ATC-STD-UPDATE-001 | ATC Update Standard — Change Control für Artefakt-Updates: Kategorien, Lifecycle, Impact/Dependency, Gates UPD-G01..G09, Rollback, Emergency, Manifeste, Review-Kadenzen | update | 1.0.0 | approved | standards/update/ATC-STD-UPDATE-001.md |
| ATC-STD-V2S-000 | Vision-to-Software Lifecycle Master Standard | v2s | 1.0.0 | approved | standards/v2s/ATC-STD-V2S-000.md |
| ATC-STD-V2S-001 | Vision Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-001.md |
| ATC-STD-V2S-002 | Idea Intake Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-002.md |
| ATC-STD-V2S-003 | Objectives Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-003.md |
| ATC-STD-V2S-004 | Requirements Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-004.md |
| ATC-STD-V2S-005 | Traceability Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-005.md |
| ATC-STD-V2S-006 | Specification Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-006.md |
| ATC-STD-V2S-007 | Architecture Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-007.md |
| ATC-STD-V2S-008 | Software Design Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-008.md |
| ATC-STD-V2S-009 | Development Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-009.md |
| ATC-STD-V2S-010 | Integration Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-010.md |
| ATC-STD-V2S-011 | Testing Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-011.md |
| ATC-STD-V2S-012 | Verification Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-012.md |
| ATC-STD-V2S-013 | Validation Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-013.md |
| ATC-STD-V2S-014 | Security Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-014.md |
| ATC-STD-V2S-015 | Software Audit Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-015.md |
| ATC-STD-V2S-016 | Release Candidate Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-016.md |
| ATC-STD-V2S-017 | Release Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-017.md |
| ATC-STD-V2S-018 | Deployment Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-018.md |
| ATC-STD-V2S-019 | Operations Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-019.md |
| ATC-STD-V2S-020 | Monitoring Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-020.md |
| ATC-STD-V2S-021 | Incident Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-021.md |
| ATC-STD-V2S-022 | Feedback Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-022.md |
| ATC-STD-V2S-023 | Improvement Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-023.md |
| ATC-STD-V2S-024 | Version Upgrade Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-024.md |
| ATC-STD-V2S-025 | Deprecation Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-025.md |
| ATC-STD-V2S-026 | End-of-Life Standard | v2s | 1.0.0 | candidate | standards/v2s/ATC-STD-V2S-026.md |
| ATC-STD-VERSION-001 | ATC Versioning Standard — einheitliche Versionierung von Software, Standards, APIs, Smart Contracts, Protokollen und Releases | version | 1.0.0 | approved | standards/version/ATC-STD-VERSION-001.md |
| ATC-STD-ZKP-001 | ZKP Architecture Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-001.md |
| ATC-STD-ZKP-002 | Proof System Interface Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-002.md |
| ATC-STD-ZKP-003 | Circuit Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-003.md |
| ATC-STD-ZKP-004 | On-Chain Verification Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-004.md |
| ATC-STD-ZKP-005 | Commitment & Nullifier Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-005.md |
| ATC-STD-ZKP-006 | ZK Identity Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-006.md |
| ATC-STD-ZKP-007 | Private Transaction Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-007.md |
| ATC-STD-ZKP-008 | ZK Rollup Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-008.md |
| ATC-STD-ZKP-009 | ZKVM Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-009.md |
| ATC-STD-ZKP-010 | ZKP Security & Audit Standard | zkp | 1.1.0 | approved | standards/zkp/ATC-STD-ZKP-010.md |

## 5. Statusmodell (Registry-Lifecycle)

Registry-Statusverteilung (Ist): {'approved': 505, 'candidate': 27}. Lifecycle der Standards-Entwicklung
gemäß ATC-STD-STDDEV-001 / ATC-STD-TAXONOMY-001: Entwurf (Owner-Entwurf/SCR) →
§9-Freigabe (Owner, Human-Gate) → APPROVED (normativ, §30-eingefroren) → ggf.
DEPRECATED/RETIRED via Change-Control (ATC-STD-CHANGE-001). Protokolle folgen
zusätzlich REQ-PROTO-021 (draft bis verifizierte Implementierung).

## 6. Prioritätsmodell

P0 (kritisch, blockiert Release) · P1 (hoch, kurzfristig) · P2 (mittel, Roadmap) ·
P3 (Backlog/Optimierung). Verwendet in registry/findings.yaml, Issues und Audit-
Klassifikation (ATC-STD-REPO-AUDIT-001/002).

## 7. Offene Punkte (Auszug — Details: STATUS.md)

- Findings OPEN: 90 (aktuelle Liste: registry/findings.yaml)
- Org-Audit-Ableitungen: Issues #94–98 (a-townchain-os) — CI 23/26, CodeQL,
  Versions-Baseline, verwaister Tag, ATC-STD-202-Klassifizierung
- ATC-LICENSE: 5 Lizenztypen PLANNED (SOURCE, COMMERCIAL, PROPRIETARY, DATA,
  EXPERIMENTAL); ATC-LICENSE.yaml-Manifeste + License Scanner S-26 ausstehend
- Protokollfamilien: 15 planned / 11 draft
  (registry/protocol-registry.yaml)

## 8. Integrität & automatische Prüfung

Die Index-Integritätsregeln werden nicht hier, sondern durch die Validator-Pipeline
erzwungen (tools/atc-std-validator/validate_all.py, je CI-Lauf): S-01 Metadaten,
S-14/S-19 Version/Status-Konsistenz, S-16 Naming, S-17 Duplikate, S-18 Registry-Parse,
S-21 Katalog, S-23 Protokolle, S-24 Taxonomie, S-25 Frontmatter — inklusive
Mutationstests M1–M12 (Fehler-erkennungs-pflichtig). Der Index selbst ist
regenerierbar und kann per Definition nicht driften.

## 9. Governance-Regel

INDEX.md ist Navigationsindex. Eine Änderung an einem Standard darf niemals
ausschließlich hier vorgenommen werden — sie läuft über: SCR → Standard-Datei →
registry/standards.yaml/versions.yaml → CHANGELOG → Validator → INDEX-Regeneration.
Dokument-ID ATC-STD-INDEX-001 ist KEIN Registry-Standard (kein REQ-Träger), sondern
die Kennung dieses generierten Dokuments.

## 10. Verzeichnisstruktur (Ist-Zustand)

atc-standards/ · INDEX.md (generiert) · README.md · CHANGELOG.md · STATUS.md ·
LICENSE (Apache-2.0) · AGENT_MANIFEST.md · AGENTS.md · governance/ (ATC-STD-000) ·
standards/<kategorie>/ (Fachstandards, 532-Bestand) · registry/ (29
SSOT-Dateien) · licenses/ (ATC-LICENSE-System) · schemas/ · tools/ (Generatoren +
atc-std-validator) · approval/ (§9-Freigabe-Archiv) · change-requests/ (SCR-0001…) ·
docs/ (Audits & Analysen) · templates/ · .github/workflows (Governance-CI, 2) +
ai/agent.yaml (Agenten-Bindung).

*ATC-STD-INDEX-001 v1.0.0 · generiert 2026-10-07 · tools/index/gen_index.py · SCR-0038 · Aurora (Superagent)*
