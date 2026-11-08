//! Architecture-neutral local/IO APIC programming contract.

use crate::interrupt::InterruptVector;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ApicId(pub u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoApicRedirection {
    pub vector: InterruptVector,
    pub destination: ApicId,
    pub masked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApicError {
    InvalidId,
    InvalidVector,
    DuplicateVector,
    MissingVector,
}

#[derive(Debug, Default)]
pub struct ApicController {
    entries: Vec<IoApicRedirection>,
}

impl ApicController {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_redirection(&mut self, entry: IoApicRedirection) -> Result<(), ApicError> {
        if entry.destination.0 == u8::MAX {
            return Err(ApicError::InvalidId);
        }
        if !entry.vector.valid() {
            return Err(ApicError::InvalidVector);
        }
        if self.entries.iter().any(|e| e.vector == entry.vector) {
            return Err(ApicError::DuplicateVector);
        }
        self.entries.push(entry);
        self.entries.sort_by_key(|e| e.vector.0);
        Ok(())
    }

    pub fn get(&self, vector: InterruptVector) -> Result<IoApicRedirection, ApicError> {
        self.entries
            .iter()
            .find(|e| e.vector == vector)
            .copied()
            .ok_or(ApicError::MissingVector)
    }

    pub fn mask(&mut self, vector: InterruptVector, masked: bool) -> Result<(), ApicError> {
        let entry = self
            .entries
            .iter_mut()
            .find(|e| e.vector == vector)
            .ok_or(ApicError::MissingVector)?;
        entry.masked = masked;
        Ok(())
    }

    pub fn entries(&self) -> &[IoApicRedirection] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn redirection_is_deterministic() {
        let mut a = ApicController::new();
        a.set_redirection(IoApicRedirection {
            vector: InterruptVector(48),
            destination: ApicId(1),
            masked: true,
        })
        .unwrap();
        assert!(a.get(InterruptVector(48)).unwrap().masked);
        a.mask(InterruptVector(48), false).unwrap();
        assert!(!a.get(InterruptVector(48)).unwrap().masked);
    }
}
