---
spec_id: ATC-CONSENSUS-312
title: "Consensus Key and Attestation Signature Contract"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain/components/algorithm
layer: L2-Blockchain-Core
owner: A-TownChain-Okosystems
created: 2026-10-02
depends:
  - ATC-CRYPTO-001
  - ATC-CONSENSUS-307
  - ATC-CONSENSUS-311
---

# Consensus Key and Attestation Signature Contract — ATC-CONSENSUS-312

## 1. Scope

This contract defines the separation between transaction/account keys, identity keys,
and consensus validator keys. It does not freeze a production consensus key algorithm.

## 2. Key-domain separation

Three domains MUST remain distinct:

1. **Transaction/account authentication**
   - secp256k1 ECDSA
   - RFC6979
   - canonical low-S
   - compressed 33-byte SEC1 public key
   - signing domain: `ATC-TX-DOMAIN-V2`

2. **Identity / node authentication**
   - governed by the identity/P2P contract
   - Ed25519 may be used where that contract explicitly requires it
   - MUST NOT be accepted as a transaction signature merely because the same key
     identifies a node or DID

3. **Consensus validator signing**
   - dedicated consensus-key role
   - algorithm/key type: **TBD until protocol freeze**
   - dedicated consensus/attestation domain: **TBD until protocol freeze**
   - MUST NOT reuse `ATC-TX-DOMAIN-V2`

## 3. Validator-key binding

A validator key MUST be bound to:

- chain_id
- consensus_version
- validator identity
- validator-set snapshot / activation context
- key role
- key identifier or canonical public key
- activation and, where supported, revocation/rotation state

A transaction public key MUST NOT implicitly become a validator key.

## 4. Attestation signature preimage

Before implementation freeze, the exact fixed-field canonical encoding MUST specify:

- domain separator
- encoding version
- chain_id
- consensus_version
- target proposal/block hash
- height
- slot/epoch
- validator identity/key reference
- validator-set snapshot identifier
- source/finality context
- signature encoding

Variable-length fields MUST have explicit canonical length encoding. Integer widths and
endianness MUST be inherited from the corresponding frozen field contracts, not a
universal serialization rule.

## 5. Rotation and revocation

Validator-key activation, rotation, revocation, and recovery MUST be deterministic and
must identify the authoritative validator-set snapshot at every affected slot.

A rotated key MUST NOT retroactively validate attestations outside its activation domain.
A revoked key MUST NOT contribute weight after the canonical effective point.

## 6. Replay and equivocation

Signatures MUST bind enough consensus context to prevent cross-chain, cross-version,
cross-proposal, and cross-slot replay.

Equivocation detection MUST operate on the frozen voting domain and canonical attestation
identity; implementation MUST NOT infer equivocation from arbitrary local serialization.

## 7. Failure semantics

Reject on:

- unsupported key type
- malformed public key
- malformed signature
- wrong domain
- wrong chain/version
- wrong snapshot
- inactive/revoked key
- replay
- non-canonical encoding

Rejected attestations MUST contribute zero stake/weight.

## 8. Conformance vectors

The freeze suite MUST include:

- valid consensus-key signature
- wrong transaction domain
- wrong consensus domain
- wrong chain_id
- wrong consensus_version
- wrong target
- wrong slot/epoch
- wrong snapshot
- inactive/revoked key
- malformed key/signature
- replay
- duplicate
- equivocation
- non-canonical encoding
- deterministic rotation boundary

## 9. Freeze gate

Production implementation is blocked until all of the following are frozen:

- consensus key algorithm
- public-key encoding
- signature encoding
- consensus domain separator
- exact attestation preimage layout
- validator-set snapshot semantics
- activation/rotation/revocation rules
- quorum/voting semantics
- cross-language golden vectors
- Exact-SHA CI evidence

No implementation in the canonical algorithm component may silently choose a different
key type or domain and call it protocol-compatible.
