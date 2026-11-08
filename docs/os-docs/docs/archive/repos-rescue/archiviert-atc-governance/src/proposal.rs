// Governance proposal creation and voting
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ProposalStatus { Pending, Active, Passed, Rejected, Executed, Cancelled }

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub proposer: String,
    pub status: ProposalStatus,
    pub for_votes: u64,
    pub against_votes: u64,
    pub abstain_votes: u64,
    pub start_block: u64,
    pub end_block: u64,
}

impl Proposal {
    pub fn new(id: u64, title: &str, desc: &str, proposer: &str, start: u64, end: u64) -> Self {
        Self {
            id, title: title.into(), description: desc.into(), proposer: proposer.into(),
            status: ProposalStatus::Pending, for_votes: 0, against_votes: 0, abstain_votes: 0,
            start_block: start, end_block: end,
        }
    }

    pub fn activate(&mut self) { self.status = ProposalStatus::Active; }
    pub fn cancel(&mut self) { self.status = ProposalStatus::Cancelled; }

    pub fn tally(&mut self, threshold: f64) {
        let total = self.for_votes + self.against_votes;
        if total == 0 { self.status = ProposalStatus::Rejected; return; }
        let ratio = self.for_votes as f64 / total as f64;
        self.status = if ratio >= threshold { ProposalStatus::Passed } else { ProposalStatus::Rejected };
    }

    pub fn execute(&mut self) -> Result<(), String> {
        if self.status != ProposalStatus::Passed { return Err("Proposal not passed".into()); }
        self.status = ProposalStatus::Executed;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proposal_lifecycle() {
        let mut p = Proposal::new(1, "Test", "Desc", "alice", 100, 200);
        p.activate();
        assert_eq!(p.status, ProposalStatus::Active);
        p.for_votes = 100;
        p.against_votes = 30;
        p.tally(0.66);
        assert_eq!(p.status, ProposalStatus::Passed);
        assert!(p.execute().is_ok());
        assert_eq!(p.status, ProposalStatus::Executed);
    }

    #[test]
    fn test_proposal_reject() {
        let mut p = Proposal::new(2, "Bad", "No", "bob", 100, 200);
        p.for_votes = 10;
        p.against_votes = 90;
        p.tally(0.66);
        assert_eq!(p.status, ProposalStatus::Rejected);
    }
}
