# 📖 atc-atclang-wiki

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen

> Offizielle Dokumentation für [atc-atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atclang) — **ATCLang Modular Extensions** im A-TownChain Ökosystem

[![Code](https://img.shields.io/badge/Code-atc--atclang-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atclang)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-Wiki-purple)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)
[![Ökosystem](https://img.shields.io/badge/Ökosystem-A--TownChain-green)](https://github.com/A-TownChain-Okosystems)

---

## 📦 Zugehöriges Code-Repo

**→ [atc-atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atclang)** (`main` Branch)

---

## 🌐 A-TownChain Ökosystem

Alle Repos des Ökosystems auf einen Blick:

| Repo | Layer | Beschreibung |
|------|-------|-------------|
| [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | `L2–L4` | Haupt-Repo — KAI-OS Core, AI, Blockchain |
| [a-townchain-os-docs](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs) | `L0` | Zentrale Governance & Master-Spezifikation |
| [atc-kernel](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-kernel) | `L2` | Microkernel, IPC, ATCFS, Consensus |
| [atcnet](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atcnet) | `L5` | P2P Netzwerk, Kademlia DHT, Bootstrap Node |
| [atc-gateway](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-gateway) | `L7` | API Gateway Port 4000, Circuit Breaker |
| [atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atclang) | `L2–L4` | ATCLang Kern-Compiler & Runtime Engine |
| [atc-atclang](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atclang) | `L2–L4` | Modulare Erweiterungen, TypeChecker, Optimizer, Extended Stdlib |
| [atc-vm](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-vm) | `L2` | Standalone ATVM Virtual Machine Engine |
| [atc-stdlib](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-stdlib) | `L2–L3` | Standalone ATCLang Standard Library Module |
| [atc-atcpkg](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-atcpkg) | `L3` | ATCLang Package Manager & Registry |
| [atc-contracts](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-contracts) | `L4/L11` | Smart Contracts: ATC-8300, ATC-9000, ATC-9900 |
| [atc-franchise](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-franchise) | `L10/L8` | Business DAO, Vault, Revenue Share |
| [atc-ui](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-ui) | `L10` | Neon Dashboard, Wallet, Explorer, Genesis Chronicles |
| [atc-standards](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-standards) | `L0` | Protokoll-Standards: ATC-0001–0009 + ATS-1000–1007 |

---

## 📑 Themen-Dokumentation

- **[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)** — Architektur der modularen Compiler- und Runtime-Pipeline
- **[docs/MODULES.md](docs/MODULES.md)** — Detaillierte Modulreferenz (Compiler, Lexer, Parser, TypeChecker, Optimizer, Stdlib, VM)
- **[docs/ROADMAP.md](docs/ROADMAP.md)** — Entwicklungs-Roadmap & Meilensteine

---

*Teil des [A-TownChain Ökosystems](https://github.com/A-TownChain-Okosystems) · v1.0.0 · Stand: 2026-08-05*

---

## ⚖️ Lizenz

Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. **Apache-2.0 lizenziert — siehe LICENSE**
ATC-LIC Lizenzmodell.
