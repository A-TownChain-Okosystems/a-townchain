# ATC-85 — Initial Sync

> **Standard-ID:** ATC-85 (ehemals ATC-1004)
> **Status:** ACCEPTED — Spezifikation vollständig, Implementation geplant in Sprint 2.1
> **Sprint:** 2.1 | **Issue:** #74 | **Wiki:** Kap. 31 (Issue-Registry)
> **Autor:** Aurora (MasterBrain · Base44)
> **Stand:** 05.07.2026 | Version 1.0.0
> **Kategorie:** Consensus Primitives  
> **Tier:** Tier 1 — Consensus Primitives  

---

## 1. Überblick

Initial Sync — Synchronisiert neue Nodes mit der aktuellen Chain. Batch-Download von Blöcken, Verifikation, State-Replay. Snapshot-basiert für schnelles Bootstrap.

### Design-Prinzipien

1. **Deterministic** — Alle Operationen deterministisch (Konsens-Sicherheit)
2. **Gas-Aware** — Jede Operation hat definierte Gas-Kosten
3. **ATCLang-Native** — Implementiert in ATCLang (Non-EVM)
4. **SHA-256** — Kryptografie basiert auf SHA-256 (Non-EVM-Standard)

### Module

```
modules/network/sync.atc — Block-Sync
modules/network/snapshot.atc — State-Snapshot-Download
```

---

## 2. API-Referenz

| Funktion | Beschreibung | Gas-Cost |
|----------|--------------|----------|
| `sync_from_genesis(peers: &Array<PeerId>) -> SyncResult` | Vollständige Sync ab Genesis | variable |
| `sync_from_snapshot(snapshot_hash: Hash) -> SyncResult` | Schnelle Sync via State-Snapshot | variable |
| `request_blocks(peer: PeerId, start: u64, count: u32) -> Array<Block>` | Fordert Blöcke von Peer an | 10 + 5/block |
| `verify_block_sequence(blocks: &Array<Block>) -> bool` | Verifiziert Block-Sequenz | 12 × len |
| `get_sync_status() -> SyncStatus` | Gibt aktuellen Sync-Status zurück | 10 |

---

## 3. Konfiguration

```toml
batch_size: 64 blocks | max_peers_sync: 3 | snapshot_interval: 10000 blocks | verify_signatures: true
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

6+ Unit-Tests: Full-Sync, Snapshot-Sync, Block-Verifikation, Peer-Ausfall während Sync, Edge-Cases

### Test-Dateien

```
tests/consensus/
├── test_atc-85.atc    # Unit-Tests
└── test_atc-85_integration.atc  # Integration-Tests
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

*Spezifikation: Aurora (MasterBrain · Base44) · 05.07.2026 · ATC-85 v1.0.0*
