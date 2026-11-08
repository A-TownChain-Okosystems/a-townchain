# GENESIS-ENGINE-MONOLITH-001

## Purpose

This document defines the monolithic integration target for Genesis Engine. All generic engine capabilities remain in one repository until a subsystem is mature enough for repository promotion.

## Integrated subsystems

| Domain | Module |
|---|---|
| Platform contracts | `atc-genesis-platform` |
| Core ECS/world/game logic | `atc-genesis-engine`, `atc-genesis-ecs`, `atc-genesis-world` |
| Rendering | `atc-genesis-renderer` |
| Physics | `atc-genesis-physics` |
| Audio | `atc-genesis-audio` |
| Animation | `atc-genesis-animation` |
| Assets | `atc-genesis-assets` |
| Runtime UI | `atc-genesis-ui` |
| Input | `atc-genesis-input` |
| Editor | `atc-genesis-editor` |
| Game SDK | `atc-genesis-sdk` |
| Gameplay AI | `atc-genesis-ai` |
| Networking | `atc-genesis-network` |
| Build/package | `atc-genesis-build` |
| Developer tools | `atc-genesis-tools` |
| CLI | `atc-genesis-cli` |
| Templates | `modules/atc-genesis-templates` |
| Samples | `modules/atc-genesis-samples` |
| Documentation | `modules/atc-genesis-docs` |

## Boundary rules

1. Platform contracts contain stable interfaces, not backend policy.
2. Gameplay code consumes the SDK and engine services; it does not modify chain truth.
3. ATCLang/ATC-VM/A-TownChain integration stays at an explicit integration boundary.
4. Subsystems must not introduce cyclic workspace dependencies.
5. Deterministic simulation must not depend on wall-clock time or nondeterministic iteration.
6. Repository promotion is optional and only follows demonstrated maturity and governance evidence.

## Verification target

The integration gate is:

```text
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Passing these commands is evidence of build/test health; it does not by itself establish production readiness.
