# STATUS — atc-sdk

**Stand:** 2026-09-16  
**Version:** v1.0.0  
**Status:** DEVELOPMENT / AUDIT REMEDIATION

## Verified scope

The repository was re-checked for documentation-to-tree consistency, architecture boundaries, and build instructions.

### Findings and remediation

- **FIXED:** README previously instructed `cargo build --workspace` from the repository root although the current tree does not contain a root `Cargo.toml`.
- **FIXED:** `ARCHITECTURE.md` previously described root-level `rust/` and `typescript/` trees that are not present on the current `main` tree.
- **FIXED:** Documentation now identifies `modules/atc-sdk`, `modules/atc-cli`, and `modules/atc-atcpkg` as the active module structure.
- **DOCUMENTED:** Module-specific build/test commands must be derived from the module manifests and CI rather than assumed at repository root.
- **DOCUMENTED:** Devnet-only integration evidence is not production-readiness evidence.

## Existing implementation evidence

The repository contains module-level implementation and integration material, including the SDK/CLI modules and a devnet integration test described in the historical audit trail. The exact current test result must come from the current CI run; absence of a connector-visible status is not treated as a passing result.

## Remaining verification gates

- Current CI/build/test run on the remediation commits.
- Dependency/security scan evidence for all active module manifests.
- Cross-repository API compatibility checks against current `atc-vm` and `atc-node` interfaces.
- Re-run of the organization fleet audit after remediation.

No `PRODUCTION_READY` claim is made by this status file.

## Organization scope

This repository is one component of the `A-TownChain-Okosystems` fleet. Repository count and integration topology are maintained by `atc-engineering` and the canonical `atc-standards` registry.
