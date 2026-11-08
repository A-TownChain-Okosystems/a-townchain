# 🔌 API Reference — atc-backend

> **Repo:** [atc-backend](https://github.com/A-TownChain-Okosystems/atc-backend)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `init()` | — | `db/repository.atc` | ATCLang |
| 2 | `save_wallet()` | bool | `db/repository.atc` | ATCLang |
| 3 | `find_wallet()` | — | `db/repository.atc` | ATCLang |
| 4 | `update_balance()` | bool | `db/repository.atc` | ATCLang |
| 5 | `list_wallets()` | u32 | `db/repository.atc` | ATCLang |
| 6 | `save_shivamon()` | bool | `db/repository.atc` | ATCLang |
| 7 | `find_shivamon()` | — | `db/repository.atc` | ATCLang |
| 8 | `update_shivamon_owner()` | bool | `db/repository.atc` | ATCLang |
| 9 | `save_tx()` | bool | `db/repository.atc` | ATCLang |
| 10 | `find_tx()` | — | `db/repository.atc` | ATCLang |
| 11 | `confirm_tx()` | bool | `db/repository.atc` | ATCLang |
| 12 | `save_block()` | bool | `db/repository.atc` | ATCLang |
| 13 | `find_block()` | — | `db/repository.atc` | ATCLang |
| 14 | `get_chain_height()` | u64 | `db/repository.atc` | ATCLang |
| 15 | `get_stats()` | — | `db/repository.atc` | ATCLang |
| 16 | `init()` | — | `db/connection.atc` | ATCLang |
| 17 | `connect()` | bool | `db/connection.atc` | ATCLang |
| 18 | `disconnect()` | bool | `db/connection.atc` | ATCLang |
| 19 | `is_connected()` | bool | `db/connection.atc` | ATCLang |
| 20 | `apply_migration()` | bool | `db/connection.atc` | ATCLang |
| 21 | `run_default_migrations()` | bool | `db/connection.atc` | ATCLang |
| 22 | `get_version()` | u16 | `db/connection.atc` | ATCLang |
| 23 | `get_migration_count()` | u16 | `db/connection.atc` | ATCLang |
| 24 | `get_config()` | — | `db/connection.atc` | ATCLang |
| 25 | `get_stats()` | — | `db/connection.atc` | ATCLang |
| 26 | `init()` | — | `api/server.atc` | ATCLang |
| 27 | `status()` | — | `api/server.atc` | ATCLang |
| 28 | `modules()` | List | `api/server.atc` | ATCLang |
| 29 | `register_module()` | — | `api/server.atc` | ATCLang |
| 30 | `register_blueprints()` | — | `api/server.atc` | ATCLang |
| 31 | `start()` | — | `api/server.atc` | ATCLang |
| 32 | `stop()` | — | `api/server.atc` | ATCLang |
| 33 | `init()` | — | `api/kai_routes.atc` | ATCLang |
| 34 | `list_agents()` | List | `api/kai_routes.atc` | ATCLang |
| 35 | `create_agent()` | Hash | `api/kai_routes.atc` | ATCLang |
| 36 | `invoke_agent()` | Hash | `api/kai_routes.atc` | ATCLang |
| 37 | `get_task_status()` | — | `api/kai_routes.atc` | ATCLang |
| 38 | `complete_task()` | bool | `api/kai_routes.atc` | ATCLang |
| 39 | `delete_agent()` | bool | `api/kai_routes.atc` | ATCLang |
| 40 | `get_agent()` | — | `api/kai_routes.atc` | ATCLang |
| 41 | `upload_storage()` | Hash | `api/kai_routes.atc` | ATCLang |
| 42 | `download_storage()` | — | `api/kai_routes.atc` | ATCLang |
| 43 | `get_storage_info()` | — | `api/kai_routes.atc` | ATCLang |
| 44 | `get_chain_status()` | — | `api/kai_routes.atc` | ATCLang |
| 45 | `get_balance()` | u128 | `api/kai_routes.atc` | ATCLang |
| 46 | `transfer_tokens()` | bool | `api/kai_routes.atc` | ATCLang |
| 47 | `list_proposals()` | List | `api/kai_routes.atc` | ATCLang |
| 48 | `vote_proposal()` | bool | `api/kai_routes.atc` | ATCLang |
| 49 | `system_status()` | — | `api/kai_routes.atc` | ATCLang |
| 50 | `system_info()` | — | `api/kai_routes.atc` | ATCLang |
| 51 | `init()` | — | `api/orchestrator/orchestrator.atc` | ATCLang |
| 52 | `start()` | — | `api/orchestrator/orchestrator.atc` | ATCLang |
| 53 | `register_service()` | bool | `api/orchestrator/orchestrator.atc` | ATCLang |
| 54 | `is_available()` | bool | `api/orchestrator/orchestrator.atc` | ATCLang |
| 55 | `create_task()` | Hash | `api/orchestrator/orchestrator.atc` | ATCLang |
| 56 | `dispatch()` | — | `api/orchestrator/orchestrator.atc` | ATCLang |
| 57 | `complete_task()` | bool | `api/orchestrator/orchestrator.atc` | ATCLang |
| 58 | `fail_task()` | bool | `api/orchestrator/orchestrator.atc` | ATCLang |
| 59 | `record_error()` | bool | `api/orchestrator/orchestrator.atc` | ATCLang |
| 60 | `reset_circuit()` | bool | `api/orchestrator/orchestrator.atc` | ATCLang |

*+103 weitere*

**Total: 163 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
