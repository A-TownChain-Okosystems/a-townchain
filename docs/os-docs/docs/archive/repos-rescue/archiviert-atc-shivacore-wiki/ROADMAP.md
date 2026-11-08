# A-TownChain — Roadmap to Mainnet

> Target: September 15, 2026 (40 Tage)
> Last Updated: 2026-08-06
> Kernel: K29 ✅ | 30 Module | 367/367 Tests

---

## Timeline Overview

```
Aug 3 ─── K3-K29 COMPLETED ────────── Aug 6 ── TODAY
  │                                        │
  │  17 Sprints | 30 Module | 367 Tests     │
  │  Genesis + Bridge + Gossip + Audit ✅   │
  │                                        ↓
 Aug 10 ── K30 Validator Setup ──────────── Issue #70
 Aug 20 ── K31 Genesis Deployment ───────── Issue #71
 Sep  1 ── K32 Pre-Launch Verification ───── (internal)
 Sep  8 ── K33 External Audit ───────────── Issue #69
 Sep 15 ── MAINNET LAUNCH ───────────────── 🚀
```

---

## Phase 1: Kernel Development (K3–K29) ✅ ABGESCHLOSSEN

**Zeitraum**: 3.–4. August 2026
**Ergebnis**: 30 Module, 367/367 Tests, 100% Pass Rate

### Sprint-Historie

| Sprint | Modul | Zeilen | Tests | Issue | Datum |
|--------|-------|--------|-------|-------|------|
| K3 | capability.rs | ~300 | 8 | — | Aug 3 |
| K3b | process.rs | ~400 | 10 | — | Aug 3 |
| K4 | scheduler.rs | ~500 | 10 | — | Aug 3 |
| K5 | ipc.rs | ~700 | 12 | — | Aug 3 |
| K6 | did.rs + remote_caps.rs | ~900 | 21 | — | Aug 3 |
| K6b | Ed25519 (did.rs) | ~100 | 15 | — | Aug 3 |
| K7 | knowledge_graph.rs | ~600 | 18 | — | Aug 3 |
| K8 | memory_manager.rs + atcfs.rs | ~800 | 34 | — | Aug 3 |
| K21 | Heap-Bridge (mem_mgr) | ~200 | 28 | — | Aug 4 |
| K22 | kernel_init.rs | ~350 | 11 | — | Aug 4 |
| K23 | cross_subsystem.rs | ~400 | 15 | — | Aug 4 |
| K24 | atcnet.rs | ~900 | 32 | — | Aug 4 |
| K25 | Type-Mismatch Fix | ~100 | — | atc-shivacore#1 ✅ | Aug 4 |
| K26 | genesis.rs | ~800 | 38 | #71 (open) | Aug 4 |
| K27 | genesis_bridge.rs | ~700 | 40 | — | Aug 4 |
| K28 | gossip_bridge.rs | ~900 | 45 | atc-shivacore#2 ✅ | Aug 4 |
| K29 | security_audit.rs | ~800 | 34 | atc-shivacore#3 ✅, #69 | Aug 4 |

### Modul-Abhängigkeiten

```
K3 (Caps) → K3b (Process) → K5 (IPC) → K6 (DID) → K6b (Ed25519)
    ↓            ↓              ↓
K4 (Sched)   K8 (MemMgr+ATCFS)   K7 (KnowledgeGraph)
    ↓            ↓
K21 (Heap-Bridge) → K22 (KernelInit) → K23 (CrossSub)
    ↓
K24 (ATCNet) → K25 (Types) → K26 (Genesis)
    ↓
K27 (GenesisBridge) → K28 (GossipBridge) → K29 (SecurityAudit)
```

### Audit-Ergebnisse (K29)

| Kategorie | Checks | Status |
|-----------|--------|--------|
| Chain-Integrity | 5 | ✅ All PASS |
| Genesis Security | 5 | ✅ All PASS |
| Validator Security | 5 | ✅ All PASS |
| PoH Integrity | 4 | ✅ All PASS |
| Capability Enforcement | 5 | ✅ All PASS |
| Network Security | 4 | ✅ All PASS |
| Block Validation | 4 | ✅ All PASS |

**5 Attack-Simulationen**: Chain Forgery, Genesis Replay, Height Skip, Orphan Block, Unsigned Genesis — alle blocked ✅

---

## Phase 2: Mainnet Preparation (AKTUELL)

### K30: Validator Node Setup
| Metrik | Wert |
|--------|------|
| **Issue** | #70 (Sprint 4.0) |
| **Ziel** | 10. August 2026 |
| **Geschätzte Tests** | ~40 |
| **Abhängigkeit** | K28 (GossipBridge) ✅, K27 (GenesisBridge) ✅ |

**Deliverables**:
- `validator_node.rs` — Validator-Konfigurations-Generator
- Multi-Node Docker-Compose (10+ Nodes)
- P2P-Konnektivitäts-Test (ATCNet Mesh)
- Proposer-Rotation Verifikation
- BFT-Quorum Test (66.7%+)
- Validator-Key-Backup-Prozedur

**Aufgaben**:
1. [ ] ValidatorConfig struct (DID, pubkey, stake, address, commission)
2. [ ] generate_validator_set() — 10+ Validatoren aus GenesisConfig
3. [ ] Multi-Node Test Harness (Bridge + Gossip + Sync)
4. [ ] Proposer-Rotation Test (round-robin stake-weighted)
5. [ ] BFT-Quorum Test (2/3+ Validatoren signieren)
6. [ ] Byzantine-Fault Test (1/3 Validatoren offline)
7. [ ] Network-Partition Recovery Test
8. [ ] Docker-Compose für 10 Nodes

### K31: Genesis Block Deployment
| Metrik | Wert |
|--------|------|
| **Issue** | #71 (Sprint 4.0) |
| **Ziel** | 20. August 2026 |
| **Geschätzte Tests** | ~20 |
| **Abhängigkeit** | K30 ⬜, K26 ✅ |

**Deliverables**:
- Finale GenesisConfig (Validatoren, Allokationen, Chain-Params)
- Genesis-Block Generierung + Signierung
- Genesis-Hash-Verteilung
- Chain-ID 9000 Aktivierung
- Genesis-Verification auf allen Nodes

**Aufgaben**:
1. [ ] Finale Validator-Liste (10+ DIDs, Pubkeys, Stakes)
2. [ ] Finale Token-Allokationen (ATC-8300)
3. [ ] Genesis-Block Signierung (Ed25519)
4. [ ] Genesis-Hash Distribution (ATCNet)
5. [ ] Chain-ID 9000 Verifikation (alle Nodes)
6. [ ] State-Root Verifikation
7. [ ] Genesis-Export (JSON + Binary)

### K32: Pre-Launch Verification
| Metrik | Wert |
|--------|------|
| **Ziel** | 1. September 2026 |
| **Abhängigkeit** | K30 ⬜, K31 ⬜ |

**Aufgaben**:
1. [ ] Vollständiger Test-Run (367 cargo + pytest)
2. [ ] Security-Audit Re-Run (7 Kategorien → PASS)
3. [ ] Attack-Simulationen (5 → alle blocked)
4. [ ] Multi-Node Convergence (3+ Nodes, 10+ Blocks)
5. [ ] Performance-Benchmark (TPS, Latenz, Block-Zeit)
6. [ ] Docker Multi-Node Up
7. [ ] Prometheus/Grafana Monitoring

### K33: External Security Audit
| Metrik | Wert |
|--------|------|
| **Issue** | #69 (Sprint 3.3) |
| **Ziel** | 8. September 2026 |
| **Abhängigkeit** | K29 ✅ |

**Aufgaben**:
1. [ ] Audit-Report Export (Markdown + PDF)
2. [ ] Code-Review durch Dritte
3. [ ] Remediation bei Fundings
4. [ ] Sign-off dokumentieren

---

## Phase 3: Mainnet Launch

| Datum | Event | Issue |
|------|-------|-------|
| Sep 15 | Genesis Block Live | #71 |
| Sep 15 | Validator Network Online | #70 |
| Sep 15 | Public API Live (port 4000) | — |
| Sep 15 | Block Explorer Live | — |
| Sep 15 | Status Page Live | — |

### Launch Sequence

1. Genesis-Block auf allen Validator-Nodes deployen
2. ATCNet P2P Verbindung aufbauen (alle Validatoren)
3. Chain-ID 9000 aktivieren
4. Erster Block: Proposer #1 → propose → gossip → verify
5. Chain-Höhe advancing (Block-Zeit ~400ms)
6. Monitoring-Dashboards aktivieren
7. Public API (port 4000) freischalten
8. Block Explorer online
9. Status-Page live

---

## Phase 4: Post-Launch (Q4 2026)

| Sprint | Feature | Standard | Ziel |
|--------|---------|----------|------|
| K34 | Cross-Chain Bridge | ATC-09 | Okt 2026 |
| K35 | ZKP Privacy | ATC-08 | Nov 2026 |
| K36 | DEX Integration | ATC-06 | Dez 2026 |
| K37 | Mobile App | — | Q1 2027 |
| K38 | Governance DAO | ATC-03 | Q1 2027 |
| K39 | AIP-001 Protocol | #80 | Q1 2027 |

---

## Konsolidierungs-Roadmap (Parallel)

| Phase | Issue | Subtasks | Status | Ziel |
|-------|-------|----------|--------|------|
| K2 | Monorepo Structure | 7/8 ✅ | Abgeschlossen | — |
| K3 | Python Backend | 7/12 | In Progress | Sep 2026 |
| K4 | Frontend | 0/10 | Not Started | Sep 2026 |
| K5-K6 | Pipeline/CI | 0/8 | Not Started | Sep 2026 |
| K8 | Release v1.0 | 0/5 | Not Started | Sep 15 |

---

## Open Issues (Stand 06.08.2026)

| Issue | Repo | Sprint | Title | Priority | Status |
|-------|------|--------|-------|----------|--------|
| #69 | a-townchain-os | 3.3 | Security-Audit (extern) | High | Open |
| #70 | a-townchain-os | 4.0 | Validator-Nodes (10+) | Medium | Open |
| #71 | a-townchain-os | 4.0 | Genesis Block (9000) | Medium | Open |
| #80 | a-townchain-os | 3.0 | AIP-001 Protocol | High | Open |
| #93 | a-townchain-os | — | Sync-Integration Warnings | Bug | Open |
| #1 | atc-shivacore | — | Type-Mismatch Fix | — | Closed ✅ |
| #2 | atc-shivacore | — | P2P Gossip Integration | — | Closed ✅ |
| #3 | atc-shivacore | — | Security Audit | — | Closed ✅ |
