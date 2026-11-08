---
document_id: ATC-DOC-ATCOS-STATUS-001
title: Repository Status — a-townchain-os
version: 1.2.0
status: active
owner: A-TownChain-Okosystems
updated: 2026-10-09
standard: ATC-STD-MD-001
---

# Repository Status — a-townchain-os

> **Evidence freshness notice (2026-10-09):** the 106/106 GATE-KAI-001 result below is historical and is not a current-main verification. The checked-in evidence registry currently declares `bound_commit: pending`, `tests: not_run`, and `latest_verified: null`. Therefore the current repository remains `development` / `NOT_READY`; do not report current build/test verification until a current exact-SHA run and evidence binding exist.

| Property | Value |
|---|---|
| Repository | a-townchain-os |
| Version | 0.1.0 |
| Status | development |
| Security Class | S3 |
| Architecture role | L7 Domain Integration / Core Runtime |
| Language boundary | Rust native infrastructure; ATCLang on-chain semantics; ATC-VM execution boundary |
| Build | NOT VERIFIED — current CI evidence required |
| Tests | 106/106 was recorded for GATE-KAI-001 Iteration 2; a fresh CI run is required for release evidence |
| Documentation | ATC-STD-README-001 / ATC-STD-MD-001 |
| Production readiness | NOT_READY |
| Current gate | GATE-KAI-001 Iteration 2: code-side closure recorded; external release gates remain |
| Last documentation cross-check | 2026-10-09 (status/evidence freshness review) |

## Current cross-check

The repository status text was reviewed for freshness on 2026-10-09. The architectural descriptions below are not a new source-code audit; current runtime behavior and release readiness require exact-SHA CI and integration evidence.

The former Issue #112 code-side work areas are present in the current tree:

- G2-A: deterministic Model Registry and SHA-256-verified model cache;
- G2-B: persistent state with WAL-before-state ordering, hash-chain validation and crash recovery;
- G2-C: real TCP transport, bounded framing/backpressure and churn/reconnect coverage;
- G2-D: versioned protocol envelopes and deterministic snapshot-return rollback semantics.

GATE-KAI-001 Review Iteration 2 records 106/106 tests green and the code-side closure of these areas. This is historical release evidence, not a substitute for a fresh CI run on the current commit.

## Remaining production gates

`PRODUCTION_READY` remains deliberately blocked until all of the following are evidenced:

1. a fresh current-commit CI/build/test run is green;
2. the external security audit described by the GATE-KAI-001 security checklist is completed and findings are dispositioned;
3. the current production network configuration, TLS/DoS hardening and multi-node operational evidence are independently verified;
4. release approval is recorded by the owner.

No production claim is inferred from documentation alone.

## Architecture consistency

The repository remains the L7 integration/runtime layer. It must not absorb ShivaCore kernel responsibilities, and it must not move blockchain consensus semantics into the OS runtime. AD-008 remains the language boundary: ATCLang is canonical for on-chain semantics, Rust carries native infrastructure, and ATC-VM is the execution boundary.

## Evidence policy

No PASS state is inferred from documentation alone. Current build, test, security and production claims require machine-readable evidence from the relevant current commit/CI run. Hardware readiness additionally requires hardware-backed evidence.
