# ARCHITECTURE.md — atc-security

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-security/
├── Cargo.toml — Security toolkit and auditor crate manifest
├── .gitignore — Git ignore configuration
└── src/
    ├── lib.rs — Security toolkit entry point and verification pipeline facade
    ├── auditor.rs — Static bytecode and assembly security analyzer
    ├── fuzzer.rs — In-memory fuzz testing framework for contract execution
    ├── verifier.rs — Formal verification and safety invariant engine
    ├── scanner.rs — Known vulnerability signature scanner
    └── anomaly.rs — Real-time execution anomaly and intrusion detector
```

## Module Descriptions
- src/lib.rs — High-level entry point exposing static and dynamic security analysis tools.
- src/auditor.rs — Performs static analysis on bytecode to detect reentrancy, integer overflow, and unauthorized access.
- src/fuzzer.rs — Generates mutated execution inputs to stress test smart contract execution state.
- src/verifier.rs — Mathematically proves invariant preservation during transaction execution.
- src/scanner.rs — Scans code patterns against known attack signatures.
- src/anomaly.rs — Analyzes execution telemetry for irregular gas consumption or call depth spikes.

## Build System
- Cargo.toml — `#![no_std]` Rust library usable in verification tooling and node runtimes.

## Dependencies
- fixedbitset — Efficient bitset operations for security vulnerability analysis.
