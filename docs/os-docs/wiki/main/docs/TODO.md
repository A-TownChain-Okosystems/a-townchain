# A-TownChain OS — Master TODO

> **Status:** AUDIT-DRIVEN / NOT COMPLETE
> **Last verified:** 2026-09-16
> **Source of truth:** active repositories + current CI evidence. Historical wiki claims are not implementation evidence.

## 🔴 P0/P1 — Active blockers

### P1 — GlobusOS / ShivaCore LKM
- [ ] Replace `DependencyGraph::dependencies()` placeholder in `globus-os/modules/atc-shivacore/kernel/src/lkm.rs`.
- [ ] Preserve deterministic dependency ordering.
- [ ] Add/retain regression coverage for dependency lookup.
- [ ] Run formatting, compile, unit tests and clippy before marking resolved.

**Classification:** P1 / Correctness / Completeness / Kernel-LKM-Dependency-Resolution
**Tags:** `P1`, `stub`, `kernel`, `lkm`, `dependency-graph`, `correctness`, `completeness`

### CI evidence integrity
- [ ] Verify the latest GlobusOS test-suite run after the evidence-gating fix.
- [ ] Do not mark PASS unless every required test job succeeds.
- [ ] Document any GitHub Actions log-access limitation separately from actual test failures.

## 🟠 Organization-wide audit backlog

For every active repository:

- [ ] Syntax and formatting audit
- [ ] Logic and functional audit
- [ ] Security/static-analysis audit
- [ ] Dependency and supply-chain audit
- [ ] Stub / TODO / FIXME / HACK / placeholder audit
- [ ] Duplicate and dead-code audit
- [ ] Architecture/interface consistency audit
- [ ] Cross-repository integration audit
- [ ] README / STATUS / CHANGELOG consistency
- [ ] Roadmap / TODO / Sprint consistency
- [ ] Wiki consistency and historical-content classification
- [ ] Error taxonomy: class → category → family → tag
- [ ] Contradiction detection and resolution
- [ ] Regression verification after every fix

## 🟡 Documentation synchronization

Historical documentation may contain completed claims that no longer describe the active architecture. Such material must remain explicitly archived/historical or be synchronized with current repository evidence.

### Required rule
**No documentation may claim production-ready, complete, green, or 100% status solely from an old snapshot.**

## 🧭 Resolution policy

Every finding follows:

`DISCOVER → CLASSIFY → DOCUMENT → FIX → TEST → RE-AUDIT → VERIFY → DOCUMENT STATE`

A finding is only **RESOLVED** when the implementation and its verification evidence agree.

## 📊 Current status

| Area | Status |
|---|---|
| Organization audit | IN PROGRESS |
| GlobusOS LKM stub | OPEN / P1 |
| CI evidence integrity fix | IMPLEMENTED; verification pending |
| Historical wiki cleanup | IN PROGRESS |
| Cross-repository consistency | IN PROGRESS |
| Security audit | IN PROGRESS |
| Functional audit | IN PROGRESS |
| Architecture audit | IN PROGRESS |

**Important:** This page intentionally replaces the previous `100% ABGESCHLOSSEN` claim because current repository evidence demonstrates open implementation work.