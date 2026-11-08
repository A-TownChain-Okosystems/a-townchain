---
document_id: ATC-DOC-GLOOS-HW-P0-001
title: "P0 Hardware Source Inventory and CPU Baseline"
version: 0.1.0
status: draft
owner: A-TownChain-Okosystems
updated: 2026-10-11
---

# P0 Hardware Source Inventory and CPU Baseline

## Scope and evidence boundary

This inventory is pinned to the inspected repository tree at base commit
`329c6a83050e12d5f69d3c6fcbfddbaa9fffa40e`. Source presence is not runtime
evidence. No claim is made here that GlobusOS boots on physical hardware or that
PCIe, IOMMU, GPU, or NPU hardware is production-supported.

## Source inventory

| Area | Current source | Observed implementation | P0 gap / required evidence |
|---|---|---|---|
| x86 CPU identification | `modules/atc-shivacore/kernel/src/hal/cpu.rs` | CPUID vendor/family/model/stepping and selected SSE2/NX/APIC/x2APIC/syscall/1-GiB-page/invariant-TSC bits; two unit tests | Add deterministic feature-decoding and unsupported-leaf tests; execute kernel crate tests on exact SHA. CPUID detection is x86_64-specific. |
| Kernel entry / boot | `modules/atc-shivacore/kernel/src/main.rs`, `modules/atc-shivacore/boot/src/main.rs` | Boot entry and boot utility source exist | UEFI/Secure Boot/ACPI end-to-end evidence is not present in this source inventory; do not infer hardware boot from entry-point presence. |
| ACPI | `system/devices/src/acpi.rs` | RSDP signature/checksum/length validation and bounded MCFG ECAM address calculation | Add adversarial malformed-length and ECAM-overflow/bounds tests; prove firmware table memory is safely supplied by a trusted boot boundary. |
| Interrupt / APIC | `system/devices/src/apic.rs`, `system/devices/src/interrupt.rs`, `system/devices/src/msi.rs` | Device-service modules are registered | Inspect concrete controller paths and hardware execution separately; module registration is not proof of live interrupt routing. |
| SMP | `system/devices/src/smp.rs`, `modules/atc-shivacore/kernel/src/smp.rs` | CPU state/registration contract and kernel scheduling/SMP source exist | Establish actual AP startup, per-CPU initialization, synchronization, and QEMU multi-vCPU evidence before claiming multiprocessor operation. |
| PCI/PCIe | `system/devices/src/pci.rs` | Address/ID/BAR data types, config-access and MMIO mapping traits, BAR validation | No evidence here of enumerating a live PCI hierarchy or safely programming BARs. Add address/BAR edge tests and a separately authorized hardware backend. |
| IOMMU/DMA | `system/devices/src/iommu.rs` | IOMMU trait and `SoftwareIommu` model for domains, device assignment, and overlap validation | The software model is not hardware DMA isolation. Require vendor-specific IOMMU programming, invalidation, failure rollback, and DMA isolation tests before device DMA is enabled. |
| Device-service boundary | `system/devices/src/lib.rs` | Device registry types and ACPI/PCI/IOMMU/SMP module exports | Keep raw config-space access, MMIO mapping, and DMA authority behind privileged HAL/service APIs; never expose ambient hardware access to Aurora or ordinary services. |
| VMM dependency | `modules/atc-shivacore/kernel/src/vmm.rs` | Current physical-address limit is a documented 48-bit baseline | Do not dynamically widen physical-address handling until CPUID width parsing, page-table constraints, overflow checks, and rollback/lifetime requirements are tested. |

## Findings and risk register

- **P0-HW-001 — CPUID test coverage (open):** current tests exercise the host CPU and vendor-string parsing, but do not deterministically test individual feature-bit decoding or missing-leaf behavior.
- **P0-HW-002 — ACPI input robustness (open):** add malformed RSDP length/checksum cases and ECAM address overflow/boundary tests. Validate data before any firmware-derived address is dereferenced.
- **P0-HW-003 — CPU/SMP hardware evidence (open):** source types and scheduling logic do not establish that application processors were started and initialized on real hardware or a multi-vCPU emulator.
- **P0-HW-004 — PCI enumeration (open):** existing contracts do not establish a complete bus scan, multifunction traversal, bridge recursion, or safe BAR sizing.
- **P0-HW-005 — IOMMU hardware isolation (open):** `SoftwareIommu` is a model only. No device DMA may be considered isolated without actual IOMMU programming and negative tests.
- **P0-HW-006 — TCB boundary (invariant):** discovery and policy should remain separable. ShivaCore owns only the minimum privileged operations; driver logic, Aurora compute backends, and device policy remain in service space.

## P0 acceptance gates

1. CPU feature decoding is deterministic and tested without depending on the host CPU model.
2. ACPI/ECAM malformed inputs are rejected without panic, wraparound, or unchecked dereference.
3. `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`,
   `cargo test --workspace --all-targets`, and
   `cargo clippy --workspace --all-targets -- -D warnings` run against the exact candidate SHA; record each result independently.
4. QEMU x86_64 boot evidence records the exact image, QEMU version/arguments, serial log, exit status, and CPU/vCPU configuration.
5. Hardware claims remain **UNVERIFIED** until physical-device evidence exists. No CI gate may be weakened to obtain a green result.

## P1 entry criteria

Begin PCI enumeration and device isolation only after the CPU/ACPI P0 tests pass and the device-service authorization boundary is reviewed. GPU/NPU backends are downstream consumers and are explicitly out of scope for this P0 change.
