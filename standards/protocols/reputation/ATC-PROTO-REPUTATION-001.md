---
protocol:
  id: ATC-PROTO-REPUTATION-001
  name: "ATC Reputation Protocol"
  version: 1.0.0
  status: draft
  domain: REPUTATION
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-REPUTATION-001 — Reputation Protocol v1.0.0

## §1 Scope
Defines deterministic reputation records, evidence, score transitions, decay, dispute handling, anti-sybil constraints, privacy boundaries and evidence references.

## §2 Existing-first evidence
Gate 0 searched the canonical ShivaCore repository for reputation, trust, score, identity and validator-reputation implementation traces. No exact canonical source result was returned in this pass. This specification therefore makes no implementation/conformance claim.

## §3 Identity binding
Reputation MUST bind to the canonical identity/reference defined by ATC-PROTO-IDENTITY-001. Reputation MUST NOT substitute for authentication, transaction authorization or consensus authority.

## §4 Evidence
Every score-affecting transition MUST reference deterministic evidence metadata. Evidence MUST be immutable by reference, domain-separated and replay-protected. Unsupported or unverifiable evidence MUST NOT change canonical state.

## §5 Score model
The activated score function MUST use integer or fixed-point arithmetic. Floating-point arithmetic MUST NOT participate in consensus-critical scoring. Bounds, overflow behavior and rounding MUST be frozen before activation.

## §6 Updates and decay
Updates MUST be deterministic and ordered. If decay is activated, its epoch/time basis, rate, rounding and bounds MUST be frozen. No wall-clock ambiguity is permitted.

## §7 Sybil resistance
Creating additional identities MUST NOT by itself increase reputation authority. Any identity weighting, stake/deposit requirement or other anti-sybil mechanism MUST be explicit before activation.

## §8 Disputes and corrections
Dispute and correction events MUST be deterministic and auditable. A correction MUST preserve a link to the original evidence and state transition.

## §9 Privacy
Sensitive evidence SHOULD remain off-chain where possible. Canonical state SHOULD store commitments/references rather than unnecessary personal data.

## §10 Resource limits
Evidence references, update frequency, record size, dispute depth and per-identity state MUST be bounded.

## §11 Conformance vectors
Positive vectors MUST cover initial state, accepted evidence, repeated updates, ordering and identity binding. Negative vectors MUST cover malformed evidence, replay, overflow, unsupported evidence, duplicate correction and unauthorized identity changes. Cross-language implementations MUST produce identical state transitions.

## §12 Activation blockers
Status remains draft until the exact score formula, decay parameters, evidence schema, anti-sybil model, dispute semantics, canonical storage owner and exact-SHA conformance tests are frozen and verified.
