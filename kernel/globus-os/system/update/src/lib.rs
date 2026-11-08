//! Atomic update state machine with rollback semantics.

pub mod boot_state;
pub mod health;
pub mod transition;

pub use boot_state::{BootState, BootStateError};
pub use health::{BootHealth, HealthError};
pub use transition::TransitionError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Slot {
    A,
    B,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateState {
    Idle,
    Downloaded,
    Verified,
    Activated,
    RolledBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpdatePlan {
    pub current: Slot,
    pub target: Slot,
    pub state: UpdateState,
}

impl UpdatePlan {
    pub fn new(current: Slot) -> Self {
        Self {
            current,
            target: match current {
                Slot::A => Slot::B,
                Slot::B => Slot::A,
            },
            state: UpdateState::Idle,
        }
    }
}

/// Verify the exact downloaded update bytes before allowing them to be staged.
/// Callers must stage only the bytes carried by the returned VerifiedArtifact.
pub fn verify_update_package(
    package: &globus_package::Package,
    downloaded_bytes: &[u8],
    keyring: &globus_package::TrustedKeyring,
) -> Result<globus_package::VerifiedArtifact, globus_package::VerifyError> {
    globus_package::verify_for_update(package, downloaded_bytes, keyring)
}

#[cfg(test)]
mod package_verification_tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use sha2::{Digest, Sha256};

    fn signed_package(bytes: &[u8], key: &SigningKey) -> globus_package::Package {
        let mut package = globus_package::Package {
            name: "system-update".into(),
            version: "2.0.0".into(),
            digest: format!("sha256:{}", hex::encode(Sha256::digest(bytes))),
            signature: String::new(),
        };
        let mut message = b"GLOBUSOS-PACKAGE-SIGNATURE-V1\0".to_vec();
        message.extend_from_slice(package.name.as_bytes());
        message.push(0);
        message.extend_from_slice(package.version.as_bytes());
        message.push(0);
        message.extend_from_slice(package.digest.as_bytes());
        package.signature = hex::encode(key.sign(&message).to_bytes());
        package
    }

    #[test]
    fn update_verification_returns_the_exact_downloaded_bytes() {
        let key = SigningKey::from_bytes(&[11_u8; 32]);
        let bytes = b"signed update image";
        let package = signed_package(bytes, &key);
        let mut keyring = globus_package::TrustedKeyring::new();
        keyring.trust_key(key.verifying_key().to_bytes());
        let verified = verify_update_package(&package, bytes, &keyring).unwrap();
        assert_eq!(verified.bytes(), bytes);
    }

    #[test]
    fn update_verification_blocks_tampered_download() {
        let key = SigningKey::from_bytes(&[11_u8; 32]);
        let package = signed_package(b"original update", &key);
        let mut keyring = globus_package::TrustedKeyring::new();
        keyring.trust_key(key.verifying_key().to_bytes());
        assert_eq!(
            verify_update_package(&package, b"tampered update", &keyring),
            Err(globus_package::VerifyError::DigestMismatch)
        );
    }

    #[test]
    fn update_verification_blocks_revoked_signing_key() {
        let key = SigningKey::from_bytes(&[11_u8; 32]);
        let bytes = b"signed update image";
        let package = signed_package(bytes, &key);
        let mut keyring = globus_package::TrustedKeyring::new();
        keyring.trust_key(key.verifying_key().to_bytes());
        keyring.revoke_key(&key.verifying_key().to_bytes());
        assert_eq!(
            verify_update_package(&package, bytes, &keyring),
            Err(globus_package::VerifyError::RevokedSigningKey)
        );
    }
}
