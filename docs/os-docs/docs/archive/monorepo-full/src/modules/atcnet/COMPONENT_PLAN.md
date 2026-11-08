# 📋 Komponenten-Plan — atcnet

> **Erstellt:** 2026-08-06 | **Agent:** Aurora (MasterBrain · Base44)

## Übersicht

**Repo:** `atcnet`
**Name:** ATCNet — P2P-Netzwerk
**Beschreibung:** P2P-Netzwerk-Stack. Discovery, Gossip, NAT-Traversal, Bootstrap, P2P-Node, P2P-Propagation, Sharding, Time-Sync, Quantum-Resistant, Global-Time, Liquid-State, Core-Node.
**Layer:** L5 — Networking
**Sprint:** 2.2
**ATC-Standards:** ATC-01
**Komponenten:** 18

---

## Komponenten-Liste

| # | Datei | Zeilen | Typ | Beschreibung |
|---|-------|--------|-----|-------------|
| 1 | `atcnet.atc` | 135 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 2 | `atcnet.py` | 487 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 3 | `bootstrap_client.atc` | 134 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 4 | `bootstrap_client.py` | 97 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 5 | `discovery.atc` | 138 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 6 | `discovery.py` | 314 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 7 | `gossip.atc` | 171 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 8 | `nat_traversal.atc` | 109 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 9 | `network/atc-02_liquid_state_migration_failover.atc` | 58 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 10 | `network/atc-05_quantumresistant_signatures.atc` | 58 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 11 | `network/atc-10_global_time_sync_oracles.atc` | 58 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 12 | `network/core_node_atc01.atc` | 164 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 13 | `network/sharding_atc07.atc` | 215 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 14 | `node.py` | 100 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 15 | `p2p_node.atc` | 159 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 16 | `p2p_propagation.atc` | 215 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 17 | `p2p_propagation.py` | 381 | .py | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |
| 18 | `service_discovery.atc` | 168 | .atc | Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownCh... |

---

## Detaillierte Komponenten

### 1. `atcnet.atc`

**Datei:** `atcnet.atc`
**Zeilen:** 135
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct PeerInfo, struct NetMessage, init, start, stop, connect, disconnect, broadcast (+5 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 2. `atcnet.py`

**Datei:** `atcnet.py`
**Zeilen:** 487
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __post_init__, encode, decode, distance, is_alive, generate_node_id, __init__, _bucket_index (+27 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 3. `bootstrap_client.atc`

**Datei:** `bootstrap_client.atc`
**Zeilen:** 134
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct Peer, init, announce, ping, get_peers

**Status:** 🟢 IMPLEMENTIERT

---

### 4. `bootstrap_client.py`

**Datei:** `bootstrap_client.py`
**Zeilen:** 97
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, announce, ping, get_peers

**Status:** 🟢 IMPLEMENTIERT

---

### 5. `discovery.atc`

**Datei:** `discovery.atc`
**Zeilen:** 138
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct PeerEntry, init, announce, get_peer_list, ping, pong, add_bootstrap_node, cleanup_expired_peers (+4 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 6. `discovery.py`

**Datei:** `discovery.py`
**Zeilen:** 314
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** to_dict, __init__, start, stop, announce, listen, _handle, _handle_announce (+11 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 7. `gossip.atc`

**Datei:** `gossip.atc`
**Zeilen:** 171
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct GossipMessage, init, register_default_handlers, compute_msg_id, is_seen, mark_seen, receive, relay (+7 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 8. `nat_traversal.atc`

**Datei:** `nat_traversal.atc`
**Zeilen:** 109
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct STUNResult, init, discover, stun_request, hole_punch, get_external_endpoint, get_nat_type, add_stun_server (+1 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 9. `network/atc-02_liquid_state_migration_failover.atc`

**Datei:** `network/atc-02_liquid_state_migration_failover.atc`
**Zeilen:** 58
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct LiquidStateMigrationFailoverMessage, struct LiquidStateMigrationFailoverState, init, on_message, verify, get_state

**Status:** 🟢 IMPLEMENTIERT

---

### 10. `network/atc-05_quantumresistant_signatures.atc`

**Datei:** `network/atc-05_quantumresistant_signatures.atc`
**Zeilen:** 58
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct QuantumresistantSignaturesMessage, struct QuantumresistantSignaturesState, init, on_message, verify, get_state

**Status:** 🟢 IMPLEMENTIERT

---

### 11. `network/atc-10_global_time_sync_oracles.atc`

**Datei:** `network/atc-10_global_time_sync_oracles.atc`
**Zeilen:** 58
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct GlobalTimeSyncOraclesMessage, struct GlobalTimeSyncOraclesState, init, on_message, verify, get_state

**Status:** 🟢 IMPLEMENTIERT

---

### 12. `network/core_node_atc01.atc`

**Datei:** `network/core_node_atc01.atc`
**Zeilen:** 164
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct NodeConfig, struct PeerInfo, struct NodeState, init_node, add_peer, remove_peer, handshake, announce_block (+5 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 13. `network/sharding_atc07.atc`

**Datei:** `network/sharding_atc07.atc`
**Zeilen:** 215
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct ShardConfig, struct Shard, struct CrossShardTx, struct ShardingState, get_shard_for_address, init_shards, route_tx, lock_cross_shard (+7 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 14. `node.py`

**Datei:** `node.py`
**Zeilen:** 100
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** __init__, connect_peer, broadcast, receive, _validate_and_add_block, get_info, __init__, add_node (+3 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 15. `p2p_node.atc`

**Datei:** `p2p_node.atc`
**Zeilen:** 159
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct Peer, init, start, stop, connect_peer, disconnect_peer, send_message, receive_message (+5 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 16. `p2p_propagation.atc`

**Datei:** `p2p_propagation.atc`
**Zeilen:** 215
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct PeerMessage, init, add_peer, remove_peer, broadcast, receive, send_to_peer, cleanup_seen_messages (+4 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 17. `p2p_propagation.py`

**Datei:** `p2p_propagation.py`
**Zeilen:** 381
**Typ:** .py
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** to_bytes, from_bytes, __init__, register_handler, register_default_handlers, start, stop, _listener_loop (+13 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

### 18. `service_discovery.atc`

**Datei:** `service_discovery.atc`
**Zeilen:** 168
**Typ:** .atc
**Beschreibung:** Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
**Funktionen/Structs:** struct ServiceEndpoint, init, register_defaults, register, deregister, check_health, run_health_cycle, get_healthy (+4 weitere)

**Status:** 🟢 IMPLEMENTIERT

---

## Test-Strategie

1. Parse-Test: Jede .atc Datei muss mit ATCLang v0.3 Parser parsen
2. Unit-Tests: Mindestens 3 Tests pro Komponente
3. Integration-Test: Komponenten interagieren korrekt
4. Coverage-Ziel: >80%

## Dokumentations-Requirements

- ARCHITECTURE.md: Architektur-Baum + Komponenten-Übersicht ✅
- COMPONENT_PLAN.md: Dieser Plan ✅
- FILE_REGISTER.md: Datei-Liste ✅
- STATUS.md: Aktueller Status ✅
- ROADMAP.md: Sprint-Zuordnung ✅
- CHANGELOG.md: Änderungs-Historie ✅

---
*Auto-generiert 2026-08-06 · Aurora (MasterBrain · Base44)*
