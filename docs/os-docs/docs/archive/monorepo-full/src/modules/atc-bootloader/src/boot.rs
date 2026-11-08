// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Boot sequence and multi-stage init
pub struct BootSequence {
    stage: u8,
}

impl BootSequence {
    pub fn new() -> Self { Self { stage: 0 } }
    pub fn next_stage(&mut self) -> Result<u8, String> {
        self.stage += 1;
        if self.stage > 4 { return Err("Boot sequence complete".into()); }
        Ok(self.stage)
    }
    pub fn current_stage(&self) -> u8 { self.stage }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_boot() {
        let mut b = BootSequence::new();
        assert_eq!(b.current_stage(), 0);
        assert_eq!(b.next_stage().unwrap(), 1);
        assert_eq!(b.next_stage().unwrap(), 2);
    }
}
