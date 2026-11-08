use crate::{AuroraError, RequestStatus};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateMachine {
    status: RequestStatus,
}

impl StateMachine {
    pub const fn new() -> Self {
        Self {
            status: RequestStatus::Created,
        }
    }

    pub const fn status(&self) -> RequestStatus {
        self.status
    }

    pub fn transition(&mut self, next: RequestStatus) -> Result<(), AuroraError> {
        if self.can_transition(next) {
            self.status = next;
            Ok(())
        } else {
            Err(AuroraError::InvalidStateTransition)
        }
    }

    fn can_transition(&self, next: RequestStatus) -> bool {
        use RequestStatus::*;
        matches!(
            (self.status, next),
            (Created, Queued)
                | (Queued, Running)
                | (Running, WaitingApproval)
                | (Running, WaitingResource)
                | (Running, Paused)
                | (Running, Verifying)
                | (Running, Failed)
                | (Running, Cancelled)
                | (Running, Timeout)
                | (Running, ResourceExhausted)
                | (Running, SecurityViolation)
                | (WaitingApproval, Running)
                | (WaitingApproval, Denied)
                | (WaitingResource, Running)
                | (WaitingResource, Failed)
                | (Paused, Running)
                | (Verifying, Completed)
                | (Verifying, Failed)
                | (Verifying, SecurityViolation)
        )
    }
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_transition() {
        let mut state = StateMachine::new();
        assert_eq!(
            state.transition(RequestStatus::Completed),
            Err(AuroraError::InvalidStateTransition)
        );
    }
}
