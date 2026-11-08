// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Asset transfer with capability checks
pub struct Transfer;
impl Transfer {
    pub fn validate(from_balance: u64, amount: u64, has_capability: bool) -> Result<(), String> {
        if !has_capability { return Err("No transfer capability".into()); }
        if from_balance < amount { return Err("Insufficient balance".into()); }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transfer() {
        assert!(Transfer::validate(100, 50, true).is_ok());
        assert!(Transfer::validate(100, 50, false).is_err());
        assert!(Transfer::validate(10, 50, true).is_err());
    }
}
