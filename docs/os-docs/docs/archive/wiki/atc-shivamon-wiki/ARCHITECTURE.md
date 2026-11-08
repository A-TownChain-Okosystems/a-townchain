# ARCHITECTURE.md — atc-shivamon
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-shivamon/
├── README.md                 # Shivamon NFT gaming system overview
├── GAME_SPEC.md              # Detailed game mechanics specification
├── requirements.txt          # Python API and engine dependencies
├── battle_system.atc         # On-chain battle calculations and rules
├── breeding.atc              # Shivamon creature breeding logic contract
├── leaderboard.atc           # Player ranking and reward contract
├── api/
│   └── marketplace_routes.py # Python API endpoints for NFT marketplace
├── contracts/
│   ├── marketplace_contract.py # Python NFT marketplace implementation
│   ├── shivamon_contract.py    # Core Shivamon NFT token contract
│   └── shivamon/
│       └── breeding.atc        # Detailed breeding smart contract
└── engine/
    └── battle_engine.py      # Off-chain Python battle simulation engine
```

## Module Descriptions
- README.md — Overview of the Shivamon creature-battling game ecosystem
- GAME_SPEC.md — Comprehensive design doc covering combat stats, genetics, and economy
- requirements.txt — Dependencies for backend service and battle simulation
- battle_system.atc — Smart contract governing battle turns, damage formulas, and wins
- breeding.atc — Smart contract governing genetic crossover and offspring generation
- leaderboard.atc — Smart contract tracking global player ranks and season payouts
- api/marketplace_routes.py — REST API handlers for trading Shivamon NFTs
- contracts/marketplace_contract.py — On-chain market listing and trading logic
- contracts/shivamon_contract.py — ERC721-compatible Shivamon token contract
- engine/battle_engine.py — Python simulation engine for fast combat validation

## Build System
- Python setuptools / pip

## Dependencies
- Python 3.10+

## Status (Active/Migrated/Legacy)
Migrated to a-townchain-os / Legacy repo
