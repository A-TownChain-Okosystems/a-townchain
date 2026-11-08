---
document_id: ATC-ENG-AUDIT-SNAPSHOT-20260916
title: Organization-wide Engineering Audit Snapshot
version: 1.3.1
status: active
audit_date: 2026-09-16
audit_time: 13:27:01 CEST
---

## Organization-wide Engineering Audit — 2026-09-16

## Scope

This snapshot covers the active A-TownChain-Okosystems repository fleet and records findings that can be verified through GitHub source search/read/write while CI execution/log retrieval is unavailable or incomplete.

## Verification policy

A source finding is not marked fixed until the changed source is re-read and the relevant CI/test evidence is available. Historical documentation is not accepted as current implementation evidence.

## Classification model

Every finding is classified by:

- **Class:** P0 / P1 / P2 / P3
- **Category:** correctness / security / consistency / completeness / architecture / CI / documentation / integration / maintainability
- **Family:** subsystem-specific family
- **Tags:** searchable normalized labels

## Confirmed findings

### F-20260916-001 — GlobusOS ShivaCore LKM dependency API stub

- Repository: `globus-os`
- Path: `modules/atc-shivacore/kernel/src/lkm.rs`
- Class: **P1**
- Category: **correctness / completeness**
- Family: **kernel / loadable-kernel-modules / dependency-resolution**
- Tags: `P1`, `stub`, `kernel`, `lkm`, `correctness`, `completeness`, `api`
- Status: **OPEN — implementation required**

`DependencyGraph::dependencies()` is an active `unimplemented!()` placeholder. The graph stores dependencies in `BTreeSet<String>`, while the declared borrowed slice API cannot safely expose that storage. The existing owned `get_dependencies()` representation is deterministic and is the canonical replacement direction.

Tracking issue: GlobusOS #18.

### F-20260916-002 — Historical repository/source-of-truth contradiction

- Repository: `atc-shivacore`
- Class: **P2**
- Category: **consistency / architecture / documentation**
- Family: **repository-boundary / source-of-truth**
- Tags: `P2`, `consistency`, `architecture`, `migration`, `documentation`
- Status: **DOCUMENTED**

The active kernel implementation was migrated to `globus-os/modules/atc-shivacore/kernel/`. `atc-shivacore` therefore cannot be described as the active kernel source. Its STATUS documentation identifies GlobusOS as canonical and as the CI owner.

### F-20260916-003 — Legacy TODO/wiki claims require historical classification

- Repository: `a-townchain-os-docs`
- Class: **P2**
- Category: **consistency / documentation**
- Family: **documentation-lifecycle / historical-archive**
- Tags: `P2`, `documentation`, `consistency`, `legacy`, `archive`
- Status: **PARTIALLY REMEDIATED**

Legacy TODO and Wiki datasets contain historical completion claims and old architecture terminology. The current master TODO page was corrected on 2026-09-16 to remove the stale `100% ABGESCHLOSSEN` claim and reference current audit evidence. Remaining legacy/archive pages require classification and synchronization.

### F-20260916-004 — LKM export/import semantic contradiction

- Repository: `globus-os`
- Path: `modules/atc-shivacore/kernel/src/lkm.rs`
- Class: **P1**
- Category: **logic / correctness / API semantics**
- Family: **kernel / loadable-kernel-modules / symbol-resolution / reference-accounting**
- Tags: `P1`, `kernel`, `lkm`, `symbols`, `imports`, `exports`, `refcount`, `logic`
- Status: **OPEN — implementation required**
- Tracking issue: **GlobusOS #19**

`ModuleDescriptor::with_export()` adds an exported symbol to `imports` as well as `exports`. Exporting a symbol and importing a symbol are distinct operations. The current implementation creates self-import metadata, can resolve a module's own export as an import, inflates imported-symbol statistics, and makes reference accounting asymmetric.

**Chosen remediation:** export builders modify only `exports`; imports remain explicit through an import API. This preserves provider/consumer separation and deterministic dependency/symbol graphs.

### F-20260916-005 — LKM topological-sort direction contradiction

- Repository: `globus-os`
- Path: `modules/atc-shivacore/kernel/src/lkm.rs`
- Class: **P1**
- Category: **correctness / logic / integration**
- Family: **kernel / LKM / dependency-resolution / graph-algorithm**
- Tags: `P1`, `kernel`, `lkm`, `dependency-graph`, `topological-sort`, `logic`
- Status: **OPEN — implementation required**

The graph represents `module → dependency`. The current Kahn implementation increments the dependency node's indegree, which produces dependent-before-dependency ordering. The separate `load_order()` implementation uses dependency-first DFS. One canonical dependency-first graph semantic must replace the contradiction, with deterministic ordering and regression tests.

### F-20260916-006 — Required symbol validation is not universally fail-closed

- Repository: `globus-os`
- Path: `modules/atc-shivacore/kernel/src/lkm.rs`
- Class: **P1**
- Category: **correctness / security / logic**
- Family: **kernel / LKM / symbol-resolution / load-validation**
- Tags: `P1`, `kernel`, `lkm`, `symbols`, `security`, `validation`, `logic`
- Status: **OPEN — implementation required**

The load path computes unresolved imports but only enters the rejection branch when `optional_deps` is non-empty. Required unresolved imports therefore lack a universal fail-closed gate. Required imports must always be rejected; optionality must be represented explicitly.

### F-20260916-007 — A-TownChain ZKP Python API is an active placeholder

- Repository: `a-townchain`
- Path: `modules/atc-blockchain/zkp/groth16.py`
- Class: **P2**
- Category: **completeness / integration / architecture**
- Family: **blockchain / ZKP / ATC-ZKP boundary**
- Tags: `P2`, `zkp`, `groth16`, `placeholder`, `python`, `integration`, `completeness`
- Status: **DOCUMENTED — planned/non-canonical path**

The file explicitly raises `NotImplementedError` for `ZKPLayer` and `get_zkp_layer()` and states that the active implementation target is the Rust `atc-zkp` module. This is not treated as a security vulnerability or as a failed implementation if the Python layer remains intentionally non-canonical, but it is an integration/completeness finding until the repository clearly prevents consumers from treating the placeholder as a usable ZKP API.

**Chosen remediation:** keep the Python file as an explicit compatibility/planning boundary only if it remains unreachable from production paths; document the canonical Rust target and add an integration guard/test so production code cannot silently select the placeholder.

## Static security checks

The following organization-wide source searches produced no matches in the indexed GitHub source at audit time:

- `pull_request_target`
- `actions/checkout@main`
- `actions/checkout@master`

Kernel `unsafe` usage remains subject to manual invariant review; presence of `unsafe` alone is not classified as a vulnerability. `panic!()` occurrences in examples/tests/kernel terminal handlers require contextual review rather than blanket classification.

## Documentation consistency checks

Current active documentation has been moved toward evidence-driven status. Historical pages may contain older sprint, roadmap, completion, or architecture claims and must remain explicitly historical/archive material or be synchronized with current source evidence. Searches still locate legacy pages with claims such as `100%`, `45+ completed`, or old sprint states; these are not accepted as current release evidence.

## CI evidence state

The organization fleet CI and GlobusOS CI execution/log endpoints were not providing usable step-level log output during this audit pass. Therefore:

- no CI failure is guessed or fabricated;
- source-level findings continue to be audited statically;
- a finding requiring runtime/CI proof remains OPEN until CI evidence is available;
- historical PASS claims are not reused as current evidence.

## Required completion gates

A repository can only be marked audit-complete when all applicable gates are satisfied:

1. source and architecture inspected;
2. syntax and type/build checks executed where CI is available;
3. logic/functionality reviewed;
4. security patterns reviewed;
5. stubs/placeholders reviewed and either implemented or explicitly accepted as non-production;
6. integration edges checked;
7. duplicate/legacy sources classified;
8. README/STATUS/ARCHITECTURE/SECURITY/CHANGELOG synchronized;
9. roadmap/TODO/sprint/wiki state synchronized;
10. changed source re-read after modification;
11. CI evidence confirms the fix where runtime verification is required.

## Current release posture

The fleet is **not globally audit-complete**. The canonical GlobusOS LKM dependency API blocker, LKM export/import semantic contradiction, LKM graph-ordering contradiction, and required-symbol validation issue remain open. The A-TownChain Python ZKP placeholder is explicitly non-canonical but requires integration guarding/documentation. CI execution evidence remains incomplete for the latest runs.
