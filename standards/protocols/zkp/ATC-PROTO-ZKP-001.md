---
protocol:
  id: ATC-PROTO-ZKP-001
  name: "ATC Zero-Knowledge Proof Protocol"
  version: 1.0.0
  status: draft
  domain: ZKP
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-ZKP-001 — Zero-Knowledge Proof Protocol v1.0.0

## §1 Scope
Defines canonical proof envelopes, statement/commitment binding, verification inputs, proof-system identifiers, replay protection and deterministic verifier behavior for zero-knowledge proofs.

## §2 Existing-first evidence
Gate 0 searched canonical ShivaCore sources for ZKP, zero-knowledge, proof, verifier and commitment implementation traces. No exact canonical source result was returned in this pass. No conformance claim is made.

## §3 Domain separation
Every proof MUST identify a versioned proof-system/domain identifier. Statement, public inputs and verification context MUST be domain-separated from unrelated protocol messages.

## §4 Canonical statement
A statement MUST define the exact circuit/relation identifier, version, public-input encoding and verification context. Ambiguous or implementation-defined serialization MUST be rejected.

## §5 Proof envelope
The envelope MUST contain version, system identifier, statement identifier, canonical public inputs, proof bytes and required verification metadata. Field widths and byte ordering MUST be frozen before activation.

## §6 Verification
Verification MUST be deterministic for identical canonical inputs. The verifier MUST reject malformed proofs, unsupported systems, wrong statement versions, wrong chain context and non-canonical encodings.

## §7 Binding and replay
Proofs MUST bind to the intended statement and public inputs. Where proofs authorize a state transition, they MUST additionally bind to chain_id 658467 and the relevant domain/nonce or state commitment. Replayed proofs MUST be rejected where replay would alter state incorrectly.

## §8 Cryptographic agility
A proof-system identifier MUST prevent algorithm confusion. Adding a new proving system requires an explicit protocol version/activation path and independent conformance vectors.

## §9 Resource limits
Proof size, public-input count/size, verification cost, recursion depth and batch size MUST be bounded before activation.

## §10 Privacy
Only required public inputs and commitments SHOULD enter canonical state. Private witnesses MUST NOT be serialized into ordinary transaction/block state.

## §11 Failure semantics
Invalid proofs MUST have deterministic rejection behavior. Verification failure MUST NOT mutate canonical state.

## §12 Conformance
Positive vectors MUST cover valid proofs, public-input binding and statement/version binding. Negative vectors MUST cover malformed proofs, wrong statement, wrong public inputs, wrong chain/domain, unsupported proof system, replay and resource-limit violations. Cross-language verifiers MUST agree bit-for-bit on accepted canonical encodings and verification results.

## §13 Activation blockers
Status remains draft until the exact proof system(s), envelope byte encoding, statement/circuit registry, verification cost model, canonical storage owner and exact-SHA conformance suite are frozen and verified.
