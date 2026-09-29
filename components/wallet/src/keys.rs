// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Trusted transaction/account key core.
//!
//! Transaction/account keys are ECDSA secp256k1 only. Ed25519 is intentionally
//! excluded from this module; it remains available for purpose-bound identity
//! domains such as node/service/agent identity.

use k256::ecdsa::{
    signature::hazmat::{PrehashSigner, PrehashVerifier},
    Signature, SigningKey, VerifyingKey,
};
use sha2::{Digest, Sha256};

pub const PRIVATE_KEY_LEN: usize = 32;
pub const PUBLIC_KEY_LEN: usize = 33;
pub const SIGNATURE_LEN: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    InvalidPrivateKey,
    InvalidPublicKey,
    InvalidSignature,
    HighS,
    SigningFailure,
}

pub struct WalletKey(SigningKey);

impl WalletKey {
    /// Construct a transaction/account key from a raw 32-byte secp256k1 scalar.
    pub fn from_private_key_bytes(bytes: &[u8; PRIVATE_KEY_LEN]) -> Result<Self, KeyError> {
        SigningKey::from_bytes(bytes.into())
            .map(Self)
            .map_err(|_| KeyError::InvalidPrivateKey)
    }

    /// Return the canonical compressed SEC1 public key (33 bytes).
    pub fn public_key(&self) -> [u8; PUBLIC_KEY_LEN] {
        self.0
            .verifying_key()
            .to_encoded_point(true)
            .as_bytes()
            .try_into()
            .expect("compressed secp256k1 public key is exactly 33 bytes")
    }

    /// Sign the SHA-256 prehash of canonical protocol bytes.
    ///
    /// k256's ECDSA prehash signer uses RFC6979 deterministic nonce generation.
    /// The resulting signature is normalized to low-S before serialization.
    pub fn sign(&self, message: &[u8]) -> Result<[u8; SIGNATURE_LEN], KeyError> {
        let prehash = Sha256::digest(message);
        let signature: Signature = self
            .0
            .sign_prehash(&prehash)
            .map_err(|_| KeyError::SigningFailure)?
            .normalize_s();

        Ok(signature.to_bytes().into())
    }

    /// Verify a canonical secp256k1 signature over the SHA-256 prehash.
    ///
    /// High-S signatures are rejected instead of normalized during verification.
    pub fn verify(
        public_key: &[u8; PUBLIC_KEY_LEN],
        message: &[u8],
        signature: &[u8; SIGNATURE_LEN],
    ) -> Result<(), KeyError> {
        let key = VerifyingKey::from_sec1_bytes(public_key)
            .map_err(|_| KeyError::InvalidPublicKey)?;
        let signature =
            Signature::try_from(signature.as_slice()).map_err(|_| KeyError::InvalidSignature)?;

        if signature.normalize_s() != signature {
            return Err(KeyError::HighS);
        }

        let prehash = Sha256::digest(message);
        key.verify_prehash(&prehash, &signature)
            .map_err(|_| KeyError::InvalidSignature)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use k256::elliptic_curve::PrimeField;

    fn key() -> WalletKey {
        WalletKey::from_private_key_bytes(&[7u8; 32]).unwrap()
    }

    #[test]
    fn secp256k1_roundtrip() {
        let key = key();
        let message = b"ATC-TX-DOMAIN-V2 test";
        let signature = key.sign(message).unwrap();

        assert_eq!(key.public_key().len(), PUBLIC_KEY_LEN);
        assert!(WalletKey::verify(&key.public_key(), message, &signature).is_ok());
    }

    #[test]
    fn signing_is_deterministic() {
        let key = key();
        let message = b"ATC-TX-DOMAIN-V2 deterministic";

        assert_eq!(key.sign(message).unwrap(), key.sign(message).unwrap());
    }

    #[test]
    fn wrong_message_is_rejected() {
        let key = key();
        let signature = key.sign(b"canonical").unwrap();

        assert!(WalletKey::verify(&key.public_key(), b"altered", &signature).is_err());
    }

    #[test]
    fn invalid_private_key_is_rejected() {
        assert!(matches!(
            WalletKey::from_private_key_bytes(&[0u8; 32]),
            Err(KeyError::InvalidPrivateKey)
        ));
    }

    #[test]
    fn high_s_signature_is_rejected() {
        let key = key();
        let message = b"high-s rejection";
        let low_s = key.sign(message).unwrap();
        let r = &low_s[..32];
        let low_s_scalar =
            k256::Scalar::from_repr(low_s[32..].try_into().unwrap()).unwrap();
        let high_s = (-low_s_scalar).to_bytes();

        let mut encoded = [0u8; SIGNATURE_LEN];
        encoded[..32].copy_from_slice(r);
        encoded[32..].copy_from_slice(&high_s);

        assert!(matches!(
            WalletKey::verify(&key.public_key(), message, &encoded),
            Err(KeyError::HighS)
        ));
    }

    #[test]
    fn malformed_public_key_is_rejected() {
        let key = key();
        let signature = key.sign(b"malformed public key").unwrap();
        let public_key = [0u8; PUBLIC_KEY_LEN];

        assert!(matches!(
            WalletKey::verify(&public_key, b"malformed public key", &signature),
            Err(KeyError::InvalidPublicKey)
        ));
    }
}
