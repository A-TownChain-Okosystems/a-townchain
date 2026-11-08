---
document_id: ATC-DOC-AI-009
title: Aurora AI Function Inventory and Gap Register
version: 0.1.0
status: draft
owner: A-TownChain-Okosystems
standard: ATC-STD-MD-001
source_branch: fix/aurora-core-p0-input-safety
source_head_at_creation: 3a8318d5cf2043b9e0bcc1996205d225fc035f5e
---

# Aurora AI Function Inventory and Gap Register

This is a source-backed inventory snapshot, not a claim of platform completeness. File presence and unit tests do not prove security or production readiness. The source SHAs below are Git blob SHAs observed during inventory; the repository commit SHA for this branch must be recorded separately by CI/review.

## Status vocabulary

- `SPECIFIED`: contract documented; no implementation claim.
- `IMPLEMENTED`: source implementation exists.
- `TESTED`: relevant test executed successfully.
- `CI_VERIFIED`: passing CI evidence is tied to the exact commit SHA.
- `INTEGRATED`: dependency integration is demonstrated.
- `E2E_VERIFIED`: end-to-end evidence is recorded.
- `GAP`: required behavior not found in inspected source.
- `UNMAPPED_RESIDUAL`: inventory is incomplete or no authoritative source/test mapping is available.

## Inspected source inventory

| Function ID | Module / source | Observed implementation | Source blob SHA | Status from inspection |
|---|---|---|---|---|
| AUR-CORE-001 | `modules/atc-aurora-core/src/aurora_core.rs` | `AuroraCore::new/start/stop/is_active/process_request`; lifecycle coordinator | `edf81f530e1a759f3b8e2af3d5f7993b59c319b4` before this branch's change | IMPLEMENTED; CI pending |
| AUR-CORE-002 | `modules/atc-aurora-core/src/model_hub.rs` | Model registration, default model, listing, placeholder inference response | `a4e38b68dcba4bd9c71c527ad089e0b2bd922e28` before this branch's change | IMPLEMENTED; real model inference GAP |
| AUR-CORE-003 | `modules/atc-aurora-core/src/llm_router.rs` | Keyword-based route rules and fallback model | `c0615d7f4106af3ef2188e69da849bcc29ce54e4` | IMPLEMENTED; provider/capability negotiation GAP |
| AUR-CORE-004 | `modules/atc-aurora-core/src/agent_registry.rs` | In-memory agent registry, role/capability labels, active flag | `ad2d7826e66ef3c20924ae10e16b6bb5abd1c32a` | IMPLEMENTED; capability enforcement GAP |
| AUR-CORE-005 | `modules/atc-aurora-core/src/config_manager.rs` | In-memory string key/value defaults and accessors | `b1adf540f1b4dc60259e3e65759b2f9bd23796a0` | IMPLEMENTED; typed schema/validation/secrets GAP |
| AUR-AGENT-001 | `modules/atc-aurora-agents/src/lib.rs` | Exposes `agents`, `agent_base`, `agent_pool` modules | `6bb82069e246b9ba474a72c0a3780c0806122151` | IMPLEMENTED; orchestration/limits require deeper audit |
| AUR-MEM-001 | `modules/atc-aurora-memory/src/lib.rs` | Exposes knowledge base, vector store, index, learning pipeline, context window | `43165c771dd83e2bfd6294ffb8554fd272ccc77d` | IMPLEMENTED; ACL/retention/poisoning controls not yet verified |
| AUR-RUNTIME-001 | `modules/atc-aurora-runtime/README.md` | Describes runtime/tool system; expected Cargo manifest and source entrypoint not found at the initially probed paths | `20e47459be937269e33022d768f1a1b0ed63c6bd` | UNMAPPED_RESIDUAL |
| AUR-PY-001 | `modules/atc-aurora-ai/README.md`, `requirements.txt` | Python AI service description and dependencies; pyproject manifest not found at the probed path | README blob SHA `07f0456835f8a4ca419c81ba09159e6e623788bd` | UNMAPPED_RESIDUAL |

## P0 security and correctness gaps

| Gap ID | Requirement | Evidence found | Required next action | Release status |
|---|---|---|---|---|
| AUR-GAP-001 | Policy engine, deny-by-default decisions and decision records | No policy engine found in inspected core sources | Locate canonical policy implementation or add an explicit policy module and tests | BLOCKING |
| AUR-GAP-002 | Capability enforcement before every tool/service invocation | Registry stores capability labels, but inspected `process_request` did not enforce them | Add authorization interface; negative tests for missing, expired, wrong-scope and revoked capabilities | BLOCKING |
| AUR-GAP-003 | Explicit approval gate for privileged operations | No approval contract found in inspected core sources | Define approval states; ensure reject/timeout never means approval | BLOCKING |
| AUR-GAP-004 | Tool contracts, side-effect declarations and service boundary | No authoritative tool execution contract found in inspected core sources | Map runtime source and enforce input/output schemas, timeouts, cancellation and resource limits | BLOCKING |
| AUR-GAP-005 | Typed, validated configuration | Config values are arbitrary strings | Define versioned schema, reject invalid values, prohibit secrets in model context/logs | BLOCKING |
| AUR-GAP-006 | Structured audit/provenance | No end-to-end decision/tool provenance found in inspected core sources | Add correlation IDs and append-only records linking intent, policy, approval, tool and result | BLOCKING |
| AUR-GAP-007 | Real model provider ABI and inference | `ModelHub::inference` returns a formatted placeholder string | Implement provider contract or mark the API explicitly as a mock; test provider failures and fallback | BLOCKING for model claims |
| AUR-GAP-008 | Agent state enforcement | `process_request` previously accepted registered inactive agents | This branch adds inactive-agent rejection and a regression test | CI pending |
| AUR-GAP-009 | UTF-8-safe prompt preview | `ModelHub::inference` sliced a string by byte index and could panic on a multibyte boundary | This branch uses character iteration and adds a regression test | CI pending |
| AUR-GAP-010 | Current exact-SHA verification | `STATUS.md` reports current build/test evidence as UNKNOWN / NOT VERIFIED until linked | Run CI on the proposed PR head and capture run/job/step/exit/log evidence | BLOCKING |

## Required evidence mapping

For every function, complete the following fields before promoting its status beyond `IMPLEMENTED`:

- stable function ID and priority;
- canonical module and contract/API signature;
- input/output schemas and error behavior;
- security, privacy and resource constraints;
- dependencies and failure behavior;
- source path, commit SHA, blob SHA and line range;
- test path and test name;
- exact GitHub Actions Run → Job → Step → exit code/log URL;
- separate implementation, test, CI and integration statuses.

## Architecture invariants

1. Aurora is a user-space AI service; it is not ShivaCore TCB, kernel authority, blockchain consensus, or canonical ATC-VM.
2. Required action path: Intent/Proposal → Policy → Capability Check → Approval when required → Tool/Service Boundary → authoritative runtime.
3. Missing policy, capability, required approval or trusted evidence means deny/fail closed.
4. Model output and retrieved content are untrusted data; they cannot grant authority or override policy.
5. No production-ready status until P0 gaps are closed and exact-SHA CI/security evidence is linked.

## Scope note

This inventory covers only files actually inspected during the current pass. It is intentionally conservative. It is not a complete recursive source-tree census, and absence in this table must not be interpreted as proof that a capability is absent elsewhere in the repository.


## ChatGPT-Comparable Functional Scope — Normative Specification Draft

**Status:** SPECIFIED (target requirements only); repository implementation and verification status remain independent.  
**Scope:** Functional coverage comparable in breadth to a general-purpose ChatGPT assistant. This does not assert feature parity, availability of a specific model/provider, or production readiness.  
**Authority:** `atc-standards` remains the normative standards registry. This section extends this catalog; it does not create a parallel runtime, chat core, memory store, policy engine, or model registry.

### Classification and ownership rules

- The IDs below are catalog identifiers for traceability, not approved standard IDs. Register or ratify identifiers only through the canonical standards process.
- Initial status for each new requirement is `SPECIFIED`. Use `MISSING`, `BLOCKED`, `DUPLICATE`, or `DISCONNECTED` where the source audit supports that finding; never infer implementation from this specification.
- Canonical ownership defaults: conversation/UI → `modules/atc-aistudio` with core/runtime APIs; model registry/routing → `modules/atc-aurora-core`; inference/scheduling/provider integration → `modules/atc-aurora-runtime` and existing Python AI services where appropriate; agents/workflows → `modules/atc-aurora-agents`; memory/RAG/retrieval → `modules/atc-aurora-memory`; authority, OS and chain enforcement remain with their canonical policy, GlobusOS, ShivaCore and A-TownChain interfaces.
- Ownership is provisional until source symbols, contracts, dependencies and existing SSOTs are inspected. Reuse or extend the existing owner; do not create a competing subsystem to satisfy a row.

### Functional component catalog

| Requirement ID | Domain / component | Required behavior | Canonical boundary | Initial status |
|---|---|---|---|---|
| CHAT-01 | Chat lifecycle | Create, open, continue, rename, archive and delete conversations | AI Studio + existing persistence API | SPECIFIED |
| CHAT-02 | History and search | Sort and search authorized history; enforce retention and deletion | AI Studio + canonical storage | SPECIFIED |
| CHAT-03 | Composer and attachments | Text, code, supported files and multimodal inputs | AI Studio + intake APIs | SPECIFIED |
| CHAT-04 | Streaming and cancellation | Stream partial responses; cancel; surface errors and supported recovery | Runtime API + UI | SPECIFIED |
| CHAT-05 | Response rendering | Markdown, code, tables, math, citations and structured artifacts | AI Studio | SPECIFIED |
| CHAT-06 | Message operations | Copy, edit, regenerate, continue and branch where supported | AI Studio + conversation API | SPECIFIED |
| CHAT-07 | Feedback and accessibility | Feedback attribution, keyboard navigation, screen-reader support and localization | AI Studio + telemetry | SPECIFIED |
| MODEL-01 | Model registry | Versioned model metadata, capabilities, license and runtime compatibility | Aurora Core / existing ModelHub | SPECIFIED |
| MODEL-02 | Provider adapters | Common versioned ABI for local and remote inference providers | Runtime + Python AI services | SPECIFIED |
| MODEL-03 | Routing and negotiation | Select by capabilities, policy, availability, context limits and resource budgets | Existing ModelHub / Runtime | SPECIFIED |
| MODEL-04 | Real inference | Execute a real provider/model; distinguish mock and placeholder behavior explicitly | Runtime/provider adapter | SPECIFIED |
| MODEL-05 | Context and tokens | Context budgeting, truncation, prioritization and safe summarization | Runtime + Memory interface | SPECIFIED |
| MODEL-06 | Retry, fallback and cancellation | Bounded retry/fallback, timeout and cancellation semantics | Runtime | SPECIFIED |
| MODEL-07 | Inference scheduling | Queueing, concurrency, priorities, quotas and resource accounting | Runtime | SPECIFIED |
| MODEL-08 | Evaluation | Compare model quality, latency, cost and failure behavior by version | Evaluation/CI | SPECIFIED |
| RESEARCH-01 | Web search and fetch | Search, fetch and parse accessible pages with provenance | Authorized runtime adapter | SPECIFIED |
| RESEARCH-02 | Deep research | Multi-step research plans, source comparison and synthesized reports | Agents + Runtime | SPECIFIED |
| RESEARCH-03 | Citations and evidence | Link claims to fetched source locations; flag unsupported or conflicting claims | Research pipeline | SPECIFIED |
| RESEARCH-04 | Internal search | Search authorized repositories, documents and connected knowledge sources | Existing connectors / adapters | SPECIFIED |
| RESEARCH-05 | Monitoring | Schedule permitted checks and compare results against prior snapshots | Existing automation/scheduler integration | SPECIFIED |
| MEMORY-01 | Working and conversation context | Scope short-lived context to the correct task and conversation | Existing Memory/Runtime contracts | SPECIFIED |
| MEMORY-02 | Long-term memory | Store and retrieve user-approved durable facts with provenance | `atc-aurora-memory` + MEMORY-001 | SPECIFIED |
| MEMORY-03 | Memory controls | Inspect, correct, export and delete memory subject to policy | Existing Memory API | SPECIFIED |
| MEMORY-04 | Isolation and retention | Tenant/user/agent isolation, expiry, access controls and retention enforcement | Existing Memory API + policy | SPECIFIED |
| MEMORY-05 | Derived-data deletion | Propagate deletion to indexes, embeddings, caches and derived summaries | Existing Memory/RAG storage | SPECIFIED |
| MEMORY-06 | Projects | Scope instructions, chats, files and context to a named workspace/project | AI Studio + existing storage APIs | SPECIFIED |
| MEMORY-07 | Personalization | Apply explicit preferences without overriding safety or current user intent | Existing Memory/Context API | SPECIFIED |
| MEMORY-08 | Federation | Authenticated, consented cross-instance sync only if MEMORY-001 implementation is verified | Existing MEMORY-FED-1 contract | SPECIFIED |
| FILE-01 | File intake and security | Validate type, size, content handling, permissions and untrusted embedded instructions | Runtime intake boundary | SPECIFIED |
| FILE-02 | Document parsing | Extract text, page structure, tables and metadata from supported formats | Existing parser services | SPECIFIED |
| FILE-03 | OCR and document vision | Process scans/images only when an actual backend is available | Provider/runtime | SPECIFIED |
| DATA-01 | Spreadsheet and data analysis | Inspect, clean, transform and calculate over user-provided data | Existing Python/data service boundary | SPECIFIED |
| DATA-02 | Sandboxed code execution | Execute analysis code with explicit CPU, memory, time, filesystem and network limits | Runtime sandbox | SPECIFIED |
| DATA-03 | Statistics and visualization | Produce reproducible calculations and charts from declared input data | Data service / artifact interface | SPECIFIED |
| DATA-04 | Export and comparison | Validate generated files; compare document/data versions and record lineage | Artifact interface | SPECIFIED |
| AGENT-01 | Agent lifecycle and state | Create, start, pause, resume, terminate and validate state transitions | `atc-aurora-agents` | SPECIFIED |
| AGENT-02 | Planning and decomposition | Break tasks into bounded steps with dependencies, progress and failure paths | Existing agent planner boundary | SPECIFIED |
| AGENT-03 | Tool registry and schemas | Register versioned tools with input/output schema, risk and required capabilities | Core/Runtime tool contract | SPECIFIED |
| AGENT-04 | Authorized invocation | Validate request, re-check capability/policy immediately before side effects and audit result | Existing capability/policy execution path | SPECIFIED |
| AGENT-05 | Workflow and scheduling | Support bounded multi-step, scheduled and event-triggered workflows | Existing Runtime/scheduler | SPECIFIED |
| AGENT-06 | Multi-agent coordination | Delegate, isolate state, merge results and resolve conflicting outputs | `atc-aurora-agents` | SPECIFIED |
| AGENT-07 | Limits and recovery | Enforce time/token/resource budgets, bounded retries, cancellation and compensation | Runtime + agents | SPECIFIED |
| AGENT-08 | Human approval | Bind approval to authenticated approver, request, principal, action and scope; reject replay | Existing approval contract + host adapter | SPECIFIED |
| MULTI-01 | Image understanding | Analyze supplied images and report uncertainty/limitations | Provider/runtime | SPECIFIED |
| MULTI-02 | Image generation and editing | Generate or edit images through an available, declared backend | Provider/runtime + artifact API | SPECIFIED |
| MULTI-03 | Speech and audio | Speech-to-text, text-to-speech, audio understanding and interruption handling | Provider/runtime | SPECIFIED |
| MULTI-04 | Voice sessions | Manage streaming sessions, cancellation, reconnects and turn-taking | Runtime + AI Studio | SPECIFIED |
| MULTI-05 | Video understanding | Analyze supported video/frames with timestamps and input limits | Provider/runtime | SPECIFIED |
| MULTI-06 | Multimodal context | Join text, files, images and audio only within provider capability and privacy constraints | Runtime + Context Engine | SPECIFIED |
| DEV-01 | Code assistance | Explain, generate, refactor and review code against supplied context | AI Studio + model/runtime APIs | SPECIFIED |
| DEV-02 | Repository intelligence | Inspect files/symbols/dependencies and respect repository boundaries | Authorized repository adapter | SPECIFIED |
| DEV-03 | Debugging and tests | Reproduce failures, propose fixes and run permitted tests in isolation | Workbench / execution adapter | SPECIFIED |
| DEV-04 | Workbench and artifacts | Iteratively edit documents/code/tables and preserve versions/provenance | AI Studio + artifact API | SPECIFIED |
| DEV-05 | SDK/API/extensions | Versioned APIs and extension contracts for models, tools, agents and skills | Existing developer surface | SPECIFIED |
| DEV-06 | Repository automation | Work with issues, PRs and CI using scoped repository permissions and governance rules | Authorized GitHub adapter | SPECIFIED |
| OPS-01 | Browser/Computer Use | Navigate supported sites and UI only through an explicitly authorized, scoped adapter | Runtime integration | SPECIFIED |
| OPS-02 | Filesystem/network/device adapters | Restrict each operation to approved scope; log access and deny by default | GlobusOS service interfaces | SPECIFIED |
| OPS-03 | Notifications and continuity | Notify on completion/errors/approval requests; resume sessions across supported devices | AI Studio + runtime | SPECIFIED |
| OPS-04 | Offline/local operation | Declare locally available features and reconcile sync conflicts safely | Local runtime + sync interfaces | SPECIFIED |
| SECURITY-01 | Identity and authorization | Separate user/service/model/agent identities; validate principal, capability, scope, audience and expiry | Existing identity/capability boundary | SPECIFIED |
| SECURITY-02 | Policy and approval | Explicit Allow/Deny/ApprovalRequired; deny on missing policy, expired/revoked grant or failed authentication | Existing policy/approval implementation | SPECIFIED |
| SECURITY-03 | Prompt injection and untrusted data | Treat web/file/tool/model content as data, never as authority; validate outputs before actions | Runtime + tool boundary | SPECIFIED |
| SECURITY-04 | Privacy and data loss prevention | Minimize data transfer, protect secrets, apply retention and prevent unauthorized exfiltration | Policy/runtime boundary | SPECIFIED |
| SECURITY-05 | Sandbox and emergency stop | Isolate execution; enforce quotas; stop privileged automation on policy/security incident | Runtime + OS interfaces | SPECIFIED |
| GOVERNANCE-01 | Audit and provenance | Correlate intent, plan, policy, approval, tool call, output and artifact | Existing audit/evidence interfaces | SPECIFIED |
| GOVERNANCE-02 | Observability and health | Record traces, latency, errors, resource usage and backend availability without leaking secrets | Runtime/operations | SPECIFIED |
| GOVERNANCE-03 | Evaluation and regression | Test correctness, groundedness, robustness, policy enforcement and regression | CI/evaluation | SPECIFIED |
| GOVERNANCE-04 | Supply chain and release | Record source/version, dependencies, signatures, reproducibility and rollback evidence | Release/security process | SPECIFIED |
| GOVERNANCE-05 | Exact-SHA verification | Tie source, tests, CI Run→Job→Step, exit/logs and residuals to exact commit | AURORA-001 evidence gate | SPECIFIED |
| DOMAIN-01 | Aurora domain intelligence | Dialogue, Quest, World, Character, Creature, Faction and Security AI capabilities | Existing domain services; no duplicate agent core | SPECIFIED |
| DOMAIN-02 | Ecosystem integrations | Authorized Genesis, A-TownChain and GlobusOS integration; no direct kernel/consensus authority | Canonical ecosystem interfaces | SPECIFIED |

### Cross-cutting interface requirements

1. **Model interface:** request/response schemas, streaming, cancellation, capability metadata, timeouts and typed errors.
2. **Agent execution:** `Intent → Plan → Capability Request → Policy → Approval when required → Execution → Result Validation → Audit`.
3. **Tool interface:** version, schema, side-effect declaration, capability requirements, budget, timeout, cancellation and failure semantics.
4. **Memory interface:** tenant/user/agent scope, provenance, retention, deletion semantics and versioned records; no parallel store.
5. **Artifact interface:** stable artifact ID, content type, provenance, version and validation result.
6. **Evidence interface:** repository, commit SHA, path/symbol, blob SHA, contract, test identifiers and exact workflow evidence.

### Mandatory acceptance gates

A requirement cannot advance beyond its evidence-supported status without: (a) one canonical owner; (b) source path and symbol; (c) input/output and error contract; (d) positive and negative tests; (e) security review where authority, execution, data access or secrets are involved; (f) CI for the exact commit; (g) integration evidence for cross-module behavior; and (h) E2E evidence before `E2E_VERIFIED`. A passing unit test or green unrelated workflow is insufficient.

### First audit targets (not implementation claims)

The existing inventory already records a placeholder inference response and open authorization/runtime/evidence gaps. The first implementation-planning pass must revalidate those findings at the current base SHA, then map every row above to source/test/evidence or record a justified residual. Do not infer that all rows are missing merely because they are newly enumerated here.
