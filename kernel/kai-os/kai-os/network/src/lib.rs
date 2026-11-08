//! KAI-OS Network — P2P Transport + Discovery + Peer Security (S07-S09, Issue #104).
//!
//! Kette (KAI-CORE-RUNTIME-001 §5):
//! Peer Discovery -> Secure Transport -> Peer Authentication -> Consensus Networking.
//!
//! Diese Schicht ist Consensus-INFRASTRUKTUR (AD-008.4), keine Semantik.
//! Ed25519-Signaturen sind per Spezifikation deterministisch — kein RNG (REQ-ENG-002).

pub mod auth;
pub mod discovery;
pub mod peer;
pub mod session;
pub mod tcp;
pub mod transport;
pub mod upgrade;

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
