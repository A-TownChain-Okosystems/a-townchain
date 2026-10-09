// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Entkopplung von Oracle-Daten und Konsens-Wahrheit.
//! Regel: Oracle-Daten werden NIE direkt als Konsens-Wahrheit uebernommen,
//! sondern erst nach Staging, Verifikation, Staleness-Check und Median/Deviation-Filterung.

use crate::feed::{aggregate_median, check_deviation, FeedError, Report};
use crate::verification::{SignatureVerifierAdapter, VerificationError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedOraclePayload {
    pub key: String,
    pub report: Report,
    pub timestamp: u64,
    pub signature: Vec<u8>,
    pub pubkey: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusBridgeError {
    VerificationFailed(VerificationError),
    StaleData { current_time: u64, payload_time: u64, max_age: u64 },
    DeviationExceeded,
    InsufficientSources { have: usize, need: usize },
    EmergencyPaused,
}

/// Staging-Puffer für Oracle-Daten. Daten in diesem Puffer sind noch KEINE Konsens-Wahrheit.
#[derive(Debug, Clone)]
pub struct OracleStagingBuffer {
    pub key: String,
    pub max_staleness_seconds: u64,
    pub max_deviation: u64,
    pub min_sources: usize,
    pub emergency_paused: bool,
    staged_payloads: Vec<SignedOraclePayload>,
}

/// Konsens-Wahrheit (Consensus Truth State) - Ein gesicherter und verifizierter Zustand.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusStateValue {
    pub key: String,
    pub value: u64,
    pub committed_timestamp: u64,
    pub source_count: usize,
}

impl OracleStagingBuffer {
    pub fn new(key: &str, max_staleness: u64, max_dev: u64, min_src: usize) -> Self {
        Self {
            key: key.to_string(),
            max_staleness_seconds: max_staleness,
            max_deviation: max_dev,
            min_sources: min_src,
            emergency_paused: false,
            staged_payloads: Vec::new(),
        }
    }

    pub fn set_emergency_paused(&mut self, paused: bool) {
        self.emergency_paused = paused;
    }

    pub fn stage_payload(&mut self, payload: SignedOraclePayload) {
        self.staged_payloads.push(payload);
    }

    pub fn staged_count(&self) -> usize {
        self.staged_payloads.len()
    }

    pub fn clear_staged(&mut self) {
        self.staged_payloads.clear();
    }

    /// Transformation von ungeprueften Oracle-Daten in verifizierte Konsens-Wahrheit.
    pub fn commit_to_consensus_truth<V: SignatureVerifierAdapter>(
        &self,
        verifier: &V,
        current_time: u64,
    ) -> Result<ConsensusStateValue, ConsensusBridgeError> {
        if self.emergency_paused {
            return Err(ConsensusBridgeError::EmergencyPaused);
        }

        let mut valid_reports = Vec::new();

        for payload in &self.staged_payloads {
            // Staleness-Pruefung
            if current_time < payload.timestamp || current_time - payload.timestamp > self.max_staleness_seconds {
                return Err(ConsensusBridgeError::StaleData {
                    current_time,
                    payload_time: payload.timestamp,
                    max_age: self.max_staleness_seconds,
                });
            }

            // Signatur-Pruefung über Adapter (REQ-TUD-011)
            let is_valid = verifier
                .verify_signature(payload.key.as_bytes(), &payload.signature, &payload.pubkey)
                .map_err(ConsensusBridgeError::VerificationFailed)?;

            if !is_valid {
                return Err(ConsensusBridgeError::VerificationFailed(
                    VerificationError::InvalidSignature,
                ));
            }

            valid_reports.push(payload.report.clone());
        }

        if valid_reports.len() < self.min_sources {
            return Err(ConsensusBridgeError::InsufficientSources {
                have: valid_reports.len(),
                need: self.min_sources,
            });
        }

        if !check_deviation(&valid_reports, self.max_deviation) {
            return Err(ConsensusBridgeError::DeviationExceeded);
        }

        let median_val = aggregate_median(&valid_reports, self.min_sources).map_err(|e| match e {
            FeedError::TooFewSources { have, need } => ConsensusBridgeError::InsufficientSources { have, need },
        })?;

        Ok(ConsensusStateValue {
            key: self.key.clone(),
            value: median_val,
            committed_timestamp: current_time,
            source_count: valid_reports.len(),
        })
    }
}
