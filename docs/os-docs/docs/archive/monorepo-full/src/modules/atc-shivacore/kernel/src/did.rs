// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Dezentrale Knoten-Identitaet (DID) (Rust).
//!
//! Portiert did.py (Python, 08.07.2026) nach Rust.
//! Dezentrale Identitaet mit kryptographischen Signaturen.
//! Zwei Implementierungen des CryptoProvider-Traits:
//!   1. SoftwareSigner — deterministische Pseudo-Signatur (fuer reproduzierbare Tests)
//!   2. Ed25519Signer — echte Ed25519-Signaturen mit ed25519-dalek

use alloc::vec;
extern crate alloc;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier, Signature};


/// Dezentrale Identitaet: did:shivacore:<hex-public-key>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Did {
    pub value: String,
}

impl Did {
    pub fn new(value: &str) -> Self { Self { value: value.to_string() } }
    pub fn as_str(&self) -> &str { &self.value }
}

/// Trait fuer kryptographische Operationen.
/// Implementierungen: Ed25519Signer (echte Kryptografie), SoftwareSigner (Tests).
pub trait CryptoProvider {
    fn did(&self) -> &Did;
    fn sign(&self, payload: &[u8]) -> Vec<u8>;
    fn verify(did: &Did, payload: &[u8], signature: &[u8]) -> bool;
}

// =========================================================================
// SoftwareSigner — deterministische Pseudo-Signatur fuer reproduzierbare Tests
// =========================================================================

#[derive(Debug, Clone)]
pub struct SoftwareSigner {
    did: Did,
    private_key: Vec<u8>,
}

impl SoftwareSigner {
    pub fn new(name: &str) -> Self {
        let private = name.as_bytes().to_vec();
        let public_b64 = hex_encode(&private);
        let did = Did::new(&format!("did:shivacore:{}", public_b64));
        Self { did, private_key: private }
    }

    pub fn with_key(private_key: &[u8]) -> Self {
        let public_b64 = hex_encode(private_key);
        let did = Did::new(&format!("did:shivacore:{}", public_b64));
        Self { did, private_key: private_key.to_vec() }
    }
}

impl CryptoProvider for SoftwareSigner {
    fn did(&self) -> &Did { &self.did }
    fn sign(&self, payload: &[u8]) -> Vec<u8> {
        let mut sig = Vec::with_capacity(payload.len() + self.private_key.len());
        for (i, b) in payload.iter().enumerate() {
            sig.push(b ^ self.private_key[i % self.private_key.len()]);
        }
        sig.extend_from_slice(&hex_encode(&self.private_key).into_bytes());
        sig
    }

    fn verify(did: &Did, payload: &[u8], signature: &[u8]) -> bool {
        let prefix = "did:shivacore:";
        if !did.value.starts_with(prefix) { return false; }
        let public_b64 = &did.value[prefix.len()..];
        let key_bytes = match hex_decode(public_b64) {
            Some(k) => k,
            None => return false,
        };
        if signature.len() < payload.len() { return false; }
        let key_hash = &signature[payload.len()..];
        let expected_hash = hex_encode(&key_bytes).into_bytes();
        if key_hash != expected_hash.as_slice() { return false; }
        for (i, b) in payload.iter().enumerate() {
            let decoded = signature[i] ^ key_bytes[i % key_bytes.len()];
            if decoded != *b { return false; }
        }
        true
    }
}

// =========================================================================
// Ed25519Signer — echte Ed25519-Signaturen mit ed25519-dalek
// =========================================================================

/// ED25519-Prefix fuer DIDs, um von SoftwareSigner-DIDs unterscheidbar zu sein
const ED25519_PREFIX: &str = "did:shivacore:ed25519:";

#[derive(Debug)]
pub struct Ed25519Signer {
    did: Did,
    signing_key: SigningKey,
}

impl Ed25519Signer {
    /// Erzeugt eine neue Ed25519-Identitaet mit frischem Schluesselpaar (deterministic seed)
    pub fn new() -> Self {
        use core::sync::atomic::{AtomicU64, Ordering};
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        let seed_val = COUNTER.fetch_add(1, Ordering::Relaxed);
        let mut seed = [0u8; 32];
        seed[0..8].copy_from_slice(&seed_val.to_le_bytes());
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();
        let public_bytes = verifying_key.to_bytes();
        let did = Did::new(&format!("{}{}", ED25519_PREFIX, hex_encode(&public_bytes)));
        Self { did, signing_key }
    }

    /// Erzeugt einen Signer aus einem bekannten SigningKey (fuer Tests)
    pub fn from_signing_key(signing_key: SigningKey) -> Self {
        let verifying_key = signing_key.verifying_key();
        let public_bytes = verifying_key.to_bytes();
        let did = Did::new(&format!("{}{}", ED25519_PREFIX, hex_encode(&public_bytes)));
        Self { did, signing_key }
    }

    /// Erzeugt einen Signer aus einem Seed (32 Bytes) — deterministisch fuer Tests
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(seed);
        Self::from_signing_key(signing_key)
    }
}

impl CryptoProvider for Ed25519Signer {
    fn did(&self) -> &Did { &self.did }

    fn sign(&self, payload: &[u8]) -> Vec<u8> {
        let sig: Signature = self.signing_key.sign(payload);
        sig.to_bytes().to_vec()
    }

    fn verify(did: &Did, payload: &[u8], signature: &[u8]) -> bool {
        if !did.value.starts_with(ED25519_PREFIX) { return false; }
        let public_hex = &did.value[ED25519_PREFIX.len()..];
        let public_bytes = match hex_decode(public_hex) {
            Some(k) if k.len() == 32 => k,
            _ => return false,
        };

        let public_bytes: [u8; 32] = match public_bytes.try_into() {
            Ok(arr) => arr,
            Err(_) => return false,
        };
        let verifying_key = match VerifyingKey::from_bytes(&public_bytes) {
            Ok(vk) => vk,
            Err(_) => return false,
        };

        if signature.len() != 64 { return false; }
        let mut sig_arr = [0u8; 64];
        sig_arr.copy_from_slice(signature);
        let sig = Signature::from_bytes(&sig_arr);

        verifying_key.verify(payload, &sig).is_ok()
    }
}

// =========================================================================
// Hilfsfunktionen
// =========================================================================

fn hex_encode(data: &[u8]) -> String {
    let mut result = String::with_capacity(data.len() * 2);
    for b in data {
        result.push_str(&format!("{:02x}", b));
    }
    result
}

fn hex_decode(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 { return None; }
    let mut result = Vec::new();
    let bytes = hex.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let hi = hex_val(bytes[i])?;
        let lo = hex_val(bytes[i + 1])?;
        result.push((hi << 4) | lo);
        i += 2;
    }
    Some(result)
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- SoftwareSigner Tests ---

    #[test]
    fn test_software_did_format() {
        let signer = SoftwareSigner::new("alice");
        assert!(signer.did().value.starts_with("did:shivacore:"));
    }

    #[test]
    fn test_software_sign_and_verify() {
        let alice = SoftwareSigner::new("alice");
        let payload = b"hello shivacore";
        let sig = alice.sign(payload);
        assert!(SoftwareSigner::verify(alice.did(), payload, &sig));
    }

    #[test]
    fn test_software_verify_rejects_wrong_signer() {
        let alice = SoftwareSigner::new("alice");
        let bob = SoftwareSigner::new("bob");
        let payload = b"hello shivacore";
        let sig = alice.sign(payload);
        assert!(!SoftwareSigner::verify(bob.did(), payload, &sig));
    }

    #[test]
    fn test_software_verify_rejects_tampered_payload() {
        let alice = SoftwareSigner::new("alice");
        let sig = alice.sign(b"original payload");
        assert!(!SoftwareSigner::verify(alice.did(), b"tampered payload", &sig));
    }

    #[test]
    fn test_software_verify_rejects_tampered_signature() {
        let alice = SoftwareSigner::new("alice");
        let payload = b"hello";
        let mut sig = alice.sign(payload);
        sig[0] ^= 0xFF;
        assert!(!SoftwareSigner::verify(alice.did(), payload, &sig));
    }

    #[test]
    fn test_did_equality() {
        let a1 = Did::new("did:shivacore:abc123");
        let a2 = Did::new("did:shivacore:abc123");
        let b = Did::new("did:shivacore:def456");
        assert_eq!(a1, a2);
        assert_ne!(a1, b);
    }

    // --- Ed25519 Tests (echte Kryptografie) ---

    #[test]
    fn test_ed25519_did_format() {
        let signer = Ed25519Signer::new();
        assert!(signer.did().value.starts_with("did:shivacore:ed25519:"));
        // Public key ist 32 bytes = 64 hex chars
        let public_hex = &signer.did().value[ED25519_PREFIX.len()..];
        assert_eq!(public_hex.len(), 64);
    }

    #[test]
    fn test_ed25519_sign_and_verify() {
        let alice = Ed25519Signer::new();
        let payload = b"hello shivacore from ed25519";
        let sig = alice.sign(payload);
        assert_eq!(sig.len(), 64); // Ed25519 signatures are 64 bytes
        assert!(Ed25519Signer::verify(alice.did(), payload, &sig));
    }

    #[test]
    fn test_ed25519_verify_rejects_wrong_signer() {
        let alice = Ed25519Signer::new();
        let bob = Ed25519Signer::new();
        let payload = b"auth test";
        let sig = alice.sign(payload);
        // Bob's DID != Alice's DID → verify fails
        assert!(!Ed25519Signer::verify(bob.did(), payload, &sig));
    }

    #[test]
    fn test_ed25519_verify_rejects_tampered_payload() {
        let alice = Ed25519Signer::new();
        let sig = alice.sign(b"original message");
        // Tampered payload → verify fails
        assert!(!Ed25519Signer::verify(alice.did(), b"tampered message", &sig));
    }

    #[test]
    fn test_ed25519_verify_rejects_tampered_signature() {
        let alice = Ed25519Signer::new();
        let payload = b"test payload";
        let mut sig = alice.sign(payload);
        sig[0] ^= 0xFF; // Flip one bit
        assert!(!Ed25519Signer::verify(alice.did(), payload, &sig));
    }

    #[test]
    fn test_ed25519_verify_rejects_short_signature() {
        let alice = Ed25519Signer::new();
        let payload = b"test";
        let short_sig = vec![0u8; 32]; // Only 32 bytes, not 64
        assert!(!Ed25519Signer::verify(alice.did(), payload, &short_sig));
    }

    #[test]
    fn test_ed25519_deterministic_from_seed() {
        let seed = [42u8; 32];
        let s1 = Ed25519Signer::from_seed(&seed);
        let s2 = Ed25519Signer::from_seed(&seed);
        // Same seed → same DID
        assert_eq!(s1.did(), s2.did());
        // Same seed → same signature
        let payload = b"deterministic test";
        assert_eq!(s1.sign(payload), s2.sign(payload));
    }

    #[test]
    fn test_ed25519_cross_verify_with_rct() {
        // Verifies that Ed25519Signer works with the RCT system
        // by checking sign/verify through the CryptoProvider trait
        let issuer = Ed25519Signer::new();
        let subject = Ed25519Signer::new();

        let payload = b"remote capability ticket payload";
        let sig = issuer.sign(payload);

        // Issuer's own DID verifies
        assert!(Ed25519Signer::verify(issuer.did(), payload, &sig));
        // Subject's DID does not verify issuer's signature
        assert!(!Ed25519Signer::verify(subject.did(), payload, &sig));
    }

    #[test]
    fn test_ed25519_large_payload() {
        let alice = Ed25519Signer::new();
        let payload = vec![0xABu8; 10000]; // 10KB payload
        let sig = alice.sign(&payload);
        assert!(Ed25519Signer::verify(alice.did(), &payload, &sig));
    }
}
