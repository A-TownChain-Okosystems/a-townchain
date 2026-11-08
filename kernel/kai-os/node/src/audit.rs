//! Append-only Audit-Logger mit Integrity-Hash-Chain (KAI-CORE-RUNTIME-001 §8):
//! KAI Event -> Structured Event -> Audit Pipeline -> Integrity Hash -> Append-only Storage.
//! Keine Wall-Clock-Timestamps im Hash (Determinismus, REQ-ENG-002).

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::PathBuf;

pub const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuditEvent {
    pub seq: u64,
    pub kind: String,
    pub detail: String,
    pub prev_hash: String,
    pub hash: String,
}

pub struct AuditLogger {
    events: Vec<AuditEvent>,
    sink: Option<PathBuf>,
}

fn sha256_hex(input: &str) -> String {
    let d = Sha256::digest(input.as_bytes());
    d.iter().map(|b| format!("{b:02x}")).collect()
}

impl AuditLogger {
    /// In-Memory-Logger (Tests).
    pub fn in_memory() -> Self {
        Self {
            events: Vec::new(),
            sink: None,
        }
    }

    /// Append-only File-Logger (Produktion).
    pub fn to_file(path: PathBuf) -> std::io::Result<Self> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        Ok(Self {
            events: Vec::new(),
            sink: Some(path),
        })
    }

    pub fn record(&mut self, kind: &str, detail: &str) -> Result<&AuditEvent, std::io::Error> {
        let seq = self.events.len() as u64 + 1;
        let prev_hash = self
            .events
            .last()
            .map(|e| e.hash.clone())
            .unwrap_or_else(|| GENESIS_HASH.to_string());
        let hash = sha256_hex(&format!("{seq}|{kind}|{detail}|{prev_hash}"));
        let ev = AuditEvent {
            seq,
            kind: kind.into(),
            detail: detail.into(),
            prev_hash,
            hash,
        };
        if let Some(path) = &self.sink {
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;
            writeln!(
                f,
                "{}",
                serde_json::to_string(&ev).map_err(std::io::Error::other)?
            )?;
        }
        self.events.push(ev);
        // Kein unwrap in Security-kritischem Code (Owner-Regel):
        // wir haben soeben gepusht, der Index ist beweisbar gueltig.
        let last = self.events.len() - 1;
        Ok(&self.events[last])
    }

    pub fn events(&self) -> &[AuditEvent] {
        &self.events
    }

    /// Integritätsprüfung: Hash-Chain komplett nachrechnen.
    /// Test-Zugriff für Manipulations-Tests.
    pub fn events_mut(&mut self) -> &mut [AuditEvent] {
        &mut self.events
    }

    pub fn verify_chain(&self) -> bool {
        let mut prev = GENESIS_HASH.to_string();
        for (i, e) in self.events.iter().enumerate() {
            if e.seq != (i as u64 + 1) || e.prev_hash != prev {
                return false;
            }
            if e.hash
                != sha256_hex(&format!(
                    "{}|{}|{}|{}",
                    e.seq, e.kind, e.detail, e.prev_hash
                ))
            {
                return false;
            }
            prev = e.hash.clone();
        }
        true
    }
}
