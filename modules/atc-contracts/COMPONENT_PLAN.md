# 📋 Komponenten-Plan — atc-contracts

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** `atc-contracts`
**Name:** ATC Contracts — Smart Contracts
**Beschreibung:** Smart-Contract-Sammlung. ATCoin Token, ATC-8300 Fungible Token, Bridge, Governance Contract, Marketplace, Shivamon, ECDSA, Keygen, Base Contract, Payment Channel, Resource Market, Agent Registry, Federated Learning Coordinator.
**Layer:** L3 — Contracts
**Sprint:** 2.5
**ATC-Standards:** ATC-87, ATC-88, ATC-89, ATC-90, ATC-91
**Komponenten:** 24

---

## Komponenten-Liste

| # | Datei | Zeilen | Typ | Beschreibung |
|---|-------|--------|-----|-------------|
| 1 | `atc8300/atc8300.atc` | 96 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 2 | `atc8300/atc8300_token.atc` | 178 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 3 | `atc8300/atc8300_token.py` | 126 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 4 | `atcoin/atcoin.atc` | 176 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 5 | `atcoin/atcoin.py` | 139 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 6 | `base/base_contract.atc` | 69 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 7 | `base/base_contract.py` | 87 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 8 | `bridge/bridge_contract.atc` | 172 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 9 | `bridge/bridge_contract.py` | 133 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 10 | `contracts/atc001/genesis_token.atc` | 6 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 11 | `contracts/atc001/genesis_token.py` | 74 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 12 | `contracts/revenue.atc` | 93 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 13 | `contracts/solidity/test/ATCBridge.test.js` | 274 | .js | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 14 | `contracts/token.atc` | 72 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 15 | `governance/governance_contract.atc` | 237 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 16 | `governance/governance_contract.py` | 299 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 17 | `smart_contract_registry.atc` | 88 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 18 | `smart_contract_registry.py` | 53 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 19 | `smart_contracts.atc` | 486 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 20 | `smart_contracts.py` | 716 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 21 | `standards/atc-13_fractional_asset_ownership.atc` | 43 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 22 | `standards/atc-15_proof_of_ai_mining.atc` | 43 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 23 | `standards/atc-16_referral_multitier_rewards.atc` | 43 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 24 | `standards/atc-20_wrapped_synthetic_assets.atc` | 43 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |

---

## Detaillierte Komponenten

### 1. `atc8300/atc8300.atc`

**Datei:** `atc8300/atc8300.atc`
**Zeilen:** 96
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, name, symbol, decimals, total_supply, balance_of, transfer, approve (+6 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 2. `atc8300/atc8300_token.atc`

**Datei:** `atc8300/atc8300_token.atc`
**Zeilen:** 178
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct Snapshot, init, name, total_supply, balance_of, mint, burn, transfer (+7 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 3. `atc8300/atc8300_token.py`

**Datei:** `atc8300/atc8300_token.py`
**Zeilen:** 126
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, name, total_supply, balance_of, mint, _mint, burn, transfer (+5 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 4. `atcoin/atcoin.atc`

**Datei:** `atcoin/atcoin.atc`
**Zeilen:** 176
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct TxRecord, init, balance_of, transfer, approve, allowance, transfer_from, transfer_impl (+8 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 5. `atcoin/atcoin.py`

**Datei:** `atcoin/atcoin.py`
**Zeilen:** 139
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, balance_of, transfer, approve, allowance, transfer_from, mint, burn (+6 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 6. `base/base_contract.atc`

**Datei:** `base/base_contract.atc`
**Zeilen:** 69
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, only_owner, when_not_paused, pause, unpause, emit_event, get_event_count, info

**Status:** 🟢 IMPLEMENTIERT

---

### 7. `base/base_contract.py`

**Datei:** `base/base_contract.py`
**Zeilen:** 87
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, to_dict, __init__, _gen_address, only_owner, when_not_paused, pause, unpause (+4 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 8. `bridge/bridge_contract.atc`

**Datei:** `bridge/bridge_contract.atc`
**Zeilen:** 172
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct BridgeTx, init, lock, mint_on_target, burn_and_release, refund, get_bridge_tx, get_stats (+2 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 9. `bridge/bridge_contract.py`

**Datei:** `bridge/bridge_contract.py`
**Zeilen:** 133
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** to_dict, __init__, _tx_id, lock_and_bridge, confirm_mint, burn_and_release, get_tx, list_txs (+3 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 10. `contracts/atc001/genesis_token.atc`

**Datei:** `contracts/atc001/genesis_token.atc`
**Zeilen:** 6
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** —

**Status:** 🔄 STUB

---

### 11. `contracts/atc001/genesis_token.py`

**Datei:** `contracts/atc001/genesis_token.py`
**Zeilen:** 74
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, name, holder, supply, lock, transfer, provenance, verify

**Status:** 🟢 IMPLEMENTIERT

---

### 12. `contracts/revenue.atc`

**Datei:** `contracts/revenue.atc`
**Zeilen:** 93
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct RevenueEntry, init, record_revenue, payout_franchisor, payout_franchisee, get_stats

**Status:** 🟢 IMPLEMENTIERT

---

### 13. `contracts/solidity/test/ATCBridge.test.js`

**Datei:** `contracts/solidity/test/ATCBridge.test.js`
**Zeilen:** 274
**Typ:** .js
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** —

**Status:** 🟢 IMPLEMENTIERT

---

### 14. `contracts/token.atc`

**Datei:** `contracts/token.atc`
**Zeilen:** 72
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, name, symbol, decimals, total_supply, balance_of, transfer, mint (+3 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 15. `governance/governance_contract.atc`

**Datei:** `governance/governance_contract.atc`
**Zeilen:** 237
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct Proposal, init, set_balance_oracle, get_voting_power, create_proposal, vote, finalize_proposal, execute_proposal (+3 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 16. `governance/governance_contract.py`

**Datei:** `governance/governance_contract.py`
**Zeilen:** 299
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** to_dict, __init__, set_balance_oracle, _get_voting_power, create_proposal, vote, finalize_proposal, execute_proposal (+5 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 17. `smart_contract_registry.atc`

**Datei:** `smart_contract_registry.atc`
**Zeilen:** 88
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct ContractEntry, init, deploy, get, list_all, call, set_paused, get_deploy_count

**Status:** 🟢 IMPLEMENTIERT

---

### 18. `smart_contract_registry.py`

**Datei:** `smart_contract_registry.py`
**Zeilen:** 53
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, deploy, get, list_all, call, getattr, get_deploy_log

**Status:** 🟢 IMPLEMENTIERT

---

### 19. `smart_contracts.atc`

**Datei:** `smart_contracts.atc`
**Zeilen:** 486
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct ResourceAuction, struct AgentRecord, struct FLRound, struct GovernanceProposal, struct PaymentChannelState, init, create_auction, place_bid (+25 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 20. `smart_contracts.py`

**Datei:** `smart_contracts.py`
**Zeilen:** 716
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, transfer, __init__, create_auction, place_bid, finalize_auction, __post_init__, __init__ (+22 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 21. `standards/atc-13_fractional_asset_ownership.atc`

**Datei:** `standards/atc-13_fractional_asset_ownership.atc`
**Zeilen:** 43
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, transfer, register, balance_of

**Status:** 🔄 STUB

---

### 22. `standards/atc-15_proof_of_ai_mining.atc`

**Datei:** `standards/atc-15_proof_of_ai_mining.atc`
**Zeilen:** 43
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, transfer, register, balance_of

**Status:** 🔄 STUB

---

### 23. `standards/atc-16_referral_multitier_rewards.atc`

**Datei:** `standards/atc-16_referral_multitier_rewards.atc`
**Zeilen:** 43
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, transfer, register, balance_of

**Status:** 🔄 STUB

---

### 24. `standards/atc-20_wrapped_synthetic_assets.atc`

**Datei:** `standards/atc-20_wrapped_synthetic_assets.atc`
**Zeilen:** 43
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, transfer, register, balance_of

**Status:** 🔄 STUB

---

## Test-Strategie

1. Parse-Test: Jede .atc Datei muss mit ATCLang v0.3 Parser parsen
2. Unit-Tests: Mindestens 3 Tests pro Komponente
3. Integration-Test: Komponenten interagieren korrekt
4. Coverage-Ziel: >80%

## Dokumentations-Requirements

- ARCHITECTURE.md: Architektur-Baum + Komponenten-Übersicht ✅
- COMPONENT_PLAN.md: Dieser Plan ✅
- FILE_REGISTER.md: Datei-Liste ✅
- STATUS.md: Aktueller Status ✅
- ROADMAP.md: Sprint-Zuordnung ✅
- CHANGELOG.md: Änderungs-Historie ✅

---
*Auto-generiert 2026-08-06 · Aurora (MasterBrain · Base44)*
