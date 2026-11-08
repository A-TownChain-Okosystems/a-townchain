# Middleware-Pipeline

```
Request → Logger → Rate-Limit → Auth → Signature-Verify → Backend
```

## Rate-Limiter (Token-Bucket)
- `/api/ai/*`: 10 Req/min
- `/api/wallet/send`: 5 Req/min
- Default: 100 Req/min

## Signatur-Validierung
Alle `/api/wallet/send` Requests müssen ECDSA-signiert sein.
Header: `X-Signature: <ecdsa-sig>`, `X-Public-Key: <pubkey>`
