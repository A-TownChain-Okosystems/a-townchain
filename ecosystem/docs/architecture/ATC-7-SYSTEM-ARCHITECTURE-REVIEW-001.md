# ATC-7-SYSTEM-ARCHITECTURE-REVIEW-001

**Status:** `DRAFT / RESIDUAL`  
**Date:** 2026-10-26  
**Decision authority:** `atc-standards` for normative contracts; component SSOT repositories for implementation; `a-townchain-ecosystem` for architecture mapping and evidence aggregation only.

## 1. Purpose

Reconcile the seven named systems into one layered architecture without changing component ownership or implying implementation maturity. This is a proposed architecture delta for review, not yet a normative standard and not a replacement for repository-local implementation/API contracts.

## 2. Architecture decision

The system is **two connected authority paths**, not one linear stack.

### Platform / operating-system path

```text
Hardware / firmware / platform interfaces
  ↓
ShivaCore kernel + TCB (capabilities, isolation, IPC, memory/process enforcement)
  ↓
GlobusOS (userspace services, drivers, shell and application/runtime services)
  ↓
Aurora AI (agents, model/runtime routing, memory, orchestration) + applications
  ↓
Genesis Engine (game/simulation/runtime/tooling product domain)
```

Genesis Engine is an application/platform consumer of GlobusOS and may consume authorized Aurora services; it is **not required to sit strictly above Aurora** and must not acquire kernel or chain authority through that relationship.

### Blockchain / deterministic-execution path

```text
ATCLang source → parsing/validation → IR/lowering → verified bytecode/artifact
  → canonical ATC-VM (a-townchain/components/vm)
  → A-TownChain state transition / node / consensus / finality
  → canonical persisted chain state
```

The exact bytecode envelope, verifier contract, gas semantics, transaction encoding, state-transition order and consensus/finality semantics must match the normative `atc-standards` registry. A code path or catalog binding is not proof that cross-repository contracts conform.

### Cross-cutting authority and evidence

Identity, capability authorization, wallet signing policy, standards, governance, audit, telemetry and exact-SHA evidence intersect both paths. They are cross-cutting controls, not a third canonical state machine. Aurora may propose actions but cannot override ShivaCore enforcement, wallet authorization, VM validation, consensus or canonical state.

## 3. Canonical ownership

| System / capability | Canonical SSOT | Boundary |
|---|---|---|
| ATCLang | `A-TownChain-Okosystems/atclang` | Language/compiler/tooling; source and artifact validation |
| ATC-VM | `A-TownChain-Okosystems/a-townchain/components/vm` | Deterministic execution; no duplicate VM |
| A-TownChain L1 and algorithm | `A-TownChain-Okosystems/a-townchain`; algorithm at `components/algorithm` | Transactions, state, node, consensus and finality |
| ShivaCore kernel | `A-TownChain-Okosystems/globus-os/modules/atc-shivacore/kernel/` | no_std kernel/TCB, capability and isolation enforcement |
| GlobusOS | `A-TownChain-Okosystems/globus-os` | OS services, platform integration and userspace |
| Aurora AI | `A-TownChain-Okosystems/aurora-ai` | AI runtime/agents/memory; no kernel, consensus or canonical wallet authority |
| Genesis Engine | `A-TownChain-Okosystems/genesis-engine` | Native engine, simulation, rendering, editor, SDK and production tooling |
| Normative registry | `A-TownChain-Okosystems/atc-standards` | Normative protocol/API/ABI contracts and change control |
| Architecture/evidence control plane | `A-TownChain-Okosystems/a-townchain-ecosystem` | Catalog, architecture, integration, compliance and evidence only |

The separate `atc-vm` repository is not the executable VM SSOT; its catalog role is specification support. Repository existence does not authorize duplicate ownership.

## 4. Architecture gaps found in the baseline

| ID | Finding | Risk | Required correction | Initial status |
|---|---|---|---|---|
| ARCH7-01 | A single vertical stack can imply Genesis Engine depends on Aurora and chain execution is simply another OS layer. | Incorrect dependency/authority model. | Keep platform and blockchain paths distinct; define explicit interfaces. | RESIDUAL |
| ARCH7-02 | Current catalog is `BASELINE_RESIDUAL`; release decision is `BLOCKED`; functional verification is not claimed. | Catalog presence mistaken for readiness. | Preserve status separation and require exact-head test evidence. | BLOCKED |
| ARCH7-03 | Some catalog functions are high-level contracts or unbound entries, not source-backed public APIs. | Function inventory undercounts or misclassifies implementation. | Census modules, public exports, internal/test surfaces and non-Rust code separately. | RESIDUAL |
| ARCH7-04 | Imported snapshots and current source heads can diverge. | Stale SHA/path evidence. | Re-fetch each canonical repository HEAD/tree and recompute blob SHAs in one census run. | BLOCKED |
| ARCH7-05 | VM runtime, bytecode/artifact, gas and chain state-transition compatibility need coordinated cross-repo contracts. | Determinism or consensus divergence. | Normative contract, shared vectors and Rust/TypeScript/wallet compatibility tests. | BLOCKED |
| ARCH7-06 | Aurora agent/tool execution must remain subordinate to capability and wallet policy. | Privilege escalation / unauthorized signing. | Enforce policy in ShivaCore/runtime and canonical wallet/chain paths; test revocation and `wallet.export_key`. | BLOCKED |
| ARCH7-07 | Genesis Engine has a multi-module Rust workspace and additional Python factory surfaces. | Rust-only inventory misses executable capabilities. | Census Rust, Python, tests, feature gates and generated/exported surfaces. | RESIDUAL |
| ARCH7-08 | Workspace/package counts are not component/function counts. | False completeness claims. | Report source files, workspace members, public symbols, function contracts and test-only symbols separately. | RESIDUAL |

## 5. Mandatory cross-layer contracts

1. **ATCLang → ATC-VM:** versioned bytecode/artifact format, canonical encoding, verifier acceptance, ABI types, deterministic errors and conformance vectors.
2. **ATC-VM → A-TownChain:** exact execution context, transaction identity, gas budget, state transition and failure/rollback semantics.
3. **ShivaCore → GlobusOS:** capability-aware syscall/IPC contracts, memory/process isolation, lifecycle and fail-closed behavior.
4. **GlobusOS → Aurora AI:** authenticated IPC/service identity, scoped capabilities, resource/network/filesystem boundaries, revocation and audit.
5. **Aurora AI → wallet/chain actions:** proposal/simulation first; canonical wallet policy and authorized approval before signing; no AI tool bypass.
6. **GlobusOS/Aurora → Genesis Engine:** explicit runtime/API/IPC contract; local/offline functionality must not require remote inference or chain service unless explicitly specified.
7. **All systems → evidence plane:** exact commit and blob identity, test/gate provenance, exit codes, logs/artifact hashes and residual classification.

## 6. Verification protocol

For each repository, record exact HEAD SHA, tree SHA, manifest/workspace package count, source-language census, Rust module declarations (external and inline), `cfg(test)`/feature/target gates, public exports, dependencies, canonical ownership, test surfaces, file blob SHA and catalog contract mapping.

- `IMPLEMENTED`: source-backed implementation exists.
- `TESTED`: relevant tests passed on a pinned commit.
- `CI_VERIFIED`: required CI checks passed for the exact tested commit.
- `E2E_VERIFIED`: cross-component path passed with explicit endpoints and evidence.
- `VERIFIED`: applicable implementation, security, governance and evidence criteria are met.
- `RESIDUAL`: partial, stale, ambiguous or incomplete evidence.
- `BLOCKED`: missing authoritative contract, required evidence, failing gate or unresolved safety-critical ownership.

No automatic merge/release follows from this review. `VERIFIED` must not be inferred from green CI, catalog entries, file existence or prose documentation.

## 7. Required acceptance gates

- [ ] Normative layer/authority review approved by relevant owners.
- [ ] Seven-system source census completed against exact current heads.
- [ ] Every catalog contract mapped or explicitly marked unbound/excluded with reason.
- [ ] Public functions and relevant internal/test surfaces enumerated without conflating them.
- [ ] External modules, inline modules, `cfg(test)`, feature and target gates separately accounted.
- [ ] Duplicate SSOTs and stale imported snapshots reconciled.
- [ ] Bytecode/ABI/gas/state-transition vectors pass across language, VM, SDK and wallet.
- [ ] Negative security tests pass for capability bypass, revocation, key export, unauthorized signing, malformed artifacts, resource exhaustion and IPC boundaries.
- [ ] OS boot/runtime and Genesis Engine smoke/integration tests pass where supported.
- [ ] Evidence binds exact SHA → workflow run → job → step → exit code/logs/artifact SHA-256.
- [ ] Security/governance review accepts all residuals or keeps release `BLOCKED`.

## 8. Decision

**Architecture direction:** accepted as a review proposal.  
**Implementation completeness:** `RESIDUAL`.  
**Release / merge readiness:** `BLOCKED` pending review, live census and exact-SHA verification.

This document records architecture decisions and gaps; it does not claim that unverified integration is already implemented.