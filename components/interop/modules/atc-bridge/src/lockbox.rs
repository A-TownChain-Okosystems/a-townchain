// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Bridge lockbox for custodial assets
use std::collections::HashMap;

pub type Amount = u128;

pub struct Lockbox {
    deposits: HashMap<String, Amount>,
    withdrawals: HashMap<String, Amount>,
}

impl Lockbox {
    pub fn new() -> Self {
        Self {
            deposits: HashMap::new(),
            withdrawals: HashMap::new(),
        }
    }

    pub fn deposit(&mut self, user: &str, amount: Amount) -> Result<(), String> {
        let entry = self.deposits.entry(user.into()).or_insert(0);
        *entry = entry
            .checked_add(amount)
            .ok_or_else(|| "lockbox deposit overflow".to_string())?;
        Ok(())
    }

    pub fn withdraw(&mut self, user: &str, amount: Amount) -> Result<(), String> {
        let balance = *self.deposits.get(user).unwrap_or(&0);
        let withdrawn = *self.withdrawals.get(user).unwrap_or(&0);
        let available = balance
            .checked_sub(withdrawn)
            .ok_or_else(|| "lockbox withdrawal state underflow".to_string())?;
        if available < amount {
            return Err("Insufficient lockbox balance".into());
        }

        let entry = self.withdrawals.entry(user.into()).or_insert(0);
        *entry = entry
            .checked_add(amount)
            .ok_or_else(|| "lockbox withdrawal overflow".to_string())?;
        Ok(())
    }

    pub fn balance(&self, user: &str) -> Result<Amount, String> {
        let deposited = *self.deposits.get(user).unwrap_or(&0);
        let withdrawn = *self.withdrawals.get(user).unwrap_or(&0);
        deposited
            .checked_sub(withdrawn)
            .ok_or_else(|| "lockbox balance invariant violated".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lockbox() {
        let mut lb = Lockbox::new();
        lb.deposit("alice", 1_000).unwrap();
        assert_eq!(lb.balance("alice").unwrap(), 1_000);
        assert!(lb.withdraw("alice", 600).is_ok());
        assert_eq!(lb.balance("alice").unwrap(), 400);
        assert!(lb.withdraw("alice", 500).is_err());
    }

    #[test]
    fn u128_max_deposit_is_supported() {
        let mut lb = Lockbox::new();
        lb.deposit("alice", u128::MAX).unwrap();
        assert_eq!(lb.balance("alice").unwrap(), u128::MAX);
    }

    #[test]
    fn deposit_overflow_is_rejected_atomically() {
        let mut lb = Lockbox::new();
        lb.deposit("alice", u128::MAX).unwrap();
        assert!(lb.deposit("alice", 1).is_err());
        assert_eq!(lb.balance("alice").unwrap(), u128::MAX);
    }

    #[test]
    fn withdrawal_overflow_is_rejected_atomically() {
        let mut lb = Lockbox::new();
        lb.deposit("alice", u128::MAX).unwrap();
        lb.withdraw("alice", u128::MAX).unwrap();
        assert!(lb.withdraw("alice", 1).is_err());
        assert_eq!(lb.balance("alice").unwrap(), 0);
    }
}
