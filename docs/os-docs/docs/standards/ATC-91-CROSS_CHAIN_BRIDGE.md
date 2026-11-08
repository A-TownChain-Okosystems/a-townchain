# ATC-91 — Cross-Chain Bridge

> **Standard-ID:** ATC-91 (ehemals ATC-)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.6
> **Sprint:** 2.6 | **Issue:** — | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Economy Primitives  
> **Tier:** Tier 2 — Economy Primitives  

---

## 1. Überblick

Cross-Chain Bridge Protocol — Multi-Sig Validator-Netzwerk für sicherte Cross-Chain-Asset-Transfers. Lock-Mint und Burn-Mint Mechanismen. Validator-Set mit 2/3-Threshold.

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/bridge/bridge.atc — Bridge-Contract
modules/bridge/validators.atc — Validator-Set-Management
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `lock_asset(target_chain: ChainId, token: TokenId, amount: u64, recipient: Address) -> BridgeId` | Lockt Asset auf Source-Chain | 50000 |
| `mint_wrapped(bridge_id: BridgeId, proofs: &Array<ValidatorProof>) -> bool` | Mintet wrapped Token auf Target-Chain | 80000 |
| `burn_wrapped(token: TokenId, amount: u64, target_chain: ChainId) -> BridgeId` | Verbrennt wrapped Token für Rücktransfer | 50000 |
| `unlock_asset(bridge_id: BridgeId, proofs: &Array<ValidatorProof>) -> bool` | Gibt gelockte Assets frei | 80000 |
| `verify_validator_proofs(proofs: &Array<ValidatorProof>) -> bool` | Verifiziert 2/3 Validator-Signaturen | 3000 × n |

---

## 3. Konfiguration

```toml
validator_threshold: 2/3 | lockup_period: 24h | supported_chains: [BTC, ETH, XDC] | fee: 0.1%
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

6+ Unit-Tests: Lock-Mint, Burn-Unlock, Proof-Verifikation, Edge-Cases (Insufficient-Proofs, Double-Mint)

### Test-Dateien

```
tests/consensus/
├── test_atc-91.atc    # Unit-Tests
└── test_atc-91_integration.atc  # Integration-Tests
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

- **Sprint 2.6** — geplant
- **Priorität:** HIGH (Sprint 2.1 Blocker für alle ATCLang-Migrationen)
- **Deliverable:** Implementierte Module + Unit-Tests

---

## Referenzen

- **Roadmap v2.0:** Sprint 2.6
- **Wiki:** Kap. 31 (Issue-Registry), Kap. 37 (P2P), Kap. 38 (Konsens)
- **AGENT_MASTERRULES.md:** Regel 0 (ATCLang First), Regel 1 (Reality-Check)

---

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-91 v1.0.0*
