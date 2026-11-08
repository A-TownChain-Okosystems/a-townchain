// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Deterministic execution state machine for Aurora Core.
//
// This module validates orchestration state only. It does not perform I/O, grant capabilities,
// execute tools, or replace policy/approval checks.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionState {
    Created,
    Queued,
    Running,
    WaitingApproval,
    WaitingResource,
    Paused,
    Verifying,
    Completed,
    Failed,
    Cancelled,
    Denied,
    Timeout,
    ResourceExhausted,
    SecurityViolation,
}

impl ExecutionState {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed
                | Self::Failed
                | Self::Cancelled
                | Self::Denied
                | Self::Timeout
                | Self::ResourceExhausted
                | Self::SecurityViolation
        )
    }

    fn may_transition_to(self, next: Self) -> bool {
        use ExecutionState::*;
        match self {
            Created => matches!(next, Queued | Denied | Cancelled),
            Queued => matches!(
                next,
                Running | WaitingApproval | WaitingResource | Denied | Cancelled | Timeout
            ),
            Running => matches!(
                next,
                WaitingApproval
                    | WaitingResource
                    | Paused
                    | Verifying
                    | Failed
                    | Denied
                    | Cancelled
                    | Timeout
                    | ResourceExhausted
                    | SecurityViolation
            ),
            WaitingApproval => matches!(next, Queued | Denied | Cancelled | Timeout),
            WaitingResource => matches!(
                next,
                Queued | Running | Cancelled | Timeout | ResourceExhausted
            ),
            Paused => matches!(next, Queued | Cancelled | Timeout),
            Verifying => matches!(next, Completed | Failed | SecurityViolation),
            Completed | Failed | Cancelled | Denied | Timeout | ResourceExhausted
            | SecurityViolation => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateTransitionError {
    pub from: ExecutionState,
    pub to: ExecutionState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateMachine {
    current: ExecutionState,
}

impl Default for StateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine {
    pub const fn new() -> Self {
        Self {
            current: ExecutionState::Created,
        }
    }

    pub const fn current(&self) -> ExecutionState {
        self.current
    }

    pub fn transition(&mut self, next: ExecutionState) -> Result<(), StateTransitionError> {
        if self.current.may_transition_to(next) {
            self.current = next;
            Ok(())
        } else {
            Err(StateTransitionError {
                from: self.current,
                to: next,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_execution_must_pass_verification() {
        let mut machine = StateMachine::new();
        assert_eq!(machine.current(), ExecutionState::Created);
        machine.transition(ExecutionState::Queued).unwrap();
        machine.transition(ExecutionState::Running).unwrap();
        machine.transition(ExecutionState::Verifying).unwrap();
        machine.transition(ExecutionState::Completed).unwrap();
        assert!(machine.current().is_terminal());
    }

    #[test]
    fn approval_wait_cannot_skip_back_to_running() {
        let mut machine = StateMachine::new();
        machine.transition(ExecutionState::Queued).unwrap();
        machine.transition(ExecutionState::WaitingApproval).unwrap();
        let error = machine.transition(ExecutionState::Running).unwrap_err();
        assert_eq!(error.from, ExecutionState::WaitingApproval);
        assert_eq!(error.to, ExecutionState::Running);
        assert_eq!(machine.current(), ExecutionState::WaitingApproval);
    }

    #[test]
    fn denied_is_terminal_and_cannot_be_reopened() {
        let mut machine = StateMachine::new();
        machine.transition(ExecutionState::Denied).unwrap();
        assert!(machine.transition(ExecutionState::Queued).is_err());
        assert_eq!(machine.current(), ExecutionState::Denied);
    }

    #[test]
    fn verification_cannot_transition_directly_to_running() {
        let mut machine = StateMachine::new();
        machine.transition(ExecutionState::Queued).unwrap();
        machine.transition(ExecutionState::Running).unwrap();
        machine.transition(ExecutionState::Verifying).unwrap();
        assert!(machine.transition(ExecutionState::Running).is_err());
    }

    #[test]
    fn security_violation_is_terminal() {
        let mut machine = StateMachine::new();
        machine.transition(ExecutionState::Queued).unwrap();
        machine.transition(ExecutionState::Running).unwrap();
        machine
            .transition(ExecutionState::SecurityViolation)
            .unwrap();
        assert!(machine.transition(ExecutionState::Queued).is_err());
    }
}
