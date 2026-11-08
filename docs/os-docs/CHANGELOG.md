# Changelog — A-TownChain OS / KAI-OS

> **Version:** 1.0.0 | **Dokumentations-Refresh:** 2026-10-09 | **Lizenz:** Apache-2.0

## [Unreleased] — 2026-10-09 — Ecosystem documentation reality refresh

- Replaced stale repository-count and auto-generated status claims with a dated 33 / 26 / 7 inventory snapshot and explicit evidence limits.
- Added a consolidated architecture/SSOT reference for standards, ATCLang, canonical ATC-VM, algorithm/consensus, ShivaCore, GlobusOS, Aurora AI, integration and orchestration.
- Reclassified old AI, consensus, compiler, kernel and cluster pages as historical where they contained unsupported current implementation or production claims.
- Added exact-PR-head CI observations and coverage limits; red and pending gates remain visible.
- Updated component documentation proposals for canonical VM/algorithm ownership, historical audit labels, stale test/status metrics and development readiness.
- No implementation, normative standard, CI gate, branch protection or release policy was changed by this documentation refresh.


---

## [1.0.0] — 2026-08-04 — ShivaCore Kernel 712/712 Tests + Parser-Fixes

### ShivaCore Kernel (Sprint 2.4 → 90%)
- ✅ **712/712 Tests GRÜN** (Rust, no_std, 0 warnings, 0 errors)
- ✅ 30 Kernel-Module mit Tests (712 total): gossip_bridge(45), genesis_bridge(40), genesis(38), mempool(36), security_audit(34), security(34), atcnet(32), vfs(31), memory_manager(31), tcpip(30), p2p(30), consensus(27), net(26), ai(23), syscall(22), ipc(22), vm(21), atcfs(21), timer(19), knowledge_graph(18), remote_caps(16), cross_subsystem(16), block(16), did(15), blockchain(15), kernel_init(14), contract(12), scheduler(10), process(10), capability(8)
- ✅ lib.rs: cfg_attr(not(test), no_std) — Kernel ist no_std, Tests nutzen std
- ✅ Bugfixes: FNV hash overflow, VM Store opcode, contract deploy, p2p msg parsing, Ed25519 unique keys, timer periodic intervals, capability check_any, consensus DAG, unban score reset
- **Commit:** `d3cb52e` in `atc-shivacore`

### ATCLang Parser (Sprint 2.1 → 98%)
- ✅ f-String-Lexer-Fix gepusht (Commit `595d731`) — `f"...{expr}..."` via Lexer-Desugaring
- 🔄 12 Python-Indent-Dialekt-Dateien (modules/assets/) — Transpiler in Arbeit
- 🔄 7 Einzelfehler kategorisiert: if-let, unit type, &mut, map/cast, tuple generics

### Sprint-Status (aktualisiert)
- 2.1: 98% | 2.2: 100% ✅ | 2.3: 95% | 2.4: 90% | 2.5: 100% ✅ | 2.6: 85% | 3.0: 20%

## [1.0.0] — 2026-07-05 — ATCLang Migration Complete + Sprint Updates

### ATCLang Migration
- ✅ **92 .atc Dateien, 15.936 Zeilen ATCLang** (0 Python-Stubs in Production)
- ✅ Compiler-Infrastruktur: Lexer, Parser, AST, TypeChecker, Compiler, Optimizer, VM, REPL (19 Python-Module)
- ✅ Stdlib: 10 Module (crypto, collections, io, math, encoding, primitives, string, wallet, chain, stdlib)
- ✅ v0.3 Features: async/await, generics, closures, module system
- ✅ 60 Tests GRÜN, 92/92 Parse OK

### Sprint-Abschlüsse
- **Sprint 2.2: 100% ✅** — P2P, Discovery, Gossip, NAT, Bootstrap, Testnet (9 Module)
- **Sprint 2.5: 100% ✅** — Base, Atcoin, Bridge, Wallet, Token, Registry, Smart Contracts, Marketplace, Shivamon (10 Module)
- **Sprint 2.3: 90%** — Hybrid Consensus v0.3 (356L), PoH (139L), PoS (163L), PoW (106L), Fork (144L), Gas (129L)
- **Sprint 2.4: 90%** — Kernel, IPC, ATCFS, Net, Process, Shell (295L), Pkg Manager (207L), ECDSA, Keygen, Wordlist
- **Sprint 2.6: 80%** — DAO (234L), Treasury (219L), Timelock (149L), Multisig v0.3 (267L), Governance x2
- **Sprint 3.0: 95%** — Server, Routes x3, DB x2, Wallet, Gateway x5, Monitor x2, CLI, Pkg, Start, BigQuery (20 Module)
- **Sprint 3.2: 55%** — AI Kernel, FL, Franchise x2, HF Pipeline, Biometric, Mobile, Renderer (8 Module)

### Cleanup
- ✅ 10 alte v0.1 Duplikate entfernt (blockchain/contracts/, blockchain/wallet/)
- ✅ 11 alte atclang/programs/ Demos entfernt (außer atcos_main.atc)
- ✅ 8 alte STUB-Dateien entfernt (dao, marketplace, bridge, dex)
- ✅ Alle Solidity-Dateien entfernt (0 .sol)
- ✅ Alle Python-Stubs migriert (0 .py in Production)
- ✅ 5 leere Verzeichnisse bereinigt
- ✅ Genesis Token auf v0.3 upgegradet

### Offen
- 21 .atc Dateien noch mit v0.1 Syntax (require, caller) — v0.3 Migration pending
- CI/CD Pipeline Fix (#79) — Branch-Protection blockiert
- AD-002 EventBus vs IPCBus — Michael-Entscheidung
- AD-005 ATC-97 Protocol — Spezifikation finalisieren

---

## [1.0.0-rc2] — 2026-06-14 — Non-EVM + Cleanup

- ATCLang v0.3.0: async/await, Generics, Closures, Modul-System, Stdlib
- ShivaOS Kernel v1.0: Syscalls, IPC Bus, ATCFS, AI Kernel
- P2P: Bootstrap, Discovery, Gossip, Block Propagation
- 69 Wiki-Kapitel, 99 Standards, Audit 94/100
- EVM-Abhängigkeiten vollständig entfernt
- Chain-ID 9000, SHA-256, Non-EVM

---

## [1.0.0-rc1] — 2026-06-10

- Solana Bridge, DEX/AMM, DAO Governance
- Mobile Wallet, Block Explorer
- Enterprise CI/CD, Prometheus+Grafana, Nginx TLS

---

## [1.0.0-beta] — 2026-06-09

- Smart Contracts Python Stubs, ECDSA Wallet, P2P Bootstrap
- 11 Issues geschlossen

---

## [0.9.0-alpha] — 2026-01 bis 2026-05

- Whitepaper v0.9, 13-Layer-Architektur, ATCLang v0.2
- Hybrid Consensus Konzept, Projektstruktur

---

*Changelog — Aurora (MasterBrain · Base44) · v1.0.0 · 05.07.2026*
