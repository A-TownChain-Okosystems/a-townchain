# 🏛️ Architektur — atc-blockchain

> **Repo:** [atc-blockchain](https://github.com/A-TownChain-Okosystems/atc-blockchain)
> **Layer:** L3-L4 | **Titel:** Blockchain Core
> **Stand:** 2026-08-06 | **Version:** v1.0.0

---

## Übersicht

Blockchain Core: Consensus (PoH+PoA), Block Production, Mempool, Validators, Smart Contracts.

## Komponenten

### ATCLang Module (.atc)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `consensus/fork_atc85.atc` | 74 | Fork Atc85 |
| `consensus/fork_resolution.atc` | 145 | Fork Resolution |
| `consensus/gas_fee.atc` | 130 | Gas Fee |
| `consensus/gas_fee_atc86.atc` | 71 | Gas Fee Atc86 |
| `consensus/hybrid_atc84.atc` | 98 | Hybrid Atc84 |
| `consensus/hybrid_consensus.atc` | 357 | Hybrid Consensus |
| `consensus/poh.atc` | 140 | Poh |
| `consensus/poh_atc83.atc` | 79 | Poh Atc83 |
| `consensus/pos.atc` | 164 | Pos |
| `consensus/pos_atc82.atc` | 92 | Pos Atc82 |
| `consensus/pow.atc` | 107 | Pow |
| `consensus/pow_atc81.atc` | 89 | Pow Atc81 |
| `contract_registry.atc` | 98 | Contract Registry |
| `contracts/atc001/genesis_token.atc` | 102 | Genesis Token |
| `contracts/contract_engine_atc14.atc` | 309 | Contract Engine Atc14 |
| `contracts/governance/governance_contract.atc` | 202 | Governance Contract |
| `contracts/shivamon/breeding.atc` | 139 | Breeding |
| `dex/amm.atc` | 277 | Amm |
| `governance/dao.atc` | 168 | Dao |
| `governance/dao_live.atc` | 235 | Dao Live |
| `governance/timelock.atc` | 150 | Timelock |
| `governance/treasury.atc` | 220 | Treasury |
| `mainnet/launch_manager.atc` | 105 | Launch Manager |
| `mainnet/mainnet_config.atc` | 151 | Mainnet Config |
| `network/atc-02_liquid_state_migration_failover.atc` | 58 | Atc-02 Liquid State Migration Failover |

*+18 weitere*

### Python Module (.py)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `consensus/poh.py` | 67 | Poh |
| `contracts/atc001/genesis_token.py` | 74 | Genesis Token |
| `contracts/atc8300/atc8300_token.py` | 126 | Atc8300 Token |
| `contracts/base/base_contract.py` | 87 | Base Contract |
| `nodes/bootstrap.py` | 257 | Bootstrap |
| `nodes/discovery.py` | 314 | Discovery |
| `nodes/p2p_propagation.py` | 381 | P2P Propagation |
| `smart_contract_registry.py` | 53 | Smart Contract Registry |
| `smart_contracts.py` | 716 | Smart Contracts |
| `wallet/did.py` | 74 | Did |
| `wallet/ecdsa.py` | 72 | Ecdsa |
| `wallet/multisig.py` | 107 | Multisig |

## Statistik

| Metrik | Wert |
|--------|------|
| .atc | 43 |
| .py | 12 |
| .rs | 0 |
| .ts | 0 |
| Total Zeilen | 8,891 |

---

*Auto-generiert 2026-08-06 · Aurora*
