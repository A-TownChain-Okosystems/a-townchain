# GlobusOS Evidence-Backed System Gap Matrix

**Snapshot basis:** repository documentation on `main` at inventory time.  
**Status:** DOCUMENTATION-BASED TRIAGE; SOURCE/CI VERIFICATION STILL REQUIRED.  
**Purpose:** Convert the component catalog and roadmap into ordered work with acceptance criteria and evidence requirements.

> This matrix distinguishes a roadmap-declared gap from a source-verified defect. "OPEN IN ROADMAP" means the current roadmap explicitly lists work as incomplete; it does not claim that no partial code exists. "FOUNDATION DECLARED" means the catalog declares an API/foundation, not that its implementation is correct or production-ready.

## 1. Source documents consulted

- `Cargo.toml`: workspace members include core, IPC, security, process, memory, VFS, network, devices, services, graphics, audio, package, update, runtime, identity, settings, Aurora core, diagnostics, ShivaCore API, kernel, bootloader, drivers, and shell-related modules.
- `docs/IMPLEMENTATION_STATUS.md`: overall release status is BLOCKED / not production-ready; exact-SHA verification and boot, security, integration, and recovery evidence are required.
- `docs/COMPONENT_FUNCTION_CATALOG.md`: service APIs are described as foundations; persistent storage, production network protocols, real drivers, supervisor behavior, cryptographic verification, and hardware backends remain verification/implementation boundaries.
- `docs/OS_STANDARD_COMPONENTS.md`: defines P0/P1/P2 coverage and kernel/userspace trust boundaries.
- `ROADMAP.md`: explicitly lists hardware/platform work under P1 and desktop/AI work under P2 as unchecked.

These sources are architecture and status records. This document does not replace inspection of source files, tests, workflows, PR heads, and commit-pinned CI results.

## 2. Initial gap matrix

| Priority | Workstream | Repository-declared evidence | Current triage | Acceptance evidence required |
|---|---|---|---|---|
| P0 | ShivaCore ABI adapter and capability handoff | `ROADMAP.md` lists adapter/handoff unchecked; catalog says ABI and capability lifecycle require validation | OPEN IN ROADMAP | ABI contract tests, capability transfer/revocation negative tests, kernel/service integration test, CI on exact SHA |
| P0 | Init executable and service supervisor | `ROADMAP.md` lists `globus-init` and supervisor unchecked; service catalog leaves restart/failure semantics open | OPEN IN ROADMAP | Boot-to-userspace test, dependency-order tests, crash/restart/failure-injection tests, logs tied to SHA |
| P0 | Boot/UEFI integration | `ROADMAP.md` lists x86_64 UEFI boot unchecked; implementation status says boot is blocked/not verified | OPEN / BLOCKING | Reproducible x86_64 boot artifact, emulator or hardware log, exact commit SHA, failure-path evidence |
| P0 | PCI/PCIe, DMA and IOMMU | `ROADMAP.md` lists services unchecked; standard component model defines discovery → IOMMU/DMA → MMIO/interrupts → driver | OPEN IN ROADMAP | Enumeration tests, invalid-resource tests, DMA isolation/IOMMU evidence, interrupt and reset tests on emulator/hardware |
| P0 | NVMe and persistent storage | `ROADMAP.md` lists NVMe/storage service and persistent VFS backend unchecked; catalog says VFS is currently a namespace/mount contract | OPEN IN ROADMAP | Controller/queue/DMA/completion tests, timeout/reset/error injection, partition + filesystem persistence test across reboot |
| P0 | Network driver and protocol stack | `ROADMAP.md` lists network driver and IPv4/IPv6/TCP/UDP unchecked; catalog identifies production protocols as open | OPEN IN ROADMAP | NIC or virtual NIC tests, protocol conformance, socket tests, malformed-packet and timeout tests, isolation checks |
| P0 | Security enforcement | Security foundation is declared; status requires threat-model, capability lifecycle, deny-by-default and audit validation | AUDIT / INTEGRATION REQUIRED | Deny-by-default tests, unauthorized IPC/device/file/network negative tests, revocation tests, audit and secret-redaction checks |
| P1 | USB and input | `ROADMAP.md` lists input/USB service unchecked | OPEN IN ROADMAP | USB enumeration/hotplug tests, malformed device handling, keyboard/mouse event path, device removal recovery |
| P1 | Display/GPU and compositor | `ROADMAP.md` lists display/GPU service and compositor/window manager unchecked; graphics catalog says hardware backend is open | OPEN IN ROADMAP | Framebuffer/software fallback, display mode and hotplug tests, GPU reset/error handling, compositor integration evidence |
| P1 | Audio backend | `ROADMAP.md` lists audio backend unchecked; catalog lists hardware backend as open | OPEN IN ROADMAP | Playback/capture path, device hotplug, permissions, stream teardown and device-failure tests |
| P1 | Package/update cryptographic verification | `ROADMAP.md` lists cryptographic verification unchecked; package/update foundations are declared | OPEN IN ROADMAP | Signature and provenance verification, tamper rejection, dependency tests, interrupted update and rollback/power-loss tests |
| P1 | Identity/session and secure storage | Identity is interface-documented; secure storage and cryptographic identity binding remain open | INTERFACE ONLY / VERIFY | Authentication lifecycle, session isolation, credential/key boundary tests, recovery and failure cases |
| P1 | Diagnostics and observability | Catalog defines the diagnostics boundary; implementation status requires exact-SHA evidence | VERIFY SOURCE AND TESTS | Structured logs, access control, rotation, redaction, service health, crash evidence and tests |
| P1 | Desktop shell and settings | `ROADMAP.md` lists shell, settings/identity UI and Aurora IPC bridge unchecked | OPEN IN ROADMAP | Session lifecycle, keyboard navigation, file picker/settings integration, IPC authorization and desktop smoke tests |
| P1 | Recovery and signed release images | `ROADMAP.md` lists recovery environment and reproducible system images unchecked | OPEN IN ROADMAP | Recover from failed boot/update, image reproducibility, signature validation and rollback evidence |
| P2 | VM/container isolation | `ROADMAP.md` lists this under optional ecosystem work only if promoted | OPTIONAL | Explicit product decision, isolation threat model, escape/permission negative tests and resource quotas |
| P2 | Additional architectures | `ROADMAP.md` lists aarch64 boot integration as platform work; architecture status needs target-specific evidence | OPEN / TARGET-SPECIFIC | Architecture-specific boot, CI, hardware/emulator profile and documented support boundary |

## 3. Required inventory actions before code changes

- [ ] Enumerate workspace members and compare each manifest with the component/function catalog.
- [ ] For every P0 row, inspect source implementation and tests; record exact paths and public APIs.
- [ ] Inspect open PR heads and their latest CI status; do not transfer findings from an old PR SHA to current `main`.
- [ ] Classify each row as `MISSING`, `SKELETON`, `IMPLEMENTED_UNTESTED`, `TESTED`, `AUDIT_REQUIRED`, or `BLOCKED`, with links to evidence.
- [ ] Record owners and dependencies, then split work into small PRs aligned with the owning crate/repository.
- [ ] Run and record applicable workspace gates on the same commit: `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo test --workspace --all-targets`, and `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Run emulator/hardware checks for boot, DMA, NVMe, network and graphics where relevant; a successful Rust build is not hardware proof.

## 4. Implementation order

1. **P0-A — Kernel contract and service startup:** ABI/capability handoff, init/supervisor, boot path.
2. **P0-B — Hardware safety:** PCI/ACPI resource discovery, IOMMU/DMA, interrupt and driver-service contracts.
3. **P0-C — Persistence:** NVMe/block layer, partitions, filesystem backend, VFS integration and reboot persistence.
4. **P0-D — Networking:** NIC path, protocol stack, sockets and failure handling.
5. **P0-E — Security validation:** capability lifecycle, authorization, audit, key boundaries and negative tests.
6. **P1 — User-visible platform:** input, graphics/compositor, audio, settings, shell, packages/updates and recovery.
7. **P2 — Optional features:** only after an explicit roadmap decision and P0/P1 acceptance evidence.

## 5. Definition of done

A row is not complete until its source path, API/IPC contract, owner, tests, security boundary, exact-SHA CI evidence and known limitations are recorded. Hardware-dependent rows additionally require reproducible emulator or target-hardware evidence. Update the component catalog and implementation status only after those records exist.

**Initial result:** roadmap-based triage is established; source-by-source implementation state and current CI state remain NOT VERIFIED. No claim is made that this matrix itself implements the listed system components.
