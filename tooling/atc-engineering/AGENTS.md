# AGENTS.md

> **ATC Engineering Agent Governance**

This repository implements the engineering and governance control plane of the A-TownChain ecosystem.

All human and AI contributors MUST follow the repository governance model.

## Authority

The normative authority chain is:

```text
ATC Standards
      ↓
Repository Policy
      ↓
Engineering Gates
      ↓
Implementation
      ↓
Validation
      ↓
Evidence
      ↓
Approval
      ↓
Release
```text

"atc-standards" is the canonical source for normative standards.

## Core Principle

**«No Evidence, No Trust.»**

Declared state MUST NOT be treated as authoritative when verifiable evidence is required.

## Agent Lifecycle

Agents MUST follow:

```text
DISCOVER
→ UNDERSTAND
→ PLAN
→ IMPLEMENT
→ TEST
→ AUDIT
→ DOCUMENT
→ REVIEW
→ COMMIT
→ PR
→ HUMAN APPROVAL
→ MERGE
```text

An agent MUST NOT skip required governance stages.

## Scope

Every agent task MUST have an explicit scope.

The scope SHOULD identify:

- Repository
- Branch
- Files or subsystem
- Applicable standards
- Allowed operations
- Required validation
- Required evidence

Agents MUST NOT expand their scope implicitly.

## Separation of Duties

The following responsibilities MUST remain logically separated:

- CODER
- VALIDATOR
- AUDITOR
- RELEASE AUTHORITY
- HUMAN APPROVER

An AI agent MUST NOT independently authorize a production release.

## Standards

Before implementation, agents MUST determine applicable standards from the canonical standards registry.

Agents MUST NOT invent normative ATC standards.

If a required standard cannot be resolved, the operation MUST be blocked or explicitly escalated.

## Security

Agents MUST:

- avoid introducing secrets
- avoid weakening security controls without authorization
- preserve least privilege
- preserve auditability
- validate dependencies
- run applicable security checks

Security controls MUST NOT be bypassed merely to make CI pass.

## Changes

Changes MUST be minimal, scoped and reviewable.

Agents SHOULD avoid unrelated refactoring.

Generated files MUST be reproducible from their authoritative inputs whenever practical.

## Testing

Relevant tests MUST be executed before completion.

At minimum, Rust changes SHOULD run:

```text
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```text

Additional conformance, security and integration tests MUST be executed when affected.

## Evidence

Agents MUST preserve evidence required to substantiate their work.

Evidence SHOULD identify:

- actor
- action
- repository
- commit
- policy
- standards
- validation result
- timestamp
- artifact
- provenance

## Git

Agents MUST NOT rewrite protected history.

Force-push operations require explicit authorization.

Commits SHOULD be atomic and semantically scoped.

## Pull Requests

Changes SHOULD be submitted through pull requests.

A pull request MUST contain sufficient information for independent review.

Required checks MUST pass before merge.

## Release

Production release requires:

```text
valid implementation
+
passing validation
+
security validation
+
valid evidence
+
required review
+
human approval
```text

Agents MUST NOT treat a local success state as production authorization.

## Failure Handling

When a required gate fails:

```text
STOP
→ RECORD
→ DIAGNOSE
→ FIX
→ REVALIDATE
```text

Agents MUST NOT conceal failures or replace failed evidence with declarations.

## Documentation

Architectural changes MUST update the relevant documentation.

Standards-impacting changes MUST reference the affected standards.

## Final Rule

When uncertain about authorization, scope or governance:

**«STOP AND ESCALATE.»**

Do not infer authority.
