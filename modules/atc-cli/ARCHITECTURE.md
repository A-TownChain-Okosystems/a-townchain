# ARCHITECTURE.md — atc-cli

> Copyright © Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-cli/
├── requirements.txt — Python package dependencies
├── setup.py — pip installation configuration
├── README.md — CLI overview and usage guide
└── src/
    ├── __init__.py — Package initialization
    ├── cli.py — Main CLI entry point and command dispatcher
    └── commands/ — Individual command implementations
```

## Module Descriptions
- `cli.py` — Main CLI entry point using click, handles argument parsing and command routing
- `commands/` — Subcommand implementations (node, wallet, blockchain, deploy)
- `requirements.txt` — Dependencies: click, rich, requests, pyyaml

## Build System
- Python 3.11+ with pip
- Entry point: `python -m src.cli` or `atc-cli` after install

## Dependencies
- atc-gateway (API Gateway on port 4000)
- atc-wallet (wallet functionality)

## Status (Active/Migrated/Legacy)
Active (Python, CLI Tool)
