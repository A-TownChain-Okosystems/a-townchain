---
document_id: ATC-DOC-MIN-STA-001
title: Status — atc-mining
version: 1.1.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-15
standard: ATC-STD-MD-001
---

# STATUS — atc-mining

Stand: 15.09.2026 · Self-Compliance: ATC-STD-201 R1

## Property-Value-Tabelle

| Property | Value |
|---|---|
| Repository | atc-mining |
| Version | 0.1.0 |
| Status | development |
| Build | IMPLEMENTED — Cargo crate with executor, deterministic SHA3 mining primitives and bounded reward ledger |
| Tests | CI PENDING — unit tests are included for executor, hashing, target comparison and reward supply bounds |
| Security | S2 (medium criticality, ATC-STD-203) |
| Documentation | compliant |
| Last Audit | 2026-09-15 |

## Implemented Core

- `src/executor.rs` — FIFO execution queue.
- `src/miner.rs` — deterministic SHA3-256 candidate hashing and bounded nonce search against an explicit target.
- `src/reward.rs` — checked reward issuance with a hard supply ceiling.
- Consensus rules remain owned by `atc-algorithm`; mining does not mutate or redefine consensus.
- Contract settlement remains external to this crate and is owned by `atc-contracts`.

## Remaining P1

- Hardware-specific CPU/GPU backends.
- Network job transport and authenticated work distribution.
- Canonical integration with `atc-algorithm` difficulty rules.
- SC-018 settlement adapter and MinerWatcher telemetry.
- Hardware/benchmark evidence before production readiness.

## Quality Assurance

Unit tests are committed with the implementation. GitHub Actions must provide the authoritative build/test evidence before the repository can claim PASS WITH EVIDENCE.
