# GlobusOS System Architecture v1

**Status:** active architecture baseline  
**Updated:** 2026-09-15  
**Lifecycle:** development / NOT_READY

## 1. Authority model

GlobusOS is userspace above ShivaCore. ShivaCore remains the kernel/TCB and owns capabilities, address spaces, scheduling, IPC primitives, interrupts and low-level resource control. GlobusOS does not place AI, blockchain or desktop policy into the kernel.

```text
Applications
    |
Aurora AI / Desktop / Shell
    |
GlobusOS service layer
    |-- service manager
    |-- security / identity / settings
    |-- process / runtime
    |-- VFS / storage
    |-- network
    |-- device services
    |-- graphics / audio
    |-- package / update / recovery
    |-- wallet integration boundary
    |
IPC + capabilities
    |
ShivaCore
    |
HAL / firmware
    |
Hardware
```

## 2. Repository implementation

The `system/` workspace is the canonical GlobusOS userspace foundation. Each crate is intentionally narrow so services can be isolated behind IPC boundaries.

| Crate | Responsibility | Current boundary |
|---|---|---|
| `globus-system-core` | System lifecycle and kernel-facing identity | implemented foundation |
| `globus-ipc` | Message/endpoint model | implemented foundation |
| `globus-security` | Explicit capability authorization | implemented foundation |
| `globus-process` | Process/thread lifecycle | implemented foundation |
| `globus-memory` | Address-space and page policy | implemented foundation |
| `globus-vfs` | VFS namespace and mounts | implemented foundation |
| `globus-net` | Network policy/socket boundary | contract; hardware/network stack incomplete |
| `globus-devices` | Device/driver service registry | contract + hardware-facing device modules |
| `globus-services` | Service lifecycle/dependency order | implemented foundation |
| `globus-graphics` | Display/compositor/GPU boundary | Desktop Core implemented; hardware GPU backend incomplete |
| `globus-audio` | Audio service boundary | service boundary |
| `globus-package` | Signed package metadata verification | implemented boundary |
| `globus-update` | Atomic A/B update and rollback state | implemented state-machine boundary |
| `globus-runtime` | Integrated userspace runtime | integration foundation |
| `globus-identity` | Identity/authentication/recovery contract | implemented boundary |
| `globus-wallet-service` | Wallet integration boundary | implemented boundary; production key storage incomplete |
| `globus-settings` | Settings service/UI boundary | implemented foundation |

## 3. Boot contract

```text
UEFI/firmware
  -> bootloader
  -> ShivaCore
  -> capability/IPC initialization
  -> GlobusOS init/service manager
  -> security + devices + storage + network
  -> runtime + graphics + audio
  -> identity/login + wallet boundary
  -> Aurora / desktop / applications
```

A service is not trusted merely because it starts successfully. Its authority must be represented by explicit capabilities and policy.

## 4. Hardware dependency chain

Hardware-dependent work follows this order:

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

Storage:

```text
NVMe controller
  -> submission/completion queues
  -> DMA buffers
  -> block layer
  -> filesystem
  -> VFS
  -> application
```

Networking:

```text
PCIe NIC
  -> IOMMU/DMA
  -> RX/TX rings
  -> Ethernet driver
  -> network stack
  -> socket API
  -> application
```

The repository currently contains hardware-facing contracts/modules for PCI, IOMMU, NVMe and Ethernet. These are not by themselves evidence of a working target-hardware driver.

## 5. Security invariants

1. No ambient authority.
2. Deny by default.
3. AI services are outside the ShivaCore TCB.
4. Blockchain services are outside the ShivaCore TCB.
5. Drivers are isolated services where the platform permits it.
6. Package installation requires metadata/integrity/signature evidence.
7. Updates must support verification and rollback.
8. A component test does not imply system-wide production readiness.
9. Private wallet/recovery material must remain behind protected identity/key boundaries.

## 6. Ecosystem boundaries

```text
ATCLang -> ATC-VM -> A-TownChain
                 ^
                 |
        explicit GlobusOS API

Aurora -> GlobusOS IPC/API -> ShivaCore

GlobusOS Identity -> wallet integration -> A-TownChain identity
```

The VM is the boundary between chain execution and the Rust system/network track. GlobusOS may host ATC tooling but does not make chain semantics a kernel responsibility.

## 7. Readiness and evidence

The canonical readiness progression is:

`NOT_PLANNED -> DESIGNED -> IMPLEMENTED -> TESTED -> AUDITED -> PRODUCTION_READY`

Source code presence means `IMPLEMENTED` only where the implementation is actually present. Hardware-dependent features require reproducible hardware/emulator evidence. Security-sensitive components require review/audit evidence before `PRODUCTION_READY`.

See [`../STATUS.md`](../STATUS.md) and [`OS_STANDARD_COMPONENTS.md`](OS_STANDARD_COMPONENTS.md) for the current project state and complete component coverage.
