# GlobusOS Desktop Implementation Plan

**Status:** PROPOSED — implementation and release evidence are not implied by this plan  
**Repository:** `A-TownChain-Okosystems/globus-os`  
**Target:** x86_64 QEMU and real PCs; native GlobusOS desktop; offline-first local operation; native applications before compatibility layers  
**Planning baseline:** 2026-10-19

## 1. Evidence rules

Use these status labels consistently:

- **IMPLEMENTED**: source or interface exists.
- **VERIFIED**: the exact commit SHA passed the applicable checks, with run/job/log evidence recorded.
- **BLOCKED**: a known dependency, failed gate, or missing evidence prevents acceptance.
- **NOT ASSESSED**: current evidence has not been collected.
- **RESIDUAL**: a known issue remains.

Never infer verification from a source file, a test definition, a merged PR, a green badge on another SHA, or a prior report. Each acceptance record must include repository, SHA, workflow/run ID, job and step, exit code, and log/artifact reference. Hardware claims additionally name the machine, firmware, CPU, memory, GPU, storage, and input devices.

## 2. Repository baseline and known evidence

The repository's `docs/IMPLEMENTATION_STATUS.md`, `docs/OS_STANDARD_COMPONENTS.md`, `ROADMAP.md`, and `.github/workflows/system-ci.yml` are the starting points. The component model explicitly distinguishes architecture boundaries from production-ready implementation.

Current targeted GitHub metadata checked for this plan:

| Item | Observed metadata | Planning consequence |
|---|---|---|
| PR #98, `p0.3/page-fault-fail-closed` | Open Draft; head `4efd7be7460816e229bdb88efd6e83fcea0febbf`; base branch `p0.2/user-mapping-nx-hardening`; base SHA `d93d6a1298bae88aae7a5de10b4db7fa21cbeda4` | Fail-closed panic avoids returning to an unrecoverable fault, but is not per-process recovery. Its own description says current-head CI/QEMU validation is in progress. |
| PR #99, `p0.4/user-page-fault-termination` | Open Draft; head `a4ec57c2039a946d83e58821cea37c90b14c6ea7`; base branch `p0.3/page-fault-fail-closed`; recorded base SHA `514d2e4f2146cf5bca2f8c186a4454ce2fc5c786`; mergeable reported false | Reconcile branch ancestry and rebase/update the branch against the intended current #98 head, then rerun all gates. Do not assume PR #99 contains the current #98 head. |
| `system/runtime/src/lib.rs` | `boot_userspace()` currently constructs a devnet GenesisConfig and initializes `GenesisBridge` before returning `BootedRuntime`. | Add an explicit offline boot path and make chain initialization an optional service; preserve a separately testable chain-enabled path. This source alone does not prove that network connectivity is currently required. |
| `.github/workflows/system-ci.yml` | Format, workspace check, tests and Clippy run on push and pull request. | Preserve these gates. Confirm actual run results and add target-specific boot/integration jobs rather than treating Rust compilation as boot evidence. |
| `ROADMAP.md` | Lists init/supervisor, UEFI, storage, network drivers, display service, persistent VFS, package verification, desktop shell and recovery as unchecked work. | Reconcile each item against source and tests before changing its state. |

This is a targeted baseline, not an exhaustive source census or a claim that CI has passed.

## 3. Architecture decisions

1. **Offline desktop is a first-class boot mode.** Kernel, init, identity/session UI, local storage, shell, terminal and file manager must not require a chain node, remote RPC, network or AI model to be available.
2. **Chain and AI are optional userspace services.** Keep blockchain execution and Aurora outside the ShivaCore trusted computing base. Gate privileged actions through capabilities and explicit user consent.
3. **Native applications first.** Define and stabilize the native executable format/ABI, process lifecycle, window/surface protocol, application manifest and permission model before pursuing Windows/Linux/macOS compatibility.
4. **QEMU is the reproducible reference; physical machines are a separate evidence track.** QEMU success never implies that a physical GPU, NVMe controller, Wi-Fi adapter or firmware combination works.
5. **Models are not backends.** A desktop state model, compositor interface, device registry, or filesystem abstraction is not accepted as a functioning subsystem until connected to the real runtime and covered by integration tests.

## 4. Milestones and implementation PRs

### M0 — Reconcile repository and exact-head evidence (P0)

**Branch/PR:** this document's branch; documentation-only review.

- Record main SHA, toolchain, lockfiles, workspace members, active workflow runs, and open PR base/head SHAs.
- Reconcile PR #97/#98/#99 ancestry; inspect diffs before rebasing. Do not merge automatically.
- Map every roadmap row to source path, owning component, test, evidence and remaining risk.
- Keep historical CI failures immutable; append new exact-SHA evidence after fixes.

**Acceptance:** no implementation status is promoted without source/test evidence; PR dependencies are explicit; no claim of a green run without its run ID and SHA.

### M1 — User address spaces and fault containment (P0; continue #97/#98/#99)

- Confirm per-process page-table roots and correct user/kernel mappings for code, data and stack.
- Enforce W^X/NX and validate mapping permissions and lifetimes.
- Prove CR3 changes on actual context switches, not only via mocked scheduler state.
- Wire the real syscall/trap path to process dispatch and scheduler state.
- Make recoverable Ring-3 page faults terminate only the offending process; Ring-0 faults remain fatal.
- Add guarded user-copy/copy-in handling against the active page tables.
- Reap faulted-process resources only after scheduler and userspace-manager references are safely detached.

**Acceptance/tests:** two processes cannot read/write each other's private mappings; attempts to execute NX pages fault; a bad user pointer is contained; a user fault yields the documented exit status (139 only if that ABI contract is retained), a second process continues, and the kernel remains operational. Run targeted tests and QEMU integration on the exact candidate SHA.

### M2 — Init and service supervisor (P0; continue #96)

- Provide an executable `globus-init` and connect it to the real process-start and IPC/capability handoff path.
- Implement dependency ordering, readiness, timeout, failure reporting, restart limits, shutdown and recovery.
- Separate mandatory local services from optional network, chain and AI services.

**Acceptance/tests:** boot starts init; services start only after dependencies/readiness; intentionally failing optional services do not prevent offline desktop boot; mandatory-service failure yields a diagnosable safe state.

### M3 — Device and persistent storage path (P0; build on #93)

- Complete PCI/ACPI inventory and safe resource ownership.
- Implement and validate MMIO mapping, interrupt routing and DMA/IOMMU isolation before enabling bus-mastering devices.
- Establish a QEMU block-device path first; connect partition parsing, filesystem, VFS, mount and sync.
- Add crash-consistency and journal-replay tests; then qualify named real NVMe/SATA devices.

**Acceptance/tests:** create/write/sync/reboot/read file in QEMU; inject interrupted writes and verify documented recovery; verify unsupported devices fail safely. Hardware compatibility is recorded per machine/device.

### M4 — Native graphics and input (P1)

- Connect a real framebuffer/display backend to a deterministic software renderer.
- Define pixel formats, surface ownership, clipping, damage regions and compositor loop.
- Connect keyboard/mouse input from device services to focus/window routing.
- Keep GPU acceleration behind a stable backend contract; it is not required for the first desktop milestone.

**Acceptance/tests:** QEMU shows a real rendered desktop, not a model-only test; pointer and keyboard work; open/move/focus/resize/close windows; renderer and input integration tests run in CI where possible.

### M5 — Native shell and applications (P1)

- Render login/session, panel/taskbar, launcher, notifications and settings.
- Connect windows to actual process IDs and surfaces.
- Launch native ELF programs through the production loader/process path.
- Deliver terminal and file manager first; define manifest and least-privilege capability grants.

**Acceptance/tests:** from a fresh boot, enter a local session, launch two native applications, interact with multiple windows, terminate/restart an app without reboot, and confirm app failure does not crash the kernel.

### M6 — Install image, package verification and recovery (P1)

- Build reproducible boot/install artifacts with a manifest pinning component versions and source SHAs.
- Verify actual package bytes and signatures against provisioned trust roots.
- Implement atomic update/rollback and protect user data/configuration.
- Provide diagnostic and recovery boot path.

**Acceptance/tests:** clean install, reboot, offline local app installation, valid/invalid signature cases, interrupted update, rollback and recovery all have repeatable test logs.

### M7 — Offline-first chain, identity and Aurora integration (P1/P2)

- Separate local session startup from chain genesis/node startup in runtime APIs.
- Provide distinct offline and chain-enabled boot configurations.
- Keep wallet keys behind a protected keystore API; never expose secrets to AI, telemetry or ordinary applications.
- Require explicit capability grants and user authorization for signing/chain actions.
- Define synchronization/retry semantics without making local desktop state depend on remote chain availability.

**Acceptance/tests:** boot with network disabled and chain service absent; local login, files, terminal and file manager remain functional. Start/stop chain and AI services independently. Test unauthorized signing, missing service and reconnect cases.

## 5. Test and evidence gates

Run existing workspace checks where supported:

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

A command that fails because of an existing compile problem is recorded as failed, not skipped or passed. Also run the repository's target-specific UEFI/kernel build, QEMU boot workflow, serial-log assertions and subsystem integration tests. If a target is not supported by the generic workspace command, record the explicit target command and result.

Every PR must state:
- exact base and head SHA;
- source paths changed and architecture impact;
- tests run, including negative tests;
- workflow run IDs and artifact/log links;
- known residual risks;
- rollback/recovery implications.

### Required hardware matrix

| Profile | Minimum evidence |
|---|---|
| QEMU x86_64, UEFI | repeatable boot, serial log, kernel/runtime markers, disk persistence, input/display, offline test |
| QEMU x86_64, BIOS (if supported) | separate boot evidence; do not infer from UEFI |
| Intel Skylake physical PC | firmware/CPU/RAM/GPU/storage inventory and tested features |
| AMD EPYC physical PC | firmware/CPU/RAM/GPU/storage inventory, SMP/interrupt and device results |

Physical-PC tests are not complete until a specific machine and firmware configuration has been recorded. Secure Boot, TPM, IOMMU, suspend, Wi-Fi, audio and accelerated GPU support remain separately qualified features.

## 6. Desktop Alpha 0.1 release gates

- [ ] Exact-SHA UEFI QEMU boot evidence
- [ ] Two isolated Ring-3 processes and real CR3-switch evidence
- [ ] User fault containment and scheduler continuation
- [ ] Capability/IPC negative tests
- [ ] Real init/supervisor boot path
- [ ] Persistent filesystem and recovery test
- [ ] Rendered, interactive native desktop
- [ ] Native terminal and file manager
- [ ] Offline desktop boot with no chain/network dependency
- [ ] At least one documented physical x86_64 PC test
- [ ] Package signature verification and update rollback
- [ ] Evidence bundle and security review for the exact release candidate SHA

The checklist is a release gate, not a report of completed work. Leave unchecked until evidence is attached.

## 7. Out of scope for Desktop Alpha 0.1

- Windows/Linux/macOS binary compatibility layers
- Universal GPU acceleration and every vendor-specific device
- Full Wi-Fi/Bluetooth/audio/power-management coverage
- Mandatory chain or AI availability for local desktop use
- Production-ready claims without security audit, reproducible images and recovery evidence
