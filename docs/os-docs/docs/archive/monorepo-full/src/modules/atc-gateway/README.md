# 🚪 atc-gateway

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen


> **ℹ️ Architektur-Hinweis:** `atc-gateway` und `atc-backend` haben eine bewusst
> bidirektionale Abhängigkeit (Gateway-Routing Pattern): Der Gateway routet Requests
> zum Backend, während der Backend den Gateway für Service-Discovery nutzt.
> Dies ist KEIN fehlerhafter Kreis, sondern ein etabliertes Microservice-Pattern.


> **API Gateway Port 4000: REST, WebSocket, Rate Limiting, Middleware, Service Discovery**

[![Layer](https://img.shields.io/badge/Layer-L7-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-gateway-wiki)

---

## 📋 Beschreibung

`atc-gateway` stellt das zentrale API-Gateway (Layer L7) auf **Port 4000** für das A-TownChain OS Ökosystem bereit. Es vermittelt Anfragen zwischen externen Clients (Web-Dashboards, Mobile Apps, DApps) und den internen Ökosystem-Diensten (Blockchain Core, Smart Contracts, P2P-Netzwerk).

---

## 🏛️ Architektur

Das Gateway ist in eine Anfrageschleife aufgeteilt, die alle Anfragen validiert, drosselt und an die jeweiligen Backend-Knoten weiterleitet:

```
[ Client / Web / App ] --(HTTP/4000 / WS)--> [ Gateway Router ]
                                                     |
                                                     v
                                          [ Middleware Pipeline ]
                                            ├─ Authentication
                                            ├─ Rate Limiter
                                            ├─ Signature Verifier
                                            └─ Logger
                                                     |
                                                     v
                                          [ Service Discovery ]
                                                     |
                                                     v
                                       [ Backend (Blockchain/Nodes) ]
```

---

## 🧩 Komponenten

- **`python/`** (Stabile Python/FastAPI Implementierung)
  - `main.py`: Gateway Server Entrypoint (Port 4000 Listener)
  - `router.py`: REST & WebSocket Route Handler
  - `middleware/auth.py`: Token & Session Authentifizierung
  - `middleware/rate_limit.py`: Token-Bucket Rate-Limiting gegen DoS
  - `middleware/signature_verify.py`: Cryptographic Request Signaturprüfung
  - `middleware/logger.py`: Anfrage- & Fehler-Logging
  - `service_discovery.atc`: Backend Service Register
- **`atclang/`** (ATCLang Port - Experimentell)
  - `main.atc`, `router.atc`, `middleware/*.atc`: Parallele ATCLang Implementierung
- **`docs/ARCHITECTURE.md`**: Gateway Architektur-Dokumentation

---

## 🚀 Usage

### Gateway Server starten
```bash
# Python Gateway Server starten (Port 4000)
python3 python/main.py
```

### Example API Request
```bash
# Gateway Status
curl http://localhost:4000/api/v1/status

# Blockchain Info abfragen
curl http://localhost:4000/api/v1/chain/info
```

---

## 🛠️ Build & Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-gateway.git
cd atc-gateway
pip install -r requirements.txt
```

---

## 🔗 Verwandte Repos & Wiki

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | Haupt-Repo — KAI-OS Core |
| [atc-gateway-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-gateway-wiki) | `Docs` | Offizielles Gateway Wiki |
| [atc-blockchain](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-blockchain) | `L3` | Blockchain Core Engine |
| [atcnet](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atcnet) | `L5` | P2P Netzwerkschicht |
| [atc-ui](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-ui) | `L10` | Web UI & Dashboard |

**📖 Offizielle Dokumentation:** [atc-gateway-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-gateway-wiki)

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Ökosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgeführt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Übersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)

## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-backend`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-backend)
