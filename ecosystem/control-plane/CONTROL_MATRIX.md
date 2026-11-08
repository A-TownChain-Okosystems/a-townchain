# Control Matrix

| Capability | Observe | Operate | Approve | Release | Audit |
|---|---:|---:|---:|---:|---:|
| Metrics/status | YES | NO | NO | NO | YES |
| CI run/inspect | YES | YES | NO | NO | YES |
| Node restart/resync | YES | YES | NO | NO | YES |
| Emergency halt | YES | RESTRICTED | OWNER | NO | YES |
| Evidence generation | YES | YES | NO | NO | YES |
| Governance decision | YES | NO | YES | NO | YES |
| Production release | YES | NO | HUMAN | HUMAN | YES |

## Separation of Duties

- Code author is not the sole validator.
- Validator is not the release authority.
- Auditor is independent of the implementation action.
- Release actions require explicit approval and a complete evidence chain.
- Destructive operations are deny-by-default.

## GitHub integration

The panel may use GitHub Actions run/job data for CI state and evidence references. GitHub exposes workflow, run, job, artifact and dispatch operations through its Actions API; write operations require corresponding Actions permissions. citeturn0search0turn0search1
