---
protocol:
  id: ATC-PROTO-BLOCK-001
  name: "ATC Block Protocol"
  version: "1.0.0"
  status: draft
  domain: BLOCK
  layer: L3
  chain_id: 658467
  authority: A-TownChain-Okosystems
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-BLOCK-001 — ATC Block Protocol v1.0.0

## §1 Scope

Defines the canonical L1 block/header contract, deterministic block identity, transaction-root commitment, block propagation payload, structural validation and conformance requirements. Consensus finality is defined by ATC-PROTO-CONSENSUS-001; transaction semantics by ATC-PROTO-TX-001.

## §2 Existing implementation baseline (Gate 0)

Exact source inspected: `A-TownChain-Okosystems/a-townchain` `modules/atc-blockchain/kernel/src/blockchain.rs`, source SHA `51f0344a412d351359dff014a587b07c50ecf42b`; node-facing header source `modules/atc-blockchain/nodes/node.atc`, SHA `36f1c014836c900ff4c73c39cbbeb115f3581985`. The implementation already defines block height, parent hash, proposer identity, timestamp, PoH hash, transactions, transaction root, state root, gas used, total fees and signature; node propagation additionally carries height/hash/parent/timestamp/proposer/tx_count.

The existing implementation is evidence of implementation traces only. It does not establish canonical wire compatibility or protocol activation.

## §3 Canonical block header

| Field | Type | Requirement |
|---|---|---|
| version | u16 | canonical transaction/block protocol version |
| chain_id | u32 | MUST equal 658467 |
| height | u64 | monotonically increasing |
| parent_hash | bytes32 | exact previous block identifier |
| proposer | canonical address/identity bytes | MUST identify proposer |
| timestamp | u64 | deterministic protocol timestamp |
| poh_hash | bytes32 | PoH commitment when enabled |
| tx_root | bytes32 | commitment to ordered transaction IDs/bytes |
| state_root | bytes32 | post-state commitment |
| tx_count | u32 | exact transaction count |
| gas_used | u64 | execution gas consumed |
| total_fees | u128 | economic fee amount |
| signature | protocol-defined signature bytes | proposer authorization |

The current implementation's `total_fees: u64` is NOT sufficient for the ecosystem economic contract and MUST NOT be treated as the final canonical field width. Canonical economic amounts are u128.

## §4 Block identity and commitments

Block identity MUST be derived from a canonical binary header/preimage. JSON/Serde/native object layout MUST NOT define consensus bytes. Field order and integer endianness MUST be frozen before activation.

The transaction commitment MUST be deterministic over the ordered transaction set. Any implementation using transaction IDs MUST define the exact transaction-ID algorithm and byte order in the corresponding conformance vector.

## §5 Structural validation

A node MUST reject a block when: chain_id is wrong; height is not the expected successor; parent_hash does not match the local predecessor; required fields are malformed; tx_count disagrees with the transaction list; transaction commitment is invalid; proposer/signature authentication fails; timestamp violates the configured validity window; or economic/gas bounds overflow.

Validation MUST be side-effect free until all structural, cryptographic and state-transition preconditions pass.

## §6 Propagation

Block propagation MUST transmit enough canonical data to identify and request the block. The existing node contract carries `height`, `hash`, `parent`, `timestamp`, `proposer` and `tx_count`. A propagation implementation MUST additionally bind the announcement to chain_id and protocol version and MUST NOT advance local canonical height solely from an unverified announcement.

## §7 Genesis and continuity

Genesis is height 0 and has no predecessor. Every non-genesis block MUST reference exactly one parent. A node MUST NOT accept competing parents as the canonical successor without the consensus protocol's explicit fork-selection/finality decision.

## §8 Economic and execution invariants

`total_fees` and all other economic amounts MUST use the ecosystem u128 contract. `gas_used` remains a counter and therefore u64. Overflow MUST be rejected, never wrapped.

## §9 Conformance

Required positive vectors: genesis; first successor; multi-transaction block; empty transaction block where permitted; maximum u128 total-fees boundary; valid parent chain; valid tx_root; valid proposer signature.

Required negative vectors: wrong chain_id; height gap; wrong parent; malformed hash; tx_count mismatch; tx_root mismatch; invalid signature; timestamp violation; u128 fee overflow; non-canonical encoding; altered field after signature.

Cross-language conformance MUST compare exact header bytes, block ID/preimage, transaction commitment and verification result.

## §10 Security

Threat coverage MUST include tampering, replayed block announcements, equivocation/fork confusion, malformed input, proposer impersonation, resource exhaustion and dependency/supply-chain compromise. Promotion to `active` requires a populated ATC protocol-security entry and exact-SHA evidence.

## §11 Status

The protocol remains `draft` until canonical wire widths/encoding are frozen and exact-SHA conformance evidence exists. This specification records existing implementation traces but does not promote them to verified protocol compliance.
