# Engineering Audit

Repository: `atc-storage`
Status: BASELINE
Last verified: 2026-09-15

Persistent storage must validate paths, sizes and serialization before mutation. Crash consistency, integrity and recovery behavior must be explicit and tested.

Automated baseline: `.github/workflows/repository-health.yml`.
