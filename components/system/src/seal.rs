// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Stufe 3: die Freigabe (ATC-SEC-EXEC-001) mit echter Signaturpruefung.
//!
//! Das Admission-Crate ordnet Entscheidungen zu und behauptet ausdruecklich
//! keine Kryptografie. Hier kommt die Krypto her: ein ECDSA-Schluessel aus
//! `atc-wallet` signiert die kanonischen Policy-Bytes, ein
//! `AuthorizedKeyVerifier` prueft sie gegen den oeffentlichen Schluessel der
//! Freigabe. Damit ist der Pfad nicht mehr "unsigniert, aber lokal vorhanden":
//! ohne gueltige Signatur ist die Entscheidung BLOCKED.

use atc_exec_admission::{ExecutionPolicy, SignatureVerifier};
use atc_vm::artifact::sha256_daten;
use atc_wallet::keys::WalletKey;
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

/// Warum ein Schluessel nicht verwendbar ist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SealError {
    InvalidSeed,
    InvalidPublicKey,
    InvalidSignatureEncoding,
}

/// Signaturschluessel der Freigabeinstanz.
pub struct SealKey {
    key: WalletKey,
}

impl SealKey {
    pub fn from_seed(seed: [u8; 32]) -> Result<Self, SealError> {
        WalletKey::from_seed(seed)
            .map(|key| SealKey { key })
            .map_err(|_| SealError::InvalidSeed)
    }

    /// Kanonische Adresse (ATC-Praefix), die als `signer_identity` in die
    /// Freigabe geht.
    pub fn address(&self) -> String {
        self.key.address()
    }

    pub fn public_key(&self) -> [u8; 33] {
        self.key.public_key()
    }

    /// Signiert einen fertigen Digest. Nur fuer Nachweise gedacht (z.B. um zu
    /// zeigen, dass eine Signatur ueber andere Bytes nicht passt).
    pub fn sign_digest(&self, digest: [u8; 32]) -> [u8; 64] {
        self.key.sign_digest(digest).serialize_compact()
    }

    /// Signiert die kanonischen Policy-Bytes. Die Domain-Trennung bringt die
    /// Policy selbst mit (`ATC-EXEC-ADM-v1\0`), deshalb wird hier nur der
    /// SHA-256 ueber genau diese Bytes signiert.
    pub fn sign_policy(&self, policy: &ExecutionPolicy) -> [u8; 64] {
        let digest = sha256_daten(&policy.canonical_bytes());
        self.key.sign_digest(digest).serialize_compact()
    }
}

/// Prueft Signaturen gegen genau einen oeffentlichen Schluessel.
pub struct AuthorizedKeyVerifier {
    public_key: [u8; 33],
}

impl AuthorizedKeyVerifier {
    pub fn new(public_key: [u8; 33]) -> Result<Self, SealError> {
        PublicKey::from_slice(&public_key).map_err(|_| SealError::InvalidPublicKey)?;
        Ok(AuthorizedKeyVerifier { public_key })
    }

    pub fn public_key(&self) -> [u8; 33] {
        self.public_key
    }
}

impl SignatureVerifier for AuthorizedKeyVerifier {
    fn verify(&self, policy_bytes: &[u8], signature: &[u8]) -> bool {
        let Ok(raw): Result<[u8; 64], _> = signature.try_into() else {
            return false;
        };
        let Ok(signature) = Signature::from_compact(&raw) else {
            return false;
        };
        let Ok(public_key) = PublicKey::from_slice(&self.public_key) else {
            return false;
        };
        let digest = sha256_daten(policy_bytes);
        Secp256k1::verification_only()
            .verify_ecdsa(Message::from_digest(digest), &signature, &public_key)
            .is_ok()
    }
}
