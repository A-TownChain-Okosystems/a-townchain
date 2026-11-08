# Engineering Audit

Repository: `genesis-engine`
Status: `AUDIT_IN_PROGRESS`
Last verified: 2026-09-16

## Current baseline

The canonical Rust workspace contains 19 Cargo members. The runtime orchestrator is `atc-genesis-runtime`; there is currently no Cargo workspace member named `atc-genesis-engine`.

The repository is therefore treated as a modular general-purpose engine workspace. Historical content under `modules/atc-genesis-engine/` is considered legacy/migration material until its ownership and lifecycle are explicitly resolved.

## CI baseline

The enterprise CI baseline includes formatting, workspace compilation, tests, Clippy with warnings denied, documentation generation, dependency auditing and pull-request dependency review. Engine-specific determinism and subsystem gates are tracked separately and must not be inferred solely from generic Rust CI.

## Audit registry

Detailed findings and remediation status are maintained in:

`docs/ENGINEERING_AUDIT_FINDINGS.md`

Current finding groups:

- P0 — workspace/core/documentation/audit integrity: GEN-001 … GEN-004
- P1 — ECS/transform/assets/physics/renderer/runtime/determinism: GEN-005 … GEN-011
- P2 — AI/network/editor/SDK/build/conformance CI: GEN-012 … GEN-017

## Closure rule

A finding is not closed merely because the repository builds. Closure requires:

```text
Root Cause
  → Implementation Fix
  → Regression Test
  → CI Verification
  → Audit Evidence
  → CLOSED
```

The repository remains `development` and this audit does not constitute `AUDITED` or `PRODUCTION_READY` status.
