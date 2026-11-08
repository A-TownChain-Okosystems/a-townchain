# Engineering Audit

Repository: `globus-os`
Status: BASELINE
Last verified: 2026-09-15

OS components must enforce privilege boundaries, validate hardware-facing inputs and keep kernel/userspace contracts synchronized. Unsupported hardware behavior must fail closed.

Automated baseline: `.github/workflows/repository-health.yml`.
