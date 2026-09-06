# atc-dns

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen


> **On-Chain Decentralized Domain Name System (.atc TLD) & Resolution Service**

[![Layer](https://img.shields.io/badge/Layer-L4%2FL7-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-dns-wiki)

---

## 📦 Description / Beschreibung

`atc-dns` bildet das dezentrale Namenssystem des A-TownChain Ökosystems für die `.atc` Top-Level-Domain. Es ermöglicht menschenlesbare Namen für Wallet-Adressen, Gateway-URIs, KAI-Agenten und verteilte Web-Ressourcen.

---

## 🏗️ Architektur

```
[ Client Request: mydomain.atc ]
               │
               v
[ Local DNS Resolver & TTL Cache Engine ]
               │ (Cache Miss)
               v
[ On-Chain Registrar Smart Contract (ATC-DNS) ]
               │
               v
[ Record Resolution: A, AAAA, ATC-URI, TXT, Agent-ID ]
```

---

## 🧱 Komponenten

- **`Registrar Contract`**: Verwalter aller registrierten `.atc` Domains und Inhaberschaften.
- **`Resolution Engine`**: Mapping von Domainnamen auf Blockchain-Adressen und Node-IPs.
- **`Record Storage`**: Unterstützung für `A`, `AAAA`, `ATC-URI`, `TXT` und `AGENT` Records.
- **`Cache Layer`**: In-Memory Caching für Hochgeschwindigkeits-Lookups unter 1ms.

---

## 🚀 Usage / Verwendung

### Domain registrieren
```bash
atc-cli dns register --domain myname.atc --target 0x9000...
```

---

## 🛠️ Build & Setup

```bash
python3 -m pytest
```

---

## 🔗 Verwandte Repos & Abhängigkeiten

**Nutzt:** [atc-contracts](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-contracts), [atc-kernel](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-kernel)  
**Wird genutzt von:** [atc-gateway](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-gateway), [atc-ui](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-ui)  
**Wiki Link:** [→ atc-dns-wiki](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/tree/main/docs/archive/wiki/atc-dns-wiki)

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
- [`A-TownChain-Okosystems/atc-shivacore`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-shivacore)
