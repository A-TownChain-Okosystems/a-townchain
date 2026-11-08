//! KAI-OS AI Runtime — IPC + Proposals + Audit Trail (S13-S15, Issue #106).
//!
//! Kernregel (AD-008 §7):
//! **AI may propose. ATCLang specifies. ATVM executes. ATC commits.**
//!
//! ARCHITEKTONISCHE GARANTIE: Diese Crate besitzt bewusst KEINE API, die
//! den kanonischen Chain-State verändert. Vorschläge werden registriert,
//! verifiziert und an die ATVM-Grenze übergeben — die Ausführung selbst
//! liegt außerhalb dieser Schicht (on-chain = ATCLang/ATVM, AD-008.1/.3).

pub mod audit;
pub mod ipc;
pub mod model_registry;
pub mod proposal;
pub mod verified_cache;

pub fn sha_hex(input: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let d = Sha256::digest(input);
    d.iter().map(|b| format!("{b:02x}")).collect()
}
