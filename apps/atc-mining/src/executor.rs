// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Ausfuehrungs-Executor (MIN-001..006, MVP).
//! GOVERNANCE: Mining FUEHRT Jobs aus — es definiert NIEMALS Konsensregeln
//! (SCR-0072-Entscheidung; kanonischer Konsens: atc-algorithm).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
    pub id: u64,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecError {
    QueueEmpty,
    InvalidJob,
}

pub struct Executor {
    queue: Vec<Job>,
    executed: Vec<Job>,
}

impl Executor {
    pub fn new() -> Self {
        Executor { queue: Vec::new(), executed: Vec::new() }
    }

    pub fn submit(&mut self, job: Job) -> Result<(), ExecError> {
        if job.payload.is_empty() {
            return Err(ExecError::InvalidJob);
        }
        self.queue.push(job);
        Ok(())
    }

    /// Fuehrt das naechste Job FIFO-maessig aus und protokolliert es.
    pub fn execute_next(&mut self) -> Result<Job, ExecError> {
        if self.queue.is_empty() {
            return Err(ExecError::QueueEmpty);
        }
        let job = self.queue.remove(0);
        self.executed.push(job.clone());
        Ok(job)
    }

    pub fn executed(&self) -> &[Job] {
        &self.executed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fifo_ordnung() {
        let mut e = Executor::new();
        e.submit(Job { id: 1, payload: vec![1] }).unwrap();
        e.submit(Job { id: 2, payload: vec![2] }).unwrap();
        assert_eq!(e.execute_next().unwrap().id, 1);
        assert_eq!(e.execute_next().unwrap().id, 2);
        assert_eq!(e.executed().len(), 2);
    }

    #[test]
    fn fehlerfaelle() {
        let mut e = Executor::new();
        assert_eq!(e.execute_next(), Err(ExecError::QueueEmpty));
        assert_eq!(e.submit(Job { id: 0, payload: vec![] }), Err(ExecError::InvalidJob));
    }
}
