# Architecture Requirement Request (ARR) Lifecycle

**Document ID:** ARR-LIFECYCLE-001  
**Version:** 0.1.0  
**Status:** PROPOSED — review required  
**Normative intent:** Proposed governance process; not yet enforced by CI.

## 1. Purpose

Define one auditable lifecycle for architecture and software requirements affecting `a-townchain-ecosystem`. Governance owns normative policy and acceptance authority; atc-engineering owns technical validation and evidence automation; this repository owns ecosystem-level architecture, requirement records, SSOT references, compliance mapping, and residual tracking. Canonical implementations remain in their designated source repositories.

This document does not assert that ARR automation, CI enforcement, or organization-wide adoption is already implemented.

## 2. Authority and separation of duties

- **Governance:** approves policy, risk acceptance, normative changes, and final acceptance where delegated by applicable governance rules.
- **atc-engineering:** maintains validators and CI enforcement; binds validation to the exact source SHA; reports gate results and residuals. It must not self-approve changes where independent approval is required.
- **a-townchain-ecosystem:** records system-level requirements, architecture contracts, canonical repository/path references, dependency relationships, compliance mappings, and evidence pointers.
- **Canonical implementation repository:** owns implementation source, component-level tests, build outputs, release artifacts, and runtime/API/ABI contracts for its domain.
- **Independent reviewers / security reviewers:** review changes according to repository protections and risk class.

No ARR record grants authority to duplicate a canonical implementation or override an owning repository's source-of-truth boundary.

## 3. ARR record minimum

Each ARR record MUST contain:
- stable unique identifier, title, schema version, record version, status, owner, priority, and scope;
- requirement statement and measurable acceptance criteria;
- affected repositories/components and canonical repository/path for each implementation target;
- SSOT reference and expected source revision when known;
- dependencies and linked issues/PRs/standards/ADRs;
- security impact, threat assumptions, required positive and negative tests;
- evidence references including full 40-character commit SHA, workflow run/job/step, exit code, and inspectable logs when execution evidence exists;
- residuals, exclusions, risk owner, and reason for any missing evidence;
- reviewer/approver identity, decision, and timestamp when formally reviewed.

Unknown or unavailable information MUST be represented as an explicit residual or pending value allowed by the schema; it MUST NOT be guessed.

## 4. Lifecycle

Allowed lifecycle states:
1. `REGISTERED` — identity, owner, scope, and priority recorded.
2. `SPECIFIED` — requirement, canonical ownership, and acceptance criteria documented.
3. `REVIEWED` — architecture, SSOT, security, and implementation boundaries reviewed.
4. `IMPLEMENTED` — the owning implementation repository contains the intended change.
5. `VERIFIED` — required checks have evidence bound to the exact source SHA and satisfy the acceptance criteria.
6. `ACCEPTED` — authorized approval is recorded after verification and residual review.
7. `RESIDUAL` — a blocker, unmet criterion, risk, or evidence gap remains.

`RESIDUAL` is a visible disposition for the current evaluation, not proof of acceptance. A changed requirement or new evidence may trigger a new version/re-evaluation according to registry rules.

## 5. Fail-closed acceptance

An ARR MUST NOT be marked `VERIFIED` solely because a PR is merged, a workflow is green, or a summary says tests passed. Verification requires:
- exact 40-character source commit SHA;
- evidence linking the workflow run to that SHA;
- relevant job and step outcomes, exit codes, and inspectable logs;
- acceptance criteria mapped to specific evidence;
- no unreviewed exclusions or unresolved blocking residuals.

If evidence is missing, ambiguous, stale, or SHA-mismatched, status MUST remain unverified or `RESIDUAL`. A green check that does not test the stated criterion is insufficient.

## 6. SSOT and repository boundaries

- Normative standards remain owned by the designated standards SSOT and must be referenced by stable ID/version, not copied as a second normative source.
- Architecture records in this repository describe integration and ownership; they do not replace component-level specifications owned elsewhere.
- The ATC-VM canonical implementation remains `a-townchain/components/vm`; this ecosystem repository must not create a competing production VM.
- Registry data is authoritative only for the registry's declared domain. Generated views must be reproducible from the designated registry SSOT.
- Repository inventories and source bindings must include observed revision identifiers and explicit residuals where coverage is incomplete.

## 7. Change control and rollout

This proposal is not enforced until:
1. governance and engineering review this document and the schema;
2. the ARR registry is reconciled with existing standards/decision registries to prevent duplicate authority;
3. a validator and tests are added and demonstrated on the exact PR SHA;
4. required CI checks are configured and confirmed fail-closed;
5. ownership, migration of existing requirements, and exception handling are approved.

No merge automation, gate relaxation, production implementation, or status promotion is authorized by this proposal alone.

## 8. Acceptance checklist

- [ ] Governance authority and separation of duties approved.
- [ ] Registry ownership reconciled with ATC-STD-REGISTRY-001 and related SSOTs.
- [ ] Schema validation and negative tests implemented.
- [ ] Exact-SHA CI evidence demonstrated.
- [ ] Required branch-protection checks verified.
- [ ] Existing records migrated or explicitly listed as residuals.
- [ ] Independent review and approval recorded.
