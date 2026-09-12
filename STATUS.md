---
document_id: ATC-DOC-ALG-002
title: "Project Status — ATC Algorithm"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-08
updated: 2026-09-08
standard: ATC-STD-MD-001
---

# Project Status — ATC Algorithm

| Property | Value |
|---|---|
| Repository | atc-algorithm |
| Version | 0.1.0 |
| Status | development |
| Build | PASS (cargo, MVP-Kern PoH+Hybrid-Selektion, CI-gruen SCR-0083) |
| Tests | PASS WITH EVIDENCE (cargo test gruen, bound_commit+test_run, SCR-0083) |
| Security | NOT AUDITED — MVP-Code vorhanden, kein Audit (SCR-0083) |
| Documentation | compliant |
| Last Audit | 2026-09-08 |

## Status Summary

Das Repository `atc-algorithm` befindet sich im Status `development` (Maturity R1-Skeleton nach ATC-STD-201, Konsens-Komponente für PoH + PoS + PoW Hybrid Consensus nach AD-044).

- 12.09.2026 (SCR-0119): ATC-HASH-001 "TownHash-256" live — EIGENSTAENDIGER Hash-Algorithmus per Owner-Direktive ("wie SHA-256, aber unserer"): Merkle-Damgard, 512-Bit-Bloecke, 8x32-Bit-Zustand, 24 Runden, Little-Endian-Wire, alle Konstanten ganzzahlig aus splitmix32 (Seed 0xA7C0DE01) als const fn abgeleitet — keine Bibliothek. Spezifikation docs/SPEC-ATC-HASH-001.md mit 6 differenzialgesicherten Testvektoren (Python-Referenz vs. Rust-Implementierung). 5 Unit-Tests: Goldene Vektoren, Determinismus/Laengenband 0-130, Bitflip-Erkennung, Padding-Kanten 55/56/57/119/120/121. Ehrlich: NICHT kryptoanalysiert — kein Mainnet ohne externe Krypto-Pruefung (F-067-Gate); Devnet-Grade als FNV-1a-Nachfolger, Adoption in atc-node als naechste Welle.
