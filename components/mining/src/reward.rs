// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Reward accounting primitives.
//!
//! Consensus issuance is canonical in atc-algorithm. This crate consumes that
//! rule and never defines an independent monetary schedule.

use atc_algorithm::economics::{MonetaryPolicy, MAX_SUPPLY};

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

pub fn canonical_block_subsidy(height: u64, issued_before_block: u128) -> u128 {
    MonetaryPolicy::subsidy(height, issued_before_block)
}

pub fn canonical_max_supply() -> u128 { MAX_SUPPLY }

impl RewardPolicy {
    pub fn validate(&self) -> Result<(), RewardError> {
        if self.reward_per_block == 0 { return Err(RewardError::ZeroReward); }
        if self.reward_per_block > self.max_supply { return Err(RewardError::SupplyExceeded); }
        Ok(())
    }
}

impl RewardLedger {
    pub const fn new() -> Self { Self { issued: 0 } }

    pub fn issue(&mut self, policy: RewardPolicy) -> Result<u128, RewardError> {
        policy.validate()?;
        let next = self.issued.checked_add(policy.reward_per_block).ok_or(RewardError::ArithmeticOverflow)?;
        if next > policy.max_supply { return Err(RewardError::SupplyExceeded); }
        self.issued = next;
        Ok(next)
    }

    pub fn issue_canonical(&mut self, height: u64) -> Result<u128, RewardError> {
        let reward = canonical_block_subsidy(height, self.issued);
        let next = self.issued.checked_add(reward).ok_or(RewardError::ArithmeticOverflow)?;
        if next > MAX_SUPPLY { return Err(RewardError::SupplyExceeded); }
        self.issued = next;
        Ok(reward)
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

    #[test]
    fn canonical_halving_schedule_is_consumed() {
        assert_eq!(canonical_block_subsidy(0, 0), 500 * 1_000_000_000_000_000_000u128);
        assert_eq!(canonical_block_subsidy(360_000, 0), 250 * 1_000_000_000_000_000_000u128);
        assert_eq!(canonical_block_subsidy(12_960_000, 0), 0);
    }

    #[test]
    fn canonical_ledger_reaches_exact_cap() {
        let mut ledger = RewardLedger::new();
        for height in 0..=12_959_999 {
            ledger.issue_canonical(height).unwrap();
        }
        assert_eq!(ledger.issued, MAX_SUPPLY);
        assert_eq!(ledger.issue_canonical(12_960_000), Ok(0));
    }
}