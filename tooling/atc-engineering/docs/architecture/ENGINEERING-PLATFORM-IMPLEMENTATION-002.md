# ENGINEERING-PLATFORM-IMPLEMENTATION-002

**Status:** IMPLEMENTED — Foundation wave

## Scope

This wave adds three machine-readable control-plane foundations:

- `atc-findings` — Finding Registry;
- `atc-graph` — Dependency/Impact Graph;
- `atc-verification` — fail-closed Verification result model.

## Finding Registry

Schema: `ATC-FND-001` / `1.0.0`.

Canonical lifecycle:

```text
DETECTED -> CLASSIFIED -> PLANNED -> FIXED -> VERIFIED -> CLOSED
```

`BLOCKED` is terminal until an explicit engineering action transitions the finding from a permitted prior state. Duplicate finding IDs are rejected. Invalid state transitions are rejected.

Persistence target:

```text
.atc/findings/registry.json
```

The application writer refuses to overwrite an existing registry file.

## Impact Graph

`atc-graph` provides typed nodes and relations for repositories, files, modules, dependencies, standards, requirements, tests, workflows, schemas and documentation.

Relations include:

`DEPENDS_ON`, `IMPLEMENTS`, `VERIFIES`, `GOVERNS`, `DOCUMENTS`, `AFFECTS`, `REQUIRES`, `SUPERSEDES`.

Traversal is deterministic and fail-closed when an unknown root node or an edge to an unknown node is supplied.

## Verification

`atc-verification` defines:

```text
PASS
FAIL
BLOCKED
NOT_EXECUTED
NOT_APPLICABLE
```

A `PASS` requires a non-empty evidence reference. A verification report is ready only when it contains results and every result is `PASS`.

The verification layer does not grant release authority. Governance, SoD, human approval and evidence gates remain separate controls.

## Integration boundary

The new crates are registered as workspace members. They are deliberately independent of `atc-maintenance` to avoid dependency cycles.

The next implementation step is orchestration integration: findings emitted by `atc-maintenance` must be converted into registry records, impact updates must populate the graph, and executable verification results must be emitted as Evidence records.

## Verification status

Source files were written and registered through GitHub. This documentation does **not** claim a successful Cargo build, test, formatting, clippy or CI run. Runtime verification must produce explicit evidence before a release/readiness state can be asserted.
