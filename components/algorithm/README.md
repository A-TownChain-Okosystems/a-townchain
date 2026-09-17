# ATC Algorithm

> **ATC COMPLIANCE: R1** — repository governance baseline. This repository is the **canonical consensus implementation target**, but it is **not yet production-ready**.

**Project:** atc-algorithm  
**Organization:** A-TownChain-Okosystems  
**Status:** `development`  
**Repository Version:** `0.1.0`  
**License:** `Apache-2.0`

## Overview

`atc-algorithm` is the canonical consensus repository for A-TownChain (Chain-ID `658467`). It contains the current MVP/skeleton implementation and the normative consensus work required for a production release.

**Critical truth:** the presence of PoH/selection prototype code and passing local tests does **not** constitute a complete, secure, deterministic or production-ready consensus protocol. Consensus remains a P0 release blocker until specification freeze, complete implementation, conformance, security review and independent evidence are complete.

## Purpose

The repository owns the canonical consensus logic for:

- Proof-of-History sequencing
- Proof-of-Stake validator weighting
- Proof-of-Work integration
- Hybrid selection rules
- Fork-choice rules
- Finality rules
- Consensus verification and adversarial testing

`a-townchain` is the orchestration/integration layer and must not become a second canonical consensus implementation.

## Status

**Current maturity:** `R1-SKELETON / DEVELOPMENT`

Evidence currently supports an MVP/skeleton implementation, not a production consensus engine. The current source tree contains only a limited implementation surface (including `poh`, selection and hash components); the full consensus engine specified by the architecture is not yet complete.

### P0 Release Gates

The consensus P0 remains **OPEN** until all of the following are evidenced:

1. **Consensus specification freeze** — complete normative rules for PoH, PoS, PoW, hybrid selection, fork choice and finality.
2. **Complete Rust implementation** — implementation matches the frozen specification.
3. **Determinism/conformance** — cross-node differential and conformance vectors pass.
4. **Security review** — independent cryptographic and consensus-security assessment.
5. **Adversarial testing** — equivocation, replay, reorg, fork-choice, validator-set and timing cases.
6. **Reproducible build/evidence** — release artifacts are reproducible and bound to evidence.

**Mainnet:** `NO-GO` while any P0 gate is open.

## Architecture

```text
Transactions / Blocks
        │
        ▼
PoH sequencing
        │
        ▼
Validator / PoS inputs + PoW constraints
        │
        ▼
Hybrid selection
        │
        ▼
Fork choice + finality
        │
        ▼
Canonical consensus result
        │
        ▼
a-townchain orchestration
```

### Components

| Component | Purpose | Current state |
|---|---|---|
| `poh` | Proof-of-History sequencing | MVP/skeleton |
| `pos` | Validator weighting / staking | Specification work |
| `pow` | PoW constraints | Specification work |
| `hybrid_engine` | Hybrid selection/finality | Specification work |
| `hash` | TownHash development component | Devnet-grade; not cryptographically audited |

## Repository Structure

```text
atc-algorithm/
├── docs/
├── src/
└── tests/
```

## Requirements

- Rust `1.75+`
- Python `3.10+` for validation tooling
- Git `2.30+`

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-algorithm.git
cd atc-algorithm
cargo build
```

## Testing

```bash
cargo test
```

A green test run proves only the tested implementation surface. It does not close the consensus P0 gates above.

## Security

Security issues must be reported according to `SECURITY.md` / ATC-STD-203. The repository remains `NOT_AUDITED` for production consensus until the required security review is complete.

## Governance

This repository follows ATC-STD-000 v1.3.0 and the A-TownChain governance framework. Consensus changes require the applicable protocol authority, review chain, evidence and release gates.

## Standards & Compliance

| Standard | Version | Status |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.2.0 | ✅ |
| ATC-STD-203 | 1.0.1 | ✅ |
| ATC-STD-204 | 1.0.0 | ✅ |

## Roadmap

1. Freeze normative consensus specification.
2. Complete PoH/PoS/PoW/hybrid implementation.
3. Add deterministic conformance vectors and differential tests.
4. Complete adversarial/fuzz testing.
5. Complete independent security review.
6. Produce reproducible release evidence.
7. Only then promote toward testnet/mainnet release gates.

## Maintainers

**Organization:** A-TownChain-Okosystems  
**Owner/Maintainer:** governed through the applicable authority and approval records.

## License

Apache-2.0 — see `LICENSE`.
