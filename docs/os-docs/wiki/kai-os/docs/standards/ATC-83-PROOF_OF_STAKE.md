# ATC-83 — Proof of Stake (PoS)

> **Standard-ID:** ATC-83 (ehemals ATC-1002)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.1
> **Sprint:** 2.1 | **Issue:** #74 | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Consensus Primitives  
> **Tier:** Tier 1 — Consensus Primitives  

---

## 1. Überblick

Proof of Stake — Validator-Selection basierend auf gestaktem Token-Volumen. Hybrid-Konsens kombiniert PoH (Zeit) mit PoS (Validator-Selection). Sliding-Window-Voting-Power mit Flash-Loan-Schutz (AD-003).

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/consensus/pos.atc — Validator-Selection
modules/consensus/staking.atc — Stake-Management
modules/consensus/slashing.atc — Slashing-Conditions
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `pos_select_validator(poh_slot: u64, stakes: &Map<Address, u64>) -> Address` | Wählt Validator für Slot (VRF-basiert) | 200 |
| `pos_register_validator(stake: u64) -> void` | Registriert als Validator mit Stake | 500 |
| `pos_unregister_validator() -> u64` | Deregistriert und gibt Stake zurück (7d Lockup) | 100 |
| `slash_validator(addr: Address, offense: SlashOffense) -> u64` | Slasht Validator bei Fehlverhalten | 1000 |
| `get_voting_power(addr: Address, snapshot_block: u64) -> u64` | Voting-Power zu Snapshot-Block (Flash-Loan-Schutz) | 50 |

---

## 3. Konfiguration

```toml
min_stake: 10000 ATC | lockup_period: 7 days | slashing_rate: 10% | snapshot_interval: 1000 blocks
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

8+ Unit-Tests: Validator-Selection, Staking, Slashing, Snapshot-Voting (Flash-Loan-Schutz), Lockup

### Test-Dateien

```
tests/consensus/
├── test_atc-83.atc    # Unit-Tests
└── test_atc-83_integration.atc  # Integration-Tests
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

- **Sprint 2.1** — #74
- **Priorität:** HIGH (Sprint 2.1 Blocker für alle ATCLang-Migrationen)
- **Deliverable:** Implementierte Module + Unit-Tests

---

## Referenzen

- **Roadmap v2.0:** Sprint 2.1
- **Wiki:** Kap. 31 (Issue-Registry), Kap. 37 (P2P), Kap. 38 (Konsens)
- **AGENT_MASTERRULES.md:** Regel 0 (ATCLang First), Regel 1 (Reality-Check)

---

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-83 v1.0.0*
