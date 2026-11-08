# A-TownChain Ecosystem Integration

This crate is the executable integration contract between the canonical ATC node runtime and the canonical Rust SDK.

## Verified path

SDK TransactionBuilder -> signed ATC transaction -> ATC Node Runtime -> mempool

The integration layer does not duplicate consensus or state logic. It fails the build when the SDK and node disagree on the chain ID or transaction identity contract.

## Run

cargo test -p atc-ecosystem-integration
