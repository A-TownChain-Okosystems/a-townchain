# GlobusOS Standard Operating System Component Model

**Status:** DEVELOPMENT BASELINE  
**Scope:** GlobusOS platform architecture and implementation planning  
**Authority:** GlobusOS architecture documentation; applicable ATC standards remain authoritative where referenced.

## 1. Purpose

This document defines the standard component coverage expected from a complete general-purpose operating system and maps that coverage to GlobusOS boundaries. It is an architecture baseline, not a claim that every component is already production-ready.

GlobusOS is implemented as an independently versioned platform above the reusable ShivaCore kernel. The component model therefore separates kernel/TCB responsibilities, userspace services, hardware integration, desktop/AI layers, and optional A-TownChain integration.

## 2. System layers

```text
0  Firmware / Hardware
1  Boot / Secure Boot / TPM
2  ShivaCore kernel / TCB
3  HAL / device / DMA / IOMMU services
4  Core OS services
5  Storage / networking / security / identity
6  Runtime / package / update / recovery
7  Graphics / audio / desktop
8  Aurora AI / applications
9  A-TownChain platform integration
10 SDK / developer tooling / observability
```

## 3. Standard component coverage

| Domain | Standard components | GlobusOS boundary | Priority |
|---|---|---|---|
| Boot | UEFI, bootloader, boot verification, boot state | boot/ + ShivaCore integration | P0 |
| Hardware | ACPI, CPU init, SMP, interrupts, timers | ShivaCore/HAL | P0 |
| Security hardware | TPM 2.0, Secure Boot, measured boot, RNG | boot/security services | P0 |
| Memory | physical memory, virtual memory, page tables, allocators | ShivaCore + memory service | P0 |
| Execution | processes, threads, scheduler, synchronization | ShivaCore + process service | P0 |
| IPC | endpoints, messages, shared memory, capability transfer | ShivaCore + globus-ipc | P0 |
| Device bus | PCI/PCIe, device discovery, MMIO, MSI/MSI-X | device services/HAL | P0 |
| Isolation | IOMMU, DMA domains, driver isolation | device services + kernel | P0 |
| Storage | VFS, block layer, partitions, filesystem, cache | globus-vfs + storage | P0 |
| NVMe | NVMe MMIO, queues, DMA, completion handling | hardware-specific device service | P0 |
| Networking | Ethernet, NIC DMA, IPv4/IPv6, TCP/UDP, DNS, routing | globus-net + NIC services | P0 |
| Security | capabilities, authorization, audit, keyring, policy | globus-security | P0 |
| Identity | authentication, user/session identity, credential boundary | identity services | P0 |
| Power | ACPI power, suspend, shutdown, reboot, thermal/battery | system services | P1 |
| Services | init, service manager, dependency ordering, health | globus-services/runtime | P0 |
| Graphics | display, KMS/DRM boundary, compositor, GPU driver | globus-graphics + Aurora | P1 |
| Audio | audio driver/service, mixer, playback/recording | globus-audio | P1 |
| Packages | package format, repository metadata, signatures, dependencies | globus-package | P1 |
| Updates | verified atomic update, A/B state, rollback, recovery | globus-update | P1 |
| Userland | shell, CLI utilities, system administration tools | userspace | P1 |
| Desktop | login, session, windowing, input, compositor, settings | Aurora/Desktop | P1 |
| Observability | logs, metrics, tracing, health, crash evidence | system/diagnostics | P1 |
| Development | SDK, ABI/API, compiler/toolchain, debugger, test framework | SDK/tooling | P1 |
| Virtualization | VM/container isolation, virtual devices, passthrough | optional platform layer | P2 |
| AI | model runtime, agents, tools, AI policy, AI UI | Aurora; outside kernel TCB | P1 |
| Blockchain | wallet, node tooling, ATC-VM, ATCLang integration | explicit platform boundary | P1 |

## 4. Hardware implementation chain

Hardware-sensitive implementation must preserve this dependency order:

```text
UEFI / ACPI
  -> PCIe discovery
  -> IOMMU configuration
  -> DMA domains
  -> device MMIO
  -> interrupts / MSI-X
  -> driver service
  -> userspace API
```

For storage:

```text
NVMe controller
  -> submission/completion queues
  -> DMA buffers
  -> block layer
  -> filesystem
  -> VFS
  -> application
```

For networking:

```text
PCIe NIC
  -> IOMMU/DMA
  -> RX/TX rings
  -> Ethernet driver
  -> network stack
  -> socket API
  -> application
```

These are implementation dependencies, not merely documentation categories.

## 5. Trust boundaries

### ShivaCore TCB

- CPU privilege and context control
- address-space isolation
- scheduling primitives
- interrupt handling primitives
- IPC primitives
- capability enforcement
- low-level resource control

### GlobusOS trusted platform services

- service lifecycle
- identity/authentication boundary
- storage and network policy
- device service policy
- package/update verification
- recovery
- system audit

### Outside the ShivaCore TCB

- Aurora AI
- desktop policy
- ordinary applications
- blockchain semantics
- ATCLang execution policy
- wallet UI and application-facing integration

AI and blockchain code must not be promoted into the kernel solely because it is integrated into GlobusOS.

## 6. Readiness states

Each component is tracked independently:

```text
NOT_PLANNED
  -> DESIGNED
  -> IMPLEMENTED
  -> TESTED
  -> AUDITED
  -> PRODUCTION_READY
```

A passing unit/component test does not imply system-wide production readiness. Hardware-dependent components require reproducible evidence from the relevant hardware or emulator/test harness.

## 7. Release model

GlobusOS does not require all components to share one implementation repository or one release lifecycle. Independently versioned component releases are assembled through the platform release manifest.

The release manifest must identify:

- component name
- repository
- exact version/tag
- source commit
- compatibility/API level
- required capabilities
- verification status
- security/audit status

## 8. Integration boundaries

```text
ATCLang
   |
 ATC-VM
   |
A-TownChain

Aurora
   |
GlobusOS IPC/API
   |
ShivaCore

GlobusOS Identity
   |
Wallet integration
   |
A-TownChain identity
```

The ATC-VM remains the boundary between chain execution and the Rust system/network track.

## 9. Definition of complete OS coverage

GlobusOS reaches complete architectural coverage when every P0/P1 domain has an explicit owner, interface, implementation state, test strategy, security boundary, and release-manifest entry. P2 domains remain optional until promoted by roadmap/governance decision.
