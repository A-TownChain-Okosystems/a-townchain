---
document_id: ATC-AI-AURORA-MATRIX-001
title: Aurora AI — Canonical Vision → Components → Functions → Evidence Matrix
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
updated: 2026-10-12
---

# Aurora AI — Canonical Function Matrix

This document is the implementation contract for the Aurora AI platform. It does not convert architectural presence into implementation or verification evidence.

## Canonical architecture

```
AI Governance
    ↓
Identity / Access
    ↓
ModelHub
    ↓
AI Runtime
    ↓
Agent System
    ↓
Capability / Policy / Approval
    ↓
Tool & Action Layer
    ↓
Memory + RAG + Data
    ↓
Evaluation + Safety + Context + Multimodal
    ↓
GlobusOS / A-TownChain interfaces
    ↓
ShivaCore enforcement boundary
    ↓
Hardware
```

## Authority invariant

```
Model output
  → Agent proposal
  → Capability check
  → Policy evaluation
  → Approval gate where required
  → Tool invocation
  → Domain validation
  → Authoritative state transition
```

Model output MUST NOT directly grant capability, authorization, OS privilege, signing authority, blockchain authority, governance authority or canonical state mutation.

## Domain matrix

| ID | Domain | Canonical functions | Primary implementation boundary |
|---|---|---|---|
| AI-01 | Vision & Governance | policy, risk classification, oversight, lifecycle, audit | docs / atc-standards |
| AI-02 | Identity & Access | user/service/model/agent identity, AuthN/AuthZ, sessions, secrets | GlobusOS + Aurora |
| AI-03 | Model Layer | model metadata, versions, local/remote providers, lifecycle | ModelHub |
| AI-04 | ModelHub | registry, discovery, routing, selection, health, compatibility | atc-aurora-core |
| AI-05 | AI Runtime | inference, context, scheduling, CPU/GPU/NPU, quotas, isolation | atc-aurora-runtime |
| AI-06 | Agent System | lifecycle, planning, decomposition, tool selection, state, termination | atc-aurora-agents |
| AI-07 | Capability / Policy | capabilities, scopes, policy evaluation, approval, revocation | capability_policy + approval + GlobusOS |
| AI-08 | Tool & Action | registry, schemas, validation, OS/network/filesystem/chain adapters | runtime / integration |
| AI-09 | Memory | working, conversational, semantic, episodic, long-term, retention | atc-aurora-memory |
| AI-10 | RAG / Knowledge | ingestion, parsing, chunking, embeddings, retrieval, ranking, attribution | atc-aurora-memory |
| AI-11 | Data | contracts, lineage, classification, quality, versioning | data/integration boundary |
| AI-12 | AI Safety & Security | injection defense, exfiltration prevention, sandboxing, fail-closed, emergency stop | policy/security boundary |
| AI-13 | Evaluation | accuracy, groundedness, robustness, safety, regression, E2E | evaluation/CI |
| AI-14 | Prompt / Context | system/developer/user context, templates, versioning, injection detection | runtime |
| AI-15 | Multimodal | text/image/audio/video, STT/TTS, vision, documents | provider/runtime boundary |
| AI-16 | OS Integration | IPC, processes, filesystem, network, devices, GPU/NPU, quotas | GlobusOS |
| AI-17 | Blockchain Integration | wallet, RPC, tx construction, signing request, state queries, contract calls | A-TownChain interfaces |
| AI-18 | Developer Platform | SDKs, APIs, CLI, tool/model/agent extension contracts | Aurora developer surface |
| AI-19 | Observability | traces, tool logs, policy decisions, resource/error telemetry, audit | runtime / governance |
| AI-20 | Operations | deployment, health, scaling, failover, rollback, updates | operations |
| AI-21 | Supply Chain | provenance, signing, SBOM, version pinning, reproducibility | release/security |
| AI-22 | Testing & Verification | unit, integration, security, policy, RAG, regression, E2E, exact-SHA | CI / evidence |
| AI-23 | Evidence & Governance | evidence provenance, status separation, verification records, residuals | atc-standards + ecosystem |

## Evidence contract

Every implemented function is classified independently:

```
Architecture
  → Source Discovery
  → Implementation
  → Contract
  → Test
  → CI
  → Exact-SHA Evidence
  → Verification
  → Residual
```

The following are never interchangeable:

```
Error Evidence ≠ Finding Evidence ≠ Verification Evidence
Source present ≠ Implemented
Implemented ≠ Tested
Tested ≠ CI Verified
CI Verified ≠ E2E Verified
```

## Implemented in this change

- `CapabilityPolicyEngine`: fail-closed exact-match capability, principal, action and scope authorization.
- `PolicyDecision` distinguishes `Allow`, `Deny` and `ApprovalRequired`; missing approval does not silently become permission.
- `ApprovalGrant` is bound to one request and its exact principal/capability/action/scope tuple and records the approver identity.
- `ApprovalEngine`: in-memory pending approval tracking, one-shot approval consumption and rejection. The engine consumes a sealed `VerifiedApproval` token. Its constructor is crate-private and intentionally unused in production until an authenticated host adapter is integrated.
- Duplicate matching policies, unknown capabilities, mismatched scopes and replayed grants fail closed.
- `StateMachine`: deterministic execution transitions; verification is required before completion, approval waits cannot be skipped, and terminal states cannot be reopened.
- `EventBus`: deterministic in-memory event ordering, request filtering and duplicate event-ID rejection.
- `AuditLogger`: append-only in-memory records, request filtering and duplicate event-ID rejection.
- Existing core modules are rustfmt-formatted and provide `Default` implementations.
- Unit tests cover policy, approval, event, audit and state-machine invariants.

## Explicit residuals

- The production host must authenticate the human approver before the crate constructs a `VerifiedApproval` token; the token constructor is crate-private and intentionally remains unused in production until that host adapter exists. The in-memory engine is not itself an identity provider or cryptographic attestation mechanism.
- Event and audit stores are in-memory only. Durable, tamper-evident audit export requires an authorized runtime adapter and a separately verified persistence boundary.
- The state machine and authorization primitives are not yet wired through every agent/tool execution path; runtime integration remains a separate gate.
- Provider-specific inference, production-grade RAG, multimodal backends, full agent planning, blockchain signing execution, deployment automation and complete end-to-end verification remain separate implementation/evidence gates.

No item in this document is a production-readiness claim.


## ChatGPT-comparable product scope mapping

The detailed component-level requirements are maintained in **`docs/AURORA_FUNCTION_CATALOG.md`**, section **“ChatGPT-Comparable Functional Scope — Normative Specification Draft”**. That catalog is the single detailed functional inventory; this matrix remains the canonical architecture/domain map and must not duplicate the per-function requirements.

The product scope maps to existing domains as follows:

| Product capability group | Canonical matrix domains | Primary boundary to validate |
|---|---|---|
| Chat, history, projects, personalization and workbench | AI-14, AI-18, AI-11 | AI Studio + existing conversation/artifact APIs |
| Model selection, real inference, context and scheduling | AI-03, AI-04, AI-05, AI-14 | Core ModelHub + Runtime/provider contracts |
| Web research, citations and knowledge retrieval | AI-10, AI-11, AI-19, AI-22 | Memory/RAG + authorized research adapters |
| Memory, retention, isolation and deletion | AI-09, AI-10, AI-11 | Canonical `atc-aurora-memory` + MEMORY-001 |
| Files, documents, spreadsheets and data analysis | AI-05, AI-11, AI-15, AI-18 | Runtime/Python service + sandbox + artifact API |
| Agents, tools, workflows and automation | AI-06, AI-07, AI-08, AI-16 | Existing agent framework + authorized execution path |
| Image, audio, voice and video | AI-03, AI-05, AI-15 | Provider capability negotiation + runtime |
| Code intelligence and repository operations | AI-18, AI-21, AI-22, AI-23 | Developer interfaces + authorized repository adapter |
| Browser, computer and OS interaction | AI-08, AI-12, AI-16 | Scoped Runtime adapters + GlobusOS/ShivaCore enforcement |
| Security, audit, evaluation and operations | AI-01, AI-02, AI-12, AI-13, AI-19, AI-20, AI-21, AI-22, AI-23 | Existing policy, security, operations and evidence gates |
| Aurora domain intelligence and ecosystem extensions | AI-16, AI-17, AI-18 | Existing domain services and canonical ecosystem interfaces |

All requirements in the detailed catalog begin as `SPECIFIED`; this mapping does not promote any component's implementation status. Source discovery, exact-SHA test/CI evidence, integration and E2E verification remain separate AURORA-001 gates. Existing modules and contracts must be reused or extended; any duplicate or disconnected implementation is a release-blocking finding.
