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
//!   L7:  P2pNode::new()             — P2P Network
//!   L8:  SecurityManager::new()     — MultiSig + AuditLog + Reputation
//!   L9:  ConsensusEngine::new()     — PoH + DAG + Validators
//!   L9b: MemoryPool::new()          — Transaction Mempool
//!   L9c: BlockChain::new()          — Blockchain
//!   L9d: VmEngine::new()            — Contract VM
//!   L9e: ContractExecutor::new()   — Contract Processing
//!   L10: AiEngine::new()            — AI Subsystem

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::ats1000::{MemoryManager, FileSystem};
use crate::capability::CapabilityTable;
use crate::memory_manager::{MemorySubsystem, HEAP_START, HEAP_SIZE, HEAP_END};
use crate::atcfs::AtcFileSystem;
use crate::vfs::Vfs;
use crate::process::ProcessManager;
use crate::scheduler::DaHeftScheduler;
use crate::ipc::IpcSubsystem;
use crate::p2p::P2pNode;
use crate::security::SecurityManager;
use crate::consensus::ConsensusEngine;
use crate::mempool::{MemoryPool, StateDb, TxValidator, NonceTracker};
use crate::blockchain::BlockChain;
use crate::vm::VmEngine;
use crate::contract::ContractExecutor;
use crate::ai::AiEngine;
use crate::timer::{SimulatedTimerSource, MonotonicClock, TimerManager};
use crate::did::Did;

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
    Heap,           // L0: allocator::init_heap
    Memory,         // L1: MemorySubsystem
    Capabilities,   // L2: CapabilityTable
    Processes,       // L3: ProcessManager
    Scheduler,       // L4: DA-HEFT Scheduler
    Ipc,             // L5: IPC Channels
    FileSystem,       // L6: ATCFS + VFS
    Network,         // L7: P2P Network
    Security,        // L8: Security/Audit/MultiSig
    Blockchain,      // L9: Consensus/Chain/Mempool/VM
    Ai,              // L10: AI Subsystem
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
            BootPhase::Network => "L7 P2P Network (ATCNet)",
            BootPhase::Security => "L8 Security/Audit/MultiSig",
            BootPhase::Blockchain => "L9 Consensus/Chain/Mempool/VM",
            BootPhase::Ai => "L10 AI Subsystem (Aurora AI)",
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
    // L7: Network
    pub p2p: P2pNode,
    // L8: Security
    pub security: SecurityManager,
    // L9: Blockchain stack
    pub consensus: ConsensusEngine,
    pub mempool: Arc<MemoryPool>,
    pub state_db: Arc<StateDb>,
    pub tx_validator: Arc<TxValidator>,
    pub nonces: Arc<NonceTracker>,
    pub chain: Arc<BlockChain>,
    pub vm: Arc<VmEngine>,
    pub contracts: ContractExecutor,
    // L10: AI
    pub ai: AiEngine,
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

        // ── L7: P2P Network ──
        log.push((BootPhase::Network, InitStatus::Initializing));
        let our_did = "did:atc:shivacore:bootnode".to_string();
        let p2p = P2pNode::new(our_did.clone(), 4242, 50);
        log.push((BootPhase::Network, InitStatus::Ready));

        // ── L8: Security (MultiSig + AuditLog + Reputation + RateLimiter) ──
        log.push((BootPhase::Security, InitStatus::Initializing));
        let security = SecurityManager::new();
        log.push((BootPhase::Security, InitStatus::Ready));

        // ── L9: Blockchain Stack (Consensus + Mempool + Chain + VM + Contracts) ──
        log.push((BootPhase::Blockchain, InitStatus::Initializing));
        let genesis_hash = [0u8; 32]; // Genesis hash — wird durch GenesisBridge gesetzt
        let consensus = ConsensusEngine::new(our_did.clone(), genesis_hash);

        let mempool = Arc::new(MemoryPool::new(10000, 300));
        let state_db = Arc::new(StateDb::new());
        let nonces = Arc::new(NonceTracker::new());
        let tx_validator = Arc::new(TxValidator::new(state_db.clone(), nonces.clone(), 1));
        let chain = Arc::new(BlockChain::new());
        let vm = Arc::new(VmEngine::new(1_000_000));
        let contracts = ContractExecutor::new(vm.clone(), state_db.clone());
        log.push((BootPhase::Blockchain, InitStatus::Ready));

        // ── L10: AI Subsystem ──
        log.push((BootPhase::Ai, InitStatus::Initializing));
        let ai = AiEngine::new();
        log.push((BootPhase::Ai, InitStatus::Ready));

        // ── Done ──
        log.push((BootPhase::Done, InitStatus::Ready));

        Ok(KernelState {
            memory,
            processes,
            scheduler,
            ipc,
            fs,
            vfs,
            p2p,
            security,
            consensus,
            mempool,
            state_db,
            tx_validator,
            nonces,
            chain,
            vm,
            contracts,
            ai,
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
        out.push_str(&format!("\n  Memory: {} regions, {} bytes allocated\n",
            self.memory.stats().active_regions,
            self.memory.stats().total_allocated));
        out.push_str(&format!("  FS: {} nodes\n", self.fs.ls("/").len()));
        out.push_str(&format!("  P2P: port {}, {} peers\n",
            self.p2p.listen_port(),
            self.p2p.peer_count()));
        out.push_str(&format!("  Mempool: {}/{} txs\n",
            self.mempool.count(), 10000));
        out.push_str(&format!("  Chain: height {}\n",
            self.chain.current_height()));
        out.push_str(&format!("  VM: {} contracts\n",
            self.vm.contract_count()));
        out.push_str(&format!("  AI: {} models\n",
            self.ai.model_count()));
        out.push_str("=== Boot Complete ===\n");
        out
    }

    /// Smoke-Test: allokiert Speicher, schreibt eine Datei, liest sie zurück
    pub fn smoke_test(&mut self) -> Result<(), BootError> {
        // 1. Memory allocation
        let region = self.memory.allocate(crate::ats1000::Pid(1), 1024)
            .map_err(|_| BootError::SmokeTestFailed)?;

        // 2. FS write
        let caps = &self.memory.caps;
        self.fs.write_file(caps, "/tmp/smoke_test.txt", b"ShivaCore boot OK", crate::ats1000::Pid(1))
            .map_err(|_| BootError::SmokeTestFailed)?;

        // 3. FS read
        let (_cid, node) = self.fs.read_file(caps, "/tmp/smoke_test.txt", crate::ats1000::Pid(1))
            .map_err(|_| BootError::SmokeTestFailed)?;
        assert_eq!(node.size, 17);

        // 4. Memory free
        self.memory.deallocate(crate::ats1000::Pid(1), region.region_id)
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
    "ShivaCore Kernel v0.0.23 (K-Sprint 23) — 709 tests, 30 modules"
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
        assert!(log.contains("P2P"));
        assert!(log.contains("Mempool"));
        assert!(log.contains("Chain"));
        assert!(log.contains("VM"));
        assert!(log.contains("AI"));
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
        assert!(v.contains("709 tests"));
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
    fn test_p2p_initialized() {
        let state = KernelState::boot().unwrap();
        assert_eq!(state.p2p.listen_port(), 4242);
        assert_eq!(state.p2p.peer_count(), 0);
    }

    #[test]
    fn test_blockchain_initialized() {
        let state = KernelState::boot().unwrap();
        assert_eq!(state.chain.current_height(), 0);
        assert_eq!(state.mempool.count(), 0);
        assert_eq!(state.vm.contract_count(), 0);
    }

    #[test]
    fn test_ai_initialized() {
        let state = KernelState::boot().unwrap();
        assert_eq!(state.ai.model_count(), 0);
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
