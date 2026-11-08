# Genesis Engine — Build Status

## Scope

This document records the evidence-backed build state of the Genesis Engine workspace. Documentation is not treated as proof of a successful build; CI results are the source of truth for the recorded status.

## Current status

**Status: BUILD-BLOCKED**

The latest recorded Rust CI run on `main` failed before the workspace reached the compiler, test, Clippy, or documentation stages because `cargo fmt --all -- --check` failed.

## Known blockers

### 1. Genesis AI parser error

The CI log reports a Rust parser error in:

`modules/atc-genesis-ai/src/lib.rs`

The reported diagnostic is:

`error: expected one of =>, if, or |`

This is a source-code error, not merely a formatting difference. It must be corrected before the workspace can progress through the normal Rust verification pipeline.

### 2. Workspace formatting

The same CI run reports formatting differences across multiple Genesis modules. The required gate is:

```text
cargo fmt --all -- --check
```

Formatting must be corrected consistently rather than bypassing the gate.

### 3. SDK dependency verification

`modules/atc-genesis-sdk/src/lib.rs` imports `atc_genesis_ecs::World`, while the SDK manifest currently declares `atc-genesis-platform` but does not declare `atc-genesis-ecs` as a direct dependency.

This must be verified with `cargo check` after the formatting/parser blockers are fixed. If confirmed, the SDK manifest must declare the ECS crate explicitly.

## Required verification order

1. `cargo fmt --all -- --check`
2. `cargo check --workspace --all-targets`
3. `cargo test --workspace --all-targets`
4. `cargo clippy --workspace --all-targets -- -D warnings`
5. `cargo doc --workspace --no-deps`
6. Determinism and governance gates

A later stage must not be described as passing until its command has actually produced passing evidence.

## Evidence policy

- README claims are descriptive only.
- Local developer success is not release evidence.
- CI success is required for workspace-level verification.
- Security-critical, deterministic, networking, rendering, audio, and distribution paths require additional component-specific evidence.

## Release implication

Genesis Engine must remain classified as development/rebuild until the blocking Rust verification gates pass and the runtime backends and integration paths are independently verified.
