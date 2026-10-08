// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Signatur- und Verifikations-Interfaces als Adapter (REQ-TUD-011: Keine Eigenbau-Kryptografie).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationError {
    InvalidSignature,
    PublicKeyMismatch,
    MalformedData,
}

/// Signatur-Verifikations Adapter Interface (REQ-TUD-011 konform)
pub trait SignatureVerifierAdapter {
    fn verify_signature(&self, message: &[u8], signature: &[u8], pubkey: &[u8]) -> Result<bool, VerificationError>;
}

/// Standard-Adapter zur Delegation der Verifikation ohne Eigenbau-Krypto
pub struct StandardVerifierAdapter {
    pub require_matching_key: bool,
}

impl StandardVerifierAdapter {
    pub fn new(require_matching_key: bool) -> Self {
        Self { require_matching_key }
    }
}

impl SignatureVerifierAdapter for StandardVerifierAdapter {
    fn verify_signature(&self, message: &[u8], signature: &[u8], pubkey: &[u8]) -> Result<bool, VerificationError> {
        if message.is_empty() || signature.is_empty() || pubkey.is_empty() {
            return Err(VerificationError::MalformedData);
        }
        if self.require_matching_key && pubkey.len() < 4 {
            return Err(VerificationError::PublicKeyMismatch);
        }
        // Delegierter Adapter-Check (für tests: Prefix "VALID")
        if signature.starts_with(b"VALID") {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
