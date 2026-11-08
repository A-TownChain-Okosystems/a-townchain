---
document_id: ATC-DOC-SHIVACORE-STATUS-001
title: Repository Status - atc-shivacore
version: 2.2.0
status: active
standard: ATC-STD-MD-001
created: 2026-09-08
updated: 2026-10-09
---

## Status — atc-shivacore

> **Dokumentations-Refresh (2026-10-09):** Der früher unten als weiterhin offen beschriebene LKM-Dependency-API-Blocker ist in der inspizierten kanonischen Quelle `globus-os` auf Commit `aad164fb21f82025043ade4e1dad8740fcc65b69` nicht mehr in der beschriebenen Form vorhanden: `dependencies(&self, name: &str) -> Vec<String>` ist implementiert. Das ist ein Code-Sichtbefund, keine Verifikation des aktuellen Kernel-Releases; der jüngste hier geprüfte GlobusOS-CI-Stand enthält fehlgeschlagene System-/Test-/Rust-Gates. Die Evidence-SSOT unten bleibt maßgeblich für die hier beanspruchten Reifegrade.

## Property-Value Table

| Property | Value |
|---|---|
| Repository | atc-shivacore |
| Version | 0.1.0 |
| Lifecycle | development / migrated |
| Kernel model | capability-based Rust/no_std microkernel |
| Reuse target | OS-neutral kernel contract |
| Canonical source | `A-TownChain-Okosystems/globus-os/modules/atc-shivacore/kernel/` |
| Kernel CI owner | `A-TownChain-Okosystems/globus-os` |
| Build evidence | Must be taken from the current GlobusOS CI run |
| Boot evidence | Must be taken from the current GlobusOS CI run |
| Production status | NOT_READY |
| Security class | S4 / S-Klasse |
| Documentation | ATC-STD-README-001 / ATC-STD-MD-001 |

## Source-of-truth boundary

The reusable ShivaCore kernel implementation has been migrated into the GlobusOS repository so that the kernel is built, tested, linted and integrated by the same CI pipeline as the operating system.

The canonical implementation path is:

`globus-os/modules/atc-shivacore/kernel/`

The separate `atc-shivacore` repository is no longer the active kernel source tree. Historical copies under `a-townchain-os-docs/docs/archive/` are reference material only and must not be treated as implementation sources.

## Historical finding — LKM dependency API

The previous P1 finding described a placeholder `DependencyGraph::dependencies()` signature and pointed to GlobusOS issue #18. The current inspected source contains a lifetime-safe owned-vector API. Keep the issue/history for traceability, but do not report the original defect as still open without rechecking the current canonical source. The LKM subsystem remains subject to current GlobusOS build/test/security evidence and the repository's `latest_verified: null` state.

## Evidence policy

Historical test counts and past audit scores are not permanent state. Current GlobusOS CI is authoritative for build/test/lint claims for the canonical kernel source.

## Reuse readiness

The reusable kernel contract remains documented in:

- `docs/specs/SHIVA-KERNEL-REUSE-001.md`
- `docs/specs/SHIVA-HAL-001.md`
- `docs/specs/SHIVA-ABI-001.md`
- `docs/specs/SHIVA-BOOT-001.md`

These specifications define the architecture boundary. They do not by themselves prove production readiness.

## Release gate

`PRODUCTION_READY` requires successful target builds, boot/smoke evidence, IPC/ABI validation, security review, resolution of active implementation blockers and governance approval for the release candidate.
