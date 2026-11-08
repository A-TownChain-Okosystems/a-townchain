# ATCLang Lexer

## Token-Kategorien
- **Keywords**: `contract`, `fn`, `state`, `return`, `if`, `else`, `for`, `while`, `match`, `use`, `struct`, `enum`, `require`, `emit`, `let`
- **Types**: `u8`, `u16`, `u32`, `u64`, `u128`, `bool`, `string`, `bytes32`, `Address`, `Vec`, `Map`
- **Literals**: Integer, Float, String, Bool, Hex
- **Operators**: `+`, `-`, `*`, `/`, `%`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`, `!`
- **Delimiters**: `{`, `}`, `(`, `)`, `[`, `]`, `;`, `:`, `,`, `.`, `->`

## Quellcode (Auszug)
```python
"""
ATCLang Lexer — Tokenizer v0.2.0
Eigene Programmiersprache für das A-TownChain Ökosystem
Erweitert: Alle Keywords, Typen, Operatoren für atcos_main.atc
"""

from enum import Enum, auto
from dataclasses import dataclass
from typing import List, Optional


# ══════════════════════════════════════════════════════════
#  TOKEN-TYPEN
# ══════════════════════════════════════════════════════════

class TT(Enum):
    # Literale
    INT        = auto()
    FLOAT      = auto()
    STRING     = auto()
    BOOL       = auto()
    HEX_INT    = auto()    # 0xFF, 0xATC...
    OCTAL_INT  = auto()    # 0o755
    BIN_INT    = auto()    # 0b1010
    BYTES_LIT  = auto()    # b"..."

    # Bezeichner & Keywords
    IDENT      = auto()
    KEYWORD    = auto()

    # Typen
    TYPE       = auto()

    # ATC-Standard-Referenz
    ATC_STD    = auto()    # ATC::Hash::sha3 etc.

    # Operatoren — Arithmetik
    PLUS       = auto()    # +
    MINUS      = auto()    # -
    STAR       = auto()    # *
    SLASH      = auto()    # /
    PERCENT    = auto()    # %
    STARSTAR   = auto()    # **  (Potenz)
    PLUSEQ     = auto()    # +=
    MINUSEQ    = auto()    # -=
    STAREQ     = auto()    # *=
    SLAS
```
