# ATC-92 — ATCLang Language Specification

> **Standard-ID:** ATC-92
> **Status:** ✅ IMPLEMENTED — Pipeline funktionsfähig (Lexer → Parser → Compiler → VM), 113 Tests GRÜN
> **Sprint:** 2.1 | **Issue:** #72 | **Wiki:** Kap. 31, 66
> **Autor:** Aurora (MasterBrain · Base44) + Michael Wroblewski
> **Stand:** 05.07.2026 | Version 0.3.0-alpha
> **Kategorie:** ATCLang | **Tier:** Tier 0 — ATCLang Core

---

## 1. Überblick

ATCLang ist eine statisch typisierte, blockchain-native Programmiersprache. Sie wurde spezifisch für das A-TownChain Ökosystem entwickelt und ist die einzige Programmiersprache im System (AD-006: ATCLang First).

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Self-hosting Ziel (aktuell Python-Stub → ATCLang Migration)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard, Chain-ID 9000)
5. **Stack-based VM** — Register-basierte Ausführung würde Konsens brechen

### Implementierung

```
atclang/
├── lexer/lexer.py        — 563 Zeilen, 18 Funktionen, ✅ funktionsfähig
├── parser/parser.py      — 399 Zeilen, 26 Methoden, ✅ funktionsfähig
├── parser/ast_nodes.py   — 264 Zeilen, 36 AST-Klassen
├── compiler/compiler.py  — 540+ Zeilen, ✅ funktionsfähig
├── vm/atcvm.py           — 887 Zeilen, 105 Opcodes, ✅ funktionsfähig
├── stdlib/               — 6 Module (geplant), aktuell Python-Builtins
├── v03/                  — v0.3 Features (async/await, Generics, Closures)
├── repl/repl.py          — 184 Zeilen, Interactive Shell
└── programs/             — 15 .atc Dateien (Contracts, Programme)
```

---

## 2. Token-Typen

| Kategorie | Tokens |
|-----------|--------|
| **Literale** | INT, FLOAT, STRING, BOOL, HEX_INT, OCTAL_INT, BIN_INT, BYTES_LIT |
| **Bezeichner** | IDENT, KEYWORD, TYPE |
| **ATC-Namespace** | ATC_STD (`ATC::Hash::sha3`) |
| **Arithmetik** | PLUS, MINUS, STAR, SLASH, PERCENT, STARSTAR, PLUSEQ, MINUSEQ, STAREQ |
| **Vergleich** | EQEQ, NEQ, LT, GT, LTE, GTE |
| **Logik** | AND, OR, NOT |
| **Bitwise** | AMP, PIPE, CARET, TILDE, SHL, SHR |
| **Delimiters** | LPAREN, RPAREN, LBRACE, RBRACE, LBRACKET, RBRACKET, COMMA, SEMI, COLON, DCOLON |
| **Spezial** | ARROW (`->`), DOT, EQ, AT (`@`), DOLLAR (`$`), QUESTION (`?`) |

### Keywords (35+)

```
wallet contract struct enum impl trait fn state let const pub priv static type interface
if else elif for while loop in break continue return match case
async await deploy call new delete import from as use
emit require event error assert genesis mint burn stake unstake vote transfer approve delegate
node consensus kernel process spawn
```

### Types (25+)

```
UInt8 UInt16 UInt32 UInt64 UInt128 UInt256 Int UInt Int8 Int16 Int32 Int64 Int128 Int256
Float Float32 Float64 Float128 Bool Char Byte String Bytes Str
Address Hash256 Hash512 PubKey PrivKey Signature TxHash BlockHash CID
Map List Set Array Vec Tuple
```

---

## 3. AST-Knoten (36 Klassen)

| Kategorie | Klassen |
|-----------|---------|
| **Literale** | IntLiteral, FloatLiteral, StringLiteral, BoolLiteral, NullLiteral |
| **Ausdrücke** | Identifier, BinaryOp, UnaryOp, Assignment, IndexAccess, DotAccess, NamespaceAccess, FunctionCall |
| **Deklarationen** | TypeAnnotation, Parameter, FunctionDef, StateField, EventDef, ErrorDef, ContractDef, WalletDef, ImportStatement |
| **Statements** | LetStatement, ReturnStatement, EmitStatement, RequireStatement, IfStatement, ForStatement, WhileStatement, BreakStatement, ContinueStatement, ExprStatement |
| **Programm** | Program (Top-Level Container) |

---

## 4. Syntax-Beispiel

```atclang
// Wallet erstellen
wallet myWallet = ATC::Wallet::new("ShivaCore")

// Contract mit State und Events
contract ShivaToken : ATC-89 {
    state balance: Map<Address, UInt256>
    state total_supply: UInt256
    
    event Transfer(from: Address, to: Address, amount: UInt256)
    error InsufficientBalance
    
    fn transfer(to: Address, amount: UInt256) -> Bool {
        require(balance[caller] >= amount, "Insufficient balance")
        balance[caller] -= amount
        balance[to] += amount
        emit Transfer(caller, to, amount)
        return true
    }
}

// Top-Level Expression
let x = 10
let y = 32
x + y  // → 42

// If/Else als Expression
if x > 5 { 100 } else { 0 }  // → 100

// Funktionen
fn double(n: UInt256) -> UInt256 {
    return n * 2
}
double(21)  // → 42

// Rekursion
fn fib(n: UInt256) -> UInt256 {
    if n <= 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
fib(10)  // → 55
```

---

## 5. Compiler-Pipeline

```
Source Code → Lexer (563 Zeilen) → Token Stream → Parser (399 Zeilen) → AST → Compiler (540+ Zeilen) → Bytecode → VM (887 Zeilen) → Result
```

### CompiledModule

| Feld | Typ | Beschreibung |
|------|-----|--------------|
| `name` | str | Modul-Name ("main") |
| `instructions` | List[Instruction] | Top-Level Bytecode |
| `constants` | List[object] | Konstanten-Pool |
| `functions` | Dict[str, List[Instruction]] | Funktionen (Name → Instruktionen) |
| `function_params` | Dict[str, List[str]] | Parameter-Namen pro Funktion |
| `exports` | List[str] | Öffentliche Funktionen |
| `source_map` | List[Tuple] | Instruktion → Source-Position |

---

## 6. Gas-Cost-Tabelle

| Operation | Gas-Cost |
|-----------|----------|
| `lex(source)` | 5 + 1/token |
| `parse(tokens)` | 10 + 2/node |
| `type_check(ast)` | 20 + 5/expr |
| `compile(ast)` | 30 + 10/expr |
| `optimize(bytecode)` | 15 + 3/instruction |

---

## 7. Context-Isolation

| Context | Verfügbar | Einschränkungen |
|---------|-----------|-----------------|
| **Node** | ✅ Full | Alle Operationen, Network, Storage |
| **Smart Contract** | ✅ (über API) | Kein direkter Network-Zugriff |
| **Test** | ✅ Mock | In-Memory Storage, Mock-Peers |

---

## 8. Testing

### Aktuelle Test-Ergebnisse: 113/113 GRÜN ✅

| Suite | Tests | Status |
|-------|-------|--------|
| TestATCLexer | 20 | ✅ |
| TestATCVM | 26 | ✅ |
| TestATCLangIntegration | 7 | ✅ |
| test_atclang_v03 | 7 | ✅ |
| unit/test_atclang | 53 | ✅ |

### Coverage-Ziel: 90%+ (aktuell ~85%)

---

## 9. Abhängigkeiten

| Abhängigkeit | Issue | Status |
|--------------|-------|--------|
| ATCLang VM (ATC-93) | #73 | ✅ Implementiert (887 Zeilen, 105 Opcodes) |
| ATCLang Stdlib (ATC-94) | #81 | 🔄 Teilweise (Python-Builtins, ATCLang-Module geplant) |
| ATCLang Test Framework (ATC-95) | — | 🔄 Teilweise (113 Tests, Framework geplant) |

---

## 10. Sprint-Zuweisung

- **Sprint 2.1** — #72
- **Priorität:** HIGH (Sprint 2.1 Blocker für alle ATCLang-Migrationen)
- **Status:** Pipeline funktionsfähig, Sprint 2.1 bei ~30%
- **Nächste Schritte:** Stdlib-Module implementieren, Type-Checker, Optimizer

---

## Referenzen

- **Roadmap v2.0:** Sprint 2.1
- **Wiki:** Kap. 31 (Issue-Registry), Kap. 66 (ATCLang Stdlib), Kap. 68 (Technische Gesamtdokumentation)
- **AGENT_MASTERRULES.md:** Regel 0 (ATCLang First), Regel 1 (Reality-Check), Regel 13 (Migration)
- **Code:** `atclang/` Root-Package (konsolidiert aus `modules/atclang/`)

---

*Spezifikation: Aurora (MasterBrain · Base44) + Michael Wroblewski · 05.07.2026 · ATC-92 v0.3.0-alpha*
