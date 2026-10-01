---
document_id: ATC-DOC-ARC-ATCH-001
title: Repository Architecture Specification
version: 1.1.0
status: active
owner: A-TownChain-Okosystems
standard: ATC-STD-MD-001
---

# Architecture Specification — a-townchain

## Canonical role

`a-townchain` is the canonical L1 blockchain monorepo. It owns the authoritative implementation of the blockchain core and migrated components under `components/*`.

The repository is **Layer L2 Blockchain Core + canonical component implementations**, not a generic L3 chain layer.

## Canonical layer ownership

| Layer | Responsibility | Canonical location |
|---|---|---|
| L0 | Hardware / secure platform | platform-specific repositories |
| L1 | Node/runtime foundation, ShivaCore integration, IPC/capabilities | `globus-os`, ShivaCore |
| L2 | Block, transaction, state, ledger, mempool, ordering, consensus, finality, validation | `a-townchain` |
| L3 | ATC-VM execution, typed values, gas/resource accounting | `components/vm` |
| L4 | P2P, synchronization, propagation, RPC/API | `components/node` |
| L5 | Economic/security services and cryptographic policy | L2 services + `atc-standards` |
| L6 | ATCLang, contracts, ABI, application protocols | `components/contracts`, `atclang`, standards |
| L7 | Wallet, SDK, explorer and developer applications | `components/wallet`, `components/sdk`, `components/explorer` |
| X | Identity, capability, policy, governance, crypto, audit, evidence, observability, interop, versioning | cross-layer control plane |

## Repository ownership rule

The monorepo migration is authoritative for implementation ownership:

- `atc-node` → `components/node`
- `atc-algorithm` → `components/algorithm`
- `atc-vm` → `components/vm`
- `atc-contracts` → `components/contracts`
- `atc-sdk` → `components/sdk`
- `atc-wallet` → `components/wallet`
- `atc-explorer` → `components/explorer`

The former standalone repositories are historical/migration sources unless explicitly designated otherwise. New canonical implementation MUST NOT be added there.

## Transaction authority boundary

The transaction contract is a protocol/L2 contract, not a Wallet-owned protocol definition.

- `atc-standards` defines the normative transaction and cryptographic contract.
- L2/node validates and consumes canonical transactions.
- `components/wallet` implements client-side construction and authorization against that contract.
- `components/sdk` exposes client protocol primitives and MUST use the same canonical bytes.
- Wallet/SDK MUST NOT introduce an alternative transaction encoding or signature domain.

Canonical transaction authorization is ECDSA secp256k1 with RFC6979 and low-S. Ed25519 remains reserved for identity/P2P contexts. The mandatory transaction domain is `ATC-TX-DOMAIN-V2`.

## Determinism and authority

The authoritative state transition is:

`F(State, Block) = State'`

The deterministic path is:

Transaction → canonical encoding → authorization validation → transaction validation → mempool → block validation → consensus/finality → VM execution → state transition → state commitment → ledger commit.

Network reachability, wallet intent, SDK output, AI/model output, indexer data, or UI state never grants authority to mutate canonical L2 state. Every authoritative action MUST cross its defined capability/policy validation boundary and produce auditable evidence.

## Evidence rule

Architecture text is descriptive. It does not constitute verification evidence.

`CLAIMED != IMPLEMENTED != VERIFIED`

Exact-SHA evidence MUST bind source SHA → workflow/run → job → step/log → result. Missing or ambiguous evidence is BLOCKED, never PASS.
