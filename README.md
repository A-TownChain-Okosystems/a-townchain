# ATC Chain Indexer & Analytics

> **ATC COMPLIANCE: R2** — auditiert am 2026-09-10 (SCR-0075; R-Level aus `.atc/repository.yaml`).


> ATC Indexer — Chain-Indexing & Analytics für die A-TownChain.

**Project:** atc-indexer
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Apache-2.0 (ATC-LIC)`

## Overview

ATC Indexer stellt die zentrale Indexierungs- und Analyse-Infrastruktur für das A-TownChain-Ökosystem (Chain-ID 658467) bereit.

## Purpose

ATC Indexer ist verantwortlich für die strukturierte Erfassung, Transformation und Analyse von Blockchain-Daten im A-TownChain-Ökosystem (Layer L5). Es ist verantwortlich für:
- High-Performance Chain Indexing für Blöcke, Transaktionen und Events
- Analytics-Pipelines & Metrik-Verarbeitung (Modul `atc-analytics`)
- Datenbereitstellung für Blockchain Explorer, Dashboards und Drittsysteme
- Aggregation von On-Chain und Off-Chain Entwicklungsmetriken

## Status

**Status:** `development`

- Stand: Vault-Restauration (07.09.2026, AD-020/026/027) aus Wiki-Vault restauriert.
- Compliance-Level: R2 — auditiert am 2026-09-07. Meilenstein M6 (Dienste laufen).

## Architecture

ATC Indexer folgt einer ereignisgesteuerten Pipeline-Architektur für Blockchain-Analytik.

### Components

| Component | Purpose | Required |
|---|---|---|
| `atc-analytics` | Analytics Core, Pipelines, Dashboards & Metrics | Yes |
| `pipelines/` | ETL Data Ingestion & Transformation Pipelines | Yes |
| `metrics/` | Aggregations- & Metrik-Engine | Yes |
| `chain/` | On-Chain Event Listener & Block Ingestion | Yes |

### Data Flow

```text
A-TownChain Node (Chain-ID 658467) -> Chain Ingestion -> ETL Pipelines -> Analytics DB -> Explorer / API
```

## Features

- Real-Time Block & Transaction Ingestion.
- Customizable Analytics Pipelines for Token Transfers & Smart Contract Events.
- Dashboard-Visualisierung (Vite / TypeScript Frontend Integration).
- Automated Reporting & Benchmark Suite.

## Repository Structure

```text
atc-indexer/
├── docs/
├── modules/
│   └── atc-analytics/
└── tests/
```

## Requirements

- Node.js `18+` / npm
- TypeScript `5+`
- Rust `1.75+` (für Native Ingestor Modules)

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-indexer.git
cd atc-indexer
cd modules/atc-analytics
npm install
```

## Configuration

Die Konfiguration der Endpunkte und Datenbanken erfolgt über `modules/atc-analytics/vite.config.ts` und Umgebungsvariablen.

## Usage

```bash
cd modules/atc-analytics
npm run dev
```

## Development

```bash
npm run build
```

## Testing

```bash
npm test
```
Erwartetes Ergebnis: `PASS` (alle Indexer- und Analytics-Tests erfolgreich).

## Security

Sicherheitsrelevante Befunde dürfen NICHT öffentlich gemeldet werden. Bitte melden Sie Schwachstellen direkt gemäß dem offiziellen ATC Security Reporting Prozess (ATC-STD-203) und [SECURITY.md](SECURITY.md).

## Documentation

- [Analytics Architecture](modules/atc-analytics/ARCHITECTURE.md)
- [Repository Standard](docs/REPOSITORY_STANDARD.md)
- [Test Plan](tests/TESTPLAN.md)
- [Architecture Details](ARCHITECTURE.md)

## Governance

Dieses Repository folgt dem A-TownChain Enterprise Governance Framework (ATC-STD-000). Review- und Approval-Pflicht für alle konsensus- und indexierungsrelevanten Schnittstellen.

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

Die Entwicklungsplanung ist in [ROADMAP.md](ROADMAP.md) und [modules/atc-analytics/ROADMAP.md](modules/atc-analytics/ROADMAP.md) hinterlegt. Ziel: Meilenstein M6 (Dienste laufen).

## Contributing

Beiträge folgen den Regeln in [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Apache-2.0 — Apache-2.0, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC). Siehe [LICENSE](LICENSE).

## Maintainers

A-TownChain Indexing & Analytics Team / ShivaCoreDev.

## Repository Metadata

<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-INDEXER-001
  name: atc-indexer
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: TypeScript
governance:
  security_class: S2
  criticality: medium
-->
