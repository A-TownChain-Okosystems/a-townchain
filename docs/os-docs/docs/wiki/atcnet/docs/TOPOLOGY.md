# ATCNet — Netzwerk-Topologie

## Kademlia DHT Topologie
```
Bootstrap Node (Port 5005)
       │
  ┌────┼────┐
Node  Node  Node  (Port 4001)
  │         │
 ...        ...
```

## Node-Typen
| Typ | Port | Funktion |
|-----|------|---------|
| Bootstrap | 5005 | Erstverbindung, Peer-Discovery |
| Validator | 4001 | Consensus, Block-Produktion |
| Full Node | 4001 | Vollständige Chain, TX-Weiterleitung |
| Light Node | 4001 | Nur Header + eigene TXs |
| Archive | 4001 | Gesamte Chain-History |

## K-Bucket Struktur
- K-Bucket-Größe: **20** Nodes
- Distanz-Metrik: XOR(NodeID_A, NodeID_B)
- Alpha (parallele Lookups): **3**
- Refresh-Interval: alle 3600s

## Peer-Discovery
1. Verbinde zu Bootstrap-Node (Port 5005)
2. Sende `HELLO` mit eigener Node-ID + Version
3. Empfange `PEERS` Liste (max 20)
4. Verbinde zu 3 zufälligen Peers
5. Sende `GET_PEERS` rekursiv bis K-Buckets voll

## Netzwerk-Parameter
| Parameter | Wert |
|-----------|------|
| Max Peers | 50 |
| Ping-Interval | 30s |
| Node-Timeout | 120s |
| K-Bucket-Größe | 20 |
| Rate-Limit | 100 Msgs/60s/Peer |
| Max-Message-Size | 64 KB |
