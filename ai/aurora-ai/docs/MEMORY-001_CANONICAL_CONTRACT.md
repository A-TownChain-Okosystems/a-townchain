# MEMORY-001 — Canonical Memory Contract

Status: SPECIFIED / DOCUMENTED
Baseline: ee21b53415964a14b6c280972e867df5227a1d0e
Lifecycle: DEVELOPMENT / NOT_READY

## Scope

MEMORY-001 establishes the machine-readable contracts required before persistent federation runtime work.

It covers A: canonical memory record schema; B: canonical serialization; C: integrity hashing and vectors; D: capability/permission schema; E: federation node handshake; F: positive/negative compatibility vectors.

It does not claim a functioning cross-device federation runtime.

## A — Canonical Memory Record

Schema: schemas/memory/memory_record.schema.json

The integrity_hash is the SHA-256 digest of the canonical serialization of the record with integrity_hash and chain_proof omitted.

## B — Canonical Serialization

The wire representation is JSON Canonicalization Scheme (JCS), RFC 8785.

1. Serialize as UTF-8.
2. Canonicalize JSON according to RFC 8785.
3. Omit integrity_hash and chain_proof from the hashed projection.
4. Hash the resulting bytes with SHA-256.
5. Encode the digest as lowercase hexadecimal.

No implementation may use map insertion order, pretty-printing, locale-dependent formatting, or an implementation-specific JSON serializer as an alternative canonical form.

The canonical serialization contract is normative for MEMORY-FED-1.

## C — Integrity

A receiver MUST reject a record when the schema is invalid, canonicalization fails, the supplied integrity hash does not match, memory type/scope is unsupported, or provenance is malformed.

## D — Capability and Policy

Schema: schemas/memory/capability.schema.json

Security sequence:

request -> identity -> capability -> policy/consent -> validation -> state change -> audit

Capabilities are scoped and may expire or be revoked. Read and write actions are distinct.

## E — Federation Handshake

Schema: schemas/memory/federation_handshake.schema.json

Protocol identifier: MEMORY-FED-1

A node MUST authenticate its identity before shared-memory authorization is accepted. A handshake MUST NOT grant access merely because a peer can establish a network connection.

## F — Compatibility Vectors

Reference vectors: tests/memory/memory_record_vectors.json and tests/memory/capability_handshake_vectors.json.

Positive vectors MUST be accepted. Negative vectors MUST be rejected.

## Evidence state

MEMORY-001 artifacts are SPECIFIED -> DOCUMENTED.

They become IMPLEMENTED only when executable code consumes these contracts. They become TESTED only when the vectors are executed by repository tests. They become CI-VERIFIED only from an exact-SHA workflow result.

No runtime federation claim is made by this change.
