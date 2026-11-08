# ATCNet — Vollständige P2P Implementierung
> Stand: 2026-06-12 | v3.2.0

## Implementierte Dateien
| Datei | Beschreibung | Status |
|-------|-------------|--------|
| `bootstrap_client.py` | Peer-Discovery | ✅ |
| `p2p_node.py` | P2P TCP Node | ✅ neu |
| `gossip.py` | Epidemic Gossip | ✅ neu |
| `nat_traversal.py` | STUN + Hole-Punch | ✅ neu |

## P2PNode
- `start()` — TCP Server auf Port 9000
- `connect(host, port)` — Peer verbinden
- `broadcast(type, payload)` — An alle Peers
- `on(type, handler)` — Message-Handler registrieren

## GossipProtocol
- Epidemic gossip mit TTL (max 10 Hops)
- Deduplication via SHA256 Message-IDs
- `broadcast_block(block, forward_fn)`
- `broadcast_tx(tx, forward_fn)`

## NATTraversal
- STUN-Discover: externe IP/Port ermitteln
- UDP Hole-Punching für NAT-Durchquerung
- Unterstützte STUN Server: Google, Cloudflare

_Stand: 2026-06-12 | Aurora Auto-Sync_
