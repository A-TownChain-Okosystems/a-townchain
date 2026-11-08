# Release Readiness

## Current state

**Repository maturity:** Development  
**Production readiness:** Not established

## Gate matrix

| Gate | Requirement | Evidence source | State |
|---|---|---|---|
| Formatting | `cargo fmt --check` | Enterprise CI | Required |
| Compile | workspace check | Enterprise CI | Required |
| Tests | workspace tests | Enterprise CI | Required |
| Static analysis | Clippy `-D warnings` | Enterprise CI | Required |
| API docs | Cargo docs | Enterprise CI | Required |
| Dependency security | RustSec audit | Enterprise CI | Required |
| Dependency review | PR dependency review | GitHub Action | Required |
| Ownership | CODEOWNERS review | GitHub | Required |
| Reproducibility | committed lockfile + pinned toolchain | Release evidence | Pending |
| SBOM | machine-readable dependency inventory | Release evidence | Pending |
| Artifact integrity | checksums/signing/attestation | Release evidence | Pending |
| Runtime integration | transport/backend/integration tests | Release evidence | Pending |
| Cross-platform | supported target matrix | Release evidence | Pending |

## Release rule

No individual checkbox is sufficient for production release. The release authority must verify the complete evidence set and record the decision in the applicable governance register.

## Evidence retention

Release evidence should include:

- commit SHA
- workflow run ID
- toolchain version
- dependency lockfile hash
- test results
- security audit result
- SBOM
- artifact checksums
- release metadata

## Failure policy

A failed mandatory gate blocks promotion. Warnings may only be accepted when the governing policy explicitly permits them and the exception is recorded with an owner, rationale and expiry.
