//! KAI-OS Runtime — Resource Manager + Sandbox + Scheduler (S04–S06, Issue #103).
//!
//! Verbindliche Ausführungskette (KAI-CORE-RUNTIME-001 §3):
//! **Policy -> Sandbox -> Capability -> Execution**
//!
//! "KI erkennt -> Policy entscheidet -> System führt aus."
//! Diese Schicht implementiert Infrastruktur (AD-008.4), keine Consensus-Semantik.

pub mod capability;
pub mod resources;
pub mod sandbox;
pub mod scheduler;

pub type SandboxId = String;
