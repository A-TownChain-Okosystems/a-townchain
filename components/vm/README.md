# ATC ATVM (A-TownChain Virtual Machine)

> **ATC COMPLIANCE: R1** — auditiert am 2026-09-10 (SCR-0075; R-Level aus `.atc/repository.yaml`).

> Verifizierte Bytecode-Ausführung und deterministische Runtime-Engine für ATCLang-Verträge. ATVM ist die technische Grenze zwischen on-chain ATCLang und der Rust-basierten Chain-Infrastruktur.

**Project:** atc-vm  
**Organization:** A-TownChain-Okosystems  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0 — A-TownChain-Okosystems`  
**Standard:** `ATC-STD-README-001`  
**Maintainer:** A-TownChain-Okosystems (ShivaCoreDev)

## Overview

`atc-vm` (A-TownChain Virtual Machine, ATVM) ist die kanonische Ausführungsumgebung für Smart Contracts im A-TownChain-Ökosystem. ATVM garantiert deterministische, verifizierte und gas-limitierte Bytecode-Ausführung mit strikter Sicherheits- und Speichersandbox.

ATVM ist die **Boundary** zwischen:

- **ATCLang:** on-chain Sprache, Contract- und State-Transition-Logik.
- **ATVM:** Bytecode-Verifikation und deterministische Ausführung.
- **Rust Chain Infrastructure:** Node, Networking, State, IPC und weitere chain-tragende Komponenten.

## Purpose

ATVM provides the canonical execution environment and verifier engine within the A-TownChain ecosystem. It is responsible for:

- Bytecode-Verifikation — kein unverifizierter Bytecode wird ausgeführt.
- Deterministische und gas-limitierte Vertragsausführung.
- Runtime- und Host-Interfaces für Syscalls, State Storage und Chain-Kontext.
- Durchsetzung definierter Sicherheitsgrenzen.

## Scope

- **Gilt für:** Kanonische Rust-Implementierung von ATVM, Bytecode-Verifier, Gas-Modell und Host-Syscall-Interface.
- **Nicht-Gilt für:** Compiler-Frontend und Bytecode-Codegen (`atclang`) sowie Konsens-/Block-Orchestrierung (`a-townchain`).
- **Kernel boundary:** Kernel- und TCB-Funktionen bleiben in `atc-shivacore`.

## Status

**Status:** `development` — R1-Skeleton. Struktur und Governance sind definiert; die weitere Implementierung folgt den qualitäts- und sicherheitsorientierten Gates.

ATVM ist **nicht automatisch production-ready**, nur weil einzelne Tests oder Integrationspfade erfolgreich sind. Release-Readiness wird aus Evidence, Conformance und Audit-Gates abgeleitet.

## Architecture

```text
ATCLang source
     │
     ▼
Compiler / Codegen
     │
     ▼
ATC Bytecode
     │
     ▼
ATVM Verifier
     │
     ▼
Deterministic Execution / Gas Engine
     │
     ▼
Host Interface
     │
     ▼
a-townchain State / Chain Infrastructure
     │
     ▼
atc-shivacore kernel boundary
```

### Components

- `Bytecode Verifier`: Statische Sicherheits- und Validitätsprüfung vor der Ausführung.
- `Execution Engine`: Deterministischer, gas-limitierter Interpreter.
- `Host Interface`: Syscall-Handling, Memory Access und State Storage Integration.
- `Sandbox Boundary`: Sicherheits- und Trust-Boundary für Contract Execution.

### Dependencies

| Component | Purpose | Required |
|---|---|---|
| `atclang` | Compiler & Bytecode Format | Yes |
| `atc-shivacore` | Kernel Process & Memory Isolation | Yes |
| `atc-standards` | Governance & Security Standards | Yes |
| `a-townchain` | Chain / State Integration | Yes |

## Features

- Statische Bytecode-Verifikation.
- Deterministischer Interpreter mit Gas-Kostenmodell.
- Definiertes Host-/Syscall-Interface.
- Rust-first Core Implementierung.
- Fuzzing- und Differential-Testing gegen Referenzpfade.

## Repository Structure

```text
.
├── docs/                # Dokumentation, Architektur und Standards
├── src/                 # ATVM Quellcode
└── tests/               # Unit-, Integrations- und Fuzzing-Tests
```

## Requirements

- Rust toolchain >= 1.75 (`cargo`, `rustc`)
- Python >= 3.11 für Referenz-Vergleichstests
- Git >= 2.30

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-vm.git
cd atc-vm
cargo build
```

## Usage

```bash
cargo build
cargo test
```

## Development

Entwicklung erfolgt nach dem Rust-first Prinzip. Commits MÜSSEN Conventional Commits entsprechen. ATVM ist eine sicherheitskritische Trust Boundary; Production-Release erfordert vollständige Evidence-, Conformance- und Security-Gates.

## Testing

```bash
cargo test
```

Testergebnisse sind nur zusammen mit dem jeweiligen Evidence-Bundle ein Release-Nachweis.

## Security

Security issues must not be disclosed publicly through GitHub Issues. Schwachstellen werden gemäß dem offiziellen ATC-Security-Reporting-Prozess (`ATC-STD-203`, `SECURITY.md`) behandelt.

## Documentation

- `ARCHITECTURE.md` — Technische Architektur der VM
- `STATUS.md` — Maschinenlesbarer Projektstatus
- `ROADMAP.md` — Entwicklungsmeilensteine
- `AGENTS.md` — Instruktionen für KI-Agenten
- `a-townchain-os-docs` — zentrale technische Dokumentation

## Governance

Dieses Repository unterliegt `ATC-STD-000` und den jeweils geltenden A-TownChain-Standards.

Die Standard-ID-Migration verwendet die Family-scoped Form `ATC-STD-F{family_id}-{sequence}`. Canonical Allocation erfolgt ausschließlich über Registry und Governance. Legacy-IDs bleiben unverändert; es gibt keine stille Umnummerierung oder Wiederverwendung.

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.2.0 | ✅ |
| ATC-STD-203 | 1.0.1 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |

## Roadmap

Siehe `ROADMAP.md`, GitHub Issues/Projects und die kanonischen Dokumentationsquellen.

**Kein Production- oder Mainnet-Claim wird allein aus README-Status abgeleitet.**

## Contributing

Beiträge erfolgen gemäß `CONTRIBUTING.md` und den Governance-Regeln von `ATC-STD-000`.

## License

Apache-2.0 — A-TownChain-Okosystems. Siehe `LICENSE`.

## Maintainers

**Organization:** A-TownChain-Okosystems  
**Maintainer:** ShivaCoreDev / Aurora Superagent

## Changelog

Siehe `CHANGELOG.md`.

## Repository Metadata

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
  criticality: CRITICAL
-->
