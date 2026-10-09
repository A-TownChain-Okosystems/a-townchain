# ATC Wallet

> Rust-only Wallet Trusted Core for A-TownChain.

**Project:** atc-wallet  
**Organization:** A-TownChain-Okosystems  
**Status:** development  
**Layer:** L5 Wallet

## Purpose
The wallet component owns the Rust trusted-core wallet functionality:
- key material handling in Rust;
- wallet balance and transaction-history state;
- wallet UI state;
- transaction construction and signing interfaces.

The wallet is part of the trusted boundary. Private keys and transaction signing must not depend on Python or an external scripting layer.

## Canonical implementation
The canonical source is components/wallet/src/.
- keys.rs — wallet key API;
- tx.rs — transaction encoding/signing interface;
- balance.rs — checked balance accounting;
- history.rs — transaction-history invariants;
- gui.rs — wallet UI state/rendering;
- main.rs — native wallet entry point.

The former Python wallet trees were removed because they duplicated wallet functionality and introduced incompatible semantics such as floating-point economic amounts, wall-clock transaction nonces, JSON transaction hashing, and a non-canonical ECDSA implementation.

## Cryptographic boundary
The repository specifications require transaction signing to use ECDSA secp256k1 with deterministic RFC 6979 signing and Low-S enforcement, while Ed25519 is reserved for P2P/DID use. The current transaction implementation therefore remains subject to the canonical crypto/conformance work and must not be treated as independently VERIFIED until the exact transaction-signing vectors and evidence gates pass.

See:
- specs/ATC-CRYPTO-001-CRYPTOGRAPHY.md
- components/wallet/docs/specs/WAL-SIGN-001-SIGNING.md
- components/wallet/docs/specs/WAL-KEY-001-KEYGEN.md

## Economic-width rule
Economic amounts are represented as u128. They must never be converted to f32/f64 for transaction or balance semantics.

## Build and test
Use the Rust toolchain from the repository governance/toolchain configuration:

    cargo build
    cargo test
    cargo fmt --check
    cargo clippy --all-targets --all-features -- -D warnings

## Architecture
The wallet is a standalone component. Integration modules may depend on it, but they do not define a second wallet implementation.

## Security
Do not report wallet cryptographic functionality as production-ready without exact-SHA conformance evidence. Security-sensitive changes require the applicable ATC standards, vectors, tests, and review gates.
