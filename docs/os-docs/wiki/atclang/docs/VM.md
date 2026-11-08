# ATCLang VM — Stack-basierte Ausführungsmaschine

## Architektur
- **Stack**: LIFO, max 1024 Elemente
- **Globals**: Contract-State (persistiert)
- **Call-Stack**: max 128 Frames (Rekursions-Schutz)
- **Gas**: max 10.000.000 pro TX
- **Memory**: isoliert pro Contract-Aufruf

## Opcodes (Auszug)
| Opcode | Beschreibung |
|--------|-------------|
| PUSH | Wert auf Stack legen |
| POP | Stack-Element entfernen |
| ADD, SUB, MUL, DIV | Arithmetik (overflow-geschützt) |
| LOAD_GLOBAL | State lesen |
| STORE_GLOBAL | State schreiben |
| CALL | Funktionsaufruf |
| RETURN | Rückgabe |
| REQUIRE | Guard (revert bei false) |
| EMIT | Event emittieren |
| SHA256, SHA3 | Kryptographie |

## Security
- Call-Depth-Limit: 128 (verhindert Stack-Overflow)
- Gas-Limit: 10M (verhindert DoS)
- Stack-Underflow: Exception bei leerem Stack

## VM Quellcode (Auszug)
```python
"""
ATCLang VM — Stack-basierte virtuelle Maschine
Version: 0.2.0 | A-TownChain Ökosystem
Erweitert für vollständige atcos_main.atc Ausführung.
"""

import hashlib, os, time, secrets, json, struct
from enum import IntEnum, auto
from dataclasses import dataclass, field
from typing import List, Any, Dict, Optional, Callable


# ══════════════════════════════════════════════════════════
#  OPCODES — VOLLSTÄNDIG
# ══════════════════════════════════════════════════════════

class OP(IntEnum):
    # ── Stack ──────────────────────────────────────────
    PUSH        = auto()   # PUSH <val>
    POP         = auto()   # POP
    DUP         = auto()   # Stack-Top duplizieren
    SWAP        = auto()   # Top zwei tauschen
    ROT         = auto()   # a b c → b c a

    # ── Arithmetik ─────────────────────────────────────
    ADD         = auto()
    SUB         = auto()
    MUL         = auto()
    DIV         = auto()
    MOD         = auto()
    POW         = auto()   # Potenz (**)
    NEG   
```
