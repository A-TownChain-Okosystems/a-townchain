# ARCHITECTURE.md — atc-zkp

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-zkp/
├── Cargo.toml — Zero-Knowledge Proof primitives crate manifest
├── .gitignore — Git ignore specifications
└── src/
    ├── lib.rs — Crate root and zero-knowledge proof API facade
    ├── snarks.rs — Groth16 / Plonk zk-SNARK prover and verifier implementation
    ├── starks.rs — Quantum-resistant zk-STARK proof generation and validation
    ├── circuits.rs — Arithmetic circuit construction and constraint system builder (R1CS)
    ├── verifier.rs — Sub-linear zero-knowledge proof verification pipeline
    └── merkle.rs — Cryptographic Merkle tree accumulator and inclusion proof generation
```

## Module Descriptions
- src/lib.rs — Main entry point providing high-level ZKP proving and verification interfaces.
- src/snarks.rs — Implements pairing-based SNARK proof generation and verification.
- src/starks.rs — Implements hash-based post-quantum STARK proving systems.
- src/circuits.rs — Builder API for expressing mathematical constraints and state transitions.
- src/verifier.rs — Optimized lightweight verification logic for on-chain and in-kernel validation.
- src/merkle.rs — Efficient Merkle tree operations for membership and non-membership proofs.

## Build System
- Cargo.toml — `#![no_std]` Rust library with SIMD hardware optimization support.

## Dependencies
- arkworks primitives or equivalent bare-metal cryptographic curve libraries.
