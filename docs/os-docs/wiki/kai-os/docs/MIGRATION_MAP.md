# 🗺️ Migration Map — A-TownChain Python → KAI-OS Substrate/Ink!

> **Version:** 1.3.3-beta | **Stand:** 2026-06-09  
> **Generiert von:** Superagent (KAI-OS Agent)

---

## Überblick

Dieses Dokument mappt alle Legacy-Python-Komponenten auf ihre Substrate/Ink!-Äquivalente und listet fehlende Migrationsschritte.

---

## 1. Kern-Komponenten

| Python-Komponente | Layer | Substrate-Äquivalent | Status | To-Do |
|-------------------|-------|---------------------|--------|-------|
| `blockchain/consensus/poh.py` | L4 | `pallet-poh` (BLAKE2b-256) | ✅ Migriert | — |
| `blockchain/consensus/hybrid_consensus.py` | L4 | `pallet-poh` + GRANDPA/BABE | 🔄 In Migration | Fork-Auflösung (#17) |
| `core/kernel.py` | L2 | `shivaos/kernel/kernel.py` | 🔄 In Migration | Kernel-Tests fehlen |
| `blockchain/wallet/ecdsa.py` | L0/S1 | Substrate sr25519/ed25519 | 🔄 In Migration | Key-Migration-Script fehlt |
| `blockchain/contracts/shivamon/` | L12 | `ShivamonNFT.ink` (ink! 5.x) | 📋 Geplant | Breeding (#11), Battle (#3) |
| `blockchain/contracts/atc8300/` | L4 | ATC-8300 Token Pallet | 📋 Geplant | Solidity (#12) |
| `backend/api/` | L7 | Substrate RPC + REST Gateway | 🔄 In Migration | Gateway-Tests (#20) |
| `shivaos/net/atcnet.py` | L5 | libp2p (Substrate built-in) | 🔄 In Migration | P2P Stack (#14–#17) |
| `shivaos/fs/atcfs.py` | L6 | IPFS + on-chain CID | 📋 Geplant | IPFS-Integration fehlt |

---

## 2. Substrate-Pallets (Sprint 2.1)

| Pallet | Zeilen | Tests | Status | Fehlende To-Dos |
|--------|--------|-------|--------|-----------------|
| `pallet-poh` | 276 | 8 | ✅ Aktiv | Coverage auf >90% erhöhen |
| `pallet-ai-registry` | 173 | 0 | ⚠️ Keine Tests | Unit-Tests schreiben |
| `pallet-agent-registry` | 182 | 0 | ⚠️ Keine Tests | Unit-Tests schreiben |
| `pallet-atc8300` | 0 | 0 | 📋 Geplant | Implementierung (#12) |
| `pallet-governance` | 0 | 0 | 📋 Geplant | DAO-Contract (#9) |
| `pallet-shivamon` | 0 | 0 | 📋 Geplant | NFT-Pallet (#11, #3) |

---

## 3. Fehlende To-Dos nach Kategorie

### 🔴 Sofort (Blocker)

- [ ] **P2P Stack komplett**: `discovery.py` + `p2p.py` + `sync_from_peer()` (Issues #14, #15, #16)
- [ ] **Fork-Auflösung**: `HybridConsensus.resolve_fork()` (Issue #17)
- [ ] **Gateway-Tests**: Unit + Integration für Port 4000 (Issue #20)
- [ ] **Key-Migration-Script**: ECDSA → sr25519/ed25519 Substrate

### 🟡 Kurzfristig (Sprint 2.x)

- [ ] **Docker Testnet**: `docker-compose.testnet.yml` 5-Node Setup (Issue #18)
- [ ] **Solidity Contracts**: ATC Token.sol + Shivamon.sol + Governance.sol (Issue #12)
- [ ] **Pallet-Tests**: Unit-Tests für `pallet-ai-registry` + `pallet-agent-registry`
- [ ] **Issue #2 schließen**: Gemini AI ist fertig, Issue noch offen
- [ ] **IPFS-Integration**: `atcfs.py` → IPFS + on-chain CID Mapping

### 🟢 Mittelfristig (Sprint 3.x)

- [ ] **ShivamonNFT.ink!**: Breeding + Battle-Logik in ink! Smart Contract
- [ ] **Cross-Chain Bridge**: Lock/Mint ATC ↔ EVM (Issue #10)
- [ ] **ATC-8300 Pallet**: Fungible Token Standard in Rust/Substrate
- [ ] **Blockchain Explorer**: Block & TX Browser im Dashboard (Issue #5)
- [ ] **Build System**: EXE + AppImage (Issue #7)

---

## 4. Neue ShivaOS-Komponenten (2026-06-08 gepusht)

| Datei | Status | Fehlende To-Dos |
|-------|--------|-----------------|
| `shivaos/kernel/kernel.py` | ✅ Gepusht | Unit-Tests fehlen |
| `shivaos/consensus/shiva_consensus.py` | ✅ Gepusht | Fork-Auflösung (#17) |
| `shivaos/net/atcnet.py` | ✅ Gepusht | libp2p-Migration, P2P-Tests |
| `shivaos/fs/atcfs.py` | ✅ Gepusht | IPFS-Anbindung fehlt |
| `shivaos/shell/__init__.py` | ✅ Gepusht | Shell-Commands implementieren |
| `shivaos/pkg/__init__.py` | ✅ Gepusht | Package-Manager-Logik |
| `atc-ui/index.html` | ✅ Gepusht | Battle-UI (#3), Explorer (#5), Nodes (#19) |

---

## 5. ATC Token Standards — Status

| Standard | Typ | Python | Solidity | Substrate Pallet | Status |
|----------|-----|--------|----------|-----------------|--------|
| ATC-001 | Genesis | ✅ | 📋 | 📋 | Nur Python |
| ATC-8300 | Fungible | ✅ | 📋 (#12) | 📋 | Nur Python |
| ATC-9000 | NFT Shivamon | ✅ | 📋 (#12) | 📋 | Nur Python |
| ATC-9900 | Governance | ✅ (a460466) | 📋 (#12) | 📋 | Python done |

---

## 6. Migrations-Fahrplan

```
Phase 1 (aktuell):    Python Prototype → läuft lokal
Phase 2 (Sprint 2.x): P2P-Netzwerk + Testnet + Gateway-Tests
Phase 3 (Sprint 3.x): Substrate/Ink! Migration + Solidity Contracts
Phase 4 (Mainnet):    Substrate Mainnet + Cross-Chain Bridge
```

---

## Links

- [A-TownChain OS Repo](https://github.com/A-TownChain-Okosystems/a-townchain-os)
- [Master To-Do Liste](../TODO.md)
- [Performance Report](../PERFORMANCE_REPORT.md)
- [Status Übersicht](../STATUS.md)

*Auto-generiert von Superagent (KAI-OS Agent) — 2026-06-09*
