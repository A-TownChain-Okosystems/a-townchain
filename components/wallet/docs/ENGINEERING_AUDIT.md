# Engineering Audit

Repository: `atc-wallet`
Status: BASELINE
Last verified: 2026-09-15

Wallet code must protect key material, validate transaction/network identity and fail closed on malformed state. No secret material belongs in source or documentation.

Automated baseline: `.github/workflows/repository-health.yml`.
