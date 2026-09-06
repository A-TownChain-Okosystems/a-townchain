# ARCHITECTURE.md — atc-dns

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-dns/
├── Cargo.toml — Decentralized DNS resolution library manifest
├── .gitignore — Git ignore configuration
└── src/
    ├── lib.rs — Crate entry point and DNS resolution facade
    ├── resolver.rs — High-speed decentralized name resolution algorithm for .atc domains
    ├── cache.rs — In-memory TTL-based DNS record cache manager
    ├── records.rs — DNS record structures (A, AAAA, TXT, DID, ATC-URI) and serialization
    └── propagation.rs — Peer-to-peer record propagation and consensus synchronization
```

## Module Descriptions
- src/lib.rs — Top-level API for performing domain lookups and record registration.
- src/resolver.rs — Resolves `.atc` human-readable domain names into cryptographic public keys and network addresses.
- src/cache.rs — High-performance concurrent cache with TTL expiration handling.
- src/records.rs — Serializable definitions of record types used across A-TownChain network.
- src/propagation.rs — P2P gossip propagation engine broadcasting DNS updates across network peers.

## Build System
- Cargo.toml — `#![no_std]` Rust library usable in standalone daemons or kernel services.

## Dependencies
- serde-no-std — Compact binary serialization for wire-protocol DNS messages.
