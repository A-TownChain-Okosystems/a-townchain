# 🔌 API Reference — franchise-factory

> **Repo:** [atc-franchise](https://github.com/A-TownChain-Okosystems/atc-franchise)
> **Stand:** 2026-08-06

---

## Öffentliche Funktionen

| # | Funktion | Rückgabe | Datei | Sprache |
|---|----------|----------|------|---------|
| 1 | `init()` | void | `contracts/revenue.atc` | ATCLang |
| 2 | `record_revenue()` | void | `contracts/revenue.atc` | ATCLang |
| 3 | `payout_franchisor()` | u128 | `contracts/revenue.atc` | ATCLang |
| 4 | `payout_franchisee()` | u128 | `contracts/revenue.atc` | ATCLang |
| 5 | `get_stats()` | Map | `contracts/revenue.atc` | ATCLang |
| 6 | `init()` | void | `contracts/registry.atc` | ATCLang |
| 7 | `register()` | u64 | `contracts/registry.atc` | ATCLang |
| 8 | `transfer()` | bool | `contracts/registry.atc` | ATCLang |
| 9 | `suspend()` | void | `contracts/registry.atc` | ATCLang |
| 10 | `is_active()` | bool | `contracts/registry.atc` | ATCLang |
| 11 | `get_license()` | FranchiseLicense | `contracts/registry.atc` | ATCLang |
| 12 | `licenses_of()` | Vec | `contracts/registry.atc` | ATCLang |
| 13 | `total_licenses()` | u64 | `contracts/registry.atc` | ATCLang |
| 14 | `init()` | void | `contracts/token.atc` | ATCLang |
| 15 | `name()` | string | `contracts/token.atc` | ATCLang |
| 16 | `symbol()` | string | `contracts/token.atc` | ATCLang |
| 17 | `decimals()` | u8 | `contracts/token.atc` | ATCLang |
| 18 | `total_supply()` | u128 | `contracts/token.atc` | ATCLang |
| 19 | `balance_of()` | u128 | `contracts/token.atc` | ATCLang |
| 20 | `transfer()` | bool | `contracts/token.atc` | ATCLang |
| 21 | `mint()` | bool | `contracts/token.atc` | ATCLang |
| 22 | `burn()` | bool | `contracts/token.atc` | ATCLang |
| 23 | `approve()` | bool | `contracts/token.atc` | ATCLang |
| 24 | `transfer_from()` | bool | `contracts/token.atc` | ATCLang |
| 25 | `deposit()` | Any | `factory.py` | Python |
| 26 | `withdraw()` | bool | `factory.py` | Python |
| 27 | `add_member()` | Any | `factory.py` | Python |
| 28 | `total_stake()` | float | `factory.py` | Python |
| 29 | `voting_power()` | float | `factory.py` | Python |
| 30 | `distribute_revenue()` | Any | `factory.py` | Python |
| 31 | `__init__()` | Any | `factory.py` | Python |
| 32 | `create()` | Franchise | `factory.py` | Python |
| 33 | `get()` | Optional | `factory.py` | Python |
| 34 | `list_all()` | List | `factory.py` | Python |
| 35 | `by_owner()` | List | `factory.py` | Python |
| 36 | `join()` | bool | `factory.py` | Python |
| 37 | `stats()` | dict | `factory.py` | Python |
| 38 | `list_franchises()` | Any | `api/routes.py` | Python |
| 39 | `create()` | Any | `api/routes.py` | Python |
| 40 | `get_franchise()` | Any | `api/routes.py` | Python |
| 41 | `join()` | Any | `api/routes.py` | Python |
| 42 | `stats()` | Any | `api/routes.py` | Python |

**Total: 42 Funktionen**

---

*Auto-generiert 2026-08-06 · Aurora*
