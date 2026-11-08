# Organisation-wide Function and Component Catalog

**Status:** specification / implementation pending  
**Owner:** `atc-engineering` (governance and evidence only)  
**Scope:** all repositories registered under `A-TownChain-Okosystems`  
**Tracking:** [Issue #13](https://github.com/A-TownChain-Okosystems/atc-engineering/issues/13)

## 1. Purpose

Produce a complete, reproducible inventory of functions, methods, APIs, components,
modules, types, interfaces, event handlers, commands, workflows, tests, and relevant
configuration across the organisation. Source-derived facts must be distinguishable
from inferred descriptions and from planned-but-unimplemented functionality.

This document defines the contract. It does **not** claim that the inventory has
already been extracted, that every symbol is documented, or that any repository has
passed verification.

## 2. Coverage boundaries

The inventory must include:

- Every active repository on its recorded default branch.
- Archived repositories in a separate legacy section, never silently mixed with active
  production ownership.
- First-party source, tests, API schemas, build and CI workflows, configuration,
  documentation, generated code, vendored code, and duplicated copies, each classified.
- Public and private symbols where parsers can identify them; CLI, RPC, HTTP, IPC,
  event, message, plugin, and FFI entry points where applicable.
- Cross-repository callers, callees, dependencies, interface contracts, tests, and
  canonical ownership where evidence is available.

A repository, directory, language, file, or symbol that cannot be analysed must have an
explicit machine-readable exclusion or error record. There must be no silent omissions.

## 3. Single source of truth

The catalog is a discovery and documentation index, not a second implementation.
Canonical ownership must follow the current architecture registry and repository
governance. In particular, do not relocate canonical implementations into an
integration or documentation repository merely to simplify the catalog.

Conflicts between code, specifications, README files, Wiki pages, API descriptions,
and prior inventories are recorded as findings with links and exact commit evidence.
Do not silently select a preferred interpretation.

## 4. Required deliverables

1. Versioned JSON Schema for catalog records.
2. Machine-readable organisation index and per-repository symbol manifests.
3. Human-readable organisation index and per-repository Markdown catalog.
4. Coverage report with repository, default branch, observed commit SHA, parser version,
   files analysed, files skipped, parse failures, generated/vendored files, and counts
   by language and symbol kind.
5. Documentation-gap report covering missing descriptions, stale links, interface
   mismatches, ownership conflicts, undocumented public APIs, and status contradictions.
6. CI validation and fixture tests for each supported language and each fallback path.
7. Evidence record binding every generated artifact to its source commit and run.

## 5. Minimum symbol record

Each symbol record must contain the following fields where applicable:

| Field | Meaning |
| --- | --- |
| `schema_version` | Catalog schema version |
| `symbol_id` | Stable, deterministic symbol identifier |
| `repository` | Fully qualified repository name |
| `path` | Repository-relative source path |
| `language` | Detected source language or format |
| `symbol_kind` | Function, method, type, trait, module, endpoint, job, test, etc. |
| `qualified_name` | Fully qualified symbol name |
| `signature` | Source-derived signature, if available |
| `visibility` | Public, private, protected, exported, or unknown |
| `summary` | Description with provenance marked |
| `parameters` | Names, types, defaults, and constraints when available |
| `return_type` | Declared or inferred type, clearly distinguished |
| `errors` | Error types, codes, and failure conditions |
| `side_effects` | I/O, state changes, network, storage, or authority effects |
| `security_boundary` | Relevant capability, trust, privilege, or isolation boundary |
| `dependencies` | Direct dependencies, when extractable |
| `callers` / `callees` | Call graph links, with completeness limitations |
| `tests` | Related test paths and test identifiers |
| `doc_url_or_path` | Canonical documentation location |
| `source_start_line` / `source_end_line` | Source range |
| `source_blob_sha` | Exact source blob SHA |
| `observed_commit_sha` | Exact repository commit used for extraction |
| `status` | Documented lifecycle state |
| `canonical_owner` | Repository/module owning the implementation |
| `evidence` | Links to source, tests, CI, and relevant review threads |
| `limitations` | Parser gaps, uncertain types, or unavailable call graph data |

Optional fields must be omitted when unavailable or set to an explicit unknown state;
they must never be fabricated.

## 6. Suggested JSON structure

The final schema is versioned and validated separately. A record may follow this shape:

```json
{
  "schema_version": "1.0.0",
  "symbol_id": "sha256:<deterministic-key>",
  "repository": "OWNER/REPOSITORY",
  "path": "src/example.rs",
  "language": "rust",
  "symbol_kind": "function",
  "qualified_name": "module::function_name",
  "signature": "pub fn function_name(input: Input) -> Result<Output, Error>",
  "visibility": "public",
  "summary": {
    "text": "Description derived from source and reviewed documentation.",
    "provenance": "source+reviewed-doc"
  },
  "source_start_line": 10,
  "source_end_line": 18,
  "source_blob_sha": "<blob-sha>",
  "observed_commit_sha": "<commit-sha>",
  "status": "DETECTED",
  "canonical_owner": "OWNER/REPOSITORY",
  "evidence": [],
  "limitations": []
}
```

This is illustrative only; placeholder values are not valid evidence and must not appear
in a verified production catalog.

## 7. Lifecycle and evidence rules

Use distinct states:

- `DETECTED`: discovered by an inventory/parser run.
- `DOCUMENTED`: description and references have been written.
- `IMPLEMENTED`: a code or documentation change exists at a known commit.
- `VERIFIED`: required checks passed on the exact recorded SHA with Run → Job → Step
  → exit code → log/artifact evidence.
- `STALE`: the source branch advanced after the inventory was generated.
- `BLOCKED`: access, parser, dependency, or policy prevents completion.
- `RESIDUAL`: a known gap remains and has an owner and next action.
- `FAILED`: an extraction or validation run failed.

A green run on another SHA, a PR merge reference, or the mere presence of a file is not
proof that a catalog is verified. If the default branch moves during a run, mark the
result stale and rerun.

## 8. Security and change controls

- Never record secrets, private keys, tokens, credentials, or sensitive runtime values.
- Never change production semantics while generating documentation.
- Never weaken required checks or delete tests to make catalog validation pass.
- Do not auto-merge catalog PRs.
- Create changes on isolated branches and review them through normal repository gates.
- Treat suspected dead code and unused symbols as candidates for review, not automatic
  deletion instructions.
- Security, cryptography, consensus, transaction encoding, VM execution, kernel/TCB,
  and capability-boundary descriptions require source-linked evidence and human review.

## 9. Completion criteria

The catalog is complete only when:

- All registered repositories have a row, including inaccessible and archived repos.
- Every in-scope file is analysed or has a reason-coded exclusion.
- Every supported language has parser fixtures and validation coverage.
- Every catalog and report records the exact source SHA.
- Schema, determinism, idempotency, cross-reference, and consistency checks pass.
- All unresolved gaps are reported with severity, owner, evidence, and next action.
- The final summary distinguishes documented coverage from verified implementation.

Until these criteria are demonstrated by CI artifacts, organisation-wide completion
must remain **BLOCKED / NOT VERIFIED**.
