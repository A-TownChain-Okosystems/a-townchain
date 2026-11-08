//! Local wallet keystore boundary.
//!
//! This module intentionally defines the contract before selecting a platform-specific
//! encrypted storage implementation. Secret bytes must never be persisted through the
//! generic account/profile layer.

use zeroize::Zeroizing;

/// Stable identifier for a locally protected wallet record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyId(String);

impl KeyId {
    pub fn new(value: impl Into<String>) -> Result<Self, KeystoreError> {
        let value = value.into();
        if value.is_empty() || value.len() > 128 {
            return Err(KeystoreError::InvalidKeyId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Metadata that may be persisted without secret material.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyMetadata {
    pub key_id: KeyId,
    pub algorithm: String,
    pub version: u16,
    pub hardware_backed: bool,
}

/// A protected private-key handle. The actual key bytes remain inside the keystore.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedKey {
    pub metadata: KeyMetadata,
}

/// One-shot signing request. The keystore owns the private key and returns only a signature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignRequest {
    pub key_id: KeyId,
    pub domain: String,
    pub message: Vec<u8>,
}

/// Signature result returned from a protected signing backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Signature {
    pub algorithm: String,
    pub bytes: Vec<u8>,
}

/// Platform keystore interface.
///
/// Production implementations should map this to TPM/TEE/OS credential facilities where
/// available. A backend must reject unauthorized access and must never export private keys.
pub trait KeyStore {
    fn provision(
        &mut self,
        key_id: KeyId,
        secret: Zeroizing<Vec<u8>>,
        algorithm: String,
    ) -> Result<ProtectedKey, KeystoreError>;
    fn sign(&self, request: SignRequest) -> Result<Signature, KeystoreError>;
    fn delete(&mut self, key_id: &KeyId) -> Result<(), KeystoreError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeystoreError {
    InvalidKeyId,
    Unauthorized,
    NotFound,
    InvalidRequest,
    Unsupported,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_id_is_validated() {
        assert_eq!(KeyId::new("").unwrap_err(), KeystoreError::InvalidKeyId);
        assert_eq!(
            KeyId::new("wallet-primary").unwrap().as_str(),
            "wallet-primary"
        );
    }
}
