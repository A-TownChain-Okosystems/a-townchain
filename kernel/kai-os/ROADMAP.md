---
document_id: ATC-DOC-RDM-001
title: Repository Roadmap
version: 1.0.0
status: active
owner: A-TownChain-Okosystems
created: 2026-09-07
updated: 2026-09-11
standard: ATC-STD-MD-001
---

# Roadmap — a-townchain-os

## Meilensteine (Lauffähigkeits-Roadmap M1–M8, AD-027)

- **M1:** Sprachanpassungen & ATCLang (G1 ✅ → G2 offen, AD-019 Phase 1 abgeschlossen)
- **M2:** Kernel Boot & Core Integration (atc-shivacore 674/674 Tests ✅)
- **M3:** KI-Layer Integration (aurora-ai / AAS-001..025)
- **M4:** Blockchain Infrastructure (2 Nodes, Chain-ID 658467, Smart Contract Execution auf ATVM)
- **M5:** OS-Schicht & Kernel-Services
- **M6:** 13 Blockchain Services Integration
- **M7:** Genesis Engine & Chronicles Integration (Spiele/NFT auf Chain)
- **M8:** Full Monorepo Release Candidate & Audit Finalisierung

## KAI-OS Core Runtime Track (S01–S26, Owner-Direktive 11.09.2026) — ABGESCHLOSSEN 12.09.2026

Implementierungsreihenfolge nach Systemkritikalität — Spezifikation: [docs/architecture/KAI-CORE-RUNTIME-001.md](docs/architecture/KAI-CORE-RUNTIME-001.md)
**Umgesetzt: 10 Crates, 80/80 Tests GRÜN, Issues #102–#111, GATE-Review #109**

| Sprint | Spezifikation | Implementiert | Status |
|---|---|---|---|
| S01–S03 | Node Daemon + Supervisor + Lifecycle | kai-os-node (14 Tests, #102) | ✅ wie spezifiziert |
| S04–S06 | Resource Manager + Sandbox | kai-os-runtime (10 Tests, #103) | ✅ wie spezifiziert |
| S07–S09 | P2P Transport + Discovery + Peer Security | kai-os-network (8 Tests, #104) | ✅ wie spezifiziert |
| S10–S12 | State Sync + Snapshots + Verification | kai-os-state (9 Tests, #105) | ✅ wie spezifiziert |
| S13–S15 | AI Agent Runtime + IPC | kai-os-ai (8 Tests, #106) + Audit-Pipeline (audit.rs) | ✅ + Audit vorgezogen |
| S16–S18 | Keyring + Capability Security | kai-os-keyring (7 Tests, #107) | ✅ wie spezifiziert |
| S19–S20 | Immutable Audit / Event System | kai-os-health: Watchdog + Health-Gossip (9 Tests, #108); Audit bereits in S13–S15 umgesetzt | ⚠️ abweichend belegt |
| S21–S22 | Model Registry + Verified Cache | kai-os-ai: model_registry.rs + verified_cache.rs (6+1 Tests, #112) | ✅ **12.09. nachgeholt** |
| S23–S24 | kai-os CLI + Control API | kai-os-cli (4 Tests, #110) | ✅ |
| S25 | ATC Module/Package Manager | kai-os-pkg (7 Tests, #110) | ✅ |
| S26 | Production Readiness Gate | GATE-KAI-001-Review #109 (5 PASS / 4 PARTIAL / 3 OPEN) + kai-os-integration Boot-Pipeline (4 Tests, #111) | ⚠️ Review statt Freigabe |

> Grundsatz: **Policy → Sandbox → Capability → Execution** · KI erkennt → Policy entscheidet → System führt aus.
> **Offen (GATE-Iteration 2):** ~~Model Registry + Verified Cache~~ ✅ G2-A fertig (12.09.) · Persistente Storage-Layer (G2-B)  · ~~Echter TCP-Transport~~ ✅ G2-C (12.09.) · ~~Upgrade/Rollback~~ ✅ G2-D (12.09., Marker-Protokoll + Versionierung) · ~~Multi-Node-DR~~ ✅ #113 (12.09., syncd-Crate, 5 E2E-Tests mit echten Prozessen) · Externes Security-Audit (G2-E — Checkliste vorbereitet, Auditer-Entscheidung beim Owner)
