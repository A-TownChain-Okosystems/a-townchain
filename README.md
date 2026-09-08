# ATC Storage

> Dezentrale Storage-Schicht für Assets, Metadaten und große Dateien im A-TownChain-Ökosystem

**Project:** atc-storage
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Apache-2.0`

<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-STORAGE-001
  name: atc-storage
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: S2
  criticality: C2
-->

## Overview

`atc-storage` stellt die dezentrale Storage-Schicht (Layer L5, Roadmap M6) des A-TownChain-Ökosystems bereit. Das System dient der Speicherung und Verteilung von NFT-Metadaten, Game-Assets, AI-Modellen, Datasets und Benutzerdateien. Es basiert auf BLAKE3 Content-Addressing und verknüpft Layer-5-ContentCap mit dem ShivaCore Kernel.

## Purpose

`atc-storage` implementiert die dezentrale Speicherinfrastruktur für A-TownChain. Es löst folgende Aufgaben:
- Content-Addressable Storage (CAS) mit BLAKE3 Hashing.
- Dezentrale Replikation und Verschlüsselung vertraulicher Daten.
- Bereitstellung von Storage-Schnittstellen für `aurora-ai`, `genesis-chronicles` und Verticals.
- Speicherung und Indizierung großer Datenmengen (Blobs) im A-TownChain Netz.

## Scope

- **In Scope:** BLAKE3 Content Addressing, Storage Node Protocol, Client SDK, Chunking & Replication, Storage Encryption, ContentCap L5.
- **Out of Scope:** On-Chain State Execution (liegt in `a-townchain`), Kernel Low-Level Primitive (liegt in `atc-shivacore`).

## Status

**Status:** `development` — Skeleton & Architektur-Fundament erstellt (AD-024). Audit-Level R1 am 2026-09-07. Implementierung verläuft qualitätsgetrieben gemäß Rebuild-Roadmap (AD-023).

## Architecture

### Components
- **Protocol Layer (`protocol/`):** Storage Node Handshake & Replikations-Protokoll.
- **Node Core (`node/`):** Local Store, BLAKE3 Indexing & Chunk Management.
- **Client SDK (`client/`, `sdk/`):** Upload, Download & Verification Schnittstelle.
- **Encryption & Replication (`encryption/`, `replication/`):** Symmetrische Verschlüsselung und Erasure Coding.

### Data Flow
1. Client hasht Daten via BLAKE3 und zerlegt Blobs in Chunks.
2. Chunks werden optional verschlüsselt und mit ContentCap-Signatur signiert.
3. Storage Nodes empfangen und replizieren Chunks im Netzwerk.
4. Clients verifizieren die Integrität beim Download via Merkle-Tree.

### Dependencies
| Component | Purpose | Required |
|---|---|---|
| Rust 1.98.1 | Core Toolchain | Yes |
| blake3 | High-Performance Hashing | Yes |
| tokio | Asynchrone I/O & Networking | Yes |

## Features

- **BLAKE3 Content Addressing:** Eindeutige und unmanipulierbare Identifikation aller Blobs.
- **ContentCap (Layer L5):** Integration mit dem ShivaCore Capability-Modell.
- **Chunking & Erasure Coding:** Hohe Ausfallsicherheit durch dezentrale Redundanz.
- **Zero-Knowledge Ready:** Verschlüsselte Speicherung für vertrauliche Daten.

## Repository Structure

```text
.
├── .atc/                # ATC-Repository-Metadaten
├── .github/             # GitHub Workflows
├── docs/                # Extended Documentation & Standards
├── tests/               # Testpläne & Testsuite
├── AGENT_MANIFEST.md    # Agent Manifest
├── AGENTS.md            # AI Agent Instructions
├── ARCHITECTURE.md      # Storage Architecture Specification
├── CHANGELOG.md         # Change History
├── CODE_OF_CONDUCT.md   # Code of Conduct
├── CODEOWNERS           # Repository Owners
├── CONTRIBUTING.md      # Beitragsrichtlinien
├── GOVERNANCE.md        # Governance Rules
├── LICENSE              # Apache-2.0 License
├── README.md            # Repository Einstiegspunkt
├── ROADMAP.md           # Development Roadmap
├── SECURITY.md          # Security Policy
└── STATUS.md            # Machine-readable Status
```

## Requirements

- Rust 1.98.1 oder neuer
- Cargo & Build Tools
- Python 3.10+ für Workspace-Skripte

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-storage.git
cd atc-storage
cargo build
```

## Configuration

Die Node-Konfiguration erfolgt über `.atc/repository.yaml` sowie Umgebungsvariablen.

## Usage

Starten einer lokalen Storage-Node:

```bash
cargo run --bin atc-storage-node
```

## Development

- Befolgen Sie Conventional Commits.
- Integration ins Monorepo via `scripts/sync_modules.py`.

## Testing

Ausführen der Tests:

```bash
cargo test
```

**Erwartetes Ergebnis:** `PASS` (Testsuite grün).

## Security

Security issues **must not** be disclosed publicly. Report vulnerabilities through the official ATC security reporting process or contact `security@a-townchain.org` (ATC-STD-203, SECURITY.md).

## Documentation

- Architecture Spec: [`ARCHITECTURE.md`](ARCHITECTURE.md)
- Testplan: [`tests/TESTPLAN.md`](tests/TESTPLAN.md)
- Organizational Docs: [a-townchain-os-docs](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs)

## Governance

Governed by **ATC-STD-000** (A-TownChain Enterprise Governance Framework).

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.2.0 | ✅ APPROVED |
| ATC-STD-README-001 | 1.0.0 | ✅ APPROVED |
| ATC-STD-MD-001 | 1.0.0 | ✅ APPROVED |
| ATC-STD-201 | 1.0.0 | ✅ APPROVED |
| ATC-STD-202 | 1.1.0 | ✅ APPROVED |
| ATC-STD-203 | 1.0.0 | ✅ APPROVED |

## Roadmap

Die Roadmap ist in [`ROADMAP.md`](ROADMAP.md) hinterlegt (Meilenstein M6 in der Lauffähigkeits-Roadmap).

## Contributing

Siehe [`CONTRIBUTING.md`](CONTRIBUTING.md) für Richtlinien.

## License

Standardisiert unter **Apache-2.0** (siehe [`LICENSE`](LICENSE)).

## Maintainers

- **Organization:** A-TownChain-Okosystems
- **Owner:** Michael Wroblewski (GitHub: ShivaCoreDev)
- **Maintainer:** Storage Team / Aurora Superagent

## Changelog

Siehe [`CHANGELOG.md`](CHANGELOG.md) für Details.
