//! Secure Session nach bestandenem Handshake: strikt steigende Sequenznummern
//! (Replay-Schutz) + signierte Envelopes (Integrität).

use crate::peer::verify_signature;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum SessionError {
    Replay { seq: u64, last: u64 },
    WrongSession { expected: String, got: String },
    BadSignature,
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::Replay { seq, last } => {
                write!(f, "session replay: seq {seq} <= last {last}")
            }
            SessionError::WrongSession { expected, got } => {
                write!(f, "wrong session: expected {expected}, got {got}")
            }
            SessionError::BadSignature => write!(f, "envelope signature invalid"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub session_id: String,
    pub seq: u64,
    pub from_peer_id: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedEnvelope {
    pub envelope: Envelope,
    pub signature: Vec<u8>,
}

pub struct SecureSession {
    pub session_id: String,
    pub local_peer_id: String,
    pub remote_peer_id: String,
    remote_public: [u8; 32],
    next_local_seq: u64,
    last_remote_seq: u64,
}

impl SecureSession {
    pub fn new(
        session_id: String,
        local_peer_id: &str,
        remote_peer_id: &str,
        remote_public: &[u8; 32],
    ) -> Self {
        Self {
            session_id,
            local_peer_id: local_peer_id.into(),
            remote_peer_id: remote_peer_id.into(),
            remote_public: *remote_public,
            next_local_seq: 1,
            last_remote_seq: 0,
        }
    }

    /// Ausgehende Nachricht: Envelope bauen + signieren.
    pub fn send(&mut self, keypair: &crate::peer::Keypair, payload: &[u8]) -> SignedEnvelope {
        let env = Envelope {
            session_id: self.session_id.clone(),
            seq: self.next_local_seq,
            from_peer_id: self.local_peer_id.clone(),
            payload: payload.to_vec(),
        };
        self.next_local_seq += 1;
        let signature = keypair.sign(&envelope_bytes(&env)).to_vec();
        SignedEnvelope {
            envelope: env,
            signature,
        }
    }

    /// Eingehende Nachricht: Signatur + Session + Sequenz prüfen (fail-closed).
    pub fn receive(&mut self, signed: &SignedEnvelope) -> Result<Vec<u8>, SessionError> {
        if signed.envelope.session_id != self.session_id {
            return Err(SessionError::WrongSession {
                expected: self.session_id.clone(),
                got: signed.envelope.session_id.clone(),
            });
        }
        if signed.envelope.seq <= self.last_remote_seq {
            return Err(SessionError::Replay {
                seq: signed.envelope.seq,
                last: self.last_remote_seq,
            });
        }
        let sig: [u8; 64] = match signed.signature.clone().try_into() {
            Ok(s) => s,
            Err(_) => return Err(SessionError::BadSignature),
        };
        if !verify_signature(&self.remote_public, &envelope_bytes(&signed.envelope), &sig) {
            return Err(SessionError::BadSignature);
        }
        self.last_remote_seq = signed.envelope.seq;
        Ok(signed.envelope.payload.clone())
    }
}

/// Kanonische Envelope-Bytes für die Signatur (deterministisch).
pub fn envelope_bytes(e: &Envelope) -> Vec<u8> {
    sha256_digest(e)
}

fn sha256_digest(e: &Envelope) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut input = Vec::with_capacity(e.payload.len() + 128);
    input.extend_from_slice(e.session_id.as_bytes());
    input.push(b'|');
    input.extend_from_slice(&e.seq.to_be_bytes());
    input.push(b'|');
    input.extend_from_slice(e.from_peer_id.as_bytes());
    input.push(b'|');
    input.extend_from_slice(&e.payload);
    Sha256::digest(&input).to_vec()
}
