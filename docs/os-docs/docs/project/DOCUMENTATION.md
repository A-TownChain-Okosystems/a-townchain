# A-TownChain — Documentation Overview

> Generated: 2026-08-06
> 102 repositories | 30 kernel modules | 367 tests

## 1. Architecture Documentation

### ShivaCore Kernel Architecture
- **Location**: `a-townchain-os-docs/docs/SHIVACORE_KERNEL_STATUS.md`
- **Content**: Kernel layer model, module status, test counts
- **Decision**: AD-008 (RESOLVED) — core/kernel.py foundation + shivaos/kernel.py process layer

### Genesis Engine v2 Architecture
- **Location**: `atc-genesis-engine/ARCHITECTURE.md`
- **Content**: Layered architecture (Editor → Engine Core → Graphics APIs)
- **Vision**: 9-stage evolution from Franchise Factory to Genesis Neural Ecosystem

### Dual-Repository Model
- **Code**: `a-townchain-os` (monorepo)
- **Docs**: `a-townchain-os-docs` (wiki)
- **Decision**: AD-089 (RESOLVED)

## 2. Standards Documentation

### ATC Standards (35 standards)
- **Location**: `a-townchain-os-docs/docs/standards/`
- **Coverage**: ATC-01 through ATC-35
- **Tiers**: 5 tiers (Core → Human-Centric)
- **Registry**: `a-townchain-os-docs/docs/STANDARDS_REGISTRY.md`

### ATS Standards (8 standards)
- **Location**: `atc-shivacore/kernel/src/ats1000.rs`
- **Coverage**: ATS-1000 through ATS-1007
- **Status**: All 8 implemented in Rust

## 3. Compliance Documentation

### BaFin Conformity Report
- **ID**: BAFIN-ATC-LIC-2026-001
- **Location**: `a-townchain-os-docs/docs/compliance/BAFIN_KONFORMITAETSBERICHT.md`
- **Size**: 16KB, 409 lines, 44 sections
- **Content**: 7-category conformity analysis, 32-item audit checklist

### Licensing Model
- **Type**: Hybrid monetized autonomous open-source
- **Enforcement**: ATVM blocks unlicensed code execution ("Code is Law")
- **License Types**: PER_CALL, SUBSCRIPTION, PERPETUAL, REVENUE_SHARE, FREEMIUM, DAO_GOVERNED
- **Location**: `a-townchain-os-docs/docs/LICENSING_OVERVIEW.md`

### Agent Policy
- **Location**: `a-townchain-os-docs/docs/AGENT_POLICY.md`
- **Mandate**: All AI agents must check STANDARDS_REGISTRY, DECISIONS_REGISTER, LICENSING_OVERVIEW
- **Reality-Check**: No task completion without API or commit confirmation

## 4. Kernel Documentation

### Module Documentation
Each of the 30 kernel modules has:
- Rust source file with inline documentation
- Test suite with descriptive test names
- Push commit message with detailed description
- Entry in SHIVACORE_KERNEL_STATUS.md

### Key Files
| File | Purpose |
|------|---------|
| `ats1000.rs` | Trait definitions (ATS-1000–1007) |
| `capability.rs` | Capability system |
| `process.rs` | Process manager |
| `scheduler.rs` | DA-HEFT scheduler |
| `ipc.rs` | IPC subsystem |
| `did.rs` | Decentralized identity |
| `remote_caps.rs` | Remote capability tickets |
| `knowledge_graph.rs` | Triple store |
| `memory_manager.rs` | Memory management |
| `atcfs.rs` | Content-addressed filesystem |
| `kernel_init.rs` | Boot sequence |
| `cross_subsystem.rs` | Integration tests |
| `atcnet.rs` | P2P protocol |
| `genesis.rs` | Genesis block config |
| `genesis_bridge.rs` | Genesis ↔ blockchain bridge |
| `gossip_bridge.rs` | P2P gossip integration |
| `security_audit.rs` | Security audit framework |

## 5. Whitepaper

- **Version**: v3.0.0
- **Location**: `atc-whitepaper`
- **Content**: Technical whitepaper covering architecture, consensus, tokenomics
- **Status**: Published

## 6. Vision Documentation

| Document | Repository | Stage |
|----------|-----------|-------|
| Franchise Factory v2.0 | `atc-genesis-engine` | 14 lifecycle modules |
| MetaFactory v3.0 | `atc-genesis-engine` | 12 infrastructure factories |
| Genesis OS v4.0 | `atc-genesis-engine` | OS-level orchestration |
| Genesis Nexus v5.0 | `atc-genesis-engine` | Network knowledge graphs |
| Genesis Neural Ecosystem v9.0 | `atc-genesis-engine` | Final vision |

## 7. Test Documentation

| Category | Count | Location |
|----------|-------|----------|
| Kernel Unit Tests | 367 | `atc-shivacore` |
| Cross-Subsystem Tests | 15 | `cross_subsystem.rs` |
| Security Audit Checks | 30+ | `security_audit.rs` |
| Attack Simulations | 5 | `security_audit.rs` |
| Multi-Node Tests | 4 | `gossip_bridge.rs` |

**Total Pass Rate: 100% (367/367)**
