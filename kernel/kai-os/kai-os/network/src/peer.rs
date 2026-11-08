//! Peer-Identität: PeerId deterministisch aus dem Ed25519-Public-Key abgeleitet.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Deterministisches Keypair — aus Seed-Bytes konstruiert, kein RNG.
pub struct Keypair {
    signing: SigningKey,
}

impl Keypair {
    pub fn from_seed(seed: &[u8; 32]) -> Self {
        Self {
            signing: SigningKey::from_bytes(seed),
        }
    }

    pub fn public_bytes(&self) -> [u8; 32] {
        self.signing.verifying_key().to_bytes()
    }

    /// PeerId = Hex des Public Keys (deterministisch).
    pub fn peer_id(&self) -> String {
        crate::hex(&self.public_bytes())
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing.verifying_key()
    }

    /// Ed25519-Signatur — deterministisch (REQ-ENG-002).
    pub fn sign(&self, msg: &[u8]) -> [u8; 64] {
        let sig: Signature = self.signing.sign(msg);
        sig.to_bytes()
    }
}

/// Verifikation einer Signatur unter einem bekannten Public Key.
pub fn verify_signature(public: &[u8; 32], msg: &[u8], sig: &[u8; 64]) -> bool {
    let vk = match VerifyingKey::from_bytes(public) {
        Ok(vk) => vk,
        Err(_) => return false,
    };
    let signature = Signature::from_bytes(sig);
    vk.verify(msg, &signature).is_ok()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeerInfo {
    pub peer_id: String,
    pub addr: String,
    pub caps: Vec<String>,
    pub last_seen_tick: u64,
}

/// Lokaler Peer-Store: bekannte Peers, Dedup über PeerId.
#[derive(Default)]
pub struct PeerStore {
    peers: HashMap<String, PeerInfo>,
}

impl PeerStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Announce einpflegen — existierende Einträge werden aktualisiert (Dedup).
    pub fn upsert(&mut self, info: PeerInfo) {
        self.peers.insert(info.peer_id.clone(), info);
    }

    pub fn get(&self, peer_id: &str) -> Option<&PeerInfo> {
        self.peers.get(peer_id)
    }

    pub fn remove(&mut self, peer_id: &str) -> bool {
        self.peers.remove(peer_id).is_some()
    }

    pub fn len(&self) -> usize {
        self.peers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }

    pub fn all(&self) -> Vec<&PeerInfo> {
        let mut v: Vec<&PeerInfo> = self.peers.values().collect();
        v.sort_by(|a, b| a.peer_id.cmp(&b.peer_id)); // deterministische Reihenfolge
        v
    }

    /// Peers entfernen, die seit `max_age` Ticks nichts von sich gegeben haben.
    pub fn evict_stale(&mut self, now_tick: u64, max_age: u64) -> Vec<String> {
        let stale: Vec<String> = self
            .peers
            .values()
            .filter(|p| now_tick.saturating_sub(p.last_seen_tick) > max_age)
            .map(|p| p.peer_id.clone())
            .collect();
        for id in &stale {
            self.peers.remove(id);
        }
        stale
    }
}
