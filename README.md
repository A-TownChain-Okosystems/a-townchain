# ATC Algorithm

> ATC-Algorithmus — Propriethärer Hybrid Consensus der A-TownChain (PoH + PoS + PoW).

**Project:** atc-algorithm
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Proprietary (ATC-LIC)`

## Overview

ATC Algorithm ist das zentrale Konsens-Repository des A-TownChain-Ökosystems (Chain-ID 658467). Es implementiert den proprietären Hybrid Consensus bestehend aus Proof of History (PoH), Proof of Stake (PoS) und Proof of Work (PoW). Priorität P0 (Konsens-Kern, AD-044).

Für KI-Agenten: Governance liegt zentral im Wiki-Repo `a-townchain-os-docs`: `AGENT_POLICY.md`, `AGENT_COORDINATION.md`, `DECISIONS_REGISTER.md` (insb. AD-001 SHA-256, AD-004 Chain-ID, AD-023 kein Mainnet-Termin, AD-044 dieses Repo).

## Purpose

ATC Algorithm stellt die kanonische Konsens-Logik für A-TownChain bereit. Es ist verantwortlich für:
- Kryptografische Zeitstempel-Kette (PoH) auf Basis verifizierbarer SHA-256 Ereignis-Reihenfolge (AD-001)
- Validator-Gewichtung & Staking-basierte Block-Produktion (PoS)
- Schwierigkeitsanteil und Schutz gegen Stake-Zentralisierung (PoW)
- Formale Hybrid-Auswahl, Fork-Choice-Rule und Konsens-Finalisierung

## Scope

In Scope:
- Spezifikation und Rust-Implementierung der PoH-Tick-Kette und Hybrid-Konsens-Regeln
- Konsens-Verifikation, Fuzzing und Test-Suiten für Konsens-Sicherheit
- Schnittstellen zur Block- und Transaktions-Orchestrierung

Out of Scope:
- Chain-Konsens-Orchestrierung und Block/Tx-Netzwerk-Propagation (liegt in `a-townchain`)
- Kernel-Isolation (liegt in `atc-shivacore`)
- Smart-Contract-Ausführung (liegt in `atc-vm`)

## Status

**Status:** `development`

- **Stand:** R1-Skeleton (ATC-STD-201) — Struktur + Governance stehen, Implementierung folgt im qualitätsgetriebenen Rebuild (AD-023).
- **Abgrenzung (AD-044):** `atc-algorithm` ist das führende Konsens-Repo; `a-townchain` dient als Referenz-Orchestrierung.
- **Nächste Schritte:** PoH-Spezifikation konsolidieren, Hybrid-Auswahlregeln formalisieren, Rust-Implementierung der PoH-Tick-Kette, Fuzzing.
- **Qualitäts-Gates:** Konsens = S4-kritisch (G18 Security-Audit vor Freeze; SHA-256 nach AD-001).

## Architecture

ATC Algorithm folgt einer mehrschichtigen Hybrid-Konsens-Architektur.

### Components

| Component | Purpose | Required |
|---|---|---|
| `poh` | Proof of History Zeitstempel-Kette (SHA-256 Verifizierbare Ticks) | Yes |
| `pos` | Proof of Stake Validator-Gewichtung & Staking-Logik | Yes |
| `pow` | Proof of Work Schwierigkeitsanteil gegen Stake-Zentralisierung | Yes |
| `hybrid_engine` | Hybrid-Auswahl, Fork-Choice-Rule & Finalitäts-Orchestrierung | Yes |

### Data Flow

```text
Transaktionen -> PoH (Zeitstempel-Kette, SHA-256 — AD-001)
                  |
                  v
           PoS/PoW-Hybrid-Auswahl (Validator-Gewichtung)
                  |
                  v
           Block finalisierung -> a-townchain (Chain) -> atc-vm (Contract-Ausführung)
```

## Features

- Verifizierbare Proof-of-History Zeitstempelkette (SHA-256).
- 5-Validator-Key PoS Gewichtung mit Staking-Regeln.
- Dynamische PoW Schwierigkeitsanpassung zur Verhinderung von Stake-Konzentration.
- Formale Fork-Choice-Rule und deterministisches Block-Finalitäts-Engine.

## Repository Structure

```text
atc-algorithm/
├── docs/
├── src/
└── tests/
```

## Requirements

- Rust `1.75+` (cargo, rustc)
- Python `3.10+` (für Validierungstools und Hilfsskripte)
- Git `2.30+`

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-algorithm.git
cd atc-algorithm
cargo build
```

## Configuration

Konfigurationsparameter für PoH-Tick-Frequenzen, Validator-Set und PoW-Schwierigkeit befinden sich in `src/` sowie `.atc/repository.yaml`.

## Usage

```bash
cargo run --bin atc-algorithm
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

- [Repository Standard](docs/REPOSITORY_STANDARD.md)
- [Architecture Details](ARCHITECTURE.md)
- [Project Status](STATUS.md)
- [AI Agent Instructions](AGENTS.md)

## Governance

Dieses Repository folgt dem A-TownChain Enterprise Governance Framework (ATC-STD-000). Review- und Approval-Pflicht für alle konsensuskritischen Schnittstellen (S4-kritisch).

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.2.0 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |
| ATC-STD-201 | 1.0.0 | ✅ |
| ATC-STD-202 | 1.0.0 | ✅ |
| ATC-STD-203 | 1.0.0 | ✅ |
| ATC-STD-204 | 1.0.0 | ✅ |

## Roadmap

Die Entwicklungsplanung ist in [ROADMAP.md](ROADMAP.md) hinterlegt. Ziel: Meilenstein M4 (Blockchain / Consensus Engine).

## Contributing

Beiträge folgen den Regeln in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Proprietär — All Rights Reserved, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC). Siehe [LICENSE](LICENSE).

## Maintainers

A-TownChain Consensus & Algorithm Team / ShivaCoreDev.

## Repository Metadata

<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-ALG-001
  name: atc-algorithm
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

## Changelog

Siehe [CHANGELOG.md](CHANGELOG.md) für die Änderungshistorie.
