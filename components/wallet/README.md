# Canonical ATC Wallet

This is the canonical wallet implementation at `a-townchain/components/wallet`.

The former standalone `atc-wallet` repository is a migration source only. New implementation MUST target this component.

Wallet creates and authorizes transactions according to the canonical protocol/cryptographic contract. It does not own consensus, canonical state, or transaction protocol semantics.

Canonical path to the node is the monorepo component `a-townchain/components/node`; no standalone Git dependency is authoritative.
