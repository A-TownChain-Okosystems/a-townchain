# atc-frontend

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> **React/TypeScript Desktop UI & Neon Dashboard für KAI-OS**

[![Layer](https://img.shields.io/badge/Layer-L10-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/atc-frontend-wiki)

---

## 📖 Beschreibung

Das Repository **atc-frontend** beinhaltet das primäre Benutzerinterface und die KAI-OS Desktop Oberflaeche für das A-TownChain Ökosystem. Es vereint ein futuristisches Neon Dashboard, Desktop-App-Navigation, Bootscreen, Battle Center und direkte API-Integrationen mit dem A-TownChain Gateway.

---

## 🏗️ Architektur

atc-frontend bildet die Layer-10 (L10) Präsentationsschicht des A-TownChain OS. Es kommuniziert mit dem Gateway-Dienst (`atc-gateway`) via REST API und WebSockets für Echtzeit-Statusupdates:

```
+-------------------------------------------------------+
|                 atc-frontend (L10)                    |
|  +-------------------+  +--------------------------+  |
|  | KAI-OS Desktop UI |  | Neon Dashboard & Widgets |  |
|  +-------------------+  +--------------------------+  |
|  | Bootscreen & Auth |  | Battle & Factory Views   |  |
|  +-------------------+  +--------------------------+  |
+--------------------------+----------------------------+
                           | REST / WS API
                           v
              +--------------------------+
              |   atc-gateway (Port 4000)|
              +--------------------------+
```

---

## 🧩 Komponenten

- **`index.html`**: Haupt-Dashboard und KAI-OS Desktop UI (Neon Theme, Window Manager, Widget Grid, System Health).
- **`bootscreen/`**: Interaktiver Boot-Screen, BIOS-Emulation und Hardware-Diagnostic Visualizer.
- **`battle/`**: Shivamon Battle Center UI für Web3 Game-Matches und NFT Arena.
- **`assets/js/api.js`**: REST- und WebSocket-Client für die Anbindung an `atc-gateway` (:4000) und Backend-Services.
- **`assets/css/`**: Styling-System basierend auf CSS Variables, Neon Effects und Responsive Grids.

---

## 🚀 Usage

Das Frontend kann direkt im Webbrowser geöffnet oder über einen HTTP-Server bereitgestellt werden:

```bash
# Webserver lokal auf Port 3000 starten
python3 -m http.server 3000

# Browser öffnen
open http://localhost:3000
```

---

## 🛠️ Build & Installation

Voraussetzungen: Node.js 18+ (optional) oder Python 3.x für statische Webserver-Bereitstellung.

```bash
# Repo klonen
git clone https://github.com/A-TownChain-Okosystems/atc-frontend.git
cd atc-frontend

# Lokalen Dev-Server starten
python3 -m http.server 3000
```

---

## 🗺️ Verwandte Repos

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [atc-ui](https://github.com/A-TownChain-Okosystems/atc-ui) | `L10` | UI Component Library & Neon Design System |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/atc-gateway) | `L7` | Central API Gateway (Port 4000) |
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2-L4` | Main OS Core Repository |
| [atc-wallet](https://github.com/A-TownChain-Okosystems/atc-wallet) | `L10` | Wallet Application |

---

## 📖 Wiki

Dokumentation und Architekturentwürfe finden Sie im [atc-frontend-wiki](https://github.com/A-TownChain-Okosystems/atc-frontend-wiki).

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Oekosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgefuehrt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Uebersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)

## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-backend`](https://github.com/A-TownChain-Okosystems/atc-backend)
