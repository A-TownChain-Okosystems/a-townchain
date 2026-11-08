//! Deterministic priority scheduler for userspace processes.

use crate::{ProcessId, ProcessInfo, ProcessState};

#[derive(Debug, Default)]
pub struct Scheduler {
    processes: Vec<ProcessInfo>,
    current: Option<ProcessId>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, process: ProcessInfo) {
        if self.processes.iter().any(|p| p.id == process.id) {
            return;
        }
        self.processes.push(process);
        self.processes
            .sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| a.id.cmp(&b.id)));
    }

    pub fn set_state(&mut self, id: ProcessId, state: ProcessState) -> bool {
        let Some(process) = self.processes.iter_mut().find(|p| p.id == id) else {
            return false;
        };
        process.state = state;
        true
    }

    pub fn next_process(&mut self) -> Option<ProcessId> {
        let next = self
            .processes
            .iter()
            .find(|p| matches!(p.state, ProcessState::Ready | ProcessState::Running))
            .map(|p| p.id);
        self.current = next;
        if let Some(id) = next {
            for p in &mut self.processes {
                if p.id == id {
                    p.state = ProcessState::Running;
                } else if p.state == ProcessState::Running {
                    p.state = ProcessState::Ready;
                }
            }
        }
        next
    }

    pub fn current(&self) -> Option<ProcessId> {
        self.current
    }
    pub fn processes(&self) -> &[ProcessInfo] {
        &self.processes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selects_highest_priority_deterministically() {
        let mut s = Scheduler::new();
        s.register(ProcessInfo {
            id: ProcessId(2),
            state: ProcessState::Ready,
            priority: 10,
        });
        s.register(ProcessInfo {
            id: ProcessId(1),
            state: ProcessState::Ready,
            priority: 10,
        });
        assert_eq!(s.next_process(), Some(ProcessId(1)));
    }
}
