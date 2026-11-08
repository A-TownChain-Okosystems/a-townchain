# File Format & Implementation Language Audit — 2026-09-16

## Status

**ACTIVE / IN PROGRESS**

This audit is CI-independent. It adds a deterministic source-tree check for file-format and canonical implementation-language drift while GitHub Actions runtime evidence is unavailable or incomplete.

## Normative intent

The audit treats file format and implementation language as architectural properties, not cosmetic preferences. A finding is raised when:

1. a governance/workflow/lock file uses a format outside the permitted format family;
2. a repository contains implementation code outside its declared canonical language boundary;
3. the language may be justified as tooling, migration, compatibility, or legacy code — in which case the finding remains reviewable rather than being auto-deleted or auto-migrated.

## Enforcement

`tools`: `scripts/file_format_language_audit.py`

The central runner now invokes this check:

`scripts/run_ci_independent_audit.sh`

Commits:

- `d398b7381365976d55bf216a85bb6270dd6f4709` — initial checker
- `355548473c053292368473919e5323cdce7e8330` — integrated into central runner

## Classification

Findings are emitted as:

- `F-FORMAT` — file-format policy mismatch
- `F-LANGUAGE` — implementation-language boundary drift

Repository-level findings MUST subsequently receive the full ATC classification tuple:

`class / category / family / tags`

and a decision of `FIX`, `JUSTIFIED`, `LEGACY`, or `MIGRATION`.

## Design decision

Automatic language migration is intentionally prohibited by this checker. Changing Rust to Python, Python to Rust, or another language can alter ABI, determinism, security properties, performance, build topology, and integration contracts. Such changes require architecture review plus tests and evidence.

For example, the A-TownChain architecture is Rust-first for chain-carrying/runtime components, while Python can remain valid for tooling or migration boundaries. The checker therefore detects drift instead of silently rewriting it.

## Verification limitation

The GitHub connector can inspect and modify repository source, but this audit cycle does not claim a successful full 31-repository execution of the new checker until a workspace containing the fleet is executed and its output is captured. No runtime CI result is inferred from source inspection.
