# ATC-88 — AMM (Constant Product)

> **Standard-ID:** ATC-88 (ehemals ATC-)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.3
> **Sprint:** 2.3 | **Issue:** #76 | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Economy Primitives  
> **Tier:** Tier 2 — Economy Primitives  

---

## 1. Überblick

Automated Market Maker — Constant-Product-Formel (x*y=k) für dezentralen Token-Austausch. Liquidity-Pools, Swap-Berechnung, Slippage-Schutz, LP-Token-Ausgabe.

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/contracts/dex.atc — AMM-Swap-Engine
modules/contracts/liquidity.atc — Liquidity-Pool-Management
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `swap_in(pool: &mut Pool, token_in: TokenId, amount_in: u64) -> u64` | Tauscht Token gegen Pool-Token | 30000 |
| `add_liquidity(pool: &mut Pool, amounts: &Map<TokenId, u64>) -> u64` | Fügt Liquidität hinzu, gibt LP-Token | 50000 |
| `remove_liquidity(pool: &mut Pool, lp_tokens: u64) -> Map<TokenId, u64>` | Entfernt Liquidität | 50000 |
| `get_price(pool: &Pool, token: TokenId) -> f64` | Aktueller Pool-Preis | 100 |
| `calculate_slippage(pool: &Pool, amount_in: u64, token_in: TokenId) -> f64` | Berechnet erwartete Slippage | 200 |

---

## 3. Konfiguration

```toml
fee_rate: 0.3% | min_liquidity: 1000 | slippage_protection: 1% max | lp_fee: 0.25%
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

8+ Unit-Tests: Swap, Add/Remove Liquidity, Slippage, Price-Calc, Edge-Cases (leerer Pool, Max-Swap)

### Test-Dateien

```
tests/consensus/
├── test_atc-88.atc    # Unit-Tests
└── test_atc-88_integration.atc  # Integration-Tests
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

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-88 v1.0.0*
