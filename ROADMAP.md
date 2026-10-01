# ROADMAP — a-townchain

> A-TownChain Blockchain (Orchestration) · Status bound to .atc/evidence/evidence.yaml
> STATUSLEITER: SPECIFIED → IMPLEMENTED → TESTED → VERIFIED → AUDITED → RELEASED
> CLAIMED ≠ PASS

## Current P0 — close the L1 contract

- [ ] Monetary contract: system-wide economic u128 / counter u64 reconciliation
- [ ] Canonical TX V2: exact byte layout frozen and consumed by Rust + TypeScript + Node
- [ ] Crypto P0: exact-SHA CI, conformance vectors and security negatives green
- [ ] Resolve normative crypto reconciliation through atc-standards#53
- [ ] Define and freeze separate consensus-key contract
- [ ] Migrate Node transaction signing/verification to the canonical TX contract
- [ ] Remove or quarantine legacy transaction-domain/signing paths
- [ ] Rust ↔ TypeScript ↔ Node byte-for-byte conformance

## L1 operational readiness

- [ ] Consensus implementation + finality conformance
- [ ] Mainnet genesis artifact, hash and initial state reproducibility
- [ ] Authenticated production P2P and multi-node operation
- [ ] Mempool admission and replay protection
- [ ] State sync + state-root verification
- [ ] Restart/resync/recovery
- [ ] Deterministic ATC-VM verifier/execution/state-transition conformance
- [ ] Storage durability and recovery
- [ ] Indexer/explorer consistency against finalized state

## Security and release

- [ ] Protocol/cryptographic/consensus/VM/P2P security reviews
- [ ] Independent security audit
- [ ] Critical/high findings remediated and evidenced
- [ ] Reproducible signed release artifact
- [ ] Supply-chain/provenance/SBOM evidence
- [ ] One immutable release-candidate SHA with all mandatory gates green

## Mainnet gate

**NO-GO until every mandatory gate reaches RELEASED on the same release candidate.**

The authoritative readiness matrix is docs/MAINNET_READINESS.md.

Historical CI, file existence, documentation claims, or devnet smoke tests do not constitute mainnet evidence.