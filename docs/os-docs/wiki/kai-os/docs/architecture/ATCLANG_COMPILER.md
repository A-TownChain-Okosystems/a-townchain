# ATCLang Compiler & VM — Architektur

> **Stand:** 05.07.2026 | **Sprint:** 2.1 | **Standards:** ATC-92, 93, 94

## Übersicht

Der ATCLang-Compiler ist eine vollständige Toolchain bestehend aus Lexer, Parser, TypeChecker, Codegenerator, Optimizer und Virtual Machine.

## Compiler-Pipeline

```
Source Code (.atc) → Lexer → Parser → AST → TypeChecker → CodeGen → Optimizer → Bytecode → VM
```

## Module (19 Python-Dateien)

| Modul | Datei | Zeilen | Beschreibung |
|-------|-------|--------|--------------|
| Lexer | atclang/lexer/lexer.py | 571 | Tokenizer für ATCLang v0.3 |
| Parser | atclang/parser/parser.py | 889 | Recursive Descent Parser |
| AST | atclang/parser/ast_nodes.py | 330 | Abstract Syntax Tree Nodes |
| TypeChecker | atclang/compiler/type_checker.py | 506 | Statische Typ-Analyse |
| CodeGen | atclang/compiler/compiler.py | 560 | Bytecode-Generator |
| Optimizer | atclang/compiler/optimizer.py | 557 | Peephole + Dead Code Elimination |
| VM | atclang/vm/atcvm.py | 977 | Stack-based Virtual Machine |
| REPL | atclang/repl/repl.py | 183 | Interactive Read-Eval-Print Loop |
| v03 Features | atclang/v03/atclang_v03_features.py | 300 | async/await, Generics, Closures |

## Stdlib (10 Module)

| Modul | Datei | Zeilen | Beschreibung |
|-------|-------|--------|--------------|
| Crypto | atclang/stdlib/crypto.py | 154 | SHA-256, ECDSA, secp256k1 |
| Collections | atclang/stdlib/collections.py | 218 | List, Map, Set, Queue |
| IO | atclang/stdlib/io.py | 106 | File I/O, Print, Input |
| Math | atclang/stdlib/math.py | 137 | Arithmetik, Power, Sqrt |
| Encoding | atclang/stdlib/encoding.py | 209 | Base58, Hex, UTF-8 |
| Primitives | atclang/stdlib/primitives.py | 243 | u8–u128, i8–i64, bool, String |
| String | atclang/stdlib/string.py | 39 | String-Manipulation |
| Wallet | atclang/stdlib/wallet.py | 77 | BIP39, Address, Keys |
| Chain | atclang/stdlib/chain.py | 40 | Block, TX, Hash |
| Stdlib | atclang/stdlib/atc_stdlib.py | 68 | Registry für alle Module |

## ATCLang v0.3 Features

- `async`/`await` für asynchrone Operationen
- Generics (`List<T>`, `Map<K,V>`)
- Closures und Higher-Order Functions
- Modul-System (`import`, `export`)
- `contract` und `struct` Keywords
- `enum` Typen
- `event` Deklarationen
- Explizite Integer-Typen (u8, u16, u32, u64, u128, i8, i16, i32, i64)

## Statistik

- **19 Python-Module** (~5.800 Zeilen Compiler-Infrastruktur)
- **92 .atc Produktionsdateien** (15.936 Zeilen)
- **60 Tests** — alle GRÜN
- **92/92 Parse-Rate** (100%)

---

*ATCLang Compiler Architecture · Sprint 2.1 · 05.07.2026 · Aurora*
