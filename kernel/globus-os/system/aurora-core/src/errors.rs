#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuroraError {
    InvalidRequest,
    Unauthorized,
    CapabilityDenied,
    PolicyDenied,
    ApprovalRequired,
    ExecutionFailed,
    VerificationFailed,
    Timeout,
    ResourceExhausted,
    SecurityViolation,
    InvalidStateTransition,
    Internal,
}

impl core::fmt::Display for AuroraError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::InvalidRequest => "invalid request",
            Self::Unauthorized => "unauthorized",
            Self::CapabilityDenied => "capability denied",
            Self::PolicyDenied => "policy denied",
            Self::ApprovalRequired => "approval required",
            Self::ExecutionFailed => "execution failed",
            Self::VerificationFailed => "verification failed",
            Self::Timeout => "timeout",
            Self::ResourceExhausted => "resource exhausted",
            Self::SecurityViolation => "security violation",
            Self::InvalidStateTransition => "invalid state transition",
            Self::Internal => "internal error",
        })
    }
}
