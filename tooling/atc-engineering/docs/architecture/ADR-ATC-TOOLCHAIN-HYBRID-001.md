# ADR-ATC-TOOLCHAIN-HYBRID-001: Hybrid Toolchain Architecture

- **Status:** Accepted as the selected architecture direction; repository ratification and implementation remain pending review and evidence.
- **Decision date:** 2026-10-12
- **Decision owner:** A-TownChain architecture/governance
- **Scope:** `A-TownChain-Okosystems/atc-toolchain`, with enforcement and evidence coordination in `atc-engineering`
- **Related repositories:** `atclang`, `a-townchain`, `atc-standards`

## 1. Context

The current `atc-toolchain/main` describes a Python stdlib-first R1 skeleton for governance and build orchestration. The `anchor/compiler-baseline` branch describes a Rust-first deterministic toolchain baseline, including bytecode verification and artifact tooling. These are different implementation directions and must not silently become competing sources of truth.

The selected architecture is **Option C — Hybrid with explicit boundaries**. This ADR records the target architecture. It does not claim that the components described here are already implemented, tested, verified, production-ready, or released.

## 2. Decision

1. Python is responsible for governance and build orchestration.
2. Rust is used for security-sensitive binary and artifact operations where its typed, memory-safe implementation provides a clear benefit.
3. Canonical language, bytecode semantics, and VM execution remain owned by their designated repositories. Toolchain modules integrate with these authorities; they do not replace them.
4. The existing R1 Python baseline remains the orchestration/governance starting point. Rust components from `anchor/compiler-baseline` may be adopted selectively only after source-SHA review, interface-contract review, tests, and provenance checks.
5. The architecture decision does not authorize a merge, release, production claim, or a migration that has not passed its required gates.

## 3. Single Sources of Truth and ownership

| Concern | Canonical owner | Toolchain responsibility |
|---|---|---|
| ATCLang language specification and canonical compiler responsibility | `atclang` | Invoke through a versioned compiler interface; do not establish a competing language or compiler authority |
| ATC-VM implementation and execution semantics | `a-townchain/components/vm` | Validate artifacts and integrate with the canonical VM; do not implement a second VM |
| Blockchain algorithm/consensus | `a-townchain/components/algorithm` | Orchestration and interface integration only; no competing consensus implementation |
| Normative standards and standard registry | `atc-standards` | Consume and enforce published standards; do not redefine normative requirements |
| Governance, CI policy, evidence and release-gate enforcement | `atc-engineering` | Own engineering controls and evidence policy, not product implementations |
| Cross-repository developer tooling | `atc-toolchain` | CLI, orchestration adapters, bytecode/artifact validation tools, packaging and evidence adapters |

A local cache, adapter, test fixture, or validator is not a new canonical authority. If sources conflict, fail closed and raise an explicit conflict for the canonical owner; do not silently choose one definition.

## 4. Runtime and language boundaries

The intended flow is:

```text
ATCLang source
  -> canonical compiler interface (owned by atclang)
  -> ATCB artifact
  -> Rust structural bytecode validator
  -> artifact integrity and provenance checks
  -> reproducible build/package result
  -> canonical ATC-VM (a-townchain/components/vm)
  -> evidence collection and policy evaluation (atc-engineering)
```

Python governance/orchestration coordinates stages and records their inputs, outputs, exit codes, logs, and artifact digests. It must not reinterpret bytecode or override validator failures. The validator must not execute programs as a substitute for the canonical VM.

## 5. Language and bytecode contracts

### 5.1 Compiler interface

The compiler boundary must be versioned and machine-checkable. Its contract must define:

- compiler identity and exact source/build revision;
- input encoding, source identity and input digest;
- target format/version and explicit configuration;
- output artifact path or transport, artifact digest, and structured diagnostics;
- deterministic error codes and non-zero exit behavior for invalid input;
- provenance fields sufficient to bind source, compiler, configuration and output.

The toolchain must call the canonical ATCLang compiler interface. Until its actual interface and implementation are inspected and pinned by exact SHA, this ADR does not prescribe an unverified CLI or ABI.

### 5.2 ATCB format

The proposed structural header contract is:

| Field | Encoding | Size |
|---|---|---:|
| Magic | ASCII `ATCB` | 4 bytes |
| Format version | unsigned 16-bit big-endian | 2 bytes |
| Instruction count | unsigned 32-bit big-endian | 4 bytes |
| Instruction stream | canonical format defined by the normative specification | variable |

The 10-byte header is a target contract, not independent proof that every repository currently implements it. The canonical format specification and the canonical VM/parser behavior must be inspected and reconciled before implementation is extended. The meaning and framing of instructions, opcode assignments, operand encoding, resource limits and semantic validity must not be invented or independently redefined by the toolchain.

`ATCB` is the selected magic in this target contract. An alternative such as `ATC1` must not be accepted or emitted as a silent fallback. Any existing conflicting definition is a release blocker until resolved by the canonical owners and documented with exact source SHAs.

### 5.3 Validator and artifact contracts

The Rust validator must:

- reject invalid magic, unsupported versions, truncated or malformed data, invalid instruction framing and unknown opcodes where defined by the canonical format;
- enforce the limits and validity rules published by the canonical format/VM specifications;
- return stable, machine-readable error codes and fail closed;
- never repair, reinterpret, or silently normalize invalid input;
- expose a versioned interface and tests for valid, invalid, boundary and adversarial inputs.

Artifact tooling must use specified canonical serialization, cryptographic digests, and explicit provenance. A digest proves identity of bytes, not semantic correctness or safety.

## 6. Responsibility split

### Python: governance and orchestration

- build-pipeline and process coordination;
- manifest, configuration and policy processing;
- CI-gate orchestration and result aggregation;
- evidence registry, audit reports and residual tracking;
- release workflow coordination, without self-approving a release;
- deterministic input/output manifests and interface invocation.

### Rust: security-sensitive tooling

- structural bytecode and binary-format validation;
- format/version and bounds checking;
- artifact integrity, canonical serialization and hashing;
- robust handling of malformed or adversarial input;
- unit, property, fuzz and regression tests appropriate to the threat model.

The language choice is a responsibility boundary, not permission to duplicate canonical business logic. Rust does not become a second VM; Python does not become a second compiler or bytecode interpreter.

## 7. Determinism and provenance

A reproducible build must bind at least:

- source repository and exact commit SHA;
- canonical compiler identity and exact revision;
- toolchain identity and exact revision;
- format/specification version;
- explicit build configuration and target;
- dependency/lockfile state where applicable;
- input and output artifact digests;
- relevant environment/tool versions and the commands executed.

For equivalent declared inputs and pinned tool versions/configuration, repeated builds must produce byte-identical artifacts or report an explicitly specified, reviewable source of nondeterminism. Timestamps, filesystem traversal order, locale, host paths and ambient environment must not affect output unless normalized or explicitly part of the contract.

## 8. Migration rules

1. Inspect `atc-toolchain/main` and `anchor/compiler-baseline` at exact commit SHAs before selecting files or behavior for migration.
2. Produce a file-level diff and classify each item as governance/orchestration, security-sensitive tooling, canonical implementation, duplicate, obsolete, or unresolved.
3. Preserve the R1 governance baseline and required CI gates. Do not replace Python orchestration wholesale merely because Rust components exist.
4. Port or adapt only components that have a clear ownership assignment, versioned interface, tests, dependency review and evidence.
5. Do not copy a VM, consensus implementation or competing language implementation into `atc-toolchain`.
6. Do not silently resolve ATCB or compiler-interface conflicts. Record each conflict, owner, exact source SHAs and disposition.
7. Every migration change must be submitted through a reviewable branch and pull request. No direct-to-main mutation, merge, auto-merge or release is authorized by this ADR.

## 9. Phased implementation and acceptance criteria

| Phase | Deliverable | Acceptance criteria |
|---|---|---|
| P0 — Architecture freeze | This ADR and ownership matrix | Option C recorded; ownership and non-goals explicit; no implementation status implied |
| P1 — Baseline reconciliation | SHA-pinned comparison of R1 `main` and `anchor/compiler-baseline` | Exact 40-character SHAs; file-level differences; dependency and capability inventory; conflicts and residuals recorded |
| P2 — Interface contracts | Versioned compiler, validator, artifact and evidence contracts | Machine-readable schemas; deterministic errors; contract and negative tests; canonical owners and source SHAs identified |
| P3 — Rust validation tools | Bytecode and artifact validation components | Valid/invalid/boundary/adversarial tests; fail-closed behavior; fuzz/property testing as applicable; no duplicate VM |
| P4 — Orchestration integration | Python orchestration invoking versioned Rust tools | End-to-end CI on a pinned source SHA; failures propagate; artifacts and provenance are retained; existing gates remain enabled |
| P5 — Verification | Exact-SHA evidence package | Run → Job → Step → exit code/log evidence; artifact hashes; all mandatory gates pass or residuals remain explicitly open |

A phase is not accepted because a document or file exists. Acceptance requires the stated evidence for the exact tested commit.

## 10. Evidence and status vocabulary

The project must distinguish at least:

- **SPECIFIED** — a normative contract has been reviewed and recorded;
- **IMPLEMENTED** — code exists at an identified commit;
- **TESTED** — named tests ran against an exact commit with recorded outcomes;
- **VERIFIED** — all required evidence and gates pass for that exact commit;
- **RESIDUAL** — an unresolved defect, conflict, missing proof, or unverified requirement remains;
- **BLOCKED** — progress is prevented by a stated dependency, conflict, or failed gate.

A green CI job is evidence for the checks it actually ran, not a blanket production certification. Missing or incomplete evidence is not a pass.

## 11. Security and governance invariants

- Fail closed on malformed bytecode, unsupported versions, missing provenance, failed integrity checks and unknown mandatory evidence.
- Do not weaken or remove formatting, test, static-analysis, security, dependency or evidence gates to obtain a green result.
- Keep architecture approval, code review, merge approval and release authorization separate.
- Keep cryptographic, protocol, VM and standards conflicts under review by their canonical owners.
- Treat any mismatch between the validator and canonical VM/specification as a blocker until resolved.
- No production or mainnet readiness claim follows from this ADR.

## 12. Consequences

**Positive:** Python's current R1 governance base can be retained; Rust can be introduced where binary safety benefits; canonical ownership remains explicit; migration and verification become evidence-driven.

**Costs and constraints:** Two language ecosystems must be tested and versioned; interfaces and error semantics need maintenance; source-SHA reconciliation and conflict resolution are required before implementation claims can be upgraded.

## 13. Open items / residuals

- [ ] Record exact commit SHAs for `atc-toolchain/main`, `anchor/compiler-baseline`, `atclang` and canonical ATC-VM source.
- [ ] Reconcile the ATCB header, instruction framing, limits and rejection behavior against canonical specifications and source.
- [ ] Inspect the actual compiler interface before freezing CLI/ABI details.
- [ ] Establish Rust/Python interface and schema versioning policy.
- [ ] Capture baseline CI and security evidence before migration.
- [ ] Obtain repository review and merge approval for this ADR.

Until these items have evidence, the architecture direction is selected, while implementation conformance remains **RESIDUAL / NOT VERIFIED**.

## 14. P1 SHA-pinned baseline audit — 2026-10-12

This section records the initial live source comparison. It is an inspection report, not a claim that the Rust baseline passes all required gates.

### 14.1 Exact source anchors

| Repository / ref | Commit SHA | Inspected file and blob SHA |
|---|---|---|
| `atc-toolchain/main` | `82fe4a238603476d7c6c19d5c3044c88b71c5f2d` | `README.md`: `5f7ca25c9c38f88016c57de005066ba70a5507e8` |
| `atc-toolchain/anchor/compiler-baseline` | `75c7c5652979b0f92b35a62e3c557b19eb81813a` | `README.md`: `178be726a88d9f09586cbf349436f66f3399f86d` |
| `atclang/main` | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` | `specs/bytecode/SPEC.md`: `0848bbf3c48237a61e0fc09d2e26877bccd1b40c`; `crates/atc-core/src/bytecode.rs`: `d63ba13087a71d6f06ec44c0dc2496644906eeea` |
| `a-townchain/main` | `c631249ed6ff450bf45338e78cb14e2bf5c65c74` | `components/vm/src/vm.rs`: `57a919d911ae7900f740bab739bb20fabf0d4313`; `components/vm/src/ops.rs`: `2b5efdff367b72e0025a6bfe7c05e89d23cf0a43` |
| Rust baseline workspace | same as `anchor/compiler-baseline` above | `Cargo.toml`: `86de03bdbbd52403834ee436a84bb23ec53a8940`; `crates/atc-verifier/src/lib.rs`: `9806ade8b2a4fa3eefda639daf1eadf4a1362afb`; `crates/atc-gas/src/lib.rs`: `63648535ef2744c96f2ed55cc82080e428d0fc8e`; `.github/workflows/rust-ci.yml`: `eb65a4e2ab0b7e21d53f71a5980ae4f6b5d8b366` |

The branch comparison reports `main` and `anchor/compiler-baseline` as **diverged**, with the Rust branch 4 commits ahead and 7 behind the main ref at comparison time. The comparison lists 22 changed files, including a new Cargo workspace, Rust CI, eight crate manifests/source trees, and architecture/roadmap documentation. Do not merge the branch wholesale.

### 14.2 Observed implementation boundaries

- **Python R1 on main:** README describes a stdlib-first governance/build-orchestration skeleton; CLI and several features are explicitly planned, and the CLI is not yet present as an implemented command surface.
- **Rust baseline:** workspace members are `atc-cli`, `atc-core`, `atc-ir`, `atc-gas`, `atc-capability`, `atc-verifier`, `atc-artifact`, and `atc-evidence`. The CLI source currently exposes only `version`, `toolchain info`, and `doctor`; README marks the complete compiler integration and several other features as roadmap work.
- **Normative ATCB authority:** `atclang/specs/bytecode/SPEC.md` identifies ATC-BC-001 v1.0.0-FROZEN as normative. It defines the 10-byte `ATCB` header, BE version/count, BE operands, and the 19-opcode catalog. This specification, not a toolchain README or a local duplicate, controls.
- **Canonical VM integration remains unproven:** inspected `a-townchain/components/vm/src/vm.rs` executes an internal `Vec<Op>` instruction model (for example `Push`, `Add`, `Load`, `Store`, `Halt`); `ops.rs` parses a textual `.ops` format. The inspected source paths do not demonstrate an ATCB-v1 byte-stream decoder or a conformance adapter from the normative ATCB opcode set into that VM. This is an integration-contract residual, not authorization to add a second VM to `atc-toolchain`.
- **Rust verifier scope:** `crates/atc-verifier/src/lib.rs` checks the header, version, opcode framing, truncation, instruction count/trailing bytes, and operation limit. The inspected implementation does not yet evidence the full ATCLang SPEC §5 checks such as stack-flow consistency, local/function references, jump-target validity, and return-path validation. Do not describe it as the complete canonical verifier.
- **Gas analyzer input contract:** `crates/atc-gas/src/lib.rs` starts decoding at byte offset zero as an opcode stream, while the verifier accepts a complete 10-byte-header ATCB container. The inspected code/tests do not establish a shared input contract or header-stripping adapter. Passing a complete ATCB container directly to this analyzer would interpret the first header byte as an opcode and fail. Define and test the interface before integration.
- **ATC-IR status:** the canonical ATCLang spec labels `specs/ir/SPEC.md` as DRAFT. The toolchain's local IR model must remain an integration proposal until a versioned canonical contract is approved.

### 14.3 CI and governance evidence

| Ref / commit | Check | Observed result |
|---|---|---|
| Rust baseline `75c7c5652979b0f92b35a62e3c557b19eb81813a` | Rust | **FAILURE** — [job 113297458914](https://github.com/A-TownChain-Okosystems/atc-toolchain/actions/runs/37773163974/job/113297458914) |
| Same Rust baseline SHA | audit | SUCCESS — [job 113297458372](https://github.com/A-TownChain-Okosystems/atc-toolchain/actions/runs/37773163883/job/113297458372) |
| Same Rust baseline SHA | smoke | SUCCESS — [job 113297458219](https://github.com/A-TownChain-Okosystems/atc-toolchain/actions/runs/37773163886/job/113297458219) |
| `main` `82fe4a238603476d7c6c19d5c3044c88b71c5f2d` | Python, smoke, audit, analysis checks returned by GitHub | SUCCESS for the listed checks on that SHA |

The branch metadata reports `protected: false` for `atc-toolchain/main` and `anchor/compiler-baseline`; the main branch protection response also reports enforcement off and no required status-check contexts. This is a governance risk to address through a separate approved repository-settings change. This ADR does not change repository settings.

### 14.4 P1 disposition

**P1 outcome: PARTIAL / RESIDUAL — NOT VERIFIED.**

The source comparison is SHA-pinned and confirms the intended hybrid split is feasible in principle, but the Rust branch is not eligible for wholesale adoption. Required follow-up:

1. Obtain the Rust failure's exact step/log and fix the cause on a review branch; rerun the full Rust gate against the resulting exact SHA.
2. Design and test an explicit adapter contract between ATCB v1 and the canonical VM's actual execution representation, or document the canonical VM's planned ATCB ingestion path with owner approval.
3. Bring the Rust verifier's claimed scope into line with ATC-BC-001 §5, or narrow its name/documentation to container framing only and make the full verification stage explicit.
4. Define whether gas analysis accepts a complete ATCB container or an instruction-only slice; enforce that choice in types, interface docs and tests.
5. Reconcile the local ATC-IR schema with the canonical DRAFT before treating it as a stable interface.
6. Address the unprotected branch / missing required-check enforcement via a separately reviewed governance action.

No implementation migration, merge, auto-merge, repository-setting change, or release is authorized by this audit.
