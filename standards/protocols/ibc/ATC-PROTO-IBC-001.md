---
protocol:
  id: ATC-PROTO-IBC-001
  name: "ATC Inter-Blockchain Communication Protocol"
  version: 1.0.0
  status: draft
  domain: IBC
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-IBC-001 — Inter-Blockchain Communication Protocol v1.0.0

## §1 Scope
Defines deterministic cross-chain channels, packet envelopes, commitments, acknowledgements, timeouts, ordering and capability-bound application messages.

## §2 Existing-first evidence
Gate 0 searched canonical ShivaCore sources for IBC, inter-blockchain, packet, channel and acknowledgement implementation traces. No exact canonical source result was returned in this pass. No conformance claim is made.

## §3 Chain and channel identity
Every channel MUST bind source and destination chain identifiers, protocol version, port/application identifier, channel identifier and negotiated capability/version. Channel identifiers MUST be unique within their domain.

## §4 Packet envelope
Packets MUST contain canonical source/destination identifiers, sequence number, payload commitment, timeout height/time and versioned metadata. Serialization MUST be deterministic.

## §5 Commitments and acknowledgements
The sender MUST create a deterministic packet commitment before dispatch. Acknowledgements MUST bind to the exact packet identity and canonical result. Acknowledgement replay MUST NOT mutate state twice.

## §6 Ordering
Channels MUST explicitly select ordered or unordered semantics. Ordered channels MUST reject unexpected sequence numbers deterministically. Unordered channels MUST retain replay protection per packet sequence.

## §7 Timeouts
Timeout height/time semantics MUST be deterministic and evaluated against the canonical source/destination consensus state. A timed-out packet MUST NOT later be accepted as a fresh delivery.

## §8 Verification
Packet delivery MUST verify channel state, packet commitment, proof/finality requirements and timeout conditions before application execution.

## §9 Capabilities
Applications MUST be explicitly bound to channel capabilities. Possession of a channel identifier alone MUST NOT grant unauthorized application authority.

## §10 Failure and recovery
Missing acknowledgements, invalid proofs, channel closure and application execution failure MUST have deterministic state transitions. Recovery MUST preserve packet/acknowledgement audit linkage.

## §11 Resource limits
Packet size, sequence ranges, outstanding packets, channel count, acknowledgement size and verification/execution budget MUST be bounded.

## §12 Privacy
Payloads SHOULD avoid unnecessary sensitive data in canonical state. Commitments MAY represent off-chain payloads where the application contract permits.

## §13 Conformance
Positive vectors MUST cover ordered/unordered delivery, valid acknowledgement and valid timeout. Negative vectors MUST cover wrong channel, replay, sequence mismatch, invalid commitment/proof, premature timeout and post-timeout delivery. Cross-language implementations MUST produce identical packet IDs, commitments and state transitions.

## §14 Activation blockers
Status remains draft until the channel handshake, packet wire encoding, proof/finality interface, timeout clock model, capability model and exact-SHA conformance suite are frozen and verified.
