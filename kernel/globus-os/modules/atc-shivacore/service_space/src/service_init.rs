// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Service-Space Init (P0 #45 Architecture Cleanup, AD-012/AD-028).
//!
//! Initialisiert die Nicht-TCB-Subsysteme, die aus dem ShivaCore-Kernel
//! ausgelagert wurden:
//!   - P2P-Netz (net/tcpip/p2p/p2p_secure/sockets)
//!   - Blockchain-Stack (mempool + vm + contract)
//!   - AI-Subsystem (ai)
//!
//! Der Kernel (shivacore) initialisiert nur noch den TCB
//! (kernel_init::KernelState). Diese Schicht baut auf dem Kernel auf,
//! niemals umgekehrt.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::ai::AiEngine;
use crate::contract::ContractExecutor;
use crate::mempool::{MemoryPool, NonceTracker, StateDb, TxValidator};
use crate::p2p::P2pNode;
use crate::vm::VmEngine;

/// Init-Status pro Service-Subsystem
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceInitStatus {
    NotStarted,
    Initializing,
    Ready,
    Failed,
}

/// Service-Boot-Phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServicePhase {
    Network,   // P2P-Netz
    Mempool,   // Transaction Mempool + State-DB
    Contracts, // Contract-VM + Executor
    Ai,        // AI-Subsystem
    Done,
}

impl ServicePhase {
    pub fn label(&self) -> &str {
        match self {
            ServicePhase::Network => "P2P Network (ATCNet)",
            ServicePhase::Mempool => "Transaction Mempool",
            ServicePhase::Contracts => "Contract VM + Executor",
            ServicePhase::Ai => "AI Subsystem (Aurora AI)",
            ServicePhase::Done => "Service Boot Complete",
        }
    }
}

/// Das vereinigte Service-Space-State-Objekt.
/// Enthält alle Nicht-TCB-Subsysteme (ursprünglich KernelState L7/L9/L10).
pub struct ServiceSpaceState {
    pub p2p: P2pNode,
    pub mempool: Arc<MemoryPool>,
    pub state_db: Arc<StateDb>,
    pub tx_validator: Arc<TxValidator>,
    pub nonces: Arc<NonceTracker>,
    pub vm: Arc<VmEngine>,
    pub contracts: ContractExecutor,
    pub ai: AiEngine,
    pub init_log: Vec<(ServicePhase, ServiceInitStatus)>,
}

impl ServiceSpaceState {
    /// Service-Space-Init-Sequenz — initialisiert alle Nicht-TCB-Subsysteme.
    ///
    /// Muss NACH dem Kernel-Boot (shivacore::kernel_init::KernelState::boot)
    /// aufgerufen werden, da die Services auf dem Kernel aufbauen.
    pub fn init() -> Result<Self, ServiceBootError> {
        let mut log = Vec::new();

        // ── P2P Network ──
        log.push((ServicePhase::Network, ServiceInitStatus::Initializing));
        let our_did = "did:atc:shivacore:bootnode".to_string();
        let p2p = P2pNode::new(our_did.clone(), 4242, 50);
        log.push((ServicePhase::Network, ServiceInitStatus::Ready));

        // ── Transaction Mempool + State-DB ──
        log.push((ServicePhase::Mempool, ServiceInitStatus::Initializing));
        let mempool = Arc::new(MemoryPool::new(10000, 300));
        let state_db = Arc::new(StateDb::new());
        let nonces = Arc::new(NonceTracker::new());
        let tx_validator = Arc::new(TxValidator::new(state_db.clone(), nonces.clone(), 1));
        log.push((ServicePhase::Mempool, ServiceInitStatus::Ready));

        // ── Contract-VM + Executor ──
        log.push((ServicePhase::Contracts, ServiceInitStatus::Initializing));
        let vm = Arc::new(VmEngine::new(1_000_000));
        let contracts = ContractExecutor::new(vm.clone(), state_db.clone());
        log.push((ServicePhase::Contracts, ServiceInitStatus::Ready));

        // ── AI Subsystem ──
        log.push((ServicePhase::Ai, ServiceInitStatus::Initializing));
        let ai = AiEngine::new();
        log.push((ServicePhase::Ai, ServiceInitStatus::Ready));

        // ── Done ──
        log.push((ServicePhase::Done, ServiceInitStatus::Ready));

        Ok(ServiceSpaceState {
            p2p,
            mempool,
            state_db,
            tx_validator,
            nonces,
            vm,
            contracts,
            ai,
            init_log: log,
        })
    }

    /// Gibt den Service-Boot-Log als formatierten String zurück
    pub fn boot_log(&self) -> String {
        let mut out = String::from("=== ShivaCore Service-Space Boot ===\n");
        for (phase, status) in &self.init_log {
            let icon = match status {
                ServiceInitStatus::Ready => "OK",
                ServiceInitStatus::Initializing => "..",
                ServiceInitStatus::Failed => "FAIL",
                ServiceInitStatus::NotStarted => "--",
            };
            out.push_str(&format!("  [{}] {}\n", icon, phase.label()));
        }
        out.push_str(&format!(
            "  P2P: port {}, {} peers\n",
            self.p2p.listen_port(),
            self.p2p.peer_count()
        ));
        out.push_str(&format!(
            "  Mempool: {}/{} txs\n",
            self.mempool.count(),
            10000
        ));
        out.push_str(&format!("  VM: {} contracts\n", self.vm.contract_count()));
        out.push_str(&format!("  AI: {} models\n", self.ai.model_count()));
        out.push_str("=== Service Boot Complete ===\n");
        out
    }
}

/// Service-Boot-Fehler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceBootError {
    NetworkInitFailed,
    MempoolInitFailed,
    ContractInitFailed,
    AiInitFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_boot() {
        let state = ServiceSpaceState::init().unwrap();
        assert!(!state.init_log.is_empty());
        assert_eq!(state.init_log.last().unwrap().0, ServicePhase::Done);
        assert_eq!(state.init_log.last().unwrap().1, ServiceInitStatus::Ready);
    }

    #[test]
    fn test_boot_log_output() {
        let state = ServiceSpaceState::init().unwrap();
        let log = state.boot_log();
        assert!(log.contains("Service-Space Boot"));
        assert!(log.contains("P2P"));
        assert!(log.contains("Mempool"));
        assert!(log.contains("VM"));
        assert!(log.contains("AI"));
        assert!(log.contains("Service Boot Complete"));
    }

    #[test]
    fn test_p2p_initialized() {
        let state = ServiceSpaceState::init().unwrap();
        assert_eq!(state.p2p.listen_port(), 4242);
        assert_eq!(state.p2p.peer_count(), 0);
    }

    #[test]
    fn test_contracts_stack_initialized() {
        let state = ServiceSpaceState::init().unwrap();
        assert_eq!(state.mempool.count(), 0);
        assert_eq!(state.vm.contract_count(), 0);
    }

    #[test]
    fn test_ai_initialized() {
        let state = ServiceSpaceState::init().unwrap();
        assert_eq!(state.ai.model_count(), 0);
    }

    #[test]
    fn test_all_phases_ready() {
        let state = ServiceSpaceState::init().unwrap();
        for (_, status) in &state.init_log {
            assert_ne!(
                *status,
                ServiceInitStatus::Failed,
                "Service-Phase fehlgeschlagen"
            );
        }
    }
}
