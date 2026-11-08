//! Peer-Authentifizierung: Challenge-Response mit Ed25519 + Replay-Schutz.
//! Nonces müssen pro Initiator strikt monoton steigern — Wiederholung = Replay.

use crate::peer::verify_signature;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

/// Domain-Separation der Challenge-Payload (SEC-P0-008).
pub const CHALLENGE_DOMAIN: &[u8] = b"ATC-KAI-CHALLENGE-V1";
/// Bindung an die ATC-Chain — verhindert Cross-Chain-Replay.
pub const CHALLENGE_CHAIN_ID: u64 = 658_467;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub from_peer_id: String,
    pub to_peer_id: String,
    pub nonce: u64,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeResponse {
    pub from_peer_id: String,
    pub to_peer_id: String,
    pub nonce: u64,
    pub signature: Vec<u8>,
}

#[derive(Debug)]
pub enum AuthError {
    Replay { nonce: u64, last: u64 },
    WrongPeer { expected: String, got: String },
    BadSignature,
    WrongChain { expected: u64, got: u64 },
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::Replay { nonce, last } => write!(f, "replay: nonce {nonce} <= last {last}"),
            AuthError::WrongPeer { expected, got } => {
                write!(f, "wrong peer: expected {expected}, got {got}")
            }
            AuthError::BadSignature => write!(f, "signature verification failed"),
            AuthError::WrongChain { expected, got } => {
                write!(f, "wrong chain: expected {expected}, got {got}")
            }
        }
    }
}

fn put_field(out: &mut Vec<u8>, value: &str) {
    out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    out.extend_from_slice(value.as_bytes());
}

/// Kanonische Challenge-Payload: Domain, Chain-ID, laengenpraefigierte
/// Peer-IDs und Nonce. Laengenpraefixe verhindern Feld-Ambiguitaet
/// (frueher kollidierten "a|b"+"c" und "a"+"b|c").
pub fn challenge_payload(c: &Challenge) -> Vec<u8> {
    let mut b = Vec::with_capacity(64 + c.from_peer_id.len() + c.to_peer_id.len());
    b.extend_from_slice(CHALLENGE_DOMAIN);
    b.extend_from_slice(&c.chain_id.to_be_bytes());
    put_field(&mut b, &c.from_peer_id);
    put_field(&mut b, &c.to_peer_id);
    b.extend_from_slice(&c.nonce.to_be_bytes());
    Sha256::digest(&b).to_vec()
}

/// Initiator-seitige Challenge-Erzeugung mit strikt monotoner Nonce.
pub struct ChallengeGenerator {
    last_nonce: u64,
}

impl Default for ChallengeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl ChallengeGenerator {
    pub fn new() -> Self {
        Self { last_nonce: 0 }
    }

    pub fn next(&mut self, from: &str, to: &str) -> Challenge {
        self.last_nonce += 1;
        Challenge {
            from_peer_id: from.into(),
            to_peer_id: to.into(),
            nonce: self.last_nonce,
            chain_id: CHALLENGE_CHAIN_ID,
        }
    }
}

/// Responder: Challenge mit eigenem Key beantworten.
pub fn respond(challenge: &Challenge, responder: &crate::peer::Keypair) -> ChallengeResponse {
    let payload = challenge_payload(challenge);
    let signature = responder.sign(&payload).to_vec();
    ChallengeResponse {
        from_peer_id: challenge.to_peer_id.clone(),
        to_peer_id: challenge.from_peer_id.clone(),
        nonce: challenge.nonce,
        signature,
    }
}

/// Initiator: Response verifizieren — PeerId, Nonce-Monotonie und Signatur.
pub struct AuthRegistry {
    last_nonce: HashMap<String, u64>,
}

impl Default for AuthRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthRegistry {
    pub fn new() -> Self {
        Self {
            last_nonce: HashMap::new(),
        }
    }

    pub fn verify(
        &mut self,
        challenge: &Challenge,
        response: &ChallengeResponse,
        responder_public: &[u8; 32],
    ) -> Result<(), AuthError> {
        if challenge.chain_id != CHALLENGE_CHAIN_ID {
            return Err(AuthError::WrongChain {
                expected: CHALLENGE_CHAIN_ID,
                got: challenge.chain_id,
            });
        }
        if response.to_peer_id != challenge.from_peer_id
            || response.from_peer_id != challenge.to_peer_id
        {
            return Err(AuthError::WrongPeer {
                expected: challenge.to_peer_id.clone(),
                got: response.from_peer_id.clone(),
            });
        }
        let last = self
            .last_nonce
            .get(&response.from_peer_id)
            .copied()
            .unwrap_or(0);
        if response.nonce <= last {
            return Err(AuthError::Replay {
                nonce: response.nonce,
                last,
            });
        }
        let payload = challenge_payload(challenge);
        let sig: [u8; 64] = match response.signature.clone().try_into() {
            Ok(s) => s,
            Err(_) => return Err(AuthError::BadSignature),
        };
        if !verify_signature(responder_public, &payload, &sig) {
            return Err(AuthError::BadSignature);
        }
        self.last_nonce
            .insert(response.from_peer_id.clone(), response.nonce);
        Ok(())
    }
}

/// Session-ID: deterministische Ableitung aus den beiden Peers + Nonce.
pub fn session_id(a: &str, b: &str, nonce: u64) -> String {
    let mut buf = Vec::with_capacity(32 + a.len() + b.len());
    buf.extend_from_slice(b"ATC-KAI-SESSION-V1");
    put_field(&mut buf, a);
    put_field(&mut buf, b);
    buf.extend_from_slice(&nonce.to_be_bytes());
    crate::hex(&Sha256::digest(&buf))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn laengenpraefix_trennt_felder_eindeutig() {
        let a = Challenge { from_peer_id: "a|b".into(), to_peer_id: "c".into(), nonce: 1, chain_id: CHALLENGE_CHAIN_ID };
        let b = Challenge { from_peer_id: "a".into(), to_peer_id: "b|c".into(), nonce: 1, chain_id: CHALLENGE_CHAIN_ID };
        assert_ne!(challenge_payload(&a), challenge_payload(&b), "Pipe-Ambiguitaet muss ausgeschlossen sein");
    }

    #[test]
    fn payload_ist_an_die_chain_gebunden() {
        let a = Challenge { from_peer_id: "p1".into(), to_peer_id: "p2".into(), nonce: 1, chain_id: CHALLENGE_CHAIN_ID };
        let mut b = a.clone();
        b.chain_id = 1;
        assert_ne!(challenge_payload(&a), challenge_payload(&b));
    }

    #[test]
    fn fremde_chain_wird_abgelehnt() {
        let mut reg = AuthRegistry::new();
        let foreign = Challenge { from_peer_id: "p1".into(), to_peer_id: "p2".into(), nonce: 1, chain_id: 1 };
        let dummy = ChallengeResponse { from_peer_id: "p2".into(), to_peer_id: "p1".into(), nonce: 1, signature: vec![0u8; 64] };
        assert!(matches!(reg.verify(&foreign, &dummy, &[0u8; 32]), Err(AuthError::WrongChain { .. })));
    }

    #[test]
    fn session_id_ist_domain_separiert() {
        assert_ne!(session_id("a|b", "c", 1), session_id("a", "b|c", 1));
        assert_eq!(session_id("a", "b", 1), session_id("a", "b", 1));
    }
}
