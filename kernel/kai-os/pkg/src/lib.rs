//! KAI-OS Package Manager (S23-S24, Issue #110).
//! Deterministisch und fail-closed: höchste passende Version gewinnt
//! (unabhängig von Publish-Reihenfolge), Zyklen und fehlende Dependencies
//! werden abgewiesen, Lockfiles sind byte-deterministisch und SHA-256-verifizierbar.

pub mod lockfile;
pub mod manifest;
pub mod registry;
pub mod version;

pub fn sha_hex(input: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let d = Sha256::digest(input);
    d.iter().map(|b| format!("{b:02x}")).collect()
}
