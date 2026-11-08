//! Boot health tracking for fail-closed A/B updates.

use crate::Slot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootHealth {
    pub slot: Slot,
    pub attempts: u32,
    pub max_attempts: u32,
    pub confirmed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthError {
    InvalidLimit,
    AlreadyConfirmed,
}

impl BootHealth {
    pub fn new(slot: Slot, max_attempts: u32) -> Result<Self, HealthError> {
        if max_attempts == 0 {
            return Err(HealthError::InvalidLimit);
        }
        Ok(Self {
            slot,
            attempts: 0,
            max_attempts,
            confirmed: false,
        })
    }

    pub fn record_attempt(&mut self) -> Result<bool, HealthError> {
        if self.confirmed {
            return Err(HealthError::AlreadyConfirmed);
        }
        self.attempts = self.attempts.saturating_add(1);
        Ok(self.should_rollback())
    }

    pub fn confirm(&mut self) {
        self.confirmed = true;
    }

    pub fn should_rollback(&self) -> bool {
        !self.confirmed && self.attempts >= self.max_attempts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rollback_after_failed_attempt_limit() {
        let mut health = BootHealth::new(Slot::B, 2).unwrap();
        assert!(!health.record_attempt().unwrap());
        assert!(health.record_attempt().unwrap());
    }
    #[test]
    fn confirmation_prevents_retry_counter() {
        let mut health = BootHealth::new(Slot::A, 2).unwrap();
        health.confirm();
        assert_eq!(health.record_attempt(), Err(HealthError::AlreadyConfirmed));
        assert!(!health.should_rollback());
    }
}
