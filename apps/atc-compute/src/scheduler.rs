// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Bounded FIFO slot scheduler for ATC Compute.
//!
//! This module is a local resource scheduler only. It does not define job
//! identity, execution, verification, settlement, or consensus semantics.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub id: u64,
    pub slots: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedError {
    QueueEmpty,
    UnknownJob,
    InvalidSlotCount,
    JobExceedsCapacity,
    DuplicateJobId,
    CapacityInvariantViolation,
}

pub struct Scheduler {
    max_slots: u32,
    free_slots: u32,
    queue: Vec<Job>,
    running: Vec<Job>,
}

impl Scheduler {
    pub fn new(max_slots: u32) -> Self {
        Self {
            max_slots,
            free_slots: max_slots,
            queue: Vec::new(),
            running: Vec::new(),
        }
    }

    /// Add a job if its resource request and identifier are valid.
    ///
    /// A job must request at least one slot and no more than the scheduler's
    /// total capacity. Identifiers are unique across queued and running jobs.
    pub fn enqueue(&mut self, job: Job) -> Result<(), SchedError> {
        if job.slots == 0 {
            return Err(SchedError::InvalidSlotCount);
        }
        if job.slots > self.max_slots {
            return Err(SchedError::JobExceedsCapacity);
        }
        if self.queue.iter().any(|queued| queued.id == job.id)
            || self.running.iter().any(|running| running.id == job.id)
        {
            return Err(SchedError::DuplicateJobId);
        }

        self.queue.push(job);
        Ok(())
    }

    /// Start FIFO jobs while capacity permits.
    ///
    /// Strict FIFO is intentional: if the head job cannot fit, later jobs do
    /// not bypass it. Every accepted job has a positive, capacity-bounded
    /// slot request, so subtraction is safe under the scheduler invariant.
    pub fn dispatch(&mut self) -> Vec<Job> {
        let mut started = Vec::new();

        while let Some(job) = self.queue.first() {
            if job.slots > self.free_slots {
                break;
            }

            let job = self.queue.remove(0);
            // The preceding comparison proves this subtraction cannot underflow.
            self.free_slots -= job.slots;
            self.running.push(job.clone());
            started.push(job);
        }

        started
    }

    /// Finish a running job and release its reserved slots.
    pub fn finish(&mut self, id: u64) -> Result<(), SchedError> {
        let pos = self
            .running
            .iter()
            .position(|job| job.id == id)
            .ok_or(SchedError::UnknownJob)?;

        let job = &self.running[pos];
        let restored = self
            .free_slots
            .checked_add(job.slots)
            .ok_or(SchedError::CapacityInvariantViolation)?;

        // Do not mutate state unless the restored capacity is valid.
        if restored > self.max_slots {
            return Err(SchedError::CapacityInvariantViolation);
        }

        self.running.remove(pos);
        self.free_slots = restored;
        Ok(())
    }

    pub fn free_slots(&self) -> u32 {
        self.free_slots
    }

    pub fn max_slots(&self) -> u32 {
        self.max_slots
    }

    pub fn queued_jobs(&self) -> usize {
        self.queue.len()
    }

    pub fn running_jobs(&self) -> usize {
        self.running.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_respects_capacity_and_fifo() {
        let mut scheduler = Scheduler::new(4);
        assert_eq!(scheduler.enqueue(Job { id: 1, slots: 2 }), Ok(()));
        assert_eq!(scheduler.enqueue(Job { id: 2, slots: 2 }), Ok(()));
        assert_eq!(scheduler.enqueue(Job { id: 3, slots: 1 }), Ok(()));

        let started = scheduler.dispatch();
        assert_eq!(
            started,
            vec![Job { id: 1, slots: 2 }, Job { id: 2, slots: 2 }]
        );
        assert_eq!(scheduler.free_slots(), 0);
        assert_eq!(scheduler.running_jobs(), 2);
        assert_eq!(scheduler.queued_jobs(), 1);

        assert_eq!(scheduler.finish(1), Ok(()));
        assert_eq!(scheduler.free_slots(), 2);
        assert_eq!(scheduler.dispatch(), vec![Job { id: 3, slots: 1 }]);
        assert_eq!(scheduler.free_slots(), 1);
    }

    #[test]
    fn rejects_zero_slots() {
        let mut scheduler = Scheduler::new(4);
        assert_eq!(
            scheduler.enqueue(Job { id: 1, slots: 0 }),
            Err(SchedError::InvalidSlotCount)
        );
        assert_eq!(scheduler.queued_jobs(), 0);
        assert_eq!(scheduler.free_slots(), 4);
    }

    #[test]
    fn rejects_job_larger_than_total_capacity() {
        let mut scheduler = Scheduler::new(4);
        assert_eq!(
            scheduler.enqueue(Job { id: 1, slots: 5 }),
            Err(SchedError::JobExceedsCapacity)
        );
        assert_eq!(scheduler.queued_jobs(), 0);
    }

    #[test]
    fn rejects_jobs_when_capacity_is_zero() {
        let mut scheduler = Scheduler::new(0);
        assert_eq!(
            scheduler.enqueue(Job { id: 1, slots: 1 }),
            Err(SchedError::JobExceedsCapacity)
        );
        assert_eq!(scheduler.free_slots(), 0);
    }

    #[test]
    fn rejects_duplicate_ids_in_queue_and_running_set() {
        let mut scheduler = Scheduler::new(2);
        assert_eq!(scheduler.enqueue(Job { id: 7, slots: 1 }), Ok(()));
        assert_eq!(
            scheduler.enqueue(Job { id: 7, slots: 1 }),
            Err(SchedError::DuplicateJobId)
        );
        assert_eq!(scheduler.dispatch().len(), 1);
        assert_eq!(
            scheduler.enqueue(Job { id: 7, slots: 1 }),
            Err(SchedError::DuplicateJobId)
        );
        assert_eq!(scheduler.free_slots(), 1);
    }

    #[test]
    fn allows_id_reuse_after_job_finishes() {
        let mut scheduler = Scheduler::new(1);
        assert_eq!(scheduler.enqueue(Job { id: 7, slots: 1 }), Ok(()));
        assert_eq!(scheduler.dispatch().len(), 1);
        assert_eq!(scheduler.finish(7), Ok(()));
        assert_eq!(scheduler.enqueue(Job { id: 7, slots: 1 }), Ok(()));
    }

    #[test]
    fn unknown_job_does_not_change_capacity() {
        let mut scheduler = Scheduler::new(2);
        assert_eq!(scheduler.finish(9), Err(SchedError::UnknownJob));
        assert_eq!(scheduler.free_slots(), 2);
    }

    #[test]
    fn repeated_finish_cannot_release_slots_twice() {
        let mut scheduler = Scheduler::new(2);
        assert_eq!(scheduler.enqueue(Job { id: 1, slots: 2 }), Ok(()));
        assert_eq!(scheduler.dispatch().len(), 1);
        assert_eq!(scheduler.finish(1), Ok(()));
        assert_eq!(scheduler.finish(1), Err(SchedError::UnknownJob));
        assert_eq!(scheduler.free_slots(), 2);
    }

    #[test]
    fn fifo_head_blocks_later_job_that_would_fit() {
        let mut scheduler = Scheduler::new(3);
        assert_eq!(scheduler.enqueue(Job { id: 1, slots: 3 }), Ok(()));
        assert_eq!(scheduler.enqueue(Job { id: 2, slots: 1 }), Ok(()));
        assert_eq!(scheduler.enqueue(Job { id: 3, slots: 2 }), Ok(()));

        assert_eq!(scheduler.dispatch().len(), 1);
        assert_eq!(scheduler.free_slots(), 0);
        assert_eq!(scheduler.finish(1), Ok(()));
        assert_eq!(scheduler.dispatch().len(), 2);
        assert_eq!(scheduler.free_slots(), 0);
    }
}
