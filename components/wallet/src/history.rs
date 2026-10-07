// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Deterministic in-memory transaction history.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryEntry {
    pub tx_id: [u8; 32],
    pub height: u64,
    pub timestamp: u64,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TransactionHistory { entries: Vec<HistoryEntry> }

impl TransactionHistory {
    pub fn push(&mut self, entry: HistoryEntry) -> Result<(), HistoryError> {
        if self.entries.last().is_some_and(|last| entry.height < last.height) {
            return Err(HistoryError::NonMonotonicHeight);
        }
        self.entries.push(entry);
        Ok(())
    }
    pub fn entries(&self) -> &[HistoryEntry] { &self.entries }
    pub fn len(&self) -> usize { self.entries.len() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryError { NonMonotonicHeight }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn history_is_monotonic() {
        let mut history = TransactionHistory::default();
        assert!(history.push(HistoryEntry { tx_id: [1; 32], height: 2, timestamp: 2 }).is_ok());
        assert!(history.push(HistoryEntry { tx_id: [2; 32], height: 1, timestamp: 3 }).is_err());
    }
}
