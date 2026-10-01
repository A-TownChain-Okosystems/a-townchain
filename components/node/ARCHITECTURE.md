---
document_id: ATC-DOC-ARC-NODE-001
title: Canonical Node Architecture
version: 1.1.0
status: active
owner: A-TownChain-Okosystems
standard: ATC-STD-MD-001
---

# Architecture Specification — a-townchain/components/node

## Canonical ownership

This component is the canonical Node Runtime / P2P implementation inside the `a-townchain` monorepo.

The former standalone `atc-node` repository is a migration source only. New implementation MUST target `components/node`.

## Layer ownership

- **L1:** OS/runtime and capability primitives remain outside this component.
- **L2:** canonical blockchain state, transaction validation, block validation, ordering, consensus and finality remain owned by the root blockchain core.
- **L4:** this component owns node networking, peer lifecycle, propagation, synchronization and RPC transport.
- **X:** identity, capability, policy and evidence controls apply across the component.

Network transport never creates L2 authority. A received transaction/block/state claim MUST cross the appropriate L2 validation boundary before becoming canonical.

## Subsystems

1. Node lifecycle/bootstrap.
2. Peer discovery and authenticated peer lifecycle.
3. Gossip and propagation.
4. Synchronization/state transfer.
5. RPC/API transport.
6. Validator-operation integration without owning consensus semantics.

## Authority boundary

`connected peer != trusted peer != authorized actor != canonical state authority`.

Privileged network actions require explicit identity/trust and capability checks and fail closed on ambiguity.

## Evidence

`CLAIMED != IMPLEMENTED != VERIFIED`.

This architecture document does not constitute CI evidence. Exact-SHA verification is required for implementation claims.
