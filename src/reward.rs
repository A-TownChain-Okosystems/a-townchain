// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Reward accounting primitives. Contract settlement remains external.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RewardPolicy {
    pub reward_per_block: u128,
    pub max_supply: u128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RewardLedger {
    pub issued: u128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RewardError {
    ZeroReward,
    SupplyExceeded,
    ArithmeticOverflow,
}

impl RewardPolicy {
    pub fn validate(&self) -> Result<(), RewardError> {
        if self.reward_per_block == 0 {
            return Err(RewardError::ZeroReward);
        }
        if self.reward_per_block > self.max_supply {
            return Err(RewardError::SupplyExceeded);
        }
        Ok(())
    }
}

impl RewardLedger {
    pub const fn new() -> Self { Self { issued: 0 } }

    pub fn issue(&mut self, policy: RewardPolicy) -> Result<u128, RewardError> {
        policy.validate()?;
        let next = self.issued.checked_add(policy.reward_per_block).ok_or(RewardError::ArithmeticOverflow)?;
        if next > policy.max_supply {
            return Err(RewardError::SupplyExceeded);
        }
        self.issued = next;
        Ok(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supply_is_bounded() {
        let mut ledger = RewardLedger::new();
        let policy = RewardPolicy { reward_per_block: 10, max_supply: 20 };
        assert_eq!(ledger.issue(policy), Ok(10));
        assert_eq!(ledger.issue(policy), Ok(20));
        assert_eq!(ledger.issue(policy), Err(RewardError::SupplyExceeded));
    }
}
