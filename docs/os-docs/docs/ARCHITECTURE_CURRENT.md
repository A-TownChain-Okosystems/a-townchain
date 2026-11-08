# A-TownChain Architecture Reference — Current Boundaries

**Snapshot:** 2026-10-09  
**Type:** system-level architecture index; not a release certification or substitute for canonical specifications.

## Source-of-truth map

| Domain | Canonical source | Responsibility |
|---|---|---|
| Normative standards | [atc-standards](https://github.com/A-TownChain-Okosystems/atc-standards) | Normative requirements, registry and generated compliance views |
| ATCLang | [atclang](https://github.com/A-TownChain-Okosystems/atclang) | Language/compiler and artifact validation; current README describes a Rust-only active path |
| Blockchain core | [a-townchain](https://github.com/A-TownChain-Okosystems/a-townchain) | Chain state, protocol integration and canonical component tree |
| ATC-VM | `a-townchain/components/vm` | Canonical VM implementation |
| Algorithm / consensus | `a-townchain/components/algorithm` | Canonical algorithm and consensus implementation; consensus choices must follow approved specs |
| Node runtime | `a-townchain/components/node` | Canonical node implementation after SCR-0127 migration. `atc-node` is a migrated source repository and must not be described as an equal, competing runtime; keep it frozen/archive-pending until its owner completes the repository lifecycle action. |
| ShivaCore kernel / TCB | [globus-os/modules/atc-shivacore/kernel/](https://github.com/A-TownChain-Okosystems/globus-os/tree/main/modules/atc-shivacore/kernel) | Capability enforcement, memory/address-space primitives, scheduling, IPC and interrupt/timer boundaries |
| GlobusOS | [globus-os](https://github.com/A-TownChain-Okosystems/globus-os) | Userspace, system services and hardware integration above the TCB |
| Aurora AI | [aurora-ai](https://github.com/A-TownChain-Okosystems/aurora-ai) | AI services and agents; no kernel, consensus or chain authority |
| Toolchain | [atc-toolchain](https://github.com/A-TownChain-Okosystems/atc-toolchain) | Build, CLI, generators and governance tooling |
| Integration control plane | [a-townchain-ecosystem](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem) | Integration, compliance and evidence aggregation, not duplicate implementations |
| System orchestrator | [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os) | Cross-repository integration and validation |
| Documentation hub | This repository | Architecture references, wiki, historical material and system-level status snapshot |

## Component ownership and alternative-selection rules

| Component | Canonical implementation / authority | Separate repository role | Boundary |
|---|---|---|---|
| ATC-VM | `a-townchain/components/vm` | `atc-vm` may own VM specifications, governance and support only where explicitly defined; it must not become a second production runtime. | Bytecode, ABI, verifier and conformance vectors must be versioned and tested together. |
| Algorithm / consensus | `a-townchain/components/algorithm` | `atc-algorithm` is not a competing production implementation; any retained role must be explicitly specification/governance-only. | No consensus choice is final without an approved normative specification and exact-SHA evidence. |
| Node runtime | `a-townchain/components/node` | `atc-node` is a migrated source repository; lifecycle status must agree with the standards registry. | Network, mempool, block production/validation and consensus interfaces must have explicit contracts. |
| Contracts | `a-townchain/components/contracts` | `atc-contracts` is a migrated source repository; open residual work must not be confused with canonical ownership. | Contract semantics and token economics must conform to normative standards. |
| ShivaCore kernel | `globus-os/modules/atc-shivacore/kernel/` | `atc-shivacore` owns supporting specifications/governance only. | Capability enforcement and isolation remain inside the TCB; AI and services cannot bypass it. |
| Aurora AI | `aurora-ai` | AI services, models and agent tooling. | Proposal/planning only; no direct kernel, consensus, VM or canonical chain authority. |
| Ecosystem control plane | `a-townchain-ecosystem` | Integration, compliance, architecture control and evidence aggregation. | No duplicate VM, algorithm, wallet, node or other canonical component implementation. |
| System orchestrator | `a-townchain-os` | Cross-repository integration and validation. | Orchestrates validation; does not become the source of truth for each component's implementation. |

The normative registry in `atc-standards` remains the single source for repository roles and approved layer taxonomy. Until the applicable layer standard is approved, a taxonomy marked DRAFT must not be presented as a normative, finalized architecture decision. Documentation updates must reconcile with the registry rather than silently redefine it.

## Alternative technology evaluation

External libraries or frameworks are candidates for evaluation, not automatic replacements. For each proposed alternative, record: requirement and scope; security and maturity evidence; license and dependency impact; target-platform and `no_std` compatibility where applicable; deterministic/reproducible behavior; performance measurements; migration/rollback plan; and an explicit owner decision. Do not change canonical transaction encoding, signed bytes, consensus serialization, ATC bytecode or kernel security boundaries without approved compatibility requirements and conformance tests.

## Runtime/security boundary

```text
User intent / application request
        ↓
Aurora AI (proposal, inference, planning)
        ↓
Policy decision
        ↓
Capability check + explicit approval when required
        ↓
Tool / service boundary
        ↓
Authoritative system service or canonical runtime
        ↓
Auditable result / evidence
```

AI may propose or orchestrate work, but must not bypass capability enforcement, policy, human approval requirements, deterministic chain rules or canonical VM behavior.

## Status and evidence model

- **SPECIFIED:** requirements/design are documented.
- **IMPLEMENTED:** code/artifact exists; no correctness inference.
- **VERIFIED:** required checks pass for the exact claimed SHA, with Run → Job → Step → exit status/log evidence.
- **BLOCKED / RESIDUAL:** unresolved finding; remains open until corrected and rerun.
- **AUDITED / CONFORMANT / PRODUCTION_READY / MAINNET_READY:** separate claims, each requiring its own applicable evidence.

A green unrelated workflow, historical audit score, PR merge reference, repository badge or README assertion does not upgrade the current source SHA. Never weaken CI, branch protection, security or release gates to manufacture a passing status.

## Important non-claims

- This model does not claim all component interfaces are connected end-to-end.
- It does not decide an unfinalized consensus algorithm.
- It does not certify kernel security, hardware boot, AI safety, standards conformance or production readiness.
- Component-specific blockers and current test evidence must be read from the canonical repository and exact-SHA workflow.

## Supporting references

- [Current ecosystem status snapshot](ECOSYSTEM_CURRENT_STATUS.md)
- [ShivaCore architecture boundary](architecture/SHIVACORE_KERNEL_ARCHITECTURE.md)
- [AI layer boundary](architecture/AI_LAYER.md)
- [Consensus source-of-truth note](architecture/CONSENSUS.md)
- [ATCLang compiler boundary](architecture/ATCLANG_COMPILER.md)
- [Cluster/deployment verification criteria](CLUSTER_ARCHITECTURE.md)
- [Normative standards SSOT](https://github.com/A-TownChain-Okosystems/atc-standards)
