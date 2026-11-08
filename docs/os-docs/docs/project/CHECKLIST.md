# A-TownChain — Mainnet Launch Checklist

> Target: Launch-Termin offen (AD-023, 06.09.2026)
> Last Updated: 2026-09-06
> Kernel: ShivaCore K29 | 30 Module | 674/674 Kernel-Tests ✅ · 731/731 Workspace-Tests ✅ (STABLE)

---

## Phase 1: Kernel Development (ABGESCHLOSSEN ✅)

| Sprint | Modul | Tests | Issue | Status |
|--------|-------|-------|-------|--------|
| K3 | capability.rs | 8 | — | ✅ |
| K3b | process.rs | 10 | — | ✅ |
| K4 | scheduler.rs | 10 | — | ✅ |
| K5 | ipc.rs | 12 | — | ✅ |
| K6 | did.rs + remote_caps.rs | 21 | — | ✅ |
| K6b | Ed25519 Signatures | 15 | — | ✅ |
| K7 | knowledge_graph.rs | 18 | — | ✅ |
| K8 | memory_manager.rs + atcfs.rs | 34 | — | ✅ |
| K21 | Heap-Bridge Integration | 28 | — | ✅ |
| K22 | kernel_init.rs | 11 | — | ✅ |
| K23 | cross_subsystem.rs | 15 | — | ✅ |
| K24 | atcnet.rs | 32 | — | ✅ |
| K25 | Type-Mismatch Fix | — | atc-shivacore#1 ✅ | ✅ |
| K26 | genesis.rs | 38 | #71 (open) | ✅ |
| K27 | genesis_bridge.rs | 40 | — | ✅ |
| K28 | gossip_bridge.rs | 45 | atc-shivacore#2 ✅ | ✅ |
| K29 | security_audit.rs | 34 | atc-shivacore#3 ✅, #69 (open) | ✅ |

**Gesamt: 367/367 Tests | 30 Module | 100% Pass Rate**

---

## Phase 2: Mainnet Preparation (AKTUELL 🔵)

### 🔴 Critical — Mainnet Block

- [ ] **K30: Validator Node Setup** — Issue #70 (Sprint 4.0)
  - [ ] Validator-Konfigurations-Generator (Rust)
  - [ ] Genesis-Distribution an 10+ Validator-Nodes
  - [ ] P2P-Konnektivität zwischen allen Validatoren (ATCNet)
  - [ ] Proposer-Rotation verifizieren (stake-weighted, PoH-seeded)
  - [ ] Chain-Synchronisationstest (Multi-Node)
  - [ ] BFT-Quorum-Test (66.7%+ signieren)
  - [ ] Validator-Key-Backup-Prozedur dokumentieren
  - **Ziel**: 10. August 2026
  - **Geschätzte Tests**: ~40

- [ ] **K31: Genesis Block Deployment** — Issue #71 (Sprint 4.0)
  - [ ] Finale GenesisConfig (Validatoren, Allokationen, Chain-Params)
  - [ ] Genesis-Block Generierung + Signierung (Chain-ID 658467)
  - [ ] Distribution an alle Validator-Nodes
  - [ ] Genesis-Hash-Verifikation auf allen Nodes
  - [ ] Chain-ID 658467 Aktivierung
  - **Ziel**: 20. August 2026
  - **Geschätzte Tests**: ~20

- [ ] **K32: Pre-Launch Verification**
  - [ ] Vollständiger Test-Suite-Run (367 cargo + pytest)
  - [ ] Security-Audit Re-Run (alle 7 Kategorien → PASS)
  - [ ] Attack-Vector-Simulationen (alle 5 → blocked)
  - [ ] Multi-Node Chain-Convergence Test (3+ Nodes)
  - [ ] Performance-Benchmark (TPS, Latenz, Block-Zeit)
  - [ ] Docker Multi-Node Orchestration verifizieren
  - **Ziel**: 1. September 2026

- [ ] **K33: External Security Audit** — Issue #69 (Sprint 3.3)
  - [ ] Audit-Report Export (security_audit.rs → PDF/Markdown)
  - [ ] Externe Code-Review (Third-Party)
  - [ ] Remediation falls erforderlich
  - [ ] Sign-off dokumentieren
  - **Ziel**: 8. September 2026

### 🟡 High — Konsolidierung

- [ ] **K3: Python Backend Migration** — Issue #87
  - [x] K3.1: core/ → src/core/ (6 files)
  - [x] K3.7: shivaos/kernel/ → src/shivaos/kernel/ (5 files)
  - [x] K3.8: blockchain/ → src/blockchain/ (6 files)
  - [x] K3.9: gateway/ → src/gateway/ (3 files)
  - [x] K3.11: atclang/ → src/atclang/ (14 files)
  - [ ] K3.2: Externe Repos migrieren (atc-blockchain, atc-contracts, atc-vm)
  - [ ] K3.4: Test-Konsolidierung (pytest → src/tests/)
  - [ ] K3.5: API-Endpunkt-Migration (→ Gateway port 4000)
  - [ ] K3.6: Database-Layer-Migration
  - [ ] K3.10: setup.py finalisieren
  - **Status**: 7/12 Subtasks ✅

- [ ] **K4: Frontend Konsolidierung** — Issue #88
  - [ ] package.json, vite.config, tsconfig erstellen
  - [ ] UI-Komponenten nach frontend/src/ migrieren
  - [ ] API-Gateway Integration (port 4000)
  - [ ] Neon/Dark Theme Implementierung
  - [ ] ATC-Token-Integration (ATC-8300/9000)
  - **Status**: 0/10 Subtasks ⬜

- [ ] **K5-K6: Pipeline Konsolidierung**
  - [ ] CI/CD Pipeline (GitHub Actions)
  - [ ] Automatisierte Tests (cargo + pytest)
  - [ ] Docker Build Pipeline
  - [ ] Release-Automatisierung
  - **Status**: Nicht begonnen ⬜

### 🟢 Open Issues (aus GitHub)

| Issue | Repo | Sprint | Title | Priority |
|-------|------|--------|-------|----------|
| #69 | a-townchain-os | 3.3 | Security-Audit (extern) | High |
| #70 | a-townchain-os | 4.0 | Validator-Nodes (10+) | Medium |
| #71 | a-townchain-os | 4.0 | Genesis Block (Chain-ID 658467) | Medium |
| #80 | a-townchain-os | 3.0 | AIP-001 Agent Interaction Protocol | High |
| #93 | a-townchain-os | — | Sync-Integration: 4 Warnungen | Bug |

---

## Phase 3: Pre-Launch Verification

| # | Check | Status | Verifiziert durch |
|---|-------|--------|------------------|
| 1 | cargo tests (367/367) | ✅ | `cargo test` |
| 2 | Chain-ID = 658467 | ✅ | security_audit.rs CHAIN-002 |
| 3 | Genesis block signed | ✅ | security_audit.rs GEN-001 |
| 4 | Validators ≥ 4, ≤ 100 | ✅ | security_audit.rs GEN-005 |
| 5 | BFT threshold = 66.7% | ✅ | security_audit.rs VAL-004 |
| 6 | No single validator > 33% | ✅ | security_audit.rs VAL-005 |
| 7 | PoH seeded with genesis hash | ✅ | security_audit.rs POH-001 |
| 8 | Chain forgery blocked | ✅ | simulate_chain_forgery() |
| 9 | Genesis replay blocked | ✅ | simulate_genesis_replay() |
| 10 | Height skip blocked | ✅ | simulate_height_skip() |
| 11 | Orphan block blocked | ✅ | simulate_orphan_block() |
| 12 | Unsigned genesis blocked | ✅ | simulate_unsigned_genesis() |
| 13 | MAX_MESSAGE_SIZE enforced | ✅ | security_audit.rs NET-003 |
| 14 | Protocol version = 1 | ✅ | security_audit.rs NET-002 |
| 15 | Parent-hash linkage | ✅ | security_audit.rs CHAIN-004 |
| 16 | No duplicate block IDs | ✅ | security_audit.rs CHAIN-005 |
| 17 | pytest tests pass | ⬜ | `pytest` |
| 18 | Docker multi-node up | ⬜ | `docker-compose up` |
| 19 | Prometheus monitoring active | ⬜ | `curl :9090` |
| 20 | API Gateway (47 endpoints) | ⬜ | `curl :4000/health` |
| 21 | Block explorer online | ⬜ | Browser check |
| 22 | External audit sign-off | ⬜ | Third-party report |

---

## Phase 4: Launch Day (Launch-Termin offen, AD-023)

- [ ] Genesis-Block auf allen Validator-Nodes deployed
- [ ] Alle Validatoren via ATCNet P2P verbunden
- [ ] Erster Block vorgeschlagen + gegossiped
- [ ] Chain-Höhe advancing (Block-Zeit ~400ms)
- [ ] Monitoring-Dashboards aktiv (Prometheus/Grafana)
- [ ] Public API live (port 4000)
- [ ] Block Explorer online
- [ ] Status-Page live
- [ ] Validator-Key-Backups verifiziert
- [ ] Emergency-Prozedur dokumentiert

---

## Post-Launch (Q4 2026)

- [ ] Cross-Chain Bridge (Ethereum/Solana) — ATC-09
- [ ] ZKP Privacy Features — ATC-08
- [ ] DEX — Decentralized Exchange
- [ ] Mobile App (iOS/Android)
- [ ] Governance — DAO Voting
- [ ] AIP-001 Agent Interaction Protocol — Issue #80

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
