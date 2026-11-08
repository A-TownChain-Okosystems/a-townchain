//! Mount-time recovery primitives for journaled filesystem state.

use crate::journal::{Journal, JournalError, JournalOp};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryError {
    Journal(JournalError),
    InvalidSequence,
}
impl From<JournalError> for RecoveryError {
    fn from(value: JournalError) -> Self {
        Self::Journal(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplayWrite {
    pub sequence: u64,
    pub block: u64,
    pub checksum: u32,
}

pub fn replay_pending(journal: &Journal) -> Result<Vec<ReplayWrite>, RecoveryError> {
    let records = journal.pending();
    let mut last = None;
    let mut out = Vec::with_capacity(records.len());
    for record in records {
        if let Some(previous) = last
            && record.sequence <= previous
        {
            return Err(RecoveryError::InvalidSequence);
        }
        last = Some(record.sequence);
        match record.op {
            JournalOp::Write => out.push(ReplayWrite {
                sequence: record.sequence,
                block: record.target_block,
                checksum: record.checksum,
            }),
            JournalOp::Clear => {}
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn replays_writes_and_ignores_clear() {
        let mut j = Journal::new(4).unwrap();
        j.append(8, JournalOp::Write).unwrap();
        j.append(8, JournalOp::Clear).unwrap();
        let writes = replay_pending(&j).unwrap();
        assert_eq!(writes.len(), 1);
        assert_eq!(writes[0].block, 8)
    }
}
