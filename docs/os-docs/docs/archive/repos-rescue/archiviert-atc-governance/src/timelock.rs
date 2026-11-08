// Timelock for proposal execution
pub struct Timelock {
    pub delay_blocks: u64,
    pub queued: Vec<(u64, u64)>,
}

impl Timelock {
    pub fn new(delay: u64) -> Self { Self { delay_blocks: delay, queued: Vec::new() } }

    pub fn queue(&mut self, proposal_id: u64, current_block: u64) {
        self.queued.push((proposal_id, current_block + self.delay_blocks));
    }

    pub fn is_ready(&self, proposal_id: u64, current_block: u64) -> bool {
        self.queued.iter().any(|(id, eta)| *id == proposal_id && current_block >= *eta)
    }

    pub fn execute(&mut self, proposal_id: u64, current_block: u64) -> Result<(), String> {
        if !self.is_ready(proposal_id, current_block) { return Err("Timelock not expired".into()); }
        self.queued.retain(|(id, _)| *id != proposal_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timelock() {
        let mut t = Timelock::new(100);
        t.queue(1, 500);
        assert!(!t.is_ready(1, 599));
        assert!(t.is_ready(1, 600));
        assert!(t.execute(1, 600).is_ok());
        assert!(t.execute(1, 600).is_err());
    }
}
