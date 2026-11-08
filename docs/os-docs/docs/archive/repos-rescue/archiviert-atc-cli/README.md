# atc-cli

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen


> **Offizielles Command-Line Interface (CLI) für A-TownChain OS & KAI-OS**

[![Layer](https://img.shields.io/badge/Layer-L10-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/atc-cli-wiki)

---

## 📦 Description / Beschreibung

Das Tool `atc-cli` bietet Entwicklern, Stakern und Administratoren eine umfassende Kommandozeilenschnittstelle zur Interaktion mit Nodes, Wallets, Smart Contracts, On-Chain DNS (.atc) und KI-Agenten im A-TownChain Netz.

---

## 🏗️ Architektur

```
  +-------------------------------------------------------+
  |                        atc-cli                        |
  |  +-------------------------------------------------+  |
  |  | Command Parser & Config Engine                  |  |
  |  +-------------------------------------------------+  |
  |  | Subcommands:                                    |  |
  |  | - node     | - wallet    | - contract           |  |
  |  | - dns      | - ai        | - config             |  |
  |  +-------------------------------------------------+  |
  |  | JSON-RPC & Gateway Client                       |  |
  +-------------------------------------------------------+
                             │
                             v
               [ atc-gateway :4000 / RPC :5000 ]
```

---

## 🧱 Komponenten

- **`node`**: Steuerung von Node-Diensten, Sync-Status und Peer-Verbindungen.
- **`wallet`**: Erstellung von Wallets, Schlüsselverwaltung, Transfer und Faucet-Anfragen.
- **`contract`**: Kompilieren, Deployen und Aufrufen von ATCLang Smart Contracts.
- **`dns`**: Registrierung und Abfrage von `.atc` Domain-Einträgen.
- **`ai`**: Interaktion mit KAI-Agenten und LLM-Inferenz-Endpunkten.

---

## 🚀 Usage / Verwendung

### Beispiele
```bash
# Wallet Kontostand abfragen
atc-cli wallet balance --address 0x1234...5678

# Smart Contract deployen
atc-cli contract deploy --file Token.atc

# On-Chain DNS auflösen
atc-cli dns resolve alice.atc
```

---

## 🛠️ Build & Setup

1. **Kompilieren / Installieren:**
   ```bash
   pip install -e .
   ```

---

## 🔗 Verwandte Repos & Abhängigkeiten

**Nutzt:** [atc-sdk](https://github.com/A-TownChain-Okosystems/atc-sdk), [atc-gateway](https://github.com/A-TownChain-Okosystems/atc-gateway)  
**Wird genutzt von:** Entwickler & Node-Betreiber  
**Wiki Link:** [→ atc-cli-wiki](https://github.com/A-TownChain-Okosystems/atc-cli-wiki)

---

## 🌐 A-TownChain Ökosystem

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | Haupt-Repo — KAI-OS Core |
| [atc-kernel](https://github.com/A-TownChain-Okosystems/atc-kernel) | `L2` | Microkernel, IPC, ATCFS |
| [atcnet](https://github.com/A-TownChain-Okosystems/atcnet) | `L5` | P2P Netzwerk, Bootstrap |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/atc-gateway) | `L7` | API Gateway Port 4000 |
| [atclang](https://github.com/A-TownChain-Okosystems/atclang) | `L2-L4` | Proprietäre Sprache |
| [atc-contracts](https://github.com/A-TownChain-Okosystems/atc-contracts) | `L4/L11` | Smart Contracts + Bridge |
| [shivamon](https://github.com/A-TownChain-Okosystems/shivamon) | `L12` | NFT Gaming |
| [atc-franchise](https://github.com/A-TownChain-Okosystems/atc-franchise) | `L10/L8` | Business DAO |
| [atc-ui](https://github.com/A-TownChain-Okosystems/atc-ui) | `L10` | Neon Dashboard |
| [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) | `L0` | Protokoll-Standards |

---

*Teil des [A-TownChain Ökosystems](https://github.com/A-TownChain-Okosystems) · v1.0.0 · Stand: 2026-08-05*

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes
Open-Source-Oekosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgefuehrt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Uebersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)

## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-backend`](https://github.com/A-TownChain-Okosystems/atc-backend)
- [`A-TownChain-Okosystems/atc-blockchain`](https://github.com/A-TownChain-Okosystems/atc-blockchain)
