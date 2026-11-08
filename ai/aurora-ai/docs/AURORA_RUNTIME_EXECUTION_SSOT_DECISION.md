---
document_id: ATC-DOC-AI-SSOT-004
title: Aurora Runtime and Tool Execution SSOT Decision
version: 1.0.0
status: proposed
owner: A-TownChain-Okosystems
created: 2026-10-24
standard: atc-standards
---

# Aurora Runtime and Tool Execution SSOT Decision

## Decision status

**PROPOSED — requires component-owner and normative-catalog review.** This document resolves the repository-level design direction for issue [#51](https://github.com/A-TownChain-Okosystems/aurora-ai/issues/51). It does not establish that the implementation is complete, integrated, or verified.

## Binding architecture constraints

- Standalone First · Native Implementation · Ecosystem Second.
- Models, prompts, retrieved memory, plans, and agent-generated tool arguments carry no authority by themselves.
- Policy decisions and approval validation must occur immediately before every privileged side effect.
- ShivaCore/GlobusOS independently enforce OS-level capabilities, process isolation, IPC, and resource/network/filesystem boundaries. Aurora is outside the kernel TCB.
- The ecosystem repository is an integration/evidence control plane and must not host duplicate Aurora runtime or policy implementations.

## Canonical ownership

### 1. Authorization and privileged action gateway

**Canonical implementation owner:** `modules/atc-aurora-core` (Rust crate `atc-aurora-core`).

Relevant existing source symbols:
- `modules/atc-aurora-core/src/aurora_core.rs::AuroraCore::authorize_action`
- `modules/atc-aurora-core/src/capability_policy.rs`
- `modules/atc-aurora-core/src/approval.rs`
- `modules/atc-aurora-core/src/audit.rs`

The only supported in-process privileged-action gateway is to be implemented in this crate and must perform a fail-closed authorization check immediately before invoking the side-effect closure. The existing open PR #48 proposes this gate; it is not considered merged or verified by this decision. No caller may invoke a privileged adapter around the gateway.

### 2. Agent/task orchestration

**Existing implementation owner:** `modules/atc-aurora-core` for its current Rust task-scheduling primitives, with agent-specific orchestration remaining in the existing canonical agent module after catalog/source reconciliation. Do not move policy ownership into the agent pool, and do not introduce another task scheduler as part of the executor fix.

The final dependency direction must remain acyclic:
`agents / task scheduler → core authorization gateway → authorized adapter → GlobusOS interface → ShivaCore enforcement`.

### 3. ATCLang runtime files

The following paths are currently stubs/planned ATCLang sources, not verified executable implementations:

- `modules/atc-aurora-runtime/src/tool_executor.atc`
- `modules/atc-aurora-runtime/src/agent_runtime.atc`
- `modules/atc-aurora-ai/runtime/tool_executor.atc`
- `modules/atc-aurora-ai/runtime/agent_runtime.atc`

The `atc-aurora-runtime` directory has no Rust `Cargo.toml` at the inspected source baseline. The two tool-executor paths must not independently execute side effects. Classify `modules/atc-aurora-runtime` as a planned ATCLang interface/specification pending a demonstrated compiler/test contract; classify the duplicate `modules/atc-aurora-ai/runtime` executor as legacy/disconnected and do not extend it. Any future migration or deletion requires catalog reconciliation and a separate reviewed change.

This decision does **not** claim that ATCLang files compile, that all tool paths use the Rust gateway, or that OS isolation is implemented by Aurora.

## Required execution contract

1. Resolve the authenticated principal and active agent identity.
2. Validate request identity, action, scope, capability and policy version.
3. Reject missing, malformed, ambiguous, expired, revoked, replayed or mismatched grants by default.
4. If approval is required, validate a one-shot approval bound to the exact request and authenticated approver. An untrusted caller must not be able to mint a verified approval.
5. Re-check authorization immediately before the side effect; invoke the side-effect closure exactly once on allow and zero times on deny/error.
6. Validate output and bind request, decision, approval, tool identity and result in correlated audit evidence.
7. Pass through the authorized GlobusOS interface; ShivaCore independently enforces OS-level capabilities. Aurora policy is not a substitute for kernel enforcement.

## Acceptance criteria for implementation

- Static call-path audit demonstrates that every side-effecting tool/adapter is reachable only through the canonical gateway.
- Negative tests cover missing policy/capability/identity/approval, inactive agent, wrong principal/scope/action, expired or revoked grant, replay, host-authentication failure, and malformed output.
- Each denied request invokes the test side effect zero times; each allowed request invokes it exactly once.
- Integration tests cover agent dispatch → gateway → test adapter and prove no side effect occurs on any failure path.
- Memory/context supplied to an agent never grants a capability or approval.
- Rust formatting, tests and Clippy run on the exact source commit. ATCLang code is not counted as tested unless the authoritative compiler and tests execute it.
- Exact commit SHA, path/symbol/blob SHA, test names/results, Actions Run → Job → Step logs/exit status and artifact SHA-256 are recorded.
- Missing or mismatched evidence remains BLOCKED. Green CI alone does not establish VERIFIED.

## Related work and gates

- [Issue #51 — canonical runtime/tool-execution SSOT](https://github.com/A-TownChain-Okosystems/aurora-ai/issues/51)
- [Issue #52 — route every side-effecting tool invocation through the authorization gateway](https://github.com/A-TownChain-Okosystems/aurora-ai/issues/52)
- [PR #48 — proposed fail-closed authorized action executor](https://github.com/A-TownChain-Okosystems/aurora-ai/pull/48)
- [PR #49 — Rust module validation workflow](https://github.com/A-TownChain-Okosystems/aurora-ai/pull/49)

PR #49 must be reviewed and, if accepted, landed before it can be relied upon as a base-branch validation workflow. Then rerun the relevant component PRs and inspect exact-head logs. This document must not be marked IMPLEMENTED or VERIFIED until the source and tests prove the stated behavior.
