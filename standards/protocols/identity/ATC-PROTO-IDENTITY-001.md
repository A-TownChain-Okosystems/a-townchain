---
protocol:
  id: ATC-PROTO-IDENTITY-001
  name: "ATC Identity Protocol"
  version: 1.0.0
  status: draft
  domain: IDENTITY
  layer: L1/L3
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-IDENTITY-001 — Identity Protocol v1.0.0

## §1 Scope
Defines canonical decentralized identity records, authentication-key bindings, lifecycle, rotation, revocation and proof verification for A-TownChain. Transaction signing remains governed by ATC-PROTO-TX-001; consensus keys remain governed by the validator/consensus contracts.

## §2 Existing-first evidence
The protocol registry records ShivaCore K6 DID/RCT and K6b Ed25519 implementation traces. Exact repository searches in the canonical A-TownChain and ShivaCore repositories did not expose a resolvable identity source path during this pass. Therefore no implementation-conformance claim is made.

## §3 Canonical identity
Each identity MUST have a unique canonical identifier and deterministic serialization. Identity records MUST bind their identifier to a versioned authentication-key set and lifecycle state.

An identity identifier MUST NOT be inferred from a mutable display name or local configuration.

## §4 Key separation
Identity authentication keys, transaction/account signing keys and consensus keys MUST have explicit roles. Cross-role reuse MUST NOT be assumed. An identity key rotation MUST NOT silently rotate transaction or consensus authority.

## §5 Authentication
Authentication proofs MUST bind to the canonical identity identifier, protocol/domain version, chain context where applicable, challenge/nonce and proof payload. Replay of an accepted proof MUST be rejected.

Ed25519 may be used for identity authentication where specified by the activated key contract; the exact key encoding and signature envelope MUST be frozen before activation.

## §6 DID/RCT records
If DID/RCT records are enabled, their canonical fields, method/version, controller relationship, key references and proof semantics MUST be deterministic and versioned. Unknown critical fields MUST be rejected.

## §7 Lifecycle
Identity states MUST include deterministic creation, active, rotated and revoked semantics. Every transition MUST have an authorized actor, canonical input and effective height/epoch or equivalent monotonic boundary.

Revocation MUST be monotonic unless an explicitly versioned recovery mechanism is activated.

## §8 Key rotation
Key rotation MUST define authorization by the currently valid key or an explicitly authorized recovery path, activation boundary, overlap semantics and prevention of historical unauthorized use.

Old keys MUST NOT authenticate operations after their revocation boundary.

## §9 Recovery
Recovery MUST require independently verifiable authorization. Local administrator configuration MUST NOT alone restore decentralized identity authority.

Recovery operations MUST be replay-protected and auditable.

## §10 Proof verification
Verification MUST use canonical bytes, fixed algorithm identifiers and deterministic validation order. Implementations MUST reject malformed encodings, unknown critical algorithms, invalid signatures, wrong identity binding, wrong challenge/domain and revoked keys.

## §11 Privacy
Identity records MUST minimize unnecessary personal data. Protocol-level identity identifiers SHOULD be pseudonymous where application semantics permit.

## §12 Resource limits
Implementations MUST bound identity-record size, key count, proof size and verification work. Limits MUST be consensus-visible when identity state affects consensus.

## §13 Conformance vectors
Positive vectors MUST cover identity creation, valid authentication, key rotation, revocation and recovery.

Negative vectors MUST cover malformed identifier, wrong domain, wrong challenge, invalid signature, unknown critical field, revoked key, replay, unauthorized rotation and oversized proof.

Cross-language implementations MUST produce identical canonical identity bytes and verification outcomes.

## §14 Activation
Status remains draft until exact implementation source evidence, canonical identifier/record encoding, key algorithm/envelope contract, rotation/revocation semantics and exact-SHA CI conformance evidence are available.
