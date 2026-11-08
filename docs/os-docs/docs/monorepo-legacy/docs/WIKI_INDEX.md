# A-TownChain OS — Wiki Kapitel-Index (v2.0)

> **Version:** 2.0 | **Zuletzt aktualisiert:** 2026-08-04
> **Status:** PUBLISHED | **Quelle:** a-townchain-os-docs + a-townchain-os/docs
> **Ersetzt:** Wiki Index v1.0 (Punkt 2: Kapitel-Nummerierung hinzugefügt)

---

## 📚 **Kapitel-Übersicht (1-40)**

### **Kap. 1-10: Kern-Architektur**

| Kap. | Titel | Datei | Scope | Status |
|------|-------|-------|-------|--------|
| 1 | A-TownChain OS — Architektur-Überblick | [ECOSYSTEM.md](../../../ECOSYSTEM.md) | System-Design, 13 Layer | ✅ PUBLISHED |
| 2 | Blockchain-Fundamentals (Konsens, PoH, PoS, PoW) | [BLOCKCHAIN_FUNDAMENTALS.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/) | Consensus, ATC-81/82/83 | ✅ PUBLISHED |
| 3 | Smart Contracts & ATCLang (Sprache, Compiler, VM) | [ATCLANG_SPEC.md](../../../atclang/ATCLANG_SPEC.md) | Contract-Sprache, ATC-05 | ✅ PUBLISHED |
| 4 | Blockchain Layer (Gas, DEX, Bridge, Governance) | BLOCKCHAIN_LAYER.md | Transaktionen, ATC-87/88 | 🟡 DRAFT |
| 5 | ShivaOS Kernel (Syscalls, Memory, Scheduling) | [KERNEL_FROM_SCRATCH_PLAN.md](../root/KERNEL_FROM_SCRATCH_PLAN.md) | Bare-Metal Rust, no_std | ✅ PUBLISHED |
| 6 | ATCFS — Dezentrales Filesystem | ATCFS_SPEC.md | File Storage, ATC-06 | 🟡 DRAFT |
| 7 | Node-Konfiguration & Deployment | NODE_CONFIG.md | Setup, Testnet, Mainnet | ✅ PUBLISHED |
| 8 | API Gateway (Middleware, Auth, Routing) | GATEWAY_SPEC.md | REST API, Port :4000 | ✅ PUBLISHED |
| 9 | Cross-Chain Bridge (ATC ↔ ETH/POLYGON/BSC) | BRIDGE_SPEC.md | Interoperability, ATC-91 | 🟡 DRAFT |
| 10 | ShivaOS Treiber & Hardware-Integration | SHIVAOS_DRIVERS.md | Device Drivers, I/O | 🔴 TODO |

---

### **Kap. 11-20: Datenebenen & Services**

| Kap. | Titel | Datei | Scope | Status |
|------|-------|-------|-------|--------|
| 11 | Wallet-System (MultiSig, HD Keys, Recovery) | WALLET_SPEC.md | Accounts, ATC-20 | ✅ PUBLISHED |
| 12 | Blockchain-Persistence (SQLite, RocksDB, State Merkle Tree) | PERSISTENCE_SPEC.md | DB Layer, ATC-12 | 🟡 DRAFT |
| 13 | P2P Network & Discovery (DHT, Gossip) | NETWORK_SPEC.md | Networking, ATC-02 | ✅ PUBLISHED |
| 14 | Tests & CI/CD Pipeline | CI_CD_SPEC.md | GitHub Actions, K6/K7/K8 | ✅ PUBLISHED |
| 15 | Monitoring & Observability (Prometheus, Grafana) | MONITORING_SPEC.md | Dashboards, Health Checks | ✅ PUBLISHED |
| 16 | Smart Contracts — Token (ERC-20 Style) | TOKEN_CONTRACTS.md | ATC-89, Standard Token | ✅ PUBLISHED |
| 17 | Smart Contracts — DAO & Governance | DAO_GOVERNANCE.md | Multi-Sig, Voting, ATC-17 | ✅ PUBLISHED |
| 18 | Smart Contracts — Multi-Sig Vaults | MULTISIG_VAULTS.md | Escrow, ATC-18 | ✅ PUBLISHED |
| 19 | Smart Contracts — NFT & Gaming (Shivamon) | NFT_SHIVAMON.md | Gen 2 Breeding, ATC-90 | ✅ PUBLISHED |
| 20 | DEX & Marketplace (Auctions, Trading) | DEX_MARKETPLACE.md | Order Book, AMM, ATC-88 | ✅ PUBLISHED |

---

### **Kap. 21-30: Sicherheit, Kryptographie, Compliance**

| Kap. | Titel | Datei | Scope | Status |
|------|-------|-------|-------|--------|
| 21 | Kryptographie-Grundlagen (SHA-256, ECDSA, EdDSA) | CRYPTO_SPEC.md | Signing, ATC-86 | ✅ PUBLISHED |
| 22 | Quantum-Resistant Signatures (QRC Layer) | QUANTUM_RESISTANT.md | Post-Quantum Crypto, ATC-05/46 | 🟡 DRAFT |
| 23 | Security-Audit-Gate & Penetration Testing | SECURITY_AUDIT.md | Vulnerability Scanning, ATC-AUDIT | 🟡 DRAFT |
| 24 | Compliance & Regulatory (BaFin, KYC/AML) | COMPLIANCE_HANDBUCH.md | Legal Framework, ATC-LIC | ✅ PUBLISHED |
| 25 | Lizenzmodell — ATC-LIC & ATC-LIC | LICENSING_OVERVIEW.md | Smart Contract Licenses | ✅ PUBLISHED |
| 26 | Federated Learning & AI Integration | FEDERATED_LEARNING.md | On-Chain AI Coordination | 🟡 DRAFT |
| 27 | Disaster Recovery & Backup Strategy | DISASTER_RECOVERY.md | State Snapshots, Restore | 🟡 DRAFT |
| 28 | Performance Tuning & Optimization | PERFORMANCE_TUNING.md | Benchmarks, Scaling | 🟡 DRAFT |
| 29 | Deprecation & Sunset Policy | DEPRECATION_POLICY.md | Versioning, EOL | 🟡 DRAFT |
| 30 | Release Management & Versioning | RELEASE_MANAGEMENT.md | Semantic Versioning, Cadence | ✅ PUBLISHED |

---

### **Kap. 31-40: Standards, Roadmap, Architektur-Entscheidungen**

| Kap. | Titel | Datei | Scope | Status |
|------|-------|-------|-------|--------|
| 31 | **Issue-Registry & Tracking** | [Issue #85-#92](https://github.com/A-TownChain-Okosystems/a-townchain-os/issues) | K1-K8 Consolidation, Sprints | ✅ LIVE |
| 32 | **DECISIONS_REGISTER** | [DECISIONS_REGISTER.md](../../DECISIONS_REGISTER.md) | AD-001 to AD-010 (Architektur) | ✅ PUBLISHED |
| 33 | **STANDARDS_REGISTRY** | [docs/standards/STANDARDS_REGISTRY.md](../../standards/STANDARDS_REGISTRY.md) | ATC-01 to ATC-97 (37+ Standards) | ✅ PUBLISHED |
| 34 | **KONSOLIDIERUNGS_ROADMAP** | [KONSOLIDIERUNGS_ROADMAP.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/KONSOLIDIERUNGS_ROADMAP.md) | K1-K8, Sprints 1.0-4.0 | ✅ PUBLISHED |
| 35 | **AGENT_POLICY & Sync-Protokoll** | [docs/AGENT_POLICY.md](./AGENT_POLICY.md) | Agent Rules, Reality-Check, Release-Blocker | ✅ PUBLISHED |
| 36 | **AGENT_COORDINATION** | [AGENT_COORDINATION.md](./AGENT_COORDINATION.md) | Live Agent Status, Aurora Handoff | ✅ PUBLISHED |
| 37 | **NAMING_CONVENTIONS** | [NAMING_CONVENTIONS.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/NAMING_CONVENTIONS.md) | Identifier Rules (19 Konzepte) | ✅ PUBLISHED |
| 38 | **CHANGELOG & Release Notes** | [CHANGELOG.md](../../../CHANGELOG.md) | v1.0.0 Release, v2.0 Roadmap | ✅ PUBLISHED |
| 39 | **DEVELOPER_ONBOARDING** | [docs/DEVELOPER_ONBOARDING.md](./DEVELOPER_ONBOARDING.md) | Setup, First Steps, Contributing | ✅ PUBLISHED |
| 40 | **PROJECT_VISION & KAI-OS v2.0** | [README.md](standards/README.md) | High-Level Vision, Features, Status | ✅ PUBLISHED |

---

## 🔗 **Cross-Repository Links**

### **Primär-Repos**
| Repo | Typ | Inhalt | Link |
|------|-----|--------|------|
| **a-townchain-os** | 💾 Code + Runtime | Monorepo: Kernel, Blockchain, Contracts, UI, Gateway | https://github.com/A-TownChain-Okosystems/a-townchain-os |
| **a-townchain-os-docs** | 📖 Dokumentation | Wiki, Standards, Roadmap, Compliance | https://github.com/A-TownChain-Okosystems/a-townchain-os-docs |

### **Spezial-Repos (Archiviert oder Aktiv)**
| Repo | Status | Scope |
|------|--------|-------|
| atc-shivacore | ✅ Aktiv | Rust Bare-Metal Kernel (no_std) |
| atc-blockchain | 🟡 Archiviert | → Migriert in a-townchain-os/src/blockchain |
| atc-frontend | 🟡 Archiviert | → Migriert in a-townchain-os/frontend |
| atc-gateway | 🟡 Archiviert | → Migriert in a-townchain-os/src/gateway |
| atc-aistudio | ✅ Aktiv | Standalone AI Studio Web App |
| atc-atclang | ✅ Aktiv | ATCLang Compiler (Standalone) |

---

## 📋 **Wie man diese Kapitel nutzt**

### **Für Architektur-Fragen:**
- Starten mit **Kap. 1** (Überblick)
- Dann **Kap. 31-36** (Decisions + Standards)
- Dann spezifisches Kapitel (z.B. Kap. 5 für Kernel)

### **Für neue Features:**
1. **Kap. 34** — Finde den relevanten Sprint (K1-K8)
2. **Kap. 31** — Finde das entsprechende GitHub Issue (#85-#92)
3. **Kap. 33** — Überprüfe relevante Standards (ATC-XX)
4. **Kap. 35** — Lies AGENT_POLICY für Compliance

### **Für Development Setup:**
- **Kap. 7** — Node-Konfiguration
- **Kap. 39** — Developer Onboarding
- **Kap. 14** — CI/CD Setup

---

## 🔄 **Synchronisierungs-Status**

```
a-townchain-os ↔ a-townchain-os-docs
├─ Automat: MANUAL (via scripts/sync-docs.sh)
├─ Letzte Sync: 2026-08-04 (Post-Gap-Analysis)
├─ Next Sync: Nach jeder Major-Release (K-Phase)
└─ Checker: AGENT_POLICY.md → Schritt 2 (GitHub Wiki Sync)
```

**Warnung:** Diese beiden Repos müssen manuell synchronisiert werden. Nutze `./scripts/sync-docs.sh --commit --push` nach Änderungen in a-townchain-os-docs.

---

## ✅ **Validierungs-Checkliste**

Vor dem Release einer neuen Kap.:

- [ ] Datei in a-townchain-os/docs/ vorhanden
- [ ] Datei in a-townchain-os-docs/ vorhanden
- [ ] Kapitel-Nummer in dieser WIKI_INDEX.md aktualisiert
- [ ] Status: PUBLISHED (oder DRAFT/TODO wenn nicht fertig)
- [ ] Verlinkungen zu anderen Kapiteln + Standards geprüft
- [ ] DECISIONS_REGISTER.md aktualisiert (falls Entscheidung nötig)
- [ ] AGENT_POLICY.md Code ↔ Doku Abgleich bestanden

---

**Zuletzt aktualisiert:** 2026-08-04 von Copilot (@copilot)  
**Status:** ✅ PUBLISHED | **Vorherige Version:** [Wiki Index v1.0](./WIKI_INDEX.md)
