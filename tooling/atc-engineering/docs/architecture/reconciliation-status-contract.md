# Reconciliation status contract

Normative machine-readable contract: [reconciliation-status-contract.v1.json](reconciliation-status-contract.v1.json).

The bounded consumer inventory is recorded in [reconciliation-status-consumer-audit.v1.json](reconciliation-status-consumer-audit.v1.json). It is explicitly marked partial; it is not organization-wide verification.

## Status meanings

- `DETECTED` means findings or candidate declarations were observed. It does not establish resolution, complete scope, or success.
- `RECONCILED` means the exact inventories, pinned commits, reviewed deltas, parser coverage, and required dispositions in the independent reconciliation report satisfy that report's contract. It is not a verification grant.
- `VERIFIED` is reserved for a separately scoped claim with explicit criteria and reproducible evidence bound to the exact checked 40-character commit SHA.

There is no automatic transition from `DETECTED` or `RECONCILED` to `VERIFIED`. Status renaming, string mapping, a green single workflow, or an empty/missing result is not evidence.

## Legacy report boundary

The legacy `reconciliation-report.json` is a distinct report type. Its `DETECTED` value must be preserved as produced and interpreted using that report's own scope and parser-coverage fields. A generated declaration count or successful generator execution does not prove authoritative parser coverage, source-to-catalog completeness, or organization-wide completeness. Null/unknown counts remain unknown, not zero.

The independent `independent-bidirectional-reconciliation.json` report has a separate contract. Its `RECONCILED` status is limited to the two identified inventories, their exact pinned commits, the reviewed expected-delta set, authoritative parser coverage, and the recorded dispositions. It cannot retroactively change or upgrade the legacy report.

## Fail-closed verification

A verification consumer must reject missing/unknown statuses, missing artifacts, invalid evidence references, incomplete criteria, invalid SHA formats, and any mismatch between the checked commit and evidence head. Evidence must identify a workflow run, job and step, plus an artifact identity/hash or reproducible log reference. An organization-wide claim requires explicit fleet scope and evidence for every in-scope repository; one successful workflow is insufficient.

This contract does not itself assert that the current repository fleet is `VERIFIED`. It defines what evidence would be required to make a narrowly scoped verification claim.
