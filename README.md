# ATC ATVM (A-TownChain Virtual Machine)

> **ATC COMPLIANCE: R1** — auditiert am 2026-09-10 (SCR-0075; R-Level aus `.atc/repository.yaml`).


> Verifizierte Bytecode-Ausführung und deterministische Runtime-Engine für ATCLang-Verträge.

**Project:** atc-vm
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Apache-2.0 — A-TownChain-Okosystems`
**Standard:** `ATC-STD-README-001`
**Maintainer:** A-TownChain-Okosystems (ShivaCoreDev)

<!-- atc metadata block (ATC-STD-README-001 §14) -->
<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-VM-001
  name: atc-vm
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust / Python (Reference)
governance:
  security_class: S4 — Core VM & Trust Boundary
  criticality: CRITICAL (P0 Core Runtime)
-->

## Overview

atc-vm (A-TownChain Virtual Machine, ATVM) ist die kanonische Ausführungsumgebung für Smart Contracts im A-TownChain-Ökosystem (P0 Kern-Laufzeit per AD-043, Chain-ID 658467). ATVM garantiert deterministische, verifizierte und gas-limitierte Bytecode-Ausführung mit strikter Sicherheits- und Speichersandbox.

## Purpose

ATC ATVM provides the canonical execution environment and verifier engine within the A-TownChain ecosystem. It is responsible for:

- Bytecode-Verifikation (kein unverifizierter Bytecode wird je ausgeführt)
- Deterministische und gas-limitierte Vertragsausführung (Interpreter & Execution Engine)
- Bereitstellung des Runtime- und Host-Interfaces (Syscalls, State Storage, Chain-Kontext)
- Durchsetzung von Sicherheitsgrenzen und Lizenz-Gate-Enforcement (ATC-LIC)

Davon hängen die Ausführung von Smart Contracts in `a-townchain` und die Validierung in `atc-contracts` ab.

## Scope

- **Gilt für:** Kanonische Rust-Implementierung von ATVM (AD-021 Rust-first), Bytecode-Verifier, Gas-Modell und Host-Syscall-Interface.
- **Nicht-Gilt für:** Compiler-Frontend und Bytecode-Codegen (Zuständigkeit `atclang`) sowie Konsens-Block-Orchestrierung (`a-townchain`).

## Status

**Status:** `development` — R1-Skeleton (ATC-STD-201). Struktur und Governance sind definiert; Modul-Migration aus atclang nach atc-vm erfolgt im qualitätsgetriebenen Rebuild (AD-023, G0–G19 Gates).

## Architecture

### Components

- `Bytecode Verifier`: Statische Sicherheits- und Validitätsprüfung vor der Ausführung
- `Execution Engine`: Deterministischer, gas-limitierter Interpreter
- `Host Interface`: Syscall-Handling, Memory Access und State Storage Integration
- `Sandbox Boundary`: Sicherheits- und Trust-Boundary gemäß ATC-STD-NET-001/002/003

### Data Flow

`atclang` (Compiler) → ATC-Bytecode → `atc-vm` (Verifier → Interpreter / Gas Engine → Host Syscalls) → State Update in `a-townchain`.

### Dependencies

| Component | Purpose | Required |
|---|---|---|
| atclang | Compiler & Bytecode Format Specification | Yes |
| atc-shivacore | Kernel Process & Memory Isolation | Yes |
| atc-standards | Governance & Security Standards | Yes |

## Features

- Statische Bytecode-Verifikation für maximale Vertrauenswürdigkeit
- Deterministischer Interpreter mit konfigurierbarem Gas-Kostenmodell
- Integriertes License-Gate-Interface (ATC-LIC Enforcement)
- Rust-first Core Implementierung mit Fuzzing-Harness gegen die Python-Referenz

## Repository Structure

```text
.
├── docs/                # Dokumentation, Architektur und Standards
├── src/                 # ATVM Quellcode (Verifier, Interpreter, Host-Interface)
└── tests/               # Unit-, Integrations- und Fuzzing-Tests
```

## Requirements

- Rust toolchain >= 1.75 (`cargo`, `rustc`)
- Python >= 3.11 (für Referenz-Vergleichstests)
- Git >= 2.30

## Installation

### Setup

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-vm.git
cd atc-vm
cargo build
```

## Configuration

Die ATVM-Laufzeit wird über Konfigurationsdateien in `src/` sowie Metadaten in `.atc/repository.yaml` gesteuert.

## Usage

ATVM bauen und testen:

```bash
cargo build
cargo test
```

## Development

Entwicklung erfolgt nach dem Rust-first Prinzip (AD-021). Commits MÜSSEN Conventional Commits entsprechen. Qualitätsgate: ATVM ist Trust Boundary (Sicherheitsklasse S4) mit obligatorischem Audit vor dem Freeze (G18).

## Testing

Run the complete test suite:

```bash
cargo test
```

Expected result: PASS (alle Unit- und Integrationstests bestanden).

## Security

Security issues must not be disclosed publicly through GitHub Issues.

Schwachstellen werden NICHT öffentlich über GitHub Issues gemeldet, sondern direkt über den offiziellen ATC-Security-Reporting-Prozess (ATC-STD-203, [SECURITY.md](SECURITY.md)). ATVM bildet eine S4-Sicherheitsgrenze.

## Documentation

- `ARCHITECTURE.md` — Technische Architektur der VM
- `STATUS.md` — Maschinenlesbarer Projektstatus
- `ROADMAP.md` — Entwicklungsmeilensteine
- `AGENTS.md` — Instruktionen für KI-Agenten
- External Wiki: [a-townchain-os-docs](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)

## Governance

This repository is governed according to the A-TownChain Enterprise Governance Framework (ATC-STD-000 v1.3.0, ATC-ENT-001..015). Architekturentscheidungen sind im central DECISIONS_REGISTER (`a-townchain-os-docs`, AD-043) dokumentiert.

## Standards & Compliance

This repository follows applicable A-TownChain standards:

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.2.0 | ✅ |
| ATC-STD-203 | 1.0.1 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |

## Roadmap

See the canonical roadmap:

- [ROADMAP.md](ROADMAP.md)
- GitHub Issues & Projects
- ATC Development Management (Notion Master Roadmap)

## Contributing

Beiträge erfolgen gemäß [CONTRIBUTING.md](CONTRIBUTING.md) und den Governance-Regeln von ATC-STD-000 §22.

## License

Apache-2.0 — A-TownChain-Okosystems. Apache-2.0 (ATC-LIC). See [LICENSE](LICENSE).

## Maintainers

**Organization:** A-TownChain-Okosystems  
**Maintainer:** ShivaCoreDev / Aurora Superagent

## Changelog

See detailed release history in [CHANGELOG.md](CHANGELOG.md).

## A-TownChain Standards

**Smart Contract Standards Framework (ATC-STD-SC-001..020, normativ seit 07.09.2026):** Gates SC-G0..G13 gelten verbindlich — kein Gate, kein Mainnet. Contract-Registry: atc-standards/contracts/registry/contracts.yaml (ATC-SC-TOKEN-001..003 = ATC-001/ATC-8300/ATC-9900, Status: development, SC-G0 offen). Agenten-Deploy nur nach ATC-STD-SC-020 (Human/Governance-Approval).
