---
protocol:
  id: ATC-PROTO-STORAGE-001
  name: "ATC Storage Protocol"
  version: 1.0.0
  status: draft
  domain: STORAGE
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-STORAGE-001 — Storage Protocol v1.0.0

## §1 Scope
Defines canonical persistent state storage, key/value encoding, namespaces, atomic transitions, snapshots, recovery and integrity verification.

## §2 Existing-first evidence
Gate 0 searched canonical ShivaCore sources for persistent storage, state databases, key/value stores, snapshots, Merkle storage and recovery implementation traces. No exact canonical source result was returned in this pass. No conformance claim is made.

## §3 Key and namespace model
Storage keys MUST have deterministic byte encoding and explicit namespace/version separation. Cross-namespace collisions MUST be impossible under the canonical encoding.

## §4 Value encoding
Values MUST use canonical, versioned binary encoding. JSON/Serde or implementation-specific representations MUST NOT become consensus-critical wire formats without an explicit protocol contract.

## §5 Atomic state transitions
Consensus-critical state updates MUST be applied atomically or through a deterministic journal/transaction mechanism. Partial writes MUST NOT become observable canonical state.

## §6 Integrity
Stored state MUST be verifiable against the activated state commitment scheme. Corruption, missing records or invalid commitments MUST fail closed for consensus-critical reads.

## §7 Snapshots
Snapshots MUST identify chain_id, state height/version, parent/state commitment and canonical format version. Restoring a snapshot MUST verify all required identity and integrity metadata before activation.

## §8 Recovery
Crash recovery MUST be deterministic. Journal replay, rollback or checkpoint recovery MUST have explicit ordering and idempotency rules.

## §9 Concurrency
Concurrent access MUST NOT produce nondeterministic consensus state. Locking, ownership or transactional isolation MUST be explicit for each consensus-critical state domain.

## §10 Resource limits
Key size, value size, batch size, snapshot size, journal depth, recovery work and namespace count MUST be bounded.

## §11 Privacy and deletion
Sensitive local data MAY be encrypted at rest where appropriate. Deletion/compaction MUST NOT remove information required to verify canonical commitments or audit state transitions.

## §12 Failure semantics
I/O failure, corruption, insufficient storage and recovery inconsistency MUST have deterministic failure behavior. Silent fallback to stale or alternate state is prohibited for consensus-critical reads.

## §13 Conformance
Positive vectors MUST cover canonical key/value encoding, atomic batch, snapshot creation/restoration and deterministic recovery. Negative vectors MUST cover malformed encoding, namespace collision, corrupted record, wrong chain/height, invalid commitment, partial journal and insufficient storage. Cross-language/state-backend implementations MUST agree on canonical bytes and commitments.

## §14 Activation blockers
Status remains draft until canonical key/value encoding, state commitment scheme, journal model, snapshot format, concurrency ownership, storage backend contract and exact-SHA conformance suite are frozen and verified.
