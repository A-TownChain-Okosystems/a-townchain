//! Package metadata and cryptographic verification policy.
//!
//! Installation and update callers must use `verify_for_install` or
//! `verify_for_update`; metadata presence alone is never a trust decision.

use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub name: String,
    pub version: String,
    /// SHA-256 digest encoded as `sha256:` followed by 64 lowercase hex digits.
    pub digest: String,
    /// Ed25519 signature encoded as 128 hexadecimal digits.
    pub signature: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verification {
    Trusted,
    Untrusted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyError {
    EmptyMetadata,
    NoTrustedKeys,
    RevokedSigningKey,
    InvalidDigest,
    DigestMismatch,
    InvalidSignature,
    UntrustedSigningKey,
}

/// A keyring is supplied by the trusted system configuration, never by package
/// metadata. Revocation is deny-by-default and takes precedence over trust.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrustedKeyring {
    trusted_keys: Vec<[u8; 32]>,
    revoked_key_ids: Vec<String>,
}

impl TrustedKeyring {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn trust_key(&mut self, key: [u8; 32]) {
        let id = key_id(&key);
        if self.revoked_key_ids.iter().any(|revoked| revoked == &id) {
            return;
        }
        if !self.trusted_keys.contains(&key) {
            self.trusted_keys.push(key);
        }
    }

    pub fn revoke_key(&mut self, key: &[u8; 32]) {
        let id = key_id(key);
        if !self.revoked_key_ids.iter().any(|revoked| revoked == &id) {
            self.revoked_key_ids.push(id);
        }
    }

    pub fn is_revoked(&self, key: &[u8; 32]) -> bool {
        let id = key_id(key);
        self.revoked_key_ids.iter().any(|revoked| revoked == &id)
    }

    fn active_keys(&self) -> Vec<[u8; 32]> {
        self.trusted_keys
            .iter()
            .filter(|key| !self.is_revoked(key))
            .copied()
            .collect()
    }
}

/// Owns the exact artifact bytes that passed verification. Fields are private
/// so callers cannot construct a verified artifact without going through this
/// module's verifier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedArtifact {
    package: Package,
    bytes: Vec<u8>,
}

impl VerifiedArtifact {
    pub fn package(&self) -> &Package {
        &self.package
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// Verify exact artifact bytes against a system-provided keyring. The returned
/// object carries those same bytes for the install/update consumer.
pub fn verify_for_install(
    pkg: &Package,
    artifact: &[u8],
    keyring: &TrustedKeyring,
) -> Result<VerifiedArtifact, VerifyError> {
    verify_with_keyring(pkg, artifact, keyring)?;
    Ok(VerifiedArtifact {
        package: pkg.clone(),
        bytes: artifact.to_vec(),
    })
}

/// Update paths must use the same fail-closed verification boundary as installs.
pub fn verify_for_update(
    pkg: &Package,
    artifact: &[u8],
    keyring: &TrustedKeyring,
) -> Result<VerifiedArtifact, VerifyError> {
    verify_for_install(pkg, artifact, keyring)
}

fn verify_with_keyring(
    pkg: &Package,
    artifact: &[u8],
    keyring: &TrustedKeyring,
) -> Result<(), VerifyError> {
    if pkg.name.trim().is_empty() || pkg.version.trim().is_empty() {
        return Err(VerifyError::EmptyMetadata);
    }
    if keyring.trusted_keys.is_empty() {
        return Err(VerifyError::NoTrustedKeys);
    }
    if keyring
        .trusted_keys
        .iter()
        .all(|key| keyring.is_revoked(key))
    {
        return Err(VerifyError::RevokedSigningKey);
    }
    let active = keyring.active_keys();
    match verify_package(pkg, artifact, &active) {
        Verification::Trusted => Ok(()),
        Verification::Untrusted => {
            if pkg.digest.strip_prefix("sha256:").is_none() {
                return Err(VerifyError::InvalidDigest);
            }
            let actual = Sha256::digest(artifact);
            let declared = pkg.digest.strip_prefix("sha256:").unwrap_or_default();
            if declared.len() != 64
                || !declared
                    .bytes()
                    .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
                || hex::decode(declared).is_err()
            {
                return Err(VerifyError::InvalidDigest);
            }
            if hex::decode(declared).ok().as_deref() != Some(actual.as_slice()) {
                return Err(VerifyError::DigestMismatch);
            }
            if hex::decode(&pkg.signature)
                .ok()
                .and_then(|bytes| Signature::from_slice(&bytes).ok())
                .is_none()
            {
                return Err(VerifyError::InvalidSignature);
            }
            let message = signed_metadata(pkg);
            for key in &keyring.trusted_keys {
                if keyring.is_revoked(key) {
                    continue;
                }
                let Ok(verifying_key) = VerifyingKey::from_bytes(key) else {
                    continue;
                };
                let Ok(signature_bytes) = hex::decode(&pkg.signature) else {
                    continue;
                };
                let Ok(signature) = Signature::from_slice(&signature_bytes) else {
                    continue;
                };
                if verifying_key.verify(&message, &signature).is_ok() {
                    return Err(VerifyError::UntrustedSigningKey);
                }
            }
            if keyring
                .trusted_keys
                .iter()
                .any(|key| keyring.is_revoked(key))
            {
                Err(VerifyError::RevokedSigningKey)
            } else {
                Err(VerifyError::UntrustedSigningKey)
            }
        }
    }
}

pub fn verify_package(pkg: &Package, artifact: &[u8], trusted_keys: &[[u8; 32]]) -> Verification {
    if pkg.name.trim().is_empty() || pkg.version.trim().is_empty() || trusted_keys.is_empty() {
        return Verification::Untrusted;
    }
    let Some(declared_digest) = pkg.digest.strip_prefix("sha256:") else {
        return Verification::Untrusted;
    };
    if declared_digest.len() != 64
        || !declared_digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Verification::Untrusted;
    }
    let actual_digest = Sha256::digest(artifact);
    let Ok(declared_digest_bytes) = hex::decode(declared_digest) else {
        return Verification::Untrusted;
    };
    if actual_digest.as_slice() != declared_digest_bytes.as_slice() {
        return Verification::Untrusted;
    }
    let Ok(signature_bytes) = hex::decode(&pkg.signature) else {
        return Verification::Untrusted;
    };
    let Ok(signature) = Signature::from_slice(&signature_bytes) else {
        return Verification::Untrusted;
    };
    let message = signed_metadata(pkg);
    for key_bytes in trusted_keys {
        let Ok(key) = VerifyingKey::from_bytes(key_bytes) else {
            continue;
        };
        if key.verify(&message, &signature).is_ok() {
            return Verification::Trusted;
        }
    }
    Verification::Untrusted
}

fn key_id(key: &[u8; 32]) -> String {
    hex::encode(Sha256::digest(key))
}

fn signed_metadata(pkg: &Package) -> Vec<u8> {
    let mut message = b"GLOBUSOS-PACKAGE-SIGNATURE-V1\0".to_vec();
    message.extend_from_slice(pkg.name.as_bytes());
    message.push(0);
    message.extend_from_slice(pkg.version.as_bytes());
    message.push(0);
    message.extend_from_slice(pkg.digest.as_bytes());
    message
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn signed_package(artifact: &[u8], signing_key: &SigningKey) -> Package {
        let digest = format!("sha256:{}", hex::encode(Sha256::digest(artifact)));
        let mut pkg = Package {
            name: "example".into(),
            version: "1.2.3".into(),
            digest,
            signature: String::new(),
        };
        pkg.signature = hex::encode(signing_key.sign(&signed_metadata(&pkg)).to_bytes());
        pkg
    }

    fn keyring(key: &SigningKey) -> TrustedKeyring {
        let mut ring = TrustedKeyring::new();
        ring.trust_key(key.verifying_key().to_bytes());
        ring
    }

    #[test]
    fn install_returns_exact_verified_bytes() {
        let key = SigningKey::from_bytes(&[7_u8; 32]);
        let artifact = b"trusted package contents";
        let pkg = signed_package(artifact, &key);
        let verified = verify_for_install(&pkg, artifact, &keyring(&key)).unwrap();
        assert_eq!(verified.bytes(), artifact);
        assert_eq!(verified.package(), &pkg);
    }

    #[test]
    fn update_rejects_modified_artifact() {
        let key = SigningKey::from_bytes(&[7_u8; 32]);
        let pkg = signed_package(b"original", &key);
        assert_eq!(
            verify_for_update(&pkg, b"modified", &keyring(&key)),
            Err(VerifyError::DigestMismatch)
        );
    }

    #[test]
    fn rejects_untrusted_signing_key() {
        let signer = SigningKey::from_bytes(&[7_u8; 32]);
        let other = SigningKey::from_bytes(&[8_u8; 32]);
        let pkg = signed_package(b"contents", &signer);
        assert_eq!(
            verify_for_install(&pkg, b"contents", &keyring(&other)),
            Err(VerifyError::UntrustedSigningKey)
        );
    }

    #[test]
    fn revoked_key_is_rejected_even_if_trust_was_configured() {
        let key = SigningKey::from_bytes(&[7_u8; 32]);
        let artifact = b"contents";
        let pkg = signed_package(artifact, &key);
        let mut ring = keyring(&key);
        ring.revoke_key(&key.verifying_key().to_bytes());
        assert!(ring.is_revoked(&key.verifying_key().to_bytes()));
        assert_eq!(
            verify_for_install(&pkg, artifact, &ring),
            Err(VerifyError::RevokedSigningKey)
        );
        ring.trust_key(key.verifying_key().to_bytes());
        assert_eq!(
            verify_for_install(&pkg, artifact, &ring),
            Err(VerifyError::RevokedSigningKey)
        );
    }

    #[test]
    fn rejects_missing_keys_and_malformed_metadata() {
        let key = SigningKey::from_bytes(&[7_u8; 32]);
        let artifact = b"contents";
        let mut pkg = signed_package(artifact, &key);
        let empty = TrustedKeyring::new();
        assert_eq!(
            verify_for_install(&pkg, artifact, &empty),
            Err(VerifyError::NoTrustedKeys)
        );
        pkg.digest = "bad".into();
        assert_eq!(
            verify_for_install(&pkg, artifact, &keyring(&key)),
            Err(VerifyError::InvalidDigest)
        );
    }

    #[test]
    fn rejects_metadata_tampering() {
        let key = SigningKey::from_bytes(&[7_u8; 32]);
        let artifact = b"contents";
        let mut pkg = signed_package(artifact, &key);
        pkg.version = "9.9.9".into();
        assert_eq!(
            verify_for_install(&pkg, artifact, &keyring(&key)),
            Err(VerifyError::UntrustedSigningKey)
        );
    }
}
