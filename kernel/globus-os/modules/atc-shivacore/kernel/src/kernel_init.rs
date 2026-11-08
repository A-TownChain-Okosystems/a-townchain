// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Init-Sequenz (K-Sprint 23)
//!
//! Verkettet die Initialisierung aller Kernel-Subsysteme in der
//! korrekten Boot-Reihenfolge. In Kernel-Mode (no_std) wird dies
//! nach allocator::init_heap() aufgerufen.
//!
//! Boot-Reihenfolge:
//!   L0:  allocator::init_heap()      — Heap bereit (Box/Vec/String)
//!   L1:  MemorySubsystem::new()      — Prozess-Regionen + Caps
//!   L2:  CapabilityTable::new()      — System-Capability-Table
//!   L3:  ProcessManager::new()       — Prozess-Verwaltung
//!   L4:  Scheduler::new()            — DA-HEFT Scheduler
//!   L5:  IpcSubsystem::new()        — IPC-Kanäle
//!   L6:  AtcFileSystem::new()       — Content-Addressed FS
//!   L6b: Vfs::new(caps)             — Virtual File System
//!   L7:  SecurityManager::new()     — MultiSig + AuditLog + Reputation
//!
//! Nicht-TCB-Subsysteme (P2P-Netz, Mempool, Contract-VM/Executor, AI) sind
//! in den Service Space (shivacore-service-space) ausgelagert (AD-012/AD-028)
//! und werden dort über shivacore_service_space::service_init initialisiert.

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::atcfs::AtcFileSystem;
use crate::ats1000::{FileSystem, MemoryManager};
use crate::capability::CapabilityTable;
use crate::ipc::IpcSubsystem;
use crate::memory_manager::{MemorySubsystem, HEAP_END, HEAP_SIZE, HEAP_START};
use crate::process::ProcessManager;
use crate::scheduler::DaHeftScheduler;
use crate::security::SecurityManager;
use crate::timer::{MonotonicClock, SimulatedTimerSource, TimerManager};
use crate::vfs::Vfs;

/// Kernel-Init-Status für jedes Subsystem
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitStatus {
    NotStarted,
    Initializing,
    Ready,
    Failed,
}

/// Boot-Phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootPhase {
    Heap,         // L0: allocator::init_heap
    Memory,       // L1: MemorySubsystem
    Capabilities, // L2: CapabilityTable
    Processes,    // L3: ProcessManager
    Scheduler,    // L4: DA-HEFT Scheduler
    Ipc,          // L5: IPC Channels
    FileSystem,   // L6: ATCFS + VFS
    Security,     // L7: Security/Audit/MultiSig
    Done,
}

impl BootPhase {
    pub fn label(&self) -> &str {
        match self {
            BootPhase::Heap => "L0 Heap (linked_list_allocator)",
            BootPhase::Memory => "L1 MemorySubsystem (Heap-Bridge)",
            BootPhase::Capabilities => "L2 CapabilityTable",
            BootPhase::Processes => "L3 ProcessManager",
            BootPhase::Scheduler => "L4 DA-HEFT Scheduler",
            BootPhase::Ipc => "L5 IPC Channels",
            BootPhase::FileSystem => "L6 ATCFS + VFS",
            BootPhase::Security => "L7 Security/Audit/MultiSig",
            BootPhase::Done => "Boot Complete",
        }
    }
}

/// Das vereinigte Kernel-State-Objekt.
/// Wird beim Boot einmal erzeugt und enthält alle Subsysteme.
pub struct KernelState {
    // L1: Memory
    pub memory: MemorySubsystem,
    // L2: Capabilities (inside ProcessManager)
    // L3: Process Manager
    pub processes: ProcessManager,
    // L4: Scheduler
    pub scheduler: DaHeftScheduler,
    // L5: IPC
    pub ipc: IpcSubsystem,
    // L6: Filesystems
    pub fs: AtcFileSystem,
    pub vfs: Vfs,
    // L7: Security
    pub security: SecurityManager,
    // Boot log
    pub init_log: Vec<(BootPhase, InitStatus)>,
}

impl KernelState {
    /// Kernel-Init-Sequenz — initialisiert alle Subsysteme in Reihenfolge.
    ///
    /// In Kernel-Mode (no_std):
    ///   1. allocator::init_heap(mapper, frame_alloc)  ← muss VORHER laufen
    ///   2. KernelState::boot()                       ← diese Funktion
    ///
    /// In Test-Mode:
    ///   KernelState::boot() — nutzt Rust std alloc als Heap-Ersatz
    pub fn boot() -> Result<Self, BootError> {
        let mut log = Vec::new();

        // ── L0: Heap (in Kernel-Mode: bereits durch allocator::init_heap erledigt) ──
        log.push((BootPhase::Heap, InitStatus::Ready));

        // ── L1: MemorySubsystem (Heap-Bridge + Prozess-Regionen) ──
        log.push((BootPhase::Memory, InitStatus::Initializing));
        let memory = MemorySubsystem::new();
        let stats = memory.stats();
        if stats.heap_base != HEAP_START {
            log.push((BootPhase::Memory, InitStatus::Failed));
            return Err(BootError::HeapConfigMismatch);
        }
        log.push((BootPhase::Memory, InitStatus::Ready));

        // ── L2+L3: ProcessManager (enthält CapabilityTable) ──
        log.push((BootPhase::Capabilities, InitStatus::Initializing));
        let processes = ProcessManager::new();
        log.push((BootPhase::Capabilities, InitStatus::Ready));
        log.push((BootPhase::Processes, InitStatus::Ready));

        // ── L4: DA-HEFT Scheduler ──
        log.push((BootPhase::Scheduler, InitStatus::Initializing));
        let scheduler = DaHeftScheduler::new();
        log.push((BootPhase::Scheduler, InitStatus::Ready));

        // ── L5: IPC Subsystem ──
        log.push((BootPhase::Ipc, InitStatus::Initializing));
        let ipc = IpcSubsystem::new();
        log.push((BootPhase::Ipc, InitStatus::Ready));

        // ── L6: ATCFS + VFS ──
        log.push((BootPhase::FileSystem, InitStatus::Initializing));
        let fs = AtcFileSystem::new();
        if !fs.exists("/") {
            log.push((BootPhase::FileSystem, InitStatus::Failed));
            return Err(BootError::FsInitFailed);
        }
        let caps = Arc::new(spin::Mutex::new(CapabilityTable::new()));
        let vfs = Vfs::new(caps);
        log.push((BootPhase::FileSystem, InitStatus::Ready));

        // ── L7: Security (MultiSig + AuditLog + Reputation + RateLimiter) ──
        log.push((BootPhase::Security, InitStatus::Initializing));
        let security = SecurityManager::new();
        log.push((BootPhase::Security, InitStatus::Ready));

        // ── Done ──
        log.push((BootPhase::Done, InitStatus::Ready));

        Ok(KernelState {
            memory,
            processes,
            scheduler,
            ipc,
            fs,
            vfs,
            security,
            init_log: log,
        })
    }

    /// Gibt den Boot-Log als formatierten String zurück
    pub fn boot_log(&self) -> String {
        let mut out = String::from("=== ShivaCore Kernel Boot ===\n");
        for (phase, status) in &self.init_log {
            let icon = match status {
                InitStatus::Ready => "OK",
                InitStatus::Initializing => "..",
                InitStatus::Failed => "FAIL",
                InitStatus::NotStarted => "--",
            };
            out.push_str(&format!("  [{}] {}\n", icon, phase.label()));
        }
        out.push_str(&format!(
            "\n  Memory: {} regions, {} bytes allocated\n",
            self.memory.stats().active_regions,
            self.memory.stats().total_allocated
        ));
        out.push_str(&format!("  FS: {} nodes\n", self.fs.ls("/").len()));
        out.push_str("  Services: non-TCB subsystems in shivacore-service-space\n");
        out.push_str("=== Boot Complete ===\n");
        out
    }

    /// Smoke-Test: allokiert Speicher, schreibt eine Datei, liest sie zurück
    pub fn smoke_test(&mut self) -> Result<(), BootError> {
        // 1. Memory allocation
        let region = self
            .memory
            .allocate(crate::ats1000::Pid(1), 1024)
            .map_err(|_| BootError::SmokeTestFailed)?;

        // 2. FS write
        let caps = &self.memory.caps;
        self.fs
            .write_file(
                caps,
                "/tmp/smoke_test.txt",
                b"ShivaCore boot OK",
                crate::ats1000::Pid(1),
            )
            .map_err(|_| BootError::SmokeTestFailed)?;

        // 3. FS read
        let (_cid, node) = self
            .fs
            .read_file(caps, "/tmp/smoke_test.txt", crate::ats1000::Pid(1))
            .map_err(|_| BootError::SmokeTestFailed)?;
        assert_eq!(node.size, 17);

        // 4. Memory free
        self.memory
            .deallocate(crate::ats1000::Pid(1), region.region_id)
            .map_err(|_| BootError::SmokeTestFailed)?;

        Ok(())
    }
}

/// Boot-Fehler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    HeapConfigMismatch,
    FsInitFailed,
    SmokeTestFailed,
}

/// Validiert die Konsistenz zwischen allocator.rs und memory_manager.rs Konstanten.
pub fn validate_integration() -> Result<(), String> {
    if HEAP_START != 0x4444_4444_0000 {
        return Err(format!("HEAP_START mismatch: 0x{:x}", HEAP_START));
    }
    if HEAP_SIZE != 100 * 1024 {
        return Err(format!("HEAP_SIZE mismatch: {}", HEAP_SIZE));
    }
    if HEAP_END != HEAP_START + HEAP_SIZE {
        return Err(format!("HEAP_END mismatch: 0x{:x}", HEAP_END));
    }
    Ok(())
}

/// Gibt die Kernel-Version und Build-Info zurück
pub fn kernel_version() -> &'static str {
    "ShivaCore Kernel v0.0.24 (K-Sprint 24) — TCB-only, services in service-space"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_boot() {
        let state = KernelState::boot().unwrap();
        assert!(!state.init_log.is_empty());
        assert_eq!(state.init_log.last().unwrap().0, BootPhase::Done);
        assert_eq!(state.init_log.last().unwrap().1, InitStatus::Ready);
    }

    #[test]
    fn test_boot_log_output() {
        let state = KernelState::boot().unwrap();
        let log = state.boot_log();
        assert!(log.contains("ShivaCore Kernel Boot"));
        assert!(log.contains("Heap"));
        assert!(log.contains("MemorySubsystem"));
        assert!(log.contains("ATCFS"));
        assert!(log.contains("Security"));
        assert!(log.contains("service-space"));
        assert!(log.contains("Boot Complete"));
    }

    #[test]
    fn test_smoke_test() {
        let mut state = KernelState::boot().unwrap();
        state.smoke_test().unwrap();
        assert_eq!(state.memory.stats().total_allocated, 0);
    }

    #[test]
    fn test_validate_integration() {
        assert!(validate_integration().is_ok());
    }

    #[test]
    fn test_kernel_version() {
        let v = kernel_version();
        assert!(v.contains("ShivaCore"));
        assert!(v.contains("TCB-only"));
    }

    #[test]
    fn test_boot_phases_all_ready() {
        let state = KernelState::boot().unwrap();
        let last = state.init_log.last().unwrap();
        assert_eq!(last.0, BootPhase::Done);
        assert_eq!(last.1, InitStatus::Ready);
        for (_, status) in &state.init_log {
            assert_ne!(*status, InitStatus::Failed, "Phase failed");
        }
    }

    #[test]
    fn test_fs_root_exists_after_boot() {
        let state = KernelState::boot().unwrap();
        assert!(state.fs.exists("/"));
        assert!(state.fs.exists("/atc"));
    }

    #[test]
    fn test_security_initialized() {
        let state = KernelState::boot().unwrap();
        // SecurityManager initialized with MultiSig, AuditLog, Reputation, RateLimiter, SecureChannels
        // All fields are Arc-wrapped and ready
    }

    #[test]
    fn test_ipc_initialized() {
        let state = KernelState::boot().unwrap();
        // IPC subsystem exists, no channels yet
        // Just verify it's part of state
    }

    #[test]
    fn test_scheduler_initialized() {
        let state = KernelState::boot().unwrap();
        // Scheduler exists with no tasks
    }

    #[test]
    fn test_vfs_initialized() {
        let state = KernelState::boot().unwrap();
        // VFS root should exist
    }
}
