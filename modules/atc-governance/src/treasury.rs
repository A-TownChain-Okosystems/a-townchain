// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Treasury management
use std::collections::HashMap;

pub type Amount = u128;

pub struct Treasury {
    balance: Amount,
    allocations: HashMap<String, Amount>,
    total_allocated: Amount,
}

impl Treasury {
    pub fn new(initial: Amount) -> Self {
        Self { balance: initial, allocations: HashMap::new(), total_allocated: 0 }
    }

    pub fn deposit(&mut self, amount: Amount) -> Result<(), String> {
        self.balance = self.balance.checked_add(amount).ok_or("Treasury balance overflow")?;
        Ok(())
    }

    pub fn allocate(&mut self, recipient: &str, amount: Amount) -> Result<(), String> {
        let available = self.balance.checked_sub(self.total_allocated).ok_or("Treasury accounting invariant violated")?;
        if available < amount {
            return Err("Insufficient treasury".into());
        }
        let new_total = self.total_allocated.checked_add(amount).ok_or("Treasury allocation overflow")?;
        let entry = self.allocations.entry(recipient.into()).or_insert(0);
        *entry = entry.checked_add(amount).ok_or("Treasury recipient allocation overflow")?;
        self.total_allocated = new_total;
        Ok(())
    }

    pub fn release(&mut self, recipient: &str) -> Result<Amount, String> {
        let amount = self.allocations.remove(recipient).ok_or("No allocation")?;
        self.total_allocated = self.total_allocated.checked_sub(amount).ok_or("Treasury accounting invariant violated")?;
        self.balance = self.balance.checked_sub(amount).ok_or("Treasury balance invariant violated")?;
        Ok(amount)
    }

    pub fn available(&self) -> Amount {
        self.balance.saturating_sub(self.total_allocated)
    }

    pub fn balance(&self) -> Amount {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_treasury() {
        let mut t = Treasury::new(1000);
        assert!(t.allocate("dev", 500).is_ok());
        assert!(t.allocate("dev", 500).is_ok());
        assert!(t.allocate("dev", 1).is_err());
        assert_eq!(t.available(), 0);
        assert_eq!(t.release("dev").unwrap(), 1000);
    }

    #[test]
    fn max_amount_is_supported() {
        let mut t = Treasury::new(u128::MAX);
        assert_eq!(t.balance(), u128::MAX);
        assert!(t.deposit(1).is_err());
    }

    #[test]
    fn allocation_overflow_is_rejected_atomically() {
        let mut t = Treasury::new(u128::MAX);
        assert!(t.allocate("dev", u128::MAX).is_ok());
        assert!(t.allocate("dev", 1).is_err());
        assert_eq!(t.available(), 0);
        assert_eq!(t.balance(), u128::MAX);
    }
}
