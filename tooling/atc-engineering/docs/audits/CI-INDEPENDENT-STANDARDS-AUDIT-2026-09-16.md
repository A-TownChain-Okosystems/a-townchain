# CI-independent Standards Enforcement Audit — 2026-09-16

## Status

**IN PROGRESS — source/static audit while GitHub Actions runtime evidence is unavailable or incomplete.**

This document does not claim successful CI execution. Runtime-only findings remain open until an executable test or GitHub Actions result is available.

## Scope

Organization: `A-TownChain-Okosystems`

Current inventory: 31 repositories.

Normative baseline examined:

- ATC-STD-000 v1.3.0 — governance hierarchy and change control
- ATC-STD-IMPLEMENTATION-001 — applicability/status/evidence matrix
- ATC-STD-201 v1.0.1 — repository structure and V-01…V-16
- ATC-STD-202 — naming/classification/ownership/lifecycle
- ATC-STD-203 — security/branching/commits/release/dependencies/gates
- repository-specific conditional standards from the current registry

## Audit contract

`DISCOVER → CLASSIFY → DOCUMENT → IMPLEMENT/FIX → TEST → RE-AUDIT → VERIFY → DOCUMENT STATE`

Every finding records severity/class, category, family and tags. A finding is not closed merely because documentation says it is fixed.

## Confirmed findings

### F-20260916-008 — Standards implementation evidence is stale

- **Class:** P1 — correctness/integration/governance evidence
- **Category:** consistency / evidence integrity
- **Family:** standards registry / implementation matrix / fleet governance
- **Tags:** P1, standards, implementation-matrix, stale-evidence, fleet, CI
- **Source:** `atc-standards/registry/standard-implementation.yaml`
- **Finding:** the current matrix declares 505 registry entries but contains enforcement evidence such as `Governance-CI 26/26` and a rollout of 26/26 repositories. The live organization inventory is now 31 repositories. This makes the historical evidence insufficient to prove current fleet-wide enforcement.
- **Impact:** a green historical count cannot be used as current compliance evidence for the five repositories added/changed since that evidence snapshot.
- **Remediation:** regenerate fleet evidence from the current 31-repository inventory, record per-repository results, and keep the implementation matrix status tied to dated evidence rather than inherited historical counts.
- **State:** OPEN until a fresh fleet audit is executed and verified.

### F-20260916-009 — Standards CI is not itself fleet-wide proof

- **Class:** P1 — governance enforcement gap
- **Category:** CI / enforcement coverage
- **Family:** governance CI / standards validator
- **Tags:** P1, CI, fleet, enforcement, registry-driven, coverage
- **Source:** `atc-standards/.github/workflows/naming-governance.yml`
- **Finding:** the workflow executes the standards validator and repository audit against the `atc-standards` checkout. It is evidence for the standards repository itself, not proof that every other organization repository is currently passing the same gates.
- **Impact:** organization-wide compliance must be established by fleet audit evidence, not inferred from the standards repository's own workflow.
- **Remediation:** use the central `atc-engineering` fleet audit as the organization-wide enforcement layer and make its output the dated evidence source for the implementation matrix.
- **State:** OPEN until a fresh fleet run is available.

### F-20260916-010 — Conflicted AGENT_MANIFEST was present in standards SSOT

- **Class:** P1 — correctness / governance
- **Category:** consistency / generated artifact integrity
- **Family:** agent governance / manifest / SSOT synchronization
- **Tags:** P1, manifest, merge-conflict, SSOT, governance
- **Source:** `atc-standards/AGENT_MANIFEST.md`
- **Finding:** the file contained unresolved Git merge markers (`<<<<<<<`, `=======`, `>>>>>>>`) and duplicated generated-state text. It also contained historical static repository counts and a stale `ATC-STD-000 v1.2.0` reference despite the current governance baseline being v1.3.0.
- **Fix:** replaced the conflicted manifest with an SSOT-bound manifest that treats the registry as authoritative and removes duplicated static repository-status tables.
- **Fix commit:** `24c25903b159d3db73746ea2dd29227a83889aa5`
- **Verification:** file was re-fetched after the commit; no merge markers remain in the fetched current content.
- **State:** FIXED at source level; runtime CI verification remains pending.

### F-20260916-011 — GlobusOS LKM dependency API placeholder

- **Class:** P1 — correctness/completeness
- **Category:** kernel / LKM / dependency resolution
- **Family:** ShivaCore kernel / loadable modules / dependency graph
- **Tags:** P1, kernel, LKM, stub, API, dependency-graph
- **Source:** `globus-os/modules/atc-shivacore/kernel/src/lkm.rs`
- **Finding:** `DependencyGraph::dependencies()` remains an active `unimplemented!()` placeholder. The existing graph representation uses `BTreeSet<String>`, while callers/tests use an owned deterministic representation.
- **State:** OPEN. Repair plan already exists; source is not marked fixed until tests and re-audit pass.

### F-20260916-012 — GlobusOS LKM topological-order semantics

- **Class:** P1 — correctness/integration
- **Category:** graph algorithm / load ordering
- **Family:** kernel / LKM / dependency graph / topological sort
- **Tags:** P1, kernel, LKM, topological-sort, deterministic-order, logic
- **Finding:** current edge semantics are `module -> dependency`, but the Kahn implementation increments the dependency node's indegree, yielding the wrong semantic direction for dependency-first loading. Existing regression expectations require dependency-first order.
- **State:** OPEN.

### F-20260916-013 — GlobusOS unresolved required imports may not fail closed

- **Class:** P1 — security/correctness
- **Category:** load validation / symbol resolution
- **Family:** kernel / LKM / symbol resolution / load validation
- **Tags:** P1, kernel, LKM, symbols, security, validation, fail-closed
- **Finding:** unresolved imports are calculated, but rejection is conditional on `optional_deps` being non-empty. Required unresolved imports therefore have a path to pass validation when the optional set is empty.
- **State:** OPEN. Required imports must always fail closed; optional semantics must be explicit and separately tested.

### F-20260916-014 — Non-canonical Python Groth16 placeholder

- **Class:** P2 — completeness/integration
- **Category:** placeholder / architecture boundary
- **Family:** blockchain / ZKP / ATC-ZKP integration
- **Tags:** P2, ZKP, Groth16, Python, placeholder, non-canonical
- **Source:** `a-townchain/modules/atc-blockchain/zkp/groth16.py`
- **Finding:** `ZKPLayer` and `get_zkp_layer()` raise `NotImplementedError`. The file explicitly describes the Python implementation as planned and identifies Rust `atc-zkp` as the active target.
- **State:** OPEN as a completeness/integration guard. This is not classified as a security vulnerability while the path remains explicitly non-canonical and unreachable from production execution.

## Security static checks

The organization-wide source search performed during this audit found no `pull_request_target`, `actions/checkout@main`, or `actions/checkout@master` matches in the previously audited search scope. Private-key and `ghp_` search hits previously reviewed were scanner definitions/documentation rather than confirmed exposed credentials.

The static result is **not** a substitute for secret scanning, dependency scanning, runtime tests, or GitHub Actions execution.

## Enforcement design decision

A new offline auditor was implemented in:

`atc-engineering/scripts/standards_enforcement_audit.py`

Commit: `dd8083e0ef725b3f900a73d555d3aee6e4f01586`

It checks, per checked-out repository:

- ATC-STD-201 maturity-dependent required metadata and documentation
- CI/test presence
- R3/R4 README compliance concepts and compliance badge
- agent manifest presence and registry-count synchronization
- credential/private-key patterns
- dangerous CI execution boundaries and mutable action references
- active implementation placeholders
- exact duplicate content groups
- machine-readable finding class/category/family/tags

The auditor is deliberately offline so it can continue to operate while GitHub Actions is unavailable.

## Best architectural solution

The organization should keep `atc-standards` as the normative SSOT and `atc-engineering` as the fleet enforcement/control plane. Duplicating enforcement logic independently in every repository creates drift. The central auditor should therefore validate each repository from the same registry-driven contract, while repository-local governance workflows remain defense-in-depth.

This separation is preferable because:

1. **Single normative source:** standards and applicability remain in the registry.
2. **Independent fleet evidence:** one repository cannot prove compliance of another repository.
3. **Fail-closed evidence:** stale historical counts cannot become current compliance claims.
4. **Defense in depth:** local CI catches changes close to their source; fleet audit detects cross-repository drift and missing gates.
5. **Auditable classification:** every finding has a stable severity/class/category/family/tag model.

## Completion gate

The organization audit remains **IN PROGRESS**. No repository is declared COMPLETE until its current source has been inspected, applicable standards have been checked, findings are fixed or explicitly classified as planned/non-canonical, documentation is synchronized, and relevant executable verification evidence exists.
