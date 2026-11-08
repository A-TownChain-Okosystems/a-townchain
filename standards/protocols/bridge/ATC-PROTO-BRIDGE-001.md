---
protocol:
  id: ATC-PROTO-BRIDGE-001
  name: "ATC Bridge Protocol"
  version: 1.0.0
  status: draft
  domain: BRIDGE
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-BRIDGE-001 — Bridge Protocol v1.0.0

## §1 Scope
Defines deterministic cross-domain message envelopes, source/destination domain binding, proof/attestation validation, replay protection, ordering, finality requirements, mint/burn/lock accounting and failure recovery.

## §2 Existing-first evidence
Gate 0 searched canonical ShivaCore sources for bridge, cross-chain, relay, light-client and bridge-proof implementation traces. No exact canonical source result was returned in this pass. No conformance claim is made.

## §3 Domain binding
Every bridge message MUST bind source domain, destination domain, source chain identifier, destination chain identifier, protocol version and message nonce. Ambiguous domain identifiers MUST be rejected.

## §4 Message envelope
The canonical envelope MUST define version, message type, source/destination identifiers, nonce, payload commitment, proof/attestation and expiry/finality metadata. Serialization MUST be deterministic and versioned.

## §5 Proof and finality
A bridge MUST verify the activated proof/attestation mechanism for the source domain and enforce the configured finality threshold before accepting state-changing messages. A message MUST NOT be accepted solely because it was observed by a relayer.

## §6 Replay and ordering
Message identifiers and source-domain nonces MUST provide deterministic replay protection. Ordering requirements MUST be explicit per bridge channel. Duplicate or stale messages MUST be rejected without state mutation.

## §7 Asset accounting
Lock/mint and burn/unlock flows MUST maintain conservation of canonical supply across the bridge. Each asset mapping MUST be versioned and uniquely identified. Partial execution MUST NOT create unbacked supply.

## §8 Relayers
Relayers are transport participants, not implicit authorities. Their permissions, fee model, rate limits and equivocation handling MUST be explicitly defined.

## §9 Failure and recovery
Timeouts, unavailable source proofs, conflicting messages, destination execution failure and bridge halt MUST have deterministic handling. Recovery MUST preserve an auditable relation to the original message and proof.

## §10 Security limits
Payload size, proof size, channel count, pending-message count, retry count and execution budget MUST be bounded.

## §11 Privacy
Only required message commitments and canonical state SHOULD be stored. Sensitive payload data SHOULD remain off-chain when protocol semantics permit.

## §12 Conformance
Positive vectors MUST cover valid message, valid proof/finality, ordered delivery and complete asset accounting. Negative vectors MUST cover wrong domains, replay, invalid proof, insufficient finality, duplicate message, conservation violation, expired message and malformed envelope. Cross-language implementations MUST agree on canonical IDs, bytes and state transitions.

## §13 Activation blockers
Status remains draft until supported domains, proof/finality mechanism, canonical envelope bytes, asset-accounting model, relayer authority model, recovery semantics and exact-SHA conformance tests are frozen and verified.
