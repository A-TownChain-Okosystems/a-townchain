# ARCHITECTURE.md — atc-game

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-game/
├── Cargo.toml — Game engine framework manifest
├── .gitignore — Git ignore configuration
└── src/
    ├── lib.rs — Game framework library root
    ├── engine.rs — Real-time game loop, tick manager, and render hooks
    ├── state.rs — Deterministic game state sync and state hash verification
    ├── assets.rs — Asset loader and on-chain NFT item binding interface
    ├── matchmaking.rs — Peer-to-peer player matchmaking and lobby session manager
    └── reward.rs — Play-to-earn token reward calculation and distribution system
```

## Module Descriptions
- src/lib.rs — Entry point for game developers integrating with A-TownChain game protocol.
- src/engine.rs — Drives real-time game loop ticks, state transitions, and frame callbacks.
- src/state.rs — Guarantees cross-player deterministic state execution and state root hash validation.
- src/assets.rs — Connects game assets directly to `atc-assets` on-chain NFTs.
- src/matchmaking.rs — Coordinates peer-to-peer player matching based on rating and latency.
- src/reward.rs — Calculates and releases verified on-chain token rewards based on game match outcomes.

## Build System
- Cargo.toml — Standard Rust `std` game engine framework crate.

## Dependencies
- serde — Game state and protocol message serialization.
- rand — Deterministic pseudo-randomness for game mechanics.
