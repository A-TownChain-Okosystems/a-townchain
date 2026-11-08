# Aurora AI 1.0 — Master Architecture and Engineering Specification

**Status:** Architecture baseline / implementation contract  
**Version:** 1.0.0  
**Target:** GlobusOS first; portable to other supported hosts

## 1. System identity

Aurora AI is the AI Control Plane, Agent Platform and Intelligence Fabric for the A-TownChain / GlobusOS ecosystem. It interprets intent, plans work, routes agents and models, enforces policy, coordinates execution, verifies results and produces auditable outcomes.

Aurora is not an operating system, kernel, blockchain consensus implementation, wallet-key authority or general-purpose language runtime.

| System | Authoritative responsibility |
|---|---|
| ATCLang | Language syntax, compiler, ABI and program/agent/workflow definitions |
| GlobusOS | Operating system, services, device/hardware resource management and system execution boundaries |
| Aurora AI | Intent, planning, agents, model routing, memory/knowledge orchestration, workflow and governance coordination |
| A-TownChain | Decentralized state, consensus, identity/trust/ownership and settlement where applicable |
| ShivaCore | Capability-based kernel and trusted computing base (TCB) |

There is no additional super-component above these authorities. Integration uses versioned interfaces and explicit gateways.

## 2. Logical architecture

This is a responsibility model, not a requirement that every layer be a separate process or crate in v1.

1. **Experience:** Web, mobile, desktop, CLI, Globus Shell, Genesis and administration clients.
2. **Control Plane:** intent, context, planning, orchestration, agent/model routing, workflows and request coordination.
3. **Fabric:** event delivery, discovery, agent communication, state/node synchronization and distributed/offline coordination.
4. **Agent and Knowledge domains:** agent registry and lifecycle alongside short-term, working, long-term, semantic and episodic memory; RAG, vector-store and knowledge-graph abstractions.
5. **Runtime and Intelligence:** agent/skill/tool/workflow execution, WASM, sandbox, scheduler, resource management and model inference across local/cloud/edge.
6. **Integration:** GlobusOS gateway, ATC gateway, ATCLang toolchain and explicitly configured external APIs.

**Governance is cross-cutting:** identity, capabilities, permissions, policy, risk, approval, verification and audit apply to every layer. Tool Fabric is a controlled execution boundary, not a bypass around governance.

## 3. Dependency and authority rules

Preferred logical flow:

Experience → Control Plane → Fabric → Agent → Skill → Tool → Runtime / Gateway

- Dependencies MUST point toward stable contracts; implementations MUST NOT create reverse dependencies.
- An Agent decides **what** should be done; a Skill describes **how**; a Tool requests a bounded operation; a Runtime executes it.
- Agents MUST NOT call concrete LLM backends directly or access OS, network, filesystem, Git, wallet or blockchain directly.
- Aurora Core MUST remain contracts/orchestration-only. It MUST NOT perform infrastructure I/O, spawn processes, access databases, call model backends or sign/broadcast transactions.
- GlobusOS and A-TownChain MUST NOT depend on Aurora Agent implementations. They expose controlled services and gateways instead.
- Model providers, databases, event transports, tool implementations and node transports MUST sit behind interfaces.
- Aurora Fabric handles AI communication. A-TownChain is used when decentralized verification, ownership, reputation, governance or settlement is required.
- Private memory and secrets are local-only by default. Wallet secrets MUST NEVER be synchronized through Aurora Fabric.

## 4. Core contracts and agent registry

Public interfaces SHOULD be idiomatic Rust traits: Agent, Skill, Tool, Model, Memory, Knowledge, Workflow, Policy, Approval, Audit, Node, GlobusGateway and AtcGateway. Public contracts MUST be versioned independently: Specification, Architecture, Agent API, Skill API, Tool API, Model API, Workflow API, Policy API, Node API, ATC Gateway API and Globus Gateway API. Breaking changes require a new major API version or an explicit migration contract.

Every registered agent MUST have machine-readable metadata for:
- agent ID, name, version and description;
- capabilities, skills, tools and required models;
- input/output schema references;
- requested permissions and resource limits;
- sandbox profile, trust level and risk level;
- approval and audit policies;
- health status and dependency declarations.

Registry metadata is descriptive, not authority: declared permissions never grant capabilities by themselves. Runtime authorization is evaluated against the active principal, policy, environment and request.

Every tool MUST declare its ID/name/version, description, input/output schemas, capabilities, required permissions, risk level, sandbox profile, resource limits, timeout, network/filesystem access and audit policy. Unspecified permissions and resource boundaries default to deny.

## 5. Agent lifecycle

Agent lifecycle:

DISCOVER → LOAD → VALIDATE → AUTHORIZE → INITIALIZE → READY → EXECUTE → VERIFY → AUDIT → SUSPEND / TERMINATE

Skill lifecycle:

DISCOVER → VALIDATE → AUTHORIZE → LOAD → EXECUTE → VERIFY → UNLOAD

Failed validation or authorization MUST prevent execution. Any state transition not explicitly permitted by the state machine MUST be rejected.

## 6. Policy, risk and approvals

A policy evaluation returns exactly one of Allow, Deny or ApprovalRequired.

- **Deny is terminal.** A denied action MUST NOT be converted into an approval request or retried through another tool.
- ApprovalRequired suspends execution until a valid, unexpired approval is granted for the same request/action scope.
- Approval does not override a separate denial.

| Risk | Example | Minimum control |
|---|---|---|
| L0 | Text generation | Automatic, bounded |
| L1 | Read approved data | Capability and policy check |
| L2 | Modify code/data | Policy plus scoped write permission |
| L3 | Git commit | Policy; approval according to repository policy |
| L4 | Deployment | Explicit approval |
| L5 | Blockchain transaction | Explicit transaction-specific approval |
| L6 | Wallet or system-critical action | Multi-approval and strongest available verification |

### Trust zones

- **Zone 0 — Untrusted:** external input, internet content and unknown tools.
- **Zone 1 — Sandbox:** generated code, experimental agents and unverified plugins.
- **Zone 2 — Trusted:** verified agents, approved tools and controlled system APIs.
- **Zone 3 — Critical:** wallet signing, credentials, kernel operations, blockchain execution and security configuration.

Cross-zone operations require explicit capability checks and policy decisions. No agent receives root, kernel, wallet or signing authority automatically.

Wallet baseline: wallet.read is allowed only through an authorized read interface; wallet.prepare may prepare a proposal; wallet.sign and wallet.broadcast require explicit approval and verification; wallet.export_key is denied.

## 7. Required execution sequence

Sensitive or externally observable actions MUST follow:

Intent → Plan/Proposal → Capability Check → Policy Evaluation → Risk Classification → Approval (if required) → Sandbox → Execution → Verification → Audit

Execution MUST NOT begin before authorization is complete. Verification failure MUST prevent success reporting. Audit failure for a security-critical action MUST fail closed unless a documented recovery policy permits a safe, non-mutating fallback.

## 8. Aurora Fabric and offline-first behavior

Fabric provides event bus, agent-to-agent communication, model-to-agent routing, tool/service discovery, distributed execution, state synchronization, node-to-node communication and offline/online synchronization.

V1 SHOULD start with in-process interfaces and deterministic in-memory implementations. Networked/distributed transports are later implementations of the same contracts, not prerequisites for Core.

Synchronization policy:
- private memory and credentials: local-only by default;
- project memory: opt-in sync;
- model metadata and approved registries: sync according to policy;
- audit evidence: append-only; replication is policy-controlled;
- wallet secrets and private keys: never sync.

Conflict resolution MUST be explicit and domain-aware. Last-write-wins MUST NOT be used for authorization, identity, approval or security state.

## 9. Intelligence and Model Router

Agents use the stable Model abstraction and MUST NOT know concrete backend/provider details. The Model Router MAY select local, cloud or edge models using availability, latency, hardware capability, task complexity, privacy, offline mode, context size, cost and energy constraints.

Local-first is the default preference when a suitable local model meets policy and task requirements. Cloud routing requires applicable privacy and network policy. Provider unavailability MUST be surfaced; the router MUST NOT silently downgrade security or disclose private context to make a task succeed.

The intelligence domain can later host LLM, SLM, vision/OCR, speech/STT/TTS, image, video, music and code models. These are capability classes, not separate mandatory crates in the initial release.

## 10. ATCLang, GlobusOS and A-TownChain integration

### ATCLang
ATCLang MAY describe agents, skills, tools, workflows and policies declaratively. Its compiler/toolchain validates definitions and emits a supported target such as ATCB/WASM/native artifacts. Aurora MUST validate manifests and runtime policy independently; a compiled artifact is not inherently trusted.

### GlobusOS
The Globus Gateway exposes explicitly authorized system services. Aurora does not bypass ShivaCore capabilities, service boundaries, sandboxing or GlobusOS identity. Kernel and TCB APIs remain outside agent control.

### A-TownChain
The ATC Gateway provides bounded interfaces for RPC, node discovery, read/index queries, contract inspection/simulation, governance and transaction proposals. A transaction path is:

Intent → Proposal → Simulation → Risk Analysis → Policy → User Approval → Signing Boundary → Broadcast → Confirmation

Aurora MUST NOT hold or export private keys, and MUST NOT treat a model response as a signature or authorization.

## 11. Repository domains and staged delivery

Domains are logical ownership boundaries. Create a separate crate only when its contract and lifecycle justify it.

- **core/**: types, errors, contracts, state machine and deterministic coordination primitives.
- **fabric/**: event bus, agent mesh, discovery, state/node/offline sync.
- **agents/**, **skills/**, **workflows/**: versioned manifests and reusable definitions.
- **runtime/**: agent, skill, tool and workflow runtimes; WASM, sandbox, scheduler and resource manager.
- **intelligence/**: model API/router and local/cloud/edge adapters.
- **memory/**, **knowledge/**: storage-independent interfaces first; backends later.
- **tools/**: capability-scoped tool adapters.
- **governance/**: identity integration, permissions, policies, risk, approvals and audit.
- **nodes/**: node identity, registries, runtime, resource management and fabric client.
- **gateway/atc**, **gateway/globus**, **gateway/external**: explicit integration boundaries.
- **marketplace/**, **studios/**, **api/**, **cli/**, **desktop/**, **mobile/**: later product surfaces.
- **docs/**, **tests/**: specifications, conformance and regression coverage.

### P0 — Core safety baseline
Typed IDs; request/response; contracts; state machine; deterministic policy/approval interfaces; in-memory event/audit implementations; tests for allow, approval-required and terminal deny. Keep Core dependency-light and free of I/O.

### P1 — Single-node runtime
Agent/skill/tool execution, sandbox, scheduler, verification, model abstraction/router, registries, memory/knowledge interfaces and GlobusOS gateway.

### P2 — Distributed/offline fabric
Node identity, service discovery, event transport, sync/conflict policy, local/edge/cloud model routing and recovery.

### P3 — Ecosystem services
ATC gateway, optional trust/settlement/anchoring, marketplace, studios, desktop/mobile surfaces, cluster orchestration and observability.

## 12. Definition of Done and evidence

A milestone is not implemented merely because a document or module exists. Statuses MUST distinguish **SPECIFIED**, **IMPLEMENTED**, **VERIFIED**, **BLOCKED** and **RESIDUAL**.

P0 acceptance requires:
1. cargo fmt --check, cargo check --workspace and cargo test --workspace pass on the exact commit SHA;
2. regression test: allowed read proceeds through verification and audit;
3. regression test: write requiring approval performs no execution before grant;
4. regression test: wallet.export_key is denied and cannot be approval-bypassed;
5. illegal state transitions are rejected;
6. Core performs no filesystem/network/process/database/model-provider I/O;
7. test and CI evidence is recorded against the exact SHA, not a PR merge ref alone.

This document freezes the architecture boundary. New functionality should be added through versioned contracts and staged milestones rather than by expanding a monolithic Core.
