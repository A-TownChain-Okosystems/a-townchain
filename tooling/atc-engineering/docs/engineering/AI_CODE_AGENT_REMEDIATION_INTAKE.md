# AI Code Agent — Remediation Intake

## Purpose

The scheduled intake gathers recent GitHub Actions runs that are failed, cancelled, or still in progress and records their run, job, step, branch, and head-SHA metadata. It is the evidence-collection stage of the remediation loop, not an autonomous code writer.

## Schedule

GitHub Actions runs at 00:17, 06:17, 12:17, and 18:17 UTC. These are four scheduled triggers per day, subject to GitHub Actions availability and schedule semantics; they are not a guarantee of four completed runs.

## Outputs

Each run publishes:

- `triage.json`: machine-readable findings, exact head SHA per workflow run, job IDs, step names/statuses, classifications, and source URLs.
- `evidence-record.json`: an `ATC-EVD-001` adapter record with SHA-256 integrity for `triage.json`. Its status is deliberately `UNKNOWN`; triage inventory alone cannot prove successful verification.
- `summary.md`: human-readable triage report attached to the workflow run summary and retained as a 90-day artifact.

The query currently inspects the first 100 workflow runs. It does not paginate; the report states this limitation. Missing/unreadable API data produces `BLOCKED`, not a clean result.

## Fail-closed boundaries

- This workflow is read-only: `actions: read`, `contents: read`.
- It does not edit source, push commits, open PRs, alter branch protection, change checks, or merge.
- The GitHub Actions Jobs API does not provide a trustworthy per-step process exit code. The report explicitly records `exit_code: null`; direct log review is still required.
- The adapter emits the existing `ATC-EVD-001` schema with status `UNKNOWN`, never `PASS`, until complete runtime evidence is reviewed.
- A green inventory or a matching SHA does not itself make a change `VERIFIED`.
- Findings must be classified (code, test, dependency, infrastructure, merge conflict, or evidence) before a repair is attempted.
- A repair attempt must produce a new diagnosis, preserve assertions and required gates, add a regression test where applicable, and be re-verified on the resulting exact head SHA.
- The policy limit is three attempts per finding; exhausted or uninspectable findings remain `BLOCKED`.

## Follow-up integration

This intake is intentionally separate from repair execution. Before any automatic repair is enabled, the workflow must be paired with a reviewed repair executor that can safely create an isolated branch, enforce an attempt ledger, validate regression tests, and request human review. No autonomous merge is permitted.
