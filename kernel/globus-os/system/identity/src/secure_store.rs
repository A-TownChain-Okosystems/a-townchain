//! Authenticated encryption boundary for durable identity blobs.
//!
//! This module deliberately does not implement a filesystem or hardware key store. The key
//! provider is an explicit dependency so production wiring can bind it to ShivaCore/TPM/TEE.
//! Backends must provide atomic replacement and monotonic generation tracking to reject rollback.

use chacha20poly1305::{
    AeadCore, KeyInit, XChaCha20Poly1305,
    aead::{Aead, OsRng},
};
use zeroize::Zeroizing;

const MAGIC: &[u8; 4] = b"GID1";
const FORMAT_VERSION: u8 = 2;
const NONCE_LEN: usize = 24;
const HEADER_LEN: usize = 4 + 1 + 8 + NONCE_LEN;

pub trait IdentityKeyProvider {
    fn load_key(&self) -> Result<Zeroizing<[u8; 32]>, SecureStoreError>;
}

/// Opaque blob storage with atomic replacement and durable monotonic generations.
pub trait BlobStore {
    fn load(&self) -> Result<Option<Vec<u8>>, SecureStoreError>;
    fn replace(&mut self, blob: &[u8], generation: u64) -> Result<(), SecureStoreError>;
    fn generation(&self) -> Result<u64, SecureStoreError>;
    fn delete(&mut self) -> Result<(), SecureStoreError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecureStoreError {
    KeyUnavailable,
    EncryptionFailed,
    DecryptionFailed,
    InvalidEnvelope,
    IntegrityFailure,
    Conflict,
    RollbackDetected { stored: u64, incoming: u64 },
    GenerationOverflow,
    BackendUnavailable,
}

/// Encrypts an opaque identity record using XChaCha20-Poly1305.
///
/// The envelope contains a format marker, monotonic generation, random nonce and ciphertext.
/// The generation and envelope header are authenticated as associated data.
pub struct EncryptedBlobStore<B, K> {
    backend: B,
    keys: K,
}

impl<B, K> EncryptedBlobStore<B, K> {
    pub fn new(backend: B, keys: K) -> Self {
        Self { backend, keys }
    }
    pub fn into_parts(self) -> (B, K) {
        (self.backend, self.keys)
    }
}

impl<B: BlobStore, K: IdentityKeyProvider> EncryptedBlobStore<B, K> {
    pub fn load_plaintext(&self) -> Result<Option<Zeroizing<Vec<u8>>>, SecureStoreError> {
        let Some(blob) = self.backend.load()? else {
            return Ok(None);
        };
        let current = self.backend.generation()?;
        decrypt(&blob, &self.keys, current)
    }

    pub fn replace_plaintext(&mut self, plaintext: &[u8]) -> Result<u64, SecureStoreError> {
        let current = self.backend.generation()?;
        let generation = current
            .checked_add(1)
            .ok_or(SecureStoreError::GenerationOverflow)?;
        let blob = encrypt(plaintext, &self.keys, generation)?;
        self.backend.replace(&blob, generation)?;
        Ok(generation)
    }

    pub fn delete(&mut self) -> Result<(), SecureStoreError> {
        self.backend.delete()
    }
}

fn encrypt<K: IdentityKeyProvider>(
    plaintext: &[u8],
    keys: &K,
    generation: u64,
) -> Result<Vec<u8>, SecureStoreError> {
    let key = keys.load_key()?;
    let cipher =
        XChaCha20Poly1305::new_from_slice(&*key).map_err(|_| SecureStoreError::KeyUnavailable)?;
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let mut header = Vec::with_capacity(HEADER_LEN);
    header.extend_from_slice(MAGIC);
    header.push(FORMAT_VERSION);
    header.extend_from_slice(&generation.to_le_bytes());
    header.extend_from_slice(&nonce);
    let ciphertext = cipher
        .encrypt(
            &nonce,
            chacha20poly1305::aead::Payload {
                msg: plaintext,
                aad: &header,
            },
        )
        .map_err(|_| SecureStoreError::EncryptionFailed)?;
    header.extend_from_slice(&ciphertext);
    Ok(header)
}

fn decrypt<K: IdentityKeyProvider>(
    blob: &[u8],
    keys: &K,
    expected_generation: u64,
) -> Result<Option<Zeroizing<Vec<u8>>>, SecureStoreError> {
    if blob.len() < HEADER_LEN + 16 {
        return Err(SecureStoreError::InvalidEnvelope);
    }
    if &blob[..4] != MAGIC || blob[4] != FORMAT_VERSION {
        return Err(SecureStoreError::InvalidEnvelope);
    }
    let generation = u64::from_le_bytes(
        blob[5..13]
            .try_into()
            .map_err(|_| SecureStoreError::InvalidEnvelope)?,
    );
    if generation != expected_generation {
        return Err(SecureStoreError::RollbackDetected {
            stored: expected_generation,
            incoming: generation,
        });
    }
    let nonce: [u8; NONCE_LEN] = blob[13..37]
        .try_into()
        .map_err(|_| SecureStoreError::InvalidEnvelope)?;
    let header = &blob[..HEADER_LEN];
    let ciphertext = &blob[HEADER_LEN..];
    let key = keys.load_key()?;
    let cipher =
        XChaCha20Poly1305::new_from_slice(&*key).map_err(|_| SecureStoreError::KeyUnavailable)?;
    let plaintext = cipher
        .decrypt(
            (&nonce).into(),
            chacha20poly1305::aead::Payload {
                msg: ciphertext,
                aad: header,
            },
        )
        .map_err(|_| SecureStoreError::DecryptionFailed)?;
    Ok(Some(Zeroizing::new(plaintext)))
}

#[derive(Debug, Default)]
pub struct InMemoryBlobStore {
    blob: Option<Vec<u8>>,
    generation: u64,
}

impl BlobStore for InMemoryBlobStore {
    fn load(&self) -> Result<Option<Vec<u8>>, SecureStoreError> {
        Ok(self.blob.clone())
    }

    fn replace(&mut self, blob: &[u8], generation: u64) -> Result<(), SecureStoreError> {
        if generation <= self.generation && self.blob.is_some() {
            return Err(SecureStoreError::RollbackDetected {
                stored: self.generation,
                incoming: generation,
            });
        }
        self.blob = Some(blob.to_vec());
        self.generation = generation;
        Ok(())
    }

    fn generation(&self) -> Result<u64, SecureStoreError> {
        Ok(self.generation)
    }

    fn delete(&mut self) -> Result<(), SecureStoreError> {
        if let Some(mut blob) = self.blob.take() {
            zeroize_bytes(&mut blob);
        }
        self.generation = self
            .generation
            .checked_add(1)
            .ok_or(SecureStoreError::GenerationOverflow)?;
        Ok(())
    }
}

fn zeroize_bytes(bytes: &mut [u8]) {
    for byte in bytes {
        *byte = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestKey(Zeroizing<[u8; 32]>);
    impl IdentityKeyProvider for TestKey {
        fn load_key(&self) -> Result<Zeroizing<[u8; 32]>, SecureStoreError> {
            Ok(self.0.clone())
        }
    }
    fn store() -> EncryptedBlobStore<InMemoryBlobStore, TestKey> {
        EncryptedBlobStore::new(
            InMemoryBlobStore::default(),
            TestKey(Zeroizing::new([7u8; 32])),
        )
    }
    #[test]
    fn round_trip_decrypts_and_increments_generation() {
        let mut store = store();
        assert_eq!(store.replace_plaintext(b"identity-record").unwrap(), 1);
        assert_eq!(
            &*store.load_plaintext().unwrap().unwrap(),
            b"identity-record"
        );
        assert_eq!(store.replace_plaintext(b"identity-record-2").unwrap(), 2);
    }
    #[test]
    fn random_nonce_produces_distinct_ciphertexts() {
        let mut first = store();
        let mut second = store();
        first.replace_plaintext(b"same").unwrap();
        second.replace_plaintext(b"same").unwrap();
        let (a, _) = first.into_parts();
        let (b, _) = second.into_parts();
        assert_ne!(a.load().unwrap(), b.load().unwrap());
    }
    #[test]
    fn tampering_is_rejected() {
        let mut store = store();
        store.replace_plaintext(b"identity-record").unwrap();
        let (mut backend, keys) = store.into_parts();
        let mut blob = backend.load().unwrap().unwrap();
        *blob.last_mut().unwrap() ^= 1;
        let generation = backend.generation().unwrap();
        backend.blob = Some(blob);
        backend.generation = generation;
        let store = EncryptedBlobStore::new(backend, keys);
        assert_eq!(
            store.load_plaintext().unwrap_err(),
            SecureStoreError::DecryptionFailed
        );
    }
    #[test]
    fn wrong_key_is_rejected() {
        let mut store = store();
        store.replace_plaintext(b"identity-record").unwrap();
        let (backend, _) = store.into_parts();
        let wrong = TestKey(Zeroizing::new([8u8; 32]));
        let store = EncryptedBlobStore::new(backend, wrong);
        assert_eq!(
            store.load_plaintext().unwrap_err(),
            SecureStoreError::DecryptionFailed
        );
    }
    #[test]
    fn rollback_generation_is_rejected() {
        let mut backend = InMemoryBlobStore::default();
        backend.replace(b"one", 2).unwrap();
        assert_eq!(
            backend.replace(b"old", 1).unwrap_err(),
            SecureStoreError::RollbackDetected {
                stored: 2,
                incoming: 1
            }
        );
    }
    #[test]
    fn delete_removes_blob_and_advances_generation() {
        let mut store = store();
        store.replace_plaintext(b"identity-record").unwrap();
        store.delete().unwrap();
        assert_eq!(store.load_plaintext().unwrap(), None);
        assert_eq!(store.backend.generation().unwrap(), 2);
    }
}
