# ATC A-TownChain

> **ATC COMPLIANCE: R4 · Standard ATC-STD-201 v1.0.0 · GATE: AUDITED (09.09.2026, Score 94/100) · README: ATC-STD-README-001 CONFORM**

> A-TownChain Blockchain L3 — Chain-ID 658467, PoW+PoS+PoH, ZKP, Governance, DNS, Testnet (AD-012: Kernel-System-Service).

**Project:** a-townchain
**Organization:** A-TownChain-Okosystems
**Status:** `development`
**Version:** `0.1.0`
**License:** `Apache-2.0`

---

## Overview

A-TownChain (`a-townchain`) bildet das kanonische Layer-3-Blockchain-Fundament der A-TownChain-Ökosystem-Architektur.

**Vault-Restauration (07.09.2026, AD-020/026/027):** Inhalt aus dem Wiki-Vault (`docs/archive/monorepo-full/`) restauriert — vor der Repo-Leerung byte-identisch gesichert. Chain-ID 658467 im Vault-Stand (24 Dateien verifiziert).

**Module:** `atc-blockchain`, `atcnet`, `atc-zkp`, `atc-governance`, `atc-dns`, `atc-testnet`.

**Meilenstein (AD-027):** M4 — Blockchain läuft: 2 Nodes Gossip-Sync, Transaktion validiert, Genesis Chain-ID 658467, ATCLang-Contract auf ATVM.

---

## Purpose

A-TownChain stellt die kanonische Referenz-Implementierung der Layer-3-Blockchain im A-TownChain-Ökosystem bereit.

- **Problemstellung:** Bereitstellung eines dezentralen, performanten und kryptografisch gesicherten State Machine Konsensus ohne Abhängigkeiten von externen Drittanbieter-Blockchains oder POSIX-Forks.
- **Einsatzgebiet:** Ausführung von Smart Contracts (ATCLang auf ATVM), Dezentrales Namensregister (ATC-DNS), Privatsphäre via Zero-Knowledge Proofs (ATC-ZKP) und On-Chain Governance.
- **Abhängigkeiten:** Baut auf `atc-shivacore` (Kernel/L1) auf und bedient Oberflächen/Dienste wie `globus-os`, `genesis-engine` und `genesis-chronicles`.

---

## Status

**Status:** `development`

- **Compliance Level:** R4 (auditiert 2026-09-07 via `atc-repo-audit`).
- **Security Class:** S4 (Critical Infrastructure).
- **Lauffähigkeits-Stufe:** Meilenstein M4 erreicht.

---

## Architecture

Die Architektur basiert auf einer modular getrennten Systemstruktur.

### Components

1. **`atc-blockchain`**: Core State Machine, Block-Erstellung, Transaktions-Pool, State DB.
2. **`atcnet`**: P2P-Netzwerkschicht (Gossip Protocol, Node Discovery, Peer Bootstrap).
3. **`atc-zkp`**: Zero-Knowledge Proof Schaltung und Verifikation (Groth16/Plonk Gadgets).
4. **`atc-governance`**: On-Chain-Abstimmung, Vorschläge, Timelock und Treasury-Verwaltung.
5. **`atc-dns`**: On-Chain Domain Name System und Dezentrale Namensauflösung.
6. **`atc-testnet`**: Testnetzwerk-Launcher, Simulations-Skripte und Node-Konfigurationen.

### Data Flow

```text
[Transactions] -> [atcnet Gossip] -> [Mempool] -> [ShivaConsensus Validation] -> [ATVM State Update] -> [Block Commitment]
```

### Component Dependencies

| Component | Purpose | Required |
|---|---|---|
| `atc-shivacore` | Kernel & Crypto Primitives | Yes |
| `atcnet` | P2P Network Propagation | Yes |
| `atc-zkp` | ZK Proof Verification | Yes |
| `atc-governance` | On-Chain Governance | Optional |
| `atc-dns` | Name Resolution | Optional |

---

## Features

- **ShivaConsensus:** Hybrider Konsens aus PoW (SHA3-ATC), PoS (reputation-weighted) und PoH (VDF-basiert).
- **Chain-ID:** 658467.
- **Token Standards:** ATC-8300 (Fungible) und ATC-9000 (Non-Fungible / Multi-Token).
- **Kryptografie:** ECDSA secp256k1 & ZKP Circuit Validation.
- **ATCLang VM (ATVM):** Deterministische Ausführungsumgebung für Smart Contracts.

---

## Repository Structure

```text
a-townchain/
├── docs/
└── modules/
```

- `docs/`: Technische Dokumentation und Architektur-Standards (`REPOSITORY_STANDARD.md`).
- `modules/`: Quellcode-Module (`atc-blockchain`, `atcnet`, `atc-zkp`, `atc-governance`, `atc-dns`, `atc-testnet`).

---

## Requirements

- **Rust:** 1.70+
- **Python:** 3.10+ (für Netzwerkskripte und tooling)
- **Cargo / Make / Docker:** für Modul-Builds und Test-Stacks

---

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain.git
cd a-townchain
pip install -r modules/atcnet/requirements.txt
```

---

## Configuration

Die Konfiguration der Blockchain-Knoten erfolgt über Umgebungsvariablen oder Konfigurationsdateien in `modules/atc-testnet/config/`:

- `CHAIN_ID`: 658467
- `LISTEN_PORT`: 8333
- `RPC_PORT`: 8545

---

## Usage

Starten einer Test-Node-Instanz im Testnet-Modus:

```bash
python3 modules/atcnet/node.py --chain-id 658467 --port 8333
```

---

## Development

- **Commit-Konvention:** Conventional Commits (`feat:`, `fix:`, `docs:`, `security:`).
- **Modul-Synchronisation:** Abgleich über Monorepo-Workspace (`a-townchain-os`).
- **Naming:** Vorgaben gemäß `ATC-STD-000` §7.

---

## Testing

Ausführung der Modul-Testsuite für Netzwerkschicht und Konsens:

```bash
pytest modules/atcnet/tests
```

Erwartetes Ergebnis: **PASS** (100% erfolgreiche Testabdeckung der P2P-Tests).

---

## Security

Sicherheitsrelevante Hinweise:

- **Security Reporting:** Sicherheitsrisiken und Vulnerabilities werden NICHT öffentlich als Issue gemeldet, sondern direkt an den Owner (ShivaCoreDev) über den offiziellen Security-Reporting-Prozess nach **ATC-STD-203** übermittelt (nicht öffentlich / not publicly disclosed).
- **Security Level:** S4 (Core Infrastructure Security Requirements).

---

## Documentation

- [STATUS.md](STATUS.md) — Aktueller Projektstatus
- [ARCHITECTURE.md](ARCHITECTURE.md) — Detaillierte Architekturübersicht
- [ROADMAP.md](ROADMAP.md) — Entwicklungs-Roadmap M1-M8
- [AGENTS.md](AGENTS.md) — Instruktionen für KI-Agenten
- [docs/REPOSITORY_STANDARD.md](docs/REPOSITORY_STANDARD.md) — Repository Compliance Standard

---

## Governance

Änderungen an Konsens-, Schnittstellen- oder Sicherheitsmodulen unterliegen dem **A-TownChain Enterprise Governance Framework**. Alle architektonischen Entscheidungen werden im zentralen `DECISIONS_REGISTER` erfasst (AD-Nummern verbindlich). Review- und Approval-Pflicht durch `ShivaCoreDev`.

---

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.2.0 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.0.0 | ✅ |
| ATC-STD-203 | 1.0.0 | ✅ |

---

## Roadmap

Die Roadmap orientiert sich an der Lauffähigkeits-Roadmap M1-M8 (AD-027). Details siehe [ROADMAP.md](ROADMAP.md).
Tracked in GitHub Projects & Issues.

---

## Contributing

Beiträge zur Entwicklung folgen den Regeln in [CONTRIBUTING.md](CONTRIBUTING.md).

---

## License

Apache-2.0 — Apache-2.0, Michael Wroblewski / ShivaCore / A-TownChain-Okosystems (ATC-LIC/ATS-LIC). Details siehe [LICENSE](LICENSE).

---

## Maintainers

- **Organisation:** A-TownChain-Okosystems
- **Lead Maintainer:** ShivaCoreDev
- **Automation Maintainer:** aurora-superagent

---

## Repository Metadata

```yaml
atc:
  standard: ATC-STD-README-001
  version: 1.0.0
repository:
  id: ATC-REPO-ATC
  name: a-townchain
  type: software
  status: development
ownership:
  organization: A-TownChain-Okosystems
technology:
  primary_language: Rust
governance:
  security_class: S4
  criticality: critical
```
