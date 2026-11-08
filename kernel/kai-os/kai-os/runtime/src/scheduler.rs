//! Deterministischer Round-Robin-Scheduler.
//! Kein RNG, keine Wall-Clock, keine Prioritäts-Inversion: identische
//! Registration -> identische Turn-Sequenz (REQ-ENG-002).

use crate::SandboxId;

pub struct Scheduler {
    order: Vec<SandboxId>,
    cursor: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            order: Vec::new(),
            cursor: 0,
        }
    }

    pub fn register(&mut self, id: &str) {
        self.order.push(id.to_string());
    }

    pub fn registered(&self) -> &[SandboxId] {
        &self.order
    }

    /// Nächster Turn — round-robin in Registration-Order (stabil, deterministisch).
    pub fn next_turn(&mut self) -> Option<SandboxId> {
        if self.order.is_empty() {
            return None;
        }
        let id = self.order[self.cursor % self.order.len()].clone();
        self.cursor = (self.cursor + 1) % self.order.len();
        Some(id)
    }

    /// Alle Sandboxes in Turn-Reihenfolge für genau einen Scheduler-Durchlauf.
    pub fn one_pass(&mut self) -> Vec<SandboxId> {
        let n = self.order.len();
        (0..n).filter_map(|_| self.next_turn()).collect()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
