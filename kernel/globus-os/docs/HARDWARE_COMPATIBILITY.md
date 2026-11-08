---
document_id: ATC-DOC-OS-HW-001
title: Hardware Compatibility and Driver Boundary
version: 0.1.0
status: proposed
owner: A-TownChain-Okosystems
updated: 2026-10-11
---

# Hardware Compatibility and Driver Boundary

## 1. Purpose and status

This document defines the compatibility contract and staged enablement policy for AMD, Intel, and NVIDIA hardware across GlobusOS, ShivaCore, and Aurora AI. It is a specification and implementation plan, not evidence that any listed device or driver is currently supported.

A device is supported only when its exact model/revision, firmware, driver/backend version, test environment, and evidence are recorded and the required gates pass. Vendor or architecture-family recognition alone is not support evidence.

## 2. Ownership and trust boundaries

- **ShivaCore kernel / TCB:** CPU bring-up, exception/interrupt foundations, memory protection, scheduler, IPC, and capability enforcement. Keep vendor-specific GPU stacks and AI runtimes out of the TCB.
- **GlobusOS:** boot and platform discovery, PCI/PCIe enumeration, device registry, driver lifecycle, DMA/IOMMU policy, power/device services, and authorized device interfaces.
- **Aurora AI / ATC AI Runtime:** backend selection and scheduling for CPU, GPU, and NPU workloads through GlobusOS-authorized interfaces.
- **ATC standards / ecosystem:** normative cross-repository contracts and evidence/integration checks only; no duplicate canonical driver implementation.

Hardware access must follow: model/agent proposal -> policy -> capability check -> optional approval -> authorized GlobusOS service -> device backend. No ambient device authority is permitted.

## 3. Vendor and architecture coverage

The following are target families, not a declaration of support.

| Vendor | Device class | Example architecture / family labels | Intended role | Initial status |
|---|---|---|---|---|
| AMD | x86-64 CPU | Zen-family Ryzen and EPYC products | Boot, SMP, memory, compute | SPECIFIED; exact models unverified |
| Intel | x86-64 CPU | Core/Core Ultra and Xeon families | Boot, SMP, memory, compute | SPECIFIED; exact models unverified |
| AMD | GPU / accelerator | Radeon and Instinct families | Display, general compute, AI workloads | PLANNED; backend and device coverage unverified |
| Intel | GPU / accelerator | Integrated graphics and Arc families | Display and general compute | PLANNED; backend and device coverage unverified |
| NVIDIA | GPU / accelerator | GeForce RTX, RTX PRO, data-center accelerators; architecture families such as Ampere, Ada, Hopper, Blackwell, and future families | Display and AI/HPC compute | PLANNED; backend and device coverage unverified |
| AMD / Intel | NPU | Only products that expose a documented, usable NPU interface | Local AI inference | PLANNED; platform-specific interface required |

Do not infer feature availability from a family name. Confirm it from the processor/device documentation, PCI IDs, firmware, driver support, and tests. Product roadmaps (including future or newly announced architectures) must not be treated as tested hardware.

## 4. Platform milestones

### P0 — Boot and CPU baseline

- x86-64 CPU identification with CPUID and safe feature gating.
- UEFI boot path and ACPI table validation.
- APIC, timer, exception, interrupt, and SMP bring-up.
- Distinguish AMD and Intel virtualization/IOMMU mechanisms behind capability-oriented interfaces.
- Maintain QEMU coverage separately from real-hardware qualification.

Exit criteria: clean build, unit tests, boot evidence tied to an exact commit, and a recorded CPU/firmware/QEMU profile. Passing QEMU does not certify physical hardware.

### P1 — Device discovery and isolation

- PCI/PCIe enumeration and validated BAR/MMIO mapping.
- DMA mapping policy and IOMMU domain lifecycle.
- NUMA and memory-topology discovery where available.
- Driver/service isolation, device reset/error paths, and resource revocation.
- Secure Boot, measured boot, TPM/TEE and platform attestation only where implementation and evidence exist.

Exit criteria: negative tests for invalid resources, unauthorized DMA, stale/revoked capabilities, malformed firmware tables, and failed device initialization.

### P2 — GPU backends

- Define a vendor-neutral device, buffer, command submission, synchronization, and error contract.
- Keep AMD, Intel, and NVIDIA adapters separate behind the contract.
- Treat ROCm and CUDA as optional backend integrations, not kernel dependencies or universally available interfaces.
- Record licensing, packaging, firmware, kernel-interface, and version constraints for each adapter.
- Add device-loss, reset, timeout, allocation failure, and cleanup tests.

Exit criteria: backend-specific conformance tests and real-device evidence for each claimed model; do not promote an entire vendor family from one successful card.

### P3 — Aurora AI compute

- Stable ATC Model ABI / ATC AI Runtime boundary for CPU, GPU, and NPU execution.
- GlobusOS-mediated device discovery and capability-checked allocation.
- Workload limits for memory, execution time, queue depth, and cancellation.
- Record backend, driver, model, precision mode, and runtime versions for reproducibility.
- Keep AI execution, model loading, and vendor runtime code outside the ShivaCore TCB.

Exit criteria: deterministic contract tests where applicable, numerical-tolerance tests for non-deterministic accelerators, authorization tests, resource-limit tests, and exact-commit CI evidence.

### P4 — Hardware qualification

- Maintain a model/revision-level matrix.
- Run QEMU tests and real-hardware tests as distinct test classes.
- Record firmware, BIOS settings, kernel build, driver/backend, test commands, logs, and exact source SHA.
- Mark hardware VERIFIED only after the required tests and security gates pass.

## 5. Machine-readable registry contract

Each hardware qualification record must contain:

- `vendor`, `device_class`, `product_family`, `architecture`, `generation`
- `vendor_id`, `device_id`, and `subsystem_id` for PCI devices when applicable
- `cpu_signature` for CPU qualification when applicable
- `firmware_version`, `driver_version`, `backend_version`, and `runtime_version` where applicable
- `capabilities`, `security_features`, and `known_limitations`
- `test_environment`, `test_commands`, `test_status`, `evidence_sha`, and evidence/log references
- `last_verified_at` and an owner for requalification

Allowed lifecycle states:

- `UNTESTED`: record exists, no implementation claim.
- `SPECIFIED`: contract and intended coverage are documented.
- `IMPLEMENTED`: source implementation exists, not yet fully verified.
- `TESTED`: relevant tests passed, CI/evidence gate not yet complete.
- `VERIFIED`: exact-SHA evidence and required gates passed for this exact profile.
- `BLOCKED`: a required gate failed or a prerequisite is missing.
- `RESIDUAL`: prior verification is no longer valid for the changed source/profile or an unresolved gap remains.

Never set `VERIFIED` based only on CPUID/PCI detection, a successful compile, a PR merge, or a different device from the same vendor.

## 6. Evidence and CI gates

For each candidate profile, preserve the chain:

`repository + exact commit SHA -> workflow run -> job -> step -> exit status/log -> profile record`.

Required checks include formatting, build, unit tests, static analysis where configured, contract/conformance tests, security boundary tests, and hardware evidence for the claim being made. A missing log or mismatched SHA fails closed. A documentation-only PR cannot mark any device implemented or verified.

## 7. Initial implementation order

1. Inventory existing boot, ACPI, interrupt, PCI, DMA, IOMMU, and device-service code in GlobusOS and ShivaCore.
2. Map each source symbol and test to P0/P1 contracts; label missing components explicitly.
3. Define the device-service ABI before implementing vendor-specific drivers.
4. Implement and test CPU/platform discovery on QEMU, then qualify selected AMD and Intel physical systems.
5. Implement GPU backends independently and qualify individual device IDs; do not claim blanket vendor support.
6. Integrate Aurora through the ATC AI Runtime contract only after the GlobusOS device boundary is enforced.

## 8. Non-goals

- This document does not claim production readiness for GlobusOS or ShivaCore.
- It does not promise support for every AMD, Intel, or NVIDIA generation.
- It does not require CUDA or ROCm as mandatory dependencies.
- It does not move Aurora AI, GPU drivers, or blockchain authority into the ShivaCore TCB.
- It does not replace ATC standards as the cross-repository normative authority.
