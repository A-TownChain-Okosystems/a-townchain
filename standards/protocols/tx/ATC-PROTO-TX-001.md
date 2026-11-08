---
standard:
  id: ATC-PROTO-TX-001
  title: "ATC Transaction Protocol"
  version: "1.0.0"
  status: draft
  category: protocol
  authority: A-TownChain-Okosystems
  owner: "A-TownChain Protocol Governance"
  normative: true
  applies_to: "A-TownChain L1 transaction submission, validation and signing"
---

# ATC-PROTO-TX-001 — Transaction Protocol v1.0.0

## 1. Scope

This specification defines the canonical transaction contract for A-TownChain L1. It covers the transaction domain, wire encoding, signing preimage, authentication, replay protection, validation and conformance vectors. It does not define block propagation or consensus.

## 2. Canonical chain parameters

| Field | Value |
|---|---|
| chain_id | 658467 |
| amount representation | unsigned 128-bit integer |
| economic amount unit | 10^-18 ATC |
| nonce | unsigned 64-bit integer |
| transaction domain | ATC-TX-DOMAIN-V2 |
| transaction signer | secp256k1 ECDSA |
| signature encoding | canonical low-S, fixed-width (r,s) |
| public key | SEC1 compressed, 33 bytes |
| hash | SHA-256 |
| signing nonce | RFC6979 |

The legacy domain ATC-TX-DOMAIN is prohibited.

## 3. Wire-format requirements

1. All consensus-relevant transaction fields MUST have one canonical binary encoding.
2. amount, balance, supply, stake, reward, slashing and treasury values transported by the transaction contract MUST be represented as unsigned 128-bit integers.
3. nonce MUST be encoded as unsigned 64-bit.
4. Integer byte order and field ordering MUST be fixed by the canonical transaction schema.
5. JSON, YAML, TOML and language-native serialization MUST NOT define consensus bytes.
6. A transaction MUST reject non-canonical encodings rather than normalizing them silently.
7. The canonical signing preimage MUST be identical across Rust, TypeScript and wallet implementations.

## 4. Signing contract

The signing preimage MUST include the domain separator ATC-TX-DOMAIN-V2 and chain_id=658467. ECDSA MUST use secp256k1 with RFC6979 deterministic nonce generation and low-S normalization. The compressed public key MUST be exactly 33 bytes.

Signature verification MUST reject invalid curve points, invalid signature ranges, high-S signatures, wrong chain ID, wrong domain separator, malformed public keys and malformed transaction encodings.

## 5. Replay protection

A transaction MUST contain a canonical account nonce and chain ID. A valid signature for another chain or domain MUST NOT be accepted. Nonce reuse MUST be rejected according to the account-state rules of the canonical transaction implementation.

## 6. Validation

Validation MUST be deterministic and ordered. At minimum it MUST verify canonical decoding, chain ID, transaction domain, sender/public-key validity, signature validity, nonce validity, amount bounds, gas fields when enabled by the transaction version, and state-dependent balance/authorization constraints.

The validator MUST return a machine-readable failure and MUST NOT mutate state on failed validation.

## 7. Gas fields

gas_price and gas_limit remain versioned transaction-contract fields until their final canonical wire widths and semantics are frozen. Implementations MUST NOT invent incompatible wire encodings. Any activation MUST update this specification, the registry, and cross-language golden vectors together.

## 8. Conformance vectors

Every implementation MUST consume the same positive and negative vectors.

Mandatory positive coverage: minimum amount; maximum u128 amount; boundary nonce values; canonical compressed public key; canonical low-S signature; chain ID 658467; ATC-TX-DOMAIN-V2.

Mandatory negative coverage: amount overflow; truncated amount; high-S signature; invalid public key; wrong chain ID; legacy domain; malformed signature; non-canonical field encoding; nonce violation.

The release gate MUST compare exact serialized bytes and signing preimages across Rust, TypeScript and wallet implementations.

## 9. Compatibility

The protocol version uses SemVer. A breaking wire change requires a MAJOR version and an explicit compatibility/migration path. Implementations MUST reject unsupported transaction versions deterministically.

## 10. Security model

Threat coverage follows ATC-STD-PROTOCOL-003: replay, spoofing, tampering, key compromise, downgrade, malformed input and dependency/supply-chain risks MUST be represented in the transaction security registry before active.

## 11. Status

draft is intentional: the specification captures the canonical contract but does not by itself prove implementation or CI conformance. Promotion requires exact-SHA implementation evidence and CONF-BRONZE under ATC-STD-PROTOCOL-002.
