---
document_id: ATC-AUDIT-SHIVA-20260916
title: Repository Audit — atc-shivacore
version: 1.1.0
status: active
owner: A-TownChain-Okosystems
audit_date: 2026-09-16
standard: ATC-STD-AUDIT-001
---

# Repository Audit — atc-shivacore

## Audit mode

CI-independent source/repository audit performed against the current `main` state and current repository tree. Findings are closed only after source change, re-read, and executable verification where applicable.

## Findings

### F-20260916-SHIVA-001 — P1 — Canonical-source metadata contradiction

**Class:** P1  
**Category:** governance / architecture / consistency  
**Family:** kernel / canonical-source / repository-metadata  
**Tags:** P1, shivacore, canonical-source, governance, metadata, consistency

`.atc/repository.yaml` declared `CORE`, `R4`, `production` and `canonical kernel`, while `STATUS.md` and the README state that the active canonical kernel source is `globus-os/modules/atc-shivacore/kernel/` and this repository is supporting material.

**Correction:** repository metadata is changed to `INFRA`, `R3`, `development`; the kernel capability is explicitly non-canonical here.

### F-20260916-SHIVA-002 — P1 — Stale evidence binding

**Class:** P1  
**Category:** evidence / traceability  
**Family:** governance / audit / release-evidence  
**Tags:** P1, evidence, stale, bound-commit, shivacore, traceability

`.atc/evidence/evidence.yaml` was bound to an earlier kernel-source commit and asserted a canonical role that is no longer true.

**Correction:** evidence is now explicitly unbound/pending until current verification CI produces commit-bound evidence, and the canonical role is false.

### F-20260916-SHIVA-003 — P1 — Invalid local build instructions

**Class:** P1  
**Category:** correctness / documentation / build  
**Family:** repository-architecture / build-system  
**Tags:** P1, cargo, documentation-drift, build, canonical-source

The repository has no root `Cargo.toml`, so the former root `cargo build --workspace` instruction was invalid for the current checkout.

**Correction:** the README now directs canonical kernel builds/tests to `globus-os`.

## Security / hack / malware posture

Static source and workflow review found no confirmed credential exposure or malicious payload in the inspected material. CI workflows include CodeQL, dependency review, RustSec auditing and determinism checks. These controls provide evidence for specific attack classes, but they do not prove universal immunity to compromise, malicious dependencies, supply-chain attacks or malware.

Stronger proof requires current CI results, cryptographically verified dependencies and commits, reproducible builds, SBOM/provenance validation, secret scanning, independent security review and runtime/hardware verification.

## Language / file format

Rust is appropriate for the canonical low-level kernel. Markdown/YAML are appropriate for contracts/governance. Supporting scripts may use Python/Rust as explicitly documented. No broad language migration is justified.

## Verification state

**IN PROGRESS.** The identified metadata/documentation contradictions were corrected and the modified sources were re-read on the audit branch. Final closure remains dependent on current executable verification and repository-standard gates.
