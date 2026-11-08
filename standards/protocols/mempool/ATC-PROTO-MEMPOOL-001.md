---
protocol:
  id: ATC-PROTO-MEMPOOL-001
  name: "ATC Mempool Protocol"
  version: 1.0.0
  status: draft
  domain: MEMPOOL
  layer: L3
  chain_id: 658467
  authority: A-TownChain-Okosystems
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-MEMPOOL-001 — Mempool Protocol v1.0.0

## §1 Scope

Defines deterministic admission, replacement, ordering, eviction, capacity, replay protection and block-selection rules for pending A-TownChain transactions. Transaction encoding/signing is defined by ATC-PROTO-TX-001; block inclusion is finalized by ATC-PROTO-BLOCK-001 and ATC-PROTO-CONSENSUS-001.

## §2 Existing-first evidence

The protocol registry records an existing mempool.rs reference-implementation trace in ShivaCore. Repository search confirms the registered implementation path in the ShivaCore file register, but the current standards repository does not contain a formal MEMPOOL protocol specification. This document therefore formalizes the contract without claiming implementation conformance.

## §3 Admission

A node MUST perform canonical transaction decoding and signature/domain/chain validation before admission. It MUST reject malformed transactions, wrong chain ID, legacy transaction domain, invalid signatures, nonce violations and amounts outside the canonical u128 range.

Admission MUST be deterministic and MUST NOT mutate account state.

## §4 Identity and duplicate protection

The canonical transaction ID MUST be derived from the canonical transaction bytes/signing contract. A transaction already present in the pool MUST NOT be admitted twice.

Transactions with conflicting (sender, nonce) MUST follow one deterministic replacement rule. An implementation MUST NOT silently keep two conflicting candidates as simultaneously includable.

## §5 Ordering

Mempool ordering MUST be deterministic for identical pool state. Ordering MUST NOT depend on hash-map iteration order, thread scheduling, locale or wall-clock formatting.

The final ordering key MUST be frozen before protocol activation. Until then, implementations MUST expose the chosen policy through conformance configuration rather than inventing consensus semantics.

## §6 Capacity and eviction

A node MUST enforce a bounded transaction count and/or bounded byte capacity. Limits MUST be explicit configuration values.

Eviction MUST be deterministic and MUST protect already-valid executable transactions from starvation. Expired, invalid or permanently non-executable transactions MAY be removed immediately.

An eviction decision MUST NOT alter blockchain state.

## §7 Nonce handling

For each sender, transactions MUST respect the account nonce contract. Gaps MAY remain pending only if the implementation explicitly supports queued future nonces; otherwise they MUST be rejected.

When multiple transactions from one sender exist, executable nonce order MUST be preserved during block selection.

## §8 Fees and replacement

Economic amounts MUST use the ATC u128 contract. gas_used/nonce-like counters remain u64 where defined by their owning protocol.

If fee-based replacement is enabled, the minimum replacement condition MUST be deterministic and encoded in conformance vectors. Nodes MUST NOT use floating-point arithmetic for admission or replacement.

## §9 Block selection

Block selection MUST consume only transactions that pass final transaction validation. The selected sequence MUST be deterministic for identical mempool and state inputs.

Selection MUST revalidate state-dependent constraints because account state may have changed after admission.

## §10 Replay and expiry

Chain ID, transaction domain, transaction ID and account nonce provide replay controls. Optional local expiry MAY remove stale transactions, but expiry MUST NOT change transaction validity on-chain.

## §11 Resource security

Implementations MUST bound transaction size, pool memory, per-sender occupancy and validation work. Repeated invalid submissions MUST be rate-limited without changing canonical transaction semantics.

## §12 Conformance vectors

Positive: canonical transaction admission; duplicate rejection; sequential sender nonces; deterministic ordering; valid replacement where enabled; deterministic block selection.

Negative: wrong chain; legacy domain; malformed encoding; invalid signature; duplicate ID; conflicting nonce without valid replacement; u128 overflow; pool-capacity overflow; nondeterministic ordering.

Cross-language conformance MUST compare exact transaction IDs, admission result, ordering key and selected transaction sequence.

## §13 Status

Status remains draft. Promotion requires exact-SHA implementation evidence, frozen ordering/replacement semantics and protocol conformance evidence under ATC-STD-PROTOCOL-002.
