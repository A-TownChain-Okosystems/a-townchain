// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Deterministic secp256k1 wallet key management for transaction/account use.

use secp256k1::{ecdsa::Signature, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

/// Kanonische ATC-Adresse (SEC-P0-005): Praefix `ATC:` + 20-Byte-Prefix von
/// SHA-256(compressed_pubkey), hex-kodiert. Bindet Adresse/DID deterministisch
/// an den Public Key, statt sie als freien String zu behandeln.
pub fn address_from_pubkey(public_key: &[u8; 33]) -> String {
    let digest = Sha256::digest(public_key);
    format!("ATC:{}", hex_lower(&digest[..20]))
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletKeyError {
    InvalidSecretKey,
}

#[derive(Clone)]
pub struct WalletKey {
    signing_key: SecretKey,
}

impl WalletKey {
    pub fn from_seed(seed: [u8; 32]) -> Result<Self, WalletKeyError> {
        SecretKey::from_byte_array(seed)
            .map(|signing_key| Self { signing_key })
            .map_err(|_| WalletKeyError::InvalidSecretKey)
    }

    pub fn public_key(&self) -> [u8; 33] {
        let secp = Secp256k1::new();
        PublicKey::from_secret_key(&secp, &self.signing_key).serialize()
    }

    /// Kanonische Adresse dieses Schluessels (siehe `address_from_pubkey`).
    pub fn address(&self) -> String {
        address_from_pubkey(&self.public_key())
    }

    pub fn sign_digest(&self, digest: [u8; 32]) -> Signature {
        let secp = Secp256k1::new();
        secp.sign_ecdsa(secp256k1::Message::from_digest(digest), &self.signing_key)
    }

    pub fn verify_digest(
        &self,
        digest: [u8; 32],
        signature: &Signature,
    ) -> Result<(), WalletKeyError> {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.signing_key);
        secp.verify_ecdsa(
            secp256k1::Message::from_digest(digest),
            signature,
            &public_key,
        )
        .map_err(|_| WalletKeyError::InvalidSecretKey)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_key_roundtrip() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let signature = key.sign_digest([3u8; 32]);
        assert!(key.verify_digest([3u8; 32], &signature).is_ok());
        assert!(key.verify_digest([4u8; 32], &signature).is_err());
        assert_eq!(key.public_key().len(), 33);
        assert!(matches!(
            WalletKey::from_seed([0u8; 32]),
            Err(WalletKeyError::InvalidSecretKey)
        ));
    }
}
