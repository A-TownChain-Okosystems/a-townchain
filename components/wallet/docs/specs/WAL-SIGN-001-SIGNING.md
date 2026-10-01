---
spec_id: WAL-SIGN-001
title: "A-TownChain Transaction Signing Implementation Contract"
version: 1.0.0
status: IMPLEMENTATION-BOUND — protocol authority remains outside Wallet
repository: a-townchain
path: components/wallet
layer: L7
owner: A-TownChain-Okosystems
license: Apache-2.0
---

# WAL-SIGN-001 — Transaction Signing

## 1. Authority

This document describes the Wallet implementation of the canonical protocol contract. It does **not** define a competing transaction protocol.

The normative protocol/cryptographic contract is owned by the standards/L2 boundary. Wallet implements it.

## 2. Canonical requirements

- ECDSA secp256k1.
- Deterministic RFC6979 nonce generation.
- Low-S signatures only; high-S signatures are rejected.
- Domain separation uses **`ATC-TX-DOMAIN-V2`**.
- Canonical transaction amounts use **u128**.
- Canonical fixed-width numeric encoding is big-endian.
- Cross-language Rust/TypeScript/Wallet vectors MUST be byte-identical.
- Legacy `ATC-TX-DOMAIN` is forbidden.

## 3. Trust boundary

Signing occurs only inside the Wallet trusted signing implementation. Network, SDK UI, AI output, or external metadata MUST NOT acquire signing authority implicitly.

## 4. Conformance

Required evidence includes:

1. deterministic RFC6979 vectors;
2. low-S positive and high-S negative vectors;
3. exact transaction preimage vectors;
4. u128 boundary vectors including zero and `2^128-1`;
5. legacy-domain rejection;
6. malformed-signature rejection;
7. Rust ↔ TypeScript ↔ Wallet byte-identical vectors.

## 5. Status

This document does not assert VERIFIED. Verification requires exact-SHA CI evidence and the complete evidence chain.
