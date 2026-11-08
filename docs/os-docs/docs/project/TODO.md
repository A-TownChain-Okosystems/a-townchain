# A-TownChain — TODO List

> Generated: 2026-09-06 (Update)
> Sorted by priority

## 🔴 Critical — Mainnet Block (Launch-Termin offen, AD-023)

- [ ] **Issue #69: Security Audit** — 69 Dependabot-Schwachstellen (3 critical, 27 high) bis 15.09. beheben
- [ ] **K30: Validator Node Setup** — Deploy 10+ Mainnet validator nodes (Issue #70)
  - [ ] Validator configuration generator
  - [ ] Genesis distribution to validators
  - [ ] P2P connectivity between all validators
  - [ ] Proposer rotation verification
  - [ ] Chain synchronization test

- [ ] **K31: Genesis Block Deployment** — Live genesis block (Issue #71)
  - [ ] Final GenesisConfig (validators, allocations, chain params)
  - [ ] Genesis block generation and signing
  - [ ] Distribution to all validator nodes
  - [ ] Chain-ID 658467 activation

- [ ] **K32: Pre-Launch Verification**
  - [ ] Full test suite run (367 tests)
  - [ ] Security audit re-run (all categories pass)
  - [ ] Attack-vector simulations (all 5 blocked)
  - [ ] Multi-node chain convergence test
  - [ ] Performance benchmark

- [ ] **K33: External Security Audit** — Third-party sign-off (Issue #69)
  - [ ] Audit report export
  - [ ] External review
  - [ ] Remediation if needed

## 🟡 High — Consolidation

- [ ] **K3: Python Backend Migration** — 7/12 subtasks done
  - [ ] K3.2: External repo migration
  - [ ] K3.4: Test consolidation
  - [ ] K3.5: API endpoint migration
  - [ ] K3.6: Database layer migration
  - [ ] K3.10: Complete setup.py

- [ ] **K4: Frontend Consolidation** — 0/10 subtasks
  - [ ] Create package.json, vite.config, tsconfig
  - [ ] Migrate UI components to /frontend/src/
  - [ ] API Gateway integration (port 4000)
  - [ ] Neon/dark theme implementation
  - [ ] ATC token integration

- [ ] **K5-K6: Pipeline Consolidation** — Not started
  - [ ] CI/CD pipeline setup
  - [ ] Automated testing pipeline
  - [ ] Docker build pipeline

- [ ] **K8: Release v1.0** — Not started
  - [ ] Version tagging
  - [ ] Release notes
  - [ ] Binary builds
  - [ ] Documentation finalization

## 🟢 Medium — Enhancement

- [ ] Docker multi-node orchestration
- [ ] Prometheus/Grafana monitoring stack
- [ ] API Gateway — 47 endpoints
- [ ] Block Explorer — Web UI
- [ ] Governance — DAO voting system
- [ ] Mobile App — iOS/Android
- [ ] DEX — Decentralized exchange
- [ ] Cross-Chain Bridge — Ethereum/Solana

## 📋 Backlog

- [ ] ZKP privacy features
- [ ] ATCLang v0.2.0 (improved compiler)
- [ ] Genesis Engine v2 (game engine)
- [ ] KAI AI Kernel integration
- [ ] ShivaMon game launch
- [ ] GlobusOS license dashboard
- [ ] Social platform features
- [ ] IDE browser integration

## Recently Completed

- [x] K29: Security Audit — 7 categories, 30+ checks, 5 attack simulations (Aug 4)
- [x] K28: P2P Gossip Integration — block-gossip, sync, vote-gossip (Aug 4)
- [x] K27: Genesis Bridge — 6 integration gaps closed (Aug 4)
- [x] K26: Genesis Block Configuration — Chain-ID 658467 (Aug 4)
- [x] K25: Type-Mismatch Fix — unified Pid (Aug 4)
- [x] K24: ATCNet Protocol — 10 message types (Aug 4)
- [x] K23: Cross-Subsystem Tests — 15 flows (Aug 4)
- [x] K22: Unified Kernel Init — boot sequence (Aug 4)
- [x] K21: Heap-Bridge Integration (Aug 4)

---

## September 2026 — Konsolidierung & Architektur-Freeze (Update 06.09.2026)

**Erreicht seit 06.08.:**
- **128→2 Konsolidierung OFFIZIELL** (03.09.): Alle 126 Alt-Repos archiviert;
  kanonisch: `a-townchain-os` (Code-Monorepo) + `a-townchain-os-docs` (Docs-Hub).
  Volldokumentation: 2.237 Dateien auditiert, 0 Fehler, 0 Datenverlust
  (Content-Hash-Verifikation), 575/575 Hub-Links grün, 93/93 Monorepo-Links grün.
- **Unified Cargo Workspace** (04.09.): 19 Rust-Crates in EINEM Build-System,
  Workspace-Tests **731/731 grün**, Kernel **674/674 auf STABLE**.
  Modul-Registry: 60 Module in `src/modules/registry.py`.
- **Chain-ID 658467** systemweit migriert (04.09., AD-004 RESOLVED; GitHub-Metadaten
  verifiziert). Bewusst unverändert: Ports, ATC-9000-NFT. BIP44-Coin-Type seit AD-042 (07.09.): m/44'/658467'.
- **Konsistenz-Audit** (05.09., Commit 6a162da): CodeQL-Workflow repariert
  (Auto-Übersetzung hatte YAML-Keywords eingedeutscht — lief nie), READMEs auf
  Konsolidierungs-Stand.
- **ShivaCore-Architektur VERBINDLICH** (06.09.): AD-012 Microkernel-Spezifikation
  + AD-013 Architektur-Gate v1.1 (SC-ARCH-001…010 Freeze-Regeln,
  SC-001…013-Reihenfolge, Scheduling Domains, Syscall-ABI, Boot Chain).
- **SC-001 Kernel Object Model** — Spezifikations-ENTWURF v0.1.0-draft1
  (29 MUST-Regeln, 11 Objekttypen, Testvektoren). Status DRAFT_REVIEW —
  wartet auf Owner-Entscheidungen SC-DEC-A…F.

**Offen bis Mainnet — Launch-Termin offen (AD-023):**
- 🔴 Issue #69: Dependabot — 69 Schwachstellen (3 critical, 27 high, 32 moderate, 7 low)
- 🔴 Issue #70 (K30): 10+ Mainnet-Validator-Nodes deployen
- 🔴 Issue #71 (K31): Genesis Block finalisieren (Chain-ID 658467)
- ⏳ SC-001-Freeze (nach SC-DEC-A…F) → SC-002 Capability Model
- ⏳ Service-Space-Migration der Blockchain-Module aus dem Kernel-Crate (eigener Sprint)
