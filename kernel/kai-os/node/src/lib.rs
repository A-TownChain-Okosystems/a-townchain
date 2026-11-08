//! KAI-OS Node Daemon — Orchestrierungsebene, nicht Implementierungsort.
//! (KAI-CORE-RUNTIME-001 §2, Issue #102, Sprint S01-S03)
//!
//! Ausführungskette: Policy -> Sandbox -> Capability -> Execution.
//! Der Daemon startet KEINE Subsysteme mit Root-Rechten (fail-closed).

pub mod audit;
pub mod config;
pub mod health;
pub mod lifecycle;
pub mod shutdown;
pub mod supervisor;

/// Fail-Closed-Root-Guard (Issue #102: kein Subsystem mit Root-Rechten).
/// `allow_root=false` (Default) -> euid 0 verweigert den Boot.
pub fn ensure_not_root(allow_root: bool, euid: u32) -> Result<(), String> {
    if euid == 0 && !allow_root {
        Err("REFUSING BOOT: running as root (euid=0) and allow_root=false (fail-closed, KAI-CORE-RUNTIME-001 §3)".into())
    } else {
        Ok(())
    }
}
