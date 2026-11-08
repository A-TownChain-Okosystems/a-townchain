---
protocol:
  id: ATC-PROTO-STAKING-001
  name: "ATC Staking Protocol"
  version: 1.0.0
  status: draft
  domain: STAKING
  layer: L3
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-STAKING-001 — Staking Protocol v1.0.0

## §1 Scope
Defines canonical staking state, bonding/unbonding, delegation, validator backing, rewards, penalties and deterministic state transitions. Validator consensus authority remains owned by ATC-PROTO-VALIDATOR-001.

## §2 Existing-first evidence
Gate 0 searched the canonical A-TownChain and ShivaCore repositories for staking/stake/delegation/slashing implementation traces. No exact canonical staking implementation source was retrievable in this pass. Therefore this specification makes no implementation-conformance claim.

## §3 Canonical stake state
Stake amounts MUST use the chain economic amount contract: unsigned 128-bit integer (u128) with 18 decimal places. Negative, fractional-wire or floating-point stake values are prohibited.

The canonical state MUST distinguish at least bonded, unbonding, withdrawn, delegated where applicable, and effective validator backing. State transitions MUST be deterministic and height/epoch-bound.

## §4 Bonding
A bond MUST identify the owner, validator target where applicable, amount, activation boundary and canonical state version. Bond activation MUST NOT retroactively change validator eligibility for an already evaluated consensus scope.

## §5 Unbonding and withdrawal
Unbonding MUST create a deterministic release boundary. Withdrawal MUST be rejected before that boundary and MUST be idempotent after completion. Exact unbonding duration remains a frozen-parameter requirement before activation.

## §6 Delegation
If delegation is enabled, every delegation MUST identify delegator, validator, amount and activation/deactivation boundaries. Delegation MUST NOT alter historical validator-set snapshots. Delegation ownership and validator consensus authority MUST remain separate.

## §7 Validator backing and voting power
Effective validator backing MUST be derived deterministically from canonical staking state. The exact mapping from stake to voting weight MUST be frozen jointly with the validator/consensus contracts. No floating-point arithmetic is permitted.

## §8 Rewards
Rewards MUST be represented as integer base units and deterministically attributed to the canonical beneficiary. Reward issuance, compounding and claim semantics MUST be explicitly versioned. No reward formula is activated by this specification alone.

## §9 Slashing and penalties
Slashing MUST require verifiable evidence and a canonical triggering condition. Penalty calculation MUST be deterministic and bounded. Equivocation evidence from the validator/consensus layer MUST be consumable without trusting local node state. Exact penalty formulas and evidence encoding remain activation blockers.

## §10 Lifecycle and transitions
Every staking transition MUST specify pre-state predicate, authorized actor, canonical input, resulting state, effective height/epoch, and emitted evidence/event where applicable. Invalid transitions MUST leave canonical state unchanged.

## §11 Replay and authorization
Staking operations MUST bind to chain_id 658467 and canonical account/validator identity. Nonces or equivalent replay protection MUST be mandatory where an operation can be replayed. Authorization MUST be checked before state mutation.

## §12 Deterministic validation order
Implementations MUST validate encoding, chain identity, authorization, account/validator existence, amount bounds, current state, timing boundaries, economic constraints and resulting invariants in a fixed order.

## §13 Resource limits
Implementations MUST bound delegation counts, operation size, evidence size and per-block staking work. Limits MUST be consensus-visible.

## §14 Conformance vectors
Positive vectors MUST cover bond, unbond, withdrawal, delegation, validator backing, reward accounting and valid penalty application. Negative vectors MUST cover zero/overflow amount, unauthorized owner, unknown validator, premature withdrawal, invalid boundary, replay, malformed evidence, excessive operation size and inconsistent state.

Cross-language implementations MUST produce identical canonical staking state and transition results.

## §15 Activation
Status remains draft until exact implementation evidence, canonical operation encoding, frozen unbonding/reward/slashing parameters and exact-SHA CI conformance evidence are available.
