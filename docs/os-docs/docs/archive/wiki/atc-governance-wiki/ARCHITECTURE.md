# ARCHITECTURE.md — atc-governance

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-governance/
├── Cargo.toml — Governance protocol crate manifest
├── .gitignore — Git ignore settings
└── src/
    ├── lib.rs — Governance subsystem entry point and execution hook dispatcher
    ├── proposal.rs — Proposal lifecycle management (creation, status tracking, execution)
    ├── voting.rs — Quadratic voting and token-weighted voting tally mechanism
    ├── treasury.rs — On-chain DAO treasury reserve management and budget distribution
    ├── timelock.rs — Mandatory execution delay queue for governance decisions
    └── delegation.rs — Representative voting power delegation and proxy management
```

## Module Descriptions
- src/lib.rs — Main entry point managing proposal registration and execution triggers.
- src/proposal.rs — Handles proposal submission requirements, voting thresholds, and state transitions.
- src/voting.rs — Calculates voting results using quadratic scaling to prevent whale manipulation.
- src/treasury.rs — Multi-signature treasury vault releasing funds for approved proposals.
- src/timelock.rs — Enforces time delay windows before approved actions take effect.
- src/delegation.rs — Dynamic vote delegation model enabling proxy voting trees.

## Build System
- Cargo.toml — `#![no_std]` Rust crate designed for protocol-level governance execution.

## Dependencies
- serde-no-std — Binary serialization for governance state persistence.
