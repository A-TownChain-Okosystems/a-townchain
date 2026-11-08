# P1 LKM repair plan — canonical GlobusOS kernel

## Required implementation order

1. Remove the impossible borrowed-slice `DependencyGraph::dependencies()` API and use one deterministic owned representation as the canonical query API.
2. Correct `topological_sort()` to honor `module -> dependency` semantics and emit dependencies before dependents.
3. Correct module-load symbol validation so unresolved required imports always fail closed, regardless of whether optional dependencies exist.
4. Remove the erroneous export-as-import behavior from both `ModuleDescriptor::with_export()` and `ModuleBuilder::export()`.
5. Add regression tests covering ordering, diamond graphs, missing required symbols, optional-dependency behavior, export/import separation, and reference accounting.
6. Run `cargo fmt`, workspace build/check, unit tests and clippy for the canonical GlobusOS workspace.
7. Re-run organization static audit and re-read all modified files.
8. Close the corresponding GitHub issues only after runtime evidence confirms the changes.

## Architectural rationale

This keeps the dependency graph deterministic, preserves the existing `BTreeSet` ordering, makes load-time validation fail closed, and separates provider (`exports`) from consumer (`imports`) semantics. These boundaries are important because LKM state, symbol ownership and unload safety are shared kernel invariants rather than independent convenience APIs.

## Status

OPEN. This file is a repair specification, not evidence that the repairs have already been applied.
