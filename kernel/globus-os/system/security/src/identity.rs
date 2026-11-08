//! Hardware-backed identity and key-use policy contracts.

/// The intended purpose of a cryptographic key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyPurpose {
    /// Key used for device identity verification.
    DeviceIdentity,
    /// Key used for user identity verification.
    UserIdentity,
    /// Key used for storage encryption.
    StorageEncryption,
    /// Key used for network communication security.
    Network,
    /// Key used for general digital signing.
    Signing,
}

/// The hardware or software backend storing cryptographic keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyBackend {
    /// Trusted Platform Module 2.0 backend.
    Tpm2,
    /// Dedicated hardware secure element backend.
    SecureElement,
    /// Software-sealed key storage backend.
    SoftwareSealed,
}

/// A handle representing a key managed by an identity provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyHandle(pub u128);

/// Errors that can occur during key operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    /// The key or backend is unavailable.
    Unavailable,
    /// Operation unauthorized for this key.
    Unauthorized,
    /// Key purpose is invalid for the requested operation.
    InvalidPurpose,
    /// The key handle is invalid.
    InvalidHandle,
}

/// Trait for hardware-backed identity and cryptographic key management.
pub trait IdentityProvider {
    /// Returns the key backend type used by this provider.
    fn backend(&self) -> KeyBackend;

    /// Generates an attestation statement using the given nonce.
    fn attest(&self, nonce: &[u8], output: &mut [u8]) -> Result<usize, KeyError>;

    /// Signs a message using the specified key handle and key purpose.
    fn sign(
        &mut self,
        key: KeyHandle,
        purpose: KeyPurpose,
        message: &[u8],
        signature: &mut [u8],
    ) -> Result<usize, KeyError>;
}

/// Checks whether a requested key purpose matches the provisioned purpose.
pub fn purpose_allowed(requested: KeyPurpose, provisioned: KeyPurpose) -> bool {
    requested == provisioned
}
