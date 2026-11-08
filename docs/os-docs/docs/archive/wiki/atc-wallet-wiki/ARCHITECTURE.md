# ARCHITECTURE.md — atc-wallet

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-wallet/
├── Cargo.toml — GUI Wallet application manifest
├── .gitignore — Git ignore rules
└── src/
    ├── main.rs — GUI wallet launcher and window setup
    ├── lib.rs — Wallet core business logic library
    ├── keys.rs — Cryptographic keypair generation, BIP-39 mnemonics, and keystore encryption
    ├── tx.rs — Transaction builder, Ed25519 payload signing, and serialization
    ├── gui.rs — Immediate-mode egui user interface components
    ├── balance.rs — Account balance tracker and token portfolio overview
    └── history.rs — Transaction history cache and status manager
```

## Module Descriptions
- src/main.rs — Initializes native desktop window and launches egui event loop.
- src/lib.rs — Core wallet backend library managing state, key security, and transaction signing.
- src/keys.rs — Generates BIP-39 mnemonic phrases, derives Ed25519 keypairs, and encrypts keystore files using AES-GCM.
- src/tx.rs — Constructs, signs, and serializes transactions prior to network broadcasting.
- src/gui.rs — Renders GUI views including account balances, token transfers, key management, and settings.
- src/balance.rs — Tracks native coin and custom token balances via periodic node queries.
- src/history.rs — Maintains local transaction history cache and updates confirmation statuses.

## Build System
- Cargo.toml — Cross-platform Rust `std` GUI app using `egui` and `eframe`.

## Dependencies
- egui / eframe — Immediate mode GUI framework.
- ed25519-dalek — Fast Ed25519 cryptographic key generation and signature creation.
- bip39 — Deterministic seed phrase generation.
