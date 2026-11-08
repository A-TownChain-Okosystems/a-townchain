//! Audit-Pipeline (KAI-CORE-RUNTIME-001 §8):
//! KAI Event -> Structured Event -> Audit Pipeline -> Integrity Hash -> Append-only Storage.
//! Kein Wall-Clock im Hash (Determinismus, REQ-ENG-002).

use serde::{Deserialize, Serialize};

pub const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub seq: u64,
    pub actor: String,
    pub kind: String,
    pub payload: String,
    pub prev_hash: String,
    pub hash: String,
}

#[derive(Default)]
pub struct AuditPipeline {
    events: Vec<AuditEvent>,
}

impl AuditPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, actor: &str, kind: &str, payload: &str) -> u64 {
        let seq = self.events.len() as u64 + 1;
        let prev_hash = self
            .events
            .last()
            .map(|e| e.hash.clone())
            .unwrap_or_else(|| GENESIS_HASH.to_string());
        let hash = crate::sha_hex(format!("{seq}|{actor}|{kind}|{payload}|{prev_hash}").as_bytes());
        self.events.push(AuditEvent {
            seq,
            actor: actor.into(),
            kind: kind.into(),
            payload: payload.into(),
            prev_hash,
            hash,
        });
        seq
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Integrität: komplette Hash-Chain nachrechnen.
    pub fn verify(&self) -> bool {
        let mut prev = GENESIS_HASH.to_string();
        for (i, e) in self.events.iter().enumerate() {
            if e.seq != (i as u64 + 1) || e.prev_hash != prev {
                return false;
            }
            let expect = crate::sha_hex(
                format!(
                    "{}|{}|{}|{}|{}",
                    e.seq, e.actor, e.kind, e.payload, e.prev_hash
                )
                .as_bytes(),
            );
            if e.hash != expect {
                return false;
            }
            prev = e.hash.clone();
        }
        true
    }

    /// Query nach Kind.
    pub fn by_kind(&self, kind: &str) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| e.kind == kind).collect()
    }

    /// Query nach Actor.
    pub fn by_actor(&self, actor: &str) -> Vec<&AuditEvent> {
        self.events.iter().filter(|e| e.actor == actor).collect()
    }
}
