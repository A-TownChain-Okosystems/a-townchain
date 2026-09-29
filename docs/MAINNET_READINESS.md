---
document_id: ATC-DOC-MAINNET-READINESS-001
title: "Mainnet Readiness and Start Gates"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-29
updated: 2026-09-29
standard: ATC-STD-MD-001
---

# Mainnet Readiness and Start Gates

## 1. Purpose

This document is the operational readiness summary for an A-TownChain mainnet start. It separates architecture, implementation, test evidence, security audit, release evidence, and mainnet authorization.

**Current decision: MAINNET NO-GO.**

No source file, document, or historical CI run constitutes current mainnet evidence. Readiness requires evidence bound to the exact release candidate SHA.

## 2. Evidence ladder

`SPECIFIED → IMPLEMENTED → TESTED → VERIFIED → AUDITED → RELEASED`

A claim may not skip a level. Historical CI results do not verify a different SHA.

## 3. Current P0 gates

| Gate | Required condition | Current state |
|---|---|---|
| Monetary contract | Economic quantities u128; counters/resource limits u64 | IN PROGRESS |
| Canonical TX contract | V2 bytes, chain ID 658467, exact field encoding | IN PROGRESS |
| TX crypto | secp256k1/ECDSA, SHA-256, RFC6979, Low-S | IMPLEMENTED ON P0 BRANCH; CI PENDING |
| Rust/TypeScript conformance | byte-for-byte preimage equality + signature vectors | PENDING |
| Negative security tests | High-S, malformed, legacy-domain, cross-purpose rejection | PARTIAL / PENDING CI |
| Consensus contract | frozen consensus state machine + key contract + finality | PENDING |
| Genesis | immutable mainnet genesis artifact + reproducible hash/state | PENDING |
| Node/P2P | authenticated production networking and multi-node operation | PENDING |
| State sync | verified initial sync, restart, resync and state-root equality | PENDING |
| VM | deterministic verifier/execution/state-transition conformance | PENDING |
| Release | reproducible artifact, provenance and pinned dependencies | PENDING |
| Security | independent protocol/crypto/implementation audit | NOT AUDITED |
| E2E | Wallet → SDK → Node → Mempool → Consensus → VM | PENDING |
| Exact-SHA release evidence | all required gates green on one release candidate SHA | PENDING |

## 4. Crypto contract

Transaction/account protocol signing is:
- secp256k1 ECDSA
- SHA-256 over exact canonical V2 signing bytes
- RFC6979 deterministic nonce
- Low-S canonicalization
- fixed 64-byte r || s
- compressed SEC1 public key, 33 bytes
- numeric chain ID 658467
- economic fields encoded as u128
- counters/resource fields encoded as u64

The reusable crypto primitive does not own transaction semantics. ATC-TX-DOMAIN-V2 and canonical transaction serialization belong to the transaction signing/encoding layer.

Purpose separation remains mandatory: `Transaction Key ≠ Consensus Key ≠ Node Identity ≠ Service Identity ≠ Aurora Agent Identity`.

Ed25519 is not globally removed; it remains permitted only for explicitly purpose-bound contracts.

## 5. Canonical transaction encoding

The current P0 contract is:

`ATC-TX-DOMAIN-V2 → chain_id u64 BE → tx_type u8 → sender_did u32-length + bytes → recipient_did optional flag + u32-length + bytes → amount u128 BE → gas_price u128 BE → gas_limit u64 BE → nonce u64 BE → timestamp u64 BE → payload u32-length + bytes → poh_hash 32 bytes`

Network ID, genesis ID, protocol version and VM version are validated runtime context and are not silently inserted into the V2 preimage.

## 6. Consensus and finality gate

Mainnet requires a frozen, independently reviewable consensus contract defining validator identity and consensus-key type, validator admission/removal, proposal rules, vote rules, quorum/finality, equivocation handling, timeout/liveness behavior, fork choice, epoch/rotation rules, slashing, restart/recovery semantics, and deterministic replay.

No mainnet validator set is authorized until these contracts and their implementation evidence agree.

## 7. Genesis and bootstrap gate

The mainnet genesis artifact must be immutable and reproducible, including canonical genesis bytes, genesis hash, chain ID 658467, mainnet network identity, protocol version, VM version, initial validator/consensus configuration, initial state/allocation data where applicable, initial state root or deterministic derivation, and artifact checksum/provenance.

Every mainnet node must reject a mismatching genesis.

## 8. Node/network gate

Production node readiness requires authenticated peer handshake, chain/genesis/protocol compatibility checks, peer discovery/bootstrap, transaction propagation, block propagation, consensus vote propagation, rate/resource limits, partition handling, reconnect behavior, deterministic state adoption, and required transport security.

Devnet-only RPC, gossip, or thread-based multi-node smoke tests are not mainnet evidence.

## 9. State/recovery gate

A release candidate must demonstrate: `fresh node → genesis verification → sync → state-root verification → ready`.

It must also demonstrate: `running node → crash → restart → local recovery/resync → identical canonical state`.

State snapshots, replay, rollback boundaries and backup/recovery procedures must be versioned and verified.

## 10. VM gate

The canonical execution path is: `ATCLang → ATC-IR/ABI → ATC Bytecode → Verifier → ATC-VM → State Transition`.

Required evidence includes deterministic execution, verifier fail-closed behavior, gas/OOG handling, storage limits, state-transition atomicity, replay equivalence, and state-root equivalence across nodes.

## 11. Release and supply-chain gate

Before release: exact dependency versions/revisions are pinned as required; build is reproducible; artifacts are cryptographically attributable; release signatures are verified; provenance/SBOM evidence is retained; no undocumented local patches are required; and the release candidate SHA is immutable for final evidence.

## 12. Security gate

Required before mainnet: protocol threat-model review, cryptographic review, consensus review, VM/security-boundary review, P2P/DoS review, key-management review, dependency/supply-chain review, independent security audit, and remediation evidence for all critical/high findings.

## 13. E2E launch gate

The minimum canonical path is: `Wallet → SDK → Node → Mempool → Signature Verification → Consensus → ATC-VM → State → Storage → Indexer`.

Required: multiple nodes, finality, restart/resync, invalid transaction rejection, signature/domain rejection, deterministic state convergence, and exact release-SHA evidence.

## 14. Current blockers

1. Consensus contract and implementation are not frozen and independently verified.
2. Mainnet genesis/bootstrap artifact is not established as the immutable release source.
3. Production P2P and multi-node consensus operation are not evidenced.
4. State sync/restart/recovery is not evidenced as a complete mainnet path.
5. VM verifier/execution/state-transition conformance is not complete.
6. System-wide monetary u128 reconciliation is not complete.
7. Rust ↔ TypeScript ↔ Node transaction conformance is not complete.
8. Independent security audit is outstanding.
9. A single exact-SHA release candidate with all mandatory gates green does not yet exist.
10. Mainnet release authorization and evidence bundle are therefore not available.

## 15. Non-blocking / deliberately deferred

Aurora, Genesis Engine, Genesis Chronicles, marketplace/launchpad, and desktop/UX feature completeness do not by themselves block the L1 start. They become launch blockers where they are part of the L1 security, governance, economic, deployment, or operational boundary.

## 16. Gate rule

**No evidence, no trust.** A component may be described as implemented only when the implementation is present and its applicable tests/evidence support that claim. A mainnet release requires the entire mandatory path to reach RELEASED on one immutable release candidate SHA.