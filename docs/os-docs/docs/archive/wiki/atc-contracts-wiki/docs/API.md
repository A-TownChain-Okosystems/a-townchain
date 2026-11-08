# 🔌 API Reference — atc-contracts

> **Repo:** [atc-contracts](https://github.com/A-TownChain-Okosystems/atc-contracts)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `init()` | — | `shivamon/shivamon.atc` | ATCLang |
| 2 | `mint()` | u64 | `shivamon/shivamon.atc` | ATCLang |
| 3 | `transfer()` | bool | `shivamon/shivamon.atc` | ATCLang |
| 4 | `level_up()` | bool | `shivamon/shivamon.atc` | ATCLang |
| 5 | `owner_of()` | Address | `shivamon/shivamon.atc` | ATCLang |
| 6 | `tokens_of()` | Vec | `shivamon/shivamon.atc` | ATCLang |
| 7 | `get_shivamon()` | Shivamon | `shivamon/shivamon.atc` | ATCLang |
| 8 | `total_supply()` | u64 | `shivamon/shivamon.atc` | ATCLang |
| 9 | `generate()` | WalletKeys | `wallet/wallet.atc` | ATCLang |
| 10 | `_derive_address()` | Address | `wallet/wallet.atc` | ATCLang |
| 11 | `_generate_mnemonic()` | string | `wallet/wallet.atc` | ATCLang |
| 12 | `restore_from_mnemonic()` | WalletKeys | `wallet/wallet.atc` | ATCLang |
| 13 | `validate_address()` | bool | `wallet/wallet.atc` | ATCLang |
| 14 | `create_tx()` | Transaction | `wallet/wallet.atc` | ATCLang |
| 15 | `verify_tx()` | bool | `wallet/wallet.atc` | ATCLang |
| 16 | `init()` | — | `atc8300/atc8300.atc` | ATCLang |
| 17 | `name()` | string | `atc8300/atc8300.atc` | ATCLang |
| 18 | `symbol()` | string | `atc8300/atc8300.atc` | ATCLang |
| 19 | `decimals()` | u8 | `atc8300/atc8300.atc` | ATCLang |
| 20 | `total_supply()` | u128 | `atc8300/atc8300.atc` | ATCLang |
| 21 | `balance_of()` | u128 | `atc8300/atc8300.atc` | ATCLang |
| 22 | `transfer()` | bool | `atc8300/atc8300.atc` | ATCLang |
| 23 | `approve()` | bool | `atc8300/atc8300.atc` | ATCLang |
| 24 | `allowance()` | u128 | `atc8300/atc8300.atc` | ATCLang |
| 25 | `transfer_from()` | bool | `atc8300/atc8300.atc` | ATCLang |
| 26 | `mint()` | bool | `atc8300/atc8300.atc` | ATCLang |
| 27 | `burn()` | bool | `atc8300/atc8300.atc` | ATCLang |
| 28 | `init_atcoin()` | — | `atc8300/atc8300.atc` | ATCLang |
| 29 | `mint()` | bool | `atc8300/atc8300.atc` | ATCLang |
| 30 | `init()` | — | `governance/governance.atc` | ATCLang |
| 31 | `propose()` | u64 | `governance/governance.atc` | ATCLang |
| 32 | `vote()` | — | `governance/governance.atc` | ATCLang |
| 33 | `finalize()` | — | `governance/governance.atc` | ATCLang |
| 34 | `get_proposal()` | Proposal | `governance/governance.atc` | ATCLang |
| 35 | `total()` | — | `shivamon/shivamon_contract.py` | Python |
| 36 | `__init__()` | — | `shivamon/shivamon_contract.py` | Python |
| 37 | `_generate_dna()` | str | `shivamon/shivamon_contract.py` | Python |
| 38 | `_generate_stats()` | ShivamonStats | `shivamon/shivamon_contract.py` | Python |
| 39 | `_assign_moves()` | list | `shivamon/shivamon_contract.py` | Python |
| 40 | `gain_xp()` | — | `shivamon/shivamon_contract.py` | Python |
| 41 | `to_dict()` | dict | `shivamon/shivamon_contract.py` | Python |
| 42 | `__init__()` | — | `shivamon/shivamon_contract.py` | Python |
| 43 | `mint()` | dict | `shivamon/shivamon_contract.py` | Python |
| 44 | `transfer()` | dict | `shivamon/shivamon_contract.py` | Python |
| 45 | `battle()` | dict | `shivamon/shivamon_contract.py` | Python |
| 46 | `get_token()` | dict | `shivamon/shivamon_contract.py` | Python |
| 47 | `get_owner_tokens()` | list | `shivamon/shivamon_contract.py` | Python |
| 48 | `get_stats()` | dict | `shivamon/shivamon_contract.py` | Python |
| 49 | `to_dict()` | — | `marketplace/marketplace_contract.py` | Python |
| 50 | `__init__()` | — | `marketplace/marketplace_contract.py` | Python |
| 51 | `set_token_contract()` | — | `marketplace/marketplace_contract.py` | Python |
| 52 | `set_balance_oracle()` | — | `marketplace/marketplace_contract.py` | Python |
| 53 | `list_for_sale()` | dict | `marketplace/marketplace_contract.py` | Python |
| 54 | `buy()` | dict | `marketplace/marketplace_contract.py` | Python |
| 55 | `cancel_listing()` | dict | `marketplace/marketplace_contract.py` | Python |
| 56 | `get_listings()` | list | `marketplace/marketplace_contract.py` | Python |
| 57 | `get_listing()` | dict | `marketplace/marketplace_contract.py` | Python |
| 58 | `get_token_listing()` | dict | `marketplace/marketplace_contract.py` | Python |
| 59 | `get_sales_history()` | list | `marketplace/marketplace_contract.py` | Python |
| 60 | `get_stats()` | dict | `marketplace/marketplace_contract.py` | Python |

*+77 weitere*

**Total: 137 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
