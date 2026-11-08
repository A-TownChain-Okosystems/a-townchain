# ATCLang — A-TownChain Language

**Version:** v0.4.0 | **Status:** In Entwicklung

## Ubersicht
ATCLang ist die native Smart-Contract-Sprache des A-TownChain OS.
Typisiert, kompiliert zu ATCLang Bytecode, lauft auf der ATC-VM.

## Komponenten
| Komponente | Datei | Status |
|------------|-------|--------|
| Lexer | atclang/lexer.py | OK |
| Parser | atclang/parser.py | OK |
| AST | atclang/ast.py | OK |
| Compiler | atclang/compiler.py | OK |
| VM | atclang/vm.py | OK |
| Type-Checker | atclang/type_checker.py | Issue #48 |
| Stdlib ATC:: | atclang/stdlib/ | Issue #48 |

## Offene Issues
- **#48** ATCLang v0.4.0 - Type System & Formal Verification

## Wiki-Kapitel
Kapitel 36 (ATCLang Spezifikation)

_Stand: 2026-06-11 | Aurora Auto-Sync_
