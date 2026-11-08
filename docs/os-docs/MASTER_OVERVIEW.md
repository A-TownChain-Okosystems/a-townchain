# MASTER_OVERVIEW — A-TownChain Okosystems Gesamtdokument

> **ATC-DOC: MASTER-OVERVIEW-001** · Status: ACTIVE · Stand: 2026-10-12 (Europe/Berlin)
> Konsolidiert: Wiki · Roadmap · Sprints · Todos · Komponenten- und Funktionskatalog · Quellen.
> Generator: Aurora (aurora-base44-superagent-6a1d37209d7f46ac065d39c5). Evidence-basiert nach vierstufiger Klassifikation (NORMATIVE → IMPLEMENTED → RESEARCH → LEGACY/UNVERIFIED).
> Maschinenlesbares Gegenstück: `MASTER_OVERVIEW-001.json` (Schema ATC-MASTER-OVERVIEW-001 v0.2.0) — verbindet alle Org-JSON-Artefakte (Katalog, Manifest, Integration, Delivery-Registry, Registries) als Cross-Link-Index; beide Dateien werden gemeinsam aktualisiert.

## 1. Organisation

33 Repositories: **26 aktiv** (davon 1 privat: `demo-repository`) + **7 archiviert** (Referenz: atc-storage, atc-explorer, atc-indexer, atc-interop, atc-mining, atc-oracle, atc-wallet).

| Layer | Repos |
|---|---|
| GOVERNANCE | atc-standards, .github |
| DOCS / Wiki | a-townchain-os-docs (dieses Repo), a-townchain-ecosystem (Integrations-/Audit-Layer) |
| CORE | a-townchain, atclang, atc-shivacore, aurora-ai, globus-os, genesis-engine, genesis-chronicles |
| PROTOCOL SERVICES | atc-node, atc-vm, atc-zkp, atc-algorithm, atc-compute, atc-contracts |
| PRODUKTE / TOOLS | atc-sdk, atc-launchpad, atc-marketplace, atc-ide, atc-toolchain, atc-engineering, genesis-franchise-factory, demo-repository |

## 2. Wiki

Zentrale Wiki- und Dokumentationsflächen im Repo `a-townchain-os-docs` (HEAD `6b311f70d`):

- `wiki/` — Themen-Wikis: aistudio-wiki, atclang-wiki, atcnet, atcpkg-wiki, backend-wiki, blockchain-wiki, bootloader-wiki, ci-wiki, cli-wiki, contracts, dns-wiki, drivers-wiki, explorer-wiki, frontend-wiki, gateway, genesis-engine-wiki, franchise, franchise-factory u.a.
- Referenzmodelle (Root): ATC_ARCHITECTURE_REFERENCE_MODEL.md (v0.1.0, Draft bis Owner-Review), ATC_OS_ARCHITECTURE_REFERENCE_MODEL.md, ATC_VM_ARCHITECTURE_REFERENCE_MODEL.md, ARCHITECTURE.md, ECOSYSTEM.md, ECOSYSTEM_STATUS.md, MILESTONES.md, FILE_REGISTER.md, REALITY_STATUS.md, COMPONENT_PLAN.md, CONNECTION_MAP.md
- Governance-Wiki: Standards-Registry in atc-standards (533 Standards inkl. ATC-AAS, ATC-ENT, SC-001..020, ZKP; Verfassung ATC-STD-000 v1.2.0)
- Integrations-/Audit-Layer: a-townchain-ecosystem (Komponenten-/Funktionskatalog, Census, Readiness-Gates; Katalog aktuell auf PR #115)

## 3. Roadmap — Meilensteine & Checkpoints

### 3.1 A-TownChain OS Meilensteine (v4.0, s. MILESTONES.md)

| MK | Name | Status | Datum | Checkpoint (Bedingung fuer "Erfuellt") |
|----|------|--------|-------|--------------------|
| MK1 | Core OS Foundation | Erfuellt | Q2 2026 | — |
| MK2 | ATCLang Language Core | Erfuellt | 05.07.2026 | Code parst fehlerfrei mit aktuellem Parser |
| MK3 | Blockchain & Consensus | Erfuellt | 05.07.2026 | — |
| MK4 | GCL v2.0 Communication | In Arbeit | Q3 2026 | Kernel-Busse als .atc muessen fehlerfrei parsen |
| MK5 | Smart Contract Engine | Erfuellt | 05.07.2026 | — |
| MK6 | Network & Sharding | Erfuellt | 05.07.2026 | — |
| MK7 | AI Agent Protocol | In Arbeit | Q3 2026 | AIP-001-Spezifikation (a-townchain-os #80) |
| MK8 | Security Audit | In Arbeit | Q3 2026 | Externe Code-Review abgeschlossen |
| MK9 | Mainnet Launch | Geplant | Q4 2026 | Genesis Block signiert (Chain-ID 9000), 10+ Validator bestatigt |
| MK10 | Genesis Engine v1.0 | In Arbeit | Q3 2026 | Code vorhanden, Laufzeit nicht verifiziert |
| MK11 | Multi-Chain Support | Geplant | Q2 2027 | — |
| MK12 | AAA Engine Runtime | In Arbeit | Q3 2026 | Asset-Module sind aktuell Python-Pseudocode, kein ATCLang |
| MK13 | Franchise Factory v2.0 | In Arbeit | Q3 2026 | 28 .atc-Dateien, 0 parsen (v1.0-Dialekt-Problem) |
| MK14 | MetaFactory v3.0 | In Arbeit | Q3 2026 | 8 .atc-Dateien, 0 parsen (v1.0-Dialekt-Problem) |
| MK15 | Civilization Platform v4.0 | In Arbeit | Q3 2026 | 11 .atc-Dateien, 0 parsen (v1.0-Dialekt-Problem) |

Bilanz: 5/15 vollstaendig erfuellt (33,3%), 8/15 in Arbeit, 2/15 geplant (Stand MILESTONES.md 06.07.2026, Status-Definition: "Erfuellt" = Code existiert UND parst fehlerfrei).

### 3.2 Lauffaehigkeits-Roadmap M1–M8 (AD-027, a-townchain-os/ROADMAP.md)

| M | Meilenstein | Status | Checkpoint |
|---|---|---|---|
| M1 | Sprachanpassungen & ATCLang | G1 erfuellt, G2 offen | AD-019 Phase 1 abgeschlossen |
| M2 | Kernel Boot & Core Integration | Erfuellt | atc-shivacore 674/674 Tests gruen |
| M3 | KI-Layer Integration | In Arbeit | aurora-ai / AAS-001..025 |
| M4 | Blockchain Infrastructure | In Arbeit | 2 Nodes, Chain-ID 658467, Contract-Execution auf ATVM |
| M5 | OS-Schicht & Kernel-Services | Offen | — |
| M6 | 13 Blockchain Services Integration | Offen | — |
| M7 | Genesis Engine & Chronicles Integration | Offen | Spiele/NFT auf Chain |
| M8 | Full Monorepo Release Candidate | Offen | Audit-Finalisierung |

### 3.3 KAI-OS Core Runtime Track (S01–S26) — ABGESCHLOSSEN 12.09.2026

10 Crates, 80/80 Tests gruen (Issues #102–#111, GATE-Review #109). Sprints S01–S26 wie spezifiziert umgesetzt (S19–S20 abweichend belegt, S21–S22 am 12.09. nachgeholt).

**GATE-Iteration-2-Checkpoints:**

| Checkpoint | Status |
|---|---|
| G2-A Model Registry + Verified Cache | Erfuellt 12.09. |
| G2-C Echter TCP-Transport | Erfuellt 12.09. |
| G2-D Upgrade/Rollback (Marker-Protokoll + Versionierung) | Erfuellt 12.09. |
| Multi-Node-DR (syncd-Crate, 5 E2E-Tests mit echten Prozessen) | Erfuellt 12.09. (#113) |
| G2-B Persistente Storage-Layer | OFFEN |
| G2-E Externes Security-Audit (Checkliste vorbereitet, Auditer-Entscheidung beim Owner) | OFFEN |
| Code closure (GATE-Iteration 2) | Verifiziert, externe Produktions-Gates offen (a-townchain-os #112) |

### 3.4 Kernel-Sprints K0–K50 (atc-shivacore / a-townchain-os)

Abgeschlossen: 60 Kernel-Module, ~2146 Rust-Tests, 30 ATCLang-Interfaces, v1.0.0-Release (K4-Konsolidierung MK-K1–K4 Done). Einzel-Sprints (K1–K50) vollstaendig in der Task DB als Done verzeichnet (K9–K13 als in andere Sprints aufgegangen dokumentiert).

### 3.5 Versions- und Release-Meilensteine

| Release | Status | Checkpoint |
|---|---|---|
| v1.0.0 (a-townchain-os) | Veroeffentlicht 04.08.2026 | Tag + GitHub Release |
| v3.2.1 — Test Coverage & Integration | Done (Task DB) | Alle Issues geschlossen |
| v2.1.0 / v2.2.0 / v3.0.0-Vorarbeiten | Done | — |
| Konsens-Spezifikation v3.0.0 (Freeze Hybrid PoH+PoS+PoW) | OFFEN | atc-algorithm, Governance-Entscheidung |
| Mainnet v1.0 (MK9) | Geplant Q4 2026 | SC-Gates G0–G13 bestanden ("kein Gate, kein Mainnet"), Genesis-Block, 10+ Validator |

### 3.6 Governance-Roadmap & Checkpoints (Stand 2026-10-12)

Abgeschlossen: 103 Standards APPROVED (Verfassung ATC-STD-000 v1.2.0, AAS-001..025, ENT-001..015, SC-001..020, README-001, MD-001, ZKP-001..010, AI-DEV-001..012, 201–204), F-160 RESOLVED, Governance-Freeze per §30, ATC-AURORA-SWARM-001 v0.1.0-DRAFT erstellt (47 REQ-ASW, SCR-0138).

| Checkpoint | Status | Naechster Schritt |
|---|---|---|
| README-/MD-Konformitaet 26 Repos (#116/#117) | In Arbeit (Frist 07.10.2026 ueberschritten) | Rollout abschliessen, CI-Integration erst nach Owner workflow-Scope |
| Agent-Manifeste + AGENTS.md (#111) | In Arbeit | Restliche Repos |
| SCR-0007 REQ-ID-Rollout 52 Standards | OFFEN | Owner-Entscheidung |
| SC-Gate-Rollout Contract-Repos + Registry-Befuellung SC-G0 (#118) | In Arbeit | je Contract |
| ASW-G0 §9-Freigabe ATC-AURORA-SWARM-001 | OFFEN | Owner-Freigabe |
| IF-002 Interface-Review / IF-001 Dependency Graph | In Arbeit / Blocked (Dependency-Graph-UI) | s. Owner-Aktionen |

### 3.7 Technische Evidence-Checkpoints

| Checkpoint | Status |
|---|---|
| GlobusOS UEFI-Boot Intel (Skylake) + AMD (EPYC) | VERIFIZIERT (PR #58/#59) |
| GlobusOS Secure Boot / ACPI / Bare-Metal | RESIDUAL_NOT_ASSERTED (PR #73 Boot-Evidence-Provenance als Draft) |
| Census-Baseline Organisation | 2026-10-12, RESIDUAL, 25 gescannt + 1 privat ausgeschlossen; a-townchain a70b16f2d (VM-Fixes PR #71) |
| Katalog-Reconcile (98 Funktionen) | RESIDUAL, 0 blockierende Findings; Merge PR #115 ausstehend |
| scanner-tests CI-Gruen | BLOCKED auf Owner-Edit der Workflow-Assertion (Vorlage liegt vor) |
| ATVM vs. ShivaVM kanonische Contract-VM | OFFEN (Governance-Entscheidung, v3.x) |
| ATC-07 Sharding-Doku-Drift | OFFEN (#84 closed, Implementierung UNVERIFIED) |

### 3.8 Notion

Master Roadmap (A-TownChain OS + KAI-OS, konsolidiert): Notion-Seite 373b826d-b85c-8125-ba83-f04995191bf0, Auto-Sync GitHub → Task DB → Notion aktiv.

## 4. Sprints (aktiv)

| Sprint | Repo | Zustand |
|---|---|---|
| Sprint 3.0 — AIP-001 Agent Interaction Protocol | a-townchain-os #80 | In Arbeit (Spezifikation) |
| Katalog-Completeness (PR #115, 98 Funktionen) | a-townchain-ecosystem | Draft, 19/20 Checks gruen, nur scanner-tests rot (wartende Workflow-Assertion, s. Owner-Aktionen) |
| GlobusOS Boot-Evidence-Provenance (PR #73) | globus-os | Draft; Secure Boot/ACPI/Bare-Metal RESIDUAL |
| ATCLang PR-Backlog (#14–#20) | atclang | 6/7 gruen, #19 wartet MD025-Policy-Pin |
| Sprint 3.3 Security / Sprint 4.0 Mainnet | a-townchain-os | Issues geschlossen (Vorbereitung MK8/MK9) |
| K1–K50 Kernel-Sprints | atc-shivacore / a-townchain-os | 60 Kernel-Module, ~2146 Rust-Tests, 30 ATCLang-Interfaces |

## 5. Todos

### Offene GitHub-Issues (46 org-weit, Stand 2026-10-12)

| Repo | Issue | Titel | Labels |
|---|---|---|---|
| .github | #12 | [AD-008] Sprach-Migration L5/L6: 8 Repos von Python/TypeScript auf Soll-Sprachen | — |
| a-townchain | #70 | P1: Verify deterministic A-TownChain runtime, persistence and multi-node consensus | P1 |
| a-townchain | #64 | SCR-0127: Resolve missing canonical components/contracts migration | architecture, migration |
| a-townchain | #31 | P0: Canonical Crypto Contract reconciliation — secp256k1 TX/Account, Ed25519 Identity | P0 |
| a-townchain | #20 | P1 audit: evidence binding and ZKP placeholder boundary | P1, security |
| a-townchain-ecosystem | #112 | P0: Vollständigen Quellcodeabgleich für Komponenten- und Funktionskatalog durchführen | — |
| a-townchain-ecosystem | #98 | Assurance Residual Register — PR #97 freeze record | — |
| a-townchain-ecosystem | #87 | P0: Fehlende Protokolle, Laufzeiten und Standardbibliothek systematisch schließen | P0 |
| a-townchain-ecosystem | #71 | Master Architecture: canonical A-TownChain layer model L0–L7 + Cross-Layer Control Pl | architecture |
| a-townchain-os | #124 | P1 audit: Integration Gate workspace path | audit |
| a-townchain-os | #122 | P1/P2 audit: workspace documentation and evidence drift | documentation, governance |
| a-townchain-os | #112 | [GATE-Iteration 2] Code closure verified — external production gates remain | — |
| a-townchain-os-docs | #41 | P0/P1: Census 2026-10-09 konsolidieren und Gap-Audit mit Exact-SHA-Evidence abschließ | documentation, audit |
| a-townchain-os-docs | #33 | CONSISTENCY-001: migrate active docs/config/wallet artifacts to canonical L1 values | consistency |
| atc-algorithm | #3 | Owner-Aktion: Dependency Graph auf Org-Ebene aktivieren — sonst bleibt P0-Gate rot (C | governance, owner-action |
| atc-engineering | #13 | P0: Organisationsweiter Funktions- und Komponenten-Census mit maschinenlesbarer Dokum | documentation, P0 |
| atc-engineering | #12 | P0: Machine-readable PR comments and gate-failure remediation loop | P0, automation |
| atc-engineering | #10 | P0: Red-Gate Triage and Fail-Closed Auto-Remediation Loop | P0, automation |
| atc-engineering | #2 | P0 — Configure ATC_FLEET_READ_TOKEN for complete private-repository fleet audit | P0, security |
| atc-launchpad | #3 | Owner-Aktion: Dependency Graph auf Org-Ebene aktivieren — sonst bleibt P0-Gate rot (C | governance, owner-action |
| atc-node | #10 | P2: RPC serve() beendet sich bei per-Verbindungs-Fehlern (EPIPE) — Devnet-Flake Root  | — |
| atc-shivacore | #19 | Fix DependencyGraph::dependencies() placeholder in LKM subsystem | bug, security |
| atc-standards | #79 | ci: restore pull-request creation permission for regenerate-views | governance |
| atc-standards | #77 | governance: enforce generated-artifact consistency for every generator | — |
| atc-standards | #76 | governance: audit main-HEAD provenance across organization | — |
| atc-standards | #70 | governance: resolve pre-existing dependency security pinning conflict | governance, security |
| atc-standards | #53 | P0: Reconcile frozen ATC-STD-600 transaction signature contract | P0 |
| atc-standards | #50 | L1-TX-001: Canonical Transaction Contract and deterministic serialization | — |
| atc-standards | #13 | Owner-Aktion: 14 CI-Gates pushen — 12× Determinism + Integration + Docs (Ziel 26/26 P | — |
| atc-vm | #8 | P1: implement enforced gas metering and canonical bytecode verification | P1, security |
| atclang | #12 | P1 audit blockers: reference VM security and Rust production boundary | security, P1 |
| atclang | #9 | P1 security: fail-closed Python reference VM crypto/auth/network stubs | security, P1 |
| aurora-ai | #37 | P0: Reconcile Aurora AI target architecture with canonical architecture and implement | documentation, architecture |
| aurora-ai | #35 | P1: Aurora AI isolated runtime, model inference and policy-bound tool execution | P1 |
| aurora-ai | #33 | P0: Aurora-AI-Grundfunktionen katalogisieren, absichern und SHA-verifizieren | — |
| genesis-engine | #16 | GENESIS: Species & Civilization Framework | architecture, gameplay |
| globus-os | #67 | feat(identity): implement username/password account lifecycle with 24-word recovery | — |
| globus-os | #63 | P0: Fail-closed GitHub API retrieval — reject empty/incomplete evidence | ci, governance |
| globus-os | #59 | governance: close post-merge assurance residuals for PR #58 | governance |
| globus-os | #57 | P1: GlobusOS user-space boot, service manager, storage, networking and recovery | P1 |
| globus-os | #51 | P0: Betriebssystem-Grundfunktionen katalogisieren, SSOT abgleichen und verifizieren | — |
| globus-os | #45 | P0: ShivaCore TCB completion — Architecture Delta, VMM, Build, Hardware and Evidence | P0 |
| globus-os | #44 | CI-RED: Rust CI "Check" fehlschlaegt am main (e28a055) — Kernel-Tests werden nie ausg | bug, kernel |
| globus-os | #20 | [P1][LKM][SECURITY] Enforce dependency ordering, fail-closed imports, and export/impo | P1, kernel |
| globus-os | #19 | P1: LKM export builder incorrectly registers exported symbols as imports | bug, P1 |
| globus-os | #18 | [P1][STUB][LKM] Replace DependencyGraph::dependencies placeholder with owned determin | P1, stub |

### Offene PR-Backlogs

| Repo | Offene PRs |
|---|---|
| .github | 2 |
| a-townchain | 17 |
| a-townchain-ecosystem | 33 |
| a-townchain-os | 4 |
| a-townchain-os-docs | 5 |
| atc-algorithm | 2 |
| atc-compute | 1 |
| atc-contracts | 6 |
| atc-engineering | 4 |
| atc-launchpad | 1 |
| atc-marketplace | 1 |
| atc-node | 3 |
| atc-sdk | 3 |
| atc-shivacore | 2 |
| atc-standards | 22 |
| atc-vm | 5 |
| atc-zkp | 2 |
| atclang | 1 |
| aurora-ai | 4 |
| demo-repository | 1 |
| genesis-chronicles | 1 |
| genesis-engine | 14 |
| genesis-franchise-factory | 1 |
| globus-os | 34 |

Groeßte Backlogs: globus-os (34), a-townchain-ecosystem (33, inkl. #115 Katalog), atc-standards (22), a-townchain (17), genesis-engine (14). Review-Last konzentriert auf diese fuenf Repos.

### Task DB — offene/blocked (Auszug)

- Blocked: SCR-0086 Evidence-Promotion (Owner workflow-Scope), INFRA-DEPGRAPH-001 Dependency Graph auf 6 Repos (UI-Aktivierung, RESIDUAL/BLOCKED)
- Open: ATVM-vs-ShivaVM-Entscheidung, Kernel-Evidence-Runs, Exact-SHA-Evidence Data/Network/Application, ATC-07-Drift, Konsens-Freeze v3.0.0, SCR-0007 REQ-ID-Rollout, F-003..F-007

### Owner-Aktionen (nicht agent-seitig loesbar)

1. **2-Zeilen-Workflow-Edit** (Ecosystem-CI): `assert active_scanned == 26` ersetzen durch `active_blocked == 0` + `active_scanned + active_excluded_private == active_expected` (Vorlage: PR #115 Kommentar issuecomment-6115900594; alternativ demo-repository public stellen oder Census-Credential provisionieren)
2. **MD025-Policy-Pin**: .github/workflows feat/org-markdownlint Pin 1c198f3→61f1941 (ATCLang #19 + markdownlint-Fehlalarme)
3. **Dependency Graph org-weit aktivieren** (UI: org Security Configuration → Dependency scanning → alle Repos), danach Re-Run der dependency-review-Jobs
4. **PR-Reviews**: #115 (Katalog) und ATCLang #14–#20
5. **ASW-G0 §9-Freigabe** ATC-AURORA-SWARM-001
6. **SCR-0086**: Branch-Protection-Blocker fuer Evidence-Commits loesen

## 6. Komponenten- und Funktionskatalog

Ort: `a-townchain-ecosystem/docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.json` + `.md` — aktuell auf **PR #115 (Draft, Head 8a4708794)**, noch nicht auf main.

- **30 Komponenten / 98 Funktionen** (63 erstes Supplement → 71 ATCLang-Pass → 98 Komponenten-/Spezifikations-Pass)
- Source-Bindings: **66 census-verifizierte Lexical-Matches** (funktionale Verifikation ausstehend), **27 nicht im Lexical-Scan gefunden** (12 Spec-/Doc-/Data-Bindings + 15 ATC-VM-Private-Symbole PRIVATE_SYMBOL_NOT_IN_PUBLIC_CENSUS), **5 explizite Blocked-Records** (archiviert/Referenz)
- repository_coverage: 33 Repos (26 aktiv, 7 archiviert), fail-closed Status je Eintrag
- Reconcile-Status: RESIDUAL (0 blockierende Findings); Validator PASS_STRUCTURAL_ONLY; Census-Basis a-townchain a70b16f2d (VM-Resource-Bounds-Fixes PR #71)
- Katalog-Baselines sind SHA-spezifisch; bei Repo-Bewegung (wie a-townchain 0419b444d→a70b16f2d) ist ein Refresh erforderlich (vorbereitet, uncommittet auf dem PR-Branch)
- Funktionsliste im Detail: Katalog-MD (Abschnitt je Komponente, FN-IDs mit Source-Pfad, Symbol, exaktem SHA, Evidence-URL)

## 7. Quellen (Evidence-Basis 2026-10-12)

| Repo | main SHA |
|---|---|
| .github | `af5d803da` |
| a-townchain | `a70b16f2d` |
| a-townchain-ecosystem | `5a66f276a` |
| a-townchain-os | `3f85a6e61` |
| a-townchain-os-docs | `6b311f70d` |
| atc-algorithm | `5a5628044` |
| atc-compute | `d0831dfb7` |
| atc-contracts | `a7c8b417f` |
| atc-engineering | `2e91ec61a` |
| atc-ide | `2a17feda7` |
| atc-launchpad | `bd1a48c69` |
| atc-marketplace | `5c3bab7da` |
| atc-node | `a817db43a` |
| atc-sdk | `34eef3459` |
| atc-shivacore | `21c997696` |
| atc-standards | `dc0ff69d8` |
| atc-toolchain | `82fe4a238` |
| atc-vm | `21132a493` |
| atc-zkp | `ffa8ec109` |
| atclang | `21c595dc3` |
| aurora-ai | `1b5ff1824` |
| demo-repository | `d9b0da9c9` |
| genesis-chronicles | `a3f417b23` |
| genesis-engine | `8326eb53e` |
| genesis-franchise-factory | `673fa5603` |
| globus-os | `c7337ef4f` |

- Census: tools/run_org_source_census.py (a-townchain-ecosystem), letzte Vollprüfung 2026-10-12: RESIDUAL, 25 gescannt + 1 privat ausgeschlossen (demo-repository, EXCLUDED_PRIVATE_NO_CI_ACCESS), 0 blockiert; 4.925 Quelldateien, 35.581 Kandidaten-Symbole
- Katalog-Audit: docs/audit/ATC-CATALOG-COMPLETENESS-AUDIT-001.md (PR #115, inkl. Ausschluss-Rationale R-1)
- Standards: Registry atc-standards/registry/standards.yaml (SSOT), Verfassung ATC-STD-000 v1.2.0, 103 APPROVED
- Notion Master Roadmap: 373b826d-b85c-8125-ba83-f04995191bf0
- MILESTONES.md / REALITY_STATUS.md / FILE_REGISTER.md (dieses Repo, SHA 6b311f70d)
- Offene Issues: Live-Scan aller 26 aktiven Repos via GitHub API (2026-10-12)

## Pflege

Dieses Dokument wird bei Sync-Laeufen aktualisiert (GitHub → Task DB → Notion Automation). Aenderungen nur mit Evidence; Statusaussagen strikt nach vierstufiger Evidence-Klassifikation. Neue Todos aus Issues/Findungen fließen automatisch in Abschnitt 5 ein.

## 8. Maschinenlesbare Artefakt-Verbindungen

`MASTER_OVERVIEW-001.json` (v0.2.0) verbindet alle organisatorischen JSON-Artefakte:

| Artefakt | Repo | Zustand | Rolle |
|---|---|---|---|
| ATC-ORG-REPOSITORY-MANIFEST-001.json | a-townchain-ecosystem | PR #115 | SSOT Repo-Inventar (33 Repos), Census-Grundlage |
| ATC-ORG-REPOSITORY-INTEGRATION-001.json | a-townchain-ecosystem | PR #115 | Integrations-/Boundary-Registry |
| ATC-COMPONENT-FUNCTION-CATALOG-001.json | a-townchain-ecosystem | PR #115 | Komponenten-/Funktionsverzeichnis (30/98) |
| ATC-COMPONENT-FUNCTION-SPECIFICATION-001.json | a-townchain-ecosystem | PR #115 | Normative Contract-Pflichtfelder |
| DELIVERY_REGISTRY.json | a-townchain-ecosystem | main | Control-Plane-Liefer-/Nachweis-Registry |
| AURORA_DECENTRALIZED_MEMORY_FUNCTION_CATALOG.json | aurora-ai | main | Spezialkatalog Aurora Memory |
| specs/language + semantics/registry.json | atclang | main | Normative ATCLang-Token-/Semantik-Registry |
| ORG-GOV-SCAN + ORG-COMPLIANCE-SCAN-*.json | .github | main | Governance-/Compliance-Snapshots |
| governance-sources.schema.json | .github | main | JSON-Schema der Governance-Quelldaten |

Dazu `cross_references.component_to_repository`: alle 30 Katalog-Komponenten auf ihre Kanonischen Repos verlinkt.

**Neu (F-001-Build-Grundlage):** `ATC-DEPENDENCY-GRAPH-001.json` (26 Knoten, 22 Interface-gekapselte Kanten, topologische Build-Ordnung, Zyklenprüfung fail-closed) und `ATC-BUILD-MANIFEST-001.json` (26 Repos: kanonische Build-/Testbefehle, 6 FREEZE-Repos mit Monorepo-Pfad, 2 BLOCKED explizit) — beide als Draft-PR auf a-townchain-ecosystem, abgeleitet aus der atc-standards-Registry-SSOT, keine zweite Wahrheitsquelle. YAML-Verwandte: system-readiness.yaml (PR #117), MIGRATION_MANIFEST.yaml. Änderungen an einem Artefakt erfordern einen Update dieses Index (Sync-Abhängigkeiten im JSON dokumentiert).
