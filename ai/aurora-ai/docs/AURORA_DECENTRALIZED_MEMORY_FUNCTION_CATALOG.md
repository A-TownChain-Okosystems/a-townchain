# Aurora Decentralized Memory Function Catalog

Status: specification / documented
Baseline: 87ed7bc87014708f3efad4c573871fe23f252b00

| Function ID | Function | Status |
|---|---|---|
| ATC-FUNC-AURORA-MEM-001 | Register an authorized memory node | SPECIFIED |
| ATC-FUNC-AURORA-MEM-002 | Authenticate a memory node | SPECIFIED |
| ATC-FUNC-AURORA-MEM-003 | Create canonical memory record | SPECIFIED |
| ATC-FUNC-AURORA-MEM-004 | Classify memory type | SPECIFIED |
| ATC-FUNC-AURORA-MEM-005 | Evaluate memory access policy | SPECIFIED |
| ATC-FUNC-AURORA-MEM-006 | Query authorized federated memory | SPECIFIED |
| ATC-FUNC-AURORA-MEM-007 | Persist local memory | SPECIFIED |
| ATC-FUNC-AURORA-MEM-008 | Synchronize memory between authorized nodes | SPECIFIED |
| ATC-FUNC-AURORA-MEM-009 | Preserve provenance and integrity metadata | SPECIFIED |
| ATC-FUNC-AURORA-MEM-010 | Resolve memory versions/conflicts deterministically | SPECIFIED |
| ATC-FUNC-AURORA-MEM-011 | Revoke memory federation access | SPECIFIED |
| ATC-FUNC-AURORA-MEM-012 | Apply retention/deletion policy | SPECIFIED |
| ATC-FUNC-AURORA-MEM-013 | Assemble authorized Aurora context | SPECIFIED |
| ATC-FUNC-AURORA-MEM-014 | Create optional ATC integrity/provenance proof | SPECIFIED |
| ATC-FUNC-AURORA-MEM-015 | Audit privileged memory operations | SPECIFIED |
| ATC-FUNC-AURORA-MEM-016 | Support offline synchronization/reconciliation | SPECIFIED |

## Function evidence contract

Each function must eventually map:

Requirement -> Specification -> Function ID -> Implementation -> Test -> Exact-SHA CI -> E2E -> Audit -> Release

No function becomes IMPLEMENTED merely because a similarly named file, struct or API exists.

## Security invariant

Every privileged operation follows:

request -> identity -> capability -> policy -> validation -> state change -> audit

## Non-claims

This catalog does not claim:
- every PC or smartphone is currently a memory node;
- cross-device synchronization is currently implemented;
- decentralized storage is currently implemented;
- federated learning is currently implemented;
- blockchain storage of memory is required;
- Aurora has implicit access to private device data;
- current code is production-ready.