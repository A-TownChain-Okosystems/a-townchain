# atc-sdk

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen


> **Multi-Language Software Development Kit (TypeScript, Python, Rust, Go)**

[![Layer](https://img.shields.io/badge/Layer-L8-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-sdk-wiki)

---

## 📦 Description / Beschreibung

Das `atc-sdk` Repository stellt Entwicklern Bibliotheken und SDKs für TypeScript, Python, Rust und Go zur Verfügung, um dezentrale Anwendungen (dApps), Wallets und KI-Agenten auf A-TownChain OS zu entwickeln.

---

## 🏗️ Architektur

```
   [ Application Layer (dApp / Web / Mobile / Backend) ]
                             │
                             v
   ┌───────────────────────────────────────────────────┐
   │                     atc-sdk                       │
   │  ┌────────────┬───────────┬───────────┬────────┐  │
   │  │ TypeScript │  Python   │   Rust    │   Go   │  │
   │  └────────────┴───────────┴───────────┴────────┘  │
   └───────────────────────────────────────────────────┘
                             │
                             v (JSON-RPC / WebSockets)
             [ atc-gateway :4000 / RPC :5000 ]
```

---

## 🧱 Komponenten

- **`@atc/sdk` (TypeScript)**: Für Frontend-Anwendungen, React/Vue Integration und Node.js Services.
- **`atc-sdk` (Python)**: Für Data Science, AI-Agenten-Pipelines und Backend-Skripte.
- **`atc-sdk` (Rust)**: Für performante High-Frequency-Trading Services und Microkernel-Extensions.
- **`atc-sdk-go` (Go)**: Für Microservices, P2P-Knoten und Enterprise-Integratoren.

---

## 🚀 Usage / Verwendung

### TypeScript Beispiel
```typescript
import { ATCClient } from '@atc/sdk';
const client = new ATCClient('https://gateway.atownchain.io');
const balance = await client.getBalance('0x1234...');
```

---

## 🛠️ Build & Setup

```bash
# TypeScript
npm install @atc/sdk

# Python
pip install atc-sdk
```

---

## 🔗 Verwandte Repos & Abhängigkeiten

**Nutzt:** [atc-gateway](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-gateway)  
**Wird genutzt von:** [atc-ui](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-ui), [atc-cli](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-cli)  
**Wiki Link:** [→ atc-sdk-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-sdk-wiki)

---

## 🌐 A-TownChain Ökosystem

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | Haupt-Repo — KAI-OS Core |
| [atc-kernel](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-kernel) | `L2` | Microkernel, IPC, ATCFS |
| [atcnet](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atcnet) | `L5` | P2P Netzwerk, Bootstrap |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-gateway) | `L7` | API Gateway Port 4000 |
| [atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atclang) | `L2-L4` | Proprietäre Sprache |
| [atc-contracts](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-contracts) | `L4/L11` | Smart Contracts + Bridge |
| [shivamon](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-shivamon) | `L12` | NFT Gaming |
| [atc-franchise](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-franchise) | `L10/L8` | Business DAO |
| [atc-ui](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-ui) | `L10` | Neon Dashboard |
| [atc-standards](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-standards) | `L0` | Protokoll-Standards |

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
- [`A-TownChain-Okosystems/atc-backend`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-backend)
- [`A-TownChain-Okosystems/atc-blockchain`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-blockchain)
