# ARCHITECTURE.md — atc-atcpkg
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-atcpkg/
├── README.md                 # Package manager design overview
├── installer.atc             # Package installation contract logic
├── manager.atc               # Package manager CLI & runtime control contract
├── manifest.atc              # Package manifest validator & specification
├── registry.atc             # Decentralized package registry contract
├── resolver.atc              # Dependency resolution logic contract
├── validator.atc             # Package security and hash verification contract
└── docs/                     # Specifications (Agent Scheduling, Kernel Protocols)
    ├── ATC-24-AGENT_SCHEDULING.md
    └── ATC-96-KERNEL_INTERFACE_PROTOCOL.md
```

## Module Descriptions
- README.md — Concept overview for ATCPKG on-chain package ecosystem
- installer.atc — Smart contract handling package deployment to nodes
- manager.atc — Package lifecycle management contract
- manifest.atc — Package descriptor parsing and validation smart contract
- registry.atc — Decentralized index of published ATCLang packages
- resolver.atc — Semantic versioning and dependency tree resolver contract
- validator.atc — Cryptographic integrity and signature verification contract
- docs/ — Technical specs on scheduling and kernel interface protocols

## Build System
- Markdown / ATCLang compiler

## Dependencies
- ATCLang runtime

## Status (Active/Migrated/Legacy)
Active (Docs / Package manager concept)
