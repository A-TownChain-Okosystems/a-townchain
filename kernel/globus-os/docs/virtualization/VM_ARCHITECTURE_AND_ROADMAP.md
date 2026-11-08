# GlobusOS Guest OS Virtualization Architecture

**Status:** Draft architecture and implementation contract  
**Component ID:** `COMP-GUEST-OS-VIRT`  
**Priority:** P1 platform capability  
**Owner:** GlobusOS platform; ShivaCore owns only the minimum privileged primitives  
**Readiness:** DESIGNED (proposal); implementation and runtime verification remain unverified

## 1. Goal

Run multiple independent guest operating systems concurrently in virtual machines managed by the GlobusOS desktop:

- Windows guests, subject to supported release, firmware, driver, and license requirements.
- Linux guests, beginning with a reproducible x86_64 distribution image.
- macOS guests only on supported Apple hardware and only where the relevant Apple license and guest-version requirements permit virtualization.

The VM manager must not imply that all three guest families are already bootable. Support is declared per guest profile and validated against an exact build and host capability set.

## 2. Architecture and trust boundary

```text
GlobusOS Desktop / VM Manager (unprivileged UI + policy)
        | capability-checked IPC
Virtualization Service (VM lifecycle, quotas, snapshots, device policy)
        | backend interface
Backend adapter(s): native hardware-virtualization / development emulator
        | bounded kernel APIs + IOMMU/DMA policy where available
ShivaCore (TCB: CPU state primitives, memory isolation, interrupts, scheduling)
        |
x86_64 hardware + firmware
```

The VM manager, guest configuration parser, virtual-device models, image handling, guest tools, and UI remain outside the ShivaCore TCB. The kernel must not parse guest disk images or execute guest device emulation logic.

The existing `modules/atc-shivacore/service_space/src/vm.rs` implements the ShivaVM contract/bytecode execution domain; it is not a hardware hypervisor. The existing `modules/atc-shivacore/kernel/src/vmm.rs` contains virtual-memory mapping and validation primitives; its existence alone does not prove VMX/SVM entry, guest-state management, EPT/NPT, IOMMU isolation, or successful guest boot.

## 3. Backend decision

Use a GlobusOS-owned, backend-neutral VM manager and stable internal backend contract. Keep platform policy, lifecycle, resource limits, security, desktop integration, snapshot metadata, and recovery under GlobusOS control.

Backend stages:

1. **Development and CI:** use QEMU as a test harness where available. QEMU TCG is a portable functional/emulation baseline; host KVM may accelerate Linux-host tests only. Neither is evidence of a native GlobusOS hypervisor.
2. **Native hardware-accelerated backend:** implement or integrate a controlled backend for Intel VT-x and AMD-V/SVM behind the same contract. This requires VMX/SVM enablement, VM control structures, EPT/NPT, guest/host state transitions, interrupt virtualization, and robust teardown. It is not considered implemented by this document.
3. **Optional compatibility backend:** may be added only if it preserves the same capability, quota, audit, and lifecycle rules. A backend must not become an uncontrolled second source of VM policy.

This design avoids coupling the GlobusOS API to QEMU or Linux KVM while permitting those tools to validate guest configuration and end-to-end behavior during development.

## 4. Internal component contract

The first implementation must define a single canonical crate/service, not duplicate managers in the kernel, desktop, and service-space trees. Required operations:

- `create_vm(config, capability)`: validate profile, image references, CPU/memory limits, and authorization before allocating resources.
- `start_vm(id)`, `pause_vm(id)`, `resume_vm(id)`, `shutdown_vm(id, mode)`, `destroy_vm(id)`: explicit lifecycle transitions with idempotent error handling.
- `list_vms()`, `get_vm_state(id)`, `get_vm_metrics(id)`: report state without granting guest control.
- `attach_disk(id, image, mode)`, `attach_network(id, policy)`, `attach_device(id, capability)`: require typed capabilities and validate backend support.
- `create_snapshot(id)`, `restore_snapshot(id, snapshot)`: record VM configuration, virtual-device state, disk lineage, and compatibility version; never claim a crash-consistent snapshot unless storage and device state are coordinated.
- `export_diagnostics(id)`: emit bounded, redacted logs and exact backend/guest/host version metadata.

All calls must return typed errors; unknown VM IDs, unsupported guest profiles, malformed images, resource exhaustion, and denied capabilities fail closed.

## 5. Resource and isolation requirements

- Per-VM vCPU, RAM, virtual-disk, I/O, and network quotas with checked arithmetic and explicit host reserve.
- Guest memory must be isolated from the host and other VMs; no shared writable mappings by default.
- Device assignment and DMA must be denied unless a validated IOMMU/DMA isolation path is active. Never infer safe passthrough from PCI enumeration alone.
- Virtual networking defaults to a restricted NAT or isolated network profile; bridged networking requires explicit authorization.
- Host folders, clipboard, drag-and-drop, USB/GPU passthrough, and guest-agent channels are opt-in capabilities, disabled by default.
- VM lifecycle and resource changes are audited. Guest crashes must not crash the host or corrupt other VMs.
- Image import verifies format, size, sparse-file bounds, and path handling. Untrusted guest images are treated as hostile input.
- Secure Boot, vTPM, nested virtualization, GPU acceleration, and live migration are separately gated features, not implied by a VM boot.

## 6. Guest profile matrix

| Guest profile | First target | Requirements and constraints | Acceptance evidence |
|---|---|---|---|
| Linux x86_64 | First end-to-end guest | UEFI/BIOS profile as selected, virtio or explicitly emulated storage/network, serial diagnostics | Installer/live image boots, reaches a deterministic marker, shuts down cleanly |
| Windows x86_64 | After Linux baseline | UEFI, compatible virtual storage/network/display, guest-appropriate firmware and documented licensing | Supported installer boots and reaches a marker; install/reboot/storage/network smoke tests |
| macOS | Conditional, later milestone | Supported Apple hardware and permitted guest/version combination; no promise of generic PC support | Explicitly qualified host/guest matrix and boot evidence on supported hardware |

Multiple guests must be tested concurrently after single-VM acceptance. Guest availability is always conditional on host CPU features, RAM, firmware, backend support, and license requirements.

## 7. Binding development order

The following sequence is authoritative. Later phases must not be treated as complete merely because their API or documentation exists.

1. **VM manager and API:** implement create, start, pause/resume, stop, destroy, list, state, and metrics operations with typed errors and tested lifecycle transitions.
2. **Linux first:** establish a reproducible x86_64 Linux guest boot test with pinned image digest, firmware, backend version, command line, serial marker, and exit status.
3. **Isolation and quotas:** prove memory separation, capability enforcement, host reserve, resource limits, cleanup, and guest/backend failure handling with negative tests.
4. **Native hardware virtualization:** integrate Intel VT-x and AMD-V/SVM behind one backend interface; require EPT/NPT or equivalent safe second-level translation and robust teardown.
5. **Windows and macOS qualification:** test each guest family independently against an explicit host/firmware/backend/version matrix; macOS remains conditional on supported Apple hardware and licensing.
6. **Desktop integration:** integrate VM windows and lifecycle, snapshots, restore/recovery, permissions, and resource monitoring only after core lifecycle, Linux boot, and isolation gates pass.

## 8. Implementation milestones

### V0 — Source and capability audit
- Inventory current `kernel/src/vm.rs`, `kernel/src/vmm.rs`, service-space VM code, process/memory/IPC/capability APIs, and device/DMA boundaries.
- Record canonical crate path, API owner, symbols, dependencies, and test gaps.
- Keep contract VM naming separate from guest OS virtualization.

### V1 — API and manager state machine (order 1)
- Add one canonical userspace virtualization crate/service and typed configuration/state/error models.
- Unit-test invalid transitions, duplicate IDs, resource overflow, authorization denial, and cleanup after partial failures.
- No guest-boot claim at this milestone.

### V2 — Linux boot baseline and backend contract (order 2)
- Define backend capabilities, VM handles, exit reasons, memory regions, virtual CPU state, virtual devices, and diagnostics.
- Use QEMU in CI/development only where available; pin arguments and versions in evidence.
- Run a Linux boot smoke test and record exact image digest, host, backend, command line, serial output, and exit status.

### V3 — Isolation and concurrency (order 3)
- Prove VM A cannot access VM B memory, host-only paths, or unauthorized devices.
- Exercise resource limits, VM termination, backend failure, and simultaneous VM workloads.
- Add hostile-image tests and fuzz the configuration/image parser.

### V4 — Native acceleration (order 4)
- Implement Intel VT-x and AMD-V/SVM capability detection and backend initialization.
- Require EPT/NPT and safe teardown before exposing hardware-accelerated guests.
- Test VM-entry failure, invalid guest state, interrupt delivery, memory pressure, and host recovery.

### V5 — Windows and macOS qualification (order 5)
- Qualify supported Windows releases independently, including installer, reboot, storage, network, and display smoke tests.
- Qualify macOS only against an explicit permitted Apple host/guest matrix.
- Publish support only for tested profiles; untested profiles remain experimental/unsupported.

### V6 — Desktop integration and snapshots (order 6)
- Integrate VM windows, lifecycle controls, permissions, notifications, and resource monitoring in the native GlobusOS desktop.
- Add snapshots and recovery only with explicit consistency guarantees, disk lineage metadata, and interruption/recovery tests.

## 9. Required automated tests

1. Configuration rejects negative/overflowing resource values and unsupported CPU features.
2. Capability-denied create/start/attach/snapshot operations make no state changes.
3. Failed backend initialization releases all acquired memory, handles, and device claims.
4. VM A cannot read/write VM B or host-protected memory.
5. Quotas prevent overcommit beyond configured policy and preserve host reserve.
6. Guest shutdown, forced stop, backend crash, and host service restart have deterministic state transitions.
7. Disk image traversal, oversized metadata, malformed formats, sparse-file abuse, and corrupt snapshots are rejected.
8. Network and passthrough defaults are deny-by-default.
9. Two or more Linux VMs run concurrently; resource limits and cross-VM isolation remain enforced.
10. End-to-end boot tests bind evidence to exact source SHA, backend version, guest image digest, CPU model, firmware, arguments, logs, and exit code.
11. Windows and macOS support claims each require their own guest-specific evidence; Linux success is not a proxy.
12. QEMU CI success does not certify native GlobusOS acceleration or physical hardware.

## 10. Release gates

The component remains **BLOCKED for production use** until:
- the backend decision and host/guest compatibility matrix are approved;
- API/state-machine tests and isolation negative tests pass on the exact candidate SHA;
- at least one Linux guest boots end-to-end through the selected backend;
- concurrency, quota enforcement, recovery, and security review are evidenced;
- Windows/macOS are marked supported only after their independent qualification;
- CI logs and artifacts identify exact commit, image digest, firmware, backend, and exit status.

Do not weaken required CI or infer success from source presence, documentation, or a library build.
