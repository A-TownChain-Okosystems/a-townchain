---
protocol:
  id: ATC-PROTO-DATA-001
  name: "ATC Data Protocol"
  version: 1.0.0
  status: draft
  domain: DATA
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-DATA-001 — Data Protocol v1.0.0

## §1 Scope
Defines deterministic data-object identity, chunking, commitments, availability metadata, retrieval references, integrity verification, retention and resource limits.

## §2 Existing-first evidence
Gate 0 searched canonical ShivaCore sources for data availability, data objects, blobs, commitments and retrieval implementation traces. No exact canonical source result was returned in this pass. No conformance claim is made.

## §3 Object identity
Every canonical data object MUST have a versioned domain, deterministic object identifier, content commitment and explicit encoding identifier. Object IDs MUST NOT depend on local filesystem paths or nondeterministic metadata.

## §4 Encoding and chunking
Canonical encoding, chunk size, chunk ordering and length representation MUST be frozen before activation. Implementations MUST reject ambiguous or non-canonical encodings.

## §5 Commitments and integrity
The object commitment MUST cover the exact canonical content and required metadata. Chunk commitments MUST bind to object identity and deterministic position. Integrity failures MUST reject the affected object.

## §6 Availability
Availability state MUST be distinct from integrity. A valid commitment alone MUST NOT imply that data is retrievable. Availability proofs/attestations, if used, MUST have an explicit versioned verification contract.

## §7 Retrieval
Retrieval references MUST be deterministic and domain-separated. Retrieval failure MUST NOT silently substitute different content. Retrieved bytes MUST be verified against the canonical commitment before use.

## §8 Retention and lifecycle
Retention, expiration, pinning and deletion semantics MUST be explicit. Canonical state MUST NOT assume indefinite external storage unless guaranteed by the activated data contract.

## §9 Resource limits
Object size, chunk size/count, metadata size, concurrent retrievals, verification cost and retention obligations MUST be bounded.

## §10 Privacy
Sensitive data SHOULD remain off-chain or encrypted where appropriate. Public commitments MUST NOT be treated as confidentiality guarantees.

## §11 Failure and recovery
Missing chunks, corrupted data, unavailable providers and commitment mismatch MUST have deterministic failure/retry behavior. Recovery MUST preserve object identity and integrity.

## §12 Conformance
Positive vectors MUST cover empty/minimum object, boundary chunk sizes, multi-chunk object and successful retrieval verification. Negative vectors MUST cover malformed encoding, commitment mismatch, reordered chunks, truncated data, wrong object ID and unavailable provider. Cross-language implementations MUST produce identical object IDs, commitments and chunk layouts.

## §13 Activation blockers
Status remains draft until canonical encoding/hash, chunk format, availability proof, retrieval reference, retention model, storage owner and exact-SHA conformance suite are frozen and verified.
