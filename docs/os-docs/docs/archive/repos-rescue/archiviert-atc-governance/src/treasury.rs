// Treasury management
use std::collections::HashMap;

pub struct Treasury {
    balance: u64,
    allocations: HashMap<String, u64>,
    total_allocated: u64,
}

impl Treasury {
    pub fn new(initial: u64) -> Self {
        Self { balance: initial, allocations: HashMap::new(), total_allocated: 0 }
    }

    pub fn deposit(&mut self, amount: u64) { self.balance += amount; }

    pub fn allocate(&mut self, recipient: &str, amount: u64) -> Result<(), String> {
        if self.balance - self.total_allocated < amount { return Err("Insufficient treasury".into()); }
        *self.allocations.entry(recipient.into()).or_insert(0) += amount;
        self.total_allocated += amount;
        Ok(())
    }

    pub fn release(&mut self, recipient: &str) -> Result<u64, String> {
        let amount = self.allocations.remove(recipient).ok_or("No allocation")?;
        self.total_allocated -= amount;
        self.balance -= amount;
        Ok(amount)
    }

    pub fn available(&self) -> u64 { self.balance - self.total_allocated }
    pub fn balance(&self) -> u64 { self.balance }
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
}
