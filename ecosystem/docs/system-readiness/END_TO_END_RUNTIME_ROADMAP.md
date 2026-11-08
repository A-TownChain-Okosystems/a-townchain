# End-to-End Runtime Readiness Roadmap

**Status:** PLANNED / NOT YET END-TO-END VERIFIED  
**Owner:** Ecosystem integration and evidence control plane  
**Tracking issue:** [#116](https://github.com/A-TownChain-Okosystems/a-townchain-ecosystem/issues/116)

## 1. Purpose

Define the implementation sequence and acceptance evidence required to operate ShivaCore, GlobusOS, A-TownChain and Aurora AI as one integrated platform. This document is a control-plane plan; it does not claim that runtime functionality has already passed.

## 2. Single-source-of-truth boundaries

| Capability | Canonical source | Responsibility |
|---|---|---|
| ATCLang | `atclang` | Language frontend and language tooling |
| ATC-VM | `a-townchain/components/vm` | Canonical VM and bytecode execution |
| Consensus / algorithm | `a-townchain/components/algorithm` | Canonical consensus and algorithm implementation |
| A-TownChain L1 | `a-townchain` | Transactions, node, state, networking and chain integration |
| ShivaCore kernel | `globus-os/modules/atc-shivacore/kernel/` | Minimal Rust/no_std capability microkernel |
| GlobusOS | `globus-os` | User-space services, lifecycle, storage and OS integration |
| Aurora AI | `aurora-ai` | Model runtime, policies, agents and tool orchestration |
| Integration evidence | `a-townchain-ecosystem` | Cross-repository contracts, source-SHA reconciliation, gates and evidence |

Imported subtree snapshots are not canonical implementations. Do not add replacement VM, consensus or kernel implementations to this repository.

## 3. Phases and acceptance criteria

### Phase 0 — Inventory and architecture reconciliation
- Record canonical default branch and exact HEAD SHA for each source repository.
- Compare imported subtree provenance with canonical source SHAs.
- Map each runtime component to its owner, ABI/API, dependency and security boundary.
- Record every unresolved conflict as BLOCKED or RESIDUAL; do not infer readiness from file presence.

**Gate G0:** ownership map and SHA reconciliation are complete, and all gaps have owners.

### Phase 1 — ShivaCore kernel boot and memory safety
- Build the canonical kernel using the repository's prescribed Rust toolchain and feature set.
- Run unit, integration, formatting, lint and static checks without weakening gates.
- Verify UEFI/QEMU boot and record serial/log evidence.
- Verify scheduler, address-space isolation, page-table mutation/rollback, IPC, capability enforcement and object/mapping lifetime.
- Maintain a minimal TCB; no AI inference, chain consensus, networking stack or smart-contract execution in kernel core.

**Gate G1:** exact-SHA build/tests and boot evidence pass; remaining blockers and residuals are explicit.

### Phase 2 — GlobusOS service space
- Verify init/boot orchestration and service manager.
- Define versioned IPC/API contracts and explicit service capabilities.
- Verify storage/filesystem, network, identity, logging, health checks, dependency order and bounded restart/recovery.
- Verify graceful shutdown and recovery after service/process failure.

**Gate G2:** reproducible boot reaches a usable user space; required services start, communicate, stop and recover under tests.

### Phase 3 — Integrated operating-system acceptance
- Run a boot-to-user-space test in QEMU with a defined success signal and timeout.
- Verify persistent data survives reboot.
- Inject controlled service failures and verify isolation/recovery.
- Record the hardware/firmware/CPU configuration and test artifact provenance.

**Gate G2A:** end-to-end OS acceptance passes on a pinned source set.

### Phase 4 — A-TownChain runtime
- Build and test the canonical chain/node, algorithm and ATC-VM from their SSOT locations.
- Verify chain ID/configuration binding, transaction signature and encoding rules, block validation and deterministic state transitions.
- Verify durable persistence, restart recovery, peer synchronization and multi-node consensus/finality.
- Test malformed inputs, resource limits, invalid signatures, replay boundaries and divergent-state detection.

**Gate G3:** local and multi-node test evidence passes for the exact source SHAs. A single-node test is not evidence of distributed consensus.

### Phase 5 — Aurora AI runtime
- Load a versioned model through the designated runtime and record model/runtime identifiers.
- Verify inference and resource limits on the declared target (CPU first; GPU/NPU only where supported).
- Enforce the action path: Intent/Proposal → Policy → Capability Check → Approval where required → Tool → Service Boundary → Audit.
- Deny by default. AI output is a proposal, not authorization; no direct kernel capability or privileged consensus authority.
- Test denied actions, invalid tool arguments, timeout, model failure and audit completeness.

**Gate G4:** inference and approved tools pass tests, and unauthorized actions are rejected.

### Phase 6 — Toolchain, ABI and release reproducibility
- Pin compiler/toolchain and dependency versions.
- Validate versioned ABI/API contracts and compatibility tests.
- Build release artifacts from a clean, documented source state.
- Run applicable security, dependency, license, performance and regression gates.
- Verify update, rollback and recovery behavior.

### Phase 7 — Full-system release candidate
- Pin all participating repository SHAs and relevant configuration/model digests.
- Run the complete boot → services → chain → AI-tool integration test.
- Produce one evidence index linking every required gate to workflow run, job, step, exit status and logs.
- Publish explicit residuals; block release on unresolved mandatory gates.

**Gate G5:** full-system evidence passes on the exact pinned source set. Production readiness remains unclaimed until all applicable release gates pass.

## 4. Evidence contract

Every gate record must contain:
- gate ID and component;
- status: `IMPLEMENTED`, `VERIFIED`, `BLOCKED`, or `RESIDUAL`;
- exact source commit SHA(s);
- workflow/run URL and run ID;
- job and step identifiers;
- exit status and relevant log URL or immutable artifact;
- test command and environment/configuration;
- timestamp and responsible workflow/owner;
- linked residual/blocker IDs.

A PR merge, source import, passing isolated component test or manually asserted status is not a substitute for this evidence.

## 5. Execution policy

1. EXISTING-FIRST: inspect canonical code and tests before proposing replacement implementations.
2. Fix the earliest blocking gate in its canonical repository.
3. Do not weaken or skip CI gates to obtain a green result.
4. Keep documentation and integration control here; implement kernel, OS, AI and blockchain behavior in their SSOT repositories.
5. Do not auto-merge PRs. Require exact-head validation and normal governance/review.
6. Reconcile subtree snapshots only after recording the source and destination SHAs.
