# 🔌 API Reference — a-townchain-os

> **Repo:** [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `init()` | — | `start.atc` | ATCLang |
| 2 | `start_all()` | bool | `start.atc` | ATCLang |
| 3 | `start_service()` | bool | `start.atc` | ATCLang |
| 4 | `stop_service()` | bool | `start.atc` | ATCLang |
| 5 | `stop_all()` | bool | `start.atc` | ATCLang |
| 6 | `get_service_status()` | — | `start.atc` | ATCLang |
| 7 | `get_running_count()` | u8 | `start.atc` | ATCLang |
| 8 | `get_startup_time()` | u64 | `start.atc` | ATCLang |
| 9 | `health_check()` | bool | `start.atc` | ATCLang |
| 10 | `boot()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 11 | `stop()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 12 | `status()` | Map | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 13 | `name()` | String | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 14 | `boot()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 15 | `spawn_process()` | UInt32 | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 16 | `kill_process()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 17 | `open_channel()` | IPCChannel | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 18 | `alloc()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 19 | `syscall()` | String | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 20 | `stats()` | Map | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 21 | `status()` | Map | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 22 | `name()` | String | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 23 | `stop()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 24 | `boot()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 25 | `mkdir()` | INode | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 26 | `write()` | INode | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 27 | `read()` | Option | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 28 | `stats()` | Map | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 29 | `status()` | Map | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 30 | `name()` | String | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 31 | `stop()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 32 | `boot()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 33 | `create_genesis()` | ATCBlock | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 34 | `add_transaction()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 35 | `register_validator()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 36 | `tick_poh()` | Hash256 | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 37 | `select_validator()` | Address | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 38 | `mine_seal()` | UInt64 | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 39 | `produce_block()` | ATCBlock | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 40 | `verify_block()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 41 | `chain_height()` | UInt64 | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 42 | `status()` | Map | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 43 | `name()` | String | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 44 | `stop()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 45 | `boot()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 46 | `connect()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 47 | `broadcast()` | UInt32 | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 48 | `discover()` | List | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 49 | `peer_count()` | UInt32 | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 50 | `status()` | Map | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 51 | `name()` | String | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 52 | `stop()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 53 | `new_wallet()` | WalletKeys | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 54 | `sign_tx()` | Bytes | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 55 | `verify_sig()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 56 | `balance()` | UInt256 | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 57 | `boot()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 58 | `status()` | Map | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 59 | `name()` | String | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |
| 60 | `stop()` | Bool | `atclang/atc-atclang/programs/atcos_main.atc` | ATCLang |

*+9867 weitere*

**Total: 9927 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
