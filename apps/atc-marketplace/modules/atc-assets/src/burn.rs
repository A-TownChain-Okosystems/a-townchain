// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Token burning mechanism
pub struct BurnEngine { pub burned: u64 }
impl BurnEngine {
    pub fn new() -> Self { Self { burned: 0 } }
    pub fn burn(&mut self, balance: u64, amount: u64) -> Result<u64, String> {
        if amount > balance { return Err("Cannot burn more than balance".into()); }
        self.burned += amount;
        Ok(balance - amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_burn() {
        let mut b = BurnEngine::new();
        assert_eq!(b.burn(100, 30).unwrap(), 70);
        assert_eq!(b.burned, 30);
        assert!(b.burn(10, 20).is_err());
    }
}
