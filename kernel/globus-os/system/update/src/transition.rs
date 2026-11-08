//! Fail-closed update state transitions.

use crate::{Slot, UpdatePlan, UpdateState};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    InvalidState,
    WrongTarget,
}

impl UpdatePlan {
    pub fn advance(&mut self, next: UpdateState) -> Result<(), TransitionError> {
        let valid = matches!(
            (self.state, next),
            (UpdateState::Idle, UpdateState::Downloaded)
                | (UpdateState::Downloaded, UpdateState::Verified)
                | (UpdateState::Verified, UpdateState::Activated)
                | (UpdateState::Activated, UpdateState::RolledBack)
        );
        if !valid {
            return Err(TransitionError::InvalidState);
        }
        self.state = next;
        if next == UpdateState::Activated {
            self.current = self.target;
            self.target = match self.current {
                Slot::A => Slot::B,
                Slot::B => Slot::A,
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn requires_verification_before_activation() {
        let mut p = UpdatePlan::new(Slot::A);
        assert!(p.advance(UpdateState::Activated).is_err());
        p.advance(UpdateState::Downloaded).unwrap();
        p.advance(UpdateState::Verified).unwrap();
        p.advance(UpdateState::Activated).unwrap();
        assert_eq!(p.current, Slot::B);
    }
}
