//! Canonical wallet signing key for ATC-TX-DOMAIN-V2.
//! Transaction/account signatures use secp256k1 ECDSA with RFC6979,
//! SHA-256 and low-S normalization.

use k256::ecdsa::{
    signature::{DigestSigner, SignatureEncoding},
    Signature, SigningKey, VerifyingKey,
};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct WalletKey {
    signing_key: SigningKey,
}

impl WalletKey {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self {
            signing_key: SigningKey::from_bytes((&seed).into()).expect("32-byte seed is a valid secp256k1 scalar"),
        }
    }

    pub fn sign(&self, signing_bytes: &[u8]) -> Signature {
        let digest = Sha256::new_with_prefix(signing_bytes);
        let signature: Signature = self.signing_key.sign_digest(digest);
        signature.normalize_s().unwrap_or(signature)
    }

    pub fn public_key(&self) -> [u8; 33] {
        let encoded = self.signing_key.verifying_key().to_encoded_point(true);
        let mut out = [0u8; 33];
        out.copy_from_slice(encoded.as_bytes());
        out
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key().clone()
    }
}
