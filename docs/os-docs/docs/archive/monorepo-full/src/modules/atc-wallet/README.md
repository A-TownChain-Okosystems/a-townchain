# atc-wallet

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> **Multi-Account HD-Wallet, Ed25519 Cryptography & Multi-Sig App**

[![Layer](https://img.shields.io/badge/Layer-L10-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-wallet-wiki)

---

## 📖 Beschreibung

Das Repository **atc-wallet** stellt die Kern-Wallet-Anwendung für das A-TownChain Ökosystem bereit. Es verwaltet Schlüsselpaare (Ed25519), Hierarchical Deterministic (HD) Wallets nach BIP44, Multi-Signature Verifikationen sowie Interaktionen mit Smart Contracts und DeFi-Protokollen.

---

## 🏗️ Architektur

Die Wallet-Architektur trennt kryptographisches Schlüsselmanagement strikt vom Netzwerk-Layer. Schlüssel bleiben lokal und AES-256 verschlüsselt gespeichert:

```
+-------------------------------------------------------+
|                   atc-wallet (L10)                    |
|  +--------------------+  +-------------------------+  |
|  | HD-Wallet (BIP44)  |  | Ed25519 Key Vault       |  |
|  +--------------------+  +-------------------------+  |
|  | Multi-Sig Engine   |  | DeFi & Staking Manager  |  |
|  +--------------------+  +-------------------------+  |
+--------------------------+----------------------------+
                           | Signed TX Payload
                           v
              +--------------------------+
              |   atc-gateway / Node RPC |
              +--------------------------+
```

---

## 🧩 Komponenten

- **HD-Wallet Core**: Schlüsselableitung nach BIP32/BIP44 Standard (`m/44'/8300'/0'/0/0`).
- **Ed25519 Cryptography**: High-Performance Signature generation und Verification.
- **Multi-Sig Controller**: Unterstuetzung für m-of-n Schwellenwert-Signaturen und GCL-v2.0 Governance.
- **AES-256 Storage Vault**: Verschlüsselte Speicherung privater Schlüssel und Seed Phrases.

---

## 🚀 Usage

Adresse und Format:
```text
ATC + 40 Hex-Zeichen (Ed25519 Public Key Hash)
Beispiel: ATCf9327118a7dfb30f72ba6aa82e1186078c42232884
```

---

## 🛠️ Build & Installation

```bash
# Repo klonen
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-wallet.git
cd atc-wallet
```

---

## 🗺️ Verwandte Repos

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [atc-sdk](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-sdk) | `L8` | Software Development Kit |
| [atc-contracts](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-contracts) | `L4/L11` | Smart Contracts & Tokens |
| [atc-cli](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-cli) | `L10` | Command Line Wallet & Tools |

---

## 📖 Wiki

Dokumentation, Sicherheits- und Architektur-Guides finden Sie im [atc-wallet-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-wallet-wiki).

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Oekosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgefuehrt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Uebersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)

## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-blockchain`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-blockchain)
