---
document_id: ATC-DOC-TOWNCHAIN-004
title: "Project Status"
version: 1.2.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-08
updated: 2026-09-29
standard: ATC-STD-MD-001
---

# Project Status — ATC A-TownChain Core

| Property | Value |
|---|---|
| Repository | a-townchain |
| Version | 0.1.0 |
| Status | development |
| Implementation | partial / prototype |
| Build | exact-SHA evidence exists for individual gates; no release-candidate green SHA |
| Tests | individual gates pass; complete launch suite not verified |
| Security | NOT AUDITED |
| Conformance | IN PROGRESS |
| Production Readiness | NO-GO |
| Mainnet Date | NONE APPROVED |
| Current Documentation Update | 2026-09-29 |

## Status Summary

a-townchain remains in development. The core contains working components and verified development/test gates, but the complete L1 mainnet path is not yet implemented and evidenced as one production release.

The canonical readiness source is docs/MAINNET_READINESS.md.

## P0 progress

The current crypto P0 work on PR #32 establishes the transaction/account trusted core around secp256k1 ECDSA, SHA-256, RFC6979 and Low-S. The canonical transaction encoding now uses u128 for economic fields and u64 for counters/resource limits.

This work is NOT yet a mainnet release. SDK/Node integration, Rust↔TypeScript conformance, consensus-key contract, mainnet genesis, production networking, state recovery, VM conformance, independent security audit and the final exact-SHA release gate remain open.

Purpose separation is mandatory: Transaction Key ≠ Consensus Key ≠ Node Identity ≠ Service Identity ≠ Aurora Agent Identity.

Ed25519 is not globally removed; it remains available only where explicitly defined by a purpose-bound identity contract.

## Mainnet release gate

Mainnet remains NO-GO until all mandatory gates reach RELEASED on one immutable release-candidate SHA.

Minimum gates:
1. canonical monetary and transaction contracts
2. frozen consensus/finality and consensus-key contract
3. reproducible mainnet genesis/bootstrap
4. production P2P and multi-node operation
5. state sync, restart and recovery
6. deterministic ATC-VM execution/state-transition conformance
7. Rust↔TypeScript↔Node transaction conformance
8. independent security audit and remediation
9. reproducible signed release artifacts
10. complete Wallet → SDK → Node → Mempool → Consensus → VM → State → Storage path
11. exact-SHA evidence bundle

Historical CI results or source-file presence do not satisfy the release gate.

## Evidence rule

SPECIFIED → IMPLEMENTED → TESTED → VERIFIED → AUDITED → RELEASED

**No evidence, no trust.**

For the detailed gate matrix, see docs/MAINNET_READINESS.md.