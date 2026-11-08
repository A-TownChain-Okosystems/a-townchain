# GlobusOS component and function catalog

**Snapshot date:** 2026-10-11  
**Scope:** repository-level inventory of documented subsystem boundaries. This is an ownership/API catalog, not proof that every feature is fully implemented.

## Canonical components

| Component / path | Responsibility | Declared state | Key function groups | Open verification / implementation boundary |
|---|---|---|---|---|
| `system/` workspace | Build and integrate OS service crates | IMPLEMENTED | workspace build, tests, lint | Exact-SHA gates must pass as a coherent set |
| `system/system-core` / `globus-system-core` | OS lifecycle and kernel-facing identity | FOUNDATION IMPLEMENTED | lifecycle, identity boundary | Validate ABI contract against canonical ShivaCore |
| `system/ipc` / `globus-ipc` | IPC endpoint and message abstractions | FOUNDATION IMPLEMENTED | message transport contract, endpoint API | Capability checks and kernel handoff need integration evidence |
| `system/security` / `globus-security` | Authorization and capability policy | FOUNDATION IMPLEMENTED | deny-by-default authorization | Threat model, capability revocation/lifecycle, audit evidence |
| `system/process` / `globus-process` | Process and thread lifecycle | FOUNDATION IMPLEMENTED | process/thread abstractions | Scheduler integration and ownership correctness |
| `system/memory` / `globus-memory` | Address-space and page policy | FOUNDATION IMPLEMENTED | mapping and memory policy | Mapping lifetime, rollback, page-table mutation, hardware evidence |
| `system/vfs` / `globus-vfs` | Filesystem namespace and mount contract | FOUNDATION IMPLEMENTED | mount and namespace interfaces | Persistent backend and filesystem implementation |
| `system/net` / `globus-net` | Network policy and socket boundary | FOUNDATION IMPLEMENTED | network/socket API | Drivers and production IPv4/IPv6/TCP/UDP implementation |
| `system/devices` / `globus-devices` | Device/driver registry boundary | FOUNDATION IMPLEMENTED | device registration and lookup | PCI/PCIe, DMA/IOMMU and actual driver services |
| `system/services` / `globus-services` | Service lifecycle and dependency order | FOUNDATION IMPLEMENTED | lifecycle, dependency sequencing | Init executable, supervisor, restart and failure semantics |
| `system/graphics` / `globus-graphics` | Display/compositor/GPU boundary | FOUNDATION IMPLEMENTED | graphics/taskbar APIs | Hardware backend and current formatting/compile failures on affected PR heads |
| `system/audio` / `globus-audio` | Audio service boundary | FOUNDATION IMPLEMENTED | audio API | Hardware backend |
| `system/package` / `globus-package` | Package metadata verification model | FOUNDATION IMPLEMENTED | package metadata checks | Production cryptographic signature verification |
| `system/update` / `globus-update` | A/B update and rollback state machine | FOUNDATION IMPLEMENTED | update, rollback states | End-to-end power-loss, integrity, and recovery tests |
| `system/runtime` / `globus-runtime` | Integrated userspace runtime | FOUNDATION IMPLEMENTED | runtime composition | End-to-end service lifecycle evidence |
| `system/identity/` | Identity, authentication, wallet-binding, recovery contract | INTERFACE DOCUMENTED | login/session, identity binding, recovery flow | Secure storage and cryptographic binding verification |
| `modules/atc-shivacore/kernel` | Canonical ShivaCore kernel source in this repository | IMPLEMENTED / BLOCKED | capability, IPC, scheduler, memory, syscall, HAL and boot integration | Current compiler/boot failures on PR heads listed in status register |
| `sdk/` | Developer SDK and application-facing integration | FOUNDATION / NOT FULLY VERIFIED | SDK build and API surface | Validate published API and compatibility independently |
| Aurora service integration | AI interaction through userspace policy/service boundaries | PLANNED | intent, policy, capability check, approval, tool/service calls | Never place Aurora authority or model execution inside the kernel TCB |
| ATC integration boundary | A-TownChain, ATC-VM and ATCLang integration via userspace | PLANNED | host/service API | Canonical VM/algorithm remain in their designated repositories |

> Path names above follow the repository's declared architecture and README. If a crate's physical directory or Cargo package name differs, reconcile this table against the current workspace manifests before treating the mapping as mechanically verified.

## Function catalog by system responsibility

| Function group | Owning boundary | Acceptance checkpoint |
|---|---|---|
| Boot and hardware handoff | bootloader / ShivaCore / HAL | Reproducible UEFI and boot test evidence |
| Process and thread management | ShivaCore + process service | Scheduler, context ownership, and lifecycle tests |
| Virtual memory and mappings | ShivaCore + memory service | Page-table mutation, lifetime, rollback and isolation tests |
| IPC and service communication | ShivaCore IPC + `globus-ipc` | Authorization, endpoint lifecycle, malformed-message and failure tests |
| Capability authorization | ShivaCore TCB + security service | Deny-by-default and least-authority tests |
| Device discovery and drivers | HAL + device services | PCI, DMA/IOMMU and per-device driver tests |
| Storage and filesystems | VFS + storage services | Persistence, corruption handling and recovery tests |
| Networking | network service + drivers | Protocol conformance, isolation and negative tests |
| Identity and sessions | identity service | Authentication lifecycle and identity-binding tests |
| Wallet integration | isolated wallet service | Key isolation, recovery and signing-boundary tests |
| Package installation | package service | Signature, provenance and tamper-rejection tests |
| Updates and rollback | update/recovery services | Atomicity, interrupted-update and rollback tests |
| Graphics and audio | media services | Backend integration and resource isolation |
| Diagnostics and audit | diagnostics/service boundary | Structured events, no secret leakage, retention/access policy |
| Aurora tools and models | Aurora userspace service | Explicit policy, capability, approval and audit before tool execution |
| ATC execution integration | userspace host/service | Stable API; no chain execution authority in the kernel |

## Source-of-truth rules

- ShivaCore kernel source remains under `modules/atc-shivacore/kernel` in this repository.
- GlobusOS owns OS userspace/service integration, not the canonical A-TownChain VM or consensus implementation.
- The canonical ATC-VM remains in `a-townchain/components/vm`; the canonical algorithm/consensus implementation remains in `a-townchain/components/algorithm`.
- Aurora is not the kernel TCB, consensus authority, or canonical ATC-VM.
- Update this catalog from workspace manifests and source inventories during each census. Do not infer implementation completeness from a module name or roadmap checkbox.
