//! Capability-scoped IPC adapter for the ShivaCore identity-key service.

use crate::key_provider::{SecureKeyId, SecureKeyService, SecureKeyServiceError};
use globus_ipc::{IdentityKeyHandle, IdentityKeyLoadRequest};
use zeroize::Zeroizing;

/// Trusted local transport used to invoke the ShivaCore identity-key endpoint.
pub trait IdentityKeyIpcTransport {
    fn load_key(
        &self,
        request: IdentityKeyLoadRequest,
    ) -> Result<Zeroizing<[u8; 32]>, SecureKeyServiceError>;
}

/// Capability-backed implementation of the identity key service over local IPC.
pub struct IpcIdentityKeyService<T> {
    transport: T,
    capability_token: u64,
}
impl<T> IpcIdentityKeyService<T> {
    pub fn new(transport: T, capability_token: u64) -> Self {
        Self {
            transport,
            capability_token,
        }
    }
}
impl<T: IdentityKeyIpcTransport> SecureKeyService for IpcIdentityKeyService<T> {
    fn load_identity_key(
        &self,
        key_id: SecureKeyId,
    ) -> Result<Zeroizing<[u8; 32]>, SecureKeyServiceError> {
        self.transport.load_key(IdentityKeyLoadRequest {
            key_handle: IdentityKeyHandle(key_id.0),
            capability_token: self.capability_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestTransport;
    impl IdentityKeyIpcTransport for TestTransport {
        fn load_key(
            &self,
            request: IdentityKeyLoadRequest,
        ) -> Result<Zeroizing<[u8; 32]>, SecureKeyServiceError> {
            assert_eq!(request.key_handle, IdentityKeyHandle(7));
            assert_eq!(request.capability_token, 99);
            Ok(Zeroizing::new([0xA5; 32]))
        }
    }
    #[test]
    fn maps_secure_key_request_to_opaque_ipc_request() {
        let service = IpcIdentityKeyService::new(TestTransport, 99);
        let key = service.load_identity_key(SecureKeyId(7)).unwrap();
        assert_eq!(&*key, &[0xA5; 32]);
    }
}
