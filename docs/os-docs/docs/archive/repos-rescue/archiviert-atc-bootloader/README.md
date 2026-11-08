# atc-bootloader

> ## 🤖 Fuer KI-Agenten — Pflichtlektuere vor jeder Aenderung
> Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`:
> 1. [`AGENT_POLICY.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_POLICY.md) — verbindliche Regeln, Reality-Check, Konsolidierungsziel
> 2. [`AGENT_COORDINATION.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/AGENT_COORDINATION.md) — wer arbeitet gerade woran, Todos, Agent-IDs
> 3. [`DECISIONS_REGISTER.md`](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) — verbindliche Architektur-Entscheidungen


> **Low-Level Bootloader: BIOS/UEFI → ShivaOS Kernel → A-TownChain Handshake**

[![Layer](https://img.shields.io/badge/Layer-L1-purple)](https://github.com/A-TownChain-Okosystems)
[![KAI-OS](https://img.shields.io/badge/KAI--OS-v1.0.0-blue)](https://github.com/A-TownChain-Okosystems/a-townchain-os/blob/main/docs/kai-os-wiki.md)
[![Org](https://img.shields.io/badge/Org-A--TownChain--Okosystems-green)](https://github.com/A-TownChain-Okosystems)
[![Wiki](https://img.shields.io/badge/Wiki-📖-blue)](https://github.com/A-TownChain-Okosystems/atc-bootloader-wiki)

---

## 📦 Description / Beschreibung

Das Repository `atc-bootloader` bildet das Fundament beim Systemstart von A-TownChain OS / KAI-OS. Es übernimmt den Übergang von Bare-Metal x86_64 / UEFI Firmware in den geschützten 64-Bit Long Mode, lädt das ATCFS Dateisystem, verifiziert die kryptografische Signatur des `atc-kernel` Images und startet das Betriebssystem.

---

## 🏗️ Architektur

Das Bootloader-Design gliedert sich in ein 3-Stufen-Modell:

```
[ Stage 1: Firmware / BIOS / UEFI ]
               |
               v (x86 Real/Protected Mode -> Long Mode Transition)
[ Stage 2: ATCFS Volume Loader & Ed25519 Image Verification ]
               |
               v (Handover: GDT, IDT, Paging & Device Tree)
[ Stage 3: ShivaOS Kernel Execution (atc-kernel) ]
```

---

## 🧱 Komponenten

- **Stage 1 (Boot Sector / EFI Application)**: Initialisiert CPU-Register, erstellt die initiale GDT (Global Descriptor Table) und schaltet in den 64-Bit Mode.
- **Stage 2 (Kernel & Memory Loader)**: Parst die ATCFS-Partitionstabelle, verifiziert den Kernel-Hash und prüft Ed25519 Signaturen.
- **Stage 3 (Chain State Handshake)**: Baut den initialen Memory Layout Map auf und übergibt die Kontrolle an den ShivaOS Microkernel.

---

## 🚀 Usage / Verwendung

### Emulator-Test mit QEMU
```bash
qemu-system-x86_64 -drive file=atc-bootloader.bin,format=raw -m 2048
```

---

## 🛠️ Build & Setup

1. **Voraussetzungen:** `nasm`, `gcc-x86-64-linux-gnu`, `qemu-system-x86`
2. **Kompilieren:**
   ```bash
   make build
   ```

---

## 🔗 Verwandte Repos & Abhängigkeiten

**Nutzt:** Bare-Metal Hardware / Firmware  
**Wird genutzt von:** [atc-kernel](https://github.com/A-TownChain-Okosystems/atc-kernel)  
**Wiki Link:** [→ atc-bootloader-wiki](https://github.com/A-TownChain-Okosystems/atc-bootloader-wiki)

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
