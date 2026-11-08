---
document_id: ATC-DOC-ARC-GLOB-002
title: GlobusOS Platform Architecture (Zielarchitektur)
version: 0.1.0
status: draft
owner: A-TownChain-Okosystems
created: 2026-09-14
updated: 2026-09-14
standard: ATC-STD-MD-001
related:
  - SCR-0118 (GlobusOS Architecture Whitepaper v0.9.1 — Systemarchitektur-REFERENZ)
  - SCR-0119 (Kanonisierungs-Antrag dieses Dokuments)
  - AD-012 (Service-Space), AD-013 (Bootchain), AD-024/026 (Repo-Landschaft/Bauhierarchie), AD-027 (M1-M8), AD-028
---

# GlobusOS Platform Architecture — Zielarchitektur (DRAFT v0.1.0)

> **Status:** ZIELARCHITEKTUR (SOLL). Implementierungsstand ausschließlich via
> `.atc/evidence/evidence.yaml` (SCR-0080) und CI-Evidence — CLAIMED != PASS, DOCUMENTED != IMPLEMENTED.
> Bezug zur Systemarchitektur-REFERENZ (SCR-0118, Whitepaper v0.9.1): dieses Dokument verfeinert
> die Plattform-Sicht; das Konflikt-Register K1-K5 von SCR-0118 gilt unverändert weiter.

## 1. Architekturgrundsatz

> **ShivaCore kontrolliert die Maschine. GlobusOS stellt die Plattform bereit. Aurora versteht
> den Benutzer. ATCLang beschreibt Programme. ATC-VM führt deterministischen Code aus.
> A-TownChain stellt die dezentrale Vertrauensschicht bereit.**

GlobusOS ist die Betriebssystem-Plattform oberhalb von ShivaCore und gleichzeitig die
Laufzeitbasis für Aurora, ATCLang, ATC-VM und A-TownChain. ShivaCore wird NICHT mit GlobusOS
verschmolzen: Der Kernel bleibt die vertrauenswürdige, minimale Basis (TCB); GlobusOS stellt
die User-Space- und Plattformfunktionalität bereit. Ein Microkernel gewinnt gerade dadurch,
dass möglichst viele Dienste außerhalb des privilegierten Kernels laufen.

## 2. Die 13-Layer-Architektur

```
L13  Applications
L12  Globus Desktop / UX
L11  Aurora AI Platform
L10  Application & Developer Platform
L09  Runtime Layer (ATCLang / ATC-VM / WASM / Native)
L08  System Services (Init / IPC / Update / Logging / Config)
L07  Security & Identity
L06  Networking
L05  Storage & Filesystems
L04  Device & Driver Framework
L03  Hardware Abstraction (HAL)
L02  ShivaCore (Kernel / TCB)
L01  Hardware / Firmware
```

Wichtige Zuordnung zur Registry-Layer-Skala (AD-026 Bauhierarchie L0-L7): Die L01-L13-Skala
dieses Dokuments ist eine INNERE Plattform-Schichtung des OS-Stacks; die Repository-Layer
(AD-026) bleiben davon unberührt (globus-os = L4-Repo, atc-shivacore = Kernel-Repo).

## 3. Verantwortungsmatrix

| Projekt | Verantwortlichkeit |
|---|---|
| ShivaCore | Kernel / TCB / Isolation (Scheduler, Memory, IPC, VSpace, Capabilities, HAL) |
| GlobusOS | Betriebssystem / Services / Desktop / Plattform (dieses Dokument) |
| Aurora | AI / Agents / AI UX (Policy-kette, kein automatisches Systemrecht) |
| ATCLang | Programmiersprache (Smart Contracts, System-Apps, deterministische Workloads) |
| ATC-VM | Deterministische VM (.atc/.atvm, verifizierte Bytecode-Ausführung) |
| A-TownChain | Blockchain (dezentrale Vertrauensschicht, NICHT Bestandteil des Kernels) |
| Genesis Engine | Universelle generische Game-Plattform (SDK/Core) |
| Genesis Chronicles | Flagship-Produkt auf Genesis Engine |

## 4. Die 15 Hauptkomponenten (Priorität + Ist/Soll)

Ehrliche Stand-Matrix (Orientierung; verbindlich bleibt evidence.yaml):

| # | Komponente | Prio | Ist-Stand (Module) | Soll |
|---|---|---|---|---|
| 01 | Boot & Hardware Initialization | P0 | atc-bootloader (Limine, AD-013), M5 CLAIMED (Evidence incomplete, SCR-0073) | UEFI, Secure Boot, SMP-Init, ACPI, PCI/PCIe-Enumeration, Recovery |
| 02 | ShivaCore Kernel | P0 | SEPARATES REPO (atc-shivacore) | TCB minimal halten; globus-os konsumiert Caps/Scheduling |
| 03 | Hardware Abstraction (HAL) | P0 | Teilweise in atc-drivers | CPU/Interrupt/Timer/PCI/DMA/Power/Clock/Firmware; x86_64 + ARM64, später RISC-V |
| 04 | Process & Resource Management | P0 | via ShivaCore + globus-init (Initial-Caps) | Process/Thread, Health, Resource Limits |
| 05 | IPC / System Calls | P0 | Service-Space (AD-012/028), Kernel-IPC in ShivaCore | Offizielle Syscall-Grenze (§10), kein direkter Kernel-Zugriff |
| 06 | Memory & Storage | P0 | atc-globus-fs (Skelett) | Block Layer, Partition/Mount Manager, Permissions |
| 07 | Device & Driver Framework | P0 | atc-drivers (Skelett) | Bus-Abstraktion (PCI/PCIe/USB/I2C/SPI/UART/virtio), Storage/Input/Graphics/Audio/Network |
| 08 | Filesystem | P0 | atc-globus-fs (Skelett) | Globus VFS + GFS (nativ), ext4/FAT32/exFAT/NTFS/NFS, Journaling, Snapshots |
| 09 | Networking | P0 | atc-globus-net (Skelett) | TCP/IP-Stack (TCP/UDP/IPv4/IPv6/DNS/DHCP/TLS), Network Manager |
| 10 | Security & Identity | P0 | Kernel-Caps in ShivaCore; OS-Seite geplant | Identity, Policy Engine, Sandbox, Secrets, Secure Update, Audit |
| 11 | Init / Service Manager | P0 | globus-init (M5-Objekt) | globus-init + globusd: Lifecycle, Dependencies, Watchdog, Crash Recovery |
| 12 | Runtime & Application Platform | P1 | ATC-VM/atclang als eigene Repos; Bindung geplant | ATCLang/ATC-VM/WASM/Native/Script Runtime + Sandbox |
| 13 | Aurora AI Platform | P1 | aurora-ai (eigenes Repo); OS-Integration geplant | Assistant/Agents/Model Runtime/Policy — AI ohne automatische Systemautorität |
| 14 | Desktop / GUI | P1 | atc-globus-desktop, atc-globus-shell (Skelette) | Compositor, Display-Server-API, WM, Settings, Terminal, Launcher |
| 15 | Developer & Distribution Platform | P1/P2 | atc-globus-registry (Skelett) | Globus SDK (C/Rust/ATCLang/WASM/GUI-API), gpm + .gpkg, Update, Observability |

## 5. Boot-Stack

```
Firmware → UEFI → Bootloader (Limine, AD-013) → ShivaCore → Kernel-Init
   → globus-init → System Services → Aurora / Desktop
```

Erforderlich: UEFI-Unterstützung, Secure Boot, Measured Boot (TPM 2.0), Boot Configuration,
Recovery Boot, Kernel Command Line, Crash/Recovery Mode. Basis-Sicht: AD-013.

## 6. ShivaCore als TCB (Grenze L02/L03)

ShivaCore-Subsysteme (Kernel-Repo, hier nur als Verbraucher definiert): CPU/Scheduler/Threads/
Context Switching/Interrupts/Exceptions/SMP/CPU-Affinity/Timers · Physical+Virtual Memory/
Page Tables/VSpace/Heap/Mapping/Protection Domains · IPC (Message Passing, Channels, Ports,
Shared Memory, Signals) · Capability Security.

Capability-Modell — **No Ambient Authority** (bestehendes Architekturprinzip):

```
Process
 ├── Capability: Memory
 ├── Capability: Device
 ├── Capability: IPC
 ├── Capability: File
 └── Capability: Network
```

Regel: GlobusOS-Module erhalten Rechte ausschließlich über explizite Initial-Caps von
globus-init — niemals ambient.

## 7. Syscall Interface (offizielle Kernel-Grenze)

```
ATCLang → ATC Runtime → GlobusOS API → Syscall → ShivaCore
```

ATCLang (und jede Anwendung) greift NIEMALS direkt auf Kernel-Interne zu. Syscall-Familien:
Process, Memory, Thread, IPC, File, Device, Network, Time, Security, Capability.

## 8. Security Architecture

```
Secure Boot → ShivaCore TCB → Capability Security → Identity → Policy Engine
   → Sandbox → Application
```

Erforderlich: Secure/Measured Boot, TPM 2.0, Hardware-Keys, Kryptographie-Subsystem, User- und
Device-Identity, Capability-Permissions, Sandbox, Prozess-Isolation, Resource Limits,
Audit-Logging, Security Policy, Secrets Manager, Key Management, Secure Update.

Für untrusted Workloads: Namespace-/Resource-Isolation und kontrollierte Ressourcenverteilung
(etablierte OS-Mechanismen; cgroups-Prinzip).

## 9. globus-init & globusd

```
globus-init
 ├── IPC            ├── Device Manager    ├── Network Manager
 ├── Storage Manager ├── Security Manager  ├── Update Manager
 ├── AI Manager     └── Desktop Manager
```

globusd: Service-Lifecycle, Dependency-Management, Startup/Shutdown, Restart, Health Checks,
Watchdog, Crash Recovery, Sandbox-Konfiguration.

## 10. Runtime Layer (L09)

Application Runtime: Native · ATCLang · ATC-VM (.atc/.atvm, deterministisch) · WASM (portabel,
Sandbox, Plugins, untrusted Code) · Script Runtime. Bindungsreihenfolge: generische Plattform
zuerst (SDK/Core), Produkte danach (Genesis-Engine-Trennprinzip gilt analog für die OS-Plattform).

## 11. Aurora AI Platform (L11)

Aurora-Subsysteme: AI Assistant, Agent Runtime, Model Runtime/Manager, Local/Remote AI, Tool
Runtime, Context Manager, Permission Engine, AI Memory, AI Security, AI Policy.

Verbindliche Policy-Kette (bestehendes Prinzip „KI erkennt → Policy entscheidet → System
führt aus"):

```
Aurora AI → Intent → Policy Engine → Permission Check
   → Human Approval / Automatic Policy → System Action
```

AI erhält NIEMALS automatisch Systemautorität.

## 12. Desktop / GUI (L12) und Application Platform (L10)

Desktop: Window Manager, Compositor, Display-Server (Wayland-ähnliche API), Input Manager,
Shell, Notifications, Settings, File Manager, Terminal, Launcher, System Tray; GPU-Beschleunigung,
HiDPI, Multi-Monitor, Touch, Accessibility, Themes, i18n.

Globus SDK: C/Rust/ATCLang/WASM/GUI-API; System-APIs: filesystem, network, graphics, audio,
input, camera, microphone, crypto, identity, notifications, IPC, AI.

## 13. Package Manager gpm & .gpkg

gpm (install/update/remove/verify) mit Paketformat `.gpkg`: Manifest, Binaries, Dependencies,
Permissions, Signatures, Hashes, Provenance, SBOM, Update Channel. Registry-Basis: atc-globus-registry.

## 14. Update-System

Atomic Updates, Signed Updates, Rollback, A/B System, Delta Updates, Security/Driver Updates,
Recovery. Idealkette: Version N → Update → N+1 → Health Check → PASS: commit / FAIL: rollback.

## 15. Observability

Logs, Metrics, Traces, Crash Dumps, Kernel Events, Performance, Health, Security Audit.
Regel: KEINE ungeprüfte Telemetrie nach außen — lokale Diagnose ist Standard.

## 16. Virtualisierung (später, P2)

VM Manager, Hypervisor Interface, Container Runtime, Sandbox Runtime, WASM Sandbox,
Resource Isolation. Zielbild: GlobusOS mit Native-, ATCLang-, WASM-Apps, Containern und VMs.

## 17. A-TownChain Integration (NICHT im Kernel)

```
GlobusOS
 └── A-TownChain Platform
      ├── Node   ├── Wallet   ├── P2P   ├── RPC
      ├── Explorer API   └── Smart Contract Runtime (ATC-VM/ATCLang)
```

ATC Network-Integration im Network Manager: A-TownChain Node, P2P, Wallet, RPC, ATC Identity.
Bezug SCR-0118: K1 (Consensus = PoH+PoS+PoW, atc-algorithm) und K2 (Verträge = ATCLang/ATC-VM,
kein EVM/Solidity) gelten hier unverändert.

## 18. Roadmap-Einordnung

1. **Zuerst (M5):** Bootchain-Evidence vervollständigen (SCR-0073) — CLAIMED → VERIFIED.
2. **Danach (P0-Komponenten):** HAL, Driver Framework, VFS/GFS, Networking, Security, globusd.
3. **Später (P1):** Runtime-Bindung (ATC-VM), Aurora-Integration, Desktop, gpm, Update.
4. **Governance:** Kanonisierung dieses Dokuments via SCR-0119 (§9-Freigabe); Whitepaper v1.0-Arbeitspaket (SCR-0118, P1) bleibt separat.

> Ehrlichkeitsregel: Dieses Dokument beschreibt ZIELARCHITEKTUR. Kein Satz hier ist als
> implementiert zitierbar — der Implementierungsstand folgt ausschließlich evidence.yaml + CI.
