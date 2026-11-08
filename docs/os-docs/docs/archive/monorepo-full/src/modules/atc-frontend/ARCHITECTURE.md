# ARCHITECTURE.md — atc-frontend
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
├── __mocks__/
│   └── styleMock.js
├── assets/
│   ├── css/
│   │   └── variables.css
│   └── js/
│       └── api.js
├── battle/
│   └── index.html
├── bootscreen/
│   └── README.md
├── css/
├── index.html
├── jest.config.js
├── jest.setup.js
├── js/
└── src/
    ├── App.tsx
    ├── components/
    │   ├── BlockCard.tsx
    │   ├── Footer.tsx
    │   ├── Header.tsx
    │   ├── StatCard.tsx
    │   └── TxRow.tsx
    └── views/
        ├── Dashboard.tsx
        ├── Explorer.tsx
        ├── Governance.tsx
        ├── Marketplace.tsx
        └── Wallet.tsx
```

## Module Descriptions
- **index.html**: Root web entry point for the A-TownChain front-end web dashboard.
- **css/** / **assets/css/**: CSS stylesheets, CSS custom properties, and UI layout theme variables.
- **js/** / **assets/js/**: ES6 JavaScript client modules, API communication layer, and DOM handlers.
- **assets/**: Media assets, icons, logos, and UI graphics.
- **battle/** & **bootscreen/**: Frontend components for boot sequence animations and interactive battle simulation screens.

## Build System
Static HTML5 / ES6 JavaScript web application architecture. Servable via any static HTTP server or bundlable with Vite/Webpack. Jest and styleMock configured for unit testing.

## Dependencies
Native ES6+ JavaScript runtime, HTML5 DOM APIs, CSS3 Custom Properties, Fetch API / WebSocket API for blockchain RPC node connection.
