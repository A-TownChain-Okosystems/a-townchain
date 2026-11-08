# ShivaCore Audit Readiness — 2026-09-15

## Purpose

This document records the implementation/evidence boundary for the later production audit. It is not a production-readiness approval and does not replace CI, hardware validation, security review, or release authority.

## Evidence rules

- Current source and current CI evidence are authoritative.
- Historical test counts and archived documentation are informational only.
- A claimed feature is not considered production-ready until its implementation, tests, and required integration evidence are available.
- `unimplemented!()`, `todo!()`, placeholder panics, and equivalent fail-open stubs in active production paths are audit blockers.

## Known active blocker at audit preparation time

`modules/atc-shivacore/kernel/src/lkm.rs` contains `DependencyGraph::dependencies()` with an `unimplemented!()` placeholder. The graph stores dependencies in `BTreeSet<String>`, while the current API promises `&[String]`; these representations are not directly compatible. The required remediation is tracked in **Issue #19** and must choose an API that preserves deterministic ordering without returning a reference to temporary storage, update all callers/tests, and remove the placeholder.

## Audit checklist

- [ ] No active `unimplemented!()` / `todo!()` blockers
- [ ] Kernel and module-management tests pass on current main
- [ ] Determinism-sensitive dependency ordering is covered by tests
- [ ] LKM load/unload, cycle detection, conflicts, references, and failure paths are covered
- [ ] Security review completed for module loading and symbol resolution
- [ ] Hardware/platform evidence completed where applicable
- [ ] Fresh CI evidence recorded
- [ ] Release approval recorded separately from implementation status

## Scope separation

Implementation work may be completed before the later audit. Audit status must remain independently derived from current evidence; this file intentionally does not mark any unchecked gate as PASS.
