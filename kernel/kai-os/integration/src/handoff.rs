//! Handoff-Signatur der Integration (SEC-P0-006, sichere Variante).
//!
//! **Kurve:** secp256k1/ECDSA mit RFC-6979-Nonce und Low-S, wie in
//! WAL-SIGN-001 fuer autoritaetstragende Signaturen vorgeschrieben. Ed25519
//! bleibt ausschliesslich im Transport-/Peer-Pfad (ATC-STD-PROTOCOL-003) und
//! ist fuer den Handoff ausdruecklich NICHT vorgesehen.
//!
//! **Kanonisierung:** Domain-Tag, Chain-ID und laengenpraefigierte Proposal-ID.
//! Damit ist eine Handoff-Signatur weder als Transaktions-Signatur noch gegen
//! eine andere Chain oder eine rohe ID wiederverwendbar.

use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

/// Domain-Tag der Handoff-Signatur.
pub const HANDOFF_DOMAIN: &[u8] = b"ATC-KAI-HANDOFF-V1";
/// Kanonische ATC-Chain-ID (identisch zu Wallet/Validator).
pub const HANDOFF_CHAIN_ID: u64 = 658_467;

/// Kanonische, domain-separierte Handoff-Nachricht.
pub fn handoff_message(chain_id: u64, proposal_id: &str) -> Vec<u8> {
    let mut b = Vec::with_capacity(HANDOFF_DOMAIN.len() + 8 + 4 + proposal_id.len());
    b.extend_from_slice(HANDOFF_DOMAIN);
    b.extend_from_slice(&chain_id.to_be_bytes());
    b.extend_from_slice(&(proposal_id.len() as u32).to_be_bytes());
    b.extend_from_slice(proposal_id.as_bytes());
    b
}

/// secp256k1-Schluessel fuer Handoff-Signaturen (Determinismus aus Seed).
pub struct HandoffKey {
    signing: SecretKey,
}

impl HandoffKey {
    pub fn from_seed(seed: &[u8; 32]) -> Result<Self, String> {
        SecretKey::from_byte_array(*seed)
            .map(|signing| Self { signing })
            .map_err(|_| "ungueltiger Seed fuer secp256k1".to_string())
    }

    pub fn public_key(&self) -> [u8; 33] {
        PublicKey::from_secret_key(&Secp256k1::new(), &self.signing).serialize()
    }

    /// Signiert SHA-256(message) mit ECDSA (RFC 6979, Low-S).
    pub fn sign(&self, message: &[u8]) -> [u8; 64] {
        let digest: [u8; 32] = Sha256::digest(message).into();
        Secp256k1::new()
            .sign_ecdsa(Message::from_digest(digest), &self.signing)
            .serialize_compact()
    }
}

/// Oeffentliche Verifikation: Signatur muss gueltig und Low-S sein.
pub fn verify_handoff(public: &[u8; 33], message: &[u8], signature: &[u8; 64]) -> bool {
    let pk = match PublicKey::from_slice(public) {
        Ok(pk) => pk,
        Err(_) => return false,
    };
    let sig = match Signature::from_compact(signature) {
        Ok(sig) => sig,
        Err(_) => return false,
    };
    let mut normalized = sig;
    normalized.normalize_s();
    if normalized != sig {
        return false; // High-S / Malleabilität ablehnen
    }
    let digest: [u8; 32] = Sha256::digest(message).into();
    Secp256k1::verification_only()
        .verify_ecdsa(Message::from_digest(digest), &sig, &pk)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signatur_roundtrip_und_ablehnungen() {
        let key = HandoffKey::from_seed(&[11u8; 32]).unwrap();
        let pubkey = key.public_key();
        let msg = handoff_message(HANDOFF_CHAIN_ID, "proposal-1");
        let sig = key.sign(&msg);
        assert!(verify_handoff(&pubkey, &msg, &sig));
        assert!(!verify_handoff(&pubkey, b"andere nachricht", &sig));
        assert!(!verify_handoff(&pubkey, &handoff_message(1, "proposal-1"), &sig));
        assert!(!verify_handoff(&pubkey, &handoff_message(HANDOFF_CHAIN_ID, "proposal-2"), &sig));
        let mut broken = sig;
        broken[0] ^= 0xff;
        assert!(!verify_handoff(&pubkey, &msg, &broken));
    }

    #[test]
    fn nachricht_ist_domain_separiert_und_kettengebunden() {
        let m = handoff_message(HANDOFF_CHAIN_ID, "p");
        assert!(m.starts_with(HANDOFF_DOMAIN));
        assert_ne!(handoff_message(HANDOFF_CHAIN_ID, "p"), handoff_message(1u64, "p"));
        assert_ne!(handoff_message(HANDOFF_CHAIN_ID, "a"), handoff_message(HANDOFF_CHAIN_ID, "b"));
    }
}
