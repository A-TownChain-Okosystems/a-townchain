# F-02 / AD-02 — Consensus Ownership and Protocol Decision Draft

> **Status:** DRAFT — NOT APPROVED  
> **Decision label:** AD-02 is a working label for this audit package, not a registered normative identifier.  
> **Scope:** Ownership, migration evidence, protocol decision prerequisites and F-105 boundary.  
> **No runtime or protocol behavior is authorized to change by this document.**

## 1. Decision requested

Establish one production consensus implementation boundary and explicitly separate:
- production implementation ownership;
- normative protocol/specification authority;
- historical migration source and reference material;
- orchestration/integration responsibilities.

The decision must be approved by the accountable owner and the applicable §9 review chain before any normative registry update.

## 2. Evidence-backed current state

1. The migration manifest records `atc-algorithm` as imported into `a-townchain/components/algorithm`, with the source repository frozen at the recorded snapshot `69eeed5`.
2. The current repository inventory and capability entries do not express one unambiguous semantic owner. SCR-0131 records this ambiguity and is still PROPOSED.
3. The standalone and monorepo PoH and selection files have different blob SHAs and different behavior. This is code divergence, not proof of a live network fork.
4. The consensus specifications ATC-CONSENSUS-301/304/305/306/DET/ENC are drafts; they are not yet an approved normative protocol baseline.
5. The legacy `modules/atc-blockchain/consensus/` source tree contains consensus-like ATCLang material. F-105 remains OPEN.
6. The standalone `atc-algorithm/main` has moved 11 commits beyond the migration freeze anchor. The drift requires review; it does not by itself establish production execution.

## 3. Options

### Option A — Monorepo implementation boundary (recommended for owner review)

- Production implementation target: `a-townchain/components/algorithm`.
- `atc-algorithm`: frozen migration source / historical reference until its approved continuing role is explicitly established.
- `a-townchain/modules/atc-blockchain`: orchestration/integration only; no competing consensus implementation.
- Normative specification authority: the standards authority designated by the approved governance registry. Do not infer normative authority from repository naming.

Advantages: matches the migration manifest and current implementation path; reduces the risk of two production implementations.

Risks/residuals: requires SCR-0131 owner/§9 decision, registry reconciliation, consumer audit, and evidence for the exact source snapshot.

### Option B — Standalone implementation boundary

Keep `atc-algorithm` as production consensus implementation and treat the monorepo copy as non-production/reference material.

This conflicts with the recorded migration target and requires a separately approved reversal/migration plan, complete dependency and deployment inventory, and proof that production consumers use only the standalone source.

### Option C — Dual production implementations

Rejected. Two independently evolving consensus authorities are not acceptable because they can produce divergent block validity, proposer selection, or fork-choice outcomes.

## 4. Proposed ownership model — pending approval

| Responsibility | Proposed boundary | Approval state |
|---|---|---|
| Production consensus implementation | `a-townchain/components/algorithm` | PROPOSED |
| Protocol standards and normative requirements | Approved standards authority | OWNER/§9 confirmation required |
| `atc-algorithm` | Frozen migration source / historical reference unless separately approved | RESIDUAL |
| Blockchain orchestration | `a-townchain/modules/atc-blockchain` | No duplicate consensus authority |
| Conformance and evidence control | Existing standards/engineering governance | Existing gates unchanged |

This table is a proposal only. It does not change `repositories.yaml`, `dependencies.yaml`, or any generated registry view.

## 5. Protocol decisions intentionally left unresolved

No algorithm choice may be inferred from this ownership proposal. The following require a separately approved normative protocol specification:

- PoH hash primitive, input domain, canonical encoding and tick semantics;
- validator stake width, ordering, weight normalization and overflow behavior;
- PoS/PoW combination, proposer eligibility and deterministic selection;
- fork-choice tie-breaking and reorganization limits;
- finality quorum, validator-set snapshot and conflicting-certificate handling;
- genesis and upgrade compatibility, including protocol versioning.

Until these decisions are approved and test vectors exist, implementation conformance remains BLOCKED. Do not choose FNV-1a, ATC-HASH-001, SHA-256, `u64`, `u128`, or any hybrid formula merely to make the two code copies match.

## 6. SCR-0127 reference disambiguation

Two distinct artifacts currently share the SCR-0127 label in discussion:

- `atc-standards/change-requests/SCR-0127.md`: Architecture Layer & Sublayer Acceptance Standard, PROPOSED.
- `a-townchain#64`: a GitHub issue tracking the migration-completeness blocker; it is not a pull request and is not the standards SCR document.

The migration manifest's reference to Issue #64 should be interpreted as a tracking/evidence reference, not as proof that the standards SCR-0127 approves or governs the algorithm migration. Future evidence should cite the full repository/path or issue URL, not the SCR number alone.

## 7. F-105 boundary requirements

The boundary check must:
- recognize Rust, Python, TypeScript and ATCLang `.atc` source files;
- traverse deterministically;
- report out-of-boundary consensus-like source paths;
- preserve report mode while legacy material is inventoried;
- fail in strict mode when out-of-boundary sources remain;
- state explicitly that path detection is not proof of runtime reachability.

Strict mode must not be activated until the legacy inventory is resolved and the owner approves the enforcement transition. No existing required check may be removed or weakened.

## 8. Required acceptance evidence

Before AD-02 can become APPROVED/VERIFIED:

1. Owner and §9 approval record, with reviewer identities and decision date.
2. Approved protocol specification with immutable version and normative registry reference.
3. Consumer/dependency inventory showing which source is compiled and executed by node, mining, validator and testnet paths.
4. Exact branch HEAD commit SHA for every repository involved.
5. Path/symbol and blob SHA bindings for all consensus source files.
6. CI Run → Job → Step → exit code, logs and artifact SHA-256 at the exact reviewed commit.
7. Differential golden vectors for PoH, validator selection, overflow, encoding and fork-choice.
8. Multi-node tests for partition, recovery, equivocation, reorganization and finality conflict.
9. Independent consensus/cryptographic security review.
10. F-105 strict-gate evidence and no unresolved duplicate production implementation.

A green CI run alone does not establish protocol correctness or VERIFIED status.

## 9. Non-goals and safety constraints

- No changes to consensus rules, hash primitives, stake types, encoding, fork choice or finality.
- No registry or generated-view changes before owner/§9 approval.
- No repository archive, deletion, unfreeze or source movement.
- No automatic promotion of draft specifications to normative status.
- No CI, security, review or release-gate weakening.
- No claim of a demonstrated network fork or live consensus failure.

## 10. Current disposition

**BLOCKED / RESIDUAL.** This draft makes the decision alternatives and required evidence explicit. It is not an approval, migration completion, protocol freeze, or release authorization.
