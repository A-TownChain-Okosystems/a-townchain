# Engineering Audit

Repository: `a-townchain`
Status: BASELINE
Last verified: 2026-09-15

## Audit contract

Code, protocol behavior, configuration and documentation must remain mutually consistent. Consensus/security changes require deterministic tests and reproducible evidence.

## Checks

- Repository-health CI is mandatory.
- Consensus and state-transition code must fail closed on malformed or invalid input.
- Security findings require remediation and regression coverage.
- Public documentation must match the implemented protocol version.

## Evidence

Record findings and verification results here as the audit proceeds. CI is authoritative for build/test evidence.
