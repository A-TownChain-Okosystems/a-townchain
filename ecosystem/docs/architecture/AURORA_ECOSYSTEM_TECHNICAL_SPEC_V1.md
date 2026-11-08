# Aurora & A-TownChain Ecosystem — Technical Architecture Specification

- **Specification ID:** ATC-ARCH-AURORA-001
- **Version:** 1.0.0-draft
- **Status:** PROPOSED — review required
- **Owner:** `a-townchain-ecosystem` (system integration architecture)
- **Normative standards SSOT:** `atc-standards`
- **Last evidence snapshot:** 2026-10-18
- **Change policy:** update this version and its evidence snapshot in a reviewed PR; do not infer implementation from architecture text.

## 1. Purpose and conformance language

This document defines the system-of-systems boundary between Aurora AI, compute workers, policy/authorization services, the A-TownChain transaction path, deterministic execution, persistence and user-facing applications. It supplements the existing [system architecture](../ARCHITECTURE.md); it does not replace component repositories' implementation/API SSOTs or the standards repository.

The terms **MUST**, **MUST NOT**, **SHOULD**, and **MAY** express normative requirements for implementations that claim conformance to this specification. This is a proposed specification until reviewed and accepted.

## 2. Canonical responsibility model: L0–L7 and X

| Domain | Responsibility | Authority and trust boundary |
|---|---|---|
| L0 — System & Network | Hardware/HAL, GlobusOS, ShivaCore, network transport and node runtime | Enforces system/process/network boundaries. MUST NOT silently elevate AI-agent privileges. |
| L1 — Data & Storage | Durable persistence, committed-state storage, history, indexer/read models | Persists committed state and derived views; MUST NOT decide consensus finality. |
| L2 — Blockchain Core & Consensus | Transactions, authorization entry, mempool, block proposal/validation, consensus, finality | Sole protocol authority for canonical chain state. |
| L3 — Deterministic Execution | ATCLang/IR/ABI, bytecode verifier, ATC-VM and deterministic state transitions | Computes candidate transitions under protocol rules; MUST be deterministic for consensus inputs. |
| L4 — Scaling & Execution Domains | Rollups, validity/settlement domains and specialized execution | Authority is limited to explicitly specified domain contracts; future/partial status must be explicit. |
| L5 — Protocol & Economic Domains | Identity, assets, oracle, compute, interoperability, marketplace, launchpad and contracts | Operates only under domain contracts, capabilities and policies. |
| L6 — AI & Intelligence | Aurora orchestration, agents, planning, RAG/memory and domain intelligence | Produces proposals/results; MUST NOT be consensus, kernel, wallet-approval or canonical-state authority. |
| L7 — Applications & UX | Genesis Engine, games/worlds, wallet UX, explorer and end-user applications | Presents and requests actions; MUST NOT bypass authorization or claim read-model data is canonical. |
| X — Cross-Layer Control Plane | Identity, capabilities, authorization, policy, governance, cryptography, audit/evidence, observability, compatibility | Cross-cutting controls and contracts, not an extra execution layer and not an independent state-transition authority. |

Layer numbering is a responsibility taxonomy, not a mandatory runtime order. A repository can span multiple layers. Each component repository remains authoritative for its own implementation, API/ABI, build, tests and release; `a-townchain-ecosystem` owns integration boundaries and evidence aggregation; `atc-standards` owns normative standards.

## 3. Trust invariants

1. **AI is advisory/orchestrating.** An AI output, memory item, agent identity or signed result MUST NOT by itself authorize a canonical state transition.
2. **Authorization is enforced at the authority boundary.** Policy and capability checks MUST fail closed on missing, invalid, expired or revoked credentials.
3. **Wallet approval is independent.** AI intent MUST NOT be treated as user approval or as a wallet signature.
4. **Execution and finality are distinct.** VM output is a candidate transition until accepted by protocol validation, consensus and finality.
5. **Read models are not authority.** Indexers, explorers, caches and AI memory MUST NOT override canonical committed state.
6. **Attestation is not correctness.** A signature proves attribution/integrity under a key; it does not alone prove a computation is correct.
7. **Evidence is commit-bound.** Build, test, security and E2E claims MUST cite the exact source SHA and run/result. A green run for a different SHA is not evidence for the assessed revision.
8. **Architecture is not implementation evidence.** A path, module, registry entry or document alone MUST NOT be used to claim an end-to-end capability.

## 4. Required interfaces and proposed contracts

The following are interface requirements to be made concrete in normative schemas under `atc-standards`. Field names below are a proposed minimum, not a claim that current implementations already satisfy them.

### 4.1 Agent identity and registration

Minimum identity record:
- `agent_id`: stable, unique identifier;
- `public_key` / credential reference and `issuer_id`;
- `capabilities`: explicit, bounded capability set;
- `policy_version`, `created_at`, `expires_at`, `status`;
- revocation reference and auditable registration event.

Requirements: duplicate IDs MUST be rejected; identity registration MUST be authenticated; expired/revoked agents MUST fail authorization; an agent MUST NOT self-grant capabilities; identity lifecycle changes MUST be auditable. A registry snapshot API is not equivalent to wired durable persistence.

### 4.2 Compute job, worker and run lifecycle

Minimum job envelope:
- `task_id`, `run_id`, `job_spec_version`, `input_hash`, `requested_capabilities`;
- `created_at`, `deadline`, `retry_policy`, `policy_version`;
- authenticated `worker_id` / `node_id`, lease ID and attempt number;
- result `artifact_hash`, status and provenance metadata.

Lifecycle MUST define submission, validation, queueing, lease, heartbeat, execution, timeout, retry, cancellation, recovery, result submission and terminal status. Duplicate, stale, expired-lease and unattributed results MUST be rejected or quarantined. Job state and lease/retry state MUST survive supported restarts. At least two independently operated workers are required to demonstrate a multi-node path; a local scheduler or multiple in-process agents is not sufficient evidence of decentralization.

### 4.3 Result attestation and verification

An attestation SHOULD bind `task_id`, `run_id`, `node_id`, `input_hash`, `artifact_hash`, method/version, timestamp/nonce and signer identity. Verifiers MUST check signature, identity/revocation, freshness, input/result binding and policy. Where correctness is not cryptographically or deterministically proven, the system MUST state the assurance model (e.g. independent recomputation, quorum, challenge/dispute or trusted execution) rather than presenting attribution as correctness. Failed or missing required verification MUST block settlement and any protected downstream action.

### 4.4 Policy and capability decision

A policy decision MUST bind subject/agent, requested action, resource, environment, policy version, decision, reason code and expiry/decision ID. Enforcement points MUST validate the decision and capability at use time, including revocation and replay protections. Failure to reach policy service MUST fail closed for protected operations. Policy decisions and enforcement outcomes MUST be auditable without exposing secrets.

### 4.5 Blockchain transaction boundary

A transaction request from Aurora/application MUST remain a proposal until the required authorization and wallet-signing process completes. The request should carry correlation IDs (`task_id`, `run_id`, `agent_id`, `policy_version`) as non-authoritative metadata where protocol support permits. The node MUST validate protocol-defined signatures, nonces, chain/domain separation, transaction rules and authorization. The VM MUST execute deterministically; consensus/finality MUST remain independent of AI output. Success MUST be reported only after the defined confirmation/finality condition, with transaction ID and evidence of final state. Rejection, timeout, revocation and reorg/failed-finality semantics must be explicit.

## 5. End-to-end reference flow

1. Aurora creates a task and correlation IDs.
2. Policy/capability enforcement authorizes the requested operation.
3. Scheduler persists the job and issues a bounded lease to an authenticated worker.
4. Worker executes and returns result plus provenance and attestation.
5. Independent verification validates attribution, freshness, integrity and the specified correctness assurance.
6. Results and decisions are durably recorded with source/version/evidence references.
7. If chain state change is requested, a separate wallet/authorization boundary obtains the required approval/signature.
8. Node validates the transaction; deterministic VM computes a candidate transition.
9. Consensus/finality accepts or rejects it; only committed state is canonical.
10. The caller receives terminal status tied to transaction ID, exact release/source SHA and evidence.

## 6. Repository evidence snapshot

These SHAs are the observed default-branch HEADs at the evidence snapshot, not automatically the SHAs used by every linked PR or CI run. The statuses below distinguish visible artifacts from proven end-to-end behavior.

| Repository | Observed HEAD | Evidence / role | Assessment |
|---|---|---|---|
| [a-townchain-ecosystem](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/commit/61635e988dcdb7afda1ef0bc48534d65149a1898) | `61635e988dcdb7afda1ef0bc48534d65149a1898` | [ARCHITECTURE.md](../ARCHITECTURE.md), [delivery registry](../control-plane/DELIVERY_REGISTRY.json), [panels](../control-plane/PANELS.md) | **IMPLEMENTED** as architecture/evidence artifacts; does not prove integrated runtime. |
| [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai/commit/648e8bbbb94c0facb140a4613dcba29b47165072) | `648e8bbbb94c0facb140a4613dcba29b47165072` | [README](https://github.com/A-TownChain-Okosystems/aurora-ai/blob/648e8bbbb94c0facb140a4613dcba29b47165072/README.md); [PR #46](https://github.com/A-TownChain-Okosystems/aurora-ai/pull/46), [PR #47](https://github.com/A-TownChain-Okosystems/aurora-ai/pull/47) | **IMPLEMENTED** for repository/primitives claimed by those artifacts; **OPEN** for production multi-node lifecycle and E2E proof. PR branch evidence must be assessed at its own head SHA. |
| [atc-compute](https://github.com/A-TownChain-Okosystems/atc-compute/commit/89d833edf338085d6141b0c703ebf92a0db83a5e) | `89d833edf338085d6141b0c703ebf92a0db83a5e` | [README](https://github.com/A-TownChain-Okosystems/atc-compute/blob/89d833edf338085d6141b0c703ebf92a0db83a5e/README.md), [architecture](https://github.com/A-TownChain-Okosystems/atc-compute/blob/89d833edf338085d6141b0c703ebf92a0db83a5e/ARCHITECTURE.md) | **OPEN** for proven production scheduler/worker/verification/settlement path; documentation or skeleton is not E2E evidence. |
| [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain/commit/a5e689671b1870699c7a1750f8a166e83590a41c) | `a5e689671b1870699c7a1750f8a166e83590a41c` | [README](https://github.com/A-TownChain-Okosystems/a-townchain/blob/a5e689671b1870699c7a1750f8a166e83590a41c/README.md), [mainnet readiness](https://github.com/A-TownChain-Okosystems/a-townchain/blob/a5e689671b1870699c7a1750f8a166e83590a41c/docs/MAINNET_READINESS.md) | **IMPLEMENTED** as source/readiness artifacts; **OPEN** for any readiness gates still listed open and for Aurora-to-finality E2E evidence. |
| [atc-node](https://github.com/A-TownChain-Okosystems/atc-node/commit/a817db43a089d172dd4688f366286c60dbf71a0e) | `a817db43a089d172dd4688f366286c60dbf71a0e` | [README](https://github.com/A-TownChain-Okosystems/atc-node/blob/a817db43a089d172dd4688f366286c60dbf71a0e/README.md) | **IMPLEMENTED** as a repository artifact; **OPEN** for full cross-layer acceptance evidence. |
| [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards/commit/f2b9b95723bbf7cd4f85c386940e1130351c59c6) | `f2b9b95723bbf7cd4f85c386940e1130351c59c6` | [findings registry](https://github.com/A-TownChain-Okosystems/atc-standards/blob/f2b9b95723bbf7cd4f85c386940e1130351c59c6/registry/findings.yaml) | **IMPLEMENTED** as standards/evidence artifact; **OPEN** until the contracts in §4 are normative, versioned and adopted by implementations. |

### Status definitions

- **IMPLEMENTED** — the identified source/documentation artifact exists at the cited revision; this label alone does not assert correctness, integration or production readiness.
- **VERIFIED** — a named acceptance test passed against the exact cited source SHA, with a retrievable run/result.
- **OPEN** — required contract, integration, test, evidence or decision remains unproven/unresolved.
- **AUDITED** and **PRODUCTION_READY** require separate documented criteria and evidence; neither follows from IMPLEMENTED or VERIFIED alone.

A future evidence update MUST add exact workflow/run URLs and tested SHAs before marking any behavior VERIFIED. The evidence snapshot is intentionally conservative: it does not claim a fully integrated decentralized intelligence network.

## 7. Prioritized implementation backlog

Create/track the following work packages in GitHub. Dependencies are expressed by issue title until issue numbers are assigned.

| Priority | Work package | Depends on | Exit evidence |
|---|---|---|---|
| P0 | Normative identity, job, attestation, policy and transaction schemas in `atc-standards` | None | Versioned schemas, compatibility policy, negative tests and review approval |
| P0 | Aurora agent registry durability, revocation and task lifecycle | Normative identity/job contract | Restart/recovery tests; duplicate/revoked/stale identity tests; exact-SHA CI |
| P0 | Evidence registry links artifacts to exact SHAs and test runs | None | Validator rejects missing/mismatched SHA or run; stale evidence is explicit |
| P1 | Multi-node compute worker path | Normative job/attestation contracts; Aurora lifecycle | Two independent workers, leases/heartbeats, retries and recovery demonstrated |
| P1 | Independent result verification and settlement guard | Attestation contract; multi-node path | Invalid/missing/stale attestation blocks settlement; positive and adversarial tests |
| P1 | Aurora-to-chain vertical E2E with wallet/policy boundary | Transaction contract; verified worker result | End-to-end trace through authorization, VM, consensus/finality and canonical state |
| P2 | Sybil/fraud resistance, dispute model, reputation and economics | Multi-node path; verification model | Threat model, measurable controls, abuse tests and documented residual risks |

## 8. Proposed acceptance criteria

The architecture is not considered end-to-end verified until evidence demonstrates all applicable criteria:

- [ ] Agent IDs are unique, authenticated, revocable and persistent across restart.
- [ ] `task_id`, `run_id`, `agent_id`, `node_id`, `artifact_hash` and `policy_version` are correlated across the full trace.
- [ ] Two independently operated workers can receive, execute and return a job through a persistent queue/lease lifecycle.
- [ ] Duplicate, stale, expired-lease, revoked and unattributed results are rejected or quarantined.
- [ ] Required attestation/verification failure blocks settlement and protected downstream actions.
- [ ] Aurora cannot bypass wallet signing, policy/capability checks, deterministic execution or consensus.
- [ ] Chain-action success is reported only after defined finality, with transaction and canonical-state evidence.
- [ ] Restart, timeout, worker loss, network partition and retry tests converge to defined terminal states without duplicate settlement.
- [ ] CI/security/E2E results are linked to the exact release candidate SHA; required gates are green.
- [ ] Status transitions to VERIFIED/AUDITED/PRODUCTION_READY have explicit criteria and durable evidence.

## 9. Open decisions

1. Which schema/versioning format is canonical for job envelopes and attestations?
2. What is the required result-correctness assurance per job class: deterministic recomputation, quorum, challenge/dispute, trusted execution or another explicit model?
3. What are the authoritative identity issuer, key rotation and revocation semantics across repositories?
4. What are lease duration, heartbeat, retry, cancellation and exactly-once settlement semantics?
5. Which chain event/finality definition does the E2E gate require?
6. Which workflows are mandatory release gates and where is the machine-readable gate contract owned?
7. Which privacy, retention and audit redaction rules apply to prompts, inputs, outputs and worker metadata?

## 10. Change control

This file is a draft on a feature branch. Merge requires review of the architecture/standards owners, resolution or explicit acceptance of open decisions, validation of evidence links and applicable repository gates. No PR or issue creation implies that the implementation is already complete. No merge is authorized by this document.
