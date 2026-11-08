# Ecosystem file/function inventory

Status: **IN PROGRESS**. This catalog is an evidence-bound inventory, not a claim that all repositories or files have been semantically audited.

## Files

- `repositories.json`: organization repository manifest; pending head/tree capture is explicit.
- `files.jsonl`: one JSON object per file record. Each record binds a repository commit, path, and blob SHA.
- `schema/file-inventory.schema.json`: JSON Schema for file records.
- `coverage.json`: measured scan coverage and outstanding limitations.
- `dependencies.json`: observed dependency/data-flow edges with evidence.

## Required invariants

1. A source-level claim must cite the exact repository, commit SHA, path, and blob SHA.
2. Do not infer functions, callers, dependencies, or test results from filenames alone.
3. A missing or unverified fact is `UNKNOWN` or `BLOCKED`, never silently treated as complete.
4. `IMPLEMENTED`, `TESTED`, `CI_VERIFIED`, `E2E_VERIFIED`, and `VERIFIED` are distinct states. Green CI alone does not imply `VERIFIED`.
5. Non-code files must receive a classification and a description of their declared purpose/effect; they must not be forced into a function list.
6. Generated, vendored, binary, and demo content must be identified explicitly. The demo repository may be inventoried but is excluded from the live drift gate.
7. This repository is the ecosystem governance/evidence control plane. It must not duplicate implementation source-of-truths or supersede `atc-standards` as the normative registry.

## Current measured baseline

- Organization repositories listed in initial discovery: 33.
- Recursive trees captured in initial baseline: 8.
- Tracked files counted in those 8 trees: 7,741.
- Seed file records written: 3, from a targeted Genesis Engine audit.
- Complete file inventory: **NO**.
- Complete symbol/caller/test coverage: **NO**.

See `coverage.json` for repository-level scope and limitations. Counts are intentionally not presented as semantic audit coverage.

## Next steps

1. Capture exact default-branch commit SHAs and recursive tree metadata for all repositories.
2. Populate one record per tracked file, then extract symbols by language.
3. Resolve callers and dependencies with evidence and mark unresolved edges.
4. Map tests and Actions run/job/step evidence to symbols where possible.
5. Validate every JSONL row against the schema and fail CI for missing provenance or unsupported status transitions.
6. Keep the PR in Draft until repository coverage, schema validation, and the first deterministic regeneration check are evidenced.
