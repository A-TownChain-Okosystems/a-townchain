# ARCHITECTURE.md — atc-assets

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-assets/
├── Cargo.toml — Asset and token standards library manifest
├── .gitignore — Git ignore settings
└── src/
    ├── lib.rs — Crate root and asset standard interface definitions
    ├── fungible.rs — ATC-20 fungible token standard implementation
    ├── nft.rs — ATC-721 non-fungible token standard with dynamic state attributes
    ├── registry.rs — Global asset registry and symbol uniqueness manager
    ├── royalty.rs — Protocol-enforced automated creator royalty distribution
    └── collection.rs — Multi-token collection manager (ATC-1155 equivalent)
```

## Module Descriptions
- src/lib.rs — Public API exposing standard token traits and event interfaces.
- src/fungible.rs — Implements balances, transfers, allowances, and minting for ATC-20 tokens.
- src/nft.rs — Implements unique token ownership, metadata pointers, and transfer validation for ATC-721 tokens.
- src/registry.rs — Tracks registered tokens and guarantees symbol and ID uniqueness.
- src/royalty.rs — Calculates and routes creator royalties automatically on asset transfers.
- src/collection.rs — Manages batch operations and heterogeneous token collections under a single handle.

## Build System
- Cargo.toml — `#![no_std]` Rust library.

## Dependencies
- serde-no-std — Lightweight serialization for token metadata payloads.
