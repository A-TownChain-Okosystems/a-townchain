---
protocol:
  id: ATC-PROTO-VALIDATOR-001
  name: "ATC Validator Protocol"
  version: 1.0.0
  status: draft
  domain: VALIDATOR
  layer: L3
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-VALIDATOR-001 — Validator Protocol v1.0.0

## §1 Scope

Defines validator eligibility, identity binding, consensus-key authentication, validator-set derivation, voting authority, lifecycle transitions and validation duties for A-TownChain.

Transaction signing and account identity remain governed by ATC-PROTO-TX-001 and the Identity protocol. A transaction key MUST NOT automatically become a consensus key.

## §2 Existing-first evidence

The protocol registry records a ShivaCore K16 Validator implementation trace. The currently retrievable exact repository tree used for this Gate-0 pass does not contain a canonical validator source path, so this specification does not claim implementation conformance.

## §3 Validator identity

Every validator MUST have a unique canonical validator identifier and an authenticated consensus key. The binding between validator identifier and consensus key MUST be derivable from canonical chain state.

A consensus key MUST be distinct from an ordinary transaction/account signing key unless an explicitly versioned protocol contract states otherwise.

## §4 Eligibility

Eligibility MUST be determined from canonical state at the applicable activation height/epoch. Local configuration alone MUST NOT make a node eligible for consensus.

The eligibility predicate MUST be deterministic and MUST define all required conditions, including registration, activation state and any economic/security requirements imposed by the owning staking protocol.

## §5 Validator set

The active validator set MUST be deterministically derived from canonical state. Set membership, ordering, voting weight and activation/deactivation boundaries MUST be identical for all honest nodes.

Validator-set snapshots used for consensus MUST be immutable for the consensus scope in which they are referenced.

## §6 Voting authority

A validator MAY issue consensus votes only for scopes for which it is eligible. A vote MUST bind at minimum to chain ID, protocol version, height, round/epoch, block ID and validator identity.

A validator MUST NOT produce conflicting votes for the same consensus scope.

## §7 Block and vote validation

Validators MUST validate canonical block structure, parent continuity, transaction validity, state-transition validity, proposer eligibility and required consensus evidence before voting.

Votes MUST be rejected when signature, domain, scope, validator membership or weight is invalid.

Validation order MUST be deterministic.

## §8 Weight and quorum

Voting weight MUST be an integer value derived from canonical validator state. Floating-point weighting is prohibited.

The quorum threshold, rounding behavior and treatment of inactive/jailed/slashed validators MUST be frozen by the consensus contract before activation.

## §9 Lifecycle

Validator lifecycle MUST provide deterministic states for at least pending/registered, active, inactive and removed/jailed where applicable.

Every transition MUST have a canonical triggering event and effective height/epoch.

A validator MUST NOT retroactively become eligible for an already-finalized consensus scope.

## §10 Key rotation and recovery

Consensus-key rotation MUST be authorized by canonical validator state and MUST have an unambiguous activation boundary.

Old and new keys MUST NOT both be valid for the same scope unless explicitly specified by a versioned overlap rule.

Key loss/recovery MUST NOT permit unauthorized historical voting.

## §11 Equivocation

Conflicting signed votes or proposals from the same validator MUST produce independently verifiable evidence containing the canonical signed messages.

Evidence MUST identify the validator, consensus scope and conflicting message identifiers.

## §12 Resource and network behavior

Validator implementations MUST bound verification work, message size, vote buffering and per-peer input. Invalid consensus messages MUST NOT cause unbounded resource consumption.

Network arrival order MUST NOT alter the resulting canonical validator decision.

## §13 Conformance vectors

Positive vectors MUST cover eligible validator derivation, valid proposal validation, valid vote, quorum calculation, validator-set transition and key rotation.

Negative vectors MUST cover unknown validator, inactive validator, invalid key, wrong chain, wrong height/round, invalid block, conflicting vote, invalid weight, invalid transition and replay.

Cross-language implementations MUST compare validator-set snapshots, eligibility decisions, vote validity, integer voting weights and quorum results byte-for-byte or by exact canonical values.

## §14 Activation

Status remains draft until exact-SHA implementation evidence, frozen eligibility/weight/quorum semantics, canonical validator-message encoding, key lifecycle vectors and exact-SHA CI conformance evidence are available.
