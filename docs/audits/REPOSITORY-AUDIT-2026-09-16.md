# Repository Audit — a-townchain — 2026-09-16

## Scope
CI-independent, source/static audit of the complete repository surface. CI evidence is treated as evidence only where an existing evidence bundle identifies a concrete run and commit; no runtime result is invented.

Audit dimensions: standards enforcement, code/syntax/logic, security, architecture, integration/connectivity, completeness/stubs, duplicates, documentation, roadmap/TODO/wiki/sprints, classification/tags/families, contradictions, file formats/languages, and evidence integrity.

## Repository classification
- Class: CORE
- Domain/component: blockchain
- Maturity: R4
- Status: development
- Primary language: Rust; Python is used for the atcnet/test-support surface.
- Security criticality: critical
- API stability: experimental
- Protocol compatibility: required

The machine-readable `.atc/repository.yaml` declares ATC-STD-201 v1.0.1, CORE/R4/development and Rust as primary language.

## Confirmed findings

### F-20260916-ATC-001
- Priority: P1
- Class: Evidence / Traceability / Consistency
- Category: stale or internally inconsistent evidence
- Family: Repository Governance / Evidence SSOT / Release Gates
- Tags: `P1 evidence traceability stale-bound-commit test-evidence consistency`
- Evidence: `.atc/evidence/evidence.yaml` declares `bound_commit: 9a65378e800804d093ff1e20b41d194f3d4b56fe`, while its recorded test run is dated 2026-09-16 and the current repository contains later changes. The evidence file itself is therefore not a trustworthy snapshot of the current HEAD unless the bound commit and evidence bundle are regenerated together.
- Impact: a PASS result can be incorrectly interpreted as evidence for a different source state.
- Remediation: regenerate evidence from the exact audited commit; record source commit, workflow/run, test scope, and timestamp atomically. Reject evidence whose bound commit differs from the audited revision.
- Closure: re-read evidence and current HEAD, compare commit identity, then obtain a fresh test/evidence result.

### F-20260916-ATC-002
- Priority: P1
- Class: Completeness / Security / Integration
- Category: executable placeholder in ZKP boundary
- Family: Blockchain / ZKP / ATC-ZKP Boundary
- Tags: `P1 zkp groth16 placeholder python integration completeness fail-closed`
- Evidence: `modules/atc-blockchain/zkp/groth16.py` contains `NotImplementedError` for the ZKP layer and `get_zkp_layer()`. The repository architecture separately identifies `modules/atc-zkp` as the intended ZKP integration surface.
- Impact: a caller reaching the Python compatibility path can encounter a deliberate runtime failure instead of a canonical proof implementation; silent fallback must be prevented.
- Remediation: keep the Python file only as an explicit non-production compatibility/planning boundary, remove any production import path to it, fail closed with a typed error, and add an integration test proving production selects the canonical `atc-zkp` implementation.
- Closure: source re-read, negative/positive integration tests, and current evidence bundle.

### F-20260916-ATC-003
- Priority: P1
- Class: Governance / Enforcement
- Category: standards claims versus current evidence
- Family: Standards Compliance / Audit Evidence
- Tags: `P1 standards enforcement evidence R4 audit freshness compliance`
- Evidence: README presents a historical R4 audit score dated 2026-09-09 while the current evidence SSOT is dated 2026-09-16 but still bound to an older source commit and reports `security: not_audited` and `conformance: not_verified`.
- Impact: readers can confuse a historical score with current repository state.
- Remediation: label historical scores explicitly and make the current evidence SSOT the sole current readiness authority. Regenerate audit metadata after each material change.
- Closure: README/evidence consistency re-check against current HEAD.

## Static security checks
Searches on the current default branch found no `unimplemented!`, merge-conflict markers, `pull_request_target`, or mutable `actions/checkout@main/master` reference in indexed code. A `NotImplementedError` search did find the ZKP placeholder described above.

These checks do not prove absence of malware, supply-chain compromise, or runtime vulnerabilities. They are source/static controls only.

## Architecture and language assessment
The repository's stated boundary is coherent: ATCLang -> ATC-VM -> a-townchain orchestration, with `atc-algorithm` as canonical consensus and Rust for chain-bearing infrastructure. Rust is the appropriate canonical language for consensus-adjacent/node/state/network code. Python is reasonable for testnet tooling and non-consensus orchestration, but must not become a consensus or security primitive without a conformance specification and deterministic test vectors.

Recommended file formats remain: Rust `.rs`, Cargo TOML, Python `.py`, YAML for machine-readable governance/evidence, Markdown for human documentation. No blanket format migration is justified.

## Roadmap / TODO / wiki / sprint state
The repository explicitly remains development/prototype and lists conformance, security audit, reproducible build and release evidence as remaining gates. The audit therefore does not mark the repository complete. The ZKP placeholder and evidence freshness finding must be represented in the active work tracking system rather than hidden in historical documentation.

## Security / hack / virus evidence model
No source audit can prove that a repository is immune to every hack or virus. The defensible proof model is layered: immutable/pinned dependencies, secret scanning, dependency/SBOM review, reproducible builds, signed/provenance-backed artifacts, least-privilege CI permissions, deterministic tests, fuzzing/property tests for parsers and consensus boundaries, sandboxing, and independent runtime/security testing. Current static review provides partial evidence only and does not establish malware immunity.

## Completion state
`IN PROGRESS` — not release-ready. No finding is closed by documentation alone. Closure requires source correction where applicable, source re-read, appropriate tests, and fresh evidence bound to the exact audited commit.
