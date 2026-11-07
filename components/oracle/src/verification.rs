// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Signatur-Verifikation über etablierte Krypto-Bibliotheken (REQ-TUD-011:
//! keine Eigenbau-Kryptografie).
//!
//! Kurven-Zuordnung ist normativ, nicht beliebig:
//! - **Ed25519** für externe, herkunfts-signierte Daten (ATC-STD-270..276, KR-1).
//! - **secp256k1** für Transaktions-Signaturen (WAL-SIGN-001, RFC 6979, Low-S).
//!
//! Es gibt in diesem Modul keinen akzeptierenden Mock. Ein Verifier, der
//! Signaturen ohne kryptografische Prüfung annimmt, gehört ausschließlich in
//! Test-Code (siehe tests/oracle_new_tests.rs), nie in den Produktionspfad.

use ed25519_dalek::{Signature as EdSignature, VerifyingKey as EdVerifyingKey};
use secp256k1::{ecdsa::Signature as SecpSignature, Message as SecpMessage, PublicKey as SecpPublicKey, Secp256k1};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    InvalidSignature,
    PublicKeyMismatch,
    MalformedData,
    UnsupportedAlgorithm,
}

/// Signatur-Verifikations Adapter Interface (REQ-TUD-011 konform)
pub trait SignatureVerifierAdapter {
    fn verify_signature(&self, message: &[u8], signature: &[u8], pubkey: &[u8]) -> Result<bool, VerificationError>;
}

/// Ed25519-Verifier — normative Kurve für externe, herkunfts-signierte Daten.
///
/// Erwartet: 32-Byte-Pubkey, 64-Byte-Signatur. Verwendet `verify_strict`
/// (lehnt Small-Order-/Non-Canonical-Komponenten ab).
pub struct Ed25519VerifierAdapter;

impl SignatureVerifierAdapter for Ed25519VerifierAdapter {
    fn verify_signature(&self, message: &[u8], signature: &[u8], pubkey: &[u8]) -> Result<bool, VerificationError> {
        if message.is_empty() || signature.is_empty() || pubkey.is_empty() {
            return Err(VerificationError::MalformedData);
        }
        let pk: [u8; 32] = pubkey.try_into().map_err(|_| VerificationError::PublicKeyMismatch)?;
        let vk = EdVerifyingKey::from_bytes(&pk).map_err(|_| VerificationError::PublicKeyMismatch)?;
        let sig = EdSignature::from_slice(signature).map_err(|_| VerificationError::MalformedData)?;
        match vk.verify_strict(message, &sig) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

/// secp256k1-Verifier — normative Kurve für Transaktions-Signaturen (WAL-SIGN-001).
///
/// Erwartet: 33-Byte-Pubkey (compressed), 64-Byte-ECDSA-Signatur (compact,
/// Low-S). Signiert wird SHA-256(message); es findet keine weitere Vor-Hashung
/// statt, damit der Aufrufer den Digest-Vertrag kontrolliert.
pub struct Secp256k1VerifierAdapter;

impl SignatureVerifierAdapter for Secp256k1VerifierAdapter {
    fn verify_signature(&self, message: &[u8], signature: &[u8], pubkey: &[u8]) -> Result<bool, VerificationError> {
        if message.is_empty() || signature.is_empty() || pubkey.is_empty() {
            return Err(VerificationError::MalformedData);
        }
        let sig_bytes: [u8; 64] = signature.try_into().map_err(|_| VerificationError::MalformedData)?;
        let pubkey = SecpPublicKey::from_slice(pubkey).map_err(|_| VerificationError::PublicKeyMismatch)?;
        let sig = SecpSignature::from_compact(&sig_bytes).map_err(|_| VerificationError::MalformedData)?;
        let digest: [u8; 32] = Sha256::digest(message).into();
        let msg = SecpMessage::from_digest(digest);
        let secp = Secp256k1::verification_only();
        match secp.verify_ecdsa(msg, &sig, &pubkey) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
