# ATC Launchpad

> **Historical governance snapshot:** R1 was recorded in an audit on 2026-09-10 (SCR-0075). This historical label is not current production-readiness or exact-SHA verification; consult `STATUS.md` and current evidence.


> Launchpad für Token- und NFT-Releases auf der A-TownChain.

**Project:** atc-launchpad
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Apache-2.0`

---

## Overview

ATC Launchpad ist das vertikale Produkt-Repository (Layer L5) der A-TownChain-Architektur (Chain-ID 658467). Es bietet die kanonischen Module für die Erstellung, Durchführung und Verwaltung von Token- und NFT-Releases, Presales, Whitelists sowie zeitgestaffelten Vesting-Plänen auf der A-TownChain.

## Purpose

ATC Launchpad bietet die kanonische Implementierung von Token- und NFT-Launchpad-Diensten innerhalb des A-TownChain-Ökosystems. Es ist zuständig für:

- **Token & NFT Presales:** Durchführung von Fair-Launches und Presales für Ökosystem-Projekte.
- **Vesting & Whitelisting:** Sichere Verwaltung von Freischaltungsplänen und Inhaber-Verifizierungen.
- **Integration:** Bereitstellung von Schnittstellen für `atc-contracts` und `atc-wallet`.
- **Standards-Konformität:** Sichere Ausführung auf Basis verifizierter ATVM-Smart-Contracts.

## Scope

- **In Scope:** Token-Launchpad-Logik, Presale-Verträge, Vesting-Mechanismen, Whitelist-Management und Integrationstests.
- **Out of Scope:** Core-Blockchain-Konsensus (liegt in `atc-node` / `a-townchain`), Token-Wallet-UI (liegt in `atc-wallet`).

## Status

**Status:** `development` — Erstinitialisierung gemäß AD-024. Die Grundstruktur steht bereit; die vertiefende Implementierung erfolgt qualitätsgetrieben gemäß AD-023.

## Architecture

### Components

- **Sales Module (`sales/`):** Abwicklung von Token-Presales, Fair-Launches und Direct Asset Distribution.
- **Vesting Module (`vesting/`):** Logik für lineare und meilensteinbasierte Token-Freischaltungen.
- **Whitelist Module (`whitelist/`):** Verwaltung von Berechtigungslisten und Proof-Validierungen.
- **Integration Layer:** Anbindung an `atc-contracts` (Smart Contract Templates) und `atc-wallet`.

### Data Flow

```text
[User / Developer] -> [Launchpad UI / SDK] -> [Sales & Whitelist Module]
                                                     |
                                                     v
                                           [atc-contracts / ATVM]
                                                     |
                                                     v
                                          [atc-node / Blockchain]
```

### Dependencies

| Component | Purpose | Required |
|---|---|---|
| atc-contracts | Smart-Contract-Standards und Vorlagen | Ja |
| atc-wallet | Wallet-Integration und Transaktionssignierung | Ja |
| atc-node | Node-RPC und Blockchain-Interaktion | Ja |

## Features

- Token- und NFT-Launchpad-Infrastruktur
- Konfigurierbare Presale- und Fair-Launch-Optionen
- Whitelist-Verwaltung und Inhaber-Validierung
- Vesting-Zeitpläne für Projekt-Token
- CI-gestützte Governance- und Standards-Prüfung

## Repository Structure

```text
├── docs/            # Projektdokumentation und Standards
└── tests/           # Testpläne und Testsuiten
```

## Requirements

- Rust 1.75+ (oder Cargo Build toolchain)
- Node.js 18+ (für Frontend / Tooling, falls zutreffend)
- Git 2.30+

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-launchpad.git
cd atc-launchpad
cargo build --release
```

## Configuration

Die Konfiguration erfolgt über Umgebungsvariablen oder Konfigurationsdateien im jeweiligen Deployment-Modul:

- `ATC_CHAIN_ID`: Chain-ID (Standard: `658467`)
- `ATC_NODE_RPC`: RPC-Endpunkt der A-TownChain Node

## Usage

```bash
cargo run --bin atc-launchpad -- --config config.toml
```

## Development

Entwicklungs-Richtlinien:
- ATCLang & Rust-first Prinzipien gemäß AD-021/AD-022.
- Commits im Format Conventional Commits mit verpflichtender Agent-Signatur: `[agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5]`.
- Keinesfalls Code ohne zugehörige Tests committen.

## Testing

Die Testsuite wird wie folgt ausgeführt:

```bash
cargo test
```

Erwartetes Ergebnis: **PASS** (alle Unit- und Integrations-Tests erfolgreich).

## Security

Security issues **must NOT** be disclosed publicly via GitHub Issues. Security vulnerabilities must be reported directly through the official ATC security reporting process.

- Report to: **ShivaCoreDev / ATC Security Team** (verbindlich gemäß ATC-STD-203).
- Weitere Details siehe [`SECURITY.md`](SECURITY.md).

## Documentation

- Einstieg und Spezifikation: [`docs/REPOSITORY_STANDARD.md`](docs/REPOSITORY_STANDARD.md)
- Architektur: [`ARCHITECTURE.md`](ARCHITECTURE.md)
- Projekt-Status: [`STATUS.md`](STATUS.md)
- Agenten-Instruktionen: [`AGENTS.md`](AGENTS.md)
- Governance-Wiki: A-TownChain Docs Hub

## Governance

Dieses Repository folgt dem A-TownChain Enterprise Governance Framework (ATC-STD-000 v1.3.0):
- Architekturentscheidungen werden im zentralen `DECISIONS_REGISTER` (AD-024, AD-026) gepflegt.
- Sicherheits- und konsensusrelevante Änderungen erfordern ein formelles Review und Owner-Freigabe (§9).

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | ✅ APPROVED |
| ATC-STD-README-001 | 1.0.0 | ✅ APPROVED |
| ATC-STD-MD-001 | 1.0.0 | ✅ APPROVED |
| ATC-STD-201 | 1.0.1 | ✅ APPROVED |
| ATC-STD-202 | 1.2.0 | ✅ APPROVED |
| ATC-STD-203 | 1.0.1 | ✅ APPROVED |

## Roadmap

Verbindliches Tracking erfolgt in den kanonischen Quellen:
- Kanonische Roadmap: [`ROADMAP.md`](ROADMAP.md)
- Task-Tracking: GitHub Issues & Projects
- Lauffähigkeits-Roadmap M1-M8 gemäß AD-027.

## Contributing

Beiträge sind willkommen. Bitte beachten Sie die Richtlinien in [`CONTRIBUTING.md`](CONTRIBUTING.md).

## License

Dieses Projekt ist unter der **Apache-2.0**-Lizenz lizenziert — siehe [`LICENSE`](LICENSE).

## Maintainers

- **Organization:** A-TownChain-Okosystems
- **Owner:** Michael Wroblewski (ShivaCoreDev)
- **Operative Agent:** Aurora (Base44 Superagent `6a2756186106d6f0fbb105b5`)

## AI Agent Instructions

Siehe [`AGENTS.md`](AGENTS.md) für ausführliche Instruktionen für KI-Agenten.

## Changelog

Siehe [`CHANGELOG.md`](CHANGELOG.md) für die Historie aller Versionen und Änderungen.

## Repository Metadata

```yaml
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-LAUNCHPAD
  name: atc-launchpad
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: S1
  criticality: low
```
