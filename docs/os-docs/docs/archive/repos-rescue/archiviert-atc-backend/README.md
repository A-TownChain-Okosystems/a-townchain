# atc-backend

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen


> **ℹ️ Architektur-Hinweis:** `atc-backend` und `atc-gateway` haben eine bewusst
> bidirektionale Abhängigkeit (Gateway-Routing Pattern): Der Gateway routet Requests
> zum Backend, während der Backend den Gateway für Service-Discovery nutzt.
> Dies ist KEINE zirkuläre Abhängigkeit im fehlerhaften Sinne, sondern ein
> etabliertes Microservice-Pattern.



> **FastAPI REST API, JSON-RPC Server & Backend Orchestrator für A-TownChain OS**

[![Layer](https://img.shields.io/badge/Layer-L7%2FL8-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/atc-backend-wiki)

---

## 📦 Description / Beschreibung

Das Repository `atc-backend` implementiert die zentrale Backend-Infrastruktur für das A-TownChain OS Ökosystem. Es vereint REST-API-Endpunkte, JSON-RPC-Schnittstellen, eine hochperformante Datenbank-Anbindung sowie das Orchestrator-System für asynchrone KI- und Blockchain-Tasks.

---

## 🏗️ Architektur

Die Architektur des Backends folgt dem **ATS-1000 Standard** (API Orchestration):

```
+-------------------------------------------------------+
|                    atc-gateway (Port 4000)            |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
|                    atc-backend (Port 5000)            |
|  +-------------------------------------------------+  |
|  | APIServer (api/server.atc)                      |  |
|  +-------------------------------------------------+  |
|  | Route Blueprints:                               |  |
|  | - Wallet Routes (api/routes/api_routes.atc)     |  |
|  | - KAI Agent Routes (api/kai_routes.atc)         |  |
|  | - AI Stream Routes (api/routes/ai_routes.atc)   |  |
|  +-------------------------------------------------+  |
|  | Task Queue Orchestrator                         |  |
|  | (api/orchestrator/orchestrator.py)              |  |
|  +-------------------------------------------------+  |
|  | Database Layer (db/repository.py, schema.sql)   |  |
|  +-------------------------------------------------+  |
+-------------------------------------------------------+
```

---

## 🧱 Komponenten

- **`api/server.atc`**: Haupteinstiegspunkt des REST-Servers auf Port 5000. Registriert alle Routing-Module.
- **`api/kai_routes.atc`**: KAI-Agenten-Schnittstellen (Agent Lifecycle, Task Execution, IPFS Storage Mapping).
- **`api/routes/api_routes.atc`**: Konsolidierte REST-Routen für Wallets, Nodes, Blockchain, Governance und Explorer.
- **`api/routes/ai_routes.atc`**: Endpunkte für KI-Abfragen (LLM Query, Prompt Streaming, Model Status).
- **`api/orchestrator/`**: Task-Queue-Orchestrator mit Multi-Thread-Worker-Pool, Load Balancing und Circuit Breaker.
- **`db/`**: Relationaler Data Layer (`repository.py`, `schema.sql`, `connection.py`).
- **`wallet/`**: Multi-Standard Wallet Service (`wallet.atc`) für Key Generation (BIP39/ECDSA) und Faucet operations.

---

## 🚀 Usage / Verwendung

### Starten des Backend-Servers
```bash
python3 -m api.orchestrator.orchestrator
```

### KI-Modell Abfrage
```bash
curl -X POST http://localhost:5000/api/v1/ai/query \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Generiere Smart Contract Template", "model": "aurora-v1"}'
```

---

## 🛠️ Build & Setup

1. **Abhängigkeiten installieren:**
   ```bash
   pip install -r requirements.txt
   ```
2. **Umgebungsvariablen konfigurieren:**
   ```bash
   cp .env.example .env
   ```
3. **Tests ausführen:**
   ```bash
   python3 -m pytest
   ```

---

## 🔗 Verwandte Repos & Abhängigkeiten

**Nutzt:** [atc-shivacore](https://github.com/A-TownChain-Okosystems/atc-shivacore), [atc-kernel](https://github.com/A-TownChain-Okosystems/atc-kernel)  
**Wird genutzt von:** [atc-gateway](https://github.com/A-TownChain-Okosystems/atc-gateway), [atc-ui](https://github.com/A-TownChain-Okosystems/atc-ui)  
**Wiki Link:** [→ atc-backend-wiki](https://github.com/A-TownChain-Okosystems/atc-backend-wiki)

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
