// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Proof of History — vereinfachte Tick-Kette (ATC-CONSENSUS-301, MVP).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tick {
    pub slot: u64,
    pub hash: u64,
}

/// FNV-1a 64-Bit-Hash (MVP; kein kryptografischer Hash — siehe Moduldoku).
pub fn fnv1a(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in data {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

pub struct PohChain {
    ticks: Vec<Tick>,
}

impl PohChain {
    pub fn genesis(seed: u64) -> Self {
        PohChain { ticks: vec![Tick { slot: 0, hash: seed }] }
    }

    pub fn tick(&mut self) -> Tick {
        let prev = self.ticks.last().unwrap();
        let next = Tick {
            slot: prev.slot + 1,
            hash: fnv1a(&prev.hash.to_le_bytes()),
        };
        self.ticks.push(next.clone());
        next
    }

    pub fn verify(&self) -> bool {
        for i in 1..self.ticks.len() {
            let prev = &self.ticks[i - 1];
            let cur = &self.ticks[i];
            if cur.slot != prev.slot + 1 || cur.hash != fnv1a(&prev.hash.to_le_bytes()) {
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
        let t = c.tick();
        assert_eq!(t.slot, 1);
        assert_eq!(c.ticks().len(), 2);
        assert!(c.verify());
    }

    #[test]
    fn manipulation_wird_erkannt() {
        let mut c = PohChain::genesis(1);
        for _ in 0..10 { c.tick(); }
        assert!(c.verify());
        c.ticks[3].hash ^= 1;
        assert!(!c.verify());
    }

    #[test]
    fn determinismus() {
        let mut a = PohChain::genesis(7);
        let mut b = PohChain::genesis(7);
        for _ in 0..50 { a.tick(); b.tick(); }
        assert_eq!(a.ticks(), b.ticks());
    }
}
