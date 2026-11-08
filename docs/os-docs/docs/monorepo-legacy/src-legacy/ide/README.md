# atc-ide

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> **IDE Extensions & Language Server Protocol (LSP) für ATCLang**

[![Layer](https://img.shields.io/badge/Layer-L10-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/atc-ide-wiki)

---

## 📖 Beschreibung

Das Repository **atc-ide** enthält Entwicklungswerkzeuge, Plugins für VS Code, Vim und JetBrains sowie die Language Server Protocol (LSP) Implementierung für die Entwicklung von ATCLang Smart Contracts und OS-Programmen.

---

## 🏗️ Architektur

Die IDE-Architektur basiert auf dem Language Server Protocol (LSP Standard). Der LSP-Server kommuniziert mit dem ATCLang Compiler und der ATVM:

```
+-------------------------------------------------------+
|                    atc-ide (L10)                      |
|  +--------------------+  +-------------------------+  |
|  | VS Code / Vim Ext  |  | JetBrains Plugin        |  |
|  +--------------------+  +-------------------------+  |
|  | LSP Language Server|  | Gas Estimator & Debug   |  |
|  +--------------------+  +-------------------------+  |
+--------------------------+----------------------------+
                           | LSP Protocol (JSON-RPC)
                           v
              +--------------------------+
              |    atclang / atc-vm      |
              +--------------------------+
```

---

## 🧩 Komponenten

- **VS Code Extension (`atc-vscode`)**: Syntax Highlighting, Snippets und Hover Information.
- **LSP Server**: Autovervollstaendigung, Go-to-Definition, Diagnosen und Refactoring.
- **Gas Estimator**: Exakte Vorhersage der Gas-Kosten vor dem Deployment.
- **Debugger Engine**: Step-by-Step Debugging in der ATVM Simulation.

---

## 🚀 Usage

LSP Capability Matrix:

```json
{
  "capabilities": {
    "textDocument/completion": true,
    "textDocument/hover": true,
    "textDocument/definition": true,
    "textDocument/references": true,
    "textDocument/diagnostic": true
  }
}
```

---

## 🛠️ Build & Installation

```bash
# Repo klonen
git clone https://github.com/A-TownChain-Okosystems/atc-ide.git
cd atc-ide
```

---

## 🗺️ Verwandte Repos

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [atclang](https://github.com/A-TownChain-Okosystems/atclang) | `L2-L4` | ATCLang Compiler & Parser |
| [atc-vm](https://github.com/A-TownChain-Okosystems/atc-vm) | `L3` | A-TownChain Virtual Machine |
| [atc-sdk](https://github.com/A-TownChain-Okosystems/atc-sdk) | `L8` | Software Development Kit |

---

## 📖 Wiki

Dokumentation und LSP-Spezifikationen finden Sie im [atc-ide-wiki](https://github.com/A-TownChain-Okosystems/atc-ide-wiki).

---

## Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **Apache-2.0 lizenziert — siehe LICENSE**

Dieses Projekt nutzt das **ATC-LIC Lizenzmodell** — ein monetarisiertes, autonomes Open-Source-Oekosystem. Unlizenzierter Code wird von der ATVM physisch nicht ausgefuehrt.

- [ATC-LIC — Smart Contract Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SMART_CONTRACT_LICENSE.md)
- [ATC-LIC — System & Hardware Licenses](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-LIC-SYSTEM_HARDWARE_LICENSE.md)
- [Compliance-Handbuch (BaFin)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/compliance/COMPLIANCE_HANDBUCH.md)
- [Lizenz-Uebersicht](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/LICENSING_OVERVIEW.md)
