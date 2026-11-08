<!-- atc metadata block (ATC-STD-README-001 §14) -->
<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-GAME-001
  name: genesis-engine
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
  secondary_language: Python
-->

# ATC Genesis Engine

[![ATC-COMPLIANCE](https://img.shields.io/badge/ATC-COMPLIANCE-v1.0-green)](./AGENTS.md)

> General-purpose, modulare Game-Engine für ECS, Weltensimulation, Rendering, Physik, Audio, Animation, Networking, AI, Editor und Franchise-Production-Workflows.

**Project:** `genesis-engine`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0 — A-TownChain-Okosystems`

## Overview

Genesis Engine ist die **general-purpose Game-Development-Plattform** des A-TownChain-Ökosystems. Die Engine stellt wiederverwendbare Runtime-, Simulations-, Tooling-, SDK- und Produktionsfunktionen bereit. `genesis-chronicles` ist ein unabhängiger Consumer/Flagship-Titel und keine technische Voraussetzung für die Engine.

Die kanonische Cargo-Workspace-Struktur umfasst Runtime-, Simulation-, Rendering-, AI-, Networking-, Editor-, Build- und SDK-Module. Zusätzlich ist die **Genesis Franchise Factory** als integrierter Produktionssubsystem-Bereich unter `franchise_factory/` Bestandteil dieses Repositories.

## Genesis Franchise Factory

Die Franchise Factory ist jetzt im Genesis-Engine-Repository verankert und umfasst die vollständige Orchestrierungsbasis:

- 17 AI-Produktions-Workflows
- Franchise Core / AD-20 Pipeline
- DAO-Modell / ATC-9900
- Lifecycle Manager / AD-43
- Typed Artifact + Provenance/Evidence Contracts
- Game Factory Dependency Graph mit Build, QA und LiveOps
- Python-Referenzimplementierung und integrierte Regressionstests
- eigene Python-Quality-Gates in GitHub Actions

```text
franchise_factory/
├── gff/
│   ├── core.py
│   ├── dao.py
│   ├── lifecycle.py
│   ├── spec_loader.py
│   ├── workflows.py
│   ├── artifacts.py
│   └── game_factory.py
├── tests/
├── specs/
└── docs/
```

Die Factory sitzt innerhalb der Engine-Plattform, ohne `genesis-chronicles` zur technischen Abhängigkeit zu machen. Provider, externe Side Effects und Chain-/VM-Anbindungen bleiben explizite Integrationsgrenzen und werden nicht als implementiert ausgegeben, solange keine Evidence vorliegt.

Weitere Details: [`franchise_factory/README.md`](franchise_factory/README.md).

## Purpose

Genesis Engine ist für die generische Laufzeit- und Simulationsinfrastruktur der Genesis-Spielplattform verantwortlich:

- Engine-Lifecycle und Runtime-Orchestrierung
- ECS-basierte Simulation
- Welt-, Chunk- und Streaming-Infrastruktur
- Gameplay-Systeme
- Rendering, Physik, Audio und Animation
- Input und UI
- AI- und Networking-Integration
- Editor- und Entwicklerwerkzeuge
- SDK- und Build-/Packaging-Infrastruktur
- Franchise- und Game-Production-Orchestrierung über die integrierte Franchise Factory

Spielspezifische Logik gehört in das jeweilige Spiel-Repository. Generische Features werden nur über den vorgesehenen Feature-Promotion-Prozess in die Engine übernommen.

## Architecture

### Dependency direction

```text
Platform primitives
       ↓
 ECS / World / Input / Assets
       ↓
Physics / Animation / Audio / Gameplay
       ↓
AI / Network / Renderer / UI
       ↓
     Runtime
       ↓
 Editor / Tools / CLI / Build / SDK
       ↓
 Franchise Factory / Production Orchestration
```

### Ecosystem Boundary

```text
ATCLang / ATC-VM / A-TownChain
              │
              │ Chain- und On-Chain-Funktionen
              ▼
       Genesis Integration
              │
              ▼
       Genesis Engine
        ┌─────┴─────┐
        │           │
 Franchise      Runtime/Editor
 Factory             │
        │            ▼
        └──────► Games / Franchises
```

Die Engine ist keine Blockchain, kein Kernel und kein Ersatz für ATC-VM oder ShivaCore. Chain-seitige Zustandsübergänge und Contracts bleiben an der vorgesehenen Chain-/VM-Grenze.

## Determinism

Deterministische Simulation ist ein explizites Engine-Ziel. Für deterministische Pfade müssen ECS-Iteration, RNG/Seeds, Gameplay-State, Physik, Streaming, Netzwerk-Ticks, Serialisierung und Replay/State-Verification kontrolliert werden.

## AI Boundary

Externe oder asynchrone AI darf den deterministischen Simulationszustand nicht direkt verändern:

```text
AI Inference
    ↓
Validated Command
    ↓
Deterministic Simulation
    ↓
State Change
```

Die Franchise Factory folgt demselben Prinzip: Workflows sind provider-neutral; AI-Provider und externe Aktionen werden über injizierte Executor-Grenzen angeschlossen.

## Requirements

- Rust >= 1.75 / Cargo
- Python >= 3.11 für Franchise-Factory-Tooling und vorhandenes Tooling
- Git >= 2.30

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/genesis-engine.git
cd genesis-engine
cargo build --workspace
cd franchise_factory
python -m pip install -e '.[dev]'
pytest -q
```

## Testing

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps

cd franchise_factory
ruff check gff tests
pytest -q
```

Testergebnisse sind Evidence. Ein erfolgreicher Testlauf bedeutet nicht automatisch `AUDITED` oder `PRODUCTION_READY`.

## Documentation

- `ARCHITECTURE.md` — technische Architektur
- `STATUS.md` — aktueller Projektstatus
- `ROADMAP.md` — Entwicklungs-Roadmap
- `franchise_factory/README.md` — integrierte Franchise Factory
- `franchise_factory/gff/` — Factory Runtime/Orchestration
- `docs/ENGINEERING_AUDIT.md` — Audit-Baseline
- `docs/ENGINEERING_AUDIT_FINDINGS.md` — Findings

## Governance

Das Repository folgt dem A-TownChain-Governance-Modell und den geltenden ATC-Standards. Änderungen an Factory-, Engine-, Chain- oder VM-Grenzen müssen nachvollziehbar dokumentiert und durch Tests/Evidence belegt werden.

## License

Apache-2.0 — A-TownChain-Okosystems. Details siehe [`LICENSE`](LICENSE).

## AI Agent Instructions

1. Lies `STATUS.md`, `AGENT_MANIFEST.md`, `ARCHITECTURE.md` und `ROADMAP.md` vor größeren Änderungen.
2. Beachte die geltenden ATC-Standards und Repository-Governance.
3. Verwende Conventional Commits.
4. Führe Engine- und Franchise-Factory-Tests aus.
5. Trenne deklarierte Zustände, Testergebnisse und Governance-Evidence strikt voneinander.
6. Verändere keine Chain-/VM-Grenzen, um Engine-Funktionalität zu implementieren.


## Compliance

<!-- ATC compliance badge -->
![ATC COMPLIANCE](https://img.shields.io/badge/ATC%20COMPLIANCE-R3%20%C2%B7%20ATC--STD--201%2F202%2F203-brightgreen)

## Architecture
## Installation
## Usage
## Configuration
## Development
## Testing
## Security
## Governance
## Contributing
## License
## Support
