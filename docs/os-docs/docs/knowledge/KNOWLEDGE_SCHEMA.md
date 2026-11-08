# Knowledge Entry Schema

Every substantive knowledge entry should answer the following.

## 1. Identity

- Name
- Repository
- Path/module
- Domain
- Owner/canonical source

## 2. Purpose

What does this component do?

## 3. Interfaces

- Inputs
- Outputs
- Public APIs
- Wire/storage formats
- Upstream dependencies
- Downstream consumers

## 4. Invariants

List protocol, economic, security or architectural invariants that must remain true.

## 5. Evidence

| Evidence | Required record |
|---|---|
| Source | exact repository/path/symbol |
| Tests | exact test/module |
| CI | workflow/run/job |
| E2E | scenario and run/artifact |

## 6. State

Use only:

MISSING, STUB, IMPLEMENTED, TESTED, CI_VERIFIED, E2E_VERIFIED, BROKEN, DISCONNECTED, DUPLICATE.

## 7. Change history

Record:

date → change → commit/PR → verification result

## 8. Open questions

Unknowns must remain explicitly unknown. Do not convert assumptions into facts.

## 9. Relationships

Link the entry to:

- architecture components
- repositories
- standards
- CI gates
- E2E scenarios
- security boundaries
- dependent knowledge entries
