---
document_id: ATC-DOC-LANG-001
title: ATCLang Repository Status
version: 1.3.0
status: active
owner: A-TownChain-Okosystems
updated: 2026-11-08
standard: ATC-STD-MD-001
---

## Status — ATCLang

| Property | Value |
|---|---|
| Repository | atclang |
| Version | 1.0.0 development |
| Production implementation | Rust canonical core — incomplete |
| Reference implementation | None — Python reference implementation removed; Rust-only canonical implementation |
| Tests | 469 Tests im Monorepo-Wurzel-Workspace, 2131 Tests ueber alle sieben Workspaces gruen (2026-11-08) |
| Security | S4 / audit in progress |
| Conformance | Not verified for production |
| Release | development / NOT PRODUCTION_READY |
| Last audit | 2026-11-08 |

## Parser and semantics — state 2026-11-08

Verified by `cargo test --workspace` (469 tests) and `./tools/ecosystem.sh test`
(2131 tests, 0 failed):

- Contract bodies are parsed into real syntax trees. Declarations: `state`
  (single and block form, with defaults), `event`, `fn`/`function`, `const`,
  `struct`, `enum`, `use`; decorators such as `@capability`; statements `emit`,
  `require`, assignments including `+=`/`-=`/`*=`; expressions with paths
  (`Hash::sha256`), field access, indexing, method calls, arrays, maps, tuples,
  struct literals and the logical operators `&&`/`||`/`!`. Language keywords are
  accepted as names (`capability: String`).
- 47 of 47 ATCLang sources in the monorepo parse; `crates/atc-core/tests/contract_sources.rs`
  keeps that as a regression.
- Semantics stage 1 (`atc check`): duplicate `state`/`fn`/`event` declarations and
  unknown `self` access are rejected (exit 1); `emit` without an event declaration
  or with a mismatching argument count is reported as a warning. `atc check
  --strict` turns warnings into errors.
- Contract registry gate A8 (SC-019): GATE PASS, 16 entries, 16 sources, 0 findings.

Open: 38 `emit` calls in existing contracts without an event declaration; type
checking and name resolution across file boundaries; code generation for
contracts; `&&`/`||` in the VM subset (lowering stays fail-closed). Roadmap
G3-G10 remains open and the release status is unchanged.

## Current verified facts

- The ATC standards profile is bound to the canonical atc-standards profile.
- The repository is Rust-only for the canonical compiler/VM implementation; the former Python reference implementation and its runtime path were removed.
- The Rust core contains the canonical lexer, parser, AST, lowering path, deterministic bytecode/VM path, and structural bytecode verification for the implemented subset.
- Control-flow support and the corresponding verifier checks are implemented for the current subset.
- Consensus-sensitive deterministic execution must not depend on wall-clock time, ambient randomness, network access, or other implicit host state.
- Evidence claims are not treated as PASS without current commit-bound CI evidence.

## Open blockers

1. Complete the canonical Rust frontend/parser and language conformance surface.
2. Define and independently verify canonical ATC-IR, bytecode, artifact, ABI, capability, resource/gas, and state-transition boundaries.
3. Complete ATVM integration and ecosystem/state-transition conformance.
4. Complete negative, malformed-input, fuzzing, and cross-component conformance coverage.
5. Restore current GitHub Actions runtime evidence and complete reproducible-build/provenance and independent security gates.
6. Regenerate and validate FILE_REGISTER against the Git tree.

No production gate is considered complete from documentation alone; implementation, tests and current evidence must converge before closure.
