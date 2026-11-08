# A-TownChain Repository Fleet Inventory

Last synchronized: 2026-09-16

This inventory is the audit scope discovered through the connected GitHub account. It is a tracking artifact, not a claim that every repository currently passes the engineering gate.

| Repository | Domain | Audit state |
|---|---|---|
| `.github` | Organization governance | DISCOVERED |
| `a-townchain` | Blockchain / L1 | DISCOVERED |
| `a-townchain-os` | L7 domain integration / OS | DISCOVERED |
| `a-townchain-os-docs` | Documentation / knowledge base | DISCOVERED |
| `atc-standards` | Normative standards / SSOT | DISCOVERED |
| `atc-engineering` | Engineering control plane | SELF-AUDITING |
| `atclang` | Language / compiler | DISCOVERED |
| `atc-shivacore` | Kernel | DISCOVERED |
| `atc-vm` | Virtual machine | DISCOVERED |
| `atc-node` | Node runtime | DISCOVERED |
| `atc-contracts` | Smart contracts | DISCOVERED |
| `atc-sdk` | SDK | DISCOVERED |
| `atc-wallet` | Wallet | DISCOVERED |
| `atc-explorer` | Explorer | DISCOVERED |
| `atc-indexer` | Indexing | DISCOVERED |
| `atc-mining` | Mining | DISCOVERED |
| `atc-interop` | Interoperability | DISCOVERED |
| `atc-oracle` | Oracle | DISCOVERED |
| `atc-storage` | Storage | DISCOVERED |
| `atc-launchpad` | Launchpad | DISCOVERED |
| `atc-marketplace` | Marketplace | DISCOVERED |
| `atc-compute` | Compute | DISCOVERED |
| `atc-algorithm` | Algorithms | DISCOVERED |
| `atc-zkp` | Zero-knowledge proofs | DISCOVERED |
| `atc-ide` | Development environment | DISCOVERED |
| `aurora-ai` | AI platform | DISCOVERED |
| `globus-os` | Operating system | DISCOVERED |
| `genesis-engine` | Game engine | DISCOVERED |
| `genesis-chronicles` | Game / content | DISCOVERED |
| `genesis-franchise-factory` | Game production tooling | DISCOVERED |
| `demo-repository` | Private/demo | AUTHENTICATED AUDIT REQUIRED |

## Audit dimensions

Every repository is intended to pass the following lifecycle:

1. Inventory and architecture boundary
2. Syntax and static validation
3. Logic and function validation
4. Unit/integration/regression tests
5. Security and secret detection
6. Dependency and supply-chain review
7. Connectivity and interface validation
8. Documentation and implementation consistency
9. Duplicate/orphan/obsolete file review
10. Completeness and missing-component analysis
11. Remediation
12. Re-test and evidence capture

A repository remains **NOT_READY** while unresolved findings exist or executable evidence is missing.
