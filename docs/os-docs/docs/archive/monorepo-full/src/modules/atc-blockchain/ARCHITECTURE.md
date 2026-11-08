# ARCHITECTURE.md — atc-blockchain

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-blockchain/
├── requirements.txt — Core blockchain node python dependencies
├── README.md — Core node architecture and setup documentation
├── consensus/ — Proof-of-Stake / Proof-of-History consensus protocol engine
├── contracts/ — Smart contract execution engine and runtime environment
├── p2p/ — Peer-to-peer network protocol layer (TCP/WebSockets)
└── genesis/ — Initial network genesis state configuration and bootstrap generator
```

## Module Descriptions
- consensus/ — Implements block production, leader election, finality voting, and fork choice rules.
- contracts/ — Contract execution environment processing state transitions and gas accounting.
- p2p/ — Peer discovery, block propagation, and transaction gossip networking.
- genesis/ — Generates initial state allocations, validator sets, and network parameters.

## Build System
- Python 3.11 asynchronous blockchain node runtime configured via `requirements.txt`.

## Dependencies
- cryptography — Cryptographic signing, verification, and hash functions.
- websockets / asyncio — Asynchronous P2P network communication framework.
