# atc-node

> **Role:** Full-Node-Binary und Runtime für eine A-TownChain-Netzwerkinstanz. `atc-node` betreibt den Node; die kanonische Chain-/Protocol-Semantik bleibt in den dafür zuständigen Komponenten.

**Project:** `atc-node`  
**Organization:** `A-TownChain-Okosystems`  
**Status:** `development`  
**Security class:** `S4`

## Purpose

`atc-node` ist das Distribution- und Laufzeitziel für den Betrieb einer A-TownChain-Node-Instanz. Es verbindet Protokoll-Core, VM, Networking, Storage und Node-Lifecycle zu einem ausführbaren Dienst.

`atc-node` definiert **nicht** eigenständig die Chain-Semantik. Es baut auf den kanonischen Komponenten auf, insbesondere:

- `a-townchain` — Chain-Protokoll / Core
- `atc-algorithm` — Konsens-/Algorithmus-Komponenten, soweit vom aktuellen Protokoll vorgesehen
- `atc-vm` — Contract-/VM-Ausführung
- Node-spezifische Netzwerk-, Storage- und Lifecycle-Komponenten in diesem Repository

## Architecture

```text
a-townchain
  │  protocol / state-transition rules
  ▼
atc-node
  ├── P2P / Networking
  ├── RPC
  ├── Storage
  ├── Runtime / lifecycle
  ├── configuration
  └── validator integration
       │
       ▼
   real network instance
```

Die Chain-Identität wird nicht durch eine README-Zahl festgelegt. Chain ID, Network ID, Genesis Identity und Environment gehören zur kanonischen Chain-Identity-/Network-Konfiguration.

## Status

`development` bezeichnet den aktuellen Entwicklungszustand. Aussagen wie `APPROVED`, `AUDITED` oder einzelne Roadmap-Milestones sind keine automatische Aussage über `PRODUCTION_READY`.

Das Repository befindet sich im qualitätsorientierten Rebuild. Vorhandene historische AD-/Vault-Dokumentation kann als Entscheidungs- bzw. Migrationskontext dienen, ersetzt aber keine aktuelle Implementierungs-Evidence.

## Repository Rules

1. `atc-node` ist die kanonische Quelle für Node-Runtime-Code; ein Monorepo darf diesen Code nur kontrolliert integrieren.
2. Keine neue Node-Funktion ohne passende Tests und aktuelle Governance-Evidence.
3. Protocol-Semantik nicht duplizieren oder lokal widersprüchlich definieren.
4. Keine ungeprüften Mainnet-/Production-Termine im README.
5. Architektur- und Sicherheitsänderungen über den geltenden ATC-Governance-Prozess führen.

## Repository Structure

```text
.
├── .atc/          # ATC repository metadata
├── .github/       # CI / automation
├── docs/          # Documentation
├── modules/       # Node modules, where present
├── tests/         # Tests, where present
├── AGENTS.md      # Agent instructions, where present
├── ARCHITECTURE.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── SECURITY.md
└── STATUS.md
```

The current source tree and Cargo manifests are authoritative for exact module names and build targets.

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/atc-node.git
cd atc-node
cargo build --workspace
```

## Usage

Use the current CLI/help output and repository documentation for the supported commands. Typical lifecycle operations include initialization, start, status, synchronization and validator operation where implemented.

```bash
cargo run -- --help
```

Do not assume a command is production-ready solely because it is documented here.

## Testing

```bash
cargo test --workspace
```

CI and the current commit determine the authoritative pass/fail state.

## Security

`atc-node` exposes a network attack surface and may handle validator/network credentials. Follow [`SECURITY.md`](SECURITY.md) and never publish sensitive credentials or vulnerability details in public issues.

## Governance

The repository follows `ATC-STD-000` and the canonical ATC standards registry. Family-scoped standard IDs use `ATC-STD-F{family}-{sequence}`. Existing legacy IDs remain immutable historical references until explicitly migrated through governance.

`APPROVED`, `IMPLEMENTED`, `AUDITED`, and `PRODUCTION_READY` are separate lifecycle/evidence states.

## Documentation

Before substantial changes, inspect:

- `AGENTS.md`
- `STATUS.md`
- `ROADMAP.md`
- `ARCHITECTURE.md`
- applicable standards in `atc-standards`
- the central governance/decision documentation where referenced by this repository

## License

See [`LICENSE`](LICENSE) for the authoritative license terms.
