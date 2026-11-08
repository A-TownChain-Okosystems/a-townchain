# Windows OS Feature Reference and GlobusOS Gap-Analysis Contract

**Document type:** reference and assessment framework  
**Status:** `DEVELOPMENT BASELINE` — not an implementation claim  
**Reference snapshot:** 2026-10-19  
**Canonical work item:** [Issue #51 — Betriebssystem-Grundfunktionen katalogisieren, SSOT abgleichen und verifizieren](https://github.com/A-TownChain-Okosystems/globus-os/issues/51)  
**Canonical inventories:** [OS standard component coverage](OS_STANDARD_COMPONENTS.md), [component/function catalog](COMPONENT_FUNCTION_CATALOG.md), [implementation status and release gates](IMPLEMENTATION_STATUS.md)

## 1. Purpose and authority

This document translates a Windows-like general-purpose operating-system feature set into a structured reference for assessing GlobusOS coverage. Windows is a breadth-of-function reference only; it is not a required implementation model and does not define GlobusOS internals.

This document is **not** a second component catalog, implementation SSOT, normative ABI, or evidence that a feature exists. Update the canonical component/function catalog and implementation-status records when an assessment has been performed. Keep this reference focused on scope, ownership boundaries, and acceptance evidence.

## 2. Architecture and canonical ownership

| Domain | Canonical responsibility | Boundary |
|---|---|---|
| Kernel / TCB | ShivaCore microkernel | Scheduling, address-space isolation, privilege transitions, exception handling, syscalls, IPC primitives and capability enforcement belong to the canonical ShivaCore kernel repository/path. |
| Boot and hardware integration | GlobusOS platform plus defined ShivaCore interfaces | Firmware handoff, boot orchestration, hardware discovery, HAL-facing contracts, interrupt/device integration, DMA/IOMMU and power-state coordination must have explicit ownership per interface. |
| Core OS services | GlobusOS user space | Init/service supervision, clocks/timers, VFS/storage, networking, device services, identity/session, policy, audit, diagnostics and recovery. |
| Security and servicing | Dedicated GlobusOS security/platform services | Trust anchors, Secure Boot integration, TPM/TEE providers, key isolation, package/update signature verification, atomic activation and rollback. Kernel enforcement stays in ShivaCore where required. |
| Desktop and app platform | Native GlobusOS services and SDK | Shell, compositor/window management, input, graphics/audio APIs, accessibility, application lifecycle, loader/runtime and versioned public APIs. |
| Optional isolation / compatibility | Explicit runtime and isolation layers | Virtual machines, containers, compatibility layers, quotas and resource boundaries; do not imply these are kernel features unless the canonical design says so. |
| Ecosystem integration | Existing canonical repositories | Aurora AI remains outside the kernel TCB. A-TownChain, ATC-VM, ATCLang and wallet functionality must use their existing SSOTs and explicit integration contracts; do not duplicate their core implementations in GlobusOS. |

## 3. Reference feature domains

The machine-readable companion `WINDOWS_OS_FEATURE_REFERENCE.json` is the structured counterpart. It lists domains, representative functions, suggested priority, canonical owner, acceptance evidence and current assessment state.

- **P0 — Kernel security and isolation:** boot/exception safety, process and thread lifecycle, scheduler, address spaces, page permissions, syscall boundary, IPC and capability authorization.
- **P0 — Boot-to-user-space:** deterministic initialization, service supervision and dependency ordering, monotonic time/timers, fault containment, watchdog behavior and recovery path.
- **P1 — Hardware and core services:** CPU/SMP, interrupt routing, firmware/ACPI, PCI/PCIe, MMIO, DMA/IOMMU, device lifecycle/hotplug, storage/NVMe, VFS/filesystems, network stack and sockets.
- **P1 — Security and lifecycle:** identity/authentication/sessions, authorization, secrets/key isolation, audit and retention, signed packages, SBOM/provenance, update activation, rollback, backup/restore and crash diagnostics.
- **P2 — Desktop and applications:** display/compositor/window manager, input, GPU/graphics, audio, shell/launcher, app loader/lifecycle, SDK/API compatibility, accessibility and localization.
- **P2 — Extended platform:** virtualization, containers, compatibility, resource quotas, SMP/NUMA scaling, power management, suspend/resume, observability and optional high availability.
- **Cross-cutting:** privacy-by-default, explicit telemetry policy, data minimization, local diagnostics, supply-chain integrity and reproducible evidence.

Priorities are planning recommendations, not statements of existing implementation or an authorization to bypass current kernel/security acceptance gates.

## 4. Required assessment record

Every assessed feature must be linked to a canonical component and include, where applicable:

1. Stable feature ID and domain.
2. Canonical owner repository and source path.
3. Exact commit SHA; source symbol and blob SHA where relevant.
4. Test names and results, including negative/security tests.
5. CI evidence at the same commit: workflow run → job → step → exit code/logs.
6. Integration and QEMU/hardware-profile evidence where the claim depends on boot, hardware, or cross-process behavior.
7. Known limitations, residual risk, dependencies and an accountable next action.

Do not copy inferred source paths or symbols into the catalog. Resolve them from the repository at the assessed commit.

## 5. State vocabulary and release rules

Use the repository's canonical status vocabulary and keep distinct states distinct:

- `SPECIFIED`: requirements/contract documented.
- `IMPLEMENTED`: source implementation or interface exists.
- `TESTED`: named tests executed with recorded outcomes.
- `CI_VERIFIED`: applicable CI gates passed for the exact assessed commit.
- `INTEGRATED`: interaction with the owning subsystem is demonstrated.
- `E2E_VERIFIED`: end-to-end acceptance evidence exists for the claimed scenario.
- `RESIDUAL`: known limitation or risk remains.
- `BLOCKED`: required dependency or evidence is missing, or a gate failed.
- `EXCLUDED`: explicitly out of scope, with rationale.
- `NOT_ASSESSED`: no assessment evidence has been collected.

A green CI run alone does not establish end-to-end verification or production readiness. Missing evidence must not be treated as a pass. For security-critical functionality, require negative tests and observable enforcement, not merely API presence. Record evidence against an immutable commit SHA.

## 6. Assessment workflow

1. Read Issue #51 and the canonical catalogs before opening new work.
2. Pin the current repository HEAD and identify the applicable owner repository for each feature.
3. Map reference features to existing catalog entries; do not create duplicates.
4. Inspect source, tests, workflows, integration boundaries and exact-SHA artifacts.
5. Record confirmed findings and evidence; mark unknowns `NOT_ASSESSED` or `BLOCKED` according to the reason.
6. Create a minimal patch request only for verified gaps, with acceptance criteria and test evidence requirements.
7. Keep release readiness separate from feature-count or catalog completeness.

## 7. Non-goals

- No claim that Windows features are all necessary for the first GlobusOS release.
- No claim that listed GlobusOS capabilities are implemented, tested, secure or production-ready.
- No new duplicate implementation SSOT.
- No changes to ShivaCore or other canonical repositories implied by this reference.
- No release/merge approval granted by cataloging alone.

## 8. Related sources

- [Issue #51 — canonical OS feature inventory and verification work](https://github.com/A-TownChain-Okosystems/globus-os/issues/51)
- [Standard OS component coverage](OS_STANDARD_COMPONENTS.md)
- [Component and function catalog](COMPONENT_FUNCTION_CATALOG.md)
- [Implementation status and release gates](IMPLEMENTATION_STATUS.md)
- [System architecture](SYSTEM_ARCHITECTURE.md)
