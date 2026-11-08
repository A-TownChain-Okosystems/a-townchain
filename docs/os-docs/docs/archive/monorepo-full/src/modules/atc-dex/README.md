# atc-dex

Dezentrale Börse (DEX) für das A-TownChain-Ökosystem.

## Features (geplant)
- AMM (Automated Market Maker, Constant Product)
- Order Books (Limit-Orders, Matching-Engine)
- Liquidity Pools (Token-Pairs, Fee-Tiers)
- Swap-Interface (Web-UI)
- Price-Oracles (TWAP, VWAP)
- Impermanent-Loss-Calculator
- Yield-Farming (LP-Rewards)

## Architektur
```
atc-dex/
├── contracts/
│   ├── amm.atc          # AMM Smart Contract
│   ├── pool.atc         # Liquidity Pool
│   └── router.atc       # Swap Router
├── frontend/
│   ├── src/
│   │   ├── components/   # Swap-UI, Pool-UI
│   │   └── pages/        # Swap, Liquidity, Farm
│   ├── package.json
│   └── tsconfig.json
└── tests/
```


## Abhängigkeiten
- [`A-TownChain-Okosystems/atc-blockchain`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-blockchain)
- [`A-TownChain-Okosystems/atc-assets`](https://github.com/A-TownChain-Okosystems/a-townchain-os/tree/main/src/modules/atc-assets)

## Copyright
Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.
