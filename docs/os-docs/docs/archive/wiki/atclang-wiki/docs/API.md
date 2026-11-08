# 🔌 API Reference — atclang

> **Repo:** [atclang](https://github.com/A-TownChain-Okosystems/atclang)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `init()` | void | `programs/atc8300.atc` | ATCLang |
| 2 | `name()` | string | `programs/atc8300.atc` | ATCLang |
| 3 | `symbol()` | string | `programs/atc8300.atc` | ATCLang |
| 4 | `decimals()` | u8 | `programs/atc8300.atc` | ATCLang |
| 5 | `total_supply()` | u128 | `programs/atc8300.atc` | ATCLang |
| 6 | `balance_of()` | u128 | `programs/atc8300.atc` | ATCLang |
| 7 | `transfer()` | bool | `programs/atc8300.atc` | ATCLang |
| 8 | `approve()` | bool | `programs/atc8300.atc` | ATCLang |
| 9 | `allowance()` | u128 | `programs/atc8300.atc` | ATCLang |
| 10 | `transfer_from()` | bool | `programs/atc8300.atc` | ATCLang |
| 11 | `mint()` | bool | `programs/atc8300.atc` | ATCLang |
| 12 | `burn()` | bool | `programs/atc8300.atc` | ATCLang |
| 13 | `init_atcoin()` | void | `programs/atc8300.atc` | ATCLang |
| 14 | `mint()` | bool | `programs/atc8300.atc` | ATCLang |
| 15 | `init()` | void | `programs/atcfs.atc` | ATCLang |
| 16 | `open()` | u64 | `programs/atcfs.atc` | ATCLang |
| 17 | `write()` | u64 | `programs/atcfs.atc` | ATCLang |
| 18 | `read()` | bytes | `programs/atcfs.atc` | ATCLang |
| 19 | `close()` | void | `programs/atcfs.atc` | ATCLang |
| 20 | `stat()` | FileStat | `programs/atcfs.atc` | ATCLang |
| 21 | `rm()` | bool | `programs/atcfs.atc` | ATCLang |
| 22 | `hash_file()` | bytes32 | `programs/atcfs.atc` | ATCLang |
| 23 | `write_encrypted()` | u64 | `programs/atcfs.atc` | ATCLang |
| 24 | `mkdir()` | bool | `programs/atcfs.atc` | ATCLang |
| 25 | `ls()` | Vec | `programs/atcfs.atc` | ATCLang |
| 26 | `generate()` | WalletKeys | `programs/wallet.atc` | ATCLang |
| 27 | `_derive_address()` | Address | `programs/wallet.atc` | ATCLang |
| 28 | `_generate_mnemonic()` | string | `programs/wallet.atc` | ATCLang |
| 29 | `restore_from_mnemonic()` | WalletKeys | `programs/wallet.atc` | ATCLang |
| 30 | `validate_address()` | bool | `programs/wallet.atc` | ATCLang |
| 31 | `create_tx()` | Transaction | `programs/wallet.atc` | ATCLang |
| 32 | `verify_tx()` | bool | `programs/wallet.atc` | ATCLang |
| 33 | `subscribe()` | void | `programs/event_bus.atc` | ATCLang |
| 34 | `unsubscribe()` | bool | `programs/event_bus.atc` | ATCLang |
| 35 | `emit()` | u64 | `programs/event_bus.atc` | ATCLang |
| 36 | `recent()` | Vec | `programs/event_bus.atc` | ATCLang |
| 37 | `stats()` | Map | `programs/event_bus.atc` | ATCLang |
| 38 | `clear()` | void | `programs/event_bus.atc` | ATCLang |
| 39 | `start()` | void | `programs/gateway.atc` | ATCLang |
| 40 | `stop()` | void | `programs/gateway.atc` | ATCLang |
| 41 | `handle()` | Response | `programs/gateway.atc` | ATCLang |
| 42 | `_check_rate()` | bool | `programs/gateway.atc` | ATCLang |
| 43 | `_requires_auth()` | bool | `programs/gateway.atc` | ATCLang |
| 44 | `_validate_key()` | bool | `programs/gateway.atc` | ATCLang |
| 45 | `_verify_tx_signature()` | bool | `programs/gateway.atc` | ATCLang |
| 46 | `_proxy()` | Response | `programs/gateway.atc` | ATCLang |
| 47 | `stats()` | Map | `programs/gateway.atc` | ATCLang |
| 48 | `generate_poh()` | bytes32 | `programs/consensus.atc` | ATCLang |
| 49 | `verify_poh()` | bool | `programs/consensus.atc` | ATCLang |
| 50 | `register_validator()` | void | `programs/consensus.atc` | ATCLang |

*+250 weitere Funktionen*

**Total: 300 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
