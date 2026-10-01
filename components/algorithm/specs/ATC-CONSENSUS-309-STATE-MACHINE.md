---
spec_id: ATC-CONSENSUS-309
title: "Consensus State Machine Contract"
version: 0.1.0-DRAFT
status: SPEC-DRAFT — normativ erst nach Spec-Freeze; Implementierung PENDING
repository: a-townchain/components/algorithm
layer: L2-Blockchain-Core
owner: A-TownChain-Okosystems
copyright: Michael Wroblewski
license: Apache-2.0
created: 2026-10-02
depends:
  - ATC-CONSENSUS-301
  - ATC-CONSENSUS-302
  - ATC-CONSENSUS-303
  - ATC-CONSENSUS-304
  - ATC-CONSENSUS-305
  - ATC-CONSENSUS-306
  - ATC-CONSENSUS-307
  - ATC-CONSENSUS-308
  - ATC-CONSENSUS-DET
  - ATC-CONSENSUS-ENC
---

# Consensus State Machine Contract — ATC-CONSENSUS-309

> **Status: SPEC-DRAFT / NON-FROZEN.**
> Dieser Vertrag verbindet PoH + PoS + PoW + PoI mit Proposal, Attestation,
> Finality, Fork Choice und State Transition. Er ersetzt keine der
> Einzel-Spezifikationen und friert noch keine konkreten Hybrid-Gewichte ein.

## 1. Canonical authority

Die Consensus-Autorität liegt ausschließlich bei:

`a-townchain/components/algorithm`

Legacy-/Simulatorpfade außerhalb dieses Components dürfen keine kanonischen
Blocks, Attestations oder Finality-Entscheidungen erzeugen.

## 2. State Machine

Ein Consensus-Slot durchläuft deterministisch:

```text
S[n]
 │
 ├─ 1. derive_slot_context
 │      └─ PoH state / canonical seed
 │
 ├─ 2. snapshot_validator_state
 │      └─ epoch-bound validator set + stake state
 │
 ├─ 3. validate_contributions
 │      ├─ PoS authorization / effective stake
 │      ├─ PoW proof / bounded work
 │      └─ PoI proof / bounded contribution
 │
 ├─ 4. select_proposer
 │      └─ ATC-CONSENSUS-304
 │
 ├─ 5. validate_proposal
 │      ├─ header / parent
 │      ├─ PoH continuity
 │      ├─ transaction/state-transition validity
 │      └─ proposer eligibility
 │
 ├─ 6. attest
 │      └─ validator signs canonical proposal context
 │
 ├─ 7. evaluate_finality
 │      └─ snapshot-bound quorum
 │
 ├─ 8. apply_fork_choice
 │      └─ only among valid non-finality-violating candidates
 │
 └─ 9. commit_state_transition
        └─ produce S[n+1]
```

A transition MUST either produce exactly one deterministic next state or reject
the candidate. No transition may depend on local randomness, wall-clock time,
unordered map iteration, floating-point arithmetic, or non-canonical external state.

## 3. Canonical transition inputs

Every transition MUST identify at minimum:

| Input | Source |
|---|---|
| chain_id | ATC-NETWORK-ID-001 |
| consensus_version | canonical consensus header/context |
| parent_block_hash | canonical parent |
| slot / epoch | canonical slot context |
| PoH state / seed | ATC-CONSENSUS-301 |
| validator-set snapshot | ATC-CONSENSUS-307 |
| effective stake | ATC-CONSENSUS-302 |
| PoW proof/contribution | ATC-CONSENSUS-303 |
| PoI proof/contribution | ATC-CONSENSUS-308 |
| proposal | canonical block/proposal contract |
| attestations | ATC-CONSENSUS-306 |
| finalized checkpoint | finality state |
| state root / resulting state | L2 state-transition contract |

## 4. Validator snapshot boundary

Validator-set changes MUST NOT affect an already executing slot.

For each slot, the state machine MUST select exactly one canonical validator-set
snapshot according to the frozen epoch-boundary rule. The same snapshot MUST be
used for proposer eligibility and the corresponding finality quorum unless the
frozen protocol explicitly defines a different, deterministic relationship.

A validator that is inactive in the selected snapshot cannot acquire consensus
weight merely by submitting a late registration or proof.

## 5. Contribution validation

Contribution processing is strictly ordered:

1. validate identity/capability;
2. validate proof format and version;
3. validate chain/slot/epoch binding;
4. validate cryptographic proof;
5. derive bounded integer contribution;
6. reject invalid/expired/replayed proofs;
7. pass only validated contributions to Hybrid Selection.

PoI MUST use ATC-CONSENSUS-308. A legacy `poI_score` field is never sufficient.

## 6. Proposal validity

A proposal is valid only if:

- parent is known and eligible under fork-choice rules;
- consensus version is supported;
- PoH continuity is valid;
- proposer belongs to the selected validator snapshot;
- proposer selection matches ATC-CONSENSUS-304;
- all included consensus proofs are valid;
- transaction/state-transition validation succeeds;
- resulting state commitment is canonical.

Invalid proposals MUST NOT enter the attestation set.

## 7. Attestation validity

An attestation MUST bind at least:

- chain_id;
- consensus_version;
- target block hash;
- target height;
- target slot/epoch;
- validator identity;
- source/finality context required by the frozen voting rule.

The signature and all contextual fields MUST verify before stake weight is counted.

Two conflicting attestations from one validator in the same forbidden voting domain
constitute equivocation according to ATC-CONSENSUS-306/307.

## 8. Finality boundary

Finality MUST be evaluated against one explicitly selected validator-set snapshot.

Until ATC-CONSENSUS-306 freezes:
- quorum arithmetic,
- rounding,
- minimum participation,
- source/target voting rules,
- snapshot selection,
- conflict handling,

no block may be treated by this draft as protocol-final solely because an implementation
returns a numeric threshold.

Once a block is final, later fork-choice/state transitions MUST reject attempts to
replace or roll back the finalized prefix.

## 9. Fork choice

Fork Choice operates only on candidates that passed proposal validation.

The final total ordering is deliberately UNFROZEN. It MUST eventually define:
- finalized-prefix compatibility;
- chain/PoH weight;
- consensus contribution treatment;
- deterministic tie-breaking;
- invalid/equivocating candidate handling.

Node-local time, randomness and implementation-dependent ordering are prohibited.

## 10. State transition

The canonical state transition is:

```text
Consensus-valid block
    ↓
transaction execution
    ↓
economic/account/state updates
    ↓
validator lifecycle updates
    ↓
PoH/checkpoint state
    ↓
consensus/finality metadata
    ↓
canonical state root
```

All economic amounts MUST follow the canonical u128 amount model.
Counters such as height, nonce, timestamp and epoch remain u64 where their
respective contracts specify them.

Overflow, underflow, malformed encoding, invalid proof, invalid signature,
invalid state root or invalid consensus transition MUST reject the candidate.

## 11. Determinism invariant

For identical canonical input state and identical canonical candidate data:

`transition(S, candidate) = transition(S, candidate)`

across independent implementations.

The implementation MUST NOT depend on:
- HashMap/BTreeMap iteration where order is not part of the contract;
- floating point;
- host locale;
- host timezone;
- filesystem state;
- network responses;
- unpinned model/runtime behavior;
- non-versioned external oracle state.

## 12. Failure semantics

A rejected candidate MUST NOT partially mutate canonical consensus state.

Consensus processing MUST use transactional/rollback-safe application semantics:

```text
validate → stage → verify → execute → commit
                         ↘ reject → discard
```

A node crash before commit MUST leave the previous committed state intact.

## 13. Conformance vectors

Before freeze, the suite MUST contain at least:

- slot-context derivation;
- validator snapshot boundary;
- PoS contribution;
- PoW contribution;
- PoI valid/invalid/replay cases;
- deterministic proposer selection;
- invalid proposal;
- valid attestation;
- invalid signature;
- equivocation;
- quorum boundary;
- finalized-prefix reorg rejection;
- fork-choice tie;
- state-transition overflow;
- state-root mismatch;
- consensus-version mismatch;
- restart/recovery.

Every vector MUST be serializable canonically and independently reproducible.

## 14. Freeze gates

- [ ] All nine state-machine transitions have normative input/output contracts.
- [ ] PoH semantics frozen.
- [ ] PoS semantics frozen.
- [ ] PoW semantics frozen.
- [ ] PoI proof types and contribution semantics frozen.
- [ ] Hybrid weighting/scaling frozen.
- [ ] Proposal contract frozen.
- [ ] Attestation/voting contract frozen.
- [ ] Finality quorum/snapshot rules frozen.
- [ ] Fork-choice total ordering frozen.
- [ ] State-transition interface frozen.
- [ ] Cross-language golden vectors committed.
- [ ] Independent implementation differential tests pass.
- [ ] Security review complete.
- [ ] Exact-SHA conformance VERIFIED.

Until all gates are satisfied, ATC-CONSENSUS-309 is **NON-FROZEN** and cannot authorize a production mainnet consensus implementation.
