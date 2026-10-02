//! Wallet key management backed by Ed25519.
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

#[derive(Clone)]
pub struct WalletKey { signing_key: SigningKey }
impl WalletKey {
    pub fn from_seed(seed: [u8; 32]) -> Self { Self { signing_key: SigningKey::from_bytes(&seed) } }
    pub fn public_key(&self) -> [u8; 32] { self.signing_key.verifying_key().to_bytes() }
    pub fn sign(&self, message: &[u8]) -> Signature { self.signing_key.sign(message) }
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool { self.signing_key.verifying_key().verify(message, signature).is_ok() }
    pub fn verifying_key(&self) -> VerifyingKey { self.signing_key.verifying_key() }
}
#[cfg(test)]
mod tests { use super::*; #[test] fn roundtrip() { let k=WalletKey::from_seed([7;32]); let s=k.sign(b"atc"); assert!(k.verify(b"atc",&s)); assert!(!k.verify(b"bad",&s)); } }
