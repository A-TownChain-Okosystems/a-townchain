# ATCNet Protokoll v2.1.0

## Ports
| Port | Verwendung |
|------|-----------|
| 4001 | P2P Node Communication |
| 5005 | Bootstrap Node |
| 9944 | WebSocket (RPC) |

## Nachrichtentypen
`HELLO`, `HELLO_ACK`, `PING`, `PONG`, `NEW_BLOCK`, `NEW_TX`, `GET_PEERS`, `PEERS`, `GET_BLOCK`, `BLOCK`, `CONSENSUS`, `DISCONNECT`

## Kademlia DHT
- K-Bucket Größe: 20
- Alpha (parallele Anfragen): 3
- Node-ID: SHA-256(IP + Port + Timestamp)

## Security
- Max 100 Nachrichten/60s pro Peer (Rate-Limit)
- Max Message-Size: 64 KB
- ECDSA-Signatur auf Block/TX-Messages

## Quellcode (Auszug)
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
    PING           = 
```
