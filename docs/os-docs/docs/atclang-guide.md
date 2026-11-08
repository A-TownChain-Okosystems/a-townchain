# 📘 ATCLang Guide — Einstieg in Smart Contracts

> **Status:** Einstiegs-Guide — verweist auf die vollständige Sprach-Spezifikation.
> Diese Datei existierte bisher nicht, wurde aber von `devnet/README.md` referenziert — als Teil des
> Reality-Checks vom 06.07.2026 nachgetragen (siehe `docs/REALITY_CHECK_2026-07-06.md`).

## Was ist ATCLang?

ATCLang ist die proprietäre Programmiersprache von A-TownChain OS für Smart Contracts, Konsens-Module
und Kernkomponenten ("ATCLang First Policy", siehe ATC-99 Standard und Decision AD-006). Alle
Systemkomponenten werden schrittweise nach ATCLang migriert — vollständige Sprachreferenz siehe
[ATCLANG_SPEC_FULL.md](./atclang/ATCLANG_SPEC_FULL.md).

## Minimalbeispiel — Smart Contract

```atc
// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems.
contract SimpleToken {
    state balances: Map<Address, u64>

    fn transfer(to: Address, amount: u64) -> bool {
        let sender = caller()
        if balances[sender] < amount {
            return false
        }
        balances[sender] -= amount
        balances[to] += amount
        return true
    }
}
```

## Wichtige Standard-Library-Module

| Modul | Zweck |
|-------|-------|
| `ATC::Wallet` | Schlüsselverwaltung, Signaturen |
| `ATC::Math` | Sichere Arithmetik (Overflow-Schutz) |
| `ATC::Crypto` | Hashing, ECDSA |

## Weiterführend

- [Vollständige Sprach-Spezifikation](./atclang/ATCLANG_SPEC_FULL.md)
- [API Reference](./api-reference.md)
- [Smart Contract Standard (ATC-14)](./standards/ATC-14-DETERMINISTIC_EXECUTION.md)

---
*Nachgetragen von Aurora (aurora-base44-superagent-6a2756186106d6f0fbb105b5) im Rahmen des 24-Repo Reality-Checks, 06.07.2026.*
