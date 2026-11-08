# Kai-OS-Wiki Migration Manifest

> **Date:** 2026-08-09 | **Agent:** Aurora (Base44)
> **Source:** kai-os-wiki (127 Repos, 1.55MB, 50 files)
> **Target:** a-townchain-os-docs/docs/archive/kai-os-legacy/

## Status

- **Batch 1 (Priority Docs):** 50/50 ✅
- **Batch 2 (Compiler + Blockchain):** 52/52 ✅
- **Remaining:** ~588 small/stub files (subdirectory contents, __init__.py files)

## Migrated Content

### Top-Level Documentation (20 files)
- AGENT_MASTERRULES.md (13KB) — Agent governance rules
- ARCHITECTURE.md (42KB) — Legacy system architecture
- FILE_REGISTER.md (41KB) — Complete file registry
- KONSOLIDIERUNGS_ROADMAP.md (15KB) — Consolidation roadmap
- SPRINT_ROADMAP.md (21KB) — Sprint planning
- ECOSYSTEM.md (8.7KB) — Ecosystem overview
- CONNECTION_MAP.md (2.6KB) — Repo connection map
- NAMING_CONVENTIONS.md (4.7KB) — Naming standards
- GENESIS_BUS_ARCHITECTURE.md (5.8KB) — Bus architecture
- GENESIS_CIVILIZATION_PLATFORM_v4.md (6KB) — Civilization platform
- GENESIS_COMMUNICATION_LAYER_v2.md (15KB) — Communication layer
- GENESIS_FRANCHISE_FACTORY_v1/v2.md (11KB) — Franchise factory
- AAA_ASSET_SYSTEM_v1.md (3.2KB) — Asset system
- PERFORMANCE_REPORT.md (3.5KB) — Performance report
- FIXES.md (3.3KB) — Bug fixes log
- MILESTONES.md (1KB) — Milestones
- TODO.md (2.4KB) — Todo list
- ATCLANG_FIRST.md (900B) — ATCLang intro
- AGENT_MANIFEST.md (2.8KB) — Agent manifest
- start.atc (4.3KB) — Main entry point

### Config (1 file)
- config/mainnet_genesis.json (3.1KB) — Mainnet genesis configuration

### docs/ Directory (24 files)
- AGENT_COORDINATION.md (26KB) — Agent coordination protocol
- AGENT_POLICY.md (13KB) — Agent policy
- ATCLANG_AGENT_BUILD_GUIDE.md (23KB) — Build guide
- SHIVACORE_KERNEL_STATUS.md (39KB) — Kernel status
- REALITY_CHECK_2026-07-06.md (29KB) — Reality check
- And 19 more...

### ATCLang Compiler (32 files)
- ATCLANG_SPEC.md (9.8KB) — Language spec
- compiler.py (21KB), optimizer.py (23KB), type_checker.py (20KB)
- lexer.py (21KB), parser.py (38KB), ast_nodes.py (7.2KB)
- atcvm.py (48KB) — Virtual Machine
- stdlib/ — 14 standard library modules
- atcos_main.atc (41KB) — Main ATCLang program

### Blockchain .atc (7 files)
- smart_contracts.atc (15KB), contract_registry.atc (3.4KB)
- smart_contract_registry.atc (2.8KB)
- consensus/fork_atc85.atc, fork_resolution.atc, gas_fee.atc, gas_fee_atc86.atc

### Gateway .atc (2 files)
- main.atc (5.3KB), service_discovery.atc (6.2KB)

### Code (Python) (7 files)
- KAI_OS_SUMMARY.py, atc_issues_summary.py, bootscreen_complete.py
- ecdsa_final.py, ecdsa_impl.py, start.py

### AIStudio (3 files)
- SOFTWARE_ROADMAP.md (39KB), AGENTS.md, GEMINI.md

### Other (4 files)
- core/kai_cli.atc, atcpkg/manager.atc, conftest.py, devnet/README.md

## Next Steps

1. Remaining ~588 subdirectory files can be migrated in subsequent batches
2. After full migration, kai-os-wiki can be archived/deleted
3. Update CONNECTION_MAP.md references in all repos
