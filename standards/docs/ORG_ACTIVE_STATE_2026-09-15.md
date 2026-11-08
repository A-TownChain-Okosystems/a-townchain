# Organization Active State — 2026-09-15

**Repository:** `atc-standards`  
**Role:** Canonical Standards Library / Registry SSOT  
**Scope:** Active organization state only; historical audit artifacts retain their original timestamps.

## Current repository inventory

The active repository registry contains **31 repositories** in `A-TownChain-Okosystems`.

Canonical inventory source:

- `registry/repositories.yaml`
- `count: 31`
- `generated: 2026-09-15`

The registry includes the currently active repositories `atc-engineering`, `atc-ide`, and `genesis-franchise-factory`, as well as the exempt governance/demo entries `.github` and `demo-repository`.

## Canonical architecture boundaries

| Capability | Canonical repository |
|---|---|
| Standards | `atc-standards` |
| Engineering governance / control plane | `atc-engineering` |
| Architecture documentation | `a-townchain-os-docs` |
| Kernel | `atc-shivacore` |
| Language | `atclang` |
| AI platform | `aurora-ai` |
| Blockchain orchestration | `a-townchain` |
| Integration | `a-townchain-os` |
| VM runtime | `atc-vm` |
| Node runtime | `atc-node` |
| Contracts | `atc-contracts` |
| Consensus | `atc-algorithm` |
| ZKP | `atc-zkp` |
| Wallet | `atc-wallet` |
| SDK | `atc-sdk` |
| Indexer | `atc-indexer` |
| Explorer | `atc-explorer` |
| Interoperability | `atc-interop` |
| Oracle | `atc-oracle` |
| Mining execution | `atc-mining` |
| Storage | `atc-storage` |
| Compute | `atc-compute` |
| Marketplace | `atc-marketplace` |
| Launchpad | `atc-launchpad` |
| Operating system | `globus-os` |
| Game engine | `genesis-engine` |
| Game | `genesis-chronicles` |
| Agent governance | `.github` |

## Readiness interpretation

Registry synchronization does **not** imply implementation completeness or production readiness.

The organization must continue to derive readiness from current implementation evidence, CI/build/test/security evidence, and the applicable governance gates. Historical statements such as earlier compliance counts remain audit history and must not be interpreted as the current state.

In particular:

- `atc-shivacore` retains its documented implementation blockers until resolved and evidenced.
- `globus-os` remains subject to its hardware, platform-integration, security, and production gates.
- `a-townchain-os` remains subject to its current GATE-KAI-001 production gates.
- Consensus-critical language migration and other P0/P1 work remain governed by their respective standards and issues.

## Synchronization rule

This document is the current-state companion to historical audit documents. Do **not** rewrite historical audit reports merely to make their dates or findings appear current. New scans create new dated evidence; the active registry and current-state documents point to the latest verified state.

## Evidence principle

**No Evidence, No Trust.** A repository being listed in the registry proves ownership/capability registration only; it does not prove that the implementation is complete, audited, secure, or production-ready.
