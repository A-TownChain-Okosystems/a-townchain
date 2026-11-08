# Engineering Audit

Repository: `atc-engineering`
Status: ACTIVE / FLEET AUDIT CONTROL PLANE
Last verified: 2026-09-16
Last reconciled: 2026-10-06

This repository is the engineering control plane for the A-TownChain-Okosystems repository fleet. Its own audit implementation is subject to the same evidence rules it enforces: deterministic behavior, explicit findings, fail-closed gates and verified remediation.

## Implemented controls

- `atc-maintenance::audit_repository` — repository-tree audit for governance files, README identity, credential patterns, unsafe shell download pipelines, Rust workspace lock evidence, Cargo path dependencies, workflow permission policy, TODO/FIXME markers and duplicate source/configuration content.
- `atc-integration::IntegrationGraph` — deterministic integration graph covering the current active repository architecture.
- `.github/workflows/fleet-audit.yml` — organization fleet audit with read-only permissions, dynamic repository discovery, private-repository support through `ATC_FLEET_READ_TOKEN`, isolated shallow clones, per-repository evidence and artifact retention.
- `scripts/fleet_static_audit.sh` — offline fleet audit for environments where GitHub Actions execution is unavailable.

## Audit sequence

For every repository:

1. Discover repository tree and implementation boundaries.
2. Validate documentation/governance/CI contract.
3. Scan credentials and unsafe execution patterns.
4. Validate workflow security indicators.
5. Validate syntax and executable tests where toolchains are available.
6. Inspect architecture and cross-repository integration.
7. Identify defects, security findings, inconsistencies, incomplete functionality and duplicates.
8. Document concrete findings.
9. Implement evidence-backed remediation.
10. Re-run validation and re-read changed files.
11. Update documentation/evidence.
12. Repeat until no known blocking finding remains.

## Current fleet scope

The historical `atc-standards` registry snapshot and prior engineering audit recorded 31 repositories. The live GitHub organization inventory was re-read on 2026-10-06 and contains **33 repositories**.

The two repositories absent from the previous active audit matrix were:

- `a-townchain-ecosystem`
- `atc-toolchain`

The active fleet matrix has been reconciled to all 33 live repositories. Historical audit snapshots that state 31 repositories remain historical evidence and are intentionally not rewritten.

Private `demo-repository` remains part of the live fleet and requires `ATC_FLEET_READ_TOKEN` for the fleet clone stage.

## Current verified actions — historical

- Organization inventory re-read: 31 repositories visible, including private `demo-repository`.
- Fleet cloning hardened in `eb707e31983f233713d1a581d94a7570289a69e2`.
- Offline audit implementation added in `907c384ba934a979c323a5a0a570ca3714b7cecc` and extended in `dc0661e5ee42a60552ca1e8fcebc9d0acfae3f08`.
- `atc-sdk` documentation/structure inconsistency corrected and re-read.
- `atc-algorithm`: PoH slot overflow and missing-genesis handling hardened with regression tests.
- `globus-os`: VFS/GPT unsafe unwrap paths hardened and re-read.
- `globus-os`: ShivaCore is now a first-class workspace member at `modules/atc-shivacore/kernel`; the standalone kernel source tree is no longer canonical. The GlobusOS Rust CI explicitly tests and lints the kernel.
- `globus-os`: ATC Test Suite evidence-integrity defect identified: the old workflow could write `test_run.result: PASS` after npm passed even when cargo failed. Fixed in `231a6efe13e4d3ba21fa4e444efbfd572f08b858` by making the workflow read-only by default and moving evidence writes to a dedicated job that requires both test jobs to succeed.
- Current architecture map is maintained in `a-townchain-os-docs/docs/REPOSITORY_MAP_CURRENT.md`.

## Fleet reconciliation state — 2026-10-06

- **Discovery:** 33 live repositories.
- **Matrix:** reconciled to 33 repositories.
- **Historical snapshots:** preserved as historical evidence; no retroactive rewriting.
- **Fresh organization-wide execution:** **UNVERIFIED**.
- **Per-repository runtime/test evidence:** remains subject to the exact-SHA verification workflow.
- **Completion status:** **IN PROGRESS**.

The matrix reconciliation is a governance/documentation correction. It does **not** itself prove that all 33 repositories are compliant, tested, or verified.

## Active findings

- **GlobusOS ATC Test Suite:** run `35088731639` failed in `cargo-tests`; `npm-tests` passed. The old evidence file nevertheless claimed PASS. The workflow logic is now corrected in `231a6efe13e4d3ba21fa4e444efbfd572f08b858`. A new run must pass before a PASS claim is accepted.
- **GlobusOS governance CI:** run `35088731594` failed in `Repository-Audit`. The available job summary did not expose the command output, so root cause remained **UNVERIFIED** and required re-run/log inspection.
- **Standalone `atc-shivacore`:** historical `modules/atc-shivacore/kernel/src/lkm.rs` contained the old `unimplemented!()` in its historical repository tree. It is no longer canonical after relocation. The canonical GlobusOS tree was searched for `unimplemented` and `DependencyGraph::dependencies` and returned no matches.
- Organization-wide `pull_request_target` and mutable `actions/checkout@main/master` searches returned no matches in indexed organization content.
- Private-key search hits were scanner definitions/audit patterns, not confirmed exposed private keys.
- Search results can include historical commits and archived documentation; actionable findings are verified against the current default branch.

## Verification state

- Canonical ShivaCore location: **VERIFIED**.
- Kernel workspace membership in GlobusOS: **VERIFIED**.
- Kernel-specific Rust CI definition: **VERIFIED**; runtime verification of the new revision is **PENDING** until the corrected workflow succeeds.
- Full fresh 33-repository hosted audit: **UNVERIFIED** until usable fleet evidence is produced.

## Roadmap / TODO / Sprint / Wiki rule

Every repository audit reconciles implementation with README, ARCHITECTURE, STATUS, CHANGELOG, ROADMAP, TODO, sprint records and wiki/navigation pages. Historical material may remain archival, but current claims must match implementation or explicitly state archival status. Relevant open implementation points must be classified and tracked.

## Runtime limitation

A full fresh 33-repository clone/test run cannot currently be claimed from the local environment because outbound access to `github.com` is unavailable. GitHub-hosted verification is authoritative; no green result is inferred from source inspection alone.
