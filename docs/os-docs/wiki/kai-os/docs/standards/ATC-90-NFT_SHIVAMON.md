# ATC-90 — NFT / Shivamon Standard

> **Standard-ID:** ATC-90 (ehemals ATC-)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.5
> **Sprint:** 2.5 | **Issue:** — | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Economy Primitives  
> **Tier:** Tier 2 — Economy Primitives  

---

## 1. Überblick

Non-Fungible Token Standard für Shivamon — ATCLang-natives NFT-Interface analog ERC-721. Mint, Transfer, Owner-Of, Token-URI, Metadata. Shivamon-spezifisch: Rarity, Attributes, Evolution.

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/contracts/shivamon.atc — NFT-Contract
modules/contracts/nft_metadata.atc — Metadata-Management
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `mint_shivamon(to: Address, attributes: &ShivamonAttributes) -> TokenId` | Mintet neues Shivamon-NFT | 50000 |
| `transfer_nft(to: Address, token_id: TokenId) -> bool` | Transferiert NFT | 25000 |
| `owner_of(token_id: TokenId) -> Address` | Gibt Owner zurück | 100 |
| `get_attributes(token_id: TokenId) -> ShivamonAttributes` | Gibt Shivamon-Attribute zurück | 200 |
| `evolve(token_id: TokenId, new_form: u8) -> bool` | Evolution des Shivamon | 30000 |
| `token_uri(token_id: TokenId) -> String` | Metadata-URI | 100 |

---

## 3. Konfiguration

```toml
max_supply: 10000 | evolution_levels: 3 | rarity_tiers: [Common, Rare, Epic, Legendary, Mythic]
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

8+ Unit-Tests: Mint, Transfer, Owner-Of, Evolution, Attributes, Edge-Cases (Double-Mint, Invalid-Evolution)

### Test-Dateien

```
tests/consensus/
├── test_atc-90.atc    # Unit-Tests
└── test_atc-90_integration.atc  # Integration-Tests
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

- **Sprint 2.5** — geplant
- **Priorität:** HIGH (Sprint 2.1 Blocker für alle ATCLang-Migrationen)
- **Deliverable:** Implementierte Module + Unit-Tests

---

## Referenzen

- **Roadmap v2.0:** Sprint 2.5
- **Wiki:** Kap. 31 (Issue-Registry), Kap. 37 (P2P), Kap. 38 (Konsens)
- **AGENT_MASTERRULES.md:** Regel 0 (ATCLang First), Regel 1 (Reality-Check)

---

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-90 v1.0.0*
