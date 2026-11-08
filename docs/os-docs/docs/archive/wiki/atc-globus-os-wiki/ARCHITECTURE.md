# ARCHITECTURE.md — atc-globus-os
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-globus-os/
├── package.json               # Package configuration & build scripts
├── tsconfig.json             # TypeScript configuration
├── src/
│   ├── index.ts              # Entry point for Globus OS core
│   ├── core/                 # Core OS runtime, registry, and initialization
│   ├── civilization/         # Civilization engine, ecosystem AI, and identity layers
│   └── economy/              # Global economic models and resource pool management
├── core/
│   ├── globus_core.atc       # Globus OS core kernel contract
│   └── world_registry.atc    # World instance and state registry
├── civilization/
│   ├── civilization_engine.atc        # Autonomous civilization simulation engine
│   ├── ecosystem_ai.atc               # Ecosystem AI decision-making layer
│   ├── experience_orchestrator.atc   # World experience orchestration
│   └── identity_layer.atc            # Civilization citizen identity layer
├── economy/
│   ├── global_economy.atc    # Global trade and fiscal mechanics
│   └── resource_pools.atc    # Decentralized resource allocation pools
├── governance/
│   ├── planetary_council.atc # Planetary governance council logic
│   └── world_governance.atc  # World policy voting and law enforcement
└── network/
    ├── globus_discovery.atc  # Node discovery in Globus OS network
    └── world_sync.atc       # Inter-world state synchronization
```

## Module Descriptions
- package.json — Package configuration and dependency manifest
- tsconfig.json — TypeScript project configuration
- src/index.ts — Main export interface for Globus OS typescript modules
- src/core/ — Core operating system architecture and registry management
- src/civilization/ — Civilization simulation, AI orchestration, and identity
- src/economy/ — Economic simulation and resource pool management
- core/globus_core.atc — Core OS kernel contract logic
- core/world_registry.atc — Central registry for connected world instances
- civilization/civilization_engine.atc — Multi-agent civilization lifecycle contract
- civilization/ecosystem_ai.atc — AI ecosystem manager contract
- civilization/experience_orchestrator.atc — User experience and dynamic event generator
- civilization/identity_layer.atc — Citizen DID and identity contract
- economy/global_economy.atc — Global monetary and trade mechanics contract
- economy/resource_pools.atc — Resource token liquidity and allocation contract
- governance/planetary_council.atc — Multi-signature council voting contract
- governance/world_governance.atc — World policy and law execution contract
- network/globus_discovery.atc — Peer discovery protocol contract
- network/world_sync.atc — Cross-world state consensus sync contract

## Build System
- TypeScript compiler (tsc) / npm build scripts

## Dependencies
- Node.js, TypeScript

## Status (Active/Migrated/Legacy)
Active (TypeScript)
