# Genesis Chronicles

> Premium-Flagship-/Reference-Game auf der ATC Genesis Engine. Das Spiel demonstriert die Fähigkeiten der Engine; spielspezifische Logik bleibt in diesem Repository.

**Project:** `genesis-chronicles`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`  
**Version:** siehe `CHANGELOG.md` / Repository-Metadaten  
**License:** `Apache-2.0`

## Overview

Genesis Chronicles ist das Referenzspiel auf `genesis-engine`. Es nutzt die generischen Engine-Funktionen, ergänzt diese aber um spielspezifische Systeme wie Gameplay, Battle, Breeding und NFT-Minting.

Das Repository hieß historisch `shivamon`. Die Migration zu Genesis Chronicles ist eine kontrollierte Namensänderung; historische Bezeichner und IDs werden nicht stillschweigend neu nummeriert.

Die Engine-/Game-Grenze ist verbindlich:

- **Genesis Engine:** generische Game-Development-Technologie
- **Genesis Chronicles:** konkrete Spiel- und Produktlogik
- **A-TownChain / ATC-VM:** Chain-seitige Ausführung und deterministische Zustandsübergänge
- **ShivaCore:** Kernel-/Capability-Basis, nicht Game-Logik

## Purpose

Genesis Chronicles ist das Premium-Flagship-/Reference-Game und dient als konkrete Referenzimplementierung für die Genesis Engine.

Verantwortlichkeiten dieses Repositories umfassen insbesondere:

- Spiel-Loop und Gameplay-Systeme
- Battle- und Creature-/Breeding-Logik, soweit spielspezifisch
- NFT-bezogene Spielintegration und Mint-Transaktionen
- Integration der generischen `genesis-engine`-Module
- Referenzimplementierung für die Engine-Produktstrategie

Generische Engine-Funktionen werden nicht dauerhaft in diesem Repository dupliziert. Neue generische Features durchlaufen das definierte Feature-Promotion-Gate der Genesis Engine.

## Status

**Status:** `development` — Rebuild-Basis. Der dokumentierte M7-Meilenstein ist ein Entwicklungsziel und darf nicht mit `APPROVED`, `AUDITED` oder `PRODUCTION_READY` gleichgesetzt werden.

Die tatsächliche Readiness wird anhand reproduzierbarer Evidence, Tests und der geltenden Release-Gates festgestellt. Ein deklarierter Meilenstein oder ein Audit-Maturity-Level allein ist kein Production-Release.

## Architecture

### Components

- `atc-shivamon` — historischer bzw. bestehender Modulname im Repository-Kontext; bei weiteren Umbenennungen sind Legacy-Referenzen zu erhalten.
- `atc-game` — spielspezifische Game-Funktionalität.
- `genesis-engine` — externe generische Engine-Abhängigkeit für Engine- und Simulationsfunktionen.

### Data Flow

```text
Player / Game Events
        ↓
Game Systems
        ↓
Genesis Engine / ECS
        ↓
Game State
        ↓
Chain Integration (falls erforderlich)
        ↓
A-TownChain / ATC-VM
```

### Chain Boundary

On-Chain-Funktionen werden nicht durch die Game-Engine ersetzt. ATCLang und ATC-VM bilden die definierte Ausführungsgrenze für Chain-seitige Logik; Rust-basierte Infrastruktur trägt die Chain-Komponenten. Das Spiel integriert diese Systeme über explizite Schnittstellen.

### Chain Identity

Für Chain-Interaktionen ist ausschließlich die kanonische Chain-Identity-/Network-Konfiguration des jeweiligen Zielnetzwerks maßgeblich. Chain IDs dürfen nicht aus historischen README-Angaben oder impliziten Defaults abgeleitet werden.

## Features

- Premium-Flagship-/Reference-Game für Genesis Engine
- Spiel- und Simulationsintegration
- Spielspezifische Gameplay-Systeme
- NFT-/Chain-Integration
- Vorbereitung auf reproduzierbare Game-State- und Mint-Transaktionsnachweise

## Repository Structure

```text
/
├── docs/          # Dokumentation und Entscheidungen
├── modules/       # Game-Module
│   ├── atc-shivamon
│   └── atc-game
├── CHANGELOG.md
├── ROADMAP.md
├── STATUS.md
└── README.md
```

Historische `shivamon`-Referenzen sind bei Migrationen kontrolliert zu behandeln und dürfen nicht ohne Governance-Entscheidung entfernt oder umnummeriert werden.

## Requirements

- Rust/Cargo gemäß den Workspace-Anforderungen
- Git >= 2.30
- Weitere Anforderungen entsprechend den jeweiligen Modulen und der Engine-Integration

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/genesis-chronicles.git
cd genesis-chronicles
cargo build --workspace
```

## Usage

Der konkrete Start-/Spiel-Workflow wird durch die aktuellen Module und `STATUS.md` bzw. `ROADMAP.md` definiert. Für einen reproduzierbaren Entwicklungscheck:

```bash
cargo test --workspace
```

## Development

Entwicklung erfolgt nach den geltenden A-TownChain-Governance- und Repository-Standards. Commits müssen dem Conventional-Commit-Modell entsprechen.

Spielspezifische Änderungen bleiben in diesem Repository. Änderungen, die generische Engine-Funktionalität betreffen, müssen gegen die Genesis-Engine-Produktstrategie geprüft werden.

## Testing

```bash
cargo test --workspace
```

Für Chain-/NFT-Funktionen sind zusätzlich reproduzierbare Evidence für die relevanten Transaktionen und Zustandsübergänge erforderlich.

## Security

Sicherheitslücken dürfen nicht öffentlich über GitHub Issues gemeldet werden. Bitte den Security-Reporting-Prozess aus `SECURITY.md` verwenden.

## Documentation

- `STATUS.md` — aktueller Entwicklungsstatus
- `ROADMAP.md` — Entwicklungs-Roadmap
- `CHANGELOG.md` — Änderungen und Releases
- `docs/` — technische Dokumentation und Entscheidungen
- `genesis-engine` — generische Engine-Dokumentation
- `a-townchain-os-docs` — zentrale Ökosystem-Dokumentation

## Governance

Das Repository folgt dem A-TownChain-Governance-Modell. Architektur-, Sicherheits- und Release-Entscheidungen müssen den vorgesehenen Review- und Approval-Prozess durchlaufen.

Canonical Standard-IDs werden ausschließlich über die Standards Registry und den Governance-Prozess vergeben. Die aktuelle Taxonomie verwendet Family-scoped IDs der Form `ATC-STD-Fxx-yyy`; Legacy-IDs bleiben historisch erhalten und werden nicht stillschweigend umnummeriert.

## Standards & Compliance

Relevante Governance-Grundlagen umfassen insbesondere:

| Standard | Version | Verwendung |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | Governance Root |
| ATC-STD-README-001 | 1.0.0 | README-Struktur und Metadaten |
| ATC-STD-MD-001 | 1.0.0 | Markdown-Konformität |
| ATC-STD-203 | 1.0.1 | Security und Release Gates |

Diese Referenzen dokumentieren die anzuwendenden Governance-Grundlagen und stellen keine pauschale `PRODUCTION_READY`-Behauptung dar.

## Roadmap

Kanonische Quellen:

- `ROADMAP.md`
- `STATUS.md`
- zentrale Roadmap in `a-townchain-os-docs`
- GitHub Issues & Projects

## Contributing

Beiträge erfolgen über den definierten ATC-Governance-Prozess. Relevante Tests und Validatoren müssen vor einem Merge erfolgreich sein.

## License

Apache-2.0 — A-TownChain-Okosystems. Details siehe [`LICENSE`](LICENSE).

## Maintainers

**Organization:** A-TownChain-Okosystems

## AI Agent Instructions

Für KI-Agenten:

1. Lies `STATUS.md`, `ROADMAP.md`, `ARCHITECTURE.md` (falls vorhanden) und relevante Governance-Dokumente vor größeren Änderungen.
2. Beachte die Engine-/Game-Grenze.
3. Verwende Conventional Commits.
4. Führe `cargo test --workspace` und die relevanten Validatoren aus.
5. Behaupte `AUDITED`, `APPROVED` oder `PRODUCTION_READY` nur bei vorhandener, autoritativer Evidence.
