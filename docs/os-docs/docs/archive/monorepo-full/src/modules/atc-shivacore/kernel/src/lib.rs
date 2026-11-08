// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Library Crate für Test-Ausführung
//!
//! Re-exportiert alle Kernel-Module für Unit- und Integrationstests.
#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

#[cfg(test)]
extern crate std;

extern crate alloc;

pub mod allocator;
pub mod ats1000;
// [K29-Build] ausgeschlossen: pub mod framebuffer;
// [K29-Build] ausgeschlossen: pub mod gdt;
// [K29-Build] ausgeschlossen: pub mod interrupts;
// [K29-Build] ausgeschlossen: pub mod memory;
// [K29-Build] ausgeschlossen: pub mod serial;
pub mod capability;
pub mod process;
pub mod scheduler;
pub mod ipc;
pub mod did;
pub mod remote_caps;
pub mod knowledge_graph;
pub mod memory_manager;
pub mod atcfs;
pub mod vfs;
// [K29-Build] ausgeschlossen: pub mod syscall;
pub mod timer;
// [K29-Build] ausgeschlossen: pub mod block;
pub mod net;
pub mod tcpip;
pub mod p2p;
pub mod security;
pub mod consensus;
pub mod mempool;
pub mod blockchain;
pub mod vm;
pub mod contract;
pub mod ai;
pub mod kernel_init;
pub mod cross_subsystem;
pub mod atcnet;
pub mod genesis;
pub mod genesis_bridge;
pub mod gossip_bridge;
pub mod security_audit;
// [K29-Build] ausgeschlossen: pub mod userspace;
// [K29-Build] ausgeschlossen: pub mod elf_loader;
// [K29-Build] ausgeschlossen: pub mod page_fault;
// [K29-Build] ausgeschlossen: pub mod user_sched;
// [K29-Build] ausgeschlossen: pub mod user_io;
// [K29-Build] ausgeschlossen: pub mod hw_drivers;
// [K29-Build] ausgeschlossen: pub mod system;
// [K29-Build] ausgeschlossen: pub mod sockets;
// [K29-Build] ausgeschlossen: pub mod devfs;
// [K29-Build] ausgeschlossen: pub mod threads;
// [K29-Build] ausgeschlossen: pub mod power;
// [K29-Build] ausgeschlossen: pub mod container;
// [K29-Build] ausgeschlossen: pub mod signals;
// [K29-Build] ausgeschlossen: pub mod smp;
// [K29-Build] ausgeschlossen: pub mod vmm;
// [K29-Build] ausgeschlossen: pub mod cow;
// [K29-Build] ausgeschlossen: pub mod tracing;
// [K29-Build] ausgeschlossen: pub mod container_net;
// [K29-Build] ausgeschlossen: pub mod lkm;
// [K29-Build] ausgeschlossen: pub mod module_security;
// [K29-Build] ausgeschlossen: pub mod fs_journal;
