# ATC-PRIVACY-POLICY-001 validator

This is **catalog/evidence tooling**, not a privacy implementation and not a GDPR compliance checker.

## Input contract

The validator consumes a JSON export with:
- `schema_id`, `version`, `status`, and a `requirements` array;
- stable `ATC-PRIV-NNN` requirement identifiers;
- normative level (`MUST`, `SHOULD`, `MAY`);
- separate catalog-mapping and implementation statuses;
- source-bound evidence for `IMPLEMENTED` and `VERIFIED` claims.

It does not invent missing requirements or assume that IDs 001–144 are 144 distinct functions. The declared count, if present, must match the number of requirement records provided.

## Run

```sh
python3 tools/validate_privacy_policy.py path/to/privacy-policy.json
python3 tools/validate_privacy_policy.py path/to/privacy-policy.json --report privacy-policy-validation-report.json
python3 -m unittest discover -s tests -p 'test_validate_privacy_policy.py'
```

Exit codes: `0` = structurally valid only; `1` = validation errors; `2` = unreadable/invalid input or report write failure.

## Fail-closed rules

- Duplicate or malformed requirement IDs fail validation.
- `MAPPED` requires catalog references; `MISSING` and `UNRESOLVED` cannot claim resolved references.
- `EXCLUDED` and `DUPLICATE` require a reason or canonical target.
- `IMPLEMENTED` / `VERIFIED` require source, test, workflow, job, step, exit-code, and review evidence.
- `VERIFIED` additionally requires a mapped catalog contract.
- A passing structural validation does not prove implementation correctness, security, legal applicability, or GDPR compliance.

The actual 144 policy statements must be imported from the approved source document and reconciled against the existing X11 catalog before this validator can report coverage.
