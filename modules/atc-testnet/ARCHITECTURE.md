# ARCHITECTURE.md — atc-testnet
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
├── .gitignore
├── CHANGELOG.md
├── COMPONENT_PLAN.md
├── FILE_REGISTER.md
├── LICENSE
├── README.md
├── ROADMAP.md
├── STATUS.md
├── config/
│   ├── genesis_config.atc
│   └── mainnet_config.atc
├── devnet/
│   └── snapshot_manager.atc
├── docker/
├── launcher/
│   ├── devnet_launcher.atc
│   └── testnet_launcher.atc
├── nodes/
│   ├── node_bootstrap.atc
│   └── validator_setup.atc
├── requirements.txt
├── scripts/
│   └── deploy_testnet.atc
└── src/
    ├── __init__.py
    ├── config/
    │   └── __init__.py
    ├── explorer/
    │   └── __init__.py
    ├── faucet/
    │   └── __init__.py
    └── validator/
        └── __init__.py
```

## Module Descriptions
- **docker/**: Docker Compose files and container definitions for spinning up local and remote multi-node testnets.
- **scripts/**: Shell and Python scripts for network launch, peer bootstrapping, state resets, and transaction load testing.
- **config/**: Genesis configuration files, network parameter files, and bootnode peer lists.
- **src/**: Testnet management CLI and Python orchestrator package.
- **requirements.txt**: Dependencies for testnet orchestration and node management scripts.

## Build System
Python 3.10+ packaging (`setuptools`), Docker and Docker Compose orchestration runtime.

## Dependencies
Python 3.10+, `docker`, `pyyaml`, `click`, `requests`, `eth-utils`.
