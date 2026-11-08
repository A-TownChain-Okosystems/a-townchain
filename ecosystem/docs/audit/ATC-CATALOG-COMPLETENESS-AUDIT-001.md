# ATC-CATALOG-COMPLETENESS-AUDIT-001

Component and function catalog completeness audit, 2026-10-12.

| Field | Value |
|---|---|
| Document type | Audit record |
| Scope | `docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.{json,md}` |
| Pull request | #115 (`audit/public-symbol-scan-001-2026-10-11`) |
| Census date | 2026-10-12 |
| Maturity class | LEGACY/UNVERIFIED mapping only: no functional verification performed |
| Release decision | `BLOCKED` (unchanged) |
| Readiness | `RESIDUAL` (unchanged) |

## 1. Purpose

Close the source-binding gaps in the component/function catalog by reconciling it against an exact-HEAD organization source census, and extend the ATCLang component where the catalog was found to be incomplete. This record documents what was changed, how it was checked, what is still open, and what a reviewer must verify.

## 2. Evidence basis

| Item | Value |
|---|---|
| Active repositories scanned | 26 of 26 |
| Source files scanned | 4,917 |
| Candidate public symbols | 35,550 |
| Catalog components | 30 |
| Catalog functions | 71 (63 before the ATCLang pass) |
| Cross-component interfaces | 7 |
| atclang census HEAD | `21c595dc389b0c62e1531a8c5fbbcd2c68794efb` |
| globus-os census HEAD (ShivaCore bindings) | `c7337ef4fcff447ad0ee31c54e8adfee7f895bf6` |

Census data is a lexical symbol scan. It proves that a named symbol exists in a file at a given SHA. It does not prove behavior, test coverage or correctness.

## 3. Changes in this pull request

| Commit | Change |
|---|---|
| `4153381d5` | Bound the 58 previously unbound function contracts to census files at exact SHAs; refreshed 10 component and 8 repository-coverage census SHAs; moved ShivaCore bindings to the current globus-os HEAD; added a completeness supplement to the catalog document. |
| `f6ad61688` | ATCLang pass: extended `COMP-LANG` from 2 to 10 functions after enumerating the atclang source tree. |

### 3.1 ATCLang functions added

The previous entry covered only parsing and native directive validation. The repository implements a full compiler pipeline.

| ID | Function | Binding | Lexically verified |
|---|---|---|---|
| FN-LANG-003 | AST-to-bytecode lowering | `crates/atc-core/src/lower.rs` (`lower_program`) | yes |
| FN-LANG-004 | Canonical bytecode encoding and verification | `crates/atc-core/src/bytecode.rs` (`verify`) | yes |
| FN-LANG-005 | ATCA artifact envelope with SHA-256 binding | `crates/atc-core/src/artifact.rs` (`decode_and_verify`) | yes |
| FN-LANG-006 | ABI value and protocol type boundary | `crates/atc-core/src/value.rs` (`AbiType`), `ir.rs` | yes |
| FN-LANG-007 | Deterministic reference execution (reference only) | `crates/atc-core/src/vm.rs` (`execute_verified`) | yes |
| FN-LANG-008 | Differential parser and corpus conformance | `crates/atc-core/tests/differential.rs` | no, test file yields no scanned symbols |
| FN-LANG-009 | Stdlib reference conformance (reference only) | `crates/atc-core/tests/stdlib_reference_conformance.rs` (`encode_be`) | yes |
| FN-LANG-010 | Determinism gate tooling | `tools/determinism_check.py` (`main`) | yes |

## 4. Binding outcome

| Binding state | Count |
|---|---|
| Census-verified lexical match, functional verification pending | 53 |
| Bound file or symbol not found by the lexical scan (spec, doc, data, `.atc`, test files) | 13 |
| Explicit blocked record | 5 |
| **Total** | **71** |

The 5 blocked records are:

- 2 in the archived `atc-storage` repository (`BLOCKED_ARCHIVED_REFERENCE_ONLY`).
- 2 demo-fixture functions with no source symbols in the census (`NO_SOURCE_SYMBOLS_IN_CENSUS`).
- 1 `atc-toolchain` function with no identified compile or package surface (`SOURCE_SURFACE_NOT_IDENTIFIED`).

No function was invented from a repository name. Where no source surface exists, the record says so.

## 5. Guardrails preserved

1. `FN-LANG-007` and `FN-LANG-009` are **reference only** (ATC-RUNTIME-CLASSIFICATION-001). They are not a consensus or runtime trust anchor.
2. The canonical L1 execution engine remains `a-townchain/components/vm`. The atclang VM does not replace it.
3. ABI wire encoding remains deferred to the ATC-ABI-001 conformance gate (still draft).
4. No readiness state was raised. Every function stays `RESIDUAL`.
5. The hybrid consensus (PoH, PoS, PoW) is not described as final or frozen.

## 6. Validation performed

| Check | Result |
|---|---|
| `validate_component_function_catalog.py` | `PASS_STRUCTURAL_ONLY`, 0 errors, 71 functions |
| `validate_org_repository_integration.py` | `PASS_STRUCTURAL_ONLY`, 0 errors |
| `reconcile_component_function_catalog.py` against the census | 0 component findings, 0 SHA mismatches |
| `tools/tests` | 11 passed |
| PR CI on `f6ad61688` | 14 of 15 checks green |

`PASS_STRUCTURAL_ONLY` states that the registry is structurally consistent. It is not a certification of production readiness.

## 7. Open items and residuals

| ID | Item | Status | Owner action |
|---|---|---|---|
| R-1 | The CI census could not authenticate against the only private repository (`demo-repository`, Issue #112). **Mitigated 2026-10-12 by documented exclusion:** the census tool records missing private checkouts as `EXCLUDED_PRIVATE_NO_CI_ACCESS` (counted in `active_excluded_private`, excluded from `missing_active`); simulated CI census is `RESIDUAL`, exit 0, 25 scanned + 1 excluded; local scans with credentials still cover 26/26. One 2-line workflow assertion edit (replace the hard `active_scanned == 26` assert with `active_blocked == 0` and `active_scanned + active_excluded_private == active_expected`) is still required for `scanner-tests` green and is blocked for the agent by the missing `workflow` OAuth scope (GH013). | RESIDUAL (owner action pending) | Apply the 2-line workflow edit via GitHub UI, or alternatively make `demo-repository` public, or provision a private-access census credential |
| R-2 | The census scanner has no extractor for ATCLang (`.atc`) sources. `atc-contracts` was scanned for only 3 non-`.atc` files. | OPEN | Implement an ATCLang symbol extractor |
| R-3 | `IF-002` assigns the bytecode wire contract to the toolchain producer, while the canonical encoder and verifier are enumerated in atclang. | OPEN | Interface review to confirm the producing component |
| R-4 | `FN-LANG-008` has a bound test file that the lexical scan cannot verify. | OPEN | Extend the scanner to Rust test targets or bind a scanned symbol |
| R-5 | Functional verification of all 71 functions against `specs/` and tests. | NOT_PERFORMED | Separate verification passes per component |
| R-6 | Pull request review and approval. | OPEN | Reviewer other than the author |

## 8. Reviewer checklist

- [ ] Every `source_binding.exact_source_sha` is a full 40-character SHA and matches the census.
- [ ] No function carries a readiness above `RESIDUAL`.
- [ ] `FN-LANG-007` and `FN-LANG-009` are clearly marked reference only.
- [ ] The 5 blocked records state their reason and are not silently dropped.
- [ ] Counts in the catalog document (71 functions, 53 / 13 / 5) match the machine-readable catalog.
- [ ] R-1 is understood as a pre-existing blocker, not a regression of this pull request.

## 9. References

- Catalog: `docs/architecture/ATC-COMPONENT-FUNCTION-CATALOG-001.md` and `.json`
- Organization manifest: `docs/audit/ATC-ORG-REPOSITORY-MANIFEST-001.json`
- Census workflow: `.github/workflows/source-symbol-inventory.yml`
- Reconciliation tool: `tools/reconcile_component_function_catalog.py`
