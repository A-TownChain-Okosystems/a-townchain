# ARCHITECTURE.md — genesis-chronicles
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0 lizenziert — siehe LICENSE

## File Tree
```tree
genesis-chronicles/
├── README.md                 # GenesisChronicles NFT gaming system overview
├── GAME_SPEC.md              # Detailed game mechanics specification
├── requirements.txt          # Python API and engine dependencies
├── battle_system.atc         # On-chain battle calculations and rules
├── breeding.atc              # GenesisChronicles creature breeding logic contract
├── leaderboard.atc           # Player ranking and reward contract
├── api/
│   └── marketplace_routes.py # Python API endpoints for NFT marketplace
├── contracts/
│   ├── marketplace_contract.py # Python NFT marketplace implementation
│   ├── genesis_chronicles_contract.py    # Core GenesisChronicles NFT token contract
│   └── genesis_chronicles/
│       └── breeding.atc        # Detailed breeding smart contract
└── engine/
    └── battle_engine.py      # Off-chain Python battle simulation engine
```

## Module Descriptions
- README.md — Overview of the GenesisChronicles creature-battling game ecosystem
- GAME_SPEC.md — Comprehensive design doc covering combat stats, genetics, and economy
- requirements.txt — Dependencies for backend service and battle simulation
- battle_system.atc — Smart contract governing battle turns, damage formulas, and wins
- breeding.atc — Smart contract governing genetic crossover and offspring generation
- leaderboard.atc — Smart contract tracking global player ranks and season payouts
- api/marketplace_routes.py — REST API handlers for trading GenesisChronicles NFTs
- contracts/marketplace_contract.py — On-chain market listing and trading logic
- contracts/genesis_chronicles_contract.py — ERC721-compatible GenesisChronicles token contract
- engine/battle_engine.py — Python simulation engine for fast combat validation

## Build System
- Python setuptools / pip

## Dependencies
- Python 3.10+

## Status (Active/Migrated/Legacy)
Migrated to a-townchain-os / Legacy repo
