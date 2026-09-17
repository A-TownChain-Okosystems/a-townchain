# ATC A-TownChain

> **ATC COMPLIANCE: R4 · Standard ATC-STD-201 v1.0.1 · GATE: AUDITED (09.09.2026, Score 94/100) · README: ATC-STD-README-001 CONFORM**

> A-TownChain is the canonical chain/orchestration layer of the A-TownChain ecosystem. **This repository is not production-ready and has no approved Mainnet date.**

**Project:** a-townchain  
**Organization:** A-TownChain-Okosystems  
**Status:** `development`  
**Version:** `0.1.0`  
**License:** `Apache-2.0`

---

## Overview

A-TownChain (`a-townchain`) bildet die kanonische Orchestrierungs- und Integrationsschicht der Blockchain-Architektur.

**Readiness:** Die Evidence-SSOT klassifiziert das Repository als **partially implemented prototype / M4 integration evidence** und ausdrücklich **nicht production-ready**. Es existiert kein freigegebener Mainnet-Termin.

**Execution boundary:** ATCLang ist die on-chain Sprache, ATC-VM (ATVM) die deterministische Ausführungsgrenze und `a-townchain` die Chain-/Node-Orchestrierung. Rust trägt die chain-kritische Infrastruktur.

**Consensus boundary:** Die kanonische Konsenslogik liegt in `atc-algorithm`; `a-townchain` darf keine konkurrierende Legacy-Konsensimplementierung als kanonisch behandeln.

**M4:** Integrations-Evidence für 2-Node-Gossip-Sync, Chain-ID 658467 und ATCLang/ATVM-Flows existiert. M4 ist **kein** Mainnet- oder Production-Readiness-Nachweis.

---

## Purpose

A-TownChain stellt die kanonische Referenz-Orchestrierung der Blockchain bereit.

- **On-chain:** ATCLang-Verträge werden über ATVM ausgeführt.
- **Chain-bearing infrastructure:** Rust implementiert Node-, Netzwerk-, State- und Integrationskomponenten.
- **Kernel boundary:** `atc-shivacore` stellt die Kernel-/TCB-Schicht bereit.
- **Consensus:** `atc-algorithm` ist die kanonische Konsenskomponente.
- **Governance:** Änderungen an normativen Standards und kritischen Architekturentscheidungen folgen dem A-TownChain-Governance-Modell.

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

```text
ATCLang
   │ on-chain source
   ▼
ATC-VM / ATVM
   │ deterministic execution boundary
   ▼
a-townchain
   │ chain / node / state orchestration
   ├── atc-algorithm       canonical consensus
   ├── atcnet              P2P networking
   ├── atc-zkp             ZKP integration
   └── atc-governance      governance integration
   │
   ▼
atc-shivacore             kernel / TCB boundary
```

### Components

1. **`atc-blockchain`**: Core State Machine, Block-Erstellung, Transaktions-Pool, State DB.
2. **`atcnet`**: P2P-Netzwerkschicht (Gossip Protocol, Node Discovery, Peer Bootstrap).
3. **`atc-zkp`**: Zero-Knowledge Proof Schaltung und Verifikation.
4. **`atc-governance`**: On-Chain-Abstimmung, Vorschläge, Timelock und Treasury-Verwaltung.
5. **`atc-dns`**: On-Chain Domain Name System und dezentrale Namensauflösung.
6. **`atc-testnet`**: Testnetzwerk-Launcher, Simulation und Node-Konfiguration.

### Data Flow

```text
[ATCLang Contract] -> [ATVM] -> [State Transition] -> [Consensus Interface] -> [Block Commitment]
                                      ▲
                                      │
                              [a-townchain / atcnet]
```

## Chain Identity

- Current development Chain-ID: `658467`.
- Chain identity is governed by the canonical Chain Identity specification.
- Network/environment identity MUST NOT be inferred from repository names, URLs or deployment labels.
- Legacy identifiers remain immutable during standards migration; no silent renumbering or reuse is permitted.

## Repository Structure

a-townchain ist das kanonische **Blockchain-Monorepo** des A-TownChain-Stacks (MIGRATION_MANIFEST.yaml). Der Kern liegt im Root; alle Blockchain-Komponenten sind unter `components/` integriert (volle Git-Historie, Import 2026-09-17, SCR-0126):

| Komponente | Pfad | Rolle |
|---|---|---|
| **Core** | `/` (Root) | L1 Protocol — Konsens-Kern, Ketten-Logik, Governance |
| atc-node | `components/node` | Node Runtime |
| atc-algorithm | `components/algorithm` | Consensus/Algorithm |
| atc-vm | `components/vm` | ATC Virtual Machine |
| atc-contracts | `components/contracts` | Smart Contracts |
| atc-sdk | `components/sdk` | Developer SDK / CLI |
| atc-wallet | `components/wallet` | Wallet |
| atc-indexer | `components/indexer` | Blockchain Indexing |
| atc-explorer | `components/explorer` | Blockchain Explorer |
| atc-mining | `components/mining` | Mining/Validation |
| atc-zkp | `components/zkp` | Zero-Knowledge Proofs |
| atc-interop | `components/interop` | Interoperability |
| atc-oracle | `components/oracle` | Oracle Infrastructure |
| atc-storage | `components/storage` | Blockchain Storage |


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

## Usage

Starten einer Test-Node-Instanz im Testnet-/Development-Modus:

```bash
python3 modules/atcnet/node.py --chain-id 658467 --port 8333
```

## Development

- Conventional Commits.
- Modul-Synchronisation über den Monorepo-/Workspace-Kontext.
- Naming und Governance gemäß `ATC-STD-000` und den jeweils geltenden Standards.
- Neue Family-scoped Standard-IDs werden ausschließlich über den kanonischen Registry-/Governance-Prozess vergeben.

## Testing

```bash
pytest modules/atcnet/tests
```

Testergebnisse gelten nur zusammen mit dem zugehörigen Evidence-Bundle als Release-Nachweis.

## Security

Sicherheitsrelevante Hinweise werden gemäß **ATC-STD-203** behandelt. Production-Readiness bleibt `NO-GO`, solange das Security-Gate nicht auf `audited` steht.

## Governance

Änderungen an Konsens-, Schnittstellen- oder Sicherheitsmodulen unterliegen dem A-TownChain Governance Framework. Konsensentscheidungen werden in `atc-algorithm` spezifiziert und dort eingefroren; `a-townchain` implementiert die Orchestrierung dagegen nicht als zweite kanonische Konsensquelle.

Die Family-scoped Standard-ID-Architektur verwendet `ATC-STD-F{family_id}-{sequence}`. Die Migration bestehender Legacy-IDs erfolgt kontrolliert; Legacy-IDs werden nicht still umnummeriert, wiederverwendet oder gelöscht.

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
