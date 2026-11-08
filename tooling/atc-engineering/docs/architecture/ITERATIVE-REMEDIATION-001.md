# ITERATIVE-REMEDIATION-001 — Closed-Loop Engineering Remediation

**Status:** IMPLEMENTED

## Objective

ATC Engineering shall not stop after detecting a finding. It shall re-audit after every safe remediation and continue until the repository is clean, no further safe progress is possible, or a hard iteration limit is reached.

## Executable Lifecycle

```text
DISCOVER
  -> DOCUMENT
  -> AUDIT
  -> CLASSIFY
  -> WRITE / IMPLEMENT SAFE CHANGE
  -> REMEDIATE
  -> UPDATE IMPACTED ARTIFACTS
  -> RE-AUDIT
       | clean -> VERIFY -> READY
       | changed -> loop
       | unchanged -> BLOCKED
```

The lifecycle is implemented by `engineer_until_clean()` in `atc-maintenance` and exposed through `atc-audit-cli --engineer`.

## Finding State

Findings are documented as they are detected and are only considered closed when a subsequent executable audit proves the exact finding absent:

`DETECTED -> CLASSIFIED -> REMEDIATED -> VERIFIED`

A finding that cannot be safely changed by the deterministic remediation layer remains open and therefore blocks readiness.

## Safety Model

The remediation engine is deliberately conservative. It may automatically apply only deterministic fixes whose correctness can be established from the local repository state.

Currently implemented safe remediations:

- create `docs/ENGINEERING_AUDIT.md` when the required file is missing;
- add canonical repository identity to `README.md` when the audit proves the identity is absent.

The engine does **not** silently rewrite:

- application or kernel source code;
- credentials or security-sensitive material;
- dependency declarations;
- shell download/execution pipelines;
- unresolved TODO/FIXME work;
- duplicate files;
- CI permissions;
- semantic configuration.

Those findings remain `BLOCKED` and require an explicit engineering change followed by another audit.

## Evidence

When evidence output is enabled, the orchestrator writes:

- `docs/engineering/ENGINEERING-LIFECYCLE.md`
- `docs/engineering/FINDINGS.md`
- `docs/engineering/ENGINEERING-RESULT.md` for a verified clean state

Every iteration records findings, and resolved findings receive a `VERIFIED` record only after re-audit.

## Deterministic Termination

The orchestrator has a hard maximum iteration limit and calculates a deterministic finding signature. If consecutive audit states do not change, it stops as `BLOCKED` instead of looping indefinitely.

A `READY` result is emitted only after a fresh executable audit reports zero findings.

## CLI

```bash
cargo run --release -p atc-audit-cli -- /path/to/repository --engineer
cargo run --release -p atc-audit-cli -- /path/to/repository --engineer --max-repair-iterations 12
cargo run --release -p atc-audit-cli -- /path/to/repository --repair --max-repair-iterations 12
```

`--engineer` is the full closed-loop lifecycle. `--repair` is the lower-level remediation primitive.

The command prints the phase, iteration count, applied repairs, findings and final readiness state. Non-clean or blocked results remain fail-closed.

## Scope Boundary

The current implementation provides deterministic documentation, auditing, classification, safe remediation, re-audit and evidence generation. It does **not** claim autonomous semantic source-code generation for arbitrary findings. Such code changes require an explicit implementation executor and must still pass the same verification loop before readiness can be granted.
