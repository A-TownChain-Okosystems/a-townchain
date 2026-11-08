// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Scheduler & Thread Domains implementation (SC-002).

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    BlockedOnIpc,
    BlockedOnTimer,
    Terminated,
}

#[derive(Clone, Debug)]
pub struct ThreadControlBlock {
    pub thread_id: u64,
    pub priority: u8,
    pub state: ThreadState,
    pub cspace_root_node: usize,
}

pub struct SchedulerDomain {
    pub domain_id: u32,
    threads: Vec<ThreadControlBlock>,
    current_idx: usize,
}

impl SchedulerDomain {
    pub fn new(domain_id: u32) -> Self {
        Self {
            domain_id,
            threads: Vec::new(),
            current_idx: 0,
        }
    }

    pub fn add_thread(&mut self, thread_id: u64, priority: u8, cspace_root_node: usize) {
        self.threads.push(ThreadControlBlock {
            thread_id,
            priority,
            state: ThreadState::Ready,
            cspace_root_node,
        });
    }

    pub fn set_state(&mut self, thread_id: u64, new_state: ThreadState) -> Result<(), String> {
        if let Some(t) = self.threads.iter_mut().find(|t| t.thread_id == thread_id) {
            t.state = new_state;
            Ok(())
        } else {
            Err(String::from("Thread not found"))
        }
    }

    pub fn schedule_next(&mut self) -> Option<&mut ThreadControlBlock> {
        if self.threads.is_empty() {
            return None;
        }

        // Find highest priority ready thread
        let mut best_idx = None;
        let mut max_prio = 0u8;

        for (i, t) in self.threads.iter().enumerate() {
            if t.state == ThreadState::Ready && (best_idx.is_none() || t.priority > max_prio) {
                max_prio = t.priority;
                best_idx = Some(i);
            }
        }

        if let Some(idx) = best_idx {
            if let Some(curr) = self.threads.get_mut(self.current_idx) {
                if curr.state == ThreadState::Running {
                    curr.state = ThreadState::Ready;
                }
            }
            self.current_idx = idx;
            self.threads[idx].state = ThreadState::Running;
            Some(&mut self.threads[idx])
        } else {
            None
        }
    }

    pub fn thread_count(&self) -> usize {
        self.threads.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_scheduling() {
        let mut domain = SchedulerDomain::new(1);
        domain.add_thread(100, 10, 0);
        domain.add_thread(101, 20, 0); // Higher priority

        let scheduled = domain.schedule_next().expect("Should schedule thread");
        assert_eq!(scheduled.thread_id, 101);
        assert_eq!(scheduled.state, ThreadState::Running);

        domain.set_state(101, ThreadState::BlockedOnIpc).unwrap();
        let scheduled2 = domain.schedule_next().expect("Should fallback to thread 100");
        assert_eq!(scheduled2.thread_id, 100);
    }
}
