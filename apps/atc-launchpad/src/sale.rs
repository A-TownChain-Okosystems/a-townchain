// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Sale-Accounting mit Hard-Cap, Soft-Cap, State Machine und Mindest-/Höchst-Beitrag (LP-SALE-001).

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaleState {
    Draft,
    Configured,
    WhitelistOpen,
    PublicSale,
    SoldOut,
    Ended,
    Claimable,
    Settled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SaleError {
    HardCapExceeded,
    BelowMinimum,
    ExceedsMaxContribution,
    InvalidStateTransition { from: SaleState, to: SaleState },
    SaleNotActive,
    SlotOutOfRange,
    SoftCapNotReached,
    RefundNotAvailable,
    NothingToRefund,
}

#[derive(Debug, Clone)]
pub struct SaleConfig {
    pub hard_cap: u128,
    pub soft_cap: u128,
    pub min_contribution: u128,
    pub max_contribution_per_user: u128,
    pub start_slot: u64,
    pub end_slot: u64,
}

impl SaleConfig {
    pub fn simple(hard_cap: u128, min_contribution: u128) -> Self {
        Self {
            hard_cap,
            soft_cap: 0,
            min_contribution,
            max_contribution_per_user: u128::MAX,
            start_slot: 0,
            end_slot: u64::MAX,
        }
    }
}

pub struct Sale {
    state: SaleState,
    config: SaleConfig,
    total: u128,
    user_contributions: HashMap<u64, u128>,
    contributions: Vec<(u64, u128)>,
}

impl Sale {
    pub fn new(config: SaleConfig) -> Self {
        Sale {
            state: SaleState::Draft,
            config,
            total: 0,
            user_contributions: HashMap::new(),
            contributions: Vec::new(),
        }
    }

    pub fn state(&self) -> SaleState {
        self.state
    }

    pub fn transition_to(&mut self, next: SaleState) -> Result<(), SaleError> {
        let valid = match (self.state, next) {
            (SaleState::Draft, SaleState::Configured) => true,
            (SaleState::Configured, SaleState::WhitelistOpen) => true,
            (SaleState::Configured, SaleState::PublicSale) => true,
            (SaleState::WhitelistOpen, SaleState::PublicSale) => true,
            (SaleState::WhitelistOpen, SaleState::Ended) => true,
            (SaleState::PublicSale, SaleState::SoldOut) => true,
            (SaleState::PublicSale, SaleState::Ended) => true,
            (SaleState::SoldOut, SaleState::Claimable) => true,
            (SaleState::Ended, SaleState::Claimable) => self.total >= self.config.soft_cap,
            (SaleState::Ended, SaleState::Settled) => self.total < self.config.soft_cap,
            (SaleState::Claimable, SaleState::Settled) => true,
            _ => false,
        };

        if valid {
            self.state = next;
            Ok(())
        } else {
            Err(SaleError::InvalidStateTransition {
                from: self.state,
                to: next,
            })
        }
    }

    pub fn contribute(&mut self, contributor: u64, amount: u128) -> Result<(), SaleError> {
        self.contribute_at_slot(contributor, amount, self.config.start_slot)
    }

    pub fn contribute_at_slot(&mut self, contributor: u64, amount: u128, slot: u64) -> Result<(), SaleError> {
        if self.state != SaleState::PublicSale && self.state != SaleState::WhitelistOpen && self.state != SaleState::Draft {
            return Err(SaleError::SaleNotActive);
        }

        if slot < self.config.start_slot || slot > self.config.end_slot {
            return Err(SaleError::SlotOutOfRange);
        }

        if amount < self.config.min_contribution {
            return Err(SaleError::BelowMinimum);
        }

        let current_user_total = self.user_contributions.get(&contributor).copied().unwrap_or(0);
        if current_user_total.saturating_add(amount) > self.config.max_contribution_per_user {
            return Err(SaleError::ExceedsMaxContribution);
        }

        if self.total.saturating_add(amount) > self.config.hard_cap {
            return Err(SaleError::HardCapExceeded);
        }

        self.total += amount;
        *self.user_contributions.entry(contributor).or_insert(0) += amount;
        self.contributions.push((contributor, amount));

        if self.total == self.config.hard_cap && self.state == SaleState::PublicSale {
            self.state = SaleState::SoldOut;
        }

        Ok(())
    }

    pub fn refund(&mut self, contributor: u64) -> Result<u128, SaleError> {
        if self.state != SaleState::Ended && self.state != SaleState::Settled {
            return Err(SaleError::RefundNotAvailable);
        }

        if self.total >= self.config.soft_cap {
            return Err(SaleError::RefundNotAvailable);
        }

        let amount = self.user_contributions.remove(&contributor).unwrap_or(0);
        if amount == 0 {
            return Err(SaleError::NothingToRefund);
        }

        Ok(amount)
    }

    pub fn total(&self) -> u128 {
        self.total
    }

    pub fn contributors(&self) -> usize {
        self.user_contributions.len()
    }

    pub fn user_contribution(&self, contributor: u64) -> u128 {
        self.user_contributions.get(&contributor).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cap_und_minimum() {
        let mut s = Sale::new(SaleConfig::simple(100, 10));
        assert_eq!(s.contribute(1, 10), Ok(()));
        assert_eq!(s.contribute(2, 90), Ok(()));
        assert_eq!(s.contribute(3, 1), Err(SaleError::BelowMinimum));
        assert_eq!(s.contribute(3, 50), Err(SaleError::HardCapExceeded));
        assert_eq!(s.total(), 100);
        assert_eq!(s.contributors(), 2);
    }

    #[test]
    fn exakt_bis_cap() {
        let mut s = Sale::new(SaleConfig::simple(100, 1));
        for i in 0..100 {
            assert_eq!(s.contribute(i, 1), Ok(()));
        }
        assert_eq!(s.contribute(999, 1), Err(SaleError::HardCapExceeded));
        assert_eq!(s.total(), 100);
    }

    #[test]
    fn test_sale_state_transitions() {
        let mut s = Sale::new(SaleConfig::simple(1000, 10));
        assert_eq!(s.state(), SaleState::Draft);
        assert_eq!(s.transition_to(SaleState::Configured), Ok(()));
        assert_eq!(s.state(), SaleState::Configured);
        assert_eq!(s.transition_to(SaleState::PublicSale), Ok(()));
        assert_eq!(s.state(), SaleState::PublicSale);
        assert_eq!(
            s.transition_to(SaleState::WhitelistOpen),
            Err(SaleError::InvalidStateTransition {
                from: SaleState::PublicSale,
                to: SaleState::WhitelistOpen
            })
        );
    }

    #[test]
    fn test_max_contribution_per_user() {
        let config = SaleConfig {
            hard_cap: 1000,
            soft_cap: 100,
            min_contribution: 10,
            max_contribution_per_user: 50,
            start_slot: 10,
            end_slot: 100,
        };
        let mut s = Sale::new(config);
        s.transition_to(SaleState::Configured).unwrap();
        s.transition_to(SaleState::PublicSale).unwrap();

        assert_eq!(s.contribute_at_slot(1, 40, 20), Ok(()));
        assert_eq!(s.contribute_at_slot(1, 20, 21), Err(SaleError::ExceedsMaxContribution));
        assert_eq!(s.user_contribution(1), 40);
    }

    #[test]
    fn test_slot_range_and_refund() {
        let config = SaleConfig {
            hard_cap: 1000,
            soft_cap: 500,
            min_contribution: 10,
            max_contribution_per_user: 1000,
            start_slot: 100,
            end_slot: 200,
        };
        let mut s = Sale::new(config);
        s.transition_to(SaleState::Configured).unwrap();
        s.transition_to(SaleState::PublicSale).unwrap();

        assert_eq!(s.contribute_at_slot(1, 100, 50), Err(SaleError::SlotOutOfRange));
        assert_eq!(s.contribute_at_slot(1, 100, 150), Ok(()));

        s.transition_to(SaleState::Ended).unwrap();
        assert_eq!(s.transition_to(SaleState::Settled), Ok(()));
        assert_eq!(s.refund(1), Ok(100));
        assert_eq!(s.refund(1), Err(SaleError::NothingToRefund));
    }
}
