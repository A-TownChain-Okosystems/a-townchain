# System Function and Component Requirements

**Status:** REQUIREMENTS_BASELINE — not proof of implementation or verification  
**Scope:** ShivaCore Kernel, A-TownChain Blockchain, ATC-VM, ATC-Toolchain, ATC-Standards, ATC-Algorithm, ATC-Lang, GlobusOS, Aurora-AI, Genesis Engine.  
**Rule:** Existing-first, single source of truth (SSOT), no duplicate production implementations. Every inventory row must eventually be linked to an exact repository path, symbol/API, owner, tests, and exact-SHA evidence.

## Status vocabulary

- `REQUIRED`: required by the architecture; implementation has not been established by this document.
- `DETECTED`: source declaration/file was found; semantics and completeness are not yet validated.
- `IMPLEMENTED`: code exists and is connected to the intended runtime path.
- `VERIFIED`: tests and required gates passed on the exact recorded commit SHA, with logs/artifacts.
- `RESIDUAL`: known gap, failure, missing evidence, or unverified integration remains.
- `NOT_APPLICABLE`: reviewed and explicitly excluded with rationale.

Never infer IMPLEMENTED or VERIFIED from a README, symbol name, merged PR, generated inventory, or successful catalog-tool smoke test alone.

## Canonical ownership

| System | Canonical ownership | Must not become |
|---|---|---|
| ShivaCore Kernel | `globus-os/modules/atc-shivacore/kernel/` — capability microkernel / TCB | Network stack, AI runtime, blockchain consensus, contract VM |
| A-TownChain Blockchain | `a-townchain/` — ledger, transactions, block/state transition and node integration | Duplicate VM or duplicate consensus implementation elsewhere |
| ATC-VM | Production VM: `a-townchain/components/vm/`; `atc-vm` repository is specification/tooling/support unless the registry explicitly changes this | A second production VM |
| ATC-Algorithm | Production consensus/algorithm: `a-townchain/components/algorithm/`; `atc-algorithm` supports specification/governance as registered | Conflicting consensus implementation |
| ATC-Toolchain | Build, lint, test, compiler/linker orchestration, reproducible builds, vectors and evidence tooling | Alternative runtime authority |
| ATC-Standards | Normative specifications, versioning, conformance rules, registries and change governance | Unreviewed executable implementation of core components |
| ATC-Lang / ATCLang | Language syntax, grammar, AST, type/semantic rules, diagnostics and frontend | A separate VM/runtime |
| GlobusOS | OS integration, boot, userspace/services, drivers/HAL and system lifecycle; embeds canonical ShivaCore kernel | Duplicate kernel or unrestricted ambient authority |
| Aurora-AI | AI models, inference/training, agent orchestration and policy-mediated tool requests | Kernel TCB, consensus authority, canonical VM, direct privileged access |
| Genesis Engine | Game-building/editor/runtime, asset and scene pipelines, project packaging, deterministic integration where required | Kernel/blockchain/VM ownership or bypass of security boundaries |

If repository registry or current source contradicts this table, record the conflict as a RESIDUAL and resolve it through the registry/architecture governance process; do not silently move ownership.

## Required functional and component inventory

The lists below are architecture requirements to audit against source. They are not claims that every item exists today.

### 1. ShivaCore Kernel — capability microkernel / TCB

**Required components/functions**

- Boot and initialization state machine; architecture/HAL boundary; CPU feature discovery.
- Memory management: physical frame allocation, virtual address spaces/page tables, mapping/unmapping, permissions, isolation, rollback on failed mutation, object/mapping lifetime and reclamation.
- Capability system: capability/object identity, rights, derivation/restriction, transfer, revocation or documented non-revocation model, validation at every privileged operation.
- IPCBus: directed message passing with explicit endpoint capabilities, sender/receiver authorization, bounded queues, cancellation/timeouts and error semantics.
- Scheduler/process/thread lifecycle: creation, state transitions, context switching, priorities/budgets, blocking/wakeup, teardown and resource cleanup.
- Syscall/ABI boundary: stable versioned types, argument validation, error codes, user/kernel memory validation and compatibility policy.
- Interrupts, timers, exceptions/fault handling, panic/fail-stop behavior and diagnostic evidence.
- Secure boot chain and identity-provider interfaces (TPM/TEE where supported); key material must not leak into logs.
- Object ownership, synchronization, atomics/locks, deadlock/priority-inversion policy where applicable.
- Kernel logging/trace with bounded resource use and security redaction.
- Verification hooks: unit/integration tests, property/fuzz tests, invariants and target-specific boot evidence.

**Acceptance checks:** no ambient authority; every privileged operation capability-checked; untrusted userspace cannot mutate kernel state except through validated ABI; failed memory operations leave state consistent; teardown does not leave dangling mappings/capabilities; boot evidence records exact image, target and logs.

### 2. A-TownChain Blockchain — ledger and protocol

**Required components/functions**

- Canonical chain configuration: chain ID, supply/economic parameters, decimals, integer widths and byte order; versioned and sourced from normative standards.
- Transaction model: canonical serialization, domain separation, signature verification, nonce/replay protection, fee/weight limits and rejection reasons.
- Cryptography: approved key/signature algorithms, canonical encoding, low-S or other malleability policy, hash identifiers, key/address derivation and test vectors.
- Mempool/admission: validation, deduplication, replacement policy if supported, expiry, resource limits and DoS resistance.
- Block production/import: header/body validation, parent linkage, timestamps, size/weight limits, fork choice and deterministic rejection.
- State transition: balances/storage/account or UTXO model as specified, overflow-safe arithmetic, atomic commit/rollback and deterministic execution.
- Consensus integration: leader/validator/miner roles, safety/liveness assumptions, fork handling, finality/reorg semantics and explicit protocol-version rules.
- Persistent storage: block/state indexes, schema/version migrations, crash recovery, pruning/archive policy and corruption detection.
- P2P/node: peer identity, handshake, protocol negotiation, message validation, sync, rate limits, bans/peer scoring and network partition recovery.
- RPC/API and event subscriptions with authentication/authorization, schema versioning, limits and stable error responses.
- Wallet/SDK boundaries: signing stays in the designated wallet/key provider; node APIs do not expose private keys.
- Observability and operational functions: health/readiness, metrics, structured logs, snapshots, backup/restore and upgrade/rollback runbooks.

**Acceptance checks:** identical valid input yields identical state root on supported implementations; malformed and replayed transactions fail closed; supply/integer invariants hold; consensus safety assumptions and finality claims are documented rather than inferred; golden vectors pass on the exact SHA.

### 3. ATC-VM — deterministic execution

**Required components/functions**

- Bytecode format and version negotiation; magic/version/instruction-count encoding defined by the normative spec.
- Decoder/verifier: bounds checks, malformed/truncated input rejection, opcode validity, operand validation and control-flow checks as applicable.
- Execution engine: deterministic opcode semantics, defined integer/byte semantics, stable trap/error behavior and no host-dependent nondeterminism.
- Stack, memory, storage and call-frame management with explicit bounds.
- Gas/fuel accounting for every instruction and costly host call; overflow-safe metering and out-of-gas rollback.
- Contract call/return, arguments/results, storage read/write, event emission and contract-to-contract calls as specified.
- Host ABI/syscalls as a narrow, versioned allowlist; capability/policy checks before external effects.
- Atomic execution and rollback for failure/trap; state-root integration with the blockchain.
- Resource quotas, recursion/call-depth policy, denial-of-service protections and deterministic limits.
- Conformance suite, malformed-bytecode corpus, fuzzing, differential tests and golden vectors.

**Acceptance checks:** same bytecode + same initial state + same host inputs produces identical output/state/gas; no undeclared host access; all resource limits are enforced; rejected/trapped execution cannot partially commit state.

### 4. ATC-Toolchain — build and verification pipeline

**Required components/functions**

- Versioned toolchain manifest and pinned compiler/linker/formatter/linter/test tools.
- Build graph/dependency resolution with lockfiles, provenance and controlled network access.
- ATCLang frontend invocation, diagnostics, code/bytecode generation and ATC-VM target/version compatibility checks.
- Native Rust/C/C++ and target-specific build orchestration where applicable; cross-compilation profiles and feature flags.
- Format, lint, static analysis, unit/integration/system tests, fuzz/property tests and conformance suites.
- Golden-vector generation/verification with canonical source, schema, expected outputs and versioning.
- Reproducible build checks, artifact hashes, SBOM/dependency/license/security scans and provenance attestations.
- CI workflow validation, exact-SHA run/job/step/exit/log/artifact capture, failure classification and rerun tracking.
- Packaging, release signing, version compatibility checks, release manifests and rollback artifacts.
- Catalog generation must disclose unsupported languages, parse failures, exclusions and coverage; it must not claim completeness from partial parsing.

**Acceptance checks:** a clean pinned environment can reproduce required artifacts; every release artifact maps to source SHA and tool versions; failed gates remain visible; no gate is weakened to obtain green status.

### 5. ATC-Standards — normative specifications and governance

**Required components/functions**

- Standards registry with stable IDs, title, version, status, owner, scope, dependencies and supersession links.
- Normative versus informative text distinction; MUST/SHOULD/MAY terms and unambiguous wire/ABI definitions.
- Canonical encoding, serialization, integer widths/endianness, cryptographic suites and domain-separation registry.
- Transaction, block, consensus, VM bytecode/opcodes, host ABI, networking/RPC and error-code specifications.
- Versioning/deprecation and compatibility policy; migration and feature-negotiation rules.
- Conformance vectors, reference fixtures, schemas and testable acceptance criteria.
- Change proposal, review/approval, conflict detection, decision record and implementation mapping.
- Traceability from each normative requirement to source symbols, tests, vectors, release and exact-SHA evidence.
- Security/privacy threat model, trust assumptions, known limitations and normative failure behavior.

**Acceptance checks:** no competing normative definitions; every standard has an accountable owner and status; every normative requirement maps to conformance tests or an explicit rationale why not.

### 6. ATC-Algorithm — consensus and protocol algorithms

**Required components/functions**

- Explicit algorithm inventory and decision status for PoH/PoW/PoS/PoI or other candidates; do not describe proposals as finalized.
- Deterministic input/output contracts, canonical hash domains, seed/randomness source and anti-manipulation assumptions.
- Validator/miner eligibility, weight/stake/identity rules, leader selection, rewards/penalties and slashing only if normative.
- Fork choice, equivocation handling, reorg limits, finality and safety/liveness assumptions.
- Difficulty/epoch/round/slot adjustment if used; overflow, boundary and time-source handling.
- Sybil resistance, long-range/nothing-at-stake/MEV or grinding analyses where applicable.
- State-transition interface and VM/transaction verification integration without duplicate implementation.
- Simulation, adversarial/property tests, attack scenarios, reproducible benchmarks and golden vectors.
- Versioned algorithm parameters and on-chain/config activation policy.

**Acceptance checks:** protocol choices and unresolved alternatives are explicit; simulation assumptions are documented; deterministic vectors and adversarial tests pass; safety/liveness claims have evidence and stated limits.

### 7. ATC-Lang / ATCLang — language and frontend

**Required components/functions**

- Grammar, lexer/parser, AST, source spans and stable syntax-version policy.
- Name resolution, modules/imports, type checking, semantic analysis and diagnostic codes.
- Literals, numeric widths/overflow rules, functions, control flow, data structures and error handling as specified.
- Contract declarations/storage/events and ABI generation if part of the language scope.
- Compiler lowering and deterministic code/bytecode emission targeting the canonical ATC-VM format.
- Standard library/import resolution with explicit version and capability requirements.
- Formatter, linter, documentation generation, language server/tooling and editor diagnostics as in-scope components.
- Source maps/debug metadata, reproducible output and compiler version stamping.
- Negative tests, parser fuzzing, type/semantic tests and end-to-end source-to-bytecode golden vectors.

**Acceptance checks:** language spec, compiler behavior and VM target agree; invalid programs fail with stable diagnostics; compilation is reproducible under pinned versions; frontend does not silently define runtime behavior contrary to standards.

### 8. GlobusOS — operating system integration

**Required components/functions**

- Boot chain, loader, kernel handoff, configuration and recoverable boot failure reporting.
- Canonical ShivaCore kernel integration and stable userspace/kernel ABI.
- CPU/platform HAL, interrupt/timer, memory and device-driver interfaces.
- Process/service manager, lifecycle supervision, dependencies, health checks and restart/backoff policy.
- Filesystem/storage abstractions, permissions, mount/volume lifecycle and recovery.
- Network stack and P2P services in service space, not inside the kernel TCB.
- Identity, key-provider, secure boot and secrets interfaces with least privilege.
- IPCBus for authorized directed communication; EventBus for pub/sub notifications; do not conflate authority-bearing IPC with notifications.
- Service discovery, configuration, logging/metrics/tracing and diagnostics.
- Package/update system, signed updates, atomic upgrade/rollback and compatibility checks.
- Aurora runtime and blockchain/VM services as isolated services with explicit capability manifests.
- Hardware/VM targets, QEMU test profiles, architecture-specific CI and boot evidence.

**Acceptance checks:** services are isolated by capability; kernel remains minimal; networking, contracts and AI are service-space components; boot, upgrade, rollback and service recovery are tested on declared targets.

### 9. Aurora-AI — AI and agent runtime

**Required components/functions**

- Model registry, model/version provenance, tokenizer/processor compatibility and model artifact integrity.
- Data ingestion, cleaning, deduplication, labeling, privacy/licensing checks and dataset versioning.
- Training/fine-tuning pipelines, evaluation harnesses, checkpointing, reproducibility metadata and rollback.
- Inference/runtime abstraction for CPU/GPU/NPU with resource quotas, cancellation and deterministic configuration where required.
- Agent/task orchestration, planning, context management, memory lifecycle and tool selection.
- Policy layer: intent/proposal → policy evaluation → capability check → required approval → tool invocation → service boundary → authoritative runtime.
- Tool schemas, argument validation, output validation, timeouts, rate limits and sandboxing.
- Human approval/escalation for high-impact operations; least privilege and explicit user/org authorization.
- Prompt/model/version audit metadata, redacted logs, safety evaluations, abuse monitoring and incident handling.
- Retrieval/indexing and knowledge-source provenance if used; freshness and citation policy.
- Evaluation for quality, robustness, security, prompt injection, data exfiltration and tool misuse.
- Deployment, model serving, health monitoring, drift detection, canary rollout and rollback.

**Acceptance checks:** Aurora is not the ShivaCore TCB, blockchain consensus authority, canonical VM, or a source of privileged authority; tools require policy and capability checks; training and runtime evidence distinguish dataset/model versions from code SHA; secrets and private user data are not exposed in logs.

### 10. Genesis Engine — game-building software

**Required components/functions**

- Project/template lifecycle, project settings, versioning and migration.
- Editor shell, scene/world hierarchy, entities/components, transforms, prefab/instancing and undo/redo.
- Asset import pipeline, metadata, dependency graph, content-addressed cache and reimport/error reporting.
- Resource/material/shader management, rendering abstraction, camera, lighting and post-processing as supported targets require.
- Input mapping, UI/layout, animation, audio, physics/collision and gameplay scripting interfaces.
- Scene serialization, runtime loading, save/load, localization and accessibility support as in-scope.
- Build/export pipeline, target profiles, packaging, asset validation and release manifests.
- Plugin/extension API with versioning, sandboxing and permissions.
- Debugger, profiler, logs, crash reports, performance budgets and automated project tests.
- Collaboration/source-control hooks and deterministic asset/build hashes where required.
- Optional A-TownChain integration only through documented SDK/service boundaries; optional Aurora features only through policy-controlled APIs.

**Acceptance checks:** a clean sample project can import assets, edit a scene, run, test and package for a declared target; broken assets and unsupported targets produce actionable diagnostics; optional blockchain/AI integrations cannot bypass OS or service security boundaries.

## Cross-system contracts (mandatory)

1. **Identity and authority:** every process/service/tool has an explicit identity and least-privilege capability manifest.
2. **Interfaces:** each public API/ABI/wire format has an owner, version, compatibility policy, error model and conformance tests.
3. **State and transactions:** authoritative state has one owner; mutations are atomic or have a documented recovery model.
4. **Determinism:** consensus and VM-critical paths exclude wall-clock, host randomness, unordered iteration and floating-point variation unless explicitly specified.
5. **Failure/recovery:** timeout, cancellation, partial failure, retry idempotency, rollback and corruption handling are specified.
6. **Security:** threat model, trust boundaries, input validation, secrets handling, audit logging and abuse limits.
7. **Observability:** health/readiness, bounded metrics/logs/traces and redaction.
8. **Supply chain:** pinned dependencies, SBOM, provenance, signed release artifacts and reproducible builds where required.
9. **Compatibility:** API/schema/bytecode/protocol versions, deprecation and migration paths.
10. **Testing:** unit, integration, conformance, fuzz/property, adversarial and end-to-end tests selected by risk.
11. **Traceability:** requirement ID → component/API/symbol → test/vector → workflow run/job/step → exit status/log/artifact → exact source SHA.
12. **Coverage accounting:** count scanned, parsed, skipped, excluded, generated, vendored, unsupported and failed files; each exclusion has a reason. No silent omissions.

## Machine-readable inventory minimum schema

Each component/function record must include:

- `requirement_id` (stable identifier), `system`, `component`, `function` or `interface`
- `canonical_repository`, `path`, `symbol` / API / ABI / protocol identifier
- `purpose`, `inputs`, `outputs`, `errors`, `authority`, `dependencies`
- `status`, `evidence_level`, `source_sha`, `test_refs`, `workflow_run_job_step`, `artifact_refs`
- `standard_refs`, `security_boundary`, `known_gaps`, `owner`, `last_reviewed`

Use stable requirement IDs such as `SC-KRN-###`, `ATC-CHAIN-###`, `ATC-VM-###`, `ATC-TC-###`, `ATC-STD-###`, `ATC-ALG-###`, `ATCLANG-###`, `GOS-###`, `AUR-###`, `GEN-###`. IDs must not be recycled after removal; supersede records instead.

## Execution order

1. Freeze the repository registry and canonical ownership map at an exact SHA.
2. Inventory all active repositories and record archived repositories separately; do not silently omit them.
3. Scan source/config/spec/test/workflow files; record exclusions, unsupported languages and parser failures.
4. Extract declarations, public APIs, ABI/wire types, feature flags, error codes and test references.
5. Reconcile discovered symbols with the requirement baseline and existing docs; flag orphan code, undocumented APIs, duplicate ownership and missing implementations.
6. Create per-system JSON catalogs plus a cross-system index and human-readable Markdown pages.
7. Validate JSON instances against the schema; test referential integrity and stable IDs.
8. Run component tests and relevant CI; record exact SHA and Run → Job → Step → exit/log/artifact.
9. Publish gap matrix with severity, owner, dependency, acceptance criteria and residual status.
10. Repeat on every relevant change; never auto-merge based on generated documentation.

## Definition of done

The system-wide catalog is complete only when all in-scope repositories and relevant files are accounted for; every required component/function is either mapped to implementation and exact-SHA verification evidence or explicitly recorded as a gap/residual; all public interfaces have owners and version rules; standards and tests are linked; exclusions and unsupported files are disclosed; and a coverage report plus gap matrix is published. The generator's own successful CI is necessary but not sufficient.
