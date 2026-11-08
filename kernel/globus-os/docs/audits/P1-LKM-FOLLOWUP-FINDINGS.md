# P1 LKM follow-up findings

See the central engineering audit for the complete classification and verification policy.

## Open findings

- **P1 / dependency graph:** `DependencyGraph::topological_sort()` is inconsistent with the graph's `module -> dependency` edge semantics and the dependency-first contract expected by the existing test.
- **P1 / symbol validation:** the load path only rejects unresolved imports when `optional_deps` is non-empty; required unresolved imports must always be rejected.

These are static-audit findings. They remain open until implementation, regression tests, source re-read and CI verification are complete.
