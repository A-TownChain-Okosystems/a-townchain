// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Canonical L1 transaction wallet key.
//!
//! Transaction/account signatures use secp256k1 ECDSA with RFC6979,
//! SHA-256, canonical low-S signatures, and compressed public keys.
//! Identity keys are a separate Ed25519 contract and are not used here.

use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
use sha2::Sha256;

#[derive(Clone)]
pub struct WalletKey(SigningKey);

impl core::fmt::Debug for WalletKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WalletKey").finish_non_exhaustive()
    }
}

impl WalletKey {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self(SigningKey::from_bytes((&seed).into()).expect("32-byte seed must be valid"))
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        let mut signature: Signature = self.0.sign(message);
        if let Some(normalized) = signature.normalize_s() {
            signature = normalized;
        }
        signature
    }

    pub fn public_key(&self) -> [u8; 33] {
        let encoded = VerifyingKey::from(&self.0).to_encoded_point(true);
        let bytes = encoded.as_bytes();
        let mut out = [0u8; 33];
        out.copy_from_slice(bytes);
        out
    }
}
