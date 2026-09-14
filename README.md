# ATC A-TownChain

> **ATC COMPLIANCE: R4 · Standard ATC-STD-201 v1.0.1 · GATE: AUDITED (09.09.2026, Score 94/100) · README: ATC-STD-README-001 CONFORM**

> A-TownChain Blockchain L3 — Chain-ID 658467, orchestration layer. **Consensus is canonical in `atc-algorithm`; this repository is not production-ready and has no approved Mainnet date.**

**Project:** a-townchain  
**Organization:** A-TownChain-Okosystems  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0`

---

## Overview

A-TownChain (`a-townchain`) bildet die kanonische Orchestrierungs- und Integrationsschicht der Layer-3-Blockchain-Architektur.

**Readiness:** Die Evidence-SSOT klassifiziert das Repository als **partially implemented prototype / M4 integration evidence** und ausdrücklich **nicht production-ready L1**. Es existiert kein freigegebener Mainnet-Termin.

**Consensus boundary:** Die kanonische Konsenslogik liegt in `atc-algorithm`; `a-townchain` darf keine konkurrierende Legacy-Konsensimplementierung als kanonisch behandeln.

**Module:** `atc-blockchain`, `atcnet`, `atc-zkp`, `atc-governance`, `atc-dns`, `atc-testnet`.

**M4:** Integrations-Evidence für 2-Node-Gossip-Sync, Chain-ID 658467 und ATCLang/ATVM-Flows existiert. M4 ist **kein** Mainnet- oder Production-Readiness-Nachweis.

---

## Purpose

A-TownChain stellt die kanonische Referenz-Orchestrierung der Layer-3-Blockchain bereit.

- **Problemstellung:** Integration von Netzwerk, Mempool, Konsensschnittstelle, VM und State-Commitment.
- **Einsatzgebiet:** Ausführung von Smart Contracts (ATCLang auf ATVM), dezentrales Namensregister, ZKP-Integration und On-Chain Governance.
- **Abhängigkeiten:** `atc-shivacore` (Kernel/L1), `atc-algorithm` (kanonischer Konsens), `atc-vm` (Ausführung).

## Status

**Status:** `development`

- **Implementation:** `partial` — prototypischer Blockchain-Kern.
- **Maturity:** M4-Integrationsstand mit Evidence, nicht production-ready.
- **Security:** `not_audited` für den Production-Release-Gate.
- **Conformance:** `not_verified`.
- **Release:** `development`.
- **Mainnet:** **NO-GO** bis alle verbindlichen Evidence-Gates erfüllt und auditiert sind.

Die maschinenlesbare Wahrheit liegt in `.atc/evidence/evidence.yaml`. Claims in dieser README ersetzen keine Evidence.

## Architecture

Die Architektur basiert auf einer modular getrennten Systemstruktur.

### Components

1. **`atc-blockchain`**: Core State Machine, Block-Erstellung, Transaktions-Pool, State DB.
2. **`atcnet`**: P2P-Netzwerkschicht (Gossip Protocol, Node Discovery, Peer Bootstrap).
3. **`atc-zkp`**: Zero-Knowledge Proof Schaltung und Verifikation.
4. **`atc-governance`**: On-Chain-Abstimmung, Vorschläge, Timelock und Treasury-Verwaltung.
5. **`atc-dns`**: On-Chain Domain Name System und dezentrale Namensauflösung.
6. **`atc-testnet`**: Testnetzwerk-Launcher, Simulation und Node-Konfiguration.

### Data Flow

```text
[Transactions] -> [atcnet Gossip] -> [Mempool] -> [Canonical Consensus Interface] -> [ATVM State Update] -> [Block Commitment]
```

### Component Dependencies

| Component | Purpose | Required |
|---|---|---|
| `atc-shivacore` | Kernel & Crypto Primitives | Yes |
| `atc-algorithm` | Canonical Consensus | Yes |
| `atcnet` | P2P Network Propagation | Yes |
| `atc-zkp` | ZK Proof Verification | Yes |
| `atc-governance` | On-Chain Governance | Optional |
| `atc-dns` | Name Resolution | Optional |

## Features

- Chain-ID: `658467`.
- ATCLang VM (ATVM) integration boundary.
- P2P gossip and node orchestration.
- ZKP verification integration.
- Governance and DNS integration points.
- **Consensus:** interface/orchestration only; canonical consensus implementation is `atc-algorithm` and remains subject to its P0 specification, implementation, security and conformance gates.

## Repository Structure

```text
a-townchain/
├── docs/
├── modules/
└── .atc/evidence/
```

## Requirements

- **Rust:** 1.70+
- **Python:** 3.10+
- **Cargo / Make / Docker:** für Modul-Builds und Test-Stacks

## Installation

```bash
git clone https://github.com/A-TownChain-Okosystems/a-townchain.git
cd a-townchain
pip install -r modules/atcnet/requirements.txt
```

## Configuration

- `CHAIN_ID`: 658467
- `LISTEN_PORT`: 8333
- `RPC_PORT`: 8545

## Usage

Starten einer Test-Node-Instanz im Testnet-/Development-Modus:

```bash
python3 modules/atcnet/node.py --chain-id 658467 --port 8333
```

## Development

- Conventional Commits.
- Modul-Synchronisation über den Monorepo-/Workspace-Kontext.
- Naming gemäß `ATC-STD-000`.

## Testing

```bash
pytest modules/atcnet/tests
```

Testergebnisse gelten nur zusammen mit dem zugehörigen Evidence-Bundle als Release-Nachweis.

## Security

Sicherheitsrelevante Hinweise werden gemäß **ATC-STD-203** behandelt. Production-Readiness bleibt `NO-GO`, solange das Security-Gate nicht auf `audited` steht.

## Governance

Änderungen an Konsens-, Schnittstellen- oder Sicherheitsmodulen unterliegen dem A-TownChain Governance Framework. Konsensentscheidungen werden in `atc-algorithm` spezifiziert und dort eingefroren; `a-townchain` implementiert die Orchestrierung dagegen nicht als zweite kanonische Konsensquelle.

## Standards & Compliance

| Standard | Version | Compliance |
|---|---:|---|
| ATC-STD-000 | 1.3.0 | ✅ |
| ATC-STD-README-001 | 1.0.0 | ✅ |
| ATC-STD-MD-001 | 1.0.0 | ✅ |
| ATC-STD-201 | 1.0.1 | ✅ |
| ATC-STD-202 | 1.2.0 | ✅ |
| ATC-STD-203 | 1.0.1 | ✅ |

## Roadmap

- Konsens-Refactor auf `atc-algorithm`.
- End-to-End-Conformance ATCLang → VM → Contract → Node → Chain.
- Security-Audit.
- Reproducible Build.
- Release erst nach vollständiger Evidence und Auditierung.

**Kein Mainnet vor Erfüllung aller verbindlichen Gates.**

## License

Apache-2.0 — Details siehe [LICENSE](LICENSE).

## Maintainers

- **Organisation:** A-TownChain-Okosystems
- **Lead Maintainer:** ShivaCoreDev
- **Automation Maintainer:** aurora-superagent

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
