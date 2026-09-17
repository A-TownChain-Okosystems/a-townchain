# ATC Oracle Services

> **ATC COMPLIANCE: R1** — auditiert am 2026-09-10 (SCR-0075; R-Level aus `.atc/repository.yaml`).


> Oracle-Schicht für externe Daten — schließt die identifizierte Lücke Oracle & External Data Binding in der A-TownChain.

**Project:** atc-oracle
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Apache-2.0 (ATC-LIC)`

## Overview

ATC Oracle Services stellt die verlässliche Anbindung externer Datenquellen (Preise, Blockchain-Daten, Off-Chain-Events) an die A-TownChain bereit.

## Purpose

ATC Oracle Services bietet die kanonische Oracle- und Data-Binding-Implementierung im A-TownChain-Ökosystem. Es ist verantwortlich für:
- Bereitstellung fälschungssicherer Preissignale (price/ ATC/USD, ETH/USD, BTC/USD)
- Integration externer Datenquellen & Data Feeds mit KI-Attestations
- Verzahnung mit dem Smart Contract Oracle Standard ATC-STD-SC-016 (Staleness, Deviation, Fallback, Emergency)
- On-Chain Zeitstempel-Verifizierung via Median Time Past (ATC-10)

## Status

**Status:** `development`

- Stand: Vertikales Produkt-Repo angelegt gemäß AD-024 (06.09.2026).
- Rebuild-Ära: Grundstruktur definiert; qualitätsgetriebener Rebuild ohne fixen Mainnet-Termin (AD-023).

## Architecture

atc-oracle ist als modulare Oracle-Schicht (L5) konzipiert.

### Components

| Component | Purpose | Required |
|---|---|---|
| `price/` | Preisfutter (ATC/USD, ETH/USD, BTC/USD) | Yes |
| `external-data/` | Externe Daten- & Blockchain-Bindung | Yes |
| `feeds/` | Oracle Feed Management & Modul-Adapter | Yes |
| `verification/` | KI-Attestations & Security Verification | Yes |
| `sc-016-binding` | SC-016 Smart Contract Oracle Protocol Binding | Yes |

### Data Flow

```text
External Data Sources / APIs / Blockchains
                 │
                 ▼
   Oracle Adapters & Price Feeds (price/, external-data/)
                 │
                 ▼
   Verification & Security Layer (AI Attestations, Median Time Past)
                 │
                 ▼
   ATC Smart Contract Protocol (ATC-STD-SC-016 Contract Interface)
                 │
                 ▼
   A-TownChain L1 Execution Environment (ATVM)
```

## Features

- Multi-Source Price Aggregation mit Median-Filterung.
- SC-016-Verzahnung: Regelung von Staleness-Schwellwerten, Deviation-Toleranzen, Fallback-Quellen und Emergency-Pausing.
- AI-Attestations zur Validierung unstrukturierter Datenfeeds.
- Modulare Einbindung via Monorepo-Workspace (`scripts/sync_modules.py`).

## Repository Structure

```text
atc-oracle/
├── docs/
└── tests/
```

## Requirements

- Rust `1.75+` / Cargo
- ATCLang Toolchain (ATC-99)
- Python `3.10+` (für Tooling / Sync)

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-oracle.git
cd atc-oracle
cargo build
```

## Configuration

Die Konfiguration der Feeds und SC-016 Parameter erfolgt über Umgebungsvariablen und konfigurierbare YAML-Specs.

## Usage

```rust
// Beispiel für Oracle Price Fetching
fn main() {
    println!("ATC Oracle Services Active");
}
```

## Development

```bash
cargo build
```

## Testing

```bash
cargo test
```
Erwartetes Ergebnis: `PASS` (alle Tests gemäß Testplan bestanden).

## Security

Sicherheitsrelevante Befunde dürfen NICHT öffentlich gemeldet werden. Bitte melden Sie Schwachstellen direkt gemäß dem offiziellen ATC Security Reporting Prozess (ATC-STD-203) und [SECURITY.md](SECURITY.md).

## Documentation

- [Repository Standard](docs/REPOSITORY_STANDARD.md)
- [Test Plan](tests/TESTPLAN.md)
- [Architecture](ARCHITECTURE.md)

## Governance

Dieses Repository folgt dem A-TownChain Enterprise Governance Framework (ATC-STD-000). Review- und Approval-Pflicht für sicherheits- und orakelkritische Änderungen.

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |
| ATC-STD-SC-016 | 1.0.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.2.0 | ✅ |
| ATC-STD-203 | 1.0.1 | ✅ |

## Roadmap

Die Meilenstein-Planung ist in [ROADMAP.md](ROADMAP.md) hinterlegt. Vollständige Oracle-Dienste sind für Meilenstein M6 vorgesehen.

## Contributing

Beiträge folgen den Regeln in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache-2.0 — Apache-2.0, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC). Siehe [LICENSE](LICENSE).

## Maintainers

A-TownChain Oracle Team / ShivaCoreDev.

## Repository Metadata

<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-ORACLE-001
  name: atc-oracle
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: S2
  criticality: medium
-->
