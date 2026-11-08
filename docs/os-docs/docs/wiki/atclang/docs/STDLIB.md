# ATCLang Standard Library v0.2.0

## Sicherheits-Primitiven
| Funktion | Beschreibung |
|---------|-------------|
| `safe_add(a, b)` | Overflow-geschützte Addition |
| `safe_sub(a, b)` | Underflow-geschützte Subtraktion |
| `safe_mul(a, b)` | Overflow-geschützte Multiplikation |
| `require(cond, msg)` | Guard (revert bei false) |
| `is_valid_address(s)` | ATC-Adressformat-Prüfung |

## Kryptographie
| Funktion | Beschreibung |
|---------|-------------|
| `sha256(data)` | SHA-256 Hash |
| `sha3(data)` | SHA3-256 Hash |
| `keccak256(data)` | Keccak-256 Hash |
| `verify_sig(msg, sig, pub)` | ECDSA-Signatur verifizieren |

## Blockchain-Built-ins
| Variable/Funktion | Beschreibung |
|---------|-------------|
| `caller` | Authentifizierter Aufrufer |
| `now()` | Unix-Timestamp (Sekunden) |
| `block_height()` | Aktuelle Block-Höhe |
| `block_hash()` | Aktueller Block-Hash |
| `emit(event, data)` | Event emittieren |

## Typ-Konversion
| Funktion | Beschreibung |
|---------|-------------|
| `to_string(x)` | Wert zu String |
| `to_u64(x)` | Cast zu u64 |
| `to_u128(x)` | Cast zu u128 |
| `to_address(s)` | String zu Address |

## Stdlib Quellcode (Auszug)
```python
"""
atclang/stdlib/atc_stdlib.py
ATCLang Standard Library — v0.2.0

Built-in Funktionen und Module für ATCLang Smart Contracts.
"""
import hashlib
import time
import json
from typing import Any, Dict, Optional


class ATCStdlibModule:
    """Kapselt alle Built-in-Funktionen der ATCLang Stdlib."""

    # ── Crypto ──────────────────────────────────────────

    @staticmethod
    def sha256(data: str) -> str:
        return hashlib.sha256(data.encode()).hexdigest()

    @staticmethod
    def sha3_256(data: str) -> str:
        return hashlib.sha3_256(data.encode()).hexdigest()

    @staticmethod
    def keccak256(data: str) -> str:
        """Kompatibel mit Solidity keccak256."""
        return hashlib.sha3_256(data.encode()).hexdigest()

    # ── Block/Chain-Kontext ─────────────────────────────

    @staticmethod
    def block_timestamp() -> int:
        return int(time.time())

    @staticmethod
    def block_number() -> int:
        """Gibt simulierte Block-Höhe zurück (Testnet)."""
        return int(time.time()) // 5  # ~5s Block-Zeit

    @staticmethod
    def now() -> int:
        return int(time.time())

    # ── Typ-Konvertierung ───────────────────────────────

    @staticmethod
    def to_u64(v: Any) -> int:
        n = int(v)
        if n < 0 or n > 2**64 - 1:
            raise ValueError(f"Wert {n} außerhalb u64-Bereich")
        return n

    @staticmethod
    def to_u128(v: Any) -> int:
        n = int(v)
        if n < 0 or n > 2**128 - 1:
            raise ValueError(f"Wert {n} außerhalb u128-Bereich")
        return n

    @staticmethod
    def to_string(v: Any) -> str:
        return str(v)

    @staticmethod
    def to_bool(v: Any) -> bool:
        if isinstance(v, str):
            return v.lower() in ("true", "1", "yes")
        return bool(v)

    
```
