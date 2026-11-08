# ATCNet Nachrichten-Protokoll

## Nachrichtenformat (JSON)
```json
{
  "type": "NEW_BLOCK",
  "version": "2.1.0",
  "chain_id": 9000,
  "timestamp": 1717948800,
  "nonce": 42,
  "payload": { ... },
  "signature": "ecdsa_sig_hex",
  "sender": "ATC..."
}
```

## Nachrichtentypen
| Typ | Richtung | Beschreibung |
|-----|----------|-------------|
| `HELLO` | → | Erstverbindung, Version-Handshake |
| `HELLO_ACK` | ← | Verbindung bestätigt |
| `PING` | ↔ | Keep-Alive (alle 30s) |
| `PONG` | ↔ | Ping-Antwort |
| `NEW_BLOCK` | broadcast | Neuen Block ankündigen |
| `NEW_TX` | broadcast | Neue Transaktion |
| `GET_PEERS` | → | Peers anfragen |
| `PEERS` | ← | Peer-Liste (max 20) |
| `GET_BLOCK` | → | Block by Hash anfragen |
| `BLOCK` | ← | Block-Daten |
| `CONSENSUS` | broadcast | Consensus-Vote |
| `DISCONNECT` | → | Verbindung trennen |

## Size-Limits (v2.1.0)
| Typ | Max-Size |
|-----|---------|
| HELLO | 512 Bytes |
| NEW_BLOCK | 64 KB |
| NEW_TX | 4 KB |
| PEERS | 4 KB |
| Default | 64 KB |
