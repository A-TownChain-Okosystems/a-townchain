# ARCHITECTURE.md — atc-social
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-social/
├── package.json               # Node.js dependencies and scripts configuration
├── tsconfig.json             # TypeScript compiler settings
├── src/
│   ├── index.ts              # Entry point for social module
│   ├── identity/             # Identity management & decentralized identifier handling
│   ├── reputation/           # On-chain reputation & trust calculation system
│   ├── messaging/            # Encrypted social messaging protocols
│   └── social_graph.rs       # High-performance social graph data structures
├── chat/
│   └── chat_system.atc       # ATCLang smart contract for real-time chat
├── feed/
│   └── social_feed.atc       # Decentralized social feed contract
├── identity/
│   └── social_identity.atc   # Social identity storage contract
├── messaging/
│   └── messaging.atc         # Messaging protocol contract
├── moderation/
│   └── moderation.atc        # Content moderation rules contract
└── reputation/
    └── reputation_system.atc # Reputation score calculation contract
```

## Module Descriptions
- package.json — Package configuration and npm build scripts for TypeScript social module
- tsconfig.json — TypeScript configuration for compilation targets and module resolution
- src/index.ts — Main export interface for social layer components
- src/identity/ — Identity management system for user DIDs and profile verification
- src/reputation/ — On-chain reputation scoring and decentralized trust calculation
- src/messaging/ — End-to-end encrypted messaging protocols and channel handlers
- src/social_graph.rs — Rust module for high-performance social graph and relationship mapping
- chat/chat_system.atc — ATCLang smart contract for real-time chat interactions
- feed/social_feed.atc — Decentralized social feed smart contract
- identity/social_identity.atc — On-chain social identity storage contract
- messaging/messaging.atc — Core messaging protocol contract
- moderation/moderation.atc — Decentralized content moderation contract
- reputation/reputation_system.atc — On-chain reputation registry smart contract

## Build System
- TypeScript compiler (tsc) / npm build scripts

## Dependencies
- Node.js, TypeScript, Rust/Cargo (for social_graph.rs)

## Status (Active/Migrated/Legacy)
Active (TypeScript)
