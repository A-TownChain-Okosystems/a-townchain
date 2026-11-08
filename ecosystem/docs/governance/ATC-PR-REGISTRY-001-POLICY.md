# ATC-PR-REGISTRY-001 — Pull Request Order and Merge Gates

**Status:** Proposed governance control; becomes binding only after review and merge.  
**Registry:** `docs/governance/ATC-PR-REGISTRY-001.json`  
**Snapshot:** 2026-10-11

## Purpose

Maintain a machine-readable, organization-wide queue so PRs are triaged, repaired, verified, and merged in dependency-safe order. This registry is a queue and evidence index; it is not permission to merge.

## Required execution order

1. **P0 — Blockers:** inspect potential consensus/determinism, transaction encoding/signing, compile failures, security boundaries, kernel/TCB, and critical runtime defects.
2. **P1 — Core and enforcement:** resolve core implementation/integration work and CI/evidence enforcement.
3. **P2 — Contracts and standards:** merge normative architecture/standard work only after the relevant owners and dependencies are established.
4. **P3 — Documentation:** reconcile status after implementation/evidence is confirmed; do not turn documentation into implementation proof.
5. **P4 — Dependency updates:** process after release-blocking fixes, unless a verified security advisory requires earlier handling.

Priority is an initial triage suggestion. A maintainer must validate it against current source, live check runs, dependencies, impact, and exact head SHA.

## Merge eligibility

A PR is eligible for merge only when all of the following are true:

- It is open, non-draft, and its target repository is active.
- Canonical repository ownership and scope are correct.
- Explicit dependencies are merged and the dependent branch is current.
- GitHub reports no unresolved merge conflict.
- All required checks pass on the exact current head SHA.
- Evidence includes Run-ID → Job → Step → exit code → log/artifact, plus the tested SHA and VerifiedTree where applicable.
- Required reviews, branch protections, and security checks pass.
- No overlapping/duplicate PR remains unresolved.
- The registry is refreshed immediately before merge.

**Never merge from a stale snapshot.** A changed head SHA invalidates previous exact-SHA evidence until rerun/reverified. Green CI from a different SHA is not proof.

## Dependency and duplicate handling

- Record dependencies as fully qualified `owner/repo#number`; never rely on bare PR numbers across repositories.
- Do not merge a dependent PR before its declared prerequisite.
- Compare overlapping fixes before selecting a canonical PR. Close superseded PRs only after their content is accounted for; do not silently discard unique changes.
- Cycles, unknown dependencies, stale evidence, or mergeability conflicts set the PR to `BLOCKED` and require manual resolution.

## Status vocabulary

- `OPEN_UNASSESSED`: discovered, not yet checked for exact-SHA evidence.
- `PENDING`: required checks/reviews are still running.
- `FAILED`: a required gate failed.
- `BLOCKED`: conflict, missing prerequisite, governance issue, or unavailable evidence.
- `VERIFIED`: all defined technical/evidence gates pass for the exact current head SHA.
- `RESIDUAL`: verified work has documented unresolved scope that prevents a stronger claim.

Do not conflate implementation, CI success, verification, and merge readiness.

## Refresh protocol

1. Discover all open PRs using partitioned queries if search result limits apply.
2. Reconcile inventory count against live API data.
3. Fetch each PR's current base/head SHA, draft/state, mergeability, labels, review state, dependencies, and required check runs.
4. Recompute priority and queue rank without deleting history.
5. Validate JSON against schema/required-field checks and verify stable IDs.
6. Merge only through the repository's approved process, one dependency-safe PR at a time.
7. Record the resulting merge commit, then refresh affected dependent PRs and rerun checks.

## Initial snapshot limitations

The initial registry contains PRs returned by four date-partitioned open-PR searches. Its completeness is intentionally not asserted until reconciled against live GitHub inventory. Head SHAs, checks, review approvals, and merge authorization are not inferred from search results and remain unset until collected.
