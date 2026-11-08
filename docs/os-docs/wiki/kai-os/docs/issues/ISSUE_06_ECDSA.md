# 📄 Issue #6 — ECDSA Signatur (Sichere TX-Autorisierung)

> **Labels:** security · backend · priority:high
> **Priorität:** 🔴 High · **Milestone:** v2.1.0
> **Referenz:** [GitHub Issue #6](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues/6)

---

## Ziel

Alle Transaktionen und NFT-Transfers müssen mit dem **Private Key des Senders signiert** werden. Ohne ECDSA-Verifikation kann aktuell jeder beliebige Transfers auslösen, wenn die Adresse bekannt ist — das ist ein kritisches Sicherheitsproblem.

---

## Sicherheitsanalyse — Aktueller Zustand

| Endpoint | Aktuell | Soll |
|----------|---------|------|
| `POST /api/wallet/send` | Keine Signatur-Prüfung | ECDSA Signatur required |
| `POST /api/game/shivamon/transfer` | Keine Signatur-Prüfung | ECDSA Signatur required |
| `POST /api/game/shivamon/battle` | Keine Auth | Owner-Signatur required |
| `POST /api/governance/vote` | — (noch nicht impl.) | ECDSA required |

---

## Technische Spezifikation

### ECDSA Implementation (secp256k1)

```python
# blockchain/wallet/ecdsa.py
from cryptography.hazmat.primitives.asymmetric import ec
from cryptography.hazmat.primitives import hashes, serialization
import hashlib, json

class ECDSASigner:

    @staticmethod
    def generate_keypair() -> tuple[bytes, bytes]:
        """Generiert ein echtes ECDSA secp256k1 Schlüsselpaar."""
        private_key = ec.generate_private_key(ec.SECP256K1())
        public_key  = private_key.public_key()
        priv_bytes  = private_key.private_bytes(
            serialization.Encoding.Raw,
            serialization.PrivateFormat.Raw,
            serialization.NoEncryption()
        )
        pub_bytes = public_key.public_bytes(
            serialization.Encoding.X962,
            serialization.PublicFormat.UncompressedPoint
        )
        return priv_bytes.hex(), pub_bytes.hex()

    @staticmethod
    def sign(tx_data: dict, private_key_hex: str) -> str:
        """Signiert eine Transaktion mit dem Private Key."""
        message     = json.dumps(tx_data, sort_keys=True).encode()
        msg_hash    = hashlib.sha256(message).digest()
        private_key = ec.derive_private_key(
            int(private_key_hex, 16), ec.SECP256K1()
        )
        signature = private_key.sign(msg_hash, ec.ECDSA(hashes.Prehashed()))
        return signature.hex()

    @staticmethod
    def verify(tx_data: dict, signature_hex: str, public_key_hex: str) -> bool:
        """Verifiziert eine Transaktion-Signatur."""
        try:
            message    = json.dumps(tx_data, sort_keys=True).encode()
            msg_hash   = hashlib.sha256(message).digest()
            public_key = ec.EllipticCurvePublicKey.from_encoded_point(
                ec.SECP256K1(), bytes.fromhex(public_key_hex)
            )
            public_key.verify(
                bytes.fromhex(signature_hex),
                msg_hash,
                ec.ECDSA(hashes.Prehashed())
            )
            return True
        except Exception:
            return False
```

### TX-Signatur-Format

```json
{
  "tx": {
    "from":      "ATC7F3A9B2C1D4E5F6A7B8C9D0E1F2A3B4C5",
    "to":        "ATC9B2C1D4E5F6A7B8C9D0E1F2A3B4C5D6E7",
    "amount":    150.0,
    "fee":       0.001,
    "nonce":     42,
    "timestamp": 1747691880
  },
  "signature":  "3045022100a3f9b2c1....",
  "public_key": "04a3f9b2c1d4e5f6..."
}
```

### Backend Middleware

```python
# gateway/middleware/signature_verify.py
from blockchain.wallet.ecdsa import ECDSASigner

def require_signature(f):
    @wraps(f)
    def decorated(*args, **kwargs):
        data      = request.json or {}
        tx        = data.get("tx", {})
        signature = data.get("signature")
        pub_key   = data.get("public_key")
        if not ECDSASigner.verify(tx, signature, pub_key):
            return jsonify({"error": "Invalid signature"}), 401
        return f(*args, **kwargs)
    return decorated
```

---

## Aufgaben

- [ ] `blockchain/wallet/ecdsa.py` — ECDSA secp256k1 implementieren
- [ ] `ATCKeyGenerator` auf echtes ECDSA umstellen
- [ ] `gateway/middleware/signature_verify.py` — Signatur-Middleware
- [ ] `POST /api/wallet/send` — Signatur-Prüfung aktivieren
- [ ] `POST /api/game/shivamon/transfer` — Signatur-Prüfung aktivieren
- [ ] Nonce-System (verhindert Replay-Attacks)
- [ ] Frontend: Signatur im Wallet-UI generieren
- [ ] `cryptography` zu `requirements.txt` hinzufügen
- [ ] Tests: `tests/test_ecdsa.py`

---

## Akzeptanzkriterien

- [ ] Transfers ohne gültige Signatur werden abgelehnt (HTTP 401)
- [ ] Replay-Attacks durch Nonce verhindert
- [ ] Tests: Sign + Verify korrekt, falsche Signatur abgelehnt
- [ ] Kein Breaking Change für bestehende Dev-Flows (Dev-Mode ohne Signatur via Flag)
