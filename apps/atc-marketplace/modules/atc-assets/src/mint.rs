// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Minting logic for ATC-8300/9000 tokens
pub struct MintEngine { pub max_supply: u64, pub minted: u64 }
impl MintEngine {
    pub fn new(max_supply: u64) -> Self { Self { max_supply, minted: 0 } }
    pub fn mint(&mut self, amount: u64) -> Result<u64, String> {
        if self.minted + amount > self.max_supply { return Err("Exceeds max supply".into()); }
        self.minted += amount;
        Ok(self.minted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mint() {
        let mut m = MintEngine::new(1000);
        assert_eq!(m.mint(500).unwrap(), 500);
        assert_eq!(m.mint(500).unwrap(), 1000);
        assert!(m.mint(1).is_err());
    }
}
