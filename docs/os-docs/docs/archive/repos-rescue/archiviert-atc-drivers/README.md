# atc-drivers

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen


> **Hardware Drivers & Device Abstraction Layer (HAL) für ShivaOS Kernel**

[![Layer](https://img.shields.io/badge/Layer-L1-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/atc-drivers-wiki)

---

## 📦 Description / Beschreibung

Das Repository `atc-drivers` stellt Treiber und die Hardware-Abstraktionsschicht (HAL) für den ShivaOS Microkernel bereit. Es ermöglicht direkte Hardware-Ansteuerung für virtualisierte KVM/QEMU-Umgebungen sowie Bare-Metal-Systeme.

---

## 🏗️ Architektur

```
[ ShivaOS Microkernel (atc-kernel) ]
                 │
                 v
[ Hardware Abstraction Layer (HAL) ]
                 │
  ┌──────────────┼──────────────┬──────────────┐
  ▼              ▼              ▼              ▼
[ VirtIO ]    [ UART ]       [ FB ]        [ E1000 ]
(Net/Block)   (Serial 16550) (Graphics)    (Intel NIC)
```

---

## 🧱 Komponenten

- **`virtio`**: VirtIO Network- und Block-Treiber für Hochleistungs-I/O in VM-Umgebungen.
- **`uart`**: Serielle UART 16550 Schnittstelle für Kernel-Debugging und Systemkonsole.
- **`fb`**: Framebuffer Driver für 2D-Grafikausgabe und Konsolen-Rendering.
- **`e1000`**: Intel 82540EM Gigabit Ethernet Treiber für physische Netzwerkverbindungen.

---

## 🚀 Usage / Verwendung

### Treiber im Kernel einbinden
```rust
use atc_drivers::virtio::VirtioNet;
let mut net_dev = VirtioNet::new(mmio_base)?;
net_dev.init()?;
```

---

## 🛠️ Build & Setup

```bash
cargo build --target x86_64-unknown-none --release
```

---

## 🔗 Verwandte Repos & Abhängigkeiten

**Nutzt:** Hardware MMIO / DMA Registers  
**Wird genutzt von:** [atc-kernel](https://github.com/A-TownChain-Okosystems/atc-kernel)  
**Wiki Link:** [→ atc-drivers-wiki](https://github.com/A-TownChain-Okosystems/atc-drivers-wiki)

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
- [`A-TownChain-Okosystems/atc-shivacore`](https://github.com/A-TownChain-Okosystems/atc-shivacore)
