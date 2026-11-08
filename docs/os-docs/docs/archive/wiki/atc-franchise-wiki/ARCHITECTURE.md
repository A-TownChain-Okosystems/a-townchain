# ARCHITECTURE.md — atc-franchise
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-franchise/
├── README.md                 # Franchise factory overview
├── factory.py                # Main franchise factory Python orchestration module
├── factory.atc               # Franchise factory smart contract logic
├── contracts/                # Franchise registries and revenue smart contracts
│   ├── registry.atc          # Franchise registry contract
│   └── revenue.atc           # Franchise revenue split contract
├── docs/                     # Franchise architectural specifications & security notes
└── gff_core_ad20.atc         # Global Franchise Factory core contract
```

## Module Descriptions
- README.md — Documentation for franchise engine and factory creation
- factory.py — Python orchestration logic for instantiating new franchises
- factory.atc — Core smart contract engine for franchise issuance
- contracts/registry.atc — On-chain franchise registry and ownership contract
- contracts/revenue.atc — Revenue distribution and royalty contract
- docs/ — Architecture and security documentation for franchise operations
- gff_core_ad20.atc — Core standard specification for Global Franchise Factory

## Build System
- Python setuptools / npm

## Dependencies
- Python 3.10+, Node.js

## Status (Active/Migrated/Legacy)
Migrated to a-townchain-os / Legacy repo
