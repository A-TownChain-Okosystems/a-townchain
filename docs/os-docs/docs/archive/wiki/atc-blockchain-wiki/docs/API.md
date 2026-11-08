# 🔌 API Reference — atc-blockchain

> **Repo:** [atc-blockchain](https://github.com/A-TownChain-Okosystems/atc-blockchain)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `init()` | — | `smart_contract_registry.atc` | ATCLang |
| 2 | `deploy()` | Address | `smart_contract_registry.atc` | ATCLang |
| 3 | `get()` | — | `smart_contract_registry.atc` | ATCLang |
| 4 | `list_all()` | List | `smart_contract_registry.atc` | ATCLang |
| 5 | `call()` | bool | `smart_contract_registry.atc` | ATCLang |
| 6 | `set_paused()` | bool | `smart_contract_registry.atc` | ATCLang |
| 7 | `get_deploy_count()` | u32 | `smart_contract_registry.atc` | ATCLang |
| 8 | `init()` | — | `smart_contracts.atc` | ATCLang |
| 9 | `create_auction()` | Hash | `smart_contracts.atc` | ATCLang |
| 10 | `place_bid()` | bool | `smart_contracts.atc` | ATCLang |
| 11 | `finalize_auction()` | — | `smart_contracts.atc` | ATCLang |
| 12 | `cancel_auction()` | bool | `smart_contracts.atc` | ATCLang |
| 13 | `get_auction()` | — | `smart_contracts.atc` | ATCLang |
| 14 | `init()` | — | `smart_contracts.atc` | ATCLang |
| 15 | `register_agent()` | Hash | `smart_contracts.atc` | ATCLang |
| 16 | `get_agent()` | — | `smart_contracts.atc` | ATCLang |
| 17 | `verify_capability()` | bool | `smart_contracts.atc` | ATCLang |
| 18 | `update_reputation()` | bool | `smart_contracts.atc` | ATCLang |
| 19 | `deactivate_agent()` | bool | `smart_contracts.atc` | ATCLang |
| 20 | `init()` | — | `smart_contracts.atc` | ATCLang |
| 21 | `start_round()` | Hash | `smart_contracts.atc` | ATCLang |
| 22 | `submit_update()` | bool | `smart_contracts.atc` | ATCLang |
| 23 | `aggregate_updates()` | Hash | `smart_contracts.atc` | ATCLang |
| 24 | `get_round_info()` | — | `smart_contracts.atc` | ATCLang |
| 25 | `init()` | — | `smart_contracts.atc` | ATCLang |
| 26 | `create_proposal()` | Hash | `smart_contracts.atc` | ATCLang |
| 27 | `cast_vote()` | bool | `smart_contracts.atc` | ATCLang |
| 28 | `finalize_vote()` | bool | `smart_contracts.atc` | ATCLang |
| 29 | `execute_proposal()` | bool | `smart_contracts.atc` | ATCLang |
| 30 | `get_proposal()` | — | `smart_contracts.atc` | ATCLang |
| 31 | `init()` | — | `smart_contracts.atc` | ATCLang |
| 32 | `open_channel()` | Hash | `smart_contracts.atc` | ATCLang |
| 33 | `send_payment()` | bool | `smart_contracts.atc` | ATCLang |
| 34 | `close_channel()` | bool | `smart_contracts.atc` | ATCLang |
| 35 | `get_channel()` | — | `smart_contracts.atc` | ATCLang |
| 36 | `init()` | — | `contract_registry.atc` | ATCLang |
| 37 | `deploy()` | DeployLog | `contract_registry.atc` | ATCLang |
| 38 | `get()` | ContractInfo | `contract_registry.atc` | ATCLang |
| 39 | `activate()` | bool | `contract_registry.atc` | ATCLang |
| 40 | `deactivate()` | bool | `contract_registry.atc` | ATCLang |
| 41 | `count()` | u128 | `contract_registry.atc` | ATCLang |
| 42 | `list_all()` | List | `contract_registry.atc` | ATCLang |
| 43 | `get_amount_out()` | u128 | `dex/amm.atc` | ATCLang |
| 44 | `create_pool()` | Hash | `dex/amm.atc` | ATCLang |
| 45 | `swap_a_to_b()` | SwapResult | `dex/amm.atc` | ATCLang |
| 46 | `swap_b_to_a()` | SwapResult | `dex/amm.atc` | ATCLang |
| 47 | `add_liquidity()` | LiquidityResult | `dex/amm.atc` | ATCLang |
| 48 | `remove_liquidity()` | RemoveLiquidityResult | `dex/amm.atc` | ATCLang |
| 49 | `get_quote()` | QuoteResult | `dex/amm.atc` | ATCLang |
| 50 | `get_tvl()` | LiquidityPool | `dex/amm.atc` | ATCLang |
| 51 | `sqrt_u128()` | u128 | `dex/amm.atc` | ATCLang |
| 52 | `init()` | bool | `network/atc-10_global_time_sync_oracles.atc` | ATCLang |
| 53 | `on_message()` | bool | `network/atc-10_global_time_sync_oracles.atc` | ATCLang |
| 54 | `verify()` | bool | `network/atc-10_global_time_sync_oracles.atc` | ATCLang |
| 55 | `get_state()` | GlobalTimeSyncOraclesState | `network/atc-10_global_time_sync_oracles.atc` | ATCLang |
| 56 | `get_shard_for_address()` | UInt16 | `network/sharding_atc07.atc` | ATCLang |
| 57 | `init_shards()` | ShardingState | `network/sharding_atc07.atc` | ATCLang |
| 58 | `route_tx()` | RouteResult | `network/sharding_atc07.atc` | ATCLang |
| 59 | `lock_cross_shard()` | Bool | `network/sharding_atc07.atc` | ATCLang |
| 60 | `verify_cross_shard()` | Bool | `network/sharding_atc07.atc` | ATCLang |

*+476 weitere*

**Total: 536 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
