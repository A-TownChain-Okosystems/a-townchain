//! Hardware/kernel key-provider boundary for identity storage.
//!
//! This module intentionally does **not** contain a key, derive a key from disk state, or
//! emulate TPM/TEE behavior. `ShivaCoreKeyProvider` is an adapter around an explicitly supplied
//! kernel/secure-element service. Production wiring must provide that service from a trusted
//! boundary (ShivaCore, TPM, TEE, or equivalent).

use zeroize::Zeroizing;

use crate::secure_store::{IdentityKeyProvider, SecureStoreError};

/// Stable identifier for a hardware/kernel-protected identity key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SecureKeyId(pub u64);

/// Narrow interface that the trusted kernel/secure element must expose to GlobusOS.
///
/// Implementations must never return key material backed by ordinary persistent application
/// storage. The returned value is zeroizing and must only live for the duration of the crypto
/// operation that needs it.
pub trait SecureKeyService {
    fn load_identity_key(
        &self,
        key_id: SecureKeyId,
    ) -> Result<Zeroizing<[u8; 32]>, SecureKeyServiceError>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureKeyServiceError {
    Unavailable,
    AccessDenied,
    InvalidKey,
    HardwareFailure,
}

/// `IdentityKeyProvider` adapter for a ShivaCore/TPM/TEE-backed key service.
///
/// This type contains only the key handle and service reference. It deliberately cannot be
/// constructed from raw key bytes, preventing application-level configuration from becoming an
/// accidental key source.
pub struct ShivaCoreKeyProvider<S> {
    service: S,
    key_id: SecureKeyId,
}

impl<S> ShivaCoreKeyProvider<S> {
    pub fn new(service: S, key_id: SecureKeyId) -> Self {
        Self { service, key_id }
    }

    pub fn key_id(&self) -> SecureKeyId {
        self.key_id
    }

    pub fn into_service(self) -> S {
        self.service
    }
}

impl<S: SecureKeyService> IdentityKeyProvider for ShivaCoreKeyProvider<S> {
    fn load_key(&self) -> Result<Zeroizing<[u8; 32]>, SecureStoreError> {
        self.service
            .load_identity_key(self.key_id)
            .map_err(|_| SecureStoreError::KeyUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSecureService;

    impl SecureKeyService for TestSecureService {
        fn load_identity_key(
            &self,
            key_id: SecureKeyId,
        ) -> Result<Zeroizing<[u8; 32]>, SecureKeyServiceError> {
            if key_id == SecureKeyId(7) {
                Ok(Zeroizing::new([7u8; 32]))
            } else {
                Err(SecureKeyServiceError::AccessDenied)
            }
        }
    }

    #[test]
    fn provider_uses_only_the_authorized_key_handle() {
        let provider = ShivaCoreKeyProvider::new(TestSecureService, SecureKeyId(7));
        assert_eq!(provider.key_id(), SecureKeyId(7));
        assert_eq!(&*provider.load_key().unwrap(), &[7u8; 32]);
    }

    #[test]
    fn provider_maps_secure_service_failure_to_key_unavailable() {
        let provider = ShivaCoreKeyProvider::new(TestSecureService, SecureKeyId(8));
        assert_eq!(
            provider.load_key().unwrap_err(),
            SecureStoreError::KeyUnavailable
        );
    }
}
