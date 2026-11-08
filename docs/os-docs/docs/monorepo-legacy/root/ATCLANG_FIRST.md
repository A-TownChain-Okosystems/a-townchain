# ⚡ ATCLang — Die einzige Programmiersprache des A-TownChain Ökosystems

> **Grundregel:** Alles ist ATCLang. Keine andere Programmiersprache.

---

## Architektur-Entscheidung: ATCLang First

ATCLang ist nicht eine von mehreren Sprachen.
ATCLang IST das A-TownChain Ökosystem.

```
Kein Python.
Kein Solidity.
Kein Rust.
Kein JavaScript.
Kein TypeScript.

→ Alles ist ATCLang.
```

---

## Was ATCLang leistet

### On-Chain (Smart Contracts)
```atc
// Kein Solidity — alles ATCLang
contract ATC Token {
    state balance: Map<Address, u256>
    state total_supply: u256

    fn transfer(to: Address, amount: u256) -> Result {
        require(balance[msg.sender] >= amount, "Insufficient balance")
        balance[msg.sender] -= amount
        balance[to] += amount
        emit Transfer(msg.sender, to, amount)
        Ok(())
    }
}
```

### Off-Chain (System-Module, Kernel, API)
```atc
// Kein Python — alles ATCLang
module consensus::poh {
    use crypto::sha3_256
    use std::time

    fn tick(prev_hash: Hash, seq: u64) -> Entry {
        let hash = sha3_256(prev_hash ++ seq.to_bytes())
        Entry { hash, seq, prev_hash }
    }
}
```

### VM / Kernel-Level (ShivaOS Syscalls)
```atc
// Kein C/Rust — alles ATCLang
syscall ATC_BALANCE(address: Address) -> u256 {
    ledger.get_balance(address)
}
```

---

## ATCLang Sprachversionen

| Version | Status | Features |
|---------|--------|---------|
| v0.1 | ✅ RELEASED | Grundtypen, Funktionen, Kontrollfluss |
| v0.2 | ✅ RELEASED | Module, Imports, Error-Handling |
| v0.3 | ✅ RELEASED | Async/Await, Generics, Closures |
| v0.4 | 🔄 IN DEV | Type System, Formal Verification |
| v1.0 | 📋 GEPLANT | Vollständige Spezifikation, zkATCLang |

---

## Migration — Was zu ersetzen ist

Der aktuelle Code-Stand enthält noch Legacy-Dateien in Python/Solidity.
Diese sind temporäre Stubs bis ATCLang v0.4/v1.0 die jeweiligen Features abdeckt.

### Migrationsstatus

| Bereich | Aktuell | Ziel | Sprint |
|---------|---------|------|--------|
| Consensus (poh, pow, pos) | Python | ATCLang | 2.3 |
| Smart Contracts (ATC-89, ATC-90) | Solidity | ATCLang | 2.5 |
| Gateway / Backend API | Python | ATCLang | 3.0 |
| ShivaOS Kernel / Syscalls | Python | ATCLang | 2.4 |
| DEX / AMM | Python | ATCLang | 2.3 |
| Bridge (ETH, SOL) | Python | ATCLang | 3.0 |
| Tests | Python pytest | ATCLang test::framework | 2.3 |

---

## Compiler-Architektur (Kap. 53, 36)

```
ATCLang Source (.atc)
       │
       ▼
  Lexer (Token-Stream)
       │
       ▼
  Parser (AST)
       │
       ▼
  Type Checker (v0.4+)
       │
       ▼
  IR Generator (ATC-IR)
       │
  ┌────┴────┐
  ▼         ▼
ATCVM     Native
(on-chain) (ShivaOS)
```

---

## Standards

- **ATC-92** — ATCLang Language Specification v1.0 (DRAFT)
- **ATC-93** — ATCLang VM Bytecode Format (DRAFT)
- **ATC-94** — ATCLang Standard Library (DRAFT)
- **ATC-95** — ATCLang Test Framework (DRAFT)

---

## AD-006 — Aufgelöst durch diese Entscheidung

Die offene Frage "Python vs. Substrate" ist damit beantwortet:
**Weder Python noch Substrate. Alles ist ATCLang.**

Python und Solidity sind temporäre Implementierungsstubs,
die schrittweise durch ATCLang ersetzt werden.

---

*Grundregel verankert von Michael Wroblewski · 2026-06-12*
*Durchgesetzt vom A-Town Ecosystem Brain (Aurora)*


---

## ATC-99 — ATCLang Universal Mandate

Die ATCLang-First-Policy ist durch **ATC-99 (ATCLang Universal Mandate)** als formeller ATC-Standard verbindlich festgeschrieben.

- **Standard-ID:** ATC-99
- **Status:** FINAL
- **Kategorie:** Language Policy (Tier 0 — Foundational)
- **Gilt für:** Alle Sprints 2.1–4.2d
- **Entscheidung:** AD-006 (RESOLVED)

**Alles wird in ATCLang programmiert. Keine Ausnahmen.**
