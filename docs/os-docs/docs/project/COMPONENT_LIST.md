# A-TownChain Ecosystem — Component List

> 102 GitHub Repositories | 50 Code + 52 Wiki | 5 Languages
> Generated: 2026-08-06

## Overview

| Metric | Value |
|--------|-------|
| **Total Repos** | 102 |
| **Code Repos** | 50 |
| **Wiki Repos** | 52 |
| **Languages** | Python (18), Rust (13), TypeScript (13), HTML (2), None (4) |
| **Organization** | A-TownChain-Okosystems |

## 1. Core Infrastructure (5)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| Main Monorepo | `a-townchain-os` | HTML | 4.5MB | Unified code monorepo |
| Documentation Hub | `a-townchain-os-docs` | TypeScript | 4.0MB | Centralized documentation wiki |
| Standards Registry | `atc-standards` | — | 41KB | ATC-01–ATC-35 standards |
| SDK | `atc-sdk` | — | 9KB | Software Development Kit |
| Package Manager | `atc-atcpkg` | — | 27KB | ATC package manager |

## 2. Kernel / OS (8)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| **ShivaCore Kernel** | `atc-shivacore` | Rust | 542KB | Main Rust kernel (K29, 30 modules) |
| Generic Kernel | `atc-kernel` | Rust | 239KB | Kernel abstraction layer |
| Bootloader | `atc-bootloader` | Rust | 9KB | System boot loader |
| Drivers | `atc-drivers` | Rust | 50KB | Hardware drivers |
| DNS | `atc-dns` | Rust | 9KB | Decentralized DNS |
| Linux Edition | `atc-linux-edition` | Rust | 7KB | Linux platform support |
| Windows Edition | `atc-windows-edition` | Rust | 10KB | Windows platform support |
| GlobusOS | `atc-globus-os` | TypeScript | — | License dashboard OS |

## 3. Blockchain / Consensus (8)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| Blockchain Core | `atc-blockchain` | Python | 157KB | Chain implementation |
| Smart Contracts | `atc-contracts` | Python | 123KB | Contract templates |
| ATC VM | `atc-vm` | Python | 26KB | Virtual machine |
| Cross-Chain Bridge | `atc-bridge` | Rust | — | Interop bridge |
| ZKP | `atc-zkp` | Rust | — | Zero-knowledge proofs |
| Governance | `atc-governance` | Rust | — | DAO governance |
| DEX | `atc-dex` | TypeScript | — | Decentralized exchange |
| Testnet | `atc-testnet` | Python | — | Test network |

## 4. Networking (1)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| ATCNet Protocol | `atcnet` | Python | 83KB | P2P mesh networking |

## 5. Frontend / UI (6)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| UI Components | `atc-ui` | TypeScript | 138KB | Shared UI library |
| Frontend | `atc-frontend` | HTML | 63KB | Main frontend |
| Block Explorer | `atc-explorer` | TypeScript | 8KB | Chain explorer |
| IDE | `atc-ide` | TypeScript | 8KB | Browser IDE |
| Social | `atc-social` | TypeScript | — | Social platform |
| Mobile | `atc-mobile` | Python | 28KB | Mobile app |

## 6. Development Tools (6)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| CLI | `atc-cli` | Python | 33KB | Command-line interface |
| DevTools | `atc-devtools` | TypeScript | — | Developer tools |
| CI/CD | `atc-ci` | TypeScript | 28KB | Continuous integration |
| Genesis Engine | `atc-genesis-engine` | Python | 64KB | Game/platform engine |
| Franchise Factory | `atc-franchise` | TypeScript | 105KB | Lifecycle management |
| ShivaCore Tools | `atc-shivacore-tools` | — | 18KB | Kernel tooling |

## 7. AI / Intelligence (2)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| AI Studio | `atc-aistudio` | TypeScript | 934KB | AI development studio |
| KAI (Aurora AI) | `atc-aurora-ai` | Python | — | AI kernel |

## 8. Security (1)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| Security Module | `atc-security` | Rust | — | Cryptographic security |

## 9. Gaming (2)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| ShivaMon | `genesis-chronicles` | Python | 62KB | Monster battle game |
| ATC Game | `atc-game` | Rust | — | Game framework |

## 10. Languages (3)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| ATCLang | `atclang` | Python | 187KB | Proprietary language v0.1.0-alpha |
| ATCLang (alt) | `atc-atclang` | Python | 105KB | ATCLang mirror |
| Standard Library | `atc-stdlib` | Python | 29KB | ATCLang stdlib |

## 11. Wallet / Finance (1)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| Wallet | `atc-wallet` | Python | 23KB | ATC wallet (ATC prefix) |

## 12. Backend / Gateway (4)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| Backend API | `atc-backend` | Python | 49KB | REST/GraphQL API |
| API Gateway | `atc-gateway` | Python | 58KB | Gateway (port 4000) |
| Monitoring | `atc-monitoring` | Python | — | Prometheus/Grafana |
| Deployment | `atc-deploy` | Python | — | Docker deployment |

## 13. Documentation (1)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| Whitepaper | `atc-whitepaper` | TypeScript | 288KB | Official whitepaper v3.0.0 |

## 14. Assets (1)

| Component | Repo | Language | Size | Description |
|-----------|------|----------|------|-------------|
| Assets | `atc-assets` | Rust | — | Digital assets |

---

## Status Summary

| Status | Count | Repos |
|--------|-------|-------|
| **Active Development** | 3 | atc-shivacore, a-townchain-os, atc-genesis-engine |
| **Implemented** | 15 | atc-blockchain, atc-contracts, atc-ui, atc-gateway, atc-kernel, atcnet, atc-wallet, atc-vm, atc-cli, atc-franchise, atc-aistudio, atc-whitepaper, genesis-chronicles, atclang, atc-stdlib |
| **Skeleton/Stub** | 12 | atc-bridge, atc-zkp, atc-dex, atc-explorer, atc-ide, atc-social, atc-devtools, atc-game, atc-globus-os, atc-monitoring, atc-testnet, atc-deploy |
| **Placeholder** | 10 | atc-bootloader, atc-dns, atc-drivers, atc-linux-edition, atc-windows-edition, atc-security, atc-governance, atc-assets, atc-aurora-ai, atc-sdk |
| **Empty** | 5 | atc-atcpkg, atc-standards, atc-mobile, atc-backend, atc-ci |
