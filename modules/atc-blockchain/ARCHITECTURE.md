# ARCHITECTURE.md — atc-blockchain

> Copyright © Michael Wroblewski / A-TownChain-Okosystems. Apache-2.0 lizenziert — siehe LICENSE

## File Tree
```tree
atc-blockchain/
├── requirements.txt — Core blockchain node python dependencies
├── README.md — Core node architecture and setup documentation
├── consensus/ — historical/spec-draft artifacts; canonical implementation is components/algorithm
├── contracts/ — Smart contract execution engine and runtime environment
├── p2p/ — Peer-to-peer network protocol layer (TCP/WebSockets)
└── genesis/ — Initial network genesis state configuration and bootstrap generator
```

## Module Descriptions
- consensus/ — Historical/spec-draft artifacts only; MUST NOT be treated as canonical implementation.
- contracts/ — Contract execution environment processing state transitions and gas accounting.
- p2p/ — Peer discovery, block propagation, and transaction gossip networking.
- genesis/ — Generates initial state allocations, validator sets, and network parameters.

## Build System
- Python 3.11 asynchronous blockchain node runtime configured via `requirements.txt`.

## Dependencies
- cryptography — Cryptographic signing, verification, and hash functions.
- websockets / asyncio — Asynchronous P2P network communication framework.
