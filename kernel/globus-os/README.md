![ATC COMPLIANCE](https://img.shields.io/badge/ATC%20COMPLIANCE-R3%20%C2%B7%20ATC--STD--201%2F202%2F203-brightgreen)

# GlobusOS

> AI-native operating system and userspace platform built on the ShivaCore kernel.

**Project:** `globus-os`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0`

## System role

GlobusOS is the complete operating-system userspace/platform layer above **ShivaCore**. ShivaCore remains the reusable kernel/TCB; GlobusOS provides the system services, device integration, storage, networking, graphics, audio, package lifecycle, identity, wallet integration and recovery required to turn the kernel into an operating system.

The complete standard OS component coverage and implementation priorities are defined in [`docs/OS_STANDARD_COMPONENTS.md`](docs/OS_STANDARD_COMPONENTS.md). That document is a coverage baseline, not a claim that every listed component is already production-ready.

The formal Aurora AI 1.0 architecture, domain boundaries, agent contracts, Fabric/offline-first rules, risk/approval model, integration boundaries and staged acceptance criteria are defined in [`docs/AURORA_AI_ARCHITECTURE_1.0.md`](docs/AURORA_AI_ARCHITECTURE_1.0.md).

```text
Applications
    ↓
Aurora AI / Desktop / Shell
    ↓
GlobusOS system services
    ├─ Identity / Authentication
    ├─ Wallet Service boundary
    ├─ Storage / VFS
    ├─ Network / Devices
    └─ Package / Update / Recovery
    ↓
IPC + capabilities
    ↓
ShivaCore kernel / TCB
    ↓
HAL / firmware / hardware
```

AI and blockchain components are not promoted into the ShivaCore TCB by integration.

## Identity & wallet

GlobusOS uses a native **Boot → Login/Register → Identity → Wallet → Desktop** flow. Creating a new user profile provisions an A-TownChain-compatible wallet identity through the dedicated wallet integration. The public wallet address is bound to the GlobusOS User ID through a cryptographically verifiable identity binding.

The wallet's 24-word recovery phrase is **recovery material, not the daily login credential**. Private keys and recovery material remain behind protected local boundaries and are never exposed to ordinary applications, Aurora AI, telemetry, logs, GitHub, or remote services.

See [`docs/IDENTITY_WALLET_ARCHITECTURE.md`](docs/IDENTITY_WALLET_ARCHITECTURE.md), [`docs/IDENTITY_WALLET_FLOW.md`](docs/IDENTITY_WALLET_FLOW.md), and the normative [`docs/ACCOUNT_WALLET_AURORA_NODE_BINDING.md`](docs/ACCOUNT_WALLET_AURORA_NODE_BINDING.md) contract for account ↔ wallet ↔ Aurora node binding, challenge signing, rotation, privacy, and evidence gates.

## Implemented system foundation

The repository now contains a Rust workspace under `system/` with explicit subsystem boundaries:

- `globus-system-core` — lifecycle and kernel-facing identity.
- `globus-ipc` — endpoints and IPC messages.
- `globus-security` — deny-by-default capability authorization.
- `globus-process` — process/thread lifecycle.
- `globus-memory` — address-space and page policy.
- `globus-vfs` — filesystem namespace and mount contract.
- `globus-net` — network policy/socket boundary.
- `globus-devices` — isolated device/driver registry.
- `globus-services` — service lifecycle and dependency ordering.
- `globus-graphics` — display/compositor/GPU boundary.
- `globus-audio` — audio service boundary.
- `globus-package` — signed package metadata verification.
- `globus-update` — atomic A/B update and rollback state machine.
- `globus-runtime` — integrated userspace runtime.
- `system/identity/` — identity, authentication, wallet-binding and recovery integration contract.

These are the canonical foundations. Hardware-specific implementations and production cryptography remain separate, evidence-driven milestones.

## Standard OS component coverage

GlobusOS tracks the standard operating-system domains as an explicit implementation matrix:

- Boot, UEFI, Secure Boot and TPM
- CPU/SMP, interrupts, timers and memory management
- PCI/PCIe, MMIO, DMA and IOMMU
- NVMe/storage, VFS and filesystems
- Ethernet/network stack
- capabilities, authorization, audit and key management
- identity, authentication and sessions
- init/service management and power management
- graphics, GPU and audio
- packages, updates and recovery
- shell/userland and developer SDK
- observability and diagnostics
- Aurora AI integration outside the kernel TCB
- A-TownChain/ATC-VM/ATCLang integration outside the kernel TCB

See [`docs/OS_STANDARD_COMPONENTS.md`](docs/OS_STANDARD_COMPONENTS.md) for ownership, priority, trust boundaries, hardware dependency chains and readiness criteria.

## Boot path

```text
UEFI / firmware
 → bootloader
 → ShivaCore
 → capability + IPC initialization
 → GlobusOS init/service manager
 → identity / security / devices / storage / network
 → wallet integration boundary
 → runtime / graphics / audio
 → Aurora / desktop / applications
```

## Development

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

CI runs the same validation gates on pushes and pull requests.

## Architecture and governance

See [`docs/SYSTEM_ARCHITECTURE.md`](docs/SYSTEM_ARCHITECTURE.md) for the normative system decomposition and trust boundaries and [`docs/OS_STANDARD_COMPONENTS.md`](docs/OS_STANDARD_COMPONENTS.md) for complete OS component coverage. Development follows `ATC-STD-000` and the applicable organization standards. `APPROVED`, `IMPLEMENTED`, `AUDITED` and `PRODUCTION_READY` remain independent states.

## Ecosystem boundaries

```text
ATCLang → ATC-VM → A-TownChain

Aurora → GlobusOS IPC/API → ShivaCore

GlobusOS Identity → atc-wallet integration → A-TownChain identity
```

The VM remains the boundary between chain execution and the Rust system/network track. GlobusOS does not embed chain semantics into the kernel.

## Security

Security-sensitive issues must not be disclosed through public GitHub Issues. Follow `SECURITY.md` and the organization disclosure process.

## License

Apache License 2.0. See [`LICENSE`](LICENSE).

## Installation

See the canonical repository documentation and applicable ATC standards.

## Usage

See the canonical repository documentation and applicable ATC standards.

## Configuration

See the canonical repository documentation and applicable ATC standards.

## Testing

See the canonical repository documentation and applicable ATC standards.

## Governance

See the canonical repository documentation and applicable ATC standards.

## Contributing

See the canonical repository documentation and applicable ATC standards.

## Support

See the canonical repository documentation and applicable ATC standards.


## Overview
GlobusOS is the operating-system userspace layer built on ShivaCore.

## Purpose
Provides the user-facing OS services, identity, wallet, applications and system integration required by the A-TownChain ecosystem.

## Status
**Status:** `development`  
**Version:** `0.1.0`

## Architecture
Components include the system services, ShivaCore integration modules and SDK. Data flow and dependencies are defined by the workspace manifests and canonical architecture documentation.

## Features
- Identity and wallet integration
- System services and media
- ShivaCore integration
- Application SDK

## Repository Structure
```text
├── system
├── modules
├── sdk
├── docs
└── tests
```

## Requirements
Rust stable and the repository's workspace toolchain are required.

## Installation
```bash
git clone https://github.com/A-TownChain-Okosystems/globus-os.git
cd globus-os
cargo check --workspace
```

## Configuration
Configuration is defined by the workspace manifests and system-specific configuration files.

## Usage
Build and test the workspace with Cargo commands documented by the repository workflows.

## Development
Use the repository workflow and ATC engineering standards for changes.

## Testing
```bash
cargo test --workspace --all-targets
```
Expected result: all applicable workspace tests pass.

## Security
Security issues must not be disclosed publicly; use the repository's official security reporting process and ATC-STD-203.

## Documentation
Canonical documentation is maintained in `docs/` and the repository's governance files.

## Governance
Changes follow ATC governance, evidence and review requirements.

## Standards & Compliance
Applicable standards include ATC-STD-000, ATC-STD-201, ATC-STD-202 and ATC-STD-203.

## Roadmap
See `ROADMAP.md` for the canonical development roadmap.

## Contributing
Contributions must pass the applicable CI and governance gates.

## License
Apache-2.0.

## Maintainers
A-TownChain-Okosystems / ShivaCoreDev.

## Repository Metadata
Canonical repository: `A-TownChain-Okosystems/globus-os`.

## Canonical status and inventories

Use these documents together; they have different purposes:

- [Implementation status and release gates](docs/IMPLEMENTATION_STATUS.md) — current declared state, blockers, acceptance checkpoints, and exact-SHA evidence requirements.
- [Component and function catalog](docs/COMPONENT_FUNCTION_CATALOG.md) — subsystem ownership, responsibility, declared state, and verification boundary.
- [Standard OS component coverage](docs/OS_STANDARD_COMPONENTS.md) — breadth/coverage baseline, not a claim of completed implementation.
- [System architecture](docs/SYSTEM_ARCHITECTURE.md) — normative decomposition and trust boundaries.
- [Roadmap](ROADMAP.md) — milestone ordering and planned work.
- [CI root-cause register PR #52](https://github.com/A-TownChain-Okosystems/globus-os/pull/52) — exact-head failure findings for selected open PRs; the register remains a draft proposal until merged.

**Evidence rule:** implementation, verification, and residual risk are separate states. Never mark a subsystem verified solely because its interface exists or a different commit passed CI.
