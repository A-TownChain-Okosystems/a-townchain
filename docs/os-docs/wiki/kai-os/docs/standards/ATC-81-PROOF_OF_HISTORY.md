# ATC-81 — Proof of History (PoH)

> **Standard-ID:** ATC-81 (ehemals ATC-81)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.1
> **Sprint:** 2.1 | **Issue:** #74 | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Consensus Primitives  
> **Tier:** Tier 1 — Consensus Primitives  

---

## 1. Überblick

Proof of History — Verifiable Delay Function (VDF) basierend auf SHA-256-Hash-Sequenzen. Erstellt eine kryptografische Zeitachse, die es Nodes erlaubt, die zeitliche Reihenfolge von Ereignissen zu verifizieren, ohne auf externe Zeitquellen angewiesen zu sein.

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/consensus/poh.atc — VDF-Hash-Sequenz-Generator
modules/consensus/poh_verifier.atc — PoH-Verifikation für eingehende Blöcke
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `poh_init(seed: Hash) -> PoHState` | Initialisiert PoH mit Genesis-Seed | 500 |
| `poh_advance(state: &mut PoHState, n: u32) -> Hash` | Führt n SHA-256-Iterationen aus | 12 × n |
| `poh_verify(sequence: &PoHSequence) -> bool` | Verifiziert eine PoH-Hash-Sequenz | 12 × len |
| `poh_current(state: &PoHState) -> Hash` | Gibt aktuellen PoH-Hash zurück | 10 |
| `poh_register_event(state: &mut PoHState, event: Hash) -> void` | Registriert Event im PoH-Stream | 20 |

---

## 3. Konfiguration

```toml
iterations_per_slot: 1000 | slot_duration_ms: 400 | seed: genesis_hash
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

8+ Unit-Tests: VDF-Korrektheit, Sequenz-Verifikation, Event-Registrierung, Edge-Cases (leere Sequenz, invalider Seed)

### Test-Dateien

```
tests/consensus/
├── test_atc-81.atc    # Unit-Tests
└── test_atc-81_integration.atc  # Integration-Tests
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

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-81 v1.0.0*
