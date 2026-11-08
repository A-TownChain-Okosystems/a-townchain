---
document_id: ATC-DOC-AI-003
title: Aurora AI Architecture
version: 2.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-09
updated: 2026-09-28
standard: ATC-STD-MD-001
---

# Aurora AI Architecture

> Canonical architecture of Aurora AI as the ecosystem intelligence platform.

**Status:** ARCHITECTURE_ONLY  
**Normative authority:** atc-standards

# Canonical System Role — GlobusOS AI Control Plane

Aurora AI is the **AI Control / Intelligence Plane of GlobusOS**.

The canonical system boundary is:

```text
AURORA AI
    │
    │ Intelligence / Planning / Agent Execution
    ▼
GLOBUS OS
    │
    │ OS Services / IPC / Resource Management
    ▼
SHIVACORE
    │
    │ Kernel / TCB / Capabilities / Isolation
    ▼
HARDWARE
```

Aurora is **not** the operating-system kernel and does not own blockchain consensus authority. GlobusOS remains the operating-system integration and service layer; ShivaCore remains the trusted kernel / TCB / capability boundary.

## Aurora Responsibilities

The Aurora platform provides the AI-side capabilities for:

- ModelHub and model/provider integration
- Agent runtime and lifecycle
- Planning and context processing
- Skills and tools
- Memory, RAG and knowledge services
- Multimodal processing
- Dialogue AI, Quest AI, World AI, Character AI, Creature AI, Faction AI and Security AI
- AI-based system analysis
- Authorized automation
- Evaluation, telemetry, provenance and evidence
- Authorized integration with Genesis, A-TownChain and external services

These are architectural responsibilities. Their implementation status is determined exclusively by **AURORA-001 evidence**.

## Canonical Layering

```text
AURORA AI
    │
    ▼
Aurora Authority Plane
    │
    ▼
Aurora Runtime
    │
    ▼
ATC AI Runtime
    │
    +---- CPU Backend
    +---- GPU Backend
    +---- NPU Backend
    │
    ▼
GlobusOS AI Services
    │
    ▼
ShivaCore IPC / Capability Boundary
```

## Authority Boundary

Aurora follows this authority chain:

```text
MODEL
  │ proposes
  ▼
AGENT
  │ requests
  ▼
CAPABILITY
  │ checked by
  ▼
POLICY
  │ may require
  ▼
APPROVAL
  │ authorizes
  ▼
TOOL
  │ executes
  ▼
GLOBUS OS
  │ capability + IPC
  ▼
SHIVACORE
```

The following invariants are mandatory:

- AI models have no implicit execution authority.
- Agents cannot bypass capability checks.
- Capabilities are subject to policy evaluation.
- Approval may be required for sensitive operations.
- Tools execute only through authorized interfaces.
- Aurora has no direct kernel authority.
- Aurora has no direct blockchain consensus authority.
- All privileged execution must be auditable.

## Hardware Target & Backends

The canonical hardware target is ATC AI Compute Platform (ATC-AICP):

```text
ATC-AICP
├── CPU
├── GPU
├── NPU
├── MEMORY
├── STORAGE
├── SECURITY
│   ├── TPM
│   ├── SECURE_PROCESSOR
│   └── TEE
├── BOOT
│   ├── SECURE_BOOT
│   └── MEASURED_BOOT
└── FIRMWARE
```

```text
Aurora Runtime
      │
      +---- CPU Backend
      +---- GPU Backend
      +---- NPU Backend
```

Backends expose capability contracts rather than vendor-specific semantics to Aurora.

## Security Boundary

Aurora communicates with the kernel only through authorized GlobusOS interfaces:

```text
Aurora
   │
   ▼
Authorized GlobusOS Interface
   │
   ▼
IPC / Capability Boundary
   │
   ▼
ShivaCore
```

Aurora does not directly access kernel memory, physical memory, device registers, unrestricted DMA, TPM private key material, or firmware replacement interfaces.

## Completeness and Authority Gate

Every Aurora capability is subject to **AURORA-001 — Platform Completeness & Authority Gate**.

Evidence progression:

```text
ARCHITECTURE_ONLY
        ↓
SPECIFIED
        ↓
IMPLEMENTED
        ↓
TESTED
        ↓
CI_VERIFIED
        ↓
INTEGRATED
        ↓
E2E_VERIFIED
```

Exceptional states are:

```text
MISSING / BLOCKED / DUPLICATE / DISCONNECTED
```

A file, documentation entry or module directory alone is not implementation evidence. CI evidence is valid only when tied to the exact source commit being assessed.

---

## Canonical Architecture Extension — Implementation Boundary (2026-10-12)

This extension preserves the existing architecture contract above while linking it to the canonical implementation and evidence matrix. It does not supersede existing hardware, GlobusOS, ShivaCore, or authority-boundary requirements.

### Canonical implementation matrix

See `docs/AURORA_CANONICAL_FUNCTION_MATRIX.md` for the 23-domain Vision → Component → Function → Evidence matrix. Architectural presence does not imply implementation or verification.

### Runtime authorization contract

The existing Rust core uses `AgentRegistry` plus `CapabilityPolicyEngine` to perform an in-process, fail-closed pre-check. The check denies when the core is inactive, the request principal does not match the agent ID, the agent is absent/inactive, or the agent registry does not grant the requested capability. Policy evaluation occurs only after these checks.

This pre-check is not kernel authorization. The target GlobusOS service and ShivaCore capability/TCB boundary must independently enforce operations. Aurora must not directly access kernel memory, physical memory, device registers, unrestricted DMA, TPM private key material, or firmware replacement interfaces.

### Approval and request integrity

The external action-request envelope carries request identity, principal, capability, action, and scope. A caller-supplied approval boolean is not an authorization grant. Approval must be represented by a trusted, request-scoped grant issued only after the host has authenticated the approver. Deny is terminal; a denied request cannot be converted into an approval-required request.

### Evidence status

```text
Architecture ≠ Implementation ≠ Test ≠ CI Evidence ≠ Verification
```

Exact-SHA CI evidence is required before marking a capability VERIFIED. CI success for the core does not prove end-to-end OS/kernel enforcement.

### Explicit residuals

Full capability-token enforcement remains a separate gate: expiry, nonce/replay protection, audience validation, revocation, identity authentication, and enforcement at the GlobusOS/ShivaCore boundary must be implemented and evidenced. Production model-provider inference, full RAG, multimodal providers, complete agent planning/execution, blockchain signing execution, deployment automation, and end-to-end verification remain unverified until supported by source and exact-SHA evidence.

