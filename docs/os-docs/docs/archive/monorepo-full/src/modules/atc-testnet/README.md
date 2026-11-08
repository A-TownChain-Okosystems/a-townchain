# atc-testnet

A-TownChain Testnet — isolierte Test-Umgebung für Blockchain-Features.

## Features (geplant)
- Lokale Testnet-Instanz (Docker-basiert)
- Validator-Simulation (PoH, PoS, Finality)
- Smart Contract Deployment & Testing
- Cross-Chain Bridge-Testing
- Faucet (Test-Token Distribution)
- Block-Explorer (Integration mit atc-explorer)

## Architektur
```
atc-testnet/
├── docker/
│   ├── validator/        # Validator-Node
│   ├── faucet/           # Token-Faucet
│   └── explorer/         # Block-Explorer
├── scripts/
│   ├── setup.sh          # Testnet-Setup
│   └── reset.sh          # Testnet-Reset
├── config/
│   └── genesis.json      # Genesis-Konfiguration
└── tests/
```


## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-blockchain`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-blockchain)

## Copyright
Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.
