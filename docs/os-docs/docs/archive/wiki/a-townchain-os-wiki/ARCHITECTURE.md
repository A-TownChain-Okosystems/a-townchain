# ARCHITECTURE.md — a-townchain-os
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
a-townchain-os/
├── package.json               # Monorepo root package configuration
├── README.md                 # Primary project overview and getting started
├── REALITY_STATUS.md         # Ecosystem operational and development state
├── backend/                  # Monorepo backend services, APIs, and microservices
├── blockchain/               # Core blockchain engine, consensus, and state machine
├── docs/                     # Architectural documentation, standards, and wiki
│   ├── kai-os-wiki.md        # KAI-OS operational wiki and architecture reference
│   └── standards/            # ATC platform standards specifications
├── src/                      # Monorepo shared libraries and runtime code
└── Dockerfile                # Containerized deployment manifest
```

## Module Descriptions
- package.json — Root manifest managing workspace packages and scripts
- README.md — Monorepo overview, architecture quickstart, and project specs
- REALITY_STATUS.md — Real-time tracking of implementation status across layers
- backend/ — Backend services, API endpoints, database orchestration, and bus gateways
- blockchain/ — Core blockchain protocol, consensus mechanism, EVM/ATC runtime, and state store
- docs/ — System documentation repository including specs and guides
- docs/kai-os-wiki.md — Comprehensive architectural wiki for KAI-OS / A-TownChain
- docs/standards/ — ATC standard specifications (ATC-01 through ATC-99)
- Dockerfile — Production deployment container specification

## Build System
- npm / Lerna / Yarn workspace monorepo build tools

## Dependencies
- Node.js, TypeScript, Python 3, Rust / Cargo

## Status (Active/Migrated/Legacy)
Active (TypeScript, Main Monorepo)
