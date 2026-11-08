# Engineering Audit

Repository: `atclang`
Status: BASELINE
Last verified: 2026-09-15

## Required audit loop

1. Implement code changes from authoritative requirements.
2. Run repository-native formatting, build and test checks.
3. Record functional errors and their evidence.
4. Record security findings and their evidence.
5. Record consistency findings across code, manifests and documentation.
6. Fix findings, then rerun the affected checks.
7. Verify module/function connectivity and fail closed on missing evidence.

## Current evidence boundary

This file establishes the organization-wide audit contract. It does **not** claim that the repository is production-ready. CI is the authoritative execution evidence.

## Security baseline

High-confidence credential patterns are rejected by `.github/workflows/repository-health.yml`. Cryptographic, parser, dependency and runtime security require repository-specific review.

## Consistency baseline

The repository name, documentation, manifests, public APIs and CI configuration must describe the same current implementation state. Target architecture must not be documented as implemented software.

## Evidence

- Automated repository-health workflow: `.github/workflows/repository-health.yml`
- Deep findings must be added here with reproducible evidence and a verification result.
