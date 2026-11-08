# Rate-Limiting — API Gateway

## Token-Bucket Algorithmus
Jeder API-Key hat einen eigenen Token-Bucket.

```
Kapazität:    100 Tokens (Standard)
Auffüll-Rate: 100 Tokens / 60 Sekunden
Verbrauch:    1 Token pro Request
```

## Endpoint-spezifische Limits
| Endpoint-Gruppe | Limit | Fenster |
|----------------|-------|---------|
| `/api/wallet/send` | 5 | 60s |
| `/api/ai/*` | 10 | 60s |
| `/api/blockchain/tx` POST | 20 | 60s |
| `/api/game/mint` | 10 | 60s |
| `/api/marketplace/*` POST | 15 | 60s |
| Alle anderen | 100 | 60s |

## P2P Rate-Limit (ATCNet)
- Max 100 Nachrichten / 60s / Peer
- Max Message-Size: 64 KB
- Automatischer Peer-Ban bei dauerhafter Überschreitung

## Response bei Rate-Limit-Überschreitung
```json
HTTP 429 Too Many Requests
{
  "error": "rate_limit_exceeded",
  "retry_after": 42,
  "limit": 10,
  "window": "60s"
}
```

## Header
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 87
X-RateLimit-Reset: 1717949160
```
