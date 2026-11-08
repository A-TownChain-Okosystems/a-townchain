# ATC-84 — Fork Resolution

> **Standard-ID:** ATC-84 (ehemals ATC-1003)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.1
> **Sprint:** 2.1 | **Issue:** #74 | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Consensus Primitives  
> **Tier:** Tier 1 — Consensus Primitives  

---

## 1. Überblick

Fork Resolution — Löst Chain-Forks auf basis der Longest-Heaviest-Chain-Regel mit PoH-Gewichtung. Erkennt Forks, vergleicht kumulative Difficulty + PoH-Länge, und reorganisiert bei Bedarf.

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/consensus/fork_resolution.atc — Fork-Erkennung + Reorg
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `detect_fork(chain: &Blockchain, new_block: &BlockHeader) -> Option<Fork>` | Erkennt Fork bei neuem Block | 50 |
| `compare_chains(fork_a: &Chain, fork_b: &Chain) -> Ordering` | Vergleicht zwei Chains (Difficulty + PoH) | 100 |
| `reorganize(chain: &mut Blockchain, new_tip: Hash) -> Result<(), ReorgError>` | Führt Chain-Reorganisation durch | 500 × depth |
| `get_finality(block: Hash) -> FinalityStatus` | Prüft Finalisierungs-Status eines Blocks | 20 |

---

## 3. Konfiguration

```toml
finality_threshold: 2/3 validators | max_reorg_depth: 50 blocks | confirmation_blocks: 15
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

6+ Unit-Tests: Fork-Erkennung, Chain-Vergleich, Reorg, Finality, Edge-Cases (gleichgewichtige Forks)

### Test-Dateien

```
tests/consensus/
├── test_atc-84.atc    # Unit-Tests
└── test_atc-84_integration.atc  # Integration-Tests
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

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-84 v1.0.0*
