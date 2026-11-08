---
protocol:
  id: ATC-PROTO-ORACLE-001
  name: "ATC Oracle Protocol"
  version: 1.0.0
  status: draft
  domain: ORACLE
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-ORACLE-001 — Oracle Protocol v1.0.0

## §1 Scope
Defines deterministic ingestion of externally sourced data, attestations, aggregation, freshness, replay protection, dispute handling and canonical oracle state.

## §2 Existing-first evidence
Gate 0 searched canonical ShivaCore sources for oracle, price-feed, attestation and external-data implementation traces. No exact canonical source result was returned in this pass. No conformance claim is made.

## §3 Data source identity
Each source MUST have a versioned identifier and explicit trust/authorization metadata. Source identity MUST be separated from the data payload.

## §4 Observation format
An observation MUST contain domain-separated source identity, feed identifier, value representation, observation timestamp/epoch, sequence/reference identifier and attestation metadata. Canonical numeric values MUST use fixed-width integers or explicitly specified fixed-point encoding.

## §5 Authentication and attestation
Oracle observations MUST be authenticated according to an activated attestation scheme. Signatures MUST use a protocol-defined domain and deterministic serialization. Unauthenticated observations MUST NOT affect canonical state.

## §6 Freshness and ordering
Freshness windows, sequence ordering and timestamp/epoch rules MUST be explicit. Older or replayed observations MUST be rejected deterministically.

## §7 Aggregation
If multiple sources exist, the aggregation function MUST be deterministic, bounded and resistant to malformed/extreme inputs. Median/weighted aggregation is permitted only after exact algorithm, weights, tie-breaking and rounding are frozen.

## §8 Disputes and correction
Conflicting observations MUST produce deterministic state transitions. Corrections MUST reference the affected observation and preserve an auditable history.

## §9 Failure handling
Source timeout, malformed data, conflicting attestations and unavailable quorum MUST have explicit deterministic behavior. A missing oracle result MUST NOT silently become zero or another default value.

## §10 Replay/resource protection
Observation IDs, feed IDs and source state MUST have bounded sizes and deterministic replay protection. Per-feed rate, history and aggregation limits MUST be defined before activation.

## §11 Privacy and external data
Only required canonical commitments/results SHOULD be stored on-chain. Sensitive external data SHOULD remain off-chain with verifiable commitments where appropriate.

## §12 Conformance
Positive vectors MUST cover one source, multiple sources, aggregation, freshness and authenticated observations. Negative vectors MUST cover replay, stale data, invalid signatures, conflicting sequence numbers, malformed values and missing quorum. Cross-language implementations MUST produce identical canonical results.

## §13 Activation blockers
Status remains draft until the observation schema, attestation algorithm/domain, freshness rules, aggregation function, quorum/failure semantics, storage owner and exact-SHA conformance tests are frozen and verified.
