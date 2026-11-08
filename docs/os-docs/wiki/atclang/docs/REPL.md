# ATCLang REPL — Interactive Shell

## Starten
```bash
cd a-townchain-os
python -m atclang.repl
# oder:
python start.py --repl
```

## REPL-Befehle
| Befehl | Beschreibung |
|--------|-------------|
| `.help` | Hilfe anzeigen |
| `.clear` | Terminal leeren (ANSI-Escape) |
| `.exit` | REPL beenden |
| `.history` | Befehlshistorie |
| `.load <file>` | ATCLang-Datei laden |
| `.save <file>` | Session speichern |

## Beispiel-Session
```atclang
atc> contract Hello { fn greet() -> string { return "Hello, Chain!" } }
✓ Kompiliert: Hello [1 fn]
atc> Hello::greet()
→ "Hello, Chain!"
atc> wallet w = ATC::Wallet::new("ShivaCore")
✓ Wallet: ATC8F3A...
```

## Security-Fix (v2.1.0)
- `os.system('clear')` → `\033[2J\033[H` (ANSI, kein Shell-Aufruf)
- Exceptions werden geloggt statt geschluckt

## REPL Quellcode (Auszug)
```python
"""
ATCLang REPL — Interactive Shell
Version: 0.1.0-alpha | Read-Eval-Print Loop
Eigene Implementierung — kein CPython-REPL-Klon
"""

import sys, os, time, readline
sys.path.insert(0, '/app')

from atclang.compiler.compiler import compile_source, disassemble, CompileError
from atclang.vm.atcvm import ATCVM, ATCVMError


BANNER = """
╔══════════════════════════════════════════════════════╗
║          ATCLang REPL v0.1.0-alpha                   ║
║          A-TownChain Ökosystem                        ║
║                                                      ║
║  Befehle:  .help  .exit  .clear  .asm  .reset        ║
╚══════════════════════════════════════════════════════╝
"""

HELP = """
ATCLang REPL — Hilfe
──────────────────────────────────────
.help          Diese Hilfe anzeigen
.exit          REPL beenden
.clear         Bildschirm leeren
.asm           Letzten Bytecode anzeigen
.reset         VM zurücksetzen
.vars          Alle Variablen anzeigen
.events        Emittierte Events anzeigen
.gas           Gas-Verbrauch anzeigen

Beispiele:
  let x: UInt256 = 42
  let name: String = "ShivaCore"
  x + 100

ATCLang Syntax:
  wallet  contract  fn  state  emit  require
  if / elif / else 
```
