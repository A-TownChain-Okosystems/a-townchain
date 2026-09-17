---
document_id: ATC-DOC-ZKP-006
title: "Technical Architecture"
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-07
standard: ATC-STD-MD-001
---

# Technical Architecture — ATC ZKP Layer

## Overview

Die ZKP Layer bildet die kryptografische Verifikationsinfrastruktur der A-TownChain.

## Components

| Component | Purpose | Required |
|---|---|---|
| `zkp-core` | Core traits and types | Yes |
| `zkp-verifier` | Verification engine | Yes |
| `zkp-prover` | Off-chain prover | Yes |

## Data Flow

```text
Tx Data -> Circuit Execution -> Proof Generation -> On-Chain Verification
```
