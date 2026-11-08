// Voting mechanism and tallying
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoteType { For, Against, Abstain }

#[derive(Debug, Clone)]
pub struct Vote {
    pub voter: String,
    pub proposal_id: u64,
    pub vote_type: VoteType,
    pub weight: u64,
}

pub struct VotingSystem {
    votes: HashMap<(String, u64), Vote>,
}

impl VotingSystem {
    pub fn new() -> Self { Self { votes: HashMap::new() } }

    pub fn cast_vote(&mut self, voter: &str, proposal_id: u64, vote_type: VoteType, weight: u64) -> Result<(), String> {
        let key = (voter.to_string(), proposal_id);
        if self.votes.contains_key(&key) { return Err("Already voted".into()); }
        self.votes.insert(key, Vote { voter: voter.into(), proposal_id, vote_type, weight });
        Ok(())
    }

    pub fn get_vote(&self, voter: &str, proposal_id: u64) -> Option<&Vote> {
        self.votes.get(&(voter.to_string(), proposal_id))
    }

    pub fn tally(&self, proposal_id: u64) -> (u64, u64, u64) {
        let (mut for_v, mut against, mut abstain) = (0u64, 0u64, 0u64);
        for vote in self.votes.values() {
            if vote.proposal_id == proposal_id {
                match vote.vote_type {
                    VoteType::For => for_v += vote.weight,
                    VoteType::Against => against += vote.weight,
                    VoteType::Abstain => abstain += vote.weight,
                }
            }
        }
        (for_v, against, abstain)
    }

    pub fn voter_count(&self, proposal_id: u64) -> usize {
        self.votes.values().filter(|v| v.proposal_id == proposal_id).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voting() {
        let mut vs = VotingSystem::new();
        assert!(vs.cast_vote("alice", 1, VoteType::For, 100).is_ok());
        assert!(vs.cast_vote("bob", 1, VoteType::Against, 50).is_ok());
        assert!(vs.cast_vote("alice", 1, VoteType::For, 100).is_err());
        let (f, a, _) = vs.tally(1);
        assert_eq!((f, a), (100, 50));
        assert_eq!(vs.voter_count(1), 2);
    }
}
