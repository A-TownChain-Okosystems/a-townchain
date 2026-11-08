# ARCHITECTURE.md — atc-ui
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-ui/
├── index.html                # Entry web application HTML template
├── DESIGN.md                 # UI design system guidelines and component tokens
├── assets/                   # Static media, icons, and client scripts
│   └── js/
│       └── api.js            # Frontend REST API client
└── src/                      # React frontend component library
    └── components/           # UI dashboards, terminal view, zk circuit editor
        ├── ATownDashboardView.tsx
        ├── AtsSuite.tsx
        ├── ConsensusIntegrationGuide.tsx
        ├── DeFiLiquidityPoolView.tsx
        ├── GitHubStatusDashboard.tsx
        ├── MetricsDashboard.tsx
        ├── OfficeSuiteView.tsx
        ├── ProjectAuditDashboard.tsx
        ├── SyncDashboardModal.tsx
        ├── SystemHealthDashboard.tsx
        ├── TerminalView.tsx
        └── ZkCircuitEditorView.tsx
```

## Module Descriptions
- index.html — HTML5 single page application entry host
- DESIGN.md — Specification of color palettes, typography, and widget interfaces
- assets/js/api.js — Client API helper library
- src/components/ATownDashboardView.tsx — Main portal dashboard component
- src/components/SystemHealthDashboard.tsx — Real-time node and telemetry health viewer
- src/components/TerminalView.tsx — Web-based interactive command-line terminal
- src/components/ZkCircuitEditorView.tsx — Visual editor for zero-knowledge circuits

## Build System
- Vite / npm

## Dependencies
- Node.js, React, TypeScript

## Status (Active/Migrated/Legacy)
Migrated to a-townchain-os / Legacy repo
