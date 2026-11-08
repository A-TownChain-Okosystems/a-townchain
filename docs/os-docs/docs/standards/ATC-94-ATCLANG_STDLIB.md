# ATC-94 — ATCLang Standard Library

> **Standard-ID:** ATC-94
> **Status:** ✅ ACCEPTED — 6 Module implementiert, 44 Tests GRÜN
> **Sprint:** 2.1 (vorzeitig aus 2.5) | **Issue:** #81 | **Wiki:** Kap. 66
> **Autor:** Aurora (MasterBrain · Base44) + Michael Wroblewski
> **Stand:** 05.07.2026 | Version 0.3.0
> **Kategorie:** ATCLang | **Tier:** Tier 0 — ATCLang Core

---

## 1. Überblick

ATCLang Standard Library — 6 Module mit kryptografischen Operationen, Datenstrukturen, I/O, Mathematik, Encoding und Blockchain-Primitiven.

### Implementierung

```
atclang/stdlib/
├── __init__.py       — Package Exports (14 Klassen)
├── crypto.py         — 154 Zeilen, 13 Tests ✅
├── collections.py    — 218 Zeilen, 7 Tests ✅
├── io.py             — 106 Zeilen, 4 Tests ✅
├── math.py           — 130+ Zeilen, 5 Tests ✅
├── encoding.py       — 209 Zeilen, 7 Tests ✅
├── primitives.py     — 243 Zeilen, 7 Tests ✅
├── wallet.py         — 77 Zeilen (bestehend)
├── chain.py          — 40 Zeilen (bestehend)
├── string.py         — 39 Zeilen (bestehend)
└── atc_stdlib.py     — 73 Zeilen (Builtins)
```

**44 Stdlib-Tests + 113 ATCLang Tests = 157 total — alle GRÜN ✅**

---

## 2. Module

### 2.1 crypto (ATC::Crypto)

| Funktion | Beschreibung | Gas |
|----------|-------------|-----|
| `sha256(data) → str` | SHA-256 Hash → hex | 30 |
| `sha256_bytes(data) → bytes` | SHA-256 → raw bytes | 30 |
| `double_sha256(data) → str` | Double SHA-256 | 60 |
| `hmac_sha256(key, msg) → str` | HMAC-SHA256 | 50 |
| `base58_encode(data) → str` | Base58 Encode | 20 |
| `base58_decode(s) → bytes` | Base58 Decode | 20 |
| `base64_encode(data) → str` | Base64 Encode | 15 |
| `base64_decode(s) → bytes` | Base64 Decode | 15 |
| `hex_encode(data) → str` | Hex Encode | 10 |
| `hex_decode(s) → bytes` | Hex Decode | 10 |
| `generate_keypair() → (priv, pub)` | ECDSA Keypair | 1000 |
| `sign(msg, priv) → str` | Sign message | 100 |
| `verify(msg, sig, pub) → bool` | Verify signature | 100 |
| `random_bytes(n) → bytes` | Crypto random | 50 |
| `random_int(min, max) → int` | Crypto random int | 50 |
| `address_from_pubkey(pub) → str` | Derive ATC address | 30 |
| `is_valid_address(addr) → bool` | Validate address | 5 |

**Tests:** 13 (SHA-256, Base58/64 roundtrip, Hex roundtrip, Sign/Verify, Address)

### 2.2 collections (ATC::Collections)

| Funktion | Beschreibung | Gas |
|----------|-------------|-----|
| `map_new/get/set/delete/contains/keys/values/size` | Map Operations | 2-10 |
| `array_new/push/pop/get/set/len/contains/slice/reverse/sort` | Array Operations | 2-20 |
| `set_new/add/contains/remove/size/union/intersection/difference` | Set Operations | 2-10 |
| `queue_new/enqueue/dequeue/peek/size` | Queue (FIFO) | 2-10 |
| `stack_new/push/pop/peek/size` | Stack (LIFO) | 2-10 |

**Tests:** 7 (Map, Array, Set, Queue, Stack vollständige Operationen)

### 2.3 io (ATC::IO)

| Funktion | Beschreibung | Gas | Context |
|----------|-------------|-----|---------|
| `print(*args)` | Console output | 5 | All |
| `println(*args)` | Console mit newline | 5 | All |
| `format(template, *args)` | String format | 10 | All |
| `file_write(path, data)` | Datei schreiben | 50 | Node |
| `file_read(path)` | Datei lesen | 50 | Node |
| `file_exists(path)` | Existenz prüfen | 10 | Node |
| `file_append(path, data)` | An Datei anhängen | 50 | Node |
| `file_delete(path)` | Datei löschen | 50 | Node |
| `dir_create(path)` | Verzeichnis erstellen | 50 | Node |
| `dir_list(path)` | Verzeichnis auflisten | 20 | Node |

**Tests:** 4 (Print, Format, File Write/Read/Append)

### 2.4 math (ATC::Math)

| Funktion | Beschreibung | Gas |
|----------|-------------|-----|
| `add/sub/mul/div/mod/pow` | Basis-Arithmetik | 3-10 |
| `safe_add/sub/mul/div/mod` | Safe mit Overflow-Check | 3-5 |
| `mod_exp(base, exp, mod)` | Modulare Exponentiation | 50 |
| `sqrt(x)` | Quadratwurzel | 10 |
| `min/max/clamp` | Vergleich | 2 |
| `percentage(amount, bps)` | Basis Points | 10 |
| `is_power_of_two(n)` | Potenz-of-2 Check | 3 |
| `abs(x)` | Absolutwert | 2 |
| `gcd(a, b)` | GGT | 20 |
| `lcm(a, b)` | KGVT | 30 |

**Tests:** 5 (Safe add/sub/mul/div, mod_exp)

### 2.5 encoding (ATC::Encoding)

| Funktion | Beschreibung | Gas |
|----------|-------------|-----|
| `json_encode(obj) → str` | JSON serialisieren | 15 |
| `json_decode(s) → obj` | JSON deserialisieren | 15 |
| `cbor_encode(obj) → bytes` | CBOR serialisieren | 20 |
| `cbor_decode(data) → obj` | CBOR deserialisieren | 20 |
| `hex_encode(data) → str` | Hex encode | 10 |
| `hex_decode(s) → bytes` | Hex decode | 10 |
| `rlp_encode/decode` | DEPRECATED (→ CBOR) | 20 |

**CBOR Support:** int, string, bytes, list, map, null, bool (full encode/decode)

**Tests:** 7 (JSON roundtrip, CBOR int/string/list/map/none/bool, Hex roundtrip)

### 2.6 primitives (ATC::Primitives)

| Klasse | Beschreibung | Gas |
|--------|-------------|-----|
| `ATCAddress` | 35-char ATC address (ATC + 32 hex) | — |
| `ATCHash` | 64-char SHA-256 hash | 30 |
| `ATCSignature` | HMAC-SHA256 signature | 100 |
| `ATCTransaction` | sender, receiver, amount, gas, nonce, sig | 50 |
| `ATCBlockHeader` | number, prev_hash, merkle_root, timestamp | 50 |

**Tests:** 7 (Address, Hash, Transaction, BlockHeader, Sign/Verify cycle, JSON roundtrip)

---

## 3. Context-Isolation

| Context | Crypto | Collections | IO | Math | Encoding | Primitives |
|---------|--------|-------------|-----|------|----------|------------|
| **Node** | ✅ Full | ✅ Full | ✅ Full | ✅ Full | ✅ Full | ✅ Full |
| **Contract** | ✅ (via API) | ✅ Full | ❌ Restricted | ✅ Full | ✅ Full | ✅ Full |
| **Test** | ✅ Mock | ✅ In-Memory | ✅ Mock | ✅ Full | ✅ Full | ✅ Mock |

---

## 4. Implementation-Reihenfolge

```
primitives → crypto → math → collections → encoding → io
```

Diese Reihenfolge minimiert Abhängigkeiten: primitives definiert Basistypen, crypto nutzt diese, math ist unabhängig, etc.

---

## 5. Abhängigkeiten

| Abhängigkeit | Issue | Status |
|--------------|-------|--------|
| ATCLang Compiler (ATC-92) | #72 | ✅ Implementiert |
| ATCLang VM (ATC-93) | #73 | ✅ Implementiert |
| ATCLang Test Framework (ATC-95) | — | ✅ Accepted |

---

## 6. Sprint-Zuweisung

- **Sprint 2.1** — Vorzeitig aktiviert (Pipeline + Stdlib)
- **Sprint 2.5** — Full ATCLang-native Stdlib (Self-hosting)
- **Status:** 6 Module implementiert, 44 Tests GRÜN
- **Nächste Schritte:** ATCLang-native .atc Module, VM Integration (STDLIB_DISPATCH)

---

## Referenzen

- **Code:** `atclang/stdlib/` (6 Module, ~930 Zeilen)
- **Tests:** `tests/test_stdlib.py` (44 Tests, alle GRÜN)
- **Wiki:** Kap. 66 (ATCLang Stdlib Technische Referenz)
- **AGENT_MASTERRULES.md:** Regel 0 (ATCLang First), Regel 13 (Migration)

---

*Spezifikation: Aurora (MasterBrain · Base44) + Michael Wroblewski · 05.07.2026 · ATC-94 v0.3.0*
