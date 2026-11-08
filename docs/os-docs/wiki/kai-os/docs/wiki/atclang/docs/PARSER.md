# ATCLang Parser — Recursive Descent

## Übersicht
Der ATCLang Parser verarbeitet den Token-Stream des Lexers zu einem **Abstract Syntax Tree (AST)**.
Implementiert als rekursiver Descent-Parser ohne externe Abhängigkeiten.

## AST-Knoten-Typen
| Knoten | Beschreibung |
|--------|-------------|
| `ContractNode` | Smart Contract Definition |
| `FunctionNode` | Funktions-Deklaration |
| `StateNode` | State-Variable |
| `IfNode` | Bedingte Verzweigung |
| `ForNode` | Schleifen |
| `AssignNode` | Zuweisung |
| `BinaryOpNode` | Binäre Operation |
| `CallNode` | Funktionsaufruf |
| `EmitNode` | Event-Emission |
| `RequireNode` | Guard-Assertion |
| `ReturnNode` | Rückgabe |
| `StructNode` | Struct-Deklaration |
| `EnumNode` | Enum-Deklaration |

## Syntax-Regeln
```
program       := (contract | struct | enum | use)*
contract      := 'contract' IDENT '{' (state | fn)* '}'
fn            := 'fn' IDENT '(' params? ')' ('->' type)? block
state         := 'state' IDENT ':' type ('=' expr)?
block         := '{' stmt* '}'
stmt          := (let | assign | if | for | while | return | require | emit | expr) ';'?
expr          := literal | IDENT | call | binary | unary | index
```

## Parser Quellcode (Auszug)
```python
"""
ATCLang Parser — Recursive Descent Parser
Wandelt Token-Liste in einen AST um
Version: 0.1.0-alpha
"""

from typing import List, Optional
from .ast_nodes import *
from ..lexer.lexer import ATCLexer, Token, TT


class ATCParser:
    """
    Recursive Descent Parser für ATCLang.
    Produziert einen vollständigen AST.
    """

    def __init__(self, tokens: List[Token]):
        self.tokens  = [t for t in tokens if t.type not in (TT.NEWLINE, TT.COMMENT)]
        self.pos     = 0

    def error(self, msg: str):
        tok = self.current()
        raise SyntaxError(f"[ATCLang Parser] {msg} @ Zeile {tok.line}:{tok.col} (bekam: {tok.type.name} = {tok.value!r})")

    def current(self) -> Token:
        return self.tokens[self.pos] if self.pos < len(self.tokens) else self.tokens[-1]

    def peek(self, offset=1) -> Token:
        idx = self.pos + offset
        return self.tokens[idx] if idx < len(self.tokens) else self.tokens[-1]

    def advance(self) -> Token:
        tok = self.current()
        self.pos += 1
        return tok

    def check(self, ttype: TT, value=None) -> bool:
        tok = self.current()
        if tok.type != ttype:
            return False
        if value is not None and tok.value != value:
            return False
        return True

    def expect(self, ttype: TT, value=None) -> Token:
        if not self.check(ttype, value):
            exp = f"{ttype.name}" + (f"('{value}')" if value else "")
            self.error(f"Erwartet {exp}")
        ret
```

## AST-Nodes (Auszug)
```python
"""
ATCLang AST-Nodes — Abstract Syntax Tree
Alle Knoten-Typen der ATCLang Grammatik
Version: 0.1.0-alpha
"""

from dataclasses import dataclass, field
from typing import List, Optional, Any


# ── Basis-Knoten ─────────────────────────────────────────
class ASTNode:
    line: int = 0
    col:  int = 0


# ── Ausdrücke (Expressions) ──────────────────────────────
@dataclass
class IntLiteral(ASTNode):
    value: int
    line:  int = 0
    col:   int = 0

@dataclass
class FloatLiteral(ASTNode):
    value: float
    line:  int = 0
    col:   int = 0

@dataclass
class StringLiteral(ASTNode):
    value: str
    line:  int = 0
    col:   int = 0

@dataclass
class BoolLiteral(ASTNode):
    value: bool
    line:  int = 0
    col:   int = 0

@dataclass
class NullLiteral(ASTNode):
    line:  int = 0
```
