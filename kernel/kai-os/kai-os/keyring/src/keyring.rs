//! Keyring: Ed25519-Keys mit strikter Isolation.
//! - Import NUR über Seed-Bytes (deterministisch, kein RNG — REQ-ENG-002)
//! - sign() liefert die Signatur, niemals das Secret
//! - public_key() liefert ausschließlich öffentliches Material
//! - revoke() entfernt den Key; danach ist Signieren fail-closed abgewiesen
//! - rotate() ersetzt das Secret; alte Signaturen bleiben mit dem alten
//!   (extern gespeicherten) Public Key prüfbar

use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyKind {
    /// Node-Identität (P2P, Block-Signaturen).
    NodeIdentity,
    /// Agent-Signaturen (Proposals, IPC).
    AgentSigning,
}

#[derive(Debug, PartialEq)]
pub enum KeyringError {
    UnknownKey(String),
    KeyRevoked(String),
    DuplicateKey(String),
}

impl std::fmt::Display for KeyringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyringError::UnknownKey(id) => write!(f, "unknown key '{id}' (fail-closed)"),
            KeyringError::KeyRevoked(id) => write!(f, "key '{id}' revoked (fail-closed)"),
            KeyringError::DuplicateKey(id) => write!(f, "key id '{id}' already exists"),
        }
    }
}

struct KeyEntry {
    signing: SigningKey,
    kind: KeyKind,
}

pub struct Keyring {
    keys: BTreeMap<String, KeyEntry>,
}

impl Keyring {
    pub fn new() -> Self {
        Self {
            keys: BTreeMap::new(),
        }
    }

    /// Key aus Seed importieren (deterministisch). Gibt DEN Public Key zurück —
    /// das Secret verlässt die Boundary nicht.
    pub fn import_seed(
        &mut self,
        key_id: &str,
        kind: KeyKind,
        seed: &[u8; 32],
    ) -> Result<[u8; 32], KeyringError> {
        if self.keys.contains_key(key_id) {
            return Err(KeyringError::DuplicateKey(key_id.to_string()));
        }
        let signing = SigningKey::from_bytes(seed);
        let public = signing.verifying_key().to_bytes();
        self.keys
            .insert(key_id.to_string(), KeyEntry { signing, kind });
        Ok(public)
    }

    /// Öffentliche Information: Key-ID, Public Key, Art.
    pub fn list(&self) -> Vec<(String, [u8; 32], KeyKind)> {
        self.keys
            .iter()
            .map(|(id, e)| (id.clone(), e.signing.verifying_key().to_bytes(), e.kind))
            .collect()
    }

    pub fn public_key(&self, key_id: &str) -> Result<[u8; 32], KeyringError> {
        match self.keys.get(key_id) {
            Some(e) => Ok(e.signing.verifying_key().to_bytes()),
            None => Err(KeyringError::UnknownKey(key_id.to_string())),
        }
    }

    /// Signieren — deterministisch (Ed25519), Secret bleibt intern.
    pub fn sign(&self, key_id: &str, msg: &[u8]) -> Result<[u8; 64], KeyringError> {
        match self.keys.get(key_id) {
            Some(e) => {
                use ed25519_dalek::Signature;
                let sig: Signature = e.signing.sign(msg);
                Ok(sig.to_bytes())
            }
            None => Err(KeyringError::UnknownKey(key_id.to_string())),
        }
    }

    /// Key entfernen — danach signiert er nicht mehr. Das Secret wird beim
    /// Drop gelöscht (ed25519-dalek Zeroize-on-Drop).
    pub fn revoke(&mut self, key_id: &str) -> Result<(), KeyringError> {
        match self.keys.remove(key_id) {
            Some(_) => Ok(()), // Drop -> Secret geloescht
            None => Err(KeyringError::UnknownKey(key_id.to_string())),
        }
    }

    /// Rotation: altes Secret raus (gelöscht), neues rein. Gibt den NEUEN
    /// Public Key zurück. Alte Signaturen bleiben mit dem extern
    /// aufbewahrten alten Public Key prüfbar.
    pub fn rotate(&mut self, key_id: &str, new_seed: &[u8; 32]) -> Result<[u8; 32], KeyringError> {
        let kind = match self.keys.get(key_id) {
            Some(e) => e.kind,
            None => return Err(KeyringError::UnknownKey(key_id.to_string())),
        };
        self.keys.remove(key_id); // Drop -> altes Secret geloescht
        let signing = SigningKey::from_bytes(new_seed);
        let public = signing.verifying_key().to_bytes();
        self.keys
            .insert(key_id.to_string(), KeyEntry { signing, kind });
        Ok(public)
    }
}

impl Default for Keyring {
    fn default() -> Self {
        Self::new()
    }
}

/// Verifikation einer Keyring-Signatur unter einem Public Key (public utility).
pub fn verify(public: &[u8; 32], msg: &[u8], sig: &[u8; 64]) -> bool {
    let vk = match VerifyingKey::from_bytes(public) {
        Ok(vk) => vk,
        Err(_) => return false,
    };
    let signature = ed25519_dalek::Signature::from_bytes(sig);
    vk.verify(msg, &signature).is_ok()
}
