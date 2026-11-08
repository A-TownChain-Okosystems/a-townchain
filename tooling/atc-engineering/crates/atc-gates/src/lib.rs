//! Deterministic engineering release gates.
//! `UNKNOWN` is never promoted: missing evidence is a blocking condition.

mod report;
pub use report::GateReport;

use atc_core::DerivedState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceStatus {
    Pass,
    Fail,
    Missing,
    Unknown,
}

impl EvidenceStatus {
    pub const fn is_blocking(self) -> bool {
        !matches!(self, Self::Pass)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Missing => "MISSING",
            Self::Unknown => "UNKNOWN",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GateDecision {
    Allow,
    Block,
}

impl GateDecision {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "ALLOW",
            Self::Block => "BLOCK",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceCheck {
    pub control_id: String,
    pub status: EvidenceStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparationOfDuties {
    pub coder: String,
    pub validator: String,
    pub auditor: String,
    pub release_authority: String,
    pub human_approval: bool,
}

impl SeparationOfDuties {
    pub fn valid(&self) -> bool {
        !self.coder.is_empty()
            && !self.validator.is_empty()
            && !self.auditor.is_empty()
            && !self.release_authority.is_empty()
            && self.coder != self.validator
            && self.coder != self.auditor
            && self.coder != self.release_authority
            && self.validator != self.auditor
            && self.validator != self.release_authority
            && self.auditor != self.release_authority
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadinessInput {
    pub evidence: Vec<EvidenceCheck>,
    pub sod: SeparationOfDuties,
    pub requested_state: DerivedState,
}

pub fn evaluate(input: &ReadinessInput) -> GateDecision {
    if input.evidence.iter().any(|e| e.status.is_blocking()) {
        return GateDecision::Block;
    }
    if input.requested_state == DerivedState::ProductionReady
        && (!input.sod.valid() || !input.sod.human_approval)
    {
        return GateDecision::Block;
    }
    GateDecision::Allow
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sod(approved: bool) -> SeparationOfDuties {
        SeparationOfDuties {
            coder: "agent-a".into(),
            validator: "agent-b".into(),
            auditor: "agent-c".into(),
            release_authority: "human-r".into(),
            human_approval: approved,
        }
    }

    #[test]
    fn missing_evidence_blocks() {
        let input = ReadinessInput {
            evidence: vec![EvidenceCheck {
                control_id: "SEC-001".into(),
                status: EvidenceStatus::Missing,
            }],
            sod: sod(true),
            requested_state: DerivedState::TestnetReady,
        };
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn production_requires_human_approval() {
        let input = ReadinessInput {
            evidence: vec![EvidenceCheck {
                control_id: "SEC-001".into(),
                status: EvidenceStatus::Pass,
            }],
            sod: sod(false),
            requested_state: DerivedState::ProductionReady,
        };
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn all_controls_pass_allows() {
        let input = ReadinessInput {
            evidence: vec![EvidenceCheck {
                control_id: "SEC-001".into(),
                status: EvidenceStatus::Pass,
            }],
            sod: sod(true),
            requested_state: DerivedState::ProductionReady,
        };
        assert_eq!(evaluate(&input), GateDecision::Allow);
    }

    #[test]
    fn duplicate_principals_block_production() {
        let input = ReadinessInput {
            evidence: vec![EvidenceCheck {
                control_id: "SEC-001".into(),
                status: EvidenceStatus::Pass,
            }],
            sod: SeparationOfDuties {
                coder: "same".into(),
                validator: "same".into(),
                auditor: "auditor".into(),
                release_authority: "release".into(),
                human_approval: true,
            },
            requested_state: DerivedState::ProductionReady,
        };
        assert_eq!(evaluate(&input), GateDecision::Block);
    }

    #[test]
    fn empty_role_blocks_production() {
        let mut roles = sod(true);
        roles.coder.clear();
        assert!(!roles.valid());
    }
}
