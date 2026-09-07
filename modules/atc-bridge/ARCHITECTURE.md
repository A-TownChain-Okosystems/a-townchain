# ARCHITECTURE.md — atc-bridge

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-bridge/
├── Cargo.toml — Cross-chain bridge protocol crate manifest
├── .gitignore — Git ignore configuration
└── src/
    ├── lib.rs — Cross-chain bridge orchestrator and main dispatch library
    ├── ethereum.rs — Ethereum EVM light client, header validation, and Merkle proof parser
    ├── polkadot.rs — Polkadot Substrate GRANDPA finality proof verifier
    ├── cosmos.rs — Cosmos Tendermint light client and IBC payload processor
    ├── relayer.rs — Multi-signature cross-chain event relayer and state proof forwarding
    └── vault.rs — Multi-chain token locking, wrapping, and cross-chain mint/burn vault
```

## Module Descriptions
- src/lib.rs — Core cross-chain router orchestrating messages between disparate blockchains.
- src/ethereum.rs — Validates Ethereum block headers, state roots, and transaction execution receipts.
- src/polkadot.rs — Verifies Substrate headers, authority sets, and GRANDPA finality proofs.
- src/cosmos.rs — Processes Tendermint block headers and validates Inter-Blockchain Communication (IBC) packets.
- src/relayer.rs — Aggregates cryptographic signatures from relayer nodes before dispatching cross-chain actions.
- src/vault.rs — Escrow vault contract logic securing locked collateral and managing wrapped tokens.

## Build System
- Cargo.toml — Configured with `#![no_std]` for embedded contract runtime integration.

## Dependencies
- parity-scale-codec — Compact binary encoding for blockchain headers and payloads.
