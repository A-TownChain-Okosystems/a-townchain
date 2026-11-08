# P1 LKM Dependency API Repair

## Finding

`modules/atc-shivacore/kernel/src/lkm.rs` contains a placeholder `DependencyGraph::dependencies()` API whose `&[String]` return type is incompatible with the graph's `BTreeSet<String>` storage.

## Classification

- Class: P1
- Category: correctness / completeness
- Family: kernel / LKM / dependency-resolution
- Tags: `P1`, `stub`, `kernel`, `lkm`, `correctness`, `completeness`, `api`

## Required implementation

Use the owned deterministic representation already provided by `get_dependencies()` as the canonical API. The implementation must:

1. return `Vec<String>` (or an equivalent lifetime-safe owned representation);
2. preserve deterministic lexical ordering from `BTreeSet`;
3. return an empty vector for a missing module unless the surrounding API contract explicitly requires an error;
4. update every caller and test to the new signature;
5. remove the placeholder comments and every `unimplemented!()` associated with this API;
6. add regression tests for ordering, missing-module behavior, and duplicate dependency insertion;
7. run `cargo fmt --all -- --check`;
8. run `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
9. run `cargo test --workspace --all-features`;
10. re-read the modified source and close GlobusOS #18 only after CI verifies the change.

## Verification rule

This document is a repair specification, not evidence that the repair has already been applied. The finding remains OPEN until the source and CI prove otherwise.
