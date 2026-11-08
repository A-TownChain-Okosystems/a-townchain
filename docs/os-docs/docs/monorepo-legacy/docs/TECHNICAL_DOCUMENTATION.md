# Technical Documentation — A-TownChain OS v1.0

> **Version:** 1.0.0 | **Date:** 04.08.2026 | **Agent:** Aurora #2 (6a275618)

---

## System Architecture

A-TownChain OS is a blockchain operating system with a monorepo structure:

```
a-townchain-os/
├── src/              # Python Backend
│   ├── gateway/      # API Gateway (Flask/FastAPI)
│   ├── blockchain/   # Blockchain Core
│   ├── core/         # Core Services
│   ├── contracts/     # Smart Contracts (ATC-8300, ATC-9900)
│   ├── franchise/    # Franchise Factory
│   └── game/         # Game Engine (ShivaMon)
├── atclang/          # ATCLang Compiler (Lexer, Parser, Type Checker, Stdlib)
├── frontend/         # React/TypeScript Frontend (Vite)
├── kernel/           # ShivaCore Kernel Interfaces (Rust ↔ ATCLang)
├── docker/           # Docker Setup (3 Dockerfiles + docker-compose.yml)
├── docs/             # Documentation, Standards, Whitepaper
├── tests/            # Test Suite (unit, integration, e2e)
├── scripts/          # Build & Test Scripts
└── modules/          # Optional Module Dependencies
```

## Consensus Model

A-TownChain OS uses a **Hybrid Consensus** model:

### Proof of History (PoH) — ATC-81
- SHA-256 based sequential hashing
- Creates a cryptographic timeline
- Each entry references the previous hash

### Proof of Work (PoW) — ATC-82
- Mining difficulty adjustment
- Block validation through hash computation
- Energy-intensive consensus layer

### Proof of Stake (PoS) — ATC-83
- Validator selection based on stake
- Slashing for malicious behavior
- Chain finality through stake-weighted voting

### Fork Resolution — ATC-84
- Deterministic SHA-256 based resolution
- Longest chain rule with stake weighting
- Automatic fork detection and resolution

## Token Standards

| Standard | Name | Type |
|----------|------|------|
| ATC-001 | Genesis Token | Base token |
| ATC-8300 | Fungible Token | ERC-20 style (non-EVM) |
| ATC-9900 | Governance/DAO | Voting + proposal system |

## Cryptography

- **ECDSA:** secp256k1 with RFC 6979 deterministic signatures (ATC-86)
- **Hash:** SHA-256 (PoH, blocks, transactions)
- **Keys:** secp256k1 key generation and management

## Persistence

- **SQLite** for block and transaction storage
- Automatic schema migration
- Crash recovery support

## ATCLang

ATCLang is the native programming language for smart contracts and DApps:
- **Lexer:** Token stream generation
- **Parser:** AST construction
- **Type Checker:** Static type analysis (34 tests)
- **Standard Library:** 44 tests covering core operations
- **Bytecode:** ATC-93 VM specification
- **Universal Mandate:** ATC-99 — everything programmed in ATCLang

## KAI-OS Integration

- Gemini AI integration for autonomous operation
- Agent Interaction Protocol (ATC-97)
- AI model marketplace (ATC-29)
- Federated learning (ATC-28)

## ShivaCore Kernel

The Rust-based kernel (no_std) provides:
- 60 kernel modules (K0-K50)
- 2146 Rust tests
- 30 ATCLang interfaces
- Boot sequence, memory management, scheduling, filesystem, networking

## Docker Services

| Service | Port | Description |
|---------|------|-------------|
| core | 4000 | Blockchain Core |
| blockchain | 5000 | Chain Node |
| frontend | 3000 | React UI |
| gateway | 80 | API Gateway |
| contracts | 8002 | Smart Contract Engine |
| franchise | 8003 | Franchise Factory |
| game | 8001 | Game Engine |

## Testing

- **29 test files, 385 test functions**
- Unit: atclang (191), core (130), blockchain (20), network, contracts
- Integration: Gateway ↔ Core ↔ Chain (7 tests)
- E2E: Frontend → Backend → Blockchain (7 tests)
- Docker-Compose: 8 tests
- Coverage threshold: ≥80%
- Frontend: Jest + React Testing Library configured

## CI/CD Pipeline

| Workflow | Jobs | Trigger |
|----------|------|---------|
| ci.yml | 5 (build-python, build-frontend, test, security-bandit, audit) | Push to main |
| codeql.yml | 1 (weekly security scan) | Weekly schedule |
| release.yml | 4 (docker, binaries, changelog, pages) | Tag v* |

## Standards

99 ATC Standards (ATC-01 to ATC-99):
- ATC-01 to ATC-10: Core protocol standards
- ATC-11 to ATC-20: Token and asset standards
- ATC-21 to ATC-30: AI and compute standards
- ATC-31 to ATC-50: Advanced AI and consciousness standards
- ATC-51 to ATC-80: Trans-reality and meta standards
- ATC-81 to ATC-99: Consensus, testing, and language standards

---

**Maintainer:** Michael Wroblewski (ShivaCore)
**Agent:** Aurora #2 (6a275618)
