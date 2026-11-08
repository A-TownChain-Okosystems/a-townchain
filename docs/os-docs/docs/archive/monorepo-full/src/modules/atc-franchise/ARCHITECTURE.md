# ARCHITECTURE.md — atc-franchise

> Copyright © Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-franchise/
├── package.json — Node.js workspace configuration
├── tsconfig.json — TypeScript compiler configuration
├── vite.config.ts — Vite build configuration
├── README.md — Franchise Factory overview
└── src/
    ├── index.ts — Main entry point
    ├── modules/ — Fr lifecycle modules
    ├── factories/ — MetaFactory infrastructure
    └── types/ — TypeScript type definitions
```

## Module Descriptions
- `src/index.ts` — Main entry point for the Franchise Factory platform
- `src/modules/` — 14 lifecycle modules (creation, deployment, monetization, etc.)
- `src/factories/` — 12 infrastructure factories (MetaFactory v3.0)
- `src/types/` — Shared TypeScript interfaces and types

## Build System
- Node.js + TypeScript + Vite
- npm workspace

## Dependencies
- atc-ui (component library)
- atc-gateway (API Gateway)
- atc-blockchain (blockchain core)

## Status (Active/Migrated/Legacy)
Active (TypeScript, Franchise Platform)
