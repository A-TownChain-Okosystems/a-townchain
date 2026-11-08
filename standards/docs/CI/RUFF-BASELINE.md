# Ruff Baseline — atc-standards

Status: VERIFIED / exact-SHA evidence
Date: 2026-10-03

## Raw baseline before format

Exact source HEAD:
`f870085f04dacd21faebbb88ab722537fef88164`

Run: `37082968604`
Job: `111087287417`

Ruff 0.16.7 reported **210** findings:

| Rule | Count | Classification at raw baseline |
|---|---:|---|
| E701 | 107 | formatting/mechanical debt |
| E702 | 70 | formatting/mechanical debt |
| I001 | 26 | mechanical |
| E401 | 5 | mechanical |
| F401 | 1 | mechanical |
| W292 | 1 | mechanical |
| **Total** | **210** | |

## Format result

Dedicated formatting commit:
`94e948cce94c6a9e050735e082dbad1c78bede97`

Ruff 0.16.7 was run against the seven measured hotspots:

- `tests/test_dependency_guard.py`
- `tools/dependency_guard.py`
- `tools/dependency_sbom.py`
- `tools/gen_views/generate_views.py`
- `tools/osv_fetch.py`
- `tools/osv_snapshot.py`
- `tools/protocol/gen_protocol_registry.py`

The formatter changed all seven files. This was an explicit formatting operation, not Ruff lint `--fix`.

The generator was then synchronized with its canonical AI source, and the registry was regenerated deterministically. The exact generator/registry gate passed on run `37112482127`, job `111172937321`.

## Post-format mechanical baseline

After formatting, the temporary E701/E702 override became stale. The central gate reported:

`RUFF_OVERRIDE_STALE: base-policy failures contain none of the relaxed rules: E701, E702`

The override was therefore removed.

The remaining active Ruff debt was mechanical import hygiene. It was fixed in the dedicated mechanical cycle, with the final current head:

`e67b9e153f96befaca5bd12faec45181deb4c6d5`

Final exact-head Ruff verification:

- Run: `37112484839`
- Job: `111172945211`
- Ruff lint: **PASS**
- Ruff format: **PASS**
- Policy TOML syntax: **PASS**
- Ruff negative F401 test: **PASS**

The mechanical cycle removed the remaining E401/I001/F401 findings and the generator W292 issue.

## Final state

Current Ruff gate debt: **0**

The historical **210** is retained as the raw pre-format baseline. It is not the current gate debt.

The former 177 E701/E702 findings were eliminated by formatting; they are not suppressed by an active exception.

## Policy-bundle contract

The central reusable Ruff workflow validates all policy TOML with Python 3.11 `tomllib`
before loading configuration or evaluating overrides. Malformed policy is classified as
`POLICY-BUNDLE-SYNTAX`, not as caller-code Ruff debt.

The C-1 override mechanism remains available for future temporary exceptions, including
fail-closed allowlist, stale detection, and sunset enforcement. No atc-standards override
is currently active.

## Exact supporting gates

- Generator regression/determinism/registry consistency: Run `37112482127`, Job `111172937321` — **PASS**
- Independent Ruff format baseline job after final formatting: Run `37112482127`, Job `111172937244` — **PASS**
- Registry Consistency: Run `37112484429`, Job `111172943752` — **PASS**

## Fix-cycle conclusion

1. Format baseline: completed.
2. Generator source synchronization + registry regeneration: completed and deterministic.
3. Mechanical Ruff debt: completed.
4. E701/E702 override: removed because it became stale.
5. Current Ruff gate: **clean**.
