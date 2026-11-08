# A-TownChain — Programming Languages

> 5 Languages across 50 code repositories
> Generated: 2026-08-06

## Language Distribution

| Language | Repos | Primary Use | Standard |
|----------|-------|-------------|----------|
| **Python** | 18 | Blockchain, Backend, AI, Tools | ATC-0001–ATC-0008 |
| **Rust** | 13 | Kernel, OS, Security | ATS-1000–ATS-1007 |
| **TypeScript** | 13 | Frontend, UI, DEX, Explorer | ATC-FE-001 |
| **HTML** | 2 | Monorepo, Frontend | ATC-FE-002 |
| **None** | 4 | Config, Standards, SDK | — |

## 1. Python (18 repos)

### Definition
- **Version**: Python 3.11+
- **Style**: PEP 8, type hints required
- **Framework**: Flask Blueprints, custom event bus
- **Testing**: pytest, ≥80% coverage required
- **Package**: pip, requirements.txt

### Repos
`atc-atclang`, `atc-aurora-ai`, `atc-backend`, `atc-blockchain`, `atc-cli`, `atc-contracts`, `atc-deploy`, `atc-gateway`, `atc-genesis-engine`, `atc-mobile`, `atc-monitoring`, `genesis-chronicles`, `atc-stdlib`, `atc-testnet`, `atc-vm`, `atc-wallet`, `atclang`, `atcnet`

### Conventions
- Wallet addresses: `ATC` prefix + 32 chars (SHA-256 derivation)
- API routing: exclusively through Gateway (port 4000)
- Token standards: ATC-8300 (fungible), ATC-9000 (NFT)
- Consensus: hybrid SHA-256 PoW + PoS + PoH

## 2. Rust (13 repos)

### Definition
- **Edition**: Rust 2021 (1.97+)
- **no_std**: Kernel modules use `alloc` crate, no `std`
- **Concurrency**: `spin::Mutex` for kernel, `std::sync` for userspace
- **Testing**: `cargo test`, all tests must pass
- **Target**: `x86_64-unknown-none` (kernel), `x86_64-unknown-linux-gnu` (tools)

### Repos
`atc-shivacore`, `atc-kernel`, `atc-bootloader`, `atc-drivers`, `atc-dns`, `atc-linux-edition`, `atc-windows-edition`, `atc-bridge`, `atc-zkp`, `atc-governance`, `atc-security`, `atc-game`, `atc-assets`

### Conventions
- Kernel standard: ATS-1000–ATS-1007
- PID type: `ats1000::Pid` (Newtype struct, unified)
- Capability system: Rights bitfield, CapabilityTable with spin::Mutex
- Memory: HEAP_START=0x444444440000, USERSPACE_BASE=0x555555550000
- Chain-ID: 658467 (constant in all modules)
- no POSIX dependencies — fully from-scratch

## 3. TypeScript (13 repos)

### Definition
- **Version**: TypeScript 5.x
- **Build**: Vite, ES modules
- **Framework**: React 18+ (frontend), Node.js (backend)
- **Styling**: Neon/dark theme, custom CSS
- **Testing**: Vitest, ≥80% coverage

### Repos
`a-townchain-os-docs`, `atc-aistudio`, `atc-analytics`, `atc-ci`, `atc-devtools`, `atc-dex`, `atc-explorer`, `atc-franchise`, `atc-globus-os`, `atc-ide`, `atc-social`, `atc-ui`, `atc-whitepaper`

### Conventions
- API calls: exclusively through Gateway (port 4000)
- UI: futuristic neon/dark aesthetic
- Components: modular, /ui directory structure
- ATC token integration: ATC-8300/ATC-9000

## 4. HTML (2 repos)

### Definition
- **Standard**: HTML5, semantic markup
- **Integration**: Flask templates, Jinja2

### Repos
`a-townchain-os`, `atc-frontend`

## 5. None / Config (4 repos)

### Definition
- Markdown, YAML, JSON documentation
- Standards specifications

### Repos
`atc-atcpkg`, `atc-sdk`, `atc-standards`, `atc-shivacore-tools`

## ATCLang (Proprietary)

- **Version**: v0.1.0-alpha
- **Type**: Stack-based VM, recursive descent parser
- **Syntax**: `ATC::Wallet::new` namespace style
- **Repos**: `atclang`, `atc-atclang`, `atc-stdlib`
- **Status**: Implemented (lexer, parser, compiler, 30+ opcodes)

## Cross-Language Standards

| Standard | Scope | Languages |
|----------|-------|-----------|
| ATC-0001–ATC-0008 | Blockchain protocols | All |
| ATS-1000–ATS-1007 | Kernel/OS operations | Rust, Python |
| ATC-LIC | Smart contract licensing | All |
| ATS-LIC | System/hardware licensing | Rust |
| ATC-8300 | Fungible tokens | All |
| ATC-9000 | NFT tokens | All |
