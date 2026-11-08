# 📜 atc-contracts

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> **Smart Contracts: Token-Standards (ATC-8300, ATCoin), Bridge, Governance, Marketplace, Shivamon**

[![Layer](https://img.shields.io/badge/Layer-L4%2FL11-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/atc-contracts-wiki)

---

## 📋 Beschreibung

`atc-contracts` enthält die Referenz-Implementierungen aller On-Chain Smart Contracts und Token-Standards für das A-TownChain OS Ökosystem (Layer L4/L11). Das Repository bietet sowohl Python- als auch ATCLang-Implementierungen für Governance-Protokolle, den nativen `ATCoin`, den erweiterbaren Token-Standard `ATC-8300`, Cross-Chain Bridge Verträge, Handelsmarktplätze und Shivamon NFT-Engines.

---

## 🏛️ Architektur

Smart Contracts laufen isoliert in der ATVM (A-Town Chain Virtual Machine) und nutzen das **ATC-LIC Lizenzmodell**:

```
[ Decentralized Application / UI ]
                |
                v
 [ Gateway API / RPC Interface ]
                |
                v
 [ ATVM Execution Environment ]
      +---> Token Standard (ATC-8300 / ATCoin)
      +---> Governance DAO (Voting / Timelock)
      +---> Cross-Chain Bridge (Lock / Mint / Vault)
      +---> Decentralized Marketplace Engine
```

---

## 🧩 Komponenten

- **`atc8300/`**: Erweiterbarer Fungible Token Standard
  - `atc8300_token.py` / `atc8300.atc`: Minting, Burning, Transfer & Allowance Logik
- **`atcoin/`**: Native Kryptowährung
  - `atcoin.py`: Native Coin State & Transfer-Prüfung
- **`base/`**: Standard Smart Contract Basisklasse
  - `base_contract.py`: Storage Management, Event Emitter & Ownership Control
- **`bridge/`**: Cross-Chain Token & Data Bridge
  - `bridge_contract.py`: Lock-and-Mint & Burn-and-Release Bridge Mechanismen
- **`governance/`**: On-Chain DAO Governance
  - `governance_contract.py` / `governance.atc`: Vorschlagserstellung, Voting, Stimmgewichtung
- **`marketplace/`**: Dezentraler Marktplatz
  - `marketplace_contract.py`: Orderbook, NFT Listing, Treuhand (Escrow) & Payment Clearing
- **`shivamon/`**: Gaming & NFT Contracts
  - `shivamon_contract.py` / `shivamon.atc`: Shivamon NFT Minting, Attributes, Breeding Engine
- **`wallet/`**: Cryptographic Key Tools
  - `ecdsa.py`, `keygen.py`, `wallet.atc`: Secp256k1/Ed25519 Schlüsselgenerierung & Signatur

---

## 🚀 Usage

### Token Contract Instanziieren
```python
from atc8300.atc8300_token import ATC8300Token

token = ATC8300Token(name="A-Town Token", symbol="ATC", initial_supply=1_000_000)
token.transfer(sender="0xOwner", recipient="0xUser", amount=500)
```

### Marketplace Listing Erstellen
```python
from marketplace.marketplace_contract import MarketplaceContract

mp = MarketplaceContract()
listing_id = mp.create_listing(seller="0xSeller", item_id="NFT-101", price=250)
```

---

## 🛠️ Build & Installation

```bash
# Repository klonen
git clone https://github.com/A-TownChain-Okosystems/atc-contracts.git
cd atc-contracts

# Requirements installieren
pip install -r requirements.txt
```

---

## 🔗 Verwandte Repos & Wiki

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | Haupt-Repo — KAI-OS Core |
| [atc-contracts-wiki](https://github.com/A-TownChain-Okosystems/atc-contracts-wiki) | `Docs` | Offizielles Wiki zu atc-contracts |
| [atc-blockchain](https://github.com/A-TownChain-Okosystems/atc-blockchain) | `L3` | Blockchain Engine & ATVM |
| [atclang](https://github.com/A-TownChain-Okosystems/atclang) | `L2–L4` | ATCLang Programmiersprache |
| [atc-franchise](https://github.com/A-TownChain-Okosystems/atc-franchise) | `L10/L8` | Business DAO & Franchise Vaults |

**📖 Offizielle Dokumentation:** [atc-contracts-wiki](https://github.com/A-TownChain-Okosystems/atc-contracts-wiki)

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Ökosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgeführt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Übersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)
