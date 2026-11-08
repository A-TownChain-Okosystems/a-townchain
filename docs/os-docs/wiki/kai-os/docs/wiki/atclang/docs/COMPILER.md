# ATCLang Compiler v0.2.0

## Compiler-Pipeline
```
Quellcode (string)
     ↓
[1] Lexer       → Token-Stream
     ↓
[2] Parser      → AST (Abstract Syntax Tree)
     ↓
[3] Type-Check  → Typ-Validierung
     ↓
[4] Optimizer   → Konstanten-Faltung, Dead-Code-Elimination
     ↓
[5] Code-Gen    → ATCLang Bytecode
     ↓
[6] VM          → Ausführung
```

## Security-Checks (v2.1.0)
- `MAX_SOURCE_SIZE = 1 MB` — verhindert DoS
- `MAX_COMPILE_DEPTH = 64` — verhindert Stack-Overflow
- `MAX_FUNCTIONS = 512` — verhindert Ressourcen-Erschöpfung
- Null-Byte-Erkennung
- Block-Count-Limit

## Label-Cache (Performance)
```python
self._label_cache = {}  # O(1) statt O(n) Label-Lookup
```

## Compiler Quellcode (Auszug)
```python
"""
ATCLang Compiler — AST → ATC-Bytecode
Version: 0.1.0-alpha | Komplett selbst geschrieben
Kein LLVM-Klon, kein GCC-Port — eigener Code-Generator
"""

import sys, os
sys.path.insert(0, '/app')

from dataclasses import dataclass, field
from typing import List, Dict, Optional, Tuple
from atclang.parser.ast_nodes import *
from atclang.vm.atcvm import Instruction, OP


# ══════════════════════════════════════════════════════════
#  BYTECODE-FORMAT (.atcb)
# ══════════════════════════════════════════════════════════

MAGIC     = b"ATCB"   # Magic Bytes
VERSION   = b"\x01\x00"  # v1.0

@dataclass
class CompiledModule:
    name:         str
    instructions: List[Instruction]
    constants:    List[object]
    functions:    Dict[str, List[Instruction]]
    exports:      List[str]
    source_map:   List[Tuple[int, int, int]]   # (instr_idx, line, col)

    def summary(self) -> str:
        return (
            f"Module '{self.name}' | "
            f"{len(self.instructions)} Instrs | "
            f"{len(self.functions)} Fns | "
            f"{len(self.constants)} Konstanten"
        )


# ══════════════════════════════════════════════════════════
#  SYMBOL-TABELLE
# ══════════════════════════════════════════════════════════

@dataclass
class Symbol:
    name:   str
    kind:   str    # "local" | "global" | "function" | "contract" | "state"
    index:  int
    typ:    str = ""

class SymbolTable:
    def __init__(self, parent=None):
        self.symbols: Dict[str, Symbol] = {}
        self.parent  = parent
        self._next   = 0

    def define(self, name: str, kind: str, typ: str = "") -> Symbol:
        sym = Symbol(name, kind, self._next, typ)
        self.symbols[name] = sym
        self._next += 1
        return sym

    def resolve(self, name: str) -> Optional[Symbol]:
        if name in self.symbols:
            return self.symbols[name]
        if self.parent:
            return self.parent.resolve(name)
        return None

    def child(self) -> 'SymbolTable':
```
