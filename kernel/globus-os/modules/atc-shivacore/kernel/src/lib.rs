// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Library Crate für Test-Ausführung
#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

extern crate alloc;
#[cfg(test)]
extern crate std;

#[cfg(test)]
pub mod allocator;
pub mod atcfs;
pub mod ats1000;
pub mod capability;
pub mod cross_subsystem;
pub mod diagnostics;
pub mod elf_loader;
pub mod hal;
pub mod ipc;
pub mod kernel_init;
pub mod memory_manager;
pub mod process;
pub mod scheduler;
pub mod security;
pub mod syscall;
pub mod system;
pub mod timer;
pub mod user_sched;
pub mod userspace;
pub mod vfs;
pub mod vmm;

#[cfg(test)]
pub mod lkm;
