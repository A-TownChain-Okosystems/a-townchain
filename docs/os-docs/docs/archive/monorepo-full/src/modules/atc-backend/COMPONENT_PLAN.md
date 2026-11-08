# 📋 Komponenten-Plan — atc-backend

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** `atc-backend`
**Name:** ATC Backend — API-Server
**Beschreibung:** Backend-API-Server. REST-Endpunkte für Wallet, Blockchain, Governance, Marketplace, Game, Explorer, Nodes, AI. Datenbank-Repository, Connection-Management, Migration.
**Layer:** L4 — Backend
**Sprint:** 3.0
**ATC-Standards:** ATC-24
**Komponenten:** 12

---

## Komponenten-Liste

| # | Datei | Zeilen | Typ | Beschreibung |
|---|-------|--------|-----|-------------|
| 1 | `api/game_routes.py` | 59 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 2 | `api/kai_routes.atc` | 229 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 3 | `api/orchestrator/orchestrator.atc` | 259 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 4 | `api/orchestrator/orchestrator.py` | 130 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 5 | `api/routes.py` | 67 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 6 | `api/routes/ai_routes.atc` | 175 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 7 | `api/routes/api_routes.atc` | 232 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 8 | `api/server.atc` | 68 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 9 | `db/connection.atc` | 125 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 10 | `db/connection.py` | 40 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 11 | `db/repository.atc` | 228 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 12 | `db/repository.py` | 196 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |

---

## Detaillierte Komponenten

### 1. `api/game_routes.py`

**Datei:** `api/game_routes.py`
**Zeilen:** 59
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** health, contract_stats, mint, get_token, owner_tokens, transfer, battle, battle_log (+1 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 2. `api/kai_routes.atc`

**Datei:** `api/kai_routes.atc`
**Zeilen:** 229
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct KAgent, struct AgentTask, struct StorageItem, init, list_agents, create_agent, invoke_agent, get_task_status (+13 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 3. `api/orchestrator/orchestrator.atc`

**Datei:** `api/orchestrator/orchestrator.atc`
**Zeilen:** 259
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct Task, struct ServiceEndpoint, init, start, register_service, is_available, create_task, dispatch (+7 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 4. `api/orchestrator/orchestrator.py`

**Datei:** `api/orchestrator/orchestrator.py`
**Zeilen:** 130
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, register_fn, start, stop, _worker_loop, _process, self._handlers[handler_names[0]], dispatch (+2 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 5. `api/routes.py`

**Datei:** `api/routes.py`
**Zeilen:** 67
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** list_franchises, create, get_franchise, join, stats

**Status:** 🟢 IMPLEMENTIERT

---

### 6. `api/routes/ai_routes.atc`

**Datei:** `api/routes/ai_routes.atc`
**Zeilen:** 175
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct AIQuery, struct AIModelInfo, init, register_model, health, query, stream, get_query_result (+3 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 7. `api/routes/api_routes.atc`

**Datei:** `api/routes/api_routes.atc`
**Zeilen:** 232
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** api_wallet_health, api_wallet_create, api_wallet_restore, api_wallet_balance, api_wallet_send, api_blockchain_health, api_blockchain_info, api_blockchain_blocks (+40 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 8. `api/server.atc`

**Datei:** `api/server.atc`
**Zeilen:** 68
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** init, status, modules, register_module, register_blueprints, start, stop

**Status:** 🟢 IMPLEMENTIERT

---

### 9. `db/connection.atc`

**Datei:** `db/connection.atc`
**Zeilen:** 125
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct DBConfig, struct Migration, init, connect, disconnect, is_connected, apply_migration, run_default_migrations (+4 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 10. `db/connection.py`

**Datei:** `db/connection.py`
**Zeilen:** 40
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** get_connection, _init_schema, close

**Status:** 🔄 STUB

---

### 11. `db/repository.atc`

**Datei:** `db/repository.atc`
**Zeilen:** 228
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct WalletRecord, struct ShivamonRecord, struct TxRecord, struct BlockRecord, init, save_wallet, find_wallet, update_balance (+11 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 12. `db/repository.py`

**Datei:** `db/repository.py`
**Zeilen:** 196
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** save, find, update_balance, list_all, count, save, find, find_by_owner (+10 weitere)

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
