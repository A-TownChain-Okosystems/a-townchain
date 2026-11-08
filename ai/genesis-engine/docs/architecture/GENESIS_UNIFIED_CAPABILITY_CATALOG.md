# Genesis Engine — Unified Function and Capability Catalog

## Canonical index

`genesis-unified-capability-catalog.json` is the canonical cross-catalog index combining the existing function contracts and Unity-/Unreal-inspired capability targets. Original records are retained in each entry's `source_record`; no requirements are silently discarded.

## Consolidated inventory

| Source | Entries | Type |
|---|---:|---|
| `genesis-function-catalog.json` | 205 | Function contracts |
| `unreal-engine-capability-parity.json` | 295 | Capability requirements |
| `unity-capability-parity.json` | 313 | Capability requirements |
| **Unified total** | **813** | **205 contracts + 608 capability requirements** |

The merged index currently contains **53 domain labels** across the source catalogs. Similar labels are not automatically treated as identical domains: taxonomy reconciliation remains an explicit follow-up.

## Stable identity and provenance

Every merged item has a unique unified `id`, the source catalog and source ID, normalized type/domain/title/description/priority/status, source binding, acceptance evidence, dependencies, and the complete original `source_record`.

Potential semantic duplicates across the catalogs are preserved until an explicit reconciliation decision documents whether they are duplicates, overlapping requirements, or distinct contracts. Do not collapse entries based only on similar names.

## Status semantics

- `NOT_ASSESSED`: no verified source-tree mapping yet
- `SPECIFIED`: contract and acceptance criteria are precise
- `MAPPED`: mapped to repository/path/symbol and exact source SHA
- `IMPLEMENTED`: code exists, but that alone does not prove correctness
- `VERIFIED`: acceptance tests and reproducible evidence pass for the exact SHA
- `BLOCKED`: unresolved dependency or decision
- `EXCLUDED`: explicit, justified exclusion
- `EXTERNAL_DEPENDENCY`: external SDK, service, or licensed platform dependency

## Required next gates

1. Validate this instance against `genesis-unified-capability-catalog.schema.json`.
2. Validate the three source catalogs against their own schemas.
3. Reconcile each entry against the actual active source tree; add repository, path, symbol, exact SHA, tests and evidence.
4. Report candidate duplicates/conflicts without silently deleting requirements.
5. Add validator and reconciliation CI, then prove green results at the exact PR HEAD.
6. Only after these gates, split implementation work into small, dependency-ordered PRs.

## Boundary

This is a clean-room target inventory, not an implementation claim or exhaustive compatibility guarantee for every version/package/platform of Unity or Unreal Engine. Proprietary engine source and assets must not be copied. The PR remains Draft until the required validation and reconciliation evidence is complete.
