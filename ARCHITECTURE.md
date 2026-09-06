# ARCHITECTURE.md — atc-sdk
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── STATUS.md
├── api_client.atc
├── client.atc
├── contract_bindings.atc
├── event_subscriber.atc
├── rust/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── types.atc
├── typescript/
│   ├── package.json
│   └── src/
└── utils.atc
```

## Module Descriptions
- **rust/src/lib.rs** & **rust/Cargo.toml**: Native Rust SDK library crate providing RPC client abstractions, binary transaction builders, and cryptographic key management.
- **typescript/src/** & **typescript/package.json**: TypeScript SDK package offering Web3 providers, contract interaction bindings, and event streaming utilities.
- Common SDK abstractions: RPC node client handlers, contract ABI binding generators, event subscriber sockets, and type definitions.

## Build System
Dual build system: Cargo for Rust crate (`cargo build --release`) and npm / `tsc` for TypeScript library publishing (`npm run build`).

## Dependencies
Rust (`tokio`, `serde`, `reqwest`, `ethers-core`), TypeScript (`axios`, `ethers`, `@types/node`).
