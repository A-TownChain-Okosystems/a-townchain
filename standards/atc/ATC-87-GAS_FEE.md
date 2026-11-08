# ATC-87 — Gas Fee (EIP-1559)

> **Standard-ID:** ATC-87 (ehemals ATC-)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.3
> **Sprint:** 2.3 | **Issue:** #76 | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Economy Primitives  
> **Tier:** Tier 2 — Economy Primitives  

---

## 1. Überblick

Gas Fee Model — EIP-1559-basiertes Gebührenmodell mit Base-Fee + Priority-Fee. Base-Fee wird verbrannt, Priority-Fee geht an Validator. Dynamische Anpassung basierend auf Block-Auslastung.

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/economy/gas.atc — Gas-Berechnung
modules/economy/fee_market.atc — Base-Fee-Adjustment
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `calculate_gas_fee(base_fee: u64, priority_fee: u64, gas_used: u64) -> u64` | Berechnet totale Gas-Kosten | 10 |
| `adjust_base_fee(parent_gas: u64, target_gas: u64, parent_base_fee: u64) -> u64` | Passt Base-Fee an | 20 |
| `estimate_gas(tx: &Transaction) -> u64` | Schätzt Gas-Verbrauch einer Transaktion | 100 |
| `burn_fee(base_fee: u64, gas_used: u64) -> void` | Verbrennt Base-Fee-Anteil | 10 |
| `get_base_fee() -> u64` | Aktuelle Base-Fee | 10 |

---

## 3. Konfiguration

```toml
target_gas: 15000000 | max_gas: 30000000 | base_fee_max_change: 12.5% | elasticity: 2x
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

6+ Unit-Tests: Fee-Berechnung, Base-Fee-Adjustment, Burn, Edge-Cases (leerer Block, voller Block)

### Test-Dateien

```
tests/consensus/
├── test_atc-87.atc    # Unit-Tests
└── test_atc-87_integration.atc  # Integration-Tests
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

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-87 v1.0.0*
