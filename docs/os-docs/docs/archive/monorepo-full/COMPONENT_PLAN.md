# 📋 Komponenten-Plan — a-townchain-os

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** `a-townchain-os`
**Name:** A-TownChain OS — Monorepo
**Beschreibung:** Zentraler Monorepo für das A-TownChain OS. Enthält ATCLang-Module für Blockchain, Kernel, Gateway, Backend, Tools und Core-Komponenten, die noch nicht in modulare Repos ausgelagert wurden.
**Layer:** L0–L12 (alle)
**Sprint:** 2.1–3.2
**ATC-Standards:** ATC-01 bis ATC-99
**Komponenten:** 26

---

## Komponenten-Liste

| # | Datei | Zeilen | Typ | Beschreibung |
|---|-------|--------|-----|-------------|
| 1 | `archive/duplicates/kai_cli.atc` | 195 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 2 | `battle_engine.atc` (archiviert: a-townchain-os-docs → docs/monorepo-legacy/src-legacy/modules/shivamon/engine/) | 153 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 3 | `src/blockchain/contract_registry.atc` | 6 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 4 | `src/blockchain/smart_contract_registry.atc` | 6 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 5 | `src/blockchain/smart_contract_registry.py` | 53 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 6 | `src/blockchain/smart_contracts.atc` | 6 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 7 | `src/contracts/atc8300_token.py` | 126 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 8 | `src/contracts/governance_contract.py` | 299 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 9 | `src/contracts/keygen.py` | 140 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 10 | `src/contracts/marketplace_contract.py` | 301 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 11 | `src/contracts/wallet_ecdsa.py` | 72 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 12 | `src/contracts/wallet_keygen.py` | 140 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 13 | `src/core/atcfs.py` | 122 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 14 | `src/core/event_bus.py` | 16 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 15 | `src/core/kai_cli.atc` | 6 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 16 | `src/core/kernel/api.py` | 882 | .py | — |
| 17 | `src/core/kernel/capabilities.py` | 159 | .py | — |
| 18 | `src/core/kernel/did.py` | 74 | .py | — |
| 19 | `src/core/kernel/kernel.py` | 423 | .py | — |
| 20 | `src/core/kernel/remote_capability.py` | 207 | .py | — |
| 21 | `src/core/kernel/syscalls.atc` | 118 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 22 | `src/core/module_loader.py` | 17 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 23 | `src/game/game_routes.py` | 59 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 24 | `src/game/marketplace_routes.py` | 93 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 25 | `src/gateway/main.py` | 47 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 26 | `start.atc` | 129 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |

---

## Detaillierte Komponenten

### 1. `archive/duplicates/kai_cli.atc`

**Datei:** `archive/duplicates/kai_cli.atc`
**Zeilen:** 195
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct CLICommand, struct CLIResult, init, execute, cmd_status, cmd_wallet, cmd_mine, cmd_send (+6 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 2. `battle_engine.atc` (archiviert: a-townchain-os-docs → docs/monorepo-legacy/src-legacy/modules/shivamon/engine/)

**Datei:** `battle_engine.atc` (archiviert: a-townchain-os-docs → docs/monorepo-legacy/src-legacy/modules/shivamon/engine/)
**Zeilen:** 153
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct BattleStats, struct BattleState, init, start_battle, execute_attack, get_battle, get_wins, get_losses (+2 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 3. `src/blockchain/contract_registry.atc`

**Datei:** `src/blockchain/contract_registry.atc`
**Zeilen:** 6
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** —

**Status:** 🔄 STUB

---

### 4. `src/blockchain/smart_contract_registry.atc`

**Datei:** `src/blockchain/smart_contract_registry.atc`
**Zeilen:** 6
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** —

**Status:** 🔄 STUB

---

### 5. `src/blockchain/smart_contract_registry.py`

**Datei:** `src/blockchain/smart_contract_registry.py`
**Zeilen:** 53
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, deploy, get, list_all, call, getattr, get_deploy_log

**Status:** 🟢 IMPLEMENTIERT

---

### 6. `src/blockchain/smart_contracts.atc`

**Datei:** `src/blockchain/smart_contracts.atc`
**Zeilen:** 6
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** —

**Status:** 🔄 STUB

---

### 7. `src/contracts/atc8300_token.py`

**Datei:** `src/contracts/atc8300_token.py`
**Zeilen:** 126
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, name, total_supply, balance_of, mint, _mint, burn, transfer (+5 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 8. `src/contracts/governance_contract.py`

**Datei:** `src/contracts/governance_contract.py`
**Zeilen:** 299
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** to_dict, __init__, set_balance_oracle, _get_voting_power, create_proposal, vote, finalize_proposal, execute_proposal (+5 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 9. `src/contracts/keygen.py`

**Datei:** `src/contracts/keygen.py`
**Zeilen:** 140
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** generate_entropy, entropy_to_mnemonic, mnemonic_to_seed, seed_to_private_key, private_to_public_key, public_key_to_address, generate_wallet, restore_from_mnemonic (+1 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 10. `src/contracts/marketplace_contract.py`

**Datei:** `src/contracts/marketplace_contract.py`
**Zeilen:** 301
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** to_dict, __init__, set_token_contract, set_balance_oracle, list_for_sale, buy, cancel_listing, get_listings (+5 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 11. `src/contracts/wallet_ecdsa.py`

**Datei:** `src/contracts/wallet_ecdsa.py`
**Zeilen:** 72
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** generate_keypair, sign, verify, build_tx

**Status:** 🟢 IMPLEMENTIERT

---

### 12. `src/contracts/wallet_keygen.py`

**Datei:** `src/contracts/wallet_keygen.py`
**Zeilen:** 140
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** generate_entropy, entropy_to_mnemonic, mnemonic_to_seed, seed_to_private_key, private_to_public_key, public_key_to_address, generate_wallet, restore_from_mnemonic (+1 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 13. `src/core/atcfs.py`

**Datei:** `src/core/atcfs.py`
**Zeilen:** 122
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** atc_content_id, __init__, _init_root, _parent, _mkdir, exists, _check_read, write (+3 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 14. `src/core/event_bus.py`

**Datei:** `src/core/event_bus.py`
**Zeilen:** 16
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, subscribe, emit

**Status:** 🔄 STUB

---

### 15. `src/core/kai_cli.atc`

**Datei:** `src/core/kai_cli.atc`
**Zeilen:** 6
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** —

**Status:** 🔄 STUB

---

### 16. `src/core/kernel/api.py`

**Datei:** `src/core/kernel/api.py`
**Zeilen:** 882
**Typ:** .py
**Beschreibung:** —
**Funktionen/Structs:** __init__, sys_spawn, sys_kill, sys_wait, sys_sleep, sys_wake, sys_process_list, sys_process_info (+44 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 17. `src/core/kernel/capabilities.py`

**Datei:** `src/core/kernel/capabilities.py`
**Zeilen:** 159
**Typ:** .py
**Beschreibung:** —
**Funktionen/Structs:** has, __init__, grant, check, require, delegate, revoke, _find_children (+1 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 18. `src/core/kernel/did.py`

**Datei:** `src/core/kernel/did.py`
**Zeilen:** 74
**Typ:** .py
**Beschreibung:** —
**Funktionen/Structs:** __str__, __init__, generate, sign, public_key_bytes, verify

**Status:** 🟢 IMPLEMENTIERT

---

### 19. `src/core/kernel/kernel.py`

**Datei:** `src/core/kernel/kernel.py`
**Zeilen:** 423
**Typ:** .py
**Beschreibung:** —
**Funktionen/Structs:** read, write, send, recv, peek, __init__, _boot, _spawn_system (+25 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 20. `src/core/kernel/remote_capability.py`

**Datei:** `src/core/kernel/remote_capability.py`
**Zeilen:** 207
**Typ:** .py
**Beschreibung:** —
**Funktionen/Structs:** signing_payload, issue_ticket, consume_operation, __init__, check_and_record, __init__, resolve, resolve_chain

**Status:** 🟢 IMPLEMENTIERT

---

### 21. `src/core/kernel/syscalls.atc`

**Datei:** `src/core/kernel/syscalls.atc`
**Zeilen:** 118
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, register, dispatch, enable, disable, list_syscalls

**Status:** 🟢 IMPLEMENTIERT

---

### 22. `src/core/module_loader.py`

**Datei:** `src/core/module_loader.py`
**Zeilen:** 17
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, load

**Status:** 🔄 STUB

---

### 23. `src/game/game_routes.py`

**Datei:** `src/game/game_routes.py`
**Zeilen:** 59
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** health, contract_stats, mint, get_token, owner_tokens, transfer, battle, battle_log (+1 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 24. `src/game/marketplace_routes.py`

**Datei:** `src/game/marketplace_routes.py`
**Zeilen:** 93
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** health, listings, get_listing, token_listing, list_nft, buy, cancel, sales_history (+1 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 25. `src/gateway/main.py`

**Datei:** `src/gateway/main.py`
**Zeilen:** 47
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** before, health, proxy

**Status:** 🔄 STUB

---

### 26. `start.atc`

**Datei:** `start.atc`
**Zeilen:** 129
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct ServiceState, init, start_all, start_service, stop_service, stop_all, get_service_status, get_running_count (+2 weitere)

**Status:** 🟢 IMPLEMENTIERT

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
