# 💎 Wallet & Key Generation — Technische Dokumentation
**Stand:** 09.06.2026 | **Version:** v2.1.0 | **Dateien:** `blockchain/wallet/`

---

## Überblick

A-TownChain verwendet ein proprietäres Wallet-System auf Basis von **ECDSA secp256k1**. Alle Wallet-Adressen beginnen mit dem Präfix `ATC` und sind 35 Zeichen lang.

```
Seed-Phrase (12 Wörter BIP39-kompatibel)
          │
     SHA-256 Hash
          │
   ECDSA secp256k1
   Private Key (32 Bytes)
          │
   Public Key (65 Bytes, uncompressed)
          │
   SHA-256 → RIPEMD-160 (oder SHA3-256)
          │
   ATC + Base58Check(20 Bytes) → ATC[32chars]
```

**Adress-Format:** `ATC` + 32 alphanumerische Zeichen = 35 Zeichen gesamt

---

## KeyGen — Schlüsselerzeugung

**Datei:** `blockchain/wallet/keygen.py` (139 Zeilen)

```python
class KeyGen:
    """
    Proprietäre Schlüsselerzeugung für A-TownChain.
    Basis: ECDSA secp256k1 + SHA-256 Adressableitung.
    """

    PREFIX    = "ATC"
    ADDR_LEN  = 32        # Zeichen nach dem Präfix
    WORDLIST_SIZE = 2048  # BIP39-kompatible Wortliste

    def generate(self) -> dict:
        """Erzeugt ein neues Schlüsselpaar + Adresse."""
        private_key = secrets.token_bytes(32)
        public_key  = self._derive_public_key(private_key)
        address     = self._derive_address(public_key)
        mnemonic    = self._generate_mnemonic(private_key)
        return {
            "address":     address,
            "private_key": private_key.hex(),
            "public_key":  public_key.hex(),
            "mnemonic":    mnemonic,
        }

    def _derive_address(self, public_key: bytes) -> str:
        """SHA-256 → Base58Check → ATC-Präfix."""
        h1   = hashlib.sha256(public_key).digest()
        h2   = hashlib.sha256(h1).digest()
        raw  = base58.b58encode_check(h2[:20]).decode()
        return self.PREFIX + raw[:self.ADDR_LEN]

    def restore_from_mnemonic(self, mnemonic: str) -> dict:
        """Wallet aus Seed-Phrase wiederherstellen."""
        ...

    def restore_from_private_key(self, priv_hex: str) -> dict:
        """Wallet aus Private Key wiederherstellen."""
        ...
```

**Unterstützte Operationen:**
| Methode | Beschreibung |
|---------|-------------|
| `generate()` | Neues Schlüsselpaar erzeugen |
| `restore_from_mnemonic(phrase)` | Aus 12-Wort-Phrase wiederherstellen |
| `restore_from_private_key(hex)` | Aus Hex-Private-Key wiederherstellen |
| `validate_address(addr)` | Adresse validieren |
| `derive_child_key(index)` | Hierarchische Ableitung (HD Wallet) |

---

## ECDSA — Transaktionssignierung

**Datei:** `blockchain/wallet/ecdsa.py` (71 Zeilen)

```python
class ECDSASigner:
    """
    ECDSA-Signaturen für A-TownChain Transaktionen.
    Kurve: secp256k1 (identisch mit Bitcoin/Ethereum)
    """

    CURVE = "secp256k1"

    def sign(self, message: str, private_key_hex: str) -> str:
        """Signiert eine Nachricht mit dem Private Key."""
        # message → SHA-256 Hash → ECDSA-Signatur (DER-Format)
        msg_hash   = hashlib.sha256(message.encode()).digest()
        private_key = bytes.fromhex(private_key_hex)
        signature   = self._ecdsa_sign(msg_hash, private_key)
        return signature.hex()

    def verify(self, message: str, signature_hex: str, public_key_hex: str) -> bool:
        """Verifiziert eine Signatur gegen den Public Key."""
        msg_hash   = hashlib.sha256(message.encode()).digest()
        signature  = bytes.fromhex(signature_hex)
        public_key = bytes.fromhex(public_key_hex)
        return self._ecdsa_verify(msg_hash, signature, public_key)
```

---

## Transaktionsformat

Jede ATC-Transaktion enthält:

```json
{
  "from":      "ATCabc...xyz",
  "to":        "ATCdef...uvw",
  "amount":    1000000,
  "fee":       1000,
  "nonce":     42,
  "timestamp": 1749484200,
  "data":      "optional contract call data",
  "signature": "3045022100...",
  "public_key":"04a1b2c3..."
}
```

| Feld | Typ | Beschreibung |
|------|-----|-------------|
| `from` | ATC-Adresse (35 Zeichen) | Sender |
| `to` | ATC-Adresse (35 Zeichen) | Empfänger |
| `amount` | uint64 (in ATC-Satoshi) | 1 ATC = 10^8 Satoshi |
| `fee` | uint64 | Mindest-Gebühr: 1000 Satoshi |
| `nonce` | uint64 | Verhindert Replay-Angriffe |
| `signature` | ECDSA DER hex | 71-72 Bytes |

---

## Sicherheitshinweise

| Risiko | Maßnahme |
|--------|---------|
| Private Key Exposure | Nie im Klartext speichern — immer verschlüsselt (AES-256-GCM) |
| Seed Phrase | Offline auf Papier/Metall — niemals digital |
| Replay-Angriff | Nonce-System: jede TX hat eindeutigen Nonce |
| Quantum-Risiko | Roadmap: v3.0 — Post-Quantum Signaturen (CRYSTALS-Dilithium) |

---

## REST API

Alle Wallet-Operationen über das API-Gateway (Port 4000):

```
POST /api/wallet/new          → Neue Wallet generieren
POST /api/wallet/restore      → Aus Mnemonic wiederherstellen
GET  /api/wallet/balance/:addr → Balance abfragen
POST /api/wallet/sign          → Transaktion signieren
POST /api/wallet/send          → Transaktion senden
GET  /api/wallet/history/:addr → TX-Geschichte
```
