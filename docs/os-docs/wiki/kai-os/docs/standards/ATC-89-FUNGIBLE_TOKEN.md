# ATC-89 — Fungible Token Standard

> **Standard-ID:** ATC-89 (ehemals ATC-)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.3
> **Sprint:** 2.3 | **Issue:** #76 | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Economy Primitives  
> **Tier:** Tier 2 — Economy Primitives  

---

## 1. Überblick

Fungible Token Standard — ATCLang-natives Token-Interface analog ERC-20. Transfer, Approve, Allowance, Mint (gepaused), Burn. Transfer-Events für Indexer.

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/contracts/token.atc — Token-Contract-Template
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `transfer(to: Address, amount: u64) -> bool` | Transferiert Token | 20000 |
| `transfer_from(from: Address, to: Address, amount: u64) -> bool` | Transfer via Allowance | 25000 |
| `approve(spender: Address, amount: u64) -> bool` | Genehmigt Spender | 20000 |
| `mint(to: Address, amount: u64) -> bool` | Mintet neue Token (nur Owner) | 30000 |
| `burn(from: Address, amount: u64) -> bool` | Verbrennt Token | 20000 |
| `balance_of(addr: Address) -> u64` | Gibt Balance zurück | 100 |
| `total_supply() -> u64` | Gibt Total-Supply zurück | 100 |

---

## 3. Konfiguration

```toml
decimals: 18 | mintable: true | burnable: true | pausable: true | max_supply: optional
```

---

## 4. Context-Isolation

| Context | Verfügbar |
|---------|-----------|
| **Node** | ✅ Full |
| **Smart Contract** | ✅ (über API) |
| **Test** | ✅ Mock |

---

## 5. Testing

8+ Unit-Tests: Transfer, Approve, Transfer-From, Mint, Burn, Balance, Edge-Cases (Insufficient-Balance, Overflow)

### Test-Dateien

```
tests/consensus/
├── test_atc-89.atc    # Unit-Tests
└── test_atc-89_integration.atc  # Integration-Tests
```

### Coverage-Ziel: 90%+

---

## 6. Abhängigkeiten

| Abhängigkeit | Issue | Status |
|--------------|-------|--------|
| ATCLang Compiler (ATC-92) | #72 | 📋 |
| ATCLang VM (ATC-93) | #73 | 📋 |
| ATCLang Stdlib (ATC-94) | #81 | 📋 |

---

## 7. Sprint-Zuweisung

- **Sprint 2.3** — #76
- **Priorität:** HIGH (Sprint 2.1 Blocker für alle ATCLang-Migrationen)
- **Deliverable:** Implementierte Module + Unit-Tests

---

## Referenzen

- **Roadmap v2.0:** Sprint 2.3
- **Wiki:** Kap. 31 (Issue-Registry), Kap. 37 (P2P), Kap. 38 (Konsens)
- **AGENT_MASTERRULES.md:** Regel 0 (ATCLang First), Regel 1 (Reality-Check)

---

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-89 v1.0.0*
