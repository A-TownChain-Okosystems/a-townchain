# ATC Aurora AI

> AI services, agent infrastructure and AI integration layer of the A-TownChain ecosystem.

**Project:** `aurora-ai`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0` (see `LICENSE`)

## Overview

Aurora AI is the AI layer of the ecosystem. It provides AI services, agent infrastructure, model/runtime integration, memory/context services and interfaces used by GlobusOS and other ecosystem components.

Aurora AI is **not part of the ShivaCore TCB**. The security boundary remains:

```text
ShivaCore
(kernel / TCB / capabilities)
      │
      ▼
GlobusOS
(operating-system userspace)
      │
      ▼
Aurora AI
(AI services / agents / inference / UX)
```

Aurora may consume authorized OS and system events, but AI decisions do not automatically override deterministic chain, kernel, security or governance authority.

## Purpose

Aurora AI provides:

- AI services and agent execution infrastructure.
- Rust-based core/runtime components where performance and isolation are required.
- Python-based AI services and model integration where applicable.
- Agent memory and context management.
- AI Studio and related development tooling.
- Security/telemetry-oriented AI components where explicitly implemented.

## Status

`development` means the repository is under active development. Individual milestones, audits or successful tests do not imply that the complete platform is `PRODUCTION_READY`.

This README makes no Mainnet or production claim.

## Architecture

Current repository components include:

- `modules/atc-aurora-core` — Rust core engine.
- `modules/atc-aurora-agents` — agent framework and role management.
- `modules/atc-aurora-memory` — memory/context services.
- `modules/atc-aurora-runtime` — runtime and integration services.
- `modules/atc-aurora-ai` — Python AI services.
- `modules/atc-aistudio` — AI development/UI tooling.

Typical integration flow:

```text
Authorized OS / IPC events
        ↓
Aurora Core / Runtime
        ↓
Agent execution
        ↓
AI services / model inference
        ↓
Memory / context
```

The exact authority of each interface is determined by the applicable system and security specifications.

## Requirements

- Rust toolchain for Rust modules.
- Python 3.10+ for Python AI modules.
- Node.js where required by `atc-aistudio`.
- Cargo and pip.

Exact module requirements are authoritative in the corresponding manifests.

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/aurora-ai.git
cd aurora-ai
```

Build and installation commands are component-specific; use the module README/manifests for the selected service.

## Testing

Run tests for the relevant modules, for example:

```bash
cd modules/atc-aurora-core && cargo test
```

```bash
cd modules/atc-aurora-ai && pytest
```

A passing test suite establishes evidence for the executed tests only. It does not by itself establish audit or production readiness.

## Security

Security-sensitive vulnerabilities must not be disclosed through public GitHub Issues. Follow `SECURITY.md` and the approved ATC security-disclosure process.

Aurora AI must not be treated as an implicit trust anchor or kernel authority.

## Governance

Development follows `ATC-STD-000` and applicable repository/security standards.

Canonical standard identifiers use the family-scoped form:

```text
ATC-STD-F{family}-{sequence}
```

Legacy IDs remain preserved during migration. No standard ID may be silently renumbered, reused, or autonomously allocated outside the canonical registry/governance process.

## Compliance terminology

- **APPROVED** — formally approved by governance.
- **IMPLEMENTED** — implementation exists.
- **AUDITED** — relevant audit has been completed and recorded.
- **PRODUCTION_READY** — all required release gates have passed.

These states must not be inferred from one another.

## Repository Structure

```text
.
├── .atc/
├── .github/
├── docs/
└── modules/
    ├── atc-aistudio/
    ├── atc-aurora-agents/
    ├── atc-aurora-ai/
    ├── atc-aurora-core/
    ├── atc-aurora-memory/
    └── atc-aurora-runtime/
```

## Documentation

Use `docs/`, module documentation, `AGENTS.md`, `ARCHITECTURE.md`, `STATUS.md`, and `ROADMAP.md` where present for detailed project information.

## License

Apache License 2.0. See `LICENSE`.

## Repository Metadata

<!-- atc metadata block (ATC-STD-README-001 §14) -->
<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-AI-001
  name: aurora-ai
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
  secondary_language: Python
  secondary_scope: model_research, non_tcb_tooling, automation
governance:
  criticality: HIGH
-->


![ATC COMPLIANCE](https://img.shields.io/badge/ATC%20COMPLIANCE-R3%20%C2%B7%20ATC--STD--201%2F202%2F203-brightgreen)
