# Kapitel 75 — v0.1 → v0.3 ATCLang Migration Plan

> **Stand:** 05.07.2026 | **Autor:** Aurora (MasterBrain · Base44)

---

## Übersicht

21 .atc Dateien verwenden noch die veraltete v0.1 Syntax. Diese müssen auf v0.3 migriert werden.

## v0.1 vs v0.3 Syntax-Unterschiede

| Feature | v0.1 | v0.3 |
|---------|------|------|
| Caller | `caller` | `msg_sender()` |
| Assertions | `require(cond, "msg")` | `if !cond { return false }` |
| Imports | `import ATC::Crypto` | Built-in `Hash::`, `Address::` |
| Integer | `Int` | `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64` |
| Boolean | `Bool` | `bool` |
| List | `List = []` | `List<T> = []` |
| Map | `Map = {}` | `Map<K,V> = {}` |
| Init | `fn init() { }` | `fn init() { }` (same) |
| Events | `event Name(Type)` | `event Name(Type)` (same) |

## Betroffene Dateien (21)

### blockchain/ (10 Dateien)
| Datei | Zeilen | Sprint | Modul |
|-------|--------|--------|-------|
| contract_registry.atc | 97 | 2.5 | Smart Contract Registry |
| contracts/shivamon/breeding.atc | 138 | 2.5 | Shivamon Breeding |
| dex/amm.atc | 276 | 2.3 | AMM DEX |
| governance/dao.atc | 167 | 2.6 | DAO Governance |
| mainnet/launch_manager.atc | 104 | 4.0 | Mainnet Launch |
| mainnet/mainnet_config.atc | 150 | 4.0 | Mainnet Config |
| nodes/block_propagation.atc | 86 | 2.2 | Block Propagation |
| nodes/node.atc | 191 | 2.2 | Core Node |
| wallet/did.atc | 121 | 2.4 | Decentralized Identity |
| zkp/groth16.atc | 88 | 3.2 | ZKP Groth16 |

### modules/ (8 Dateien)
| Datei | Zeilen | Sprint | Modul |
|-------|--------|--------|-------|
| contracts/marketplace/marketplace_contract.atc | 235 | 2.5 | NFT Marketplace |
| franchise/contracts/registry.atc | 119 | 3.2 | Franchise Registry |
| franchise/contracts/revenue.atc | 92 | 3.2 | Franchise Revenue |
| franchise/contracts/token.atc | 71 | 3.2 | Franchise Token |
| kernel/fs/atcfs.atc | 141 | 2.4 | ATCFS |
| kernel/ipc/ipc_bus.atc | 101 | 2.4 | IPC Bus |
| kernel/net/atcnet.atc | 134 | 2.4 | Network Stack |
| kernel/process/process_mgr.atc | 160 | 2.4 | Process Manager |

### shivaos/ (2 Dateien)
| Datei | Zeilen | Sprint | Modul |
|-------|--------|--------|-------|
| fs/atcfs_module.atc | 125 | 2.4 | ATCFS Module |
| kernel/syscalls.atc | 117 | 2.4 | System Calls |

### modules/shivamon/ (1 Datei)
| Datei | Zeilen | Sprint | Modul |
|-------|--------|--------|-------|
| engine/battle_engine.atc | 0 | 2.5 | Battle Engine (empty) |

## Migrations-Strategie

1. **Priorität 1 (Sprint 2.4):** kernel/ Module (4 Dateien) — blockiert AD-002
2. **Priorität 2 (Sprint 2.2):** node.atc, block_propagation.atc — Core Node
3. **Priorität 3 (Sprint 2.5):** marketplace_contract, breeding — Smart Contracts
4. **Priorität 4 (Sprint 2.6):** dao.atc — Governance
5. **Priorität 5 (Sprint 3.2):** franchise, groth16, battle_engine — AI/Future

---

*Kapitel 75 · v0.1→v0.3 Migration Plan · 05.07.2026 · Aurora*
