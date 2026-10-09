// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Mining manager & work distribution primitives.

use crate::miner::{MiningJob, MiningError};

pub struct MiningManager {
    current_target: [u8; 32],
    job_queue: Vec<MiningJob>,
}

impl MiningManager {
    pub fn new(initial_target: [u8; 32]) -> Self {
        Self {
            current_target: initial_target,
            job_queue: Vec::new(),
        }
    }

    pub fn update_target(&mut self, new_target: [u8; 32]) -> Result<(), MiningError> {
        if new_target == [0u8; 32] {
            return Err(MiningError::InvalidTarget);
        }
        self.current_target = new_target;
        Ok(())
    }

    pub fn current_target(&self) -> [u8; 32] {
        self.current_target
    }

    pub fn push_job(&mut self, mut job: MiningJob) -> Result<(), MiningError> {
        if job.header.is_empty() {
            return Err(MiningError::EmptyHeader);
        }
        job.target = self.current_target;
        self.job_queue.push(job);
        Ok(())
    }

    pub fn next_job(&mut self) -> Option<MiningJob> {
        if self.job_queue.is_empty() {
            None
        } else {
            Some(self.job_queue.remove(0))
        }
    }

    pub fn queue_len(&self) -> usize {
        self.job_queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mining_manager_job_lifecycle() {
        let initial_target = [0xffu8; 32];
        let mut mgr = MiningManager::new(initial_target);

        let job = MiningJob {
            job_id: 101,
            header: b"header1".to_vec(),
            target: [0u8; 32],
        };

        assert!(mgr.push_job(job).is_ok());
        assert_eq!(mgr.queue_len(), 1);

        let popped = mgr.next_job().unwrap();
        assert_eq!(popped.job_id, 101);
        assert_eq!(popped.target, initial_target);
        assert_eq!(mgr.queue_len(), 0);
    }

    #[test]
    fn test_mining_manager_target_update() {
        let mut mgr = MiningManager::new([0xffu8; 32]);
        let new_target = [0x7fu8; 32];
        assert!(mgr.update_target(new_target).is_ok());
        assert_eq!(mgr.current_target(), new_target);

        assert_eq!(mgr.update_target([0u8; 32]), Err(MiningError::InvalidTarget));
    }
}
