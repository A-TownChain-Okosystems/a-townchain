# ATC-COMP-500..509 — Requirements-to-Implementation Matrix

**Status:** implementation planning artifact; the component specifications remain `SPEC-DRAFT`.
**Scope:** map each draft contract to the current source baseline, missing implementation, security consequence, and acceptance evidence before protocol code is treated as conformant.
**Authority:** this matrix does not supersede the ATC-COMP specifications, canonical A-TownChain consensus/economics, or the architecture recorded in ecosystem issue #121.

## Current source baseline

- Repository: `A-TownChain-Okosystems/atc-compute`
- Audited baseline: `d0831dfb72590537d8b0ca4d435a3649e71cdb61`
- `src/lib.rs` exports only the scheduler module.
- `src/scheduler.rs` implements a FIFO slot scheduler; it is not a job protocol, worker runtime, attestation verifier, or settlement implementation.
- Existing scheduler tests cover basic slot dispatch and an unknown-job finish attempt only.
- `.atc/evidence/evidence.yaml` binds an older test run to commit `5d7ee0efa053a52628e605a8759971aedb206bc2`; it is not evidence for the audited baseline.
- The canonical consensus and monetary-policy implementation remains in `a-townchain`. Compute must not create an alternate issuance or validator-reward policy.

## Requirements matrix

| Spec | Contract area | Existing source coverage | Required implementation | Principal risk if absent | Required evidence |
|---|---|---|---|---|---|
| ATC-COMP-500 | Architecture and trust boundaries | Architecture docs only; scheduler skeleton | Define module boundaries and versioned interfaces between job API, scheduler, worker, verifier, attestation, and settlement adapter. Keep execution off-chain and consensus authority independent. | Duplicate SSOT; AI output influencing consensus; confused-deputy authority | Architecture review; dependency-boundary tests; repo ownership and API/ABI mapping |
| ATC-COMP-501 | Canonical job schema and lifecycle | No canonical job type/hash/state machine in source | Versioned job schema; canonical encoding; job hash; explicit state transitions; bounded resource budget; owner/policy-authorized cancellation; idempotent requests | Hash ambiguity, invalid transitions, resource exhaustion, duplicate work | Positive/negative schema vectors; canonical-encoding vectors; transition table tests; replay/idempotency tests |
| ATC-COMP-502 | Scheduling | FIFO slot scheduler only | Validate nonzero and bounded slot requests; reject duplicate IDs; prevent capacity underflow/overflow; specify queue fairness, cancellation, retries, deadlines, and cleanup semantics | Resource-accounting corruption; starvation; stuck queue; duplicate execution | Boundary tests (0, max, max+1); duplicate-ID tests; invariant/property tests; deterministic scheduling tests |
| ATC-COMP-503 | Worker execution | No worker implementation | Worker lifecycle; runtime/artifact pinning; sandbox and resource enforcement; timeout/cancellation; structured execution record | Arbitrary code execution, data leakage, resource abuse, irreproducible results | Sandbox tests; timeout/cancel tests; resource-limit tests; pinned-runtime integration tests |
| ATC-COMP-504 | Resource metering | No authoritative metering contract | Versioned resource record with bounded integer units, provenance, missing-meter behavior, and checked arithmetic | Provider-invented usage; overflow; disputed or inflated charges | Unit-conversion vectors; overflow/negative-input rejection; metering consistency tests |
| ATC-COMP-505 | Signed attestation | No attestation type or verifier | Signed payload containing job/input/result/runtime/model/environment digests, resource record, worker key, nonce, and slot/time; domain `atc-compute.attestation.v1`; replay rejection | Forged results, replay, cross-protocol signature reuse | Signature vectors; tamper tests; domain-separation tests; duplicate nonce/slot tests |
| ATC-COMP-506 | Result verification | No task verifier or quorum logic | Task-class-specific policy; deterministic re-execution where feasible; AI acceptance rules; independent-worker quorum where specified; fail-closed PASS/REJECT decisions | False acceptance, correlated/colluding workers, undefined probabilistic output validity | PASS/REJECT vectors; verifier adversarial tests; quorum/collusion cases; deterministic state-hash tests |
| ATC-COMP-507 | Determinism | No canonical protocol state hash | Canonical serialization and deterministic transition/accounting rules; isolate nondeterministic AI execution from protocol state | Platform-dependent outcomes and consensus divergence | Cross-run/cross-platform vectors; canonical state-hash vectors; replayed-transition equivalence |
| ATC-COMP-508 | Threat model and controls | No component-level security implementation/audit | Threat model and controls for Sybil, replay, collusion, duplicate claims, worker compromise, input/output privacy, licensing, and fail-closed errors | Fraud, double payment, data exfiltration, unsafe artifact execution | Threat-model review; negative/adversarial tests; dependency/security audit; residual register |
| ATC-COMP-509 | Economics and settlement interface | No escrow/settlement code | User-funded escrow by default; pay only after PASS; deterministic refund/failure handling; budget ceiling; integer micro-ATC arithmetic; consume canonical chain economics without duplicating issuance | Double payment, unfunded liabilities, inflation, rounding exploits | Arithmetic vectors; budget-conservation invariant; replay/double-claim tests; integration evidence against canonical settlement API |

## Protocol invariants to preserve

1. **Independent consensus:** useful AI compute does not confer validator status, block weight, finality power, or canonical-state authority.
2. **Off-chain execution:** AI workloads execute outside ATC-VM and validator consensus paths. On-chain operations are bounded settlement/state transitions only.
3. **Fail closed:** missing/invalid policy, signature, task evidence, budget, authorization, or replay metadata cannot produce a successful acceptance or payment.
4. **Separate accounting:** compute payments, transaction fees, validator rewards, and issuance remain distinct categories.
5. **Canonical monetary policy:** preserve the 360,000,000 ATC cap and 18 decimals. Do not invent reward rates, fee splits, slashing percentages, or new issuance.
6. **No legacy protocol assumption:** ATC-15's legacy coupling of AI work to network security and its 21M supply claim are not authoritative for Variant A; reconcile through a reviewed change, not parallel rules.
7. **Evidence discipline:** statuses must distinguish SPECIFIED, IMPLEMENTED, TESTED, CI_VERIFIED, INTEGRATED, and E2E_VERIFIED. A verification claim requires an exact commit SHA and terminal Run → Job → Step → exit/log evidence.

## Freeze blockers before protocol conformance claims

- Resolve canonical encoding and exact job-hash preimage in ATC-COMP-501.
- Define what qualifies as an independent worker and how correlated AI outputs are handled in ATC-COMP-506; a sample quorum such as 2-of-3 is not a finalized protocol constant.
- Freeze resource units, rounding, and meter provenance in ATC-COMP-504/509.
- Define lifecycle semantics for cancellation, timeout, retry, partial completion, dispute, and refund across ATC-COMP-501/502/509.
- Specify escrow and settlement API ownership in canonical `a-townchain`; this repository must not implement a competing ledger or issuance path.
- Complete the threat model and test vectors before declaring the drafts frozen.

## Implementation sequence

1. Review and freeze the contract decisions above; record explicit decisions rather than silently selecting protocol constants.
2. Harden the existing scheduler invariants and add boundary/property tests.
3. Implement canonical job schema, lifecycle, and deterministic encoding with vectors.
4. Add worker execution/resource records and signed attestations.
5. Add task-specific verifier interfaces and fail-closed decision logic.
6. Integrate through a narrow settlement adapter to canonical A-TownChain interfaces; keep consensus and issuance out of this repository.
7. Bind tests and security evidence to the exact tested HEAD. Do not claim production readiness while any required gate is open.

## Status

This document is an implementation matrix, **not evidence that the listed missing functionality exists**. No production conformance, security audit, integration, or mainnet readiness is claimed.
