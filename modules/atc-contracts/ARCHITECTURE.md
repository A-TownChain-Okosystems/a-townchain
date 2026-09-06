# ARCHITECTURE.md — atc-contracts
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-contracts/
├── README.md                 # Legacy smart contracts repository overview
├── atc8300/                  # ATC8300 token standards & smart contract implementations
├── atcoin/                   # ATCoin native currency contracts (.atc / .py)
├── base/                     # Base contract abstractions & standard interfaces
├── bridge/                   # Cross-chain bridge contracts (.atc / .py)
└── contracts/                # Specific legacy contract suites (ATC001 token, revenue)
```

## Module Descriptions
- README.md — Overview of historical smart contract implementations
- atc8300/ — Legacy token standard implementations (atc8300_token.py, atc8300.atc)
- atcoin/ — Native coin contracts (atcoin.py, atcoin.atc)
- base/ — Base contract class hierarchy and access control (base_contract.py)
- bridge/ — Cross-chain bridge contracts (bridge_contract.py)
- contracts/ — Specific legacy contracts including genesis_token.py and revenue.atc

## Build System
- Python setuptools / pytest

## Dependencies
- Python 3.10+

## Status (Active/Migrated/Legacy)
Migrated to a-townchain-os / Legacy repo
