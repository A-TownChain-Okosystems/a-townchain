// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Deterministic Ed25519 wallet key management.

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};

#[derive(Clone)]
pub struct WalletKey {
    signing_key: SigningKey,
}

impl WalletKey {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self { signing_key: SigningKey::from_bytes(&seed) }
    }

    pub fn public_key(&self) -> [u8; 32] {
        self.signing_key.verifying_key().to_bytes()
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        self.signing_key.verifying_key().verify(message, signature).is_ok()
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_key_roundtrip() {
        let key = WalletKey::from_seed([7u8; 32]);
        let signature = key.sign(b"atc-wallet");
        assert!(key.verify(b"atc-wallet", &signature));
        assert!(!key.verify(b"tampered", &signature));
    }
}
