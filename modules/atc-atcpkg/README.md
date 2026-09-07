# 📦 ATCLang Package Manager (`atcpkg`)

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

**`atcpkg` Package Manager** — Offizielles Paketverwaltungs-System für ATCLang Bibliotheken, Smart Contracts und KAI-OS Kernel-Module. Bietet automatisierte Dependency Resolution, Package Registry Integration, Plugin-Verwaltung und Anbindung an das KAI-OS Kernel Interface (ATC-96, ATC-24).

[![Layer](https://img.shields.io/badge/Layer-L3-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖_atc--atcpkg--wiki-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-atcpkg-wiki)

---

## 🏛️ Architektur Diagramm

```
+---------------------------------------------------------------+
|                       atcpkg CLI / Manager                    |
+---------------------------------------------------------------+
        |                                       |
        v                                       v
+-----------------------+               +-----------------------+
| Kernel Interface      |               | Package Registry      |
| (kernel/manager.atc)  |               | (tools/manager.atc)   |
+-----------------------+               +-----------------------+
        |                                       |
        v                                       v
+---------------------------------------------------------------+
| Dependency Resolver & Plugin System (ATC-24 / ATC-96)        |
+---------------------------------------------------------------+
```

---

## 🧩 Komponenten Tabelle

| Komponente | Datei / Pfad | Beschreibung |
|------------|--------------|--------------|
| **Kernel Manager** | `kernel/manager.atc` | KAI-OS Kernel-Schnittstelle zur Modul-Registrierung & Memory Mapping |
| **Tools Manager** | `tools/manager.atc` | CLI-Tools, Dependency Resolution & Package Unpacking |
| **Agent Scheduling** | `docs/ATC-24-AGENT_SCHEDULING.md` | Spezifikation zur Agenten-Einplanung & Paket-Hooks |
| **Kernel Protocol** | `docs/ATC-96-KERNEL_INTERFACE_PROTOCOL.md` | Protokoll zur Interaktion zwischen `atcpkg` und Kernel |
| **Plugin Architecture** | `docs/ISSUE_27...md` | Spezifikation des erweiterbaren Plugin-Systems |

---

## 💻 Usage Example

```bash
# Paket im Projekt installieren
atcpkg install atc-crypto-ext@1.0.0

# Neue Abhängigkeit hinzufügen
atcpkg add stdlib-collections

# Paket veröffentlichen im A-TownChain Netz
atcpkg publish --license ATC-LIC

# Kernel Status abfragen
atcpkg status
```

---

## 🛠️ Build & Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atcpkg.git
cd atc-atcpkg

# ATCLang Programme kompilieren oder in KAI-OS einbinden
atclang kernel/manager.atc
```

---

## 🌐 Verwandte Repos

| Repo | Beschreibung |
|------|--------------|
| [atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atclang) | ATCLang Kern-Sprache |
| [atc-stdlib](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-stdlib) | Offizielle Standardbibliothek |
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | KAI-OS Hauptsystem |

---

## 📖 Wiki Link

Vollständige Handbücher und Kernel-Protokolle:
👉 **[atc-atcpkg-wiki Repository](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-atcpkg-wiki)**

---

## ⚖️ Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **All Rights Reserved.**
ATC-LIC Lizenzmodell.
