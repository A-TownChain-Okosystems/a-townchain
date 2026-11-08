# IPC — Inter-Process-Communication (ATS-1007)

## EventBus
Zentrales Pub/Sub-System des ShivaOS-Kernels.

```python
from core.event_bus import EventBus
bus = EventBus()

# Subscriber
bus.subscribe("block.new", lambda data: print(f"Neuer Block: {data}"))

# Publisher
bus.emit("block.new", {"height": 42, "hash": "abc..."})

# Wildcard
bus.subscribe("block.*", handler)
```

## Channel-Typen
| Typ | Beschreibung | Max-Queue |
|-----|-------------|----------|
| `SYNC` | Synchron, blockierend | 1 |
| `ASYNC` | Asynchron, nicht-blockierend | 100 |
| `BROADCAST` | 1:N, alle Subscriber | 500 |
| `RING` | Ringbuffer, älteste überschreiben | 500 |

## Kern-Events
| Event | Payload | Beschreibung |
|-------|---------|-------------|
| `kernel.start` | `{version}` | Kernel gestartet |
| `kernel.stop` | `{reason}` | Kernel gestoppt |
| `block.new` | `{height,hash}` | Neuer Block |
| `tx.new` | `{hash,from,to,amount}` | Neue TX |
| `consensus.round` | `{round,validator}` | Consensus-Runde |
| `peer.connect` | `{addr,port}` | Peer verbunden |
| `peer.disconnect` | `{addr}` | Peer getrennt |
| `contract.call` | `{address,method}` | Contract-Aufruf |

## Performance
- Ringbuffer: max 500 Einträge (RING-Kanäle)
- Wildcard-Lookup: O(1) via Prefix-Index
- Event-History: letzte 500 Events gecacht
