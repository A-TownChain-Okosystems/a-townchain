# Current Repository Map — A-TownChain-Okosystems

**Verified:** 2026-09-16  
**Source:** live GitHub organization inventory + `atc-standards` registry + `atc-engineering` integration graph

## Active organization repositories

The current organization inventory contains 31 repositories:

`.github`, `atc-standards`, `atclang`, `a-townchain`, `a-townchain-os`, `a-townchain-os-docs`, `atc-shivacore`, `atc-contracts`, `atc-node`, `atc-compute`, `atc-oracle`, `atc-sdk`, `atc-marketplace`, `atc-wallet`, `atc-vm`, `atc-zkp`, `aurora-ai`, `globus-os`, `genesis-engine`, `genesis-chronicles`, `atc-mining`, `atc-indexer`, `atc-interop`, `atc-launchpad`, `atc-explorer`, `atc-storage`, `atc-algorithm`, `atc-ide`, `genesis-franchise-factory`, `atc-engineering`, `demo-repository`.

`demo-repository` is private and therefore has a separate fleet-audit cloning requirement.

## Architecture boundaries

```text
ATCLang
   │ on-chain language / deterministic semantics
   ▼
ATC-VM
   │ execution boundary
   ▼
A-TownChain
   │ consensus / chain protocol
   ├── atc-node
   ├── atc-storage
   ├── atc-indexer
   ├── atc-oracle
   ├── atc-interop
   ├── atc-zkp
   ├── atc-wallet
   └── atc-algorithm / atc-mining

GlobusOS
   │ OS userspace / platform
   │
   └── modules/atc-shivacore/kernel
       │ kernel / capability boundary / TCB
       └── Aurora AI

Genesis Engine
   └── Genesis Chronicles / Genesis Franchise Factory

atc-sdk / atc-ide / explorer / marketplace / launchpad
   └── developer, application and integration surfaces

a-townchain-os
   └── system integration / deployment surface
```

## Canonicality rules

1. `atc-standards` is the standards source of truth.
2. `atc-engineering` owns organization-wide audit/integration evidence and fleet controls.
3. Repository-local implementation remains authoritative for code and tests.
4. Historical repository maps dated before the current inventory are archival evidence, not current topology.
5. No repository is considered complete merely because governance files exist; implementation, tests, security evidence, integration evidence and documentation must all be verified.
6. Hosted CI results are authoritative for runtime/build verification. When GitHub Actions provides no usable run evidence, the corresponding gate remains **UNVERIFIED**.

## Current audit findings — 2026-09-16

- **ShivaCore relocation:** canonical kernel source is now `globus-os/modules/atc-shivacore/kernel`, a member of the GlobusOS Cargo workspace and explicitly covered by GlobusOS Rust CI.
- **Old `atc-shivacore` stub:** the standalone repository's historical tree still contains the old `DependencyGraph::dependencies()` `unimplemented!()` source, but this is no longer the canonical kernel source. The canonical GlobusOS tree was searched for both `unimplemented` and `DependencyGraph::dependencies` and returned no matches.
- **GlobusOS test evidence defect:** run `35088731639` failed in cargo tests while npm tests passed, but the old workflow recorded a PASS evidence record. The workflow was corrected in `231a6efe13e4d3ba21fa4e444efbfd572f08b858`; evidence is now writable only by a dedicated job requiring both test jobs to succeed.
- **GlobusOS governance CI:** run `35088731594` failed in the repository audit step. Root cause remains unverified until job output is available.
- `atc-algorithm`: PoH slot overflow and missing-genesis error handling were implemented and regression-tested.
- `globus-os`: unsafe VFS/GPT unwrap paths were hardened and re-read.
- Organization-wide searches for `pull_request_target` and mutable `actions/checkout@main/master` returned no indexed matches. Private-key search hits were scanner definitions/audit patterns rather than confirmed exposed secrets.

## TODO / roadmap / wiki synchronization rule

Every repository audit reconciles current implementation with README, ARCHITECTURE, STATUS, CHANGELOG, ROADMAP, TODO, sprint records and wiki/navigation pages. Historical material may remain archival, but current claims must match implementation or explicitly state archival status. Relevant open implementation points must be classified and tracked rather than silently ignored.

## Historical-document rule

Documents that still describe the organization as 22, 26, or another historical repository count must be treated as historical snapshots unless explicitly updated. This file is the current navigation point.
