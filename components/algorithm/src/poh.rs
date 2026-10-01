// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Proof of History — vereinfachte Tick-Kette (ATC-CONSENSUS-301, MVP).
//!
//! The MVP uses SHA-256 as a deterministic sequencing primitive. The
//! final PoH cryptographic proof contract remains SPEC-DRAFT until freeze.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tick {
    pub slot: u64,
    pub hash: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PohError {
    SlotOverflow,
    MissingGenesis,
}

impl std::fmt::Display for PohError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PohError::SlotOverflow => write!(f, "PoH slot counter overflow"),
            PohError::MissingGenesis => write!(f, "PoH chain has no genesis tick"),
        }
    }
}

impl std::error::Error for PohError {}

/// SHA-256 sequencing primitive for the consensus MVP boundary.
/// The final consensus hash contract remains SPEC-DRAFT until freeze.
pub fn hash64(data: &[u8]) -> u64 {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(data);
    u64::from_be_bytes(digest[..8].try_into().expect("fixed SHA-256 prefix"))
}

pub struct PohChain {
    ticks: Vec<Tick>,
}

impl PohChain {
    pub fn genesis(seed: u64) -> Self {
        PohChain { ticks: vec![Tick { slot: 0, hash: seed }] }
    }

    /// Append the next PoH tick and reject invalid chain state explicitly.
    pub fn tick(&mut self) -> Result<Tick, PohError> {
        let Some(prev) = self.ticks.last() else {
            return Err(PohError::MissingGenesis);
        };
        let slot = prev.slot.checked_add(1).ok_or(PohError::SlotOverflow)?;
        let next = Tick {
            slot,
            hash: hash64(&prev.hash.to_le_bytes()),
        };
        self.ticks.push(next.clone());
        Ok(next)
    }

    pub fn verify(&self) -> bool {
        for i in 1..self.ticks.len() {
            let prev = &self.ticks[i - 1];
            let cur = &self.ticks[i];
            let Some(expected_slot) = prev.slot.checked_add(1) else {
                return false;
            };
            if cur.slot != expected_slot || cur.hash != hash64(&prev.hash.to_le_bytes()) {
                return false;
            }
        }
        true
    }

    pub fn ticks(&self) -> &[Tick] {
        &self.ticks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_und_tick_kette() {
        let mut c = PohChain::genesis(42);
        assert_eq!(c.ticks().len(), 1);
        let t = c.tick().expect("first tick must succeed");
        assert_eq!(t.slot, 1);
        assert_eq!(c.ticks().len(), 2);
        assert!(c.verify());
    }

    #[test]
    fn manipulation_wird_erkannt() {
        let mut c = PohChain::genesis(1);
        for _ in 0..10 {
            c.tick().expect("tick must succeed");
        }
        assert!(c.verify());
        c.ticks[3].hash ^= 1;
        assert!(!c.verify());
    }

    #[test]
    fn determinismus() {
        let mut a = PohChain::genesis(7);
        let mut b = PohChain::genesis(7);
        for _ in 0..50 {
            a.tick().expect("tick must succeed");
            b.tick().expect("tick must succeed");
        }
        assert_eq!(a.ticks(), b.ticks());
    }

    #[test]
    fn slot_overflow_is_rejected() {
        let mut c = PohChain::genesis(7);
        c.ticks[0].slot = u64::MAX;
        assert_eq!(c.tick(), Err(PohError::SlotOverflow));
        assert!(!c.verify());
        assert_eq!(c.ticks().len(), 1);
    }

    #[test]
    fn missing_genesis_is_rejected() {
        let mut c = PohChain::genesis(7);
        c.ticks.clear();
        assert_eq!(c.tick(), Err(PohError::MissingGenesis));
        assert!(c.verify());
    }
}
