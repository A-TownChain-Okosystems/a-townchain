# P1 LKM follow-up findings — 2026-09-16

## Scope

Static review of the canonical GlobusOS ShivaCore LKM implementation while GitHub CI step evidence is unavailable/incomplete.

## F-20260916-005 — Topological sort direction is inconsistent with dependency semantics

- Repository: `globus-os`
- Path: `modules/atc-shivacore/kernel/src/lkm.rs`
- Class: **P1**
- Category: correctness / logic / integration
- Family: kernel / LKM / dependency-resolution / graph-algorithm
- Tags: `P1`, `kernel`, `lkm`, `dependency-graph`, `topological-sort`, `logic`
- Status: OPEN

`DependencyGraph` stores edges as `module -> dependency`. The Kahn implementation increments the dependency node's in-degree, which causes a normal topological sort to emit dependents before their dependencies. The existing regression test expects dependency-first order (`a < b < c` for `c -> b -> a`), so the implementation and test contract are inconsistent. `load_order()` separately performs dependency-first DFS and therefore represents the intended runtime semantics.

Required fix: implement topological sorting with dependency-first semantics (or explicitly invert the graph for Kahn), add deterministic regression coverage for chains and diamonds, and ensure the public contract matches `load_order()`.

## F-20260916-006 — Required unresolved symbol imports are not rejected when optional dependencies are empty

- Repository: `globus-os`
- Path: `modules/atc-shivacore/kernel/src/lkm.rs`
- Class: **P1**
- Category: correctness / security / logic
- Family: kernel / LKM / symbol-resolution / load-validation
- Tags: `P1`, `kernel`, `lkm`, `symbols`, `security`, `validation`, `logic`
- Status: OPEN

The load path computes unresolved imports, but only enters the rejection branch when `optional_deps` is non-empty. A module with required imports and no optional dependencies can therefore continue loading despite unresolved symbols. This violates the stated symbol-resolution invariant and can create a runtime invalid module state.

Required fix: always validate unresolved imports. Optional dependency semantics must be explicit and must not suppress validation of unrelated required symbol imports. Add regression tests for (1) required unresolved import with no optional dependencies, (2) unresolved optional dependency symbol if supported by the model, and (3) fully resolved imports.

## Verification requirement

Neither finding is marked fixed until source is changed, re-read, regression tests are added, and runtime CI evidence is available. Static analysis may establish the defect, but not a completed runtime verification.
