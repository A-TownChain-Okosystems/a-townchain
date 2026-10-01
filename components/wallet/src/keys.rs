// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Canonical L1 transaction/account signing key.
//!
//! Consensus identity keys are a separate contract. This type is only for
//! transaction/account authentication and uses secp256k1 ECDSA.

use k256::ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct WalletKey {
    signing_key: SigningKey,
}

impl WalletKey {
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes((&seed).into())
            .expect("32-byte seed must produce a valid secp256k1 signing key");
        Self { signing_key }
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.signing_key.sign(message)
    }

    pub fn public_key(&self) -> [u8; 33] {
        let verifying_key = VerifyingKey::from(&self.signing_key);
        let encoded = verifying_key.to_encoded_point(true);
        let bytes = encoded.as_bytes();
        let mut out = [0u8; 33];
        out.copy_from_slice(bytes);
        out
    }

    pub fn signing_key_fingerprint(&self) -> [u8; 32] {
        Sha256::digest(self.public_key()).into()
    }
}

impl core::fmt::Debug for WalletKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WalletKey")
            .field("public_key", &self.public_key())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_key_is_canonical_compressed_sec1() {
        let key = WalletKey::from_seed([7u8; 32]);
        let public_key = key.public_key();
        assert_eq!(public_key.len(), 33);
        assert!(public_key[0] == 0x02 || public_key[0] == 0x03);
        assert!(VerifyingKey::from_sec1_bytes(&public_key).is_ok());
    }

    #[test]
    fn signature_is_verifiable() {
        let key = WalletKey::from_seed([7u8; 32]);
        let signature = key.sign(b"ATC-TX-DOMAIN-V2");
        let verifying_key = VerifyingKey::from_sec1_bytes(&key.public_key()).unwrap();
        assert!(verifying_key.verify(b"ATC-TX-DOMAIN-V2", &signature).is_ok());
    }
}
