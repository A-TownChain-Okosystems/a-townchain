---
spec_id: WAL-TX-001
title: "Canonical Transaction Specification"
version: 1.0.0
status: IMPLEMENTATION-BOUND — protocol authority remains outside Wallet
repository: a-townchain/components/wallet
layer: L7-Wallet
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-09-10
scr: SCR-0071
depends:
  - ATC-STD-600
---

# WAL-TX-001 — Wallet Transaction Mapping

> **Authority rule:** This document does not define a competing transaction protocol. The normative transaction and cryptographic contract is owned by the L2/standards boundary, currently represented by ATC-STD-600. The Wallet is an L7 implementation and must remain byte-compatible with that contract.

## 1. Purpose

Define the Wallet-side mapping and conformance obligations for the canonical L1 transaction. Protocol changes MUST be made in the authoritative L2/standards contract first and then propagated to the Wallet implementation.

## 2. Canonical mapping

The Wallet implementation in `components/wallet/src/tx.rs` implements the current contract:

- `chain_id`: `u64`, canonical value `658467`
- `tx_type`: `u8`
- `sender_did`: length-prefixed UTF-8 bytes
- `recipient_did`: presence byte + length-prefixed UTF-8 bytes when present
- `amount`: `u128`, fixed 16-byte big-endian
- `gas_price`: `u128`, fixed 16-byte big-endian
- `gas_limit`: `u64`, big-endian
- `nonce`: `u64`, big-endian
- `timestamp`: `u64`, big-endian
- `payload`: u32 length + bytes
- `poh_hash`: exactly 32 bytes
- domain separator: `ATC-TX-DOMAIN-V2`
- transaction digest: SHA-256 of the canonical signing preimage
- authorization: ECDSA secp256k1 with RFC6979 and low-S enforcement

## 3. Forbidden legacy contract

The former little-endian / 65-byte-recovery-ID draft in this file is obsolete and MUST NOT be implemented. In particular, the legacy `ATC-TX-DOMAIN` is forbidden.

## 4. Conformance requirements

- Rust and TypeScript preimages MUST be byte-identical.
- `amount` and `gas_price` MUST remain `u128` end-to-end.
- Boundary vectors MUST cover `0` and `2^128-1`.
- Positive low-S and negative high-S signature vectors MUST exist.
- Legacy-domain and malformed-signature tests MUST fail closed.
- Exact-SHA CI evidence is required before verification status may be asserted.

## 5. Evidence status

This document does not assert `VERIFIED`. Verification requires the exact source SHA, the corresponding CI Run/Job/Step/Log chain, and conformance evidence bound to that SHA.

## 6. References

- ATC-STD-600 — normative transaction/cryptographic contract
- `components/wallet/src/tx.rs` — Wallet implementation
- `components/sdk/typescript/chain-identity.ts` — TypeScript conformance implementation
