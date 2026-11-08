# ARCHITECTURE.md — atc-wallet

> Copyright © Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-wallet/
├── requirements.txt — Python dependencies (ecdsa, pysha3, requests)
├── setup.py — pip installation configuration
├── README.md — ATC Wallet overview
└── src/
    ├── __init__.py — Package initialization
    ├── wallet.py — Wallet core (address generation, signing, balance)
    └── crypto.py — Cryptographic helpers (ECDSA, SHA-256)
```

## Module Descriptions
- `wallet.py` — Wallet implementation with ATC-prefixed addresses (SHA-256 derivation)
- `crypto.py` — ECDSA signing and verification, key management

## Build System
- Python 3.11+ with pip

## Dependencies
- atc-blockchain (for transaction submission)
- atc-gateway (API Gateway for blockchain queries)

## Status (Active/Migrated/Legacy)
Active (Python, Wallet)
