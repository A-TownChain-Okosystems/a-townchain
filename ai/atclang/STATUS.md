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
| Tests | 498 Tests im Monorepo-Wurzel-Workspace, 2160 Tests ueber alle sieben Workspaces gruen (2026-11-08) |
| Security | S4 / audit in progress |
| Conformance | Not verified for production |
| Release | development / NOT PRODUCTION_READY |
| Last audit | 2026-11-08 |

## Parser and semantics — state 2026-11-08

Verified by `cargo test --workspace` (498 tests) and `./tools/ecosystem.sh test`
(2160 tests, 0 failed):

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
- VM subset: `&&` and `||` lower with short-circuit semantics (the right operand
  is not evaluated when the left one already decides), `!` normalises to 1/0.
  Unsupported operators still fail closed in the front end (`%`, for example).
- Semantics stage 2 (`crates/atc-core/src/types.rs`): type and signature
  checking, deliberately conservative. It reports only when both sides carry a
  known type and contradict each other — `return` against the declared return
  type, `let x: T = ...` against the annotation, assignments to state fields,
  `require`/`if`/`while` conditions that are not `bool`, and `emit` arguments
  against the event declaration. Unknown names, cross-file calls and nominal
  type names stay `Unknown` and never produce a finding, so the check claims no
  type correctness it cannot prove. Counter-test: a deliberately broken contract
  yields 4 findings and exit code 1; the 47 real sources yield none.
- Semantics stage 3 (`crates/atc-core/src/project.rs`): cross-file resolution.
  `atc check <verzeichnis>` indexes every `.atc` below the path and checks what a
  single file cannot see — contract names that occur more than once (mirror
  copies), bases that no file declares, and type names that are neither builtin
  nor declared nor imported. The canonical chain types (`Address`, `Hash`,
  `Bytes`, `Signature`, `PublicKey`, `Timestamp`, `Amount`, `TokenId`) count as
  builtin; unknown names are reported once per file and name.
  Current result over `apps/atc-contracts`: 47 files, 29 contracts, 0 errors,
  18 warnings (17 mirror duplicates, 1 unknown type `bytes32` in the
  standards-refs mirror). The 38 `emit` calls that previously lacked an event
  declaration now have one: `TokenInitialized`, `Transfer`, `Approval`, `Mint`,
  `Burn`, `ATCoinInitialized` in `atc8300/atc8300.atc` and `contracts/token.atc`,
  `RevenueShareInitialized`, `RevenueRecorded`, `PayoutSent` in
  `contracts/revenue.atc`, and `RegistryInitialized`, `LicenseIssued`,
  `LicenseTransferred`, `LicenseSuspended` in the standards-refs registry.
- Contract ABI (`crates/atc-core/src/abi.rs`, roadmap G7 groundwork): `atc abi
  <datei|verzeichnis>` emits one canonical JSON line per contract with
  `abi_sha256` — state fields, events and function signatures, stable key order,
  symbols sorted, so declaration order and re-runs cannot change the hash.
  `atc abi --hash` prints only the hashes. The ABI describes the interface, not
  the execution: it is not bytecode and contract codegen (G11) stays open.
- Contract codegen (G11) S1: `atc_core::storage::StorageLayout` ordnet Zustand
  deterministisch ABI-Slots zu (32 Byte, `(Vertrag, Feld)` sortiert,
  Kollisionsschutz, `layout_version`, `layout_hash`); `atc storage <ziel>` zeigt
  Layouts. 34 Quellen mit Zustand, 1 vollstaendig S1-adressierbar. State-Instruktionen
  sind durch den Freeze von ATC-BC-001 blockiert und brauchen eine ATCB-v2-Version.
- Contract codegen (G11) scoping: `docs/CODEGEN_G11_SCOPE.md` inventories what is
  missing from the verified code (no state slots, no require/emit, no types
  beyond i64, no host calls; contract bodies are rejected by the lowering), lays
  out six independently verifiable slices S1-S6 and lists four owner decisions
  (state model, u256, slice order, artifact format). Still NOT PRODUCTION_READY.
  Regression binds the ABI to the contract registry: every non-archived entry
  must yield at least one contract with functions and a stable 64-hex hash, an
  archived entry must yield none.
- Import resolution (stage 3, part of `atc check <verzeichnis>`): every `use`
  path must name a symbol that is declared somewhere in the project (struct,
  enum, contract, top-level function) or a canonical builtin type/function —
  `Address`, `Hash`, `Bytes`, `Signature`, `PublicKey`, `Timestamp`, `Amount`,
  `TokenId` and the host functions `sha256`, `keccak256`, `msg_sender`, `now`,
  `ed25519_verify`, `secp256k1_verify` and friends. The root namespace must be
  `ATC`, and an import that shadows a declaration of the same file is reported.
  Imported symbols never confirm themselves: a typo in the path cannot satisfy
  its own resolution. Both real imports (`ATC::Types::Address`,
  `ATC::Crypto::sha256`) resolve, so the contract sources stay at 56 warnings.
- Contract registry gate A8 (SC-019): GATE PASS, 16 entries, 16 sources, 0 findings.

Open: mirror copies still declare the same contract names (17 warnings by
design, excluded from the registry); one unknown type name `bytes32` in the
standards-refs mirror; code generation for contracts. The four
standards-contracts ATC-13/15/16/20 are the generic token template with renamed
identifiers, and their specs state "nicht implementiert", so no
category-specific invariant can be tested yet; the ATCoin supply cap differs
between the canonical source (360M) and its mirror (21M). Roadmap G3-G10 remains open and the release status is unchanged.

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
