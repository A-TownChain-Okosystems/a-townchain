# ARCHITECTURE.md — atc-shivacore

> Copyright © Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.

## File Tree
```tree
atc-shivacore/
├── Cargo.toml — Kernel crate configuration for no_std x86_64-unknown-none target
├── .gitignore — Git ignore configuration for build artifacts and target directories
└── kernel/
    └── src/
        ├── lib.rs — Kernel crate root, global module declarations, and main initialization
        ├── boot.rs — Early boot sequence & multi-stage initialization pipeline (K0-K40)
        ├── gdt.rs — Global Descriptor Table setup, segment descriptors, and TSS loading
        ├── interrupts.rs — Interrupt Descriptor Table, exception vectors, and ISR routines
        ├── serial.rs — Serial port driver for kernel logging and debug output
        ├── allocator.rs — Global memory allocator implementation for no_std environment
        ├── capability.rs — Capability-based access control framework and security tokens
        ├── process.rs — Process control blocks (PCB), process creation, and lifecycle
        ├── scheduler.rs — Preemptive task scheduler & multi-core CPU scheduling (DA-HEFT)
        ├── ipc.rs — Inter-process communication primitives and zero-copy message queues
        ├── did.rs — Decentralized Identity primitives integrated into kernel safety layer
        ├── remote_caps.rs — Remote capability tickets, resolver, and replay protection
        ├── knowledge_graph.rs — In-kernel structured knowledge graph and state store
        ├── memory_manager.rs — Memory manager with heap bridge and capability-gated allocation
        ├── atcfs.rs — Content-addressed filesystem (ATCFS) with BLAKE3 hashing
        ├── ats1000.rs — ATS-1000 trait definitions for kernel subsystem interfaces
        ├── cross_subsystem.rs — Cross-subsystem integration tests and workflow validation
        ├── kernel_init.rs — Unified boot sequence (L0-L10) and KernelState management
        ├── atcnet.rs — ATCNet P2P protocol handler with 10 message types
        ├── block.rs — Block structure, serialization, and chain management
        ├── consensus.rs — Consensus engine with PoW+PoS+PoH hybrid validation
        ├── genesis.rs — Genesis block configuration, signing, and verification (Chain-ID 9000)
        ├── genesis_bridge.rs — Bridge between genesis config and blockchain/consensus
        ├── gossip_bridge.rs — P2P gossip integration for block/vote/mempool propagation
        ├── security_audit.rs — Security auditor with 7 categories and 5 attack simulations
        ├── ai.rs — AI subsystem integration layer
        ├── container.rs — Container isolation and resource limits
        ├── container_net.rs — Container networking and namespace management
        ├── contract.rs — Smart contract execution interface
        ├── cow.rs — Copy-on-write memory management
        ├── devfs.rs — Device filesystem (/dev) nodes and dynamic driver bindings
        ├── elf_loader.rs — Executable and Linkable Format (ELF) binary loader
        ├── framebuffer.rs — Framebuffer driver for graphics output
        ├── fs_journal.rs — Filesystem journaling for crash recovery
        ├── lkm.rs — Loadable kernel module support
        ├── mempool.rs — Transaction mempool management
        ├── module_security.rs — Module signing and integrity verification
        ├── net.rs — Network stack abstraction and packet management
        ├── p2p.rs — Peer-to-peer networking primitives
        ├── page_fault.rs — Page fault interrupt handler, demand paging
        ├── power.rs — Power management and ACPI interface
        ├── security.rs — Kernel ring protection, permissions, and access control
        ├── signals.rs — Signal handling and delivery
        ├── smp.rs — Symmetric multiprocessing support
        ├── sockets.rs — Socket network primitives for userspace applications
        ├── system.rs — System telemetry, system info syscalls, and shutdown/reboot
        ├── tcpip.rs — In-kernel TCP/IP protocol suite implementation
        ├── threads.rs — Kernel thread management and synchronization primitives
        ├── timer.rs — Programmable Interval Timer (PIT) and LAPIC timer drivers
        ├── tracing.rs — Execution tracing and performance profiling
        ├── user_io.rs — User-space buffer memory validation and safe copying
        ├── user_sched.rs — User-level thread scheduling policy and task queue
        ├── userspace.rs — Ring 3 context switching and user execution isolation
        ├── vfs.rs — Virtual File System abstraction, mount table, and file descriptors
        └── vm.rs — Virtual machine and hypervisor abstractions
```

## Module Descriptions
- kernel/src/lib.rs — Entry point for ShivaCore kernel library exposing core subsystems.
- kernel/src/boot.rs — Executes boot stages K0 through K40 to initialize hardware, memory, and kernel subsystems.
- kernel/src/gdt.rs / interrupts.rs / serial.rs — Sets up low-level CPU descriptor tables, interrupt handling, and serial I/O.
- kernel/src/allocator.rs / memory_manager.rs — Manages no_std global heap allocator and capability-gated memory regions.
- kernel/src/capability.rs / security.rs — Implements object-level capability security and ring protection.
- kernel/src/process.rs / scheduler.rs / threads.rs — Controls task state, process tree execution, context switching, and multi-core scheduling.
- kernel/src/ipc.rs / sockets.rs — Provides synchronous and asynchronous zero-copy inter-process communication channels.
- kernel/src/did.rs / remote_caps.rs — Embedded cryptographic identity verification and remote capability tickets for decentralized kernel operations.
- kernel/src/knowledge_graph.rs — In-kernel graph database for dynamic entity and permission state tracking.
- kernel/src/atcfs.rs / devfs.rs — Content-addressed filesystem and device node abstractions.
- kernel/src/ats1000.rs / cross_subsystem.rs / kernel_init.rs — Trait definitions, integration tests, and unified boot sequence.
- kernel/src/atcnet.rs / net.rs / p2p.rs / tcpip.rs — ATCNet P2P protocol, network stack, and TCP/IP suite.
- kernel/src/block.rs / consensus.rs / genesis.rs / genesis_bridge.rs / gossip_bridge.rs — Blockchain, consensus, genesis configuration, and P2P gossip integration.
- kernel/src/security_audit.rs — Security auditor with 7 categories and 5 attack vector simulations.
- kernel/src/userspace.rs / elf_loader.rs / page_fault.rs / user_sched.rs / user_io.rs — User-space execution, binary loading, demand paging, and safe I/O.

## Build System
- Cargo.toml — Configured for bare-metal `#![no_std]` Rust compilation targeting `x86_64-unknown-none`.
- 367/367 tests passing across 30 modules (K-Sprint 29).

## Dependencies
- spin — Bare-metal spinlock primitives for multi-threaded kernel synchronization.
- x86_64 — Low-level x86_64 CPU instructions, control registers, and page table definitions.
- volatile — Safe access wrappers for memory-mapped hardware I/O addresses.
- lazy_static / conquer-once — Thread-safe static initialization for kernel data structures.
- ed25519-dalek — Ed25519 cryptographic signature verification.
- sha3 — SHA-3 (Keccak) hashing for content-addressed storage.
- linked_list_allocator — Heap allocator for no_std environments.
