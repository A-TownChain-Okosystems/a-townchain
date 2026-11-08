# Repository Knowledge

Current GitHub inventory for A-TownChain-Okosystems: 32 repositories.

| Repository | Intended domain |
|---|---|
| .github | Organization-wide GitHub configuration and reusable governance/CI assets |
| a-townchain-ecosystem | Integration/assembly repository for the ecosystem |
| a-townchain | Blockchain core |
| a-townchain-os | A-TownChain OS / system monorepo |
| a-townchain-os-docs | Canonical documentation and knowledge hub |
| atclang | ATCLang language |
| atc-shivacore | ShivaCore Rust/no_std capability microkernel |
| atc-standards | Standards and governance SSOT |
| atc-engineering | Engineering, governance and evidence tooling |
| atc-node | Node/network layer |
| atc-vm | ATC virtual machine |
| atc-sdk | SDK |
| atc-wallet | Wallet |
| atc-contracts | Contracts |
| atc-zkp | Zero-knowledge proofs |
| atc-algorithm | Consensus/economic algorithms; canonical monetary schedule |
| atc-storage | Storage |
| atc-indexer | Indexer |
| atc-explorer | Explorer |
| atc-interop | Interoperability |
| atc-oracle | Oracle services |
| atc-compute | Compute |
| atc-mining | Mining/reward integration |
| atc-marketplace | Marketplace/DEX domain |
| atc-launchpad | Launchpad |
| aurora-ai | Aurora AI platform |
| globus-os | GlobusOS |
| genesis-engine | Genesis Engine |
| genesis-chronicles | Genesis Chronicles |
| genesis-franchise-factory | Genesis franchise tooling |
| atc-ide | ATC IDE |
| demo-repository | Private/demo repository |

## Status discipline

The table above records repository role, not implementation completeness.

For each repository, future entries should capture:

- canonical purpose
- key modules
- public APIs/interfaces
- upstream/downstream dependencies
- tests
- CI workflows/gates
- E2E paths
- security controls
- open defects
- evidence links
- current evidence state

## Canonical ownership rules

- Monetary emission/supply schedule: atc-algorithm::economics
- Standards/governance SSOT: atc-standards
- Ecosystem integration: a-townchain-ecosystem
- OS documentation/knowledge hub: a-townchain-os-docs

When two repositories contain competing implementations of the same invariant, record the duplication and resolve it through evidence rather than silently selecting one.
