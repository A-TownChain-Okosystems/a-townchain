---
document_id: ATC-DOC-AI-010
title: Aurora ChatGPT Scope Source-to-Catalog Audit
version: 0.1.0
status: draft
baseline_commit: 032519b5bac505b25651214ec717f448264c8948
related_pr: 50
---

# Aurora AI — Source-to-Catalog Audit

## 1. Audit boundary and decision rule

This audit is bound to repository commit `032519b5bac505b25651214ec717f448264c8948` (Draft PR #50 head). It maps all 71 requirements in the ChatGPT-comparable scope to a canonical owner, the closest inspected source/symbol, relevant tests and contracts, and an evidence-bounded status.

A path in the “source / symbol” column can mean either (a) an observed implementation, or (b) the canonical place inspected where no implementation was found. “No symbol found” is not proof of global absence; it is a GAP only within the audited canonical path and must remain a residual if another SSOT has not been searched. File presence, inline tests, and green CI do not establish feature completeness.

Status vocabulary used here:
- **PARTIAL** — some relevant behavior exists, but the requirement is not fulfilled end-to-end.
- **GAP** — audited canonical source contains an explicit placeholder or lacks the required behavior.
- **UNMAPPED_RESIDUAL** — canonical symbol/test/evidence not yet established; no absence claim.
- **IMPLEMENTED (unit scope only)** — narrow implementation and inline unit tests exist; not a production or integration claim.
- **SPECIFIED** — requirement exists only in the catalog/specification.

## 2. Requirement-by-requirement mapping

| Requirement | Canonical module / source and symbol (at audited SHA) | Tests / contract found | Audit status |
|---|---|---|---|
| CHAT-01 | `modules/atc-aistudio/src/App.tsx`, `DesktopApp.tsx`; conversation CRUD symbol not established | No conversation lifecycle test identified | UNMAPPED_RESIDUAL |
| CHAT-02 | AI Studio + persistence boundary; inspect `src/db/schema.ts`, `src/lib/indexedDb.ts`; authorized history search symbol not established | No history retention/deletion contract/test identified | UNMAPPED_RESIDUAL |
| CHAT-03 | `modules/atc-aistudio/src/App.tsx`; assistant composer/attachment intake symbol not established | No assistant attachment intake contract/test identified | UNMAPPED_RESIDUAL |
| CHAT-04 | `modules/atc-aurora-runtime/src/agent_runtime.atc`; `start/stop` are TODO placeholders, no streaming/cancel API | Runtime STATUS reports planned / 0 tests | GAP |
| CHAT-05 | `modules/atc-aistudio/src/App.tsx`; response renderer/citation renderer symbol not established | No rendering conformance test identified | UNMAPPED_RESIDUAL |
| CHAT-06 | AI Studio conversation API boundary; edit/regenerate/branch symbols not established | No message operation tests identified | UNMAPPED_RESIDUAL |
| CHAT-07 | AI Studio UI; accessibility/feedback contract and tested entrypoints not established | No relevant accessibility/feedback test identified | UNMAPPED_RESIDUAL |
| MODEL-01 | `modules/atc-aurora-core/src/model_hub.rs`: `ModelHub`, `ModelInfo::register/list_models` | Inline `test_model_hub`; no license/provider compatibility contract | PARTIAL |
| MODEL-02 | `modules/atc-aurora-runtime/src/`; no Rust manifest/typed provider adapter found; Python boundary `modules/atc-aurora-ai/runtime/llm_router.atc` is declarative | No versioned provider ABI test/contract found | GAP |
| MODEL-03 | `modules/atc-aurora-core/src/llm_router.rs`: `LlmRouter::route` uses keyword matching and fixed fallback | Inline `test_router`; no capability/resource negotiation tests | PARTIAL |
| MODEL-04 | `modules/atc-aurora-core/src/model_hub.rs`: `ModelHub::inference` returns formatted placeholder text | Inline tests assert response shape/model name, not provider execution | GAP |
| MODEL-05 | `modules/atc-aurora-memory/src/context_window.rs` and `modules/atc-aurora-ai/memory/context_window.atc`; integration/token accounting not established | No provider token-budget conformance test identified | PARTIAL / UNMAPPED_RESIDUAL |
| MODEL-06 | Runtime `agent_runtime.atc` / `tool_executor.atc` contain TODOs; retry/timeout/cancel contract not found | No bounded retry/cancellation test identified | GAP |
| MODEL-07 | `modules/atc-aurora-core/src/task_scheduler.rs`; integration with model inference/resource quotas not established | Core module tests run in CI; scheduler-to-inference contract not found | PARTIAL |
| MODEL-08 | Evaluation/CI owner not established; nearest quality workflow `.github/workflows/code-quality.yml` | CI is not model-quality evaluation; no benchmark contract/test found | UNMAPPED_RESIDUAL |
| RESEARCH-01 | `modules/atc-aurora-agents/src/agents.rs`, `research_agent.atc`; no authorized web-search adapter symbol established | No search/fetch provenance test identified | UNMAPPED_RESIDUAL |
| RESEARCH-02 | `modules/atc-aurora-agents/src/agent_pool.rs`: `AgentPool::execute`; planning/decomposition workflow not established | Inline `test_pool` checks dispatch only | PARTIAL |
| RESEARCH-03 | Core `audit.rs` is an audit logger; claim-to-source citation/provenance pipeline not established | No citation-grounding test identified | UNMAPPED_RESIDUAL |
| RESEARCH-04 | Repository/document connector boundary not established in canonical Rust modules; AI Studio has GitHub sync views | `modules/atc-aistudio/tests/GitHubRepoSyncView.test.tsx` tests repo sync UI, not general internal search | UNMAPPED_RESIDUAL |
| RESEARCH-05 | Core `task_scheduler.rs`; external monitoring/snapshot diff integration not established | No recurring research monitor test identified | UNMAPPED_RESIDUAL |
| MEMORY-01 | `modules/atc-aurora-memory/src/context_window.rs`: `ContextWindow`; cross-conversation/task isolation not established | No isolation tests identified for this module | PARTIAL |
| MEMORY-02 | `modules/atc-aurora-memory/src/knowledge_base.rs`: `KnowledgeBase::add/query`; `memory_index.rs`: `MemoryIndex::add/by_tag` | Inline `test_kb`, `test_index`; no user-approved durable memory contract enforced in these types | PARTIAL |
| MEMORY-03 | `modules/atc-aurora-memory/src/memory_index.rs`, `vector_store.rs`; update/export/delete operations absent from inspected types | MEMORY-001 documents contract; no executable API conformance demonstrated | GAP |
| MEMORY-04 | `knowledge_base.rs`, `memory_index.rs`, `vector_store.rs` use in-memory collections without owner/tenant/ACL fields | `ATC-AI-MEM-001` specifies isolation; required negative tests not found | GAP |
| MEMORY-05 | `MemoryIndex` and `VectorStore` expose no coordinated deletion API | `ATC-AI-MEM-001 REQ-MEM-003` requires deletion completeness; no deletion-completeness test found | GAP |
| MEMORY-06 | AI Studio project UI candidate `modules/atc-aistudio/src/components/ProjectHubView.tsx`; project-to-chat/files/context binding not established | No cross-scope project isolation test identified | UNMAPPED_RESIDUAL |
| MEMORY-07 | Memory/context modules; explicit preference schema and precedence resolver not established | No personalization policy test identified | UNMAPPED_RESIDUAL |
| MEMORY-08 | `docs/MEMORY-001_CANONICAL_CONTRACT.md` specifies MEMORY-FED-1; no federation runtime in audited memory module | Vector files `tests/memory/memory_record_vectors.json`, `capability_handshake_vectors.json`; execution by Rust tests not established | SPECIFIED / GAP |
| FILE-01 | AI Studio file manager candidates `src/components/FileManagerView.tsx`, `FolderView.tsx`; assistant intake security boundary not established | No hostile embedded-instruction / size/type validation test identified | UNMAPPED_RESIDUAL |
| FILE-02 | Document parser service not established in canonical modules | No parser contract or format fixture suite identified | UNMAPPED_RESIDUAL |
| FILE-03 | OCR/document vision provider not established in canonical modules | No OCR backend conformance test identified | SPECIFIED |
| DATA-01 | No canonical assistant data-analysis service identified; `atc-aistudio` includes dashboard/data views but not a verified analysis execution API | No reproducible analysis test contract identified | UNMAPPED_RESIDUAL |
| DATA-02 | `modules/atc-aurora-runtime/src/sandbox.atc` exists in tree but runtime status says planned; sandbox implementation not verified | No resource/network/filesystem isolation test identified | GAP |
| DATA-03 | AI Studio chart components exist; general artifact-producing statistics pipeline not established | No calculation reproducibility test identified | UNMAPPED_RESIDUAL |
| DATA-04 | Artifact/version/lineage contract not established | No export validation/version comparison test identified | UNMAPPED_RESIDUAL |
| AGENT-01 | `modules/atc-aurora-agents/src/agent_pool.rs`: register/list/execute only; no pause/resume/terminate lifecycle | Inline `test_pool` checks count and dispatch | PARTIAL |
| AGENT-02 | Agent trait `modules/atc-aurora-agents/src/agent_base.rs`; no bounded planner/dependency graph contract established | No planning/failure-path tests identified | UNMAPPED_RESIDUAL |
| AGENT-03 | `agent_base.rs` exposes trait capabilities; runtime `tool_executor.atc` registration is TODO | No versioned tool schema/side-effect contract test found | GAP |
| AGENT-04 | `modules/atc-aurora-core/src/aurora_core.rs`: `authorize_action`; `agent_pool.rs::execute` does not call it | Core inline authorization tests exist; no test proves every tool side effect passes authorization | PARTIAL / BLOCKING |
| AGENT-05 | `modules/atc-aurora-core/src/task_scheduler.rs`; runtime session/tool execution is TODO | Scheduler tests do not establish end-to-end agent workflows | PARTIAL |
| AGENT-06 | `AgentPool` holds independent agents; delegation/state merge/conflict resolution not established | `test_pool` covers dispatch only | UNMAPPED_RESIDUAL |
| AGENT-07 | Runtime tool executor has no verified timeout/budget/cancellation enforcement | No adversarial resource exhaustion tests identified | GAP |
| AGENT-08 | `modules/atc-aurora-core/src/approval.rs`: one-shot request-scoped approval object; authenticated host adapter explicitly not integrated | Inline tests cover replay/duplicate/reject; host identity/authentication tests absent | PARTIAL / BLOCKING |
| MULTI-01 | No canonical image-understanding provider API identified; AI Studio image tools are not proof of assistant vision backend | No image-understanding provider tests identified | UNMAPPED_RESIDUAL |
| MULTI-02 | `modules/atc-aistudio/src/components/ImageGeneratorTab.tsx` is a UI candidate; backend execution/authorization not established | No backend conformance test identified | UNMAPPED_RESIDUAL |
| MULTI-03 | Audio UI candidate `AiAudioEngineTab.tsx`; speech/audio backend contract not established | No STT/TTS/audio interruption tests identified | UNMAPPED_RESIDUAL |
| MULTI-04 | Runtime streaming/session API not established; `agent_runtime.atc` start/stop are TODO | No reconnect/turn-taking tests identified | GAP |
| MULTI-05 | Video UI candidate `VideoGeneratorTab.tsx`; video-understanding backend not established | No timestamped frame analysis tests identified | UNMAPPED_RESIDUAL |
| MULTI-06 | No provider capability negotiation for mixed media established; core model metadata only has name/version/context window/loaded flag | `test_model_hub` does not test multimodal capability negotiation | GAP |
| DEV-01 | AI Studio developer views exist; general assistant code-generation workflow not mapped to an authoritative symbol | No assistant code workflow test identified | UNMAPPED_RESIDUAL |
| DEV-02 | `modules/atc-aistudio/src/services/githubSync.ts`, `src/components/GitHubRepoSyncView.tsx`; scoped repository intelligence is not established by sync UI alone | `tests/GitHubRepoSyncView.test.tsx` | PARTIAL |
| DEV-03 | `src/components/AtvmSandboxView.tsx` and its test are sandbox UI candidates; safe repository test execution boundary not established | `AtvmSandboxView.test.tsx` tests UI scope, not a secure general-purpose code runner | UNMAPPED_RESIDUAL |
| DEV-04 | AI Studio editor/component surfaces; stable artifact ID/version/provenance contract not established | No artifact lifecycle test identified | UNMAPPED_RESIDUAL |
| DEV-05 | No unified versioned extension API across model/tool/agent/skill found in audited contracts | No extension compatibility suite identified | GAP |
| DEV-06 | GitHub adapter/UI candidates above; repository automation authorization must be traced through execution boundary | UI sync test exists; no permission-negative integration test identified | PARTIAL / BLOCKING |
| OPS-01 | No authorized browser/computer-use runtime adapter identified | No scoped browser action / prompt-injection tests identified | UNMAPPED_RESIDUAL |
| OPS-02 | GlobusOS boundary is documented in `docs/GLOBUSOS_INTEGRATION.md`; actual file/network/device capability adapter not found in audited runtime | Capability-token spec remains draft; no host-enforcement evidence | SPECIFIED / GAP |
| OPS-03 | AI Studio notification/session-continuity owner not established | No completion notification/resume tests identified | UNMAPPED_RESIDUAL |
| OPS-04 | `modules/atc-aistudio/src/lib/syncLogic.ts` has sync logic; local AI runtime feature declaration/reconciliation not established | `syncLogic.test.ts` tests sync logic only | PARTIAL |
| SECURITY-01 | `modules/atc-aurora-core/src/capability_policy.rs`: principal/capability/action/scope exact match; token issuer/audience/expiry/revocation not modeled here | Inline policy tests; `ATC-AI-CAP-001` is draft, implementation pending | PARTIAL / BLOCKING |
| SECURITY-02 | `CapabilityPolicyEngine`, `ApprovalEngine`; host authentication and global per-invocation enforcement not integrated | Inline policy/approval tests cover local decisions only | PARTIAL / BLOCKING |
| SECURITY-03 | Security invariant documented in policy source, but tool/web/file content intake and result-validation pipeline not established | No end-to-end prompt-injection/adversarial tool test identified | GAP |
| SECURITY-04 | Config/audit source exists; secret redaction, DLP and data minimization contract not established in audited execution path | No secret-leakage/DLP test identified | UNMAPPED_RESIDUAL / BLOCKING |
| SECURITY-05 | Runtime sandbox is a declarative TODO/planned surface; emergency-stop integration not established | No sandbox escape/quota/stop tests identified | GAP |
| GOVERNANCE-01 | `modules/atc-aurora-core/src/audit.rs`: `AuditLogger` / `AuditRecord`; end-to-end linkage of policy, approval, tool, artifact not proven | Core unit tests if present do not establish complete provenance chain; inspect exact test cases before promotion | PARTIAL |
| GOVERNANCE-02 | Core events/audit exist; runtime tracing, metrics and secret-safe telemetry integration not established | No end-to-end observability evidence identified | PARTIAL / UNMAPPED_RESIDUAL |
| GOVERNANCE-03 | `.github/workflows/code-quality.yml` runs module tests and clippy; quality/groundedness/security evaluation suite not established | Exact-SHA workflow results listed in §4 | PARTIAL |
| GOVERNANCE-04 | `.github/workflows/dependency-review.yml`, CodeQL and repository governance workflows exist; signed/reproducible release and rollback evidence not established by these alone | Workflows are not a release-signing/rollback proof | PARTIAL |
| GOVERNANCE-05 | `docs/audits/AURORA-001-COMPLETENESS.yaml` and `.atc/evidence/evidence.yaml` are evidence locations; per-requirement binding remains incomplete | PR-head workflow run IDs recorded in §4; no E2E evidence established | PARTIAL |
| DOMAIN-01 | `modules/atc-aurora-agents/src/agents.rs` and domain `.atc` files; domain intelligence behavior contracts not established | Agent pool dispatch test does not validate domain capability | UNMAPPED_RESIDUAL |
| DOMAIN-02 | `docs/GLOBUSOS_INTEGRATION.md` and AI Studio integration views; authoritative Genesis/ATC/GlobusOS action adapters not established | No cross-repository end-to-end integration test identified | UNMAPPED_RESIDUAL |

## 3. Revalidated P0 findings

### Inference
- `ModelHub::inference` checks whether a name is registered, then returns a generated placeholder string. It does not invoke a model/provider.
- The current inline test `test_model_hub` and UTF-8 regression test `test_inference_handles_multibyte_prompt_boundary` establish narrow behavior only, not inference correctness.
- Required follow-up: provider trait/ABI, one real test provider, success/error/timeout/cancellation tests, and an integration test proving the router reaches the provider.

### Authorization and approval
- `CapabilityPolicyEngine` has deny-by-default behavior for unknown and duplicate matching policies, exact principal/capability/action/scope matching, and approval-required decisions.
- `AuroraCore::authorize_action` checks core active state, principal identity, agent active state, registered capability and policy.
- `AgentPool::execute` directly calls `agent.execute(ctx)`; no call to `authorize_action` or an equivalent guard is present in this inspected function. Thus the local policy unit tests do not prove all side effects are mediated.
- `ApprovalEngine` uses a one-shot request-bound grant, but `VerifiedApproval::from_authenticated_host` is documented as a future host adapter and is crate-private. Actual human authentication is not integrated.
- Capability-token specification ATC-AI-CAP-001 remains a draft; token expiry, audience, revocation and replay enforcement are not represented by the inspected `CapabilityPolicy` structure.
- Required follow-up: one canonical execution gateway used by every tool invocation; deny-by-default negative tests for missing/wrong/revoked/expired scope and unauthenticated/replayed approval; prove a denied action never reaches the tool.

### Memory
- The inspected `KnowledgeBase`, `MemoryIndex` and `VectorStore` use in-memory collections and expose add/query or store/search primitives.
- No owner/tenant/agent ACL fields, provenance/integrity enforcement, retention policy, delete operation, or coordinated index/embedding/cache deletion is present in these inspected types.
- MEMORY-001 and ATC-AI-MEM-001 are documentation contracts; the compatibility JSON vectors exist, but the audit has not established that the Rust module executes them.
- Required follow-up: one canonical memory API enforcing owner/scope on every read/write, provenance validation, retention/deletion orchestration, and negative cross-tenant plus deletion-completeness tests.

### Runtime
- The Rust manifests for core, agents and memory exist; no Cargo.toml is present under `modules/atc-aurora-runtime` in the audited tree.
- Runtime ATCLang files contain TODO/default-return placeholders. `ARCHITECTURE.md` and `STATUS.md` still describe the module as planned / zero tests, despite the files existing in the tree.
- A second declarative tool executor exists at `modules/atc-aurora-ai/runtime/tool_executor.atc`; it also contains stub bodies. This is a duplicate/disconnected implementation risk, not a reason to create a third executor.
- Required follow-up: designate one canonical execution owner and retire/bridge the duplicate through a documented SSOT decision; add parser/runtime execution tests and tool-boundary integration evidence before implementation claims.

## 4. Exact-SHA CI evidence observed

Commit: `032519b5bac505b25651214ec717f448264c8948`.

| Workflow | Run ID | Result | Evidence URL |
|---|---:|---|---|
| Repository Governance (ATC-STD-201/202/203) | 39458604443 | success | https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39458604443 |
| SDK Build and Test | 39458604344 | success | https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39458604344 |
| Dependency Review | 39458604775 | success | https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39458604775 |
| Aurora AI Code Quality | 39458604332 | success | https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39458604332 |
| CodeQL | 39458604435 | success | https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39458604435 |

These are PR-triggered workflow run results associated with the exact head SHA. They establish that the configured jobs passed, not that the 71 product requirements are implemented or E2E-verified. No separate combined commit status entries were returned by the status endpoint. Job/step-level evidence should be attached to each implementation task as relevant.

## 5. Minimal implementation work packages — proposed, not authorized or started

### P0-A — Real inference provider path
**Owner:** `modules/atc-aurora-core` + designated canonical runtime/provider adapter.
**Scope:** Replace placeholder inference behind a versioned provider interface; preserve explicit mock mode only when marked as such.
**Tests:** fake-provider success; unknown model; provider error; timeout; cancellation; UTF-8 input; router-to-provider integration; verify no silent mock fallback.
**Evidence:** exact commit SHA; source path/symbol/blob SHA; test names and outputs; CI Run→Job→Step logs; provider integration test result. No real model credentials in CI logs.

### P0-B — Single authorized tool execution gateway
**Owner:** choose one canonical runtime execution path; do not extend both ATCLang tool-executor stubs independently.
**Scope:** `Intent → Policy → Capability → Approval if required → Tool invocation → Result validation → Audit`. Every side effect must pass through the gateway immediately before execution.
**Tests:** missing policy/capability denied; wrong principal/scope denied; inactive agent denied; approval absent/rejected/expired/replayed denied; allowed exact tuple invokes once; denied request invokes zero times; tool output validation and audit correlation.
**Evidence:** call graph showing all tool paths enter the gateway; negative-test logs; exact-SHA CI; integration test with a test tool that records invocation count.

### P0-C — Memory security and deletion
**Owner:** `modules/atc-aurora-memory` and existing MEMORY-001 contract.
**Scope:** tenant/user/agent owner + ACL, provenance/integrity, read/write capability checks, retention, delete API that covers primary records, tag indexes, vectors and derived caches; no second memory store.
**Tests:** cross-tenant read/write denied; wrong-agent denied; malformed provenance rejected; retention expiry; deletion completeness across all stores; vector version mismatch rejected; poisoning content never treated as authority.
**Evidence:** contract-to-symbol mapping, unit and adversarial test results, deletion inventory, exact-SHA CI, integration proof that derived records cannot be retrieved after deletion.

### P0-D — Runtime ownership and executable boundary
**Owner:** architecture decision for `modules/atc-aurora-runtime` versus `modules/atc-aurora-ai/runtime`.
**Scope:** record canonical SSOT and dependency direction; only then replace TODOs with executable lifecycle/tool/sandbox contracts.
**Tests:** parse/compile valid and invalid runtime files; register/invoke tool through P0-B gateway; timeout/cancel; resource and filesystem/network isolation; failed startup and cleanup.
**Evidence:** SSOT decision and dependency graph, runnable tests on exact SHA, negative security tests, CI logs and end-to-end tool invocation evidence.

## 6. Release gate

No broad product implementation should be authorized until P0-A through P0-D have an accepted owner, stable contract, explicit tests and exact-SHA evidence. Keep all other requirements as `UNMAPPED_RESIDUAL` or `SPECIFIED` until their canonical source, symbol, contract and test are actually established. No E2E or production-ready claim is made by this audit.


## 7. Final CI evidence for audit-document head

The audit document itself is present in commit `81675db31375bc4b72b0a969d630c7b58b517a91`. All five PR-triggered workflows for that commit subsequently completed successfully. This is documentation-branch CI evidence only; it does not prove the functional requirements are implemented.

| Workflow | Run | Job / ID | Key validation steps | Conclusion |
|---|---|---|---|---|
| Repository Governance (ATC-STD-201/202/203) | [39459145387](https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39459145387) | `audit` / `118753709044` | Repository-Audit (R-Level aus .atc/repository.yaml) | success |
| Dependency Review | [39459145388](https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39459145388) | `dependency-review` / `118753709163` | Dependency Review | success |
| CodeQL | [39459145412](https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39459145412) | `analyze` / `118753709393` | CodeQL init; CodeQL analyze | success |
| SDK Build and Test | [39459145378](https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39459145378) | `cargo` / `118753709133` | Build; Test | success |
| Aurora AI Code Quality | [39459145415](https://github.com/A-TownChain-Okosystems/aurora-ai/actions/runs/39459145415) | `python` / `118753709152`; `rust` / `118753709679`; `node` / `118753710021` | Compile Python modules; Validate Rust modules; Validate Node module | success |

All listed jobs and their named validation steps report `completed/success`. These runs are associated with head SHA `81675db31375bc4b72b0a969d630c7b58b517a91`; they must not be conflated with the earlier source-inspection baseline `032519b5bac505b25651214ec717f448264c8948`. The PR remains a Draft. No implementation, security integration, or E2E status is promoted by these documentation-only CI results.
