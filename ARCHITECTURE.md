# ARCHITECTURE.md — atc-explorer
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
├── address_view.atc
├── block_view.atc
├── mempool_view.atc
├── package.json
├── search.atc
├── src/
│   ├── api/
│   ├── components/
│   ├── index.ts
│   └── pages/
├── stats_dashboard.atc
├── tsconfig.json
└── tx_view.atc
```

## Module Descriptions
- **src/components/**: Reusable React UI components for transaction tables, block height badges, address cards, and search inputs.
- **src/pages/**: Application routes for block view, transaction view, address balance inspection, and mempool status dashboard.
- **src/api/**: Blockchain REST and RPC API integration layer querying A-TownChain indexers.
- **package.json** & **tsconfig.json**: Node package dependencies, build scripts, and strict TypeScript compiler settings.

## Build System
npm / pnpm with TypeScript compiler (`tsc`) and Vite / Webpack frontend bundler.

## Dependencies
TypeScript 5.0+, React, Axios / Fetch API, Chart.js / Recharts for visualizations, Tailwind CSS.
