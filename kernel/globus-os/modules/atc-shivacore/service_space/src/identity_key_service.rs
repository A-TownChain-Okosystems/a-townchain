//! Capability-gated identity key service boundary for GlobusOS.
//!
//! This module does not contain key material. A production backend must bind the opaque
//! `KeyHandle` to a TPM, TEE, secure element, or other hardware-backed key slot.

#![allow(dead_code)]

use alloc::vec::Vec;

extern crate alloc;

/// Opaque identifier for a key held by the secure key service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyHandle(pub u64);

/// Operations that may be requested from the identity key service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyOperation {
    LoadEncryptionKey,
}

/// Errors returned without exposing key material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityKeyError {
    InvalidHandle,
    NotAuthorized,
    KeyUnavailable,
    HardwareFailure,
}

/// Backend boundary. Implementations must never return a persistent raw key through IPC.
pub trait IdentityKeyBackend {
    /// Loads a transient 256-bit encryption key into the caller's protected memory.
    fn load_key(&self, handle: KeyHandle) -> Result<[u8; 32], IdentityKeyError>;
}

/// Capability-gated service facade.
pub struct IdentityKeyService<B> {
    backend: B,
}

impl<B> IdentityKeyService<B> {
    pub const fn new(backend: B) -> Self {
        Self { backend }
    }
}

impl<B: IdentityKeyBackend> IdentityKeyService<B> {
    /// Resolve a key handle after the caller has been authorized by the surrounding
    /// ShivaCore capability boundary.
    pub fn load_key(
        &self,
        handle: KeyHandle,
        authorized: bool,
    ) -> Result<[u8; 32], IdentityKeyError> {
        if !authorized {
            return Err(IdentityKeyError::NotAuthorized);
        }
        self.backend.load_key(handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestBackend;

    impl IdentityKeyBackend for TestBackend {
        fn load_key(&self, handle: KeyHandle) -> Result<[u8; 32], IdentityKeyError> {
            if handle.0 == 0 {
                return Err(IdentityKeyError::InvalidHandle);
            }
            Ok([0xA5; 32])
        }
    }

    #[test]
    fn authorization_is_required() {
        let service = IdentityKeyService::new(TestBackend);
        assert_eq!(
            service.load_key(KeyHandle(1), false),
            Err(IdentityKeyError::NotAuthorized)
        );
    }

    #[test]
    fn authorized_handle_is_resolved() {
        let service = IdentityKeyService::new(TestBackend);
        assert_eq!(service.load_key(KeyHandle(1), true).unwrap(), [0xA5; 32]);
    }

    #[test]
    fn invalid_handle_is_rejected() {
        let service = IdentityKeyService::new(TestBackend);
        assert_eq!(
            service.load_key(KeyHandle(0), true),
            Err(IdentityKeyError::InvalidHandle)
        );
    }
}
