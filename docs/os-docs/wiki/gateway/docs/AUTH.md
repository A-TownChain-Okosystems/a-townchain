# Gateway Authentifizierung

## API-Key Auth
```http
Header: X-API-Key: <32+ Zeichen>
```
- Minimum-Länge: 32 Zeichen
- Rotation empfohlen: alle 30 Tage
- Storage: SHA-256 Hash (nicht Plaintext)

## ECDSA Signatur (für TX-Endpoints)
```http
Header: X-Signature: <ecdsa_hex>
Header: X-Public-Key: <pubkey_hex>
Header: X-Nonce: <uint64>
```

## Replay-Schutz
- Nonce muss strikt monoton steigen
- Nonce wird 24h gecacht
- Chain-ID 9000 in Nachricht enthalten

## Beispiel (Python)
```python
from blockchain.wallet.ecdsa import ECDSASigner
signer = ECDSASigner()
msg = f'{{"to":"ATC...","amount":1000,"nonce":42,"chain_id":9000}}'
sig = signer.sign(msg, private_key)
headers = {
    "X-API-Key":    api_key,
    "X-Signature":  sig,
    "X-Public-Key": public_key,
    "X-Nonce":      "42"
}
```

## Rate-Limits
| Endpoint | Limit |
|----------|-------|
| `/api/wallet/send` | 5/min |
| `/api/ai/*` | 10/min |
| `/api/blockchain/*` | 100/min |
| Default | 100/min |
