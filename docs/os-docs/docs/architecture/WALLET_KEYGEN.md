# 🔑 Wallet Key Generation — Technische Dokumentation

> **Datei:** `blockchain/wallet/keygen.py`
> **Standard:** BIP39-kompatibel · ATC-Adressformat

---

## Überblick

```
Entropy (256 bit)
    │
    ▼ entropy_to_mnemonic()
Seed Phrase (24 Wörter)     ← BIP39 Wordlist (2048 Wörter)
    │
    ▼ mnemonic_to_seed() — PBKDF2-HMAC-SHA512 (2048 Iterationen)
512-bit Seed
    │
    ▼ seed_to_private_key() — HMAC-SHA256
Private Key (256 bit / 64 hex)
    │
    ▼ private_to_public_key() — SHA-256
Public Key (256 bit / 64 hex)
    │
    ▼ public_key_to_address()
ATC Adresse (ATC + 32 hex = 35 Zeichen)
```

---

## Seed Phrase Generierung

```python
# 256 Bit Entropy → 24 Wörter

entropy   = os.urandom(32)               # 32 Bytes = 256 Bit
checksum  = sha256(entropy)[0:1]         # 8-bit Checksum
combined  = entropy_bits + checksum_bits # 264 Bit gesamt
words     = [WORDLIST[combined[i*11:(i+1)*11]] for i in range(24)]
# 264 Bit / 11 Bit pro Wort = 24 Wörter
```

| Entropy-Bits | Wörter | Checksum-Bits |
|-------------|--------|---------------|
| 128 | 12 | 4 |
| 160 | 15 | 5 |
| 192 | 18 | 6 |
| 224 | 21 | 7 |
| **256** | **24** | **8** |

---

## Adress-Schema

```
ATC  +  [28 hex uppercase]  +  [4 hex Checksum]  =  35 Zeichen
 3        28                      4

Beispiel:
ATC  7F3A9B2C1D4E5F6A7B8C9D0E1F2A  3B4C

Derivation:
  step1    = sha256(public_key)
  step2    = sha256(step1)
  checksum = sha256(step2)[:4].upper()
  address  = "ATC" + step2[:28].upper() + checksum
```

---

## Sicherheitshinweise

| Aspekt | Maßnahme |
|--------|----------|
| Private Key | Nur einmalig anzeigen, nie speichern |
| Seed Phrase | Offline aufbewahren (Papier/Metall) |
| Passphrase | Optional, erhöht Sicherheit |
| PBKDF2 | 2048 Iterationen (brute-force-resistent) |
| Entropy | `os.urandom()` — kryptographisch sicher |

---

## Wallet wiederherstellen

```python
keygen = ATCKeyGenerator()

# Aus Seed Phrase
wallet = keygen.restore_from_mnemonic(
    mnemonic   = ["abandon", "ability", ...],   # 24 Wörter
    passphrase = "A-TownChain"                  # optional
)
# → gleiche Adresse wie beim Original
```

---

> **Dokument:** `docs/architecture/WALLET_KEYGEN.md`
> **Datum:** 2026-05-19 · **Autor:** A-TownChain-Okosystems × Aurora AI
