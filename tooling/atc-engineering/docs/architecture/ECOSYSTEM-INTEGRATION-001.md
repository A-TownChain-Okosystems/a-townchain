# ECOSYSTEM-INTEGRATION-001 — A-TownChain System Integration

## Purpose

This document defines the executable integration boundary for the repository fleet. The Rust crate `atc-integration` is the machine-readable source for the dependency graph; this document explains the contract semantics.

## Canonical flow

```text
atc-standards
      │ standards-registry
      ▼
atc-engineering ───────────── governed-build-evidence ─────► globus-os
      │                                                       ▲
      │                                                       │ kernel-boundary
      │                                                       │
      │                                               atc-shivacore
      │
      ├── atclang ── bytecode-abi ──► atc-vm ── deterministic-execution ──► a-townchain
      │                                  ▲                  ▲
      │                                  │                  │
      │                         atc-contracts          atc-node
      │
      ├── atc-storage ── persistent-state ──► atc-node
      ├── atc-indexer ── chain-events ───────► a-townchain
      ├── atc-oracle ── external-data ───────► a-townchain
      ├── atc-interop ─ cross-system-messaging ► a-townchain
      ├── atc-zkp ───── proof-verification ──► atc-vm
      ├── atc-wallet ── account-signing ─────► a-townchain
      ├── atc-sdk ───── client-api ──────────► atc-node
      ├── aurora-ai ─── ai-runtime ──────────► globus-os
      └── genesis-engine ─ game-chain-sdk ───► atc-sdk
```

## Integration rule

Every cross-repository dependency must have:

1. an explicit contract name;
2. a versioned interface or schema where applicable;
3. deterministic validation where the dependency affects consensus, boot, security or release state;
4. evidence emitted by the producing system;
5. fail-closed behavior when required evidence or contracts are unavailable.

## Readiness semantics

`Integrated` means the software boundary is represented and wired in the repository graph. It does **not** mean hardware-verified or production-ready. Hardware execution, cryptographic evidence, performance validation and release approval remain separate gates.

## Verification

The integration graph is validated by:

```text
cargo test -p atc-integration
```

The canonical graph must be acyclic and every edge must reference a declared system and non-empty contract.
