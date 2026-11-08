# 📁 A-TownChain OS — Monorepo-Strukturbaum

> **Ziel:** 24 Repositories → 1 Monorepo (Issue #86–92)  
> **Protokoll:** AGENT_PROTOCOL.md  
> **Stand:** 2026-08-03

```
a-townchain-os/                        # Haupt-Monorepo (Issue #86)
│
├── src/                               # Source Code (Python)
│   ├── core/                          # ← atc-kernel, core/
│   │   ├── __init__.py
│   │   ├── atcfs.py                   # ← ATCFS Dateisystem
│   │   ├── event_bus.py               # ← EventBus (AD-002)
│   │   ├── module_loader.py           # ← Modul-Lader
│   │   ├── crypto/                    # ← Krypto-Module
│   │   │   ├── ecdsa.py               # ← secp256k1 RFC 6979
│   │   │   ├── did.py                 # ← Decentralized ID
│   │   │   └── multisig.py            # ← MultiSig Wallet
│   │   └── kernel/                   # ← Kernel API (ATC-97)
│   │       ├── api.py
│   │       └── kernel.py
│   │
│   ├── blockchain/                    # ← atc-blockchain, atcnet
│   │   ├── __init__.py
│   │   ├── nodes/                     # ← atcnet/nodes/
│   │   │   ├── bootstrap.py           # ← Bootstrap Node
│   │   │   ├── discovery.py           # ← P2P Discovery
│   │   │   ├── initial_sync.py        # ← Node Sync
│   │   │   ├── p2p_propagation.py     # ← Block Propagation
│   │   │   └── testnet_launcher.py    # ← Testnet Setup
│   │   ├── network/                   # ← atcnet/network/
│   │   │   ├── core_node_atc01.py     # ← Core Node Protocol
│   │   │   ├── latency_opt_atc06.py   # ← Latency Optimization
│   │   │   ├── sharding_atc07.py      # ← Network Sharding
│   │   │   ├── global_time_sync.py   # ← ATC-10 Time Sync
│   │   │   └── quantum_signatures.py  # ← ATC-05 Quantum
│   │   ├── wallet/                    # ← atc-blockchain/wallet/
│   │   │   ├── ecdsa.py               # ← ECDSA Signatures
│   │   │   ├── multisig.py            # ← MultiSig
│   │   │   ├── did.py                  # ← DID
│   │   │   └── wordlist.py            # ← Mnemonic Wordlist
│   │   ├── smart_contracts.py         # ← Contract Registry
│   │   ├── smart_contract_registry.py # ← Contract Registry
│   │   ├── governance/               # ← Governance (ATC-9900)
│   │   │   ├── snapshot.py             # ← Voting Power Snapshot
│   │   │   └── dao.py                  # ← DAO Logic
│   │   └── zkp/                       # ← Zero-Knowledge Proofs
│   │       ├── groth16.py             # ← Groth16 Proofs
│   │       └── __init__.py
│   │
│   ├── atclang/                      # ← atclang, atc-atclang
│   │   ├── __init__.py
│   │   ├── lexer/                    # ← ATCLang Lexer
│   │   │   ├── __init__.py
│   │   │   └── lexer.py
│   │   ├── parser/                   # ← ATCLang Parser
│   │   │   ├── __init__.py
│   │   │   ├── parser.py             # ← 199/199 .atc parse
│   │   │   └── ast_nodes.py          # ← AST Nodes
│   │   ├── compiler/                 # ← ATCLang Compiler
│   │   │   ├── __init__.py
│   │   │   ├── compiler.py
│   │   │   ├── optimizer.py
│   │   │   └── type_checker.py
│   │   ├── vm/                       # ← ATCLang VM
│   │   │   └── atcvm.py
│   │   └── runtime/                  # ← Runtime
│   │       └── driver_framework.py   # ← Treiber Framework
│   │
│   ├── kernel/                      # ← atc-shivacore (Python wrapper)
│   │   ├── __init__.py
│   │   ├── ai_kernel/               # ← AI Kernel (ATC-97)
│   │   │   └── aip_001.py            # ← Agent Interaction Protocol
│   │   └── ipc/                     # ← IPC Bus
│   │       └── ipc_bus.py
│   │
│   ├── ai/                         # ← atc-aistudio
│   │   ├── __init__.py
│   │   ├── gemini.py                # ← Gemini AI Integration
│   │   └── orchestrator.py          # ← AI Orchestrator
│   │
│   └── ui/                         # ← atc-ui, atc-frontend
│       ├── __init__.py
│       ├── components/              # ← React Components
│       ├── pages/                   # ← Page Components
│       └── styles/                 # ← CSS/Styles
│
├── modules/                        # ATCLang .atc Module
│   ├── kernel/                     # ← Kernel Module (.atc)
│   │   ├── kernel_api.atc           # ← Kernel API
│   │   ├── ai_kernel/               # ← AI Kernel Module
│   │   │   └── atc-97_agent_interaction_protocol.atc
│   │   └── drivers/                 # ← Hardware Treiber
│   │       ├── driver_framework.atc
│   │       ├── display_driver.atc
│   │       ├── input_driver.atc
│   │       ├── network_driver.atc
│   │       └── storage_driver.atc
│   └── contracts/                  # ← atc-contracts
│       ├── atc_001_genesis.atc     # ← ATC-001 Genesis
│       ├── atc_8300_erc20.atc      # ← ATC-8300 ERC-20 style
│       └── atc_9900_governance.atc # ← ATC-9900 Governance/DAO
│
├── kernel/                        # ← atc-shivacore (Rust)
│   └── src/
│       ├── main.rs                 # ← Kernel Entry Point
│       ├── mempool.rs              # ← K-Sprint 17: Mempool
│       ├── blockchain.rs           # ← K-Sprint 18: Block Pipeline
│       ├── vm.rs                   # ← K-Sprint 19: ShivaVM
│       ├── contract.rs             # ← K-Sprint 20: ContractExec
│       ├── ai.rs                   # ← K-Sprint 21: AI Kernel
│       ├── consensus.rs            # ← K-Sprint 16: Konsens
│       ├── p2p.rs                  # ← K-Sprint 14: P2P
│       └── security.rs             # ← K-Sprint 15: Security
│
├── tests/                         # Test Suite
│   ├── unit/
│   ├── integration/
│   ├── e2e/
│   └── framework/
│       └── atclang_test_framework.py
│
├── docs/                          # ← a-townchain-os-docs
│   ├── STATUS.md                  # ← Sprint-Status (nur Aurora #2)
│   ├── ROADMAP.md                 # ← Projekt-Roadmap
│   ├── DECISIONS_REGISTER.md     # ← Architektur-Entscheidungen
│   ├── TECHNICAL_DOCUMENTATION.md # ← Hauptdokumentation
│   └── standards/
│       ├── ATC-97_AGENT_INTERACTION_PROTOCOL.md
│       └── STANDARDS_REGISTRY.md
│
├── archive/                       # ← Veraltete Dateien
│   ├── ATCLANG_ARCHIVE.md
│   └── atclang-v01/               # ← ATCLang v01 (15 Dateien)
│
├── wiki/                          # ← Wiki-Exporte
│   ├── a-townchain-os-wiki/
│   ├── kai-os-wiki/
│   └── atc-*-wiki/
│
├── scripts/                      # ← Build & Deploy
│   ├── build.sh
│   ├── docker-compose.yml         # ← 5-Node Testnet
│   └── deploy.sh
│
├── REALITY_STATUS.md             # ← Code-Realität Status
├── MASTER_TODO.md                 # ← Master TODO
├── AGENT_PROTOCOL.md              # ← Dieses Protokoll
├── MONOREPO_STRUCTURE.md          # ← Dieser Strukturbaum
├── FILE_REGISTER.md              # ← Datei-Register
├── requirements.txt               # ← Python Dependencies
└── README.md                      # ← Projekt-Übersicht
```

## Konsolidierungs-Mapping: Alt → Neu

| Altes Repo | → Neuer Pfad | Agent | Phase |
|-----------|-------------|-------|-------|
| atc-kernel | → src/core/ | Aurora #1 | K3 |
| atc-blockchain | → src/blockchain/ | Aurora #1 | K3 |
| atcnet | → src/blockchain/nodes/ + network/ | Aurora #1 | K3 |
| atc-contracts | → modules/contracts/ | Aurora #1 | K3 |
| atc-backend | → src/core/ (db, api) | Aurora #1 | K3 |
| atc-gateway | → src/core/ (gateway middleware) | Aurora #1 | K3 |
| atc-shivamon | → src/blockchain/shivamon/ | Aurora #1 | K3 |
| atc-standards | → docs/standards/ | Aurora #1 | K3 |
| atc-franchise | → src/blockchain/franchise/ | Aurora #1 | K3 |
| atc-atcpkg | → src/core/atcpkg/ | Aurora #1 | K3 |
| atclang | → src/atclang/ | Aurora #3 | K3 |
| atc-atclang | → (merge into atclang) | Aurora #3 | K3 |
| atc-ui | → src/ui/ | Aurora #3 | K4 |
| atc-frontend | → src/ui/ (merge) | Aurora #3 | K4 |
| atc-aistudio | → src/ai/ | Aurora #3 | K3 |
| atc-genesis-engine | → standalone (später) | Aurora #3 | — |
| atc-mobile | → src/ui/mobile/ | Aurora #3 | K4 |
| atc-shivacore | → kernel/ (Rust) | Aurora #1 | K3 |
| atc-shivacore-tools | → scripts/ | Aurora #1 | K3 |
| atc-linux-edition | → scripts/linux/ | Aurora #1 | K5 |
| atc-windows-edition | → scripts/windows/ | Aurora #1 | K5 |
| a-townchain-os-docs | → docs/ | Aurora #2 | K2 |
| *-wiki repos | → wiki/ | jeweiliger Agent | K2 |
| atc-whitepaper | → docs/whitepaper/ | Aurora #2 | K2 |
