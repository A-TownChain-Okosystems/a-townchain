// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Schaltkreis-Statistik mit Validierung (R1-MVP).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitStats {
    pub gates: u64,
    pub public_inputs: u64,
    pub private_witnesses: u64,
}

impl CircuitStats {
    pub fn valid(&self) -> bool {
        self.gates > 0 && self.public_inputs + self.private_witnesses > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stats_validierung() {
        assert!(CircuitStats { gates: 1, public_inputs: 1, private_witnesses: 0 }.valid());
        assert!(!CircuitStats { gates: 0, public_inputs: 1, private_witnesses: 0 }.valid());
        assert!(!CircuitStats { gates: 1, public_inputs: 0, private_witnesses: 0 }.valid());
    }
}
