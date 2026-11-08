# GlobusOS Architecture

The normative architecture is documented in [`docs/SYSTEM_ARCHITECTURE.md`](docs/SYSTEM_ARCHITECTURE.md).

GlobusOS is the operating-system platform built above the ShivaCore kernel. ShivaCore is now part of this repository at `modules/atc-shivacore/kernel` and is verified by the same GlobusOS Rust CI pipeline.

## Security boundary

```text
Hardware -> HAL -> ShivaCore TCB -> IPC/capabilities -> GlobusOS -> Aurora/apps
```

ShivaCore remains reusable at the kernel boundary, while its canonical source is maintained and CI-verified in this GlobusOS repository. GlobusOS owns system services, device integration, storage, networking, graphics, audio, package lifecycle and recovery. Aurora remains an AI platform outside the kernel TCB.

## Kernel integration

- Kernel source: `modules/atc-shivacore/kernel`
- Kernel crate: `shivacore`
- Stable CI: kernel library tests and Clippy run explicitly from the kernel directory.
- Workspace CI: the kernel is a workspace member and is included in formatting, build and workspace test checks.
- Boot path: the `x86-boot` feature remains a separate nightly/boot-integration milestone; it is not represented as production-ready merely because the library tests pass.
- The former standalone `atc-shivacore/modules/atc-shivacore` source tree has been relocated here; the old module path is no longer authoritative.

## Current implementation

- Process/thread model
- Address-space and memory policy types
- Capability authorization boundary
- IPC endpoint/message model
- VFS mount contract
- Network policy boundary
- Device registry
- Service lifecycle/dependency order
- Graphics and audio API boundaries
- Package verification model
- Atomic update/rollback state machine
- Integrated runtime status
- ShivaCore kernel library integration and CI verification
- CI validation for formatting, compilation, tests and Clippy

These components establish the API and trust boundaries; hardware-specific drivers, protocol implementations, complete boot integration and production-grade cryptography remain explicit implementation milestones and are not claimed complete by this document.