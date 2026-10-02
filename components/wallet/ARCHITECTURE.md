# ARCHITECTURE.md — atc-wallet

## Role
components/wallet is the canonical Rust wallet Trusted Core for the A-TownChain ecosystem.

## File tree
```text
components/wallet/
├── Cargo.toml
├── src/
│   ├── balance.rs
│   ├── gui.rs
│   ├── history.rs
│   ├── keys.rs
│   ├── lib.rs
│   ├── main.rs
│   └── tx.rs
└── docs/
    └── specs/
```

## Ownership
- keys.rs owns wallet key APIs.
- tx.rs owns transaction encoding/signing interfaces.
- balance.rs owns checked economic balance state.
- history.rs owns history ordering invariants.
- gui.rs owns wallet UI state.
- main.rs owns the native executable entry point.

There is no parallel Python wallet implementation.

## Trust boundary
Private-key handling and transaction signing belong to the Rust trusted core. Python is not part of the signing boundary. This follows the wallet signing/key-generation specifications.

## Canonical crypto status
The repository specifications require ECDSA secp256k1, RFC 6979 deterministic signing, Low-S enforcement, and canonical transaction-domain separation. Those requirements remain subject to spec freeze and conformance evidence. No legacy Python signer is retained as a compatibility implementation.

## Design rule
Standalone First, Ecosystem Second: integration modules consume the canonical wallet component; they do not fork or redefine wallet cryptography or economic types.
