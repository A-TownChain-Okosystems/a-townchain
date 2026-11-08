# Engineering Audit

Repository: `atc-compute`
Status: BASELINE
Last verified: 2026-09-15

Compute jobs must validate resource limits and untrusted inputs before execution. Determinism, resource accounting and failure behavior must match the documented contract.

Automated baseline: `.github/workflows/repository-health.yml`.
