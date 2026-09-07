# ATC ZKP Layer

> Zero-Knowledge Proof Layer der A-TownChain — Verifikationsschicht zwischen L1 und Anwendungen.

**Project:** atc-zkp
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Proprietary (ATC-LIC)`

## Overview

ATC ZKP Layer stellt die kanonische Zero-Knowledge-Proof-Infrastruktur für das A-TownChain-Ökosystem bereit. Es bildet die Verifikationsschicht zwischen L1 und dezentralen Anwendungen.

## Purpose

ATC ZKP Layer bietet die kanonische Implementierung der Zero-Knowledge-Proof-Systeme innerhalb des A-TownChain-Ökosystems. Es ist verantwortlich für:
- On-Chain Proof Verification Engine (zkp-verifier)
- Off-Chain Proof Generation (zkp-prover)
- Circuit Registry & Versionierung (zkp-circuits)
- Commitment & Nullifier Management (zkp-crypto)

## Status

**Status:** `development`

- Stand: R1-Skeleton (ATC-STD-201) — Struktur, Governance und Standards definiert.
- Qualitätssicherung: Kryptografie ist S4-kritisch — G18 Security-Audit vor Freeze (AD-023), Trusted-Setup-Kriterien (ZKP-010).

## Architecture

ATC ZKP Layer basiert auf einer schichtenbasierten Verifikationsarchitektur.

### Components

| Component | Purpose | Required |
|---|---|---|
| `zkp-core` | Kern-Typen, ProofSystem-Trait, Registry-Interfaces | Yes |
| `zkp-verifier` | On-Chain-Verifikations-Engine | Yes |
| `zkp-prover` | Off-Chain Proof-Generator | Yes |
| `zkp-circuits` | Circuit Registry, Versionierung & Compiler-Bindung | Yes |
| `zkp-crypto` | Commitments, Nullifier, Pedersen/Merkle-Primitiv & SHA-256 | Yes |
| `zkp-vm` | ZKVM: ATCLang-Bytecode → Execution Trace → Proof | Yes |
| `zkp-sdk` | Entwickler-API für dApps und Wallet | Yes |

### Data Flow

```text
dApps / Wallet / GameFi / DeFi
          │
          ▼
   ZKP Application / SDK Layer
          │
          ▼
   ZKP-Layer (Proof Generator, Circuit Registry, Verification Engine)
          │
          ▼
   ATC Cryptographic Core (SHA-256)
          │
          ▼
   A-TownChain L1 (Consensus, State, Tx)
```

Pluggable Proof Architecture: **Groth16 · PLONK · Halo2 · STARK** + zukünftige Systeme — austauschbar ohne Blockchain-Umbau (ZKP-002).

## Features

- Pluggable Proof Architecture (Groth16, PLONK, Halo2, STARK).
- On-Chain Verifikation für ATCLang Smart Contracts.
- Offene Test-Vektor-Generierung und Validierung für Proof-Systeme.
- Integrated ZKVM for Bytecode Verification.

## Repository Structure

```text
atc-zkp/
├── crates/
│   ├── zkp-circuits/
│   ├── zkp-core/
│   ├── zkp-crypto/
│   ├── zkp-prover/
│   ├── zkp-sdk/
│   ├── zkp-verifier/
│   └── zkp-vm/
└── docs/
```

## Requirements

- Rust `1.75+` / Cargo
- OpenSSL / Cryptographic Libraries
- ATCLang Toolchain

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-zkp.git
cd atc-zkp
cargo build
```

## Configuration

Die Konfiguration erfolgt über `Cargo.toml` sowie Laufzeitparameter für Prover und Verifier.

## Usage

```rust
// Beispiel zur Initialisierung der ZKP Verification Engine
use zkp_core::ProofSystem;

fn main() {
    println!("ATC ZKP Layer Initialized");
}
```

## Development

```bash
cargo build --all-targets
```

## Testing

```bash
cargo test
```
Erwartetes Ergebnis: `PASS` (alle Unit- und Integrationstests erfolgreich).

## Security

Sicherheitsrelevante Befunde dürfen NICHT öffentlich gemeldet werden. Bitte melden Sie Schwachstellen direkt gemäß dem offiziellen ATC Security Reporting Prozess (ATC-STD-203) und [SECURITY.md](SECURITY.md).

## Documentation

- [Architecture Overview](docs/ZKP_ARCHITECTURE.md)
- [Repository Standard](docs/REPOSITORY_STANDARD.md)
- [Architecture Details](ARCHITECTURE.md)

## Governance

Dieses Repository unterliegt dem A-TownChain Enterprise Governance Framework. Review- und Freigabepflichten gemäß ATC-STD-000 §9.

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.2.0 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |
| ATC-STD-201 | 1.0.0 | ✅ |
| ATC-STD-202 | 1.0.0 | ✅ |
| ATC-STD-203 | 1.0.0 | ✅ |

## Roadmap

Die Roadmap ist kanonisch in [ROADMAP.md](ROADMAP.md) dokumentiert.
Aktuelle Aufgabe: Erstellung und Validierung offener Test-Vektoren für die Pluggable Proof Architecture.

## Contributing

Beiträge folgen den Regeln in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Proprietaer — All Rights Reserved (ATC-LIC). Siehe [LICENSE](LICENSE).

## Maintainers

A-TownChain Core Cryptography Team / ShivaCore.

## Repository Metadata

<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-ZKP-001
  name: atc-zkp
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: S4
  criticality: high
-->
