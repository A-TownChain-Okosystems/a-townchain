# atc-explorer

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> **Block Explorer & Chain Analytics für A-TownChain OS**

[![Layer](https://img.shields.io/badge/Layer-L10-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-explorer-wiki)

---

## 📖 Beschreibung

Das Repository **atc-explorer** beinhaltet die Web-Anwendung und API-Dienste für den A-TownChain Block Explorer. Es ermöglicht die Inspektion von Blöcken, Transaktionen, Adressen, Smart Contracts, Token-Transfers und Netzwerkanalysen in Echtzeit.

---

## 🏗️ Architektur

Der Explorer indiziert Blockchain-Daten über den Node-RPC/Gateway-Dienst und stellt REST API-Endpunkte für das Frontend bereit:

```
+-------------------------------------------------------+
|                  atc-explorer (L10)                   |
|  +--------------------+  +-------------------------+  |
|  | Block & TX Viewer  |  | Contract Storage Inspector|
|  +--------------------+  +-------------------------+  |
|  | Address Ledger     |  | Network Metrics         |  |
|  +--------------------+  +-------------------------+  |
+--------------------------+----------------------------+
                           | RPC Query
                           v
              +--------------------------+
              |   atc-kernel / Gateway   |
              +--------------------------+
```

---

## 🧩 Komponenten

- **Block Inspector**: Echtzeit-Anzeige neu erzeugter Blöcke, Gas-Usage und Validator-Signaturen.
- **Transaction Tracker**: Detailansicht für Transaktions-Status, Parameter, Payloads und Logs.
- **Contract Viewer**: Code-Inspektion, ABI-Visualisierung und Storage-State Analysen.
- **API Engine**: REST API Endpunkte (`/api/blocks`, `/api/tx`, `/api/address`, `/api/stats`).

---

## 🚀 Usage

Beispiel REST API Endpunkte:

```http
GET /api/blocks?page=1&limit=20
GET /api/block/:hash
GET /api/tx/:hash
GET /api/address/:addr
GET /api/stats
```

---

## 🛠️ Build & Installation

```bash
# Repo klonen
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-explorer.git
cd atc-explorer
```

---

## 🗺️ Verwandte Repos

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [atc-frontend](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-frontend) | `L10` | React Desktop UI |
| [atc-backend](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-backend) | `L7` | Backend Services |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-gateway) | `L7` | Central API Gateway |

---

## 📖 Wiki

Dokumentation und API-Spezifikationen finden Sie im [atc-explorer-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-explorer-wiki).

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Oekosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgefuehrt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Uebersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)
