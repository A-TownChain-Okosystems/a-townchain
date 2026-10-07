// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Canonical wallet balance model. Economic amounts are u128.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Balance {
    available: u128,
    locked: u128,
}

impl Balance {
    pub const fn new(available: u128, locked: u128) -> Self { Self { available, locked } }
    pub const fn available(&self) -> u128 { self.available }
    pub const fn locked(&self) -> u128 { self.locked }
    pub fn total(&self) -> Option<u128> { self.available.checked_add(self.locked) }
    pub fn lock(&mut self, amount: u128) -> Result<(), BalanceError> {
        self.available = self.available.checked_sub(amount).ok_or(BalanceError::InsufficientAvailable)?;
        self.locked = self.locked.checked_add(amount).ok_or(BalanceError::Overflow)?;
        Ok(())
    }
    pub fn unlock(&mut self, amount: u128) -> Result<(), BalanceError> {
        self.locked = self.locked.checked_sub(amount).ok_or(BalanceError::InsufficientLocked)?;
        self.available = self.available.checked_add(amount).ok_or(BalanceError::Overflow)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceError { InsufficientAvailable, InsufficientLocked, Overflow }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lock_and_unlock_are_atomic() {
        let mut balance = Balance::new(10, 0);
        assert!(balance.lock(7).is_ok());
        assert_eq!(balance, Balance::new(3, 7));
        assert!(balance.unlock(7).is_ok());
        assert_eq!(balance, Balance::new(10, 0));
    }
}
