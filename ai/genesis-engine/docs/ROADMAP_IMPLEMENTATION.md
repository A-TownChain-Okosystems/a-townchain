# Genesis Engine Implementation Roadmap

This roadmap converts the current engine foundations into concrete implementation gates.

## P0 — Restore a verifiable workspace

- Fix the parser error in `atc-genesis-ai`.
- Make the complete workspace pass `cargo fmt --all -- --check`.
- Run and record `cargo check --workspace --all-targets`.
- Run and record `cargo test --workspace --all-targets`.
- Run and record Clippy and documentation checks.
- Verify the SDK's direct ECS dependency and all workspace manifests.

## P1 — Complete runtime backends

### Renderer

- Define a concrete GPU backend boundary.
- Implement a first supported backend for the target platform.
- Add shader compilation/loading and resource lifetime management.
- Add integration tests using a headless backend where hardware is unavailable.

### Audio

- Define the device callback contract.
- Implement a real platform output backend.
- Add streaming and decoder integration without making proprietary codecs mandatory.

### Networking

- Implement a real socket transport.
- Add connection lifecycle, reliable/unreliable channels and packet-loss handling.
- Add authentication/encryption at the transport boundary.
- Add integration tests with a local client/server pair.

### Assets

- Implement format-specific importers.
- Implement deterministic cooking and cache invalidation.
- Add binary artifact validation and content-addressed output.

### Input

- Connect OS/window events to the input state model.
- Add controller/gamepad support and configurable action bindings.

## P1 — Engine integration

- Connect renderer, physics, audio, animation, input and gameplay through `PlatformServices`.
- Establish a frame lifecycle with explicit ordering.
- Add startup/shutdown ownership rules.
- Add deterministic integration tests covering a complete frame.

## P2 — Editor and developer experience

- Implement editor viewport and renderer integration.
- Add hierarchy, inspector, gizmos and asset browser.
- Implement persistent project/scene files.
- Complete CLI `run` and `package` operations.
- Add diagnostics, logging and trace export.

## P2 — Distribution and release engineering

- Implement reproducible target builds.
- Add artifact signing and verification.
- Generate SBOM/provenance metadata.
- Add release manifests and compatibility metadata.
- Add end-to-end sample projects.

## Completion rule

A subsystem is not marked complete because its interface exists. Completion requires executable implementation, tests appropriate to the risk, integration evidence, and documented provenance/license status.
