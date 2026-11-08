# A-TownChain OS — Roadmap

> **Status:** AUDIT-DRIVEN / NOT COMPLETE
> **Last verified:** 2026-09-16
> **Current architecture:** separate active repositories with `globus-os` owning the canonical ShivaCore kernel source and CI.

Historical feature milestones remain useful as history, but they are not evidence that the current repository fleet is production-ready or audit-complete.

---

## Phase A — Architecture and repository boundaries

- [x] Establish active repository fleet and integration graph
- [x] Establish `atc-standards` as standards source of truth
- [x] Establish `atc-engineering` as organization engineering/audit control plane
- [x] Move canonical ShivaCore kernel under `globus-os/modules/atc-shivacore/kernel/`
- [x] Make GlobusOS CI responsible for canonical kernel verification
- [ ] Complete repository-by-repository architecture/interface audit
- [ ] Complete duplicate/legacy source classification

## Phase B — Current P0/P1 implementation gates

### P1 — ShivaCore LKM dependency API
- [ ] Replace `DependencyGraph::dependencies()` placeholder with a lifetime-safe deterministic API.
- [ ] Keep `get_dependencies()` compatibility only where required; avoid duplicate semantic APIs.
- [ ] Add regression tests.
- [ ] Run format/build/test/clippy and re-audit.

### P1 — LKM symbol export/import semantics
- [ ] Stop `with_export()` from adding exports to `imports`.
- [ ] Stop `ModuleBuilder::export()` from adding exports to `imports`.
- [ ] Keep imports explicit through `import_symbol()`.
- [ ] Add export-only and import/refcount regression tests.
- [ ] Re-audit symbol registration, resolution and unload ordering.

Tracking: GlobusOS #18 and #19.

## Phase C — CI and evidence integrity

- [x] Separate GlobusOS test jobs from evidence publication.
- [x] Publish PASS evidence only after all required test jobs succeed.
- [ ] Verify the newest post-fix workflow run with usable job evidence.
- [ ] Complete organization fleet CI audit.
- [ ] Classify GitHub Actions access/logging limitations separately from code findings.

## Phase D — Organization-wide engineering audit

For every active repository:

- [ ] Syntax / formatting
- [ ] Type/build correctness
- [ ] Logic and functional behavior
- [ ] Security and unsafe-boundary review
- [ ] Dependency/supply-chain review
- [ ] Error handling and failure modes
- [ ] Stub / placeholder / TODO / FIXME / HACK review
- [ ] Duplicate/dead-code review
- [ ] Architecture and interface consistency
- [ ] Cross-repository connectivity
- [ ] README / STATUS / SECURITY / CHANGELOG consistency
- [ ] Roadmap / TODO / Sprint consistency
- [ ] Wiki/current-vs-historical classification
- [ ] Error classification: class / category / family / tag
- [ ] Contradiction detection
- [ ] Fix and regression verification

## Phase E — Release readiness

- [ ] All P0/P1 implementation findings resolved and verified
- [ ] Security audit evidence current
- [ ] Integration evidence current
- [ ] Reproducible builds/tests verified
- [ ] Release documentation synchronized
- [ ] Mainnet/testnet readiness independently reviewed

---

## Audit completion rule

A milestone is **not complete** merely because an old issue list is closed. It is complete only when the current implementation, documentation, architecture and verification evidence agree.

Required lifecycle:

`DISCOVER → CLASSIFY → DOCUMENT → FIX → TEST → RE-AUDIT → VERIFY → SYNCHRONIZE`

### Finding taxonomy

`Class (P0–P3) → Category → Family → Tags → Evidence → Status`

### Current status

| Area | Status |
|---|---|
| Historical v2/v3 feature milestones | Historical record |
| Current repository architecture | Active |
| Canonical ShivaCore source | GlobusOS |
| LKM dependency stub | **OPEN / P1** |
| LKM export/import semantic bug | **OPEN / P1** |
| CI evidence verification | **PENDING** |
| Organization-wide audit | **IN PROGRESS** |
| Release readiness | **NOT ESTABLISHED** |
