//! KAI-OS State — Sync + Snapshots + Merkle Verification (S10-S12, Issue #105).
//!
//! Kernprinzip (KAI-CORE-RUNTIME-001 §6):
//! **Ein Snapshot wird NIEMALS vertraut, sondern gegen einen kryptographisch
//! definierten Chain-/State-Root verifiziert.**
//!
//! Diese Schicht ist Verifikations-INFRASTRUKTUR (AD-008.4); die kanonische
//! State-Semantik bleibt ATCLang/ATVM vorbehalten.

pub mod merkle;
pub mod persistence;
pub mod snapshot;
pub mod state;
pub mod sync;
pub mod wal;

pub fn sha_hex(input: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let d = Sha256::digest(input);
    d.iter().map(|b| format!("{b:02x}")).collect()
}
