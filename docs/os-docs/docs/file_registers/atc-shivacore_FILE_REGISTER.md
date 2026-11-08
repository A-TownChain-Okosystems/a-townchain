# 📋 File Register — atc-shivacore

> **Regeneriert 06.09.2026:** Pfade zeigen auf `src/modules/atc-shivacore/` im Monorepo `a-townchain-os` (nach 128→2-Konsolidierung).

> **Zweck:** Vollständige Liste aller existierenden Dateien in `atc-shivacore`.
> Dient als Nachweis, welche Dateien tatsächlich existieren.
>
> **Auto-generiert:** 2026-08-05 04:00 UTC | **Agent:** Aurora (MasterBrain · Base44)
> **Letzter Commit:** `382f30d feat: K-Sprint 50 — Filesystem Journaling (1161 Zeilen, 55 Tests)`

---

## Zusammenfassung

| Metrik | Wert |
|--------|------|
| **Total Dateien** | 2151 |
| **Markdown (.md)** | 1 |
| **Python (.py)** | 0 |
| **ATCLang (.atc)** | 0 |
| **Rust (.rs)** | 71 |
| **TypeScript (.ts)** | 0 |
| **Andere** | 2079 |

---

## Dateiliste

| Datei | Zeilen | Typ |
|------|--------|-----|
| `Cargo.toml` | 12 | toml |
| `README.md` | 1074 | md |
| `src/modules/atc-shivacore/boot/Cargo.toml` | 8 | toml |
| `src/modules/atc-shivacore/boot/src/main.rs` | 30 | rs |
| `src/modules/atc-shivacore/kernel/.cargo/config.toml` | 2 | toml |
| `kernel/.gitignore` | 1 | gitignore |
| `src/shivacore/kernel/Cargo.lock` | 497 | lock |
| `src/modules/atc-shivacore/kernel/Cargo.toml` | 37 | toml |
| `src/modules/atc-shivacore/kernel/src/ai.rs` | 75 | rs |
| `src/modules/atc-shivacore/kernel/src/allocator.rs` | 47 | rs |
| `src/modules/atc-shivacore/kernel/src/atcfs.rs` | 627 | rs |
| `src/modules/atc-shivacore/kernel/src/atcnet.rs` | 1139 | rs |
| `src/modules/atc-shivacore/kernel/src/ats1000.rs` | 94 | rs |
| `src/modules/atc-shivacore/kernel/src/block.rs` | 548 | rs |
| `src/modules/atc-shivacore/kernel/src/blockchain.rs` | 57 | rs |
| `src/modules/atc-shivacore/kernel/src/capability.rs` | 248 | rs |
| `src/modules/atc-shivacore/kernel/src/consensus.rs` | 961 | rs |
| `src/modules/atc-shivacore/kernel/src/container.rs` | 2757 | rs |
| `src/modules/atc-shivacore/kernel/src/container_net.rs` | 632 | rs |
| `src/modules/atc-shivacore/kernel/src/contract.rs` | 38 | rs |
| `src/modules/atc-shivacore/kernel/src/cow.rs` | 1484 | rs |
| `src/modules/atc-shivacore/kernel/src/cross_subsystem.rs` | 483 | rs |
| `src/modules/atc-shivacore/kernel/src/devfs.rs` | 921 | rs |
| `src/modules/atc-shivacore/kernel/src/did.rs` | 350 | rs |
| `src/modules/atc-shivacore/kernel/src/elf_loader.rs` | 1104 | rs |
| `src/modules/atc-shivacore/kernel/src/framebuffer.rs` | 122 | rs |
| `src/modules/atc-shivacore/kernel/src/fs_journal.rs` | 1161 | rs |
| `src/modules/atc-shivacore/kernel/src/gdt.rs` | 59 | rs |
| `src/modules/atc-shivacore/kernel/src/genesis.rs` | 1111 | rs |
| `src/modules/atc-shivacore/kernel/src/genesis_bridge.rs` | 1097 | rs |
| `src/modules/atc-shivacore/kernel/src/gossip_bridge.rs` | 1410 | rs |
| `src/modules/atc-shivacore/kernel/src/hw_drivers.rs` | 1267 | rs |
| `src/modules/atc-shivacore/kernel/src/interrupts.rs` | 100 | rs |
| `src/modules/atc-shivacore/kernel/src/ipc.rs` | 600 | rs |
| `src/modules/atc-shivacore/kernel/src/kernel_init.rs` | 431 | rs |
| `src/modules/atc-shivacore/kernel/src/knowledge_graph.rs` | 755 | rs |
| `src/modules/atc-shivacore/kernel/src/lib.rs` | 73 | rs |
| `src/modules/atc-shivacore/kernel/src/lkm.rs` | 2997 | rs |
| `src/modules/atc-shivacore/kernel/src/main.rs` | 164 | rs |
| `src/modules/atc-shivacore/kernel/src/memory.rs` | 75 | rs |
| `src/modules/atc-shivacore/kernel/src/memory_manager.rs` | 829 | rs |
| `src/modules/atc-shivacore/kernel/src/mempool.rs` | 75 | rs |
| `src/modules/atc-shivacore/kernel/src/module_security.rs` | 1682 | rs |
| `src/modules/atc-shivacore/kernel/src/net.rs` | 802 | rs |
| `src/modules/atc-shivacore/kernel/src/p2p.rs` | 861 | rs |
| `src/modules/atc-shivacore/kernel/src/page_fault.rs` | 1371 | rs |
| `src/modules/atc-shivacore/kernel/src/power.rs` | 1153 | rs |
| `src/modules/atc-shivacore/kernel/src/process.rs` | 360 | rs |
| `src/modules/atc-shivacore/kernel/src/remote_caps.rs` | 629 | rs |
| `src/modules/atc-shivacore/kernel/src/scheduler.rs` | 389 | rs |
| `src/modules/atc-shivacore/kernel/src/security.rs` | 879 | rs |
| `src/modules/atc-shivacore/kernel/src/security_audit.rs` | 1264 | rs |
| `src/modules/atc-shivacore/kernel/src/serial.rs` | 42 | rs |
| `src/modules/atc-shivacore/kernel/src/signals.rs` | 2249 | rs |
| `src/modules/atc-shivacore/kernel/src/smp.rs` | 2506 | rs |
| `src/modules/atc-shivacore/kernel/src/sockets.rs` | 1526 | rs |
| `src/modules/atc-shivacore/kernel/src/syscall.rs` | 1081 | rs |
| `src/modules/atc-shivacore/kernel/src/system.rs` | 1254 | rs |
| `src/modules/atc-shivacore/kernel/src/tcpip.rs` | 860 | rs |
| `src/modules/atc-shivacore/kernel/src/threads.rs` | 1467 | rs |
| `src/modules/atc-shivacore/kernel/src/timer.rs` | 528 | rs |
| `src/modules/atc-shivacore/kernel/src/tracing.rs` | 2254 | rs |
| `src/modules/atc-shivacore/kernel/src/user_io.rs` | 1323 | rs |
| `src/modules/atc-shivacore/kernel/src/user_sched.rs` | 1201 | rs |
| `src/modules/atc-shivacore/kernel/src/userspace.rs` | 840 | rs |
| `src/modules/atc-shivacore/kernel/src/vfs.rs` | 1099 | rs |
| `src/modules/atc-shivacore/kernel/src/vm.rs` | 54 | rs |
| `src/modules/atc-shivacore/kernel/src/vmm.rs` | 2362 | rs |

---

*atc-shivacore · A-TownChain Ökosystem · v1.0.0*
