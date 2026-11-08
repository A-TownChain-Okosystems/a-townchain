---
protocol:
  id: ATC-PROTO-MINING-001
  name: "ATC Mining Protocol"
  version: 1.0.0
  status: draft
  domain: MINING
  layer: L3/L5
  chain_id: 658467
  umbrella: ATC-STD-PROTOCOL-001
---

# ATC-PROTO-MINING-001 — Mining Protocol v1.0.0

## §1 Scope
Defines deterministic Proof-of-Work mining inputs, proof validation, difficulty, nonce handling, reward eligibility and duplicate-claim protection. It does not override canonical block or monetary-supply contracts.

## §2 Existing-first evidence
Gate 0 found mining architecture evidence at A-TownChain source SHA `51f0344a412d351359dff014a587b07c50ecf42b`: mining documentation describes block-header/difficulty job distribution, nonce/proof return and reward claims. The same SHA contains a TypeScript PoW fallback loop in the blockchain engine. This is implementation evidence, not conformance evidence.

## §3 Canonical mining domain
Every mining proof MUST bind to chain_id 658467 and the exact canonical block candidate. The proof domain MUST include canonical header fields and nonce according to the activated block encoding.
Mining MUST NOT mutate canonical state merely by discovering a proof; state mutation occurs only through canonical block acceptance.

## §4 Proof validation
A node MUST independently recompute the proof from canonical bytes and reject wrong chain, malformed proof, mismatched candidate, invalid nonce, insufficient work, stale/replayed proof, or invalid ancestry. Validation MUST be deterministic and independent of miner-provided difficulty claims.

## §5 Difficulty
Difficulty/target MUST use exact integer encoding. Floating-point difficulty is prohibited. Adjustment algorithm, target bounds, retarget interval and timestamp constraints MUST be frozen before activation. Local miner configuration MUST NOT alter consensus difficulty.

## §6 Nonce
Nonce encoding MUST have fixed width and byte order. Exhaustion MUST be deterministic. Search-space partitioning MUST not change validation results.

## §7 Rewards
Mining rewards MUST use integer base units and respect MAX_SUPPLY 360,000,000 ATC. Reward calculation MUST not exceed remaining issuance. The exact reward schedule MUST come from the canonical monetary/emission contract; this protocol does not invent one.

## §8 Claims and replay
Every reward claim MUST bind to a unique accepted block/proof context. Duplicate claims MUST be rejected. A proof for an orphaned/rejected block MUST NOT create canonical reward state.

## §9 Resource limits
Proof/header size, verification cost and claim processing MUST be bounded.

## §10 Conformance
Positive vectors MUST cover valid proof, boundary target, nonce encoding, accepted reward and unique claim. Negative vectors MUST cover wrong chain, wrong parent, changed header, insufficient work, malformed nonce, stale proof, duplicate claim, orphaned proof and supply-overflow reward. Cross-language implementations MUST agree on canonical mining bytes, proof result, target comparison and reward amount.

## §11 Activation
Status remains draft until exact implementation source is normalized into the canonical mining contract, PoW hash/domain and target representation are frozen, reward linkage is exact, and exact-SHA CI conformance evidence exists.