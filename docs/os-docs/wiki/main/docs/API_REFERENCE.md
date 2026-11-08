# A-TownChain API Reference v3.0.0

> 47 Endpoints | Port 4000 | ECDSA Auth | Rate-Limited

## Auth
```
X-API-Key: <32+ chars>           (alle Endpoints)
X-Signature: <ECDSA secp256k1>  (state-mutating)
X-Public-Key: <64 hex>
X-Nonce: <monoton steigend>
```

## Quick Reference

| Kategorie | Endpoints | Auth | Rate |
|-----------|-----------|------|------|
| Blockchain | 4 | API-Key | 1000/min |
| Wallet | 3 | API-Key | 500/min |
| Contracts | 3 | API-Key+ECDSA | 200/min |
| Bridge | 3 | API-Key+ECDSA | 20/min |
| DEX | 5 | API-Key+ECDSA | 100–500/min |
| Gaming | 4 | API-Key+ECDSA | 10–500/min |
| Marketplace | 3 | API-Key+ECDSA | 20–500/min |
| AI | 2 | API-Key+Gemini | 20–50/min |
| Mobile | 3 | Session-Token | 5–100/min |
| Governance | 3 | API-Key+ECDSA | 2–200/min |
| Franchise | 2 | API-Key+ECDSA | 2–100/min |
| System | 4 | None/Internal | ∞ |

## Error Codes
`400` invalid_request | `401` missing_api_key | `401` invalid_signature
`403` forbidden | `404` not_found | `409` nonce_too_low
`422` invalid_address | `429` rate_limit_exceeded | `500` internal_error

## Examples

```bash
# Balance abfragen
curl -H "X-API-Key: your_key" \
  http://localhost:4000/api/wallet/balance/ATC...

# DEX Quote
curl -H "X-API-Key: your_key" \
  "http://localhost:4000/api/dex/quote?token_in=ATC&token_out=FFT&amount=1000000000000000000"

# TX senden
curl -X POST -H "X-API-Key: key" -H "X-Signature: sig" \
  -d '{"from":"ATC...","to":"ATC...","amount":1000,"nonce":1}' \
  http://localhost:4000/api/blockchain/tx/send
```
