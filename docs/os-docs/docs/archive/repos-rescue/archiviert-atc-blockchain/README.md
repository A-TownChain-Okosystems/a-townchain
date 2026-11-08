# ⛓️ atc-blockchain

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> **Blockchain Core: Consensus (PoH + PoA), Block Production, Mempool, Validators, Smart Contracts**

[![Layer](https://img.shields.io/badge/Layer-L3-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/atc-blockchain-wiki)

---

## 📋 Beschreibung

`atc-blockchain` ist die zentrale Blockchain-Kernkomponente des A-TownChain OS Ökosystems. Das Modul bildet die Konsens- und Ausführungsebene (Layer L3) und kombiniert **Proof of History (PoH)** zur sequentiellen Zeitverifikation mit einem **Proof of Authority (PoA)** / Proof of Stake (PoS) Hybrid-Mechanismus zur schnellen Blockvalidierung und Finalität.

### Netzwerke & Chain-IDs
- **Mainnet:** Chain-ID `9000`
- **Testnet:** Chain-ID `9001`

---

## 🏛️ Architektur

Die Architektur von `atc-blockchain` ist modular aufgebaut und trennt Konsens, P2P-Protokoll, Zustandsverwaltung und Smart-Contract-Ausführung:

```
[ P2P Network (atcnet) ] <---> [ Mempool & Transaction Queue ]
                                      |
                                      v
 [ PoH Generator (Verifiable Time) ] ---> [ Consensus Engine (PoH + PoA) ]
                                                  |
                                                  v
 [ ATCLang VM / Contract Engine ] <--- [ Block Production & State Transition ]
                                                  |
                                                  v
                                      [ State Storage & Ledger ]
```

---

## 🧩 Komponenten

- **`consensus/`**: Hybrid-Konsensmodul
  - `poh.py` / `poh.atc`: Proof of History Zeitstempel-Engine & Hash-Kette
  - `pos.atc` / `pos_atc82.atc`: Validator-Auswahl und Block-Finalität
  - `hybrid_consensus.atc`: Integrierte PoH+PoA Konsenssteuerung
  - `fork_resolution.atc`: Gabelungsbehandlung und Longest-Chain-Regel
  - `gas_fee.atc`: Dynamisches Gaspreis- und Gebührenmodell
- **`nodes/`**: Validator- und Netzwerk-Knoten
  - `node.atc`: Validator Node Daemon und State Machine
  - `block_propagation.atc`: Block-Senden und Empfangen über P2P
  - `bootstrap.py` / `bootstrap.atc`: Bootstrap Node Service & Initial Sync
  - `discovery.py`: Kademlia DHT Discovery Integration
  - `testnet_launcher.atc`: Multi-Node Testnetz-Launcher
- **`contracts/`**: On-Chain Smart Contracts & Virtual Machine Engine
  - `atc001/genesis_token.py`: Genesis Token Standard
  - `atc8300/atc8300_token.py`: ATC-8300 Token Standard
  - `base/base_contract.py`: Standard-Vertragsbasisklasse
  - `governance/governance_contract.atc`: Governance Vertrag
  - `contract_engine_atc14.atc`: ATCLang Contract Execution Engine
- **`governance/`**: On-Chain Governance & Treasury
  - `dao.atc` / `dao_live.atc`: DAO-Abstimmungen & Vorschlagsverwaltung
  - `timelock.atc`: Verzögerte Ausführung kritischer Transaktionen
  - `treasury.atc`: Belohnungen & Ökosystem-Guthaben
- **`wallet/`**: Krypto- & Identitäts-Funktionen
  - `ecdsa.py` / `did.py`: ECDSA Signaturprüfung & Dezentrale Identitäten (DID)
  - `multisig.py` / `multisig.atc`: Mehrfachsignatur-Wallets
- **`dex/`**: Dezentrale Börse
  - `amm.atc`: Automated Market Maker & Liquiditätspools
- **`zkp/`**: Zero-Knowledge Proofs
  - `groth16.atc`: ZK-Snark Verifikation für private Transaktionen
- **`mainnet/`**: Genesis & Mainnet Deployment Configs

---

## 🚀 Usage

### Node starten (Testnet)
```bash
# Python Bootstrap Node starten
python3 nodes/bootstrap.py --chain-id 9001 --port 9000

# Node Discovery ausführen
python3 nodes/discovery.py
```

### Transaktion signieren & senden
```python
from wallet.ecdsa import KeyPair
from smart_contracts import Transaction

key = KeyPair.generate()
tx = Transaction(sender=key.address, recipient="0x...", amount=100, nonce=1)
tx.sign(key.private_key)
```

---

## 🛠️ Build & Installation

### Voraussetzungen
- Python 3.10+
- Pip dependencies (`requirements.txt`)

```bash
# Repository klonen
git clone https://github.com/A-TownChain-Okosystems/atc-blockchain.git
cd atc-blockchain

# Testsuite ausführen
python3 -m unittest discover -s . -p "*test*.py"
```

---

## 🔗 Verwandte Repos & Wiki

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | KAI-OS Haupt-Repository |
| [atc-blockchain-wiki](https://github.com/A-TownChain-Okosystems/atc-blockchain-wiki) | `Docs` | Offizielles Wiki & Architektur-Handbuch |
| [atc-kernel](https://github.com/A-TownChain-Okosystems/atc-kernel) | `L2` | ShivaOS Microkernel |
| [atcnet](https://github.com/A-TownChain-Okosystems/atcnet) | `L5` | P2P Netzwerkschicht |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/atc-gateway) | `L7` | API Gateway Port 4000 |
| [atc-contracts](https://github.com/A-TownChain-Okosystems/atc-contracts) | `L4/L11` | Smart Contracts Standard-Bibliothek |

**📖 Offizielle Dokumentation:** [atc-blockchain-wiki](https://github.com/A-TownChain-Okosystems/atc-blockchain-wiki)

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Ökosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgeführt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Übersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)

## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-vm`](https://github.com/A-TownChain-Okosystems/atc-vm)
