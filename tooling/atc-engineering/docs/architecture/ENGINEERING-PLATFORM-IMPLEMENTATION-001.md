# ENGINEERING-PLATFORM-IMPLEMENTATION-001

**Status:** IMPLEMENTED — Phase 1 extension

## Purpose

This document defines the implementation boundary for the next ATC Engineering platform layer. The platform is built around the principle **No Evidence, No Trust** and remains fail-closed.

## Implemented in this wave

### 1. Evidence Plane foundation

New crate:

`crates/atc-evidence`

The crate provides:

- canonical schema identifier `ATC-EVD-001`;
- schema version `1.0.0`;
- `EvidenceStatus::{Pass, Fail, Unknown, Missing}`;
- explicit trust evaluation where only `PASS` is trustworthy;
- `EvidenceRecord` with repository, commit, producer, standards, artifacts, provenance and integrity fields;
- fail-closed validation of required evidence fields;
- deterministic JSON serialization without an external serialization dependency;
- immutable-once-written behavior: an existing evidence file cannot be overwritten through `write_to()`.

### 2. Workspace integration

`atc-evidence` is registered as a first-class Rust workspace member and depends only on `atc-core`.

Dependency direction:

```text
atc-core
   ↑
atc-evidence
```

The evidence crate does not depend on the maintenance engine, preventing a dependency cycle.

## Evidence contract

An evidence record requires:

```text
Evidence ID
Schema + version
Evidence type
Repository
Commit
Producer
Status
Integrity algorithm
Integrity digest
```

Optional but supported:

```text
Standards
Artifacts
Provenance
```

A record without repository, commit, producer or integrity digest is rejected.

## Persistence contract

Evidence is stored below:

```text
.atc/evidence/EVD-YYYY-NNNNNN.json
```

The writer refuses to replace an existing record. This is intentional: historical evidence must be append-only at the application layer.

## Closed-loop engineering target

The existing maintenance orchestrator already implements:

```text
DISCOVER
→ DOCUMENT
→ AUDIT
→ CLASSIFY
→ REMEDIATE
→ IMPACT-UPDATE
→ RE-AUDIT
→ VERIFY
→ COMPLETE / BLOCKED
```

The next integration step is to make every engineering run emit an `atc-evidence` record rather than relying only on Markdown evidence.

## Finding registry target

The canonical finding lifecycle remains:

```text
DETECTED
→ CLASSIFIED
→ PLANNED
→ FIXED
→ VERIFIED
→ CLOSED
```

`BLOCKED` is terminal until an explicit engineering action changes the state.

The registry must preserve:

- finding ID;
- iteration;
- repository;
- artifact/path;
- family/category;
- maintenance class;
- severity/risk/priority;
- root cause;
- affected artifacts;
- implementation plan;
- changed files;
- verification evidence;
- commit/reference;
- state transitions.

## Impact graph target

The current maintenance implementation records impact classes. The full graph implementation remains a subsequent wave.

Required graph nodes:

- repository;
- file;
- module/crate;
- dependency;
- standard;
- requirement;
- test;
- workflow;
- schema;
- documentation.

Required relations:

`DEPENDS_ON`, `IMPLEMENTS`, `VERIFIES`, `GOVERNS`, `DOCUMENTS`, `AFFECTS`, `REQUIRES`, `SUPERSEDES`.

## Implementation executor boundary

The platform must not pretend to be an unrestricted autonomous source-code generator. Safe deterministic remediation remains automatic; semantic, security-critical, kernel, dependency and governance changes require an explicit implementation executor and the normal verification loop.

Target interface:

```rust
trait ImplementationExecutor {
    fn plan(&self, finding: &Finding) -> Result<ImplementationPlan, Error>;
    fn apply(&self, plan: &ImplementationPlan) -> Result<ImplementationResult, Error>;
}
```

Every applied plan must be followed by audit and verification. No executor may directly grant `READY`.

## Readiness rule

```text
No findings
+ verification evidence
+ valid governance gates
+ valid SoD
+ required human approval
= READY
```

Anything else remains `NOT_READY` or `BLOCKED`.

## Verification boundary

This implementation was written through GitHub source operations. A successful compiler, test or CI result is **not inferred** from source inspection. Runtime verification remains a separate evidence-producing step.
