# ARCHITECTURE.md — atc-kernel
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-kernel/
├── README.md                 # Legacy kernel specification & overview
├── SECURITY.md               # Security model for kernel execution
├── ai_bus_ad13.atc           # AI communication bus contract definition
├── asset_bus_ad08.atc        # Asset transfer bus contract definition
├── command_bus_ad02.atc      # System command bus contract definition
├── ai_kernel/                # Core AI orchestration, protocol, and neural mesh
│   ├── ai_kernel.atc         # AI kernel main logic contract
│   ├── atc-97_agent_interaction_protocol.atc
│   ├── distributed_intelligence/ # Neural mesh and quantum-resistant crypto
│   └── orchestration/        # Tensor compute and reputation scoring
└── consensus/                # Consensus sub-system specifications
```

## Module Descriptions
- README.md — Architectural description of the legacy kernel architecture
- SECURITY.md — Kernel security sandbox boundaries and isolation rules
- ai_bus_ad13.atc — AI bus for inter-agent intent passing
- asset_bus_ad08.atc — High-throughput asset transaction bus
- command_bus_ad02.atc — Low-level administrative kernel command bus
- ai_kernel/ — Complete AI operating kernel suite, neural mesh, and tensor orchestration
- consensus/ — Legacy state machine consensus contracts

## Build System
- Cargo (Rust) / Python setuptools

## Dependencies
- Rust edition 2021, Python 3.10+

## Status (Active/Migrated/Legacy)
Migrated to a-townchain-os / Legacy repo
