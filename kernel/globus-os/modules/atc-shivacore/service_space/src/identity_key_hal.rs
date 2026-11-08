//! Hardware-backed identity-key HAL boundary.
//!
//! This module deliberately contains no software key fallback and no persistent key
//! storage. Platform implementations must resolve an opaque key handle to a TPM, TEE,
//! or secure-element slot and keep key material inside the protected execution boundary.

#![allow(dead_code)]

/// Hardware key slot selected by the capability layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HardwareKeyHandle(pub u64);

/// Hardware-backed identity-key failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareKeyError {
    InvalidHandle,
    NotProvisioned,
    Unavailable,
    AccessDenied,
    HardwareFailure,
}

/// Minimal HAL required by the identity key service.
///
/// Implementations must not persist or export the key through ordinary storage or
/// configuration. The returned bytes are transient and must be zeroized by callers.
pub trait IdentityKeyHal {
    fn load_encryption_key(&self, handle: HardwareKeyHandle) -> Result<[u8; 32], HardwareKeyError>;
}

/// Explicit production backend marker. No software implementation is provided.
pub struct HardwareIdentityKeyBackend<H> {
    hal: H,
}

impl<H> HardwareIdentityKeyBackend<H> {
    pub const fn new(hal: H) -> Self {
        Self { hal }
    }
}

impl<H: IdentityKeyHal> super::identity_key_service::IdentityKeyBackend
    for HardwareIdentityKeyBackend<H>
{
    fn load_key(
        &self,
        handle: super::identity_key_service::KeyHandle,
    ) -> Result<[u8; 32], super::identity_key_service::IdentityKeyError> {
        self.hal
            .load_encryption_key(HardwareKeyHandle(handle.0))
            .map_err(|error| match error {
                HardwareKeyError::InvalidHandle => {
                    super::identity_key_service::IdentityKeyError::InvalidHandle
                }
                HardwareKeyError::AccessDenied => {
                    super::identity_key_service::IdentityKeyError::NotAuthorized
                }
                HardwareKeyError::NotProvisioned => {
                    super::identity_key_service::IdentityKeyError::KeyUnavailable
                }
                HardwareKeyError::Unavailable | HardwareKeyError::HardwareFailure => {
                    super::identity_key_service::IdentityKeyError::HardwareFailure
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHal;

    impl IdentityKeyHal for TestHal {
        fn load_encryption_key(
            &self,
            handle: HardwareKeyHandle,
        ) -> Result<[u8; 32], HardwareKeyError> {
            if handle.0 == 0 {
                return Err(HardwareKeyError::InvalidHandle);
            }
            Ok([0xA5; 32])
        }
    }

    #[test]
    fn hardware_backend_maps_success() {
        let backend = HardwareIdentityKeyBackend::new(TestHal);
        let key = super::super::identity_key_service::IdentityKeyBackend::load_key(
            &backend,
            super::super::identity_key_service::KeyHandle(1),
        )
        .unwrap();
        assert_eq!(key, [0xA5; 32]);
    }

    #[test]
    fn invalid_hardware_handle_fails_closed() {
        let backend = HardwareIdentityKeyBackend::new(TestHal);
        let result = super::super::identity_key_service::IdentityKeyBackend::load_key(
            &backend,
            super::super::identity_key_service::KeyHandle(0),
        );
        assert_eq!(
            result,
            Err(super::super::identity_key_service::IdentityKeyError::InvalidHandle)
        );
    }
}
