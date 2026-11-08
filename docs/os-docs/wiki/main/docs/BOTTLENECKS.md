# Bottlenecks & Performance-Optimierungen

## Identifizierte Bottlenecks (Stand: 10.06.2026)

### Behoben ✅

| # | Bottleneck | Problem | Fix | Verbesserung |
|---|-----------|---------|-----|-------------|
| B-001 | VM Label-Lookup | O(n) pro Sprung | `_label_cache` dict | O(n) → O(1) |
| B-002 | Orchestrator Sleep | Unbegrenzt | Cap 5s | stabile Retry-Zeit |
| B-003 | P2P Broadcast | Kein Limit | 100/60s | Netzwerk-Schutz |
| B-004 | Compiler Source | Kein Limit | MAX 1MB | OOM-Schutz |
| B-005 | BaseContract | Kein Reentrancy | `_locked` Flag | Angriffs-Schutz |
| B-006 | Block-Propagation | Keine Priorisierung | Validator-zuerst | -15% Latenz |

### Offen 🔄 (v3.0.0)

| # | Bottleneck | Problem | Geplante Lösung |
|---|-----------|---------|----------------|
| B-007 | VM Interpreter | Python-Overhead | WASM-Kompilierung [#16] |
| B-008 | P2P Serialization | JSON statt Binär | MessagePack |
| B-009 | Consensus PoW | CPU-intensiv | ASIC-Resistenz SHA3-Tuning |
| B-010 | ATCFS Memory | In-Memory FS | Disk-Backed-Store (v3.0) |
| B-011 | Gateway Session | Stateless | Connection-Pool |
| B-012 | Marketplace Query | Linear-Scan | Index auf price/token_id |

## Performance-Metriken (Testnet)

| Metrik | Wert | Ziel v3.0 |
|--------|------|----------|
| TPS (Testnet) | ~50 TX/s | 1.000 TX/s |
| Block-Zeit | ~3s | ~1s |
| P2P-Latenz | ~80ms | ~20ms |
| VM-Geschwindigkeit | ~10.000 Opcodes/s | ~1.000.000/s |
| Memory pro Node | ~200 MB | ~100 MB |
| Sync-Zeit (frischer Node) | ~2 min | ~30s |

## Optimierungs-Roadmap

```
v3.0.0:
  B-007 → ATCLang → WASM (×100 Speedup VM)
  B-008 → MessagePack P2P (+30% Durchsatz)
  B-012 → Marketplace-Index (×50 Query-Speed)

v4.0.0:
  B-009 → Consensus-Tuning (2× TPS)
  B-010 → Persistent ATCFS (unbegrenzte Datenmenge)
  B-011 → Gateway Connection-Pool (+20% Throughput)
```
