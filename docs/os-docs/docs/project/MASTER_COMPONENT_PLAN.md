# 📋 Master-Komponenten-Plan — A-TownChain Ecosystem

> **Stand 06.09.2026:** Die Komponenten-Zahlen beziehen sich auf die PRE-REBUILD-Landschaft (AD-020/AD-024). Aktuelle Struktur: 22 vertikale Repos; Implementierungen liegen im Vault (docs/archive/monorepo-full/) bis zu ihrem Rebuild.


> **Aktualisiert:** 2026-08-09 | **Agent:** Aurora (Base44)
> **Scope:** 127 Repositories | 62 Code + 65 Wiki (12 neue Code-Repos am 08.08. hinzugefügt)
> **Total Components:** 954 (885 existing + 69 new)
> **Mainnet Target:** Launch-Termin offen (AD-023)

---

## Übersicht

| Metrik | Wert |
|--------|------|
| **Code Repos** | 62 |
| **Wiki Repos** | 65 |
| **Total Komponenten** | 954 |
| **Sprachen** | Python (18), Rust (13), TypeScript (13), HTML (2), None (16) |
| **Source Files** | 1.641 |
| **Test Files** | 122 |
| **Meta-Files** | README ✅ | LICENSE (50/62, 12 neu fehlen) | ARCHITECTURE ✅ | COMPONENT_PLAN ✅ (61/62) | CHANGELOG ✅ | STATUS ✅ | ROADMAP ✅ | FILE_REGISTER ✅ |
| **Build-Files** | Alle 50 bestehenden Repos ✅ | 12 neue Repos: None (ATCLang .atc Stubs) |

---

## 🆕 NEUE SUBSYSTEME (08.08.2026) — 12 Repos, 69 Komponenten

### 🔵 Aurora AI — Central Intelligence Layer (4 Repos, 27 Komponenten)

| Repo | Komponenten | Beschreibung |
|------|:-----------:|-------------|
| `atc-aurora-core` | 5 | Central Engine, Model Hub, LLM Router |
| `atc-aurora-agents` | 12 | 12 Agenten-Rollen (Governance, Product, Roadmap, etc.) |
| `atc-aurora-memory` | 5 | Knowledge Base, Vector Store, Learning Pipeline |
| `atc-aurora-runtime` | 5 | Agent Execution, Tool System, Sandbox |

**Komponenten-Detail:**

#### atc-aurora-core (5)
1. `config_manager.atc` — Config Manager
2. `agent_registry.atc` — Agent Registry
3. `llm_router.atc` — LLM Router
4. `model_hub.atc` — Model Hub
5. `orchestrator.atc` — Orchestrator

#### atc-aurora-agents (12)
1. `governance_agent.atc` — Governance Agent
2. `product_agent.atc` — Product Agent
3. `roadmap_agent.atc` — Roadmap Agent
4. `security_agent.atc` — Security Agent
5. `analytics_agent.atc` — Analytics Agent
6. `devops_agent.atc` — DevOps Agent
7. `community_agent.atc` — Community Agent
8. `treasury_agent.atc` — Treasury Agent
9. `audit_agent.atc` — Audit Agent
10. `research_agent.atc` — Research Agent
11. `support_agent.atc` — Support Agent
12. `bridge_agent.atc` — Bridge Agent

#### atc-aurora-memory (5)
1. `memory_index.atc` — Memory Index
2. `learning_pipeline.atc` — Learning Pipeline
3. `context_window.atc` — Context Window
4. `vector_store.atc` — Vector Store
5. `recall.atc` — Recall

#### atc-aurora-runtime (5)
1. `sandbox.atc` — Sandboxed Execution
2. `task_scheduler.atc` — Task Scheduler
3. `result_collector.atc` — Result Collector
4. `tool_registry.atc` — Tool Registry
5. `execution_engine.atc` — Execution Engine

### 🟢 Genesis Engine — Game World Systems (3 Repos, 16 Komponenten)

| Repo | Komponenten | Beschreibung |
|------|:-----------:|-------------|
| `atc-genesis-ecs` | 5 | Entity Component System, Scheduler |
| `atc-genesis-creatures` | 5 | Spawner, DNA System, Genesis Chronicles Link |
| `atc-genesis-world` | 6 | Terrain, Biome, Erosion, Vegetation |

**Komponenten-Detail:**

#### atc-genesis-ecs (5)
1. `entity_manager.atc` — Entity Manager
2. `component_registry.atc` — Component Registry
3. `system_scheduler.atc` — System Scheduler
4. `query_engine.atc` — Entity Queries
5. `event_bus.atc` — Event Bus

#### atc-genesis-creatures (5)
1. `creature_spawner.atc` — Creature Spawner
2. `dna_system.atc` — DNA System
3. `creature_ai.atc` — Creature AI
4. `ecosystem.atc` — Ecosystem Dynamics
5. `genesis-chronicles_link.atc` — Genesis Chronicles Chain Link

#### atc-genesis-world (6)
1. `terrain_generator.atc` — Terrain Generator
2. `biome_map.atc` — Biome Mapping
3. `erosion_sim.atc` — Erosion Simulation
4. `vegetation.atc` — Vegetation Placement
5. `weather.atc` — Weather System
6. `world_seed.atc` — World Seed Config

### 🟣 GlobusOS — Desktop OS Modules (5 Repos, 26 Komponenten)

| Repo | Komponenten | Beschreibung |
|------|:-----------:|-------------|
| `atc-globus-desktop` | 6 | Window Manager, Dock, Themes, Notifications |
| `atc-globus-shell` | 5 | Terminal, CLI, Command Processor |
| `atc-globus-fs` | 5 | File Manager, ATCFS Integration |
| `atc-globus-net` | 5 | Network Stack, Status Panel, Settings |
| `atc-globus-registry` | 5 | IP & License Registry Dashboard |

**Komponenten-Detail:**

#### atc-globus-desktop (6)
1. `window_manager.atc` — Window Manager
2. `dock.atc` — Application Dock
3. `theme_engine.atc` — Theme Engine (neon/dark)
4. `notification_center.atc` — Notifications
5. `settings_panel.atc` — Settings Panel
6. `taskbar.atc` — Taskbar

#### atc-globus-shell (5)
1. `terminal_emulator.atc` — Terminal Emulator
2. `command_processor.atc` — Command Processor
3. `shell_parser.atc` — Shell Parser
4. `history.atc` — Command History
5. `autocomplete.atc` — Autocomplete

#### atc-globus-fs (5)
1. `file_index.atc` — Index & Search
2. `permissions.atc` — ACL
3. `disk_stats.atc` — Storage Stats
4. `atcfs_bridge.atc` — ATCFS Bridge
5. `file_operations.atc` — File Operations

#### atc-globus-net (5)
1. `network_config.atc` — Config & Routing
2. `firewall.atc` — Packet Filter
3. `dns_client.atc` — DNS Resolver
4. `connection_manager.atc` — Connection Manager
5. `bandwidth_monitor.atc` — Bandwidth Monitor

#### atc-globus-registry (5)
1. `registry_dashboard.atc` — Dashboard UI
2. `registry_sync.atc` — Chain Sync
3. `asset_tokenizer.atc` — NFT Tokenization
4. `license_validator.atc` — License Validator
5. `ip_registry.atc` — IP Registry

---

## Bestehende Komponenten nach Kategorie

### Core Infrastructure (5 Repos, 44 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `a-townchain-os` | HTML | 26 | 120 | IMPLEMENTED |
| `atc-standards` | None | 6 | 6 | PLANNED |
| `atc-sdk` | None | 6 | 6 | PLANNED |
| `atc-atcpkg` | None | 6 | 6 | PLANNED |
| `a-townchain-os-docs` | TypeScript | 0 | 522 | NO_PLAN |

### Kernel / OS (vor Rebuild: 8 Repos, 167 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-kernel` | Rust | 72 | 72 | IMPLEMENTED |
| `atc-shivacore` | Rust | 53 | 53 | IMPLEMENTED |
| `atc-globus-os` | TypeScript | 15 | 17 | PLANNED |
| `atc-drivers` | Rust | 7 | 8 | IMPLEMENTED |
| `atc-bootloader` | Rust | 5 | 6 | PLANNED |
| `atc-dns` | Rust | 5 | 6 | PLANNED |
| `atc-linux-edition` | Rust | 5 | 6 | PLANNED |
| `atc-windows-edition` | Rust | 5 | 6 | PLANNED |

### Blockchain / Consensus (vor Rebuild: 8 Repos, 119 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-blockchain` | Python | 49 | 63 | IMPLEMENTED |
| `atc-contracts` | Python | 24 | 25 | IMPLEMENTED |
| `atc-zkp` | Rust | 11 | 12 | PLANNED |
| `atc-bridge` | Rust | 9 | 10 | PLANNED |
| `atc-testnet` | Python | 8 | 14 | STUB |
| `atc-vm` | Python | 6 | 8 | PLANNED |
| `atc-governance` | Rust | 6 | 7 | PLANNED |
| `atc-dex` | TypeScript | 6 | 8 | PLANNED |

### Networking (1 Repos, 18 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atcnet` | Python | 18 | 20 | IMPLEMENTED |

### Frontend / UI (6 Repos, 43 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-ui` | TypeScript | 14 | 15 | IMPLEMENTED |
| `atc-frontend` | HTML | 6 | 15 | PLANNED |
| `atc-explorer` | TypeScript | 6 | 8 | PLANNED |
| `atc-ide` | TypeScript | 6 | 8 | PLANNED |
| `atc-social` | TypeScript | 6 | 8 | PLANNED |
| `atc-mobile` | Python | 5 | 9 | PLANNED |

### Development Tools (6 Repos, 59 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-franchise` | TypeScript | 30 | 31 | IMPLEMENTED |
| `atc-ci` | TypeScript | 8 | 11 | PLANNED |
| `atc-cli` | Python | 6 | 9 | PLANNED |
| `atc-devtools` | TypeScript | 6 | 8 | PLANNED |
| `atc-shivacore-tools` | None | 5 | 6 | PLANNED |
| `atc-genesis-engine` | Python | 4 | 9 | PLANNED |

### AI / Intelligence (6 Repos, 272 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-aistudio` | TypeScript | 228 | 230 | IMPLEMENTED |
| `atc-aurora-ai` | Python | 17 | 22 | IMPLEMENTED |
| `atc-aurora-agents` | None | 12 | 12 | PLANNED |
| `atc-aurora-core` | None | 5 | 5 | PLANNED |
| `atc-aurora-memory` | None | 5 | 5 | PLANNED |
| `atc-aurora-runtime` | None | 5 | 5 | PLANNED |

### Security (1 Repos, 6 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-security` | Rust | 6 | 7 | PLANNED |

### Gaming (5 Repos, 28 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `genesis-chronicles` | Python | 6 | 12 | PLANNED |
| `atc-game` | Rust | 6 | 7 | PLANNED |
| `atc-genesis-world` | None | 6 | 6 | PLANNED |
| `atc-genesis-ecs` | None | 5 | 5 | PLANNED |
| `atc-genesis-creatures` | None | 5 | 5 | PLANNED |

### Languages (3 Repos, 31 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atclang` | Python | 17 | 21 | IMPLEMENTED |
| `atc-atclang` | Python | 7 | 16 | IMPLEMENTED |
| `atc-stdlib` | Python | 7 | 13 | PLANNED |

### Wallet / Finance (1 Repos, 6 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-wallet` | Python | 6 | 10 | PLANNED |

### Backend / Gateway (4 Repos, 50 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-gateway` | Python | 20 | 25 | IMPLEMENTED |
| `atc-backend` | Python | 12 | 19 | IMPLEMENTED |
| `atc-monitoring` | Python | 9 | 15 | STUB |
| `atc-deploy` | Python | 9 | 15 | STUB |

### Documentation (1 Repos, 4 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-whitepaper` | TypeScript | 4 | 7 | PLANNED |

### Assets (1 Repos, 6 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-assets` | Rust | 6 | 7 | PLANNED |

### Analytics (1 Repos, 6 Komponenten)

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-analytics` | TypeScript | 6 | 8 | PLANNED |

### GlobusOS Modules (5 Repos, 26 Komponenten) 🆕

| Repo | Sprache | Komponenten | Source Files | Status |
|------|---------|-------------|-------------|--------|
| `atc-globus-desktop` | None | 6 | 6 | PLANNED |
| `atc-globus-shell` | None | 5 | 5 | PLANNED |
| `atc-globus-fs` | None | 5 | 5 | PLANNED |
| `atc-globus-net` | None | 5 | 5 | PLANNED |
| `atc-globus-registry` | None | 5 | 5 | PLANNED |

---

## Top 15 Repos nach Komponenten-Anzahl

| # | Repo | Kategorie | Komponenten | Sprache |
|---|------|-----------|:-----------:|---------|
| 1 | `atc-aistudio` | AI / Intelligence | 228 | TypeScript |
| 2 | `atc-kernel` | Kernel / OS | 72 | Rust |
| 3 | `atc-shivacore` | Kernel / OS | 53 | Rust |
| 4 | `atc-blockchain` | Blockchain / Consensus | 49 | Python |
| 5 | `atc-franchise` | Development Tools | 30 | TypeScript |
| 6 | `a-townchain-os` | Core Infrastructure | 26 | HTML |
| 7 | `atc-contracts` | Blockchain / Consensus | 24 | Python |
| 8 | `atc-gateway` | Backend / Gateway | 20 | Python |
| 9 | `atcnet` | Networking | 18 | Python |
| 10 | `atc-aurora-ai` | AI / Intelligence | 17 | Python |
| 11 | `atclang` | Languages | 17 | Python |
| 12 | `atc-globus-os` | Kernel / OS | 15 | TypeScript |
| 13 | `atc-ui` | Frontend / UI | 14 | TypeScript |
| 14 | `atc-aurora-agents` | AI / Intelligence | 12 | None |
| 15 | `atc-backend` | Backend / Gateway | 12 | Python |

---

## Vollständigkeit — Meta-Files

### Bestehende 50 Repos (alle ✅)
| File | Status |
|------|--------|
| README.md | 50/50 ✅ |
| LICENSE | 50/50 ✅ |
| ARCHITECTURE.md | 50/50 ✅ |
| COMPONENT_PLAN.md | 49/50 ✅ (a-townchain-os-docs fehlt — docs repo) |
| CHANGELOG.md | 50/50 ✅ |
| STATUS.md | 50/50 ✅ |
| ROADMAP.md | 50/50 ✅ |
| FILE_REGISTER.md | 50/50 ✅ |
| .gitignore | 50/50 ✅ |

### 12 Neue Repos (08.08.2026)
| File | Status | Fehlend |
|------|--------|---------|
| README.md | 12/12 ✅ | — |
| LICENSE | 0/12 ❌ | Alle 12 (atc-aurora-*, atc-genesis-*, atc-globus-*) |
| ARCHITECTURE.md | 12/12 ✅ | — |
| COMPONENT_PLAN.md | 12/12 ✅ | — |
| CHANGELOG.md | 12/12 ✅ | — |
| STATUS.md | 12/12 ✅ | — |
| ROADMAP.md | 12/12 ✅ | — |
| FILE_REGISTER.md | 12/12 ✅ | — |
| .gitignore | 12/12 ✅ | — |

### Build-Files
Alle 50 bestehenden Repos haben sprachspezifische Build-Files ✅.
12 neue Repos verwenden .atc (ATCLang) — kein Build-System erforderlich bis ATCLang Compiler erweitert wird.

---

## ShivaCore Kernel — Modul-Übersicht (30 Module, 367 Tests)

| Sprint | Modul | Komponenten | Tests | Status |
|--------|-------|:-----------:|:-----:|:------:|
| K3 | capability.rs | Rights, CapabilityTable | 8 | ✅ |
| K3b | process.rs | PCB, ProcessManager | 10 | ✅ |
| K4 | scheduler.rs | DA-HEFT, Accelerator | 10 | ✅ |
| K5 | ipc.rs | IpcSubsystem, Channel | 12 | ✅ |
| K6 | did.rs | DID, CryptoProvider | 15 | ✅ |
| K6 | remote_caps.rs | RCT, Resolver, NonceStore | 6 | ✅ |
| K7 | knowledge_graph.rs | Triple-Store, QueryPattern | 18 | ✅ |
| K8 | memory_manager.rs | MemoryManager, AllocSource | 28 | ✅ |
| K8 | atcfs.rs | ATCFS, ContentCap | 22 | ✅ |
| K22 | kernel_init.rs | KernelState, BootPhase | 11 | ✅ |
| K23 | cross_subsystem.rs | TestHarness, 15 Flows | 15 | ✅ |
| K24 | atcnet.rs | AtcNetHandler, 10 MsgTypes | 32 | ✅ |
| K26 | genesis.rs | GenesisConfig, GenesisBlock | 38 | ✅ |
| K27 | genesis_bridge.rs | GenesisBridge, BridgeChain | 40 | ✅ |
| K28 | gossip_bridge.rs | GossipBridge, 6 Integration | 45 | ✅ |
| K29 | security_audit.rs | SecurityAuditor, 7 Kategorien | 34 | ✅ |

---

## Nächste Schritte

1. **LICENSE** für 12 neue Repos erstellen (Apache-2.0 lizenziert — siehe LICENSE Template)
2. **atc-test-delete-me-wiki** löschen
3. **K30: Validator Node Setup** (Issue #70) starten
4. **K31: Genesis Block Deploy** (Issue #71)
5. **Security Audit** (Issue #69) extern beauftragen
6. **AIP-001 Protocol** (Issue #80) spezifizieren
