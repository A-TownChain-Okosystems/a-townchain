# ATCNet Protokoll v2.1.0

## Kademlia DHT
- K-Bucket Größe: 20
- Alpha: 3 parallele Anfragen
- Node-ID: SHA-256(IP + Port + Timestamp)

## Security (v2.1.0)
- Rate-Limit: 100 Nachrichten/60s/Peer
- Message-Size: max 64 KB
- ECDSA-Signatur auf Block/TX-Messages

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
            raw = f"{self.from_node}{self.timestamp}{self.msg_type}".encode()
            self.msg_id = hashlib.sha3_256(raw).hexdigest()[:16]

    def encode(self) -> bytes:
        """ATCMessage → Bytes serialisieren."
```
