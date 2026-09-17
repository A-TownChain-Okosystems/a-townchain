# ATC Wallet

> **ATC COMPLIANCE: R2** — auditiert am 2026-09-10 (SCR-0075; R-Level aus `.atc/repository.yaml`).


> Wallet-Kernkomponente, Key-Management und Transaktionssignierung für das A-TownChain-Ökosystem.

**Project:** atc-wallet
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `1.0.0`
**License:** `Apache-2.0 — A-TownChain-Okosystems`

<!-- atc metadata block (ATC-STD-README-001 §14) -->
<!--
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-WAL-001
  name: atc-wallet
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust / Python
governance:
  security_class: S2
  criticality: medium
-->

## Overview

ATC Wallet ist die zentrale Frontend- und Keystore-Anwendung der A-TownChain-Architektur (Layer L5). Die Komponente ist für die Schlüsselgenerierung, Adressableitung mit `ATC`-Präfix, BIP44-Derivation (`m/44'/658467'`), Transaktionssignierung und Kontoverwaltung zuständig.

Inhalt aus dem Vault wurde am 07.09.2026 (AD-020/026/027) konsistent restauriert.

## Purpose

ATC Wallet provides the canonical wallet and key management implementation within the A-TownChain ecosystem. It is responsible for:

- Sichere Generierung von A-TownChain-Adressen mit `ATC`-Präfix (32 Zeichen).
- Hierarchisch-deterministische Schlüsselableitung nach BIP44 mit Coin-Type 658467 (`m/44'/658467'`).
- Elliptic Curve Digital Signature Algorithm (ECDSA secp256k1) Signierung von Transaktionen.
- Verwahrung von Schlüsselmaterial und Schnittstellen für Guthaben-, Historien- und Faucet-Abfragen.

## Status

**Status:** `development`

Maturity: R2 (auditiert am 07.09.2026). Meilenstein-Einordnung: M6 (Dienste laufen).

## Architecture

### Components

- **Keystore & Key Derive (`src/keys.rs`, `src/crypto.py`):** Deterministische Schlüsselgenerierung (BIP44) und secp256k1-Kryptographie.
- **Wallet Core & Transaction Signer (`src/wallet.py`, `src/tx.rs`):** Erstellung, Signierung und Validierung von Transaktionen.
- **Account Services (`src/balance.rs`, `src/history.rs`):** Schnittstellen für Kontostands- und Verlaufsabfragen.
- **CLI & Module Interface (`modules/atc-wallet`):** Integration in das A-TownChain Monorepo und ATCLang Workspace.

### Data Flow

Nutzer-Eingabe → BIP44-Schlüsselableitung (`m/44'/658467'`) → Transaktionserstellung → ECDSA secp256k1 Signierung → Übertragung an API-Gateway / `atc-node`.

### Dependencies

| Component | Purpose | Required |
|---|---|---|
| atc-shivacore / atc-node | Blockchain-Konsensus und Node-Kommunikation | Yes |
| atc-contracts | Smart Contract Interfaces für Assets und Token | Yes |
| Rust Runtime / Python 3.11 | Ausführungsumgebung für Wallet-Core | Yes |

## Features

- Generierung von ATC-Adressen mit Präfix (`ATC...`)
- BIP44 Derivationspfad `m/44'/658467'`
- ECDSA secp256k1 Signierung und Verifizierung
- Kontostands- und Transaktionshistorien-Visualisierung
- Faucet- und NFT-Viewer-Einbindung

## Repository Structure

```text
/
├── .atc/
├── .github/
├── docs/
├── modules/
├── src/
├── tests/
├── wallet/
├── AGENT_MANIFEST.md
├── AGENTS.md
├── ARCHITECTURE.md
├── CHANGELOG.md
├── CODEOWNERS
├── CODE_OF_CONDUCT.md
├── COMPONENT_PLAN.md
├── CONTRIBUTING.md
├── FILE_REGISTER.md
├── GOVERNANCE.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── SECURITY.md
└── STATUS.md
```

## Requirements

- Rust >= 1.75
- Python >= 3.11
- dependencies: `ecdsa`, `pysha3`, `requests`

## Installation

``bash
git clone https://github.com/A-TownChain-Okosystems/atc-wallet.git
cd atc-wallet
pip install -r requirements.txt
cargo build --release
```

## Configuration

Die Konfiguration erfolgt über die Datei `.atc/repository.yaml` sowie Umgebungsvariablen. Die Netzwerkparameter richten sich nach Chain ID 658467.

## Usage

Nutzung der Wallet CLI und Python-Komponenten:

``bash
python3 src/wallet.py --generate-address
cargo run --bin atc-wallet
```

## Development

Entwicklung erfolgt nach den Conventional Commits Regeln und A-TownChain Governance-Standards. Modul-Synchronisation wird über `scripts/sync_modules.py` gesteuert.

## Testing

Ausführung der vollständigen Testsuite:

``bash
cargo test
pytest
```

Erwartetes Ergebnis: PASS (alle Testfälle grün).

## Security

Sicherheitsrelevante Schwachstellen werden NICHT öffentlich über GitHub Issues gemeldet. Melden Sie Sicherheitsfragen ausschließlich über den offiziellen A-TownChain Security Reporting Prozess (ATC-STD-203, `SECURITY.md`). Klassifizierung: S2.

## Documentation

Vertiefende Dokumentation ist wie folgt strukturiert:

- [ARCHITECTURE.md](ARCHITECTURE.md) — Systemarchitektur und Modulbeschreibungen
- [docs/REPOSITORY_STANDARD.md](docs/REPOSITORY_STANDARD.md) — Repository-Standards und Zweistufen-Modell
- [STATUS.md](STATUS.md) — Aktueller Entwicklungsstand
- [ROADMAP.md](ROADMAP.md) — Meilensteinplanung
- Zentales Docs-Hub: [a-townchain-os-docs](docs/REPOSITORY_STANDARD.md)

## Governance

Dieses Repository unterliegt dem A-TownChain Enterprise Governance Framework (ATC-STD-000, ATC-ENT-001..015). Architektur- und API-Änderungen erfordern Owner-Freigabe (§9) und SCR-Prozess.

## Standards & Compliance

Dieses Repository hält folgende A-TownChain-Standards ein:

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.2.0 | ✅ |
| ATC-STD-203 | 1.0.1 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |

## Roadmap

Die kanonische Roadmap ist in [ROADMAP.md](ROADMAP.md) hinterlegt. Nachverfolgung erfolgt über GitHub Issues und Development Management.

## Contributing

Beiträge müssen den Regeln in [CONTRIBUTING.md](CONTRIBUTING.md) entsprechen.

## License

Apache-2.0 — Copyright Michael Wroblewski (Org-Einheitslizenz per AD-F-046). Siehe [LICENSE](LICENSE).

## Maintainers

**Organization:** A-TownChain-Okosystems

Maintainers: ShivaCoreDev, Aurora Superagent.

## Repository Metadata

Maschinenlesbarer Metadaten-Block siehe Header (ATC-STD-README-001 §14). Registry-ID: ATC-REPO-WAL-001.
