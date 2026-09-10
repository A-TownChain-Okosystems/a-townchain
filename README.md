# ATC Mining

> Mining-Stack für Rewards und Emissionen im A-TownChain-Ökosystem mit SC-018-Verzahnung und MinerWatcherGPT-Anbindung.

**Project:** atc-mining
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Apache-2.0 — A-TownChain-Okosystems`

<!-- atc metadata block (ATC-STD-README-001 §14) -->
<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-MINING-001
  name: atc-mining
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

## Overview

`atc-mining` bildet den spezialisierten Mining-Stack des A-TownChain-Ökosystems. Es trennt die hardwarenahe Hashing- und Node-Logik strikt vom Konsensus-Core (`a-townchain` / `atc-algorithm`), um den Core regeldefinierend und nicht hardwaretreibend zu halten. Das Repository implementiert Mining-Rewards, Emissionen, Double-Claim-Schutz und Event-Emittierung gemäß ATC-STD-SC-018 zur Echtzeitüberwachung durch MinerWatcherGPT.

## Purpose

ATC Mining stellt die kanonische Implementierung der Mining-Dienste (L5) im A-TownChain-Ökosystem bereit. Es ist verantwortlich für:
- Implementierung von Hashing-Algorithmen (SHA3-ATC / PoW-Infrastruktur) für GPU-, CPU- und Mobile-Mining.
- Berechnung und Abwicklung von Block-Rewards und Emissionsplänen in Verzahnung mit Smart Contracts nach ATC-STD-SC-018.
- Absicherung von Reward-Claims durch automatischen Double-Claim-Schutz.
- Emittierung von Audit- und Mining-Events nach ATC-STD-SC-008 zur Anbindung von MinerWatcherGPT für Telemetrie und Forensik.

## Status

**Status:** `development`

Das Repository befindet sich in der aktiven Entwicklung (L5, Roadmap-Meilenstein M6). Es ist strukturell mit dem Governance-Framework (ATC-STD-201/202/203) synchronisiert.

## Architecture

### Components
- `miner/`: Hardware-Treiber und Hashing-Engines (CPU/GPU/Mobile).
- `algorithms/`: Implementierung der ATC-Hashing-Algorithmen (SHA3-ATC).
- `mining-manager/`: Orchestrierung von Jobs, Work-Distributor und Difficulty-Tracking.
- `network/`: Netzwerk-Schnittstellen und Protokoll-Handler.
- `telemetry/`: Event-Emittierung an MinerWatcherGPT (ATC-STD-SC-008 / SC-018).

### Data Flow
1. **Job Distribution**: Der Blockchain-Core (`a-townchain`) liefert aktuelle Block-Header und Difficulty-Vorgaben an den `mining-manager`.
2. **Execution**: Der `miner` führt Hashing aus und meldet gültige Nonces/Proofs zurück.
3. **Reward Claim**: Gefundene Blöcke lösen Transaktionen gegen SC-018-Mining-Contracts aus.
4. **Monitoring**: Telemetrie-Events fließen in Echtzeit an MinerWatcherGPT.

### Dependencies

| Component | Purpose | Required |
|---|---|---|
| `a-townchain` | Blockchain-Core, State & Tx-Orchestrierung | Ja |
| `atc-algorithm` | Konsensus- und Difficulty-Regeln (PoH+PoS+PoW) | Ja |
| `atc-contracts` | SC-018 Mining Contracts & Event-Interfaces | Ja |
| MinerWatcherGPT | KI-Service für Mining-Monitoring & Forensik | Ja |

## Features

- Entkoppelte Mining-Architektur (Core bleibt regel-definierend).
- SC-018-konforme Emissionsrechner und Reward-Klassen.
- Schutz vor Double-Claiming bei Mining-Rewards.
- MinerWatcherGPT-Event-Schnittstellen für Telemetrie und Anomalieerkennung.

## Repository Structure

```text
atc-mining/
├── docs/       System- und Architektur-Dokumentation
└── tests/      Testpläne und Testsuiten
```

## Requirements

- **Rust**: `>= 1.75.0`
- **Cargo**: `>= 1.75.0`
- **Python**: `>= 3.10` (für Governance-Tools & Agenten-Scripts)

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-mining.git
cd atc-mining
cargo build --release
```

## Configuration

Die Konfiguration erfolgt über Umgebungsvariablen oder `config/mining.toml`:

```toml
[mining]
algorithm = "sha3-atc"
threads = 4
node_url = "http://localhost:8545"

[telemetry]
minerwatcher_enabled = true
event_target = "minerwatcher-gpt"
```

## Usage

Starten eines Mining-Knotens im lokalen Modus:

```bash
./target/release/atc-mining --config config/mining.toml --node http://localhost:8545
```

## Development

Entwicklungs-Workflow folgt ATC-STD-000 und ATC-STD-201:
- Commits im Conventional-Commit-Format (`feat:`, `fix:`, `docs:`).
- Trailer: `[agent: aurora-base44-superagent-6a2756186106d6f0fbb105b5]`.
- Naming gemäß ATC-STD-000 §7.

## Testing

Ausführung der vollständigen Test-Suite:

```bash
cargo test --all
```

Erwartetes Ergebnis: `PASS` (alle Unit- und Integrationstests erfolgreich).

## Security

Sicherheitsrelevante Schwachstellen dürfen **nicht öffentlich** als GitHub Issue gemeldet werden. Bitte beachten Sie den offiziellen ATC-Security-Reporting-Prozess gemäß [SECURITY.md](SECURITY.md) und ATC-STD-203.
Kritizität: `medium`, Security-Klasse: `S2`.

## Documentation

Vertiefende Dokumentation liegt unter `docs/`:
- [Architecture](ARCHITECTURE.md) — Detaillierte Systemarchitektur
- [Repository Standard](docs/REPOSITORY_STANDARD.md) — Governance-Standards
- [Roadmap](ROADMAP.md) — Entwicklungsstufen M1–M8
- [Status](STATUS.md) — Aktueller Projekt- und Gate-Status

## Governance

Dieses Repository unterliegt dem A-TownChain Enterprise Governance Framework (ATC-STD-000, ATC-STD-201).
Architekturentscheidungen und API-Änderungen erfordern Review- und Approval-Prozesse gemäß SCR-Verfahren.
Ownership: `ShivaCoreDev` / `aurora-superagent`.

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.2.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.0.0 | ✅ |
| ATC-STD-203 | 1.0.0 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |
| ATC-STD-SC-018 | 1.0.0 | ✅ |

## Roadmap

Die Meilensteine richten sich nach der kanonischen Lauffähigkeits-Roadmap (M1–M8, AD-027). Details finden Sie in [ROADMAP.md](ROADMAP.md) sowie im kanonischen Development Management.

## Contributing

Beiträge zum Repository richten sich nach den Richtlinien in [CONTRIBUTING.md](CONTRIBUTING.md) sowie dem Verhaltenskodex in [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## License

Apache-2.0 — Apache-2.0, A-TownChain-Okosystems. Siehe [LICENSE](LICENSE).

## Maintainers

- **ShivaCoreDev** — Primary Maintainer
- **aurora-superagent** — AI Agent Maintainer (`6a2756186106d6f0fbb105b5`)

## Repository Metadata

<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-MINING-001
  name: atc-mining
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

## AI Agent Instructions

Für KI-Agenten gelten die verbindlichen Instruktionen in [AGENTS.md](AGENTS.md).
Pflichteinstieg: `README.md` -> `STATUS.md` -> `ARCHITECTURE.md` -> `ROADMAP.md` -> `CHANGELOG.md`.
