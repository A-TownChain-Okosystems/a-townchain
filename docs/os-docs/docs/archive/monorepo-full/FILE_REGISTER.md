# 📋 File Register — A-TownChain OS

> Jeder Agent dokumentiert hier welche Dateien er erstellt/geändert hat und wofür sie dienen.

## Format
| Datei | Agent | Zweck | Erstellt | Geändert |
|------|-------|------|---------|---------|
| pfad/datei.py | Aurora #N | Kurze Beschreibung | YYYY-MM-DD | YYYY-MM-DD |

---

## Aurora #1 — ShivaCore Builder

| Datei | Zweck | Erstellt | Geändert |
|------|------|---------|---------|
| src/modules/atc-shivacore/kernel/src/mempool.rs | K-Sprint 17: Memory-Pool & Tx-Validation (7 Tx-Types, NonceTracker) | 2026-08-03 | 2026-08-03 |
| src/modules/atc-shivacore/kernel/src/blockchain.rs | K-Sprint 18: Block-Proposal-Pipeline (Block → BlockChain) | 2026-08-03 | 2026-08-03 |
| src/modules/atc-shivacore/kernel/src/vm.rs | K-Sprint 19: ShivaVM (27 Opcodes, Stack-Interpreter, Gas-Metering) | 2026-08-03 | 2026-08-03 |
| src/modules/atc-shivacore/kernel/src/contract.rs | K-Sprint 20: ContractExecutor (Deploy/Call, ShivaVM↔Mempool) | 2026-08-03 | 2026-08-03 |
| src/modules/atc-shivacore/kernel/src/ai.rs | K-Sprint 21: AI-Kernel (Tensor-Ops, Neural Layers, Model-Registry) | 2026-08-03 | 2026-08-03 |
| src/modules/atc-shivacore/kernel/src/consensus.rs | K-Sprint 16: Konsens (DAG, PoH, Validator) | 2026-08-03 | 2026-08-03 |
| src/modules/atc-shivacore/kernel/src/p2p.rs | K-Sprint 14: P2P (PeerTable, Gossip, P2pNode) | 2026-08-03 | 2026-08-03 |
| src/modules/atc-shivacore/kernel/src/security.rs | K-Sprint 15: Security (Multi-Sig, Audit-Log) | 2026-08-03 | 2026-08-03 |
| src/modules/atc-shivacore/kernel/src/main.rs | Kernel Entry Point (mod-Registrierungen) | 2026-08-03 | 2026-08-03 |

## Aurora #2 — Main Developer

| Datei | Zweck | Erstellt | Geändert |
|------|------|---------|---------|
| src/modules/atc-kernel/kernel_api.atc | ATC-97 Kernel API für dezentrales KI-Betriebssystem | 2026-08-03 | 2026-08-03 |
| src/modules/atc-kernel/drivers/driver_framework.atc | Treiber Framework (abstrakte Basis) | 2026-08-03 | 2026-08-03 |
| src/modules/atc-kernel/drivers/display_driver.atc | Display-Treiber | 2026-08-03 | 2026-08-03 |
| src/modules/atc-kernel/drivers/input_driver.atc | Input-Treiber | 2026-08-03 | 2026-08-03 |
| src/modules/atc-kernel/drivers/network_driver.atc | Netzwerk-Treiber | 2026-08-03 | 2026-08-03 |
| src/modules/atc-kernel/drivers/storage_driver.atc | Storage-Treiber | 2026-08-03 | 2026-08-03 |
| src/modules/atc-kernel/ai_kernel/atc-97_agent_interaction_protocol.atc | ATC-97 Protokoll-Definition | 2026-08-03 | 2026-08-03 |
| src/modules/atclang/runtime/driver_framework.py | Python Runtime für Treiber | 2026-08-03 | 2026-08-03 |
| tests/unit/core/test_driver_framework.py | Treiber Framework Tests | 2026-08-03 | 2026-08-03 |
| src/core/kernel/api.py | Kernel API Python-Wrapper | 2026-08-03 | 2026-08-03 |
| src/core/kernel/kernel.py | Kernel Hauptmodul | 2026-08-03 | 2026-08-03 |
| tests/unit/core/test_kernel_api.py | Kernel API Unit Tests | 2026-08-03 | 2026-08-03 |
| [docs/standards/ATC-97_AGENT_INTERACTION_PROTOCOL.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/standards/ATC-97_AGENT_INTERACTION_PROTOCOL.md) | ATC-97 Spezifikation | 2026-08-03 | 2026-08-03 |
| [docs/DECISIONS_REGISTER.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/DECISIONS_REGISTER.md) | Architektur-Entscheidungen (AD-005) | 2026-08-03 | 2026-08-03 |

## Aurora #3 — ATCLang Engineer

| Datei | Zweck | Erstellt | Geändert |
|------|------|---------|---------|
| src/modules/atclang/parser/parser.py | ATCLang Parser (199/199 .atc Dateien, 100%) | 2026-08-03 | 2026-08-03 |
| src/modules/atclang/parser/ast_nodes.py | AST Node Definitionen | 2026-08-03 | 2026-08-03 |
| src/modules/atclang/lexer/lexer.py | ATCLang Lexer | 2026-08-03 | 2026-08-03 |
| src/modules/atclang/compiler/compiler.py | ATCLang Compiler | 2026-08-03 | 2026-08-03 |
| src/modules/atclang/compiler/type_checker.py | Type Checker | 2026-08-03 | 2026-08-03 |
| src/modules/atclang/compiler/optimizer.py | Code Optimizer | 2026-08-03 | 2026-08-03 |
| src/modules/atc-atclang/vm/atcvm.py | ATCLang VM | 2026-08-03 | 2026-08-03 |
| [REALITY_STATUS.md](https://github.com/A-TownChain-Okosystems/a-townchain-os-docs/blob/main/docs/monorepo-legacy/root/REALITY_STATUS.md) | Code-Realität Status (Parser 100%, Tests Status) | 2026-08-03 | 2026-08-03 |

## Replit Agent

| Datei | Zweck | Erstellt | Geändert |
|------|------|---------|---------|
| src/pages/Architecture.tsx (archiviert — altes Frontend) | GlobusOS Architektur-Seite | 2026-07-29 | 2026-07-29 |
| src/pages/Protocols.tsx (archiviert — altes Frontend) | Protokoll-Übersicht | 2026-07-28 | 2026-07-28 |
| src/pages/WasmRegistry.tsx (archiviert — altes Frontend) | Wasm Registry Seite | 2026-07-29 | 2026-07-29 |

---

*Letzte Aktualisierung: 2026-08-03 von Aurora #2*

## Updates 04.08.2026

| Datei | Agent | Zweck | Geändert |
|------|-------|------|---------|
| src/modules/atc-kernel/signals/signal_handler.atc | Aurora #2 | K42 ATCLang Interface vervollständigt (+89 Zeilen: SignalDisposition, SignalHandlerFlags, SignalAuditEntry, PendingEntry, ProcessSignalState, 20+ Methoden) | 2026-08-04 |

## K49 Update 04.08.2026

| Datei | Agent | Zweck | Geändert |
|------|-------|------|---------|
| src/modules/atc-shivacore/kernel/src/module_security.rs | Aurora #2 | K49 Module Verification & Signing (1682 Zeilen, 65 Tests, 7-Check Pipeline) | 2026-08-04 |
| src/modules/atc-kernel/module_security/module_security.atc | Aurora #2 | K49 ATCLang Interface (vollständig) | 2026-08-04 |

## K50 Update 04.08.2026

| Datei | Agent | Zweck | Geändert |
|------|-------|------|---------|
| src/modules/atc-shivacore/kernel/src/fs_journal.rs | Aurora #2 | K50 Filesystem Journaling (1161 Zeilen, 55 Tests) | 2026-08-04 |
| src/modules/atc-kernel/fs_journal/fs_journal.atc | Aurora #2 | K50 ATCLang Interface | 2026-08-04 |
| src/modules/atc-kernel/did/did.atc | Aurora #2 | K6 ATCLang Interface (nachträglich) | 2026-08-04 |
| src/modules/atc-kernel/mempool/mempool.atc | Aurora #2 | K17 ATCLang Interface (nachträglich) | 2026-08-04 |
| src/modules/atc-kernel/vm/vm.atc | Aurora #2 | K19 ATCLang Interface (nachträglich) | 2026-08-04 |
| src/modules/atc-kernel/contract/contract.atc | Aurora #2 | K20 ATCLang Interface (nachträglich) | 2026-08-04 |
| src/modules/atc-kernel/userspace/userspace.atc | Aurora #2 | K30 ATCLang Interface (nachträglich) | 2026-08-04 |
| src/modules/atc-kernel/elf_loader/elf_loader.atc | Aurora #2 | K31 ATCLang Interface (nachträglich) | 2026-08-04 |
| src/modules/atc-kernel/page_fault/page_fault.atc | Aurora #2 | K32 ATCLang Interface (nachträglich) | 2026-08-04 |
| src/modules/atc-kernel/sockets/sockets.atc | Aurora #2 | K37 ATCLang Interface (nachträglich) | 2026-08-04 |
| src/modules/atc-kernel/threads/threads.atc | Aurora #2 | K39 ATCLang Interface (nachträglich) | 2026-08-04 |
| src/modules/atc-kernel/power/power.atc | Aurora #2 | K40 ATCLang Interface (nachträglich) | 2026-08-04 |
| docs/K9_K13_GAP.md (archiviert → Docs-Hub) | Aurora #2 | K9-K13 Sprint Gap Dokumentation | 2026-08-04 |
