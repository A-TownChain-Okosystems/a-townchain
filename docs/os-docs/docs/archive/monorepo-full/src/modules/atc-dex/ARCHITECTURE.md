# ARCHITECTURE.md — atc-dex
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
├── amm/
│   └── amm.atc
├── contracts/
├── fees/
│   └── dex_fees.atc
├── frontend/
│   └── src/
├── oracle/
│   └── price_oracle.atc
├── orders/
│   └── limit_orders.atc
├── package.json
├── pool/
│   └── pool_manager.atc
├── src/
│   └── index.ts
├── swap/
│   └── swap_router.atc
└── tsconfig.json
```

## Module Descriptions
- **contracts/**: Smart contracts for Automated Market Maker (AMM) pools, liquidity providers, fee routers, and limit orders.
- **frontend/src/**: Trading interface web frontend built with React/TypeScript, providing swap UI and liquidity dashboards.
- **amm/**, **fees/**, **oracle/**, **orders/**, **pool/**, **swap/**: DEX sub-components managing price calculation, swap execution, fee distribution, and price oracle updates.
- **package.json** & **tsconfig.json**: Dependency specifications and TypeScript compiler configuration.

## Build System
Hardhat / Foundry for contract compilation & unit tests; Vite / Node.js for frontend web compilation.

## Dependencies
TypeScript, Ethers.js / Viem, React, Hardhat, OpenZeppelin Contracts, Tailwind CSS.
