# ATCNet — Proprietärer P2P Stack

## Übersicht
ATCNet implementiert Kademlia DHT für dezentrale Node-Discovery nach ATS-1004 + ATC-0005.

## Node-Typen
- **Bootstrap Node**: Port 5005 — Erstverbindung
- **Validator Node**: Port 4001 — Consensus-Teilnehmer
- **Light Node**: Port 4001 — Nur TX-Propagation
- **Archive Node**: Port 4001 — Vollständige Chain

## Security (v2.1.0)
```python
_MAX_MSG_PER_WINDOW = 100  # Max Nachrichten/60s/Peer
_WINDOW_SECONDS     = 60.0
_MAX_MSG_SIZE       = 65_536  # 64 KB
```

## Nachrichtentypen
`HELLO`, `HELLO_ACK`, `PING`, `PONG`, `NEW_BLOCK`, `NEW_TX`, `GET_PEERS`, `PEERS`, `GET_BLOCK`, `BLOCK`, `CONSENSUS`, `DISCONNECT`

## ATCNet Quellcode (Auszug)
```python
"""
ATCNet — Proprietärer P2P Netzwerk-Stack
Version: 1.0.0-alpha | ATS-1006 konform
Kein libp2p-Klon — eigene DHT + Routing-Implementierung
"""

import socket, threading, time, json, hashlib, struct, random
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Callable, Tuple, Set
from enum import IntEnum, auto


# ══════════════════════════════════════════════════════════
#  PROTOKOLL-KONSTANTEN
# ══════════════════════════════════════════════════════════

ATCNET_VERSION  = 1
ATCNET_PORT     = 4001
MAGIC_BYTES     = b"\xA7\xC0\x01"   # ATC Magic
MAX_MSG_SIZE    = 4 * 1024 * 1024   # 4MB
K_BUCKET_SIZE   = 20                # Kademlia k
ALPHA           = 3                 # Parallele Lookups
TTL_DEFAULT     = 10                # Max Hops


# ══════════════════════════════════════════════════════════
#  NACHRICHTENTYPEN (ATC-0007)
# ══════════════════════════════════════════════════════════

class MsgType(IntEnum):
    HELLO          = 1
    PING           = 2
    PONG           = 3
    GET_PEERS      = 4
    PEERS          = 5
    GET_BLOCK      = 6
    BLOCK          = 7
    BROADCAST_TX   = 8
    TX             = 9
    CONSENSUS_VOTE = 10
    CONSENSUS_BLOCK= 11
    KI_QUERY       = 12
    KI_RESPONSE    = 13
    FIND_NODE      = 14
    FOUND_NODE     = 15
    DISCONNECT     = 16
    ERROR          = 99


@dataclass
class ATCMessage:
    version:   int
    msg_type:  MsgType
    from_node: str          # NodeID (hex)
    to_node:   str          # NodeID oder "ff" * 20 für Broadcast
    payload:   bytes
    ttl:       int = TTL_DEFAULT
    timestamp: int = 0
    msg_id:    str = ""

    def __post_init__(self):
        if not self.timestamp:
            self.timestamp = int(time.time() * 1000)
        if not self.msg_id:
            raw = f"
```
