# ARCHITECTURE.md — atcnet
> Copyright © Michael Wroblewski / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atcnet/
├── README.md                 # P2P network specification overview
├── PROTOCOL.md               # Wire protocol & packet format spec
├── requirements.txt          # Python networking dependencies
├── atcnet.py                 # Core P2P network interface and node host
├── atcnet.atc                # Smart contract network interface
├── bootstrap_client.py       # Node bootstrap and peer discovery client
├── discovery.py              # Kademlia-style peer discovery mechanism
├── gossip.atc                # Gossip protocol contract specification
├── nat_traversal.atc         # STUN/TURN NAT traversal contract logic
├── p2p_node.atc              # P2P node state contract
├── p2p_propagation.py        # Block and transaction propagation logic
└── network/                  # Specialized network contracts (Sharding, Time Sync)
```

## Module Descriptions
- README.md — Documentation for the legacy networking protocol stack
- PROTOCOL.md — Wire message formats, encryption handshake, and packet headers
- requirements.txt — Python async socket and cryptography dependencies
- atcnet.py — Primary Python daemon managing TCP/UDP peer connections
- bootstrap_client.py — Seed node client for initial peer network discovery
- discovery.py — Node discovery implementation using DHT lookup
- p2p_propagation.py — Broadcast and flood fill block propagation algorithm
- network/ — Specialized protocol contracts including liquid state migration and time sync

## Build System
- Python setuptools / pip

## Dependencies
- Python 3.10+, asyncio, cryptography

## Status (Active/Migrated/Legacy)
Migrated to a-townchain-os / Legacy repo
