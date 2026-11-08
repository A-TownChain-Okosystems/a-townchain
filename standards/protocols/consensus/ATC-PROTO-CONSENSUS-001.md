---
protocol:
  id: ATC-PROTO-CONSENSUS-001
  name: "ATC Consensus Protocol"
  version: 1.0.0
  status: draft
  domain: CONSENSUS
  layer: L1/L3
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-CONSENSUS-001 — Consensus Protocol v1.0.0

## §1 Scope

Defines the deterministic consensus contract for A-TownChain block ordering, proposal, voting, finality and fork choice. It does not redefine transaction encoding (ATC-PROTO-TX-001), block encoding (ATC-PROTO-BLOCK-001), validator identity or staking.

## §2 Existing-first status

The protocol registry records implementation traces described as ShivaCore K16 (DAG + PoH + Validator + Voting + Finality). Current repository-tree evidence available during this specification pass did not expose a matching canonical source path at an exact current SHA. Therefore this document formalizes requirements but does not claim implementation conformance.

## §3 Determinism

For identical canonical chain state, validator set, protocol version and received consensus messages, every honest node MUST derive the same proposal eligibility, vote validity, fork-choice result and finality result.

Consensus MUST NOT depend on hash-map iteration, thread scheduling, local wall-clock formatting, floating-point arithmetic or nondeterministic serialization.

## §4 Chain identity and epochs

Consensus messages MUST bind to chain_id 658467, protocol version, epoch and target height. Messages for another chain, incompatible protocol version, stale epoch or invalid height MUST be rejected.

Height and epoch counters MUST use their canonical u64 counter representation.

## §5 Proposal

A proposal MUST identify the target height, parent block, proposer identity and canonical block identifier. The proposer MUST be eligible for the target round/height according to the frozen validator-selection rule.

A proposal referencing an invalid parent, invalid block, conflicting height or non-canonical block bytes MUST be rejected.

## §6 Proof-of-History

Where PoH is enabled, a proposal MUST carry the canonical PoH commitment required by ATC-PROTO-BLOCK-001. Verification MUST use deterministic input bytes and fixed validation order.

PoH MUST NOT be treated as a substitute for validator authorization or finality voting.

## §7 Voting

Votes MUST identify the target height, round/epoch, block identifier and voting authority. A validator MUST NOT cast conflicting votes for the same consensus scope.

Vote signatures MUST authenticate the consensus authority using the canonical consensus-key contract. Transaction signing keys MUST NOT be implicitly reused as consensus keys.

## §8 DAG and fork choice

If the implementation uses a DAG of proposals/votes, DAG edges and parent references MUST be validated before fork choice. Fork choice MUST be deterministic and MUST use only canonical protocol state.

No node may finalize a block solely because it was locally observed first.

## §9 Finality

Finality MUST require the protocol-defined quorum of eligible voting power/validators. The quorum formula, validator weighting and rounding rules MUST be frozen before activation and covered by golden vectors.

Once a block is finalized, an honest node MUST NOT accept a conflicting finalized block at the same height under the same consensus domain.

## §10 Validator-set transitions

Validator-set changes MUST have a deterministic activation height/epoch. A validator set used for a vote MUST be unambiguously derivable from canonical state.

Changes MUST NOT retroactively alter the validity of already-finalized blocks.

## §11 Replay and equivocation

Consensus messages MUST contain sufficient domain separation to prevent cross-height, cross-epoch and cross-chain replay.

Equivocation MUST be detectable from canonical signed evidence. Evidence handling MUST be deterministic and MUST NOT itself permit two conflicting finalized histories.

## §12 Recovery and timeouts

Timeout/round-change messages MUST be authenticated and bound to the same chain, height and round. Timeout handling MUST be deterministic.

Network delay MUST NOT cause local wall-clock values to become consensus state unless explicitly encoded by the protocol.

## §13 Conformance

Positive vectors MUST cover: valid proposal, valid vote, quorum finality, deterministic fork choice, valid epoch transition and valid timeout/round change.

Negative vectors MUST cover: wrong chain, invalid parent, invalid proposer, invalid signature, conflicting vote, replayed vote, insufficient quorum, invalid validator-set transition and conflicting finalized block.

Cross-language implementations MUST compare canonical message bytes, vote validity, fork-choice result and finality result.

## §14 Activation status

Status remains `draft`. Promotion requires:
1. exact-SHA implementation evidence for the canonical ShivaCore consensus implementation;
2. frozen quorum, validator weighting, proposer selection and round-change semantics;
3. canonical consensus-message wire encoding;
4. positive/negative golden vectors;
5. exact-SHA CI evidence under the protocol verification gate.
