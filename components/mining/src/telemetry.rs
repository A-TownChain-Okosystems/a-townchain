// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Telemetry & MinerWatcherGPT event interface with double-claim protection.

use std::collections::HashSet;
use crate::miner::MiningError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TelemetryEvent {
    JobDispatched { job_id: u64 },
    ProofSubmitted { job_id: u64, nonce: u64 },
    RewardClaimed { claim_id: [u8; 32], amount: u128 },
    DoubleClaimAttempted { claim_id: [u8; 32] },
    AnomalyDetected { description: String },
}

#[derive(Debug, Default)]
pub struct DoubleClaimGuard {
    seen_claims: HashSet<[u8; 32]>,
}

impl DoubleClaimGuard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_claim(&mut self, claim_id: [u8; 32]) -> Result<(), MiningError> {
        if self.seen_claims.contains(&claim_id) {
            Err(MiningError::DoubleClaim)
        } else {
            self.seen_claims.insert(claim_id);
            Ok(())
        }
    }

    pub fn is_claimed(&self, claim_id: &[u8; 32]) -> bool {
        self.seen_claims.contains(claim_id)
    }
}

pub struct TelemetryEmitter {
    events: Vec<TelemetryEvent>,
    guard: DoubleClaimGuard,
}

impl TelemetryEmitter {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            guard: DoubleClaimGuard::new(),
        }
    }

    pub fn emit(&mut self, event: TelemetryEvent) {
        self.events.push(event);
    }

    pub fn claim_reward(&mut self, claim_id: [u8; 32], amount: u128) -> Result<(), MiningError> {
        if let Err(e) = self.guard.register_claim(claim_id) {
            self.emit(TelemetryEvent::DoubleClaimAttempted { claim_id });
            return Err(e);
        }
        self.emit(TelemetryEvent::RewardClaimed { claim_id, amount });
        Ok(())
    }

    pub fn events(&self) -> &[TelemetryEvent] {
        &self.events
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_claim_protection() {
        let mut guard = DoubleClaimGuard::new();
        let claim_id = [0x42u8; 32];

        assert!(guard.register_claim(claim_id).is_ok());
        assert!(guard.is_claimed(&claim_id));

        assert_eq!(guard.register_claim(claim_id), Err(MiningError::DoubleClaim));
    }

    #[test]
    fn test_telemetry_emitter() {
        let mut emitter = TelemetryEmitter::new();
        let claim_id = [0x01u8; 32];

        assert!(emitter.claim_reward(claim_id, 5000).is_ok());
        assert_eq!(emitter.claim_reward(claim_id, 5000), Err(MiningError::DoubleClaim));

        let events = emitter.events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0], TelemetryEvent::RewardClaimed { claim_id, amount: 5000 });
        assert_eq!(events[1], TelemetryEvent::DoubleClaimAttempted { claim_id });
    }
}
