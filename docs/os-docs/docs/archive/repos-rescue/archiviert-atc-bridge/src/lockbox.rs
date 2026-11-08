// Bridge lockbox for custodial assets
use std::collections::HashMap;

pub struct Lockbox {
    deposits: HashMap<String, u64>,
    withdrawals: HashMap<String, u64>,
}

impl Lockbox {
    pub fn new() -> Self { Self { deposits: HashMap::new(), withdrawals: HashMap::new() } }
    pub fn deposit(&mut self, user: &str, amount: u64) {
        *self.deposits.entry(user.into()).or_insert(0) += amount;
    }
    pub fn withdraw(&mut self, user: &str, amount: u64) -> Result<(), String> {
        let balance = *self.deposits.get(user).unwrap_or(&0);
        let withdrawn = *self.withdrawals.get(user).unwrap_or(&0);
        if balance - withdrawn < amount { return Err("Insufficient lockbox balance".into()); }
        *self.withdrawals.entry(user.into()).or_insert(0) += amount;
        Ok(())
    }
    pub fn balance(&self, user: &str) -> u64 {
        *self.deposits.get(user).unwrap_or(&0) - *self.withdrawals.get(user).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lockbox() {
        let mut lb = Lockbox::new();
        lb.deposit("alice", 1000);
        assert_eq!(lb.balance("alice"), 1000);
        assert!(lb.withdraw("alice", 600).is_ok());
        assert_eq!(lb.balance("alice"), 400);
        assert!(lb.withdraw("alice", 500).is_err());
    }
}
