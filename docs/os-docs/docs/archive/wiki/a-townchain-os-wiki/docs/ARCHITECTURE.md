# 🏛️ Architektur — a-townchain-os

> **Repo:** [a-townchain-os](https://github.com/A-TownChain-Okosystems/a-townchain-os)
> **Layer:** L0–L12 | **Titel:** Monorepo
> **Stand:** 2026-08-06 | **Version:** v1.0.0

---

## Übersicht

Monorepo: 284 ATCLang-Module, 240 Python-Stubs, 63 Rust-Kernel, 199 TypeScript Frontend. Alle Layer L0–L12.

## Komponenten

### ATCLang Module (.atc)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `archive/atclang-v01/atcos_main.atc` | 1161 | Atcos Main |
| `archive/atclang-v01/consensus/fork_resolution.atc` | 145 | Fork Resolution |
| `archive/atclang-v01/consensus/gas_fee.atc` | 130 | Gas Fee |
| `archive/atclang-v01/consensus/hybrid_consensus.atc` | 357 | Hybrid Consensus |
| `archive/atclang-v01/consensus/poh.atc` | 140 | Poh |
| `archive/atclang-v01/consensus/pos.atc` | 164 | Pos |
| `archive/atclang-v01/consensus/pow.atc` | 107 | Pow |
| `archive/atclang-v01/contracts/breeding.atc` | 139 | Breeding |
| `archive/atclang-v01/contracts/contract_engine_atc14.atc` | 309 | Contract Engine Atc14 |
| `archive/atclang-v01/contracts/genesis_token.atc` | 102 | Genesis Token |
| `archive/atclang-v01/contracts/governance_contract.atc` | 202 | Governance Contract |
| `archive/duplicates/contract_registry.atc` | 98 | Contract Registry |
| `archive/duplicates/kai_cli.atc` | 195 | Kai Cli |
| `archive/duplicates/smart_contract_registry.atc` | 88 | Smart Contract Registry |
| `archive/duplicates/smart_contracts.atc` | 486 | Smart Contracts |
| `atclang/atc-atclang/programs/atcos_main.atc` | 1161 | Atcos Main |
| `atclang/programs/atc8300.atc` | 96 | Atc8300 |
| `atclang/programs/atcfs.atc` | 142 | Atcfs |
| `atclang/programs/atcnet.atc` | 135 | Atcnet |
| `atclang/programs/atcos_main.atc` | 9 | Atcos Main |
| `atclang/programs/consensus.atc` | 144 | Consensus |
| `atclang/programs/event_bus.atc` | 75 | Event Bus |
| `atclang/programs/gateway.atc` | 138 | Gateway |
| `atclang/programs/governance.atc` | 113 | Governance |
| `atclang/programs/kernel.atc` | 148 | Kernel |

*+259 weitere*

### Python Module (.py)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `atclang/atc-atclang/compiler/compiler.py` | 561 | Compiler |
| `atclang/atc-atclang/compiler/optimizer.py` | 558 | Optimizer |
| `atclang/atc-atclang/compiler/type_checker.py` | 507 | Type Checker |
| `atclang/atc-atclang/lexer/lexer.py` | 671 | Lexer |
| `atclang/atc-atclang/parser/ast_nodes.py` | 392 | Ast Nodes |
| `atclang/atc-atclang/parser/parser.py` | 1431 | Parser |
| `atclang/atc-atclang/repl/repl.py` | 184 | Repl |
| `atclang/atc-atclang/stdlib/atc_stdlib.py` | 69 | Atc Stdlib |
| `atclang/atc-atclang/stdlib/chain.py` | 41 | Chain |
| `atclang/atc-atclang/stdlib/collections.py` | 219 | Collections |
| `atclang/atc-atclang/stdlib/collections_ext.py` | 143 | Collections Ext |
| `atclang/atc-atclang/stdlib/crypto.py` | 155 | Crypto |
| `atclang/atc-atclang/stdlib/crypto_ext.py` | 149 | Crypto Ext |
| `atclang/atc-atclang/stdlib/encoding.py` | 210 | Encoding |
| `atclang/atc-atclang/stdlib/io.py` | 107 | Io |
| `atclang/atc-atclang/stdlib/io_ext.py` | 123 | Io Ext |
| `atclang/atc-atclang/stdlib/math.py` | 154 | Math |
| `atclang/atc-atclang/stdlib/primitives.py` | 244 | Primitives |
| `atclang/atc-atclang/stdlib/string.py` | 99 | String |
| `atclang/atc-atclang/stdlib/wallet.py` | 78 | Wallet |
| `atclang/atc-atclang/v03/atclang_v03_features.py` | 352 | Atclang V03 Features |
| `atclang/atc-atclang/vm/atcvm.py` | 997 | Atcvm |
| `atclang/compiler.py` | 102 | Compiler |
| `atclang/compiler/compiler.py` | 626 | Compiler |
| `atclang/compiler/optimizer.py` | 558 | Optimizer |

*+123 weitere*

### Rust Module (.rs)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `linux/src/main.rs` | 15 | Main |
| `shivacore/boot/src/main.rs` | 30 | Main |
| `shivacore/kernel/src/ai.rs` | 75 | Ai |
| `shivacore/kernel/src/allocator.rs` | 46 | Allocator |
| `shivacore/kernel/src/atcfs.rs` | 627 | Atcfs |
| `shivacore/kernel/src/atcnet.rs` | 1139 | Atcnet |
| `shivacore/kernel/src/ats1000.rs` | 85 | Ats1000 |
| `shivacore/kernel/src/block.rs` | 548 | Block |
| `shivacore/kernel/src/blockchain.rs` | 57 | Blockchain |
| `shivacore/kernel/src/capability.rs` | 248 | Capability |
| `shivacore/kernel/src/consensus.rs` | 961 | Consensus |
| `shivacore/kernel/src/container.rs` | 2757 | Container |
| `shivacore/kernel/src/container_net.rs` | 632 | Container Net |
| `shivacore/kernel/src/contract.rs` | 38 | Contract |
| `shivacore/kernel/src/cow.rs` | 1484 | Cow |
| `shivacore/kernel/src/cross_subsystem.rs` | 483 | Cross Subsystem |
| `shivacore/kernel/src/devfs.rs` | 921 | Devfs |
| `shivacore/kernel/src/did.rs` | 350 | Did |
| `shivacore/kernel/src/elf_loader.rs` | 1104 | Elf Loader |
| `shivacore/kernel/src/framebuffer.rs` | 122 | Framebuffer |
| `shivacore/kernel/src/fs_journal.rs` | 1161 | Fs Journal |
| `shivacore/kernel/src/gdt.rs` | 59 | Gdt |
| `shivacore/kernel/src/genesis.rs` | 1111 | Genesis |
| `shivacore/kernel/src/genesis_bridge.rs` | 1097 | Genesis Bridge |
| `shivacore/kernel/src/gossip_bridge.rs` | 1410 | Gossip Bridge |

*+38 weitere*

### TypeScript Module (.ts/.tsx)

| Datei | Zeilen | Beschreibung |
|------|--------|---------------|
| `aistudio/mark_completed.ts` | 15 | Mark Completed |
| `aistudio/mark_completed_src.ts` | 33 | Mark Completed Src |
| `aistudio/server.ts` | 866 | Server |
| `aistudio/src/App.tsx` | 5440 | App |
| `aistudio/src/DesktopApp.tsx` | 2740 | Desktopapp |
| `aistudio/src/atcLangRoadmapData.ts` | 201 | Atclangroadmapdata |
| `aistudio/src/atcLangWikiData.ts` | 227 | Atclangwikidata |
| `aistudio/src/auditData.ts` | 76 | Auditdata |
| `aistudio/src/backend/blockchain/engine.ts` | 129 | Engine |
| `aistudio/src/backend/p2p/network.ts` | 77 | Network |
| `aistudio/src/components/ATCAssetView.tsx` | 191 | Atcassetview |
| `aistudio/src/components/ATCDjStudioView.tsx` | 445 | Atcdjstudioview |
| `aistudio/src/components/ATCLangEditor.tsx` | 625 | Atclangeditor |
| `aistudio/src/components/ATCWalletView.tsx` | 498 | Atcwalletview |
| `aistudio/src/components/ATownDashboardView.tsx` | 302 | Atowndashboardview |
| `aistudio/src/components/ATownOSNode.tsx` | 1439 | Atownosnode |
| `aistudio/src/components/ATownTestView.tsx` | 111 | Atowntestview |
| `aistudio/src/components/AgentCivilizationView.tsx` | 152 | Agentcivilizationview |
| `aistudio/src/components/Ai3DRenderEngineTab.tsx` | 199 | Ai3Drenderenginetab |
| `aistudio/src/components/AiAnimationEngineTab.tsx` | 198 | Aianimationenginetab |
| `aistudio/src/components/AiAudioEngineTab.tsx` | 198 | Aiaudioenginetab |
| `aistudio/src/components/AiCharacterBioTab.tsx` | 199 | Aicharacterbiotab |
| `aistudio/src/components/AiGameEngineTab.tsx` | 200 | Aigameenginetab |
| `aistudio/src/components/AiKernelView.tsx` | 128 | Aikernelview |
| `aistudio/src/components/AiOsEngineView.tsx` | 490 | Aiosengineview |

*+168 weitere*

## Statistik

| Metrik | Wert |
|--------|------|
| .atc | 284 |
| .py | 148 |
| .rs | 63 |
| .ts | 193 |
| Total Zeilen | 191,089 |

---

*Auto-generiert 2026-08-06 · Aurora*
