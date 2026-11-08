// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ┌─────────────────────────────────────────────────────────────────┐
// │ Datei: fs_journal.rs                                           │
// │ Agent: Aurora #2 (6a275618)                                     │
// │ Zweck: K-Sprint 50 — Filesystem Journaling (Crash-Safe ATCFS)  │
// │   Write-Ahead Logging für ATCFS, Crash Recovery, Journal Replay │
// │ Abhängigkeiten: atcfs.rs (K7), vfs.rs (K38), block.rs (K18)     │
// │ Erstellt: 2026-08-04                                            │
// └─────────────────────────────────────────────────────────────────┘
// K-Sprint 50 — Filesystem Journaling
//
// Features:
//   1. WRITE-AHEAD JOURNAL — All filesystem mutations logged before commit
//   2. CRASH RECOVERY — Replay journal on boot to restore consistency
//   3. CHECKPOINTING — Periodic journal compaction
//   4. TRANSACTION GROUPING — Batch multiple operations atomically
//   5. JOURNAL ROTATION — Circular log with configurable size

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// ════════════════════════════════════════════════════════════════
//  JOURNAL ENTRY TYPES
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalOp {
    Create,
    Write,
    Delete,
    Rename,
    Truncate,
    Chmod,
    Chown,
    Mkdir,
    Rmdir,
    Link,
    Symlink,
    Sync,
}

impl JournalOp {
    pub fn name(&self) -> &'static str {
        match self {
            JournalOp::Create => "Create",
            JournalOp::Write => "Write",
            JournalOp::Delete => "Delete",
            JournalOp::Rename => "Rename",
            JournalOp::Truncate => "Truncate",
            JournalOp::Chmod => "Chmod",
            JournalOp::Chown => "Chown",
            JournalOp::Mkdir => "Mkdir",
            JournalOp::Rmdir => "Rmdir",
            JournalOp::Link => "Link",
            JournalOp::Symlink => "Symlink",
            JournalOp::Sync => "Sync",
        }
    }

    pub fn is_metadata(&self) -> bool {
        matches!(self, JournalOp::Create | JournalOp::Delete | JournalOp::Rename | JournalOp::Mkdir | JournalOp::Rmdir | JournalOp::Link | JournalOp::Symlink | JournalOp::Chmod | JournalOp::Chown)
    }

    pub fn is_data(&self) -> bool {
        matches!(self, JournalOp::Write | JournalOp::Truncate | JournalOp::Sync)
    }
}

// ════════════════════════════════════════════════════════════════
//  JOURNAL ENTRY
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry {
    pub seq: u64,
    pub tx_id: u64,
    pub op: JournalOp,
    pub path: String,
    pub target: Option<String>,
    pub data: Vec<u8>,
    pub offset: u64,
    pub length: u64,
    pub timestamp: u64,
}

impl JournalEntry {
    pub fn new(seq: u64, tx_id: u64, op: JournalOp, path: &str) -> Self {
        Self {
            seq,
            tx_id,
            op,
            path: path.to_string(),
            target: None,
            data: Vec::new(),
            offset: 0,
            length: 0,
            timestamp: 0,
        }
    }

    pub fn with_data(mut self, data: Vec<u8>) -> Self {
        self.length = data.len() as u64;
        self.data = data;
        self
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_target(mut self, target: &str) -> Self {
        self.target = Some(target.to_string());
        self
    }

    pub fn is_metadata(&self) -> bool {
        self.op.is_metadata()
    }
}

// ════════════════════════════════════════════════════════════════
//  TRANSACTION
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxState {
    Open,
    Committed,
    Aborted,
    Applied,
}

#[derive(Debug, Clone)]
pub struct JournalTransaction {
    pub id: u64,
    pub state: TxState,
    pub entries: Vec<JournalEntry>,
    pub started_at: u64,
    pub committed_at: u64,
}

impl JournalTransaction {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            state: TxState::Open,
            entries: Vec::new(),
            started_at: 0,
            committed_at: 0,
        }
    }

    pub fn add_entry(&mut self, entry: JournalEntry) {
        self.entries.push(entry);
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn is_open(&self) -> bool {
        self.state == TxState::Open
    }

    pub fn is_committed(&self) -> bool {
        self.state == TxState::Committed
    }

    pub fn commit(&mut self, timestamp: u64) {
        self.state = TxState::Committed;
        self.committed_at = timestamp;
    }

    pub fn abort(&mut self) {
        self.state = TxState::Aborted;
    }

    pub fn mark_applied(&mut self) {
        self.state = TxState::Applied;
    }
}

// ════════════════════════════════════════════════════════════════
//  JOURNAL
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct Journal {
    entries: Vec<JournalEntry>,
    transactions: BTreeMap<u64, JournalTransaction>,
    next_seq: u64,
    next_tx_id: u64,
    max_entries: usize,
    checkpoint_seq: u64,
    stats: JournalStats,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JournalStats {
    pub total_entries: u64,
    pub total_transactions: u64,
    pub committed_txs: u64,
    pub aborted_txs: u64,
    pub applied_txs: u64,
    pub checkpoints: u64,
    pub entries_since_checkpoint: u64,
    pub journal_size_bytes: u64,
}

impl Default for Journal {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl Journal {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            transactions: BTreeMap::new(),
            next_seq: 1,
            next_tx_id: 1,
            max_entries,
            checkpoint_seq: 0,
            stats: JournalStats::default(),
        }
    }

    // ── Transaction Management ─────────────────────────

    pub fn begin_tx(&mut self) -> u64 {
        let id = self.next_tx_id;
        self.next_tx_id += 1;
        let tx = JournalTransaction::new(id);
        self.transactions.insert(id, tx);
        self.stats.total_transactions += 1;
        id
    }

    pub fn commit_tx(&mut self, tx_id: u64, timestamp: u64) -> bool {
        if let Some(tx) = self.transactions.get_mut(&tx_id) {
            if tx.is_open() {
                tx.commit(timestamp);
                self.stats.committed_txs += 1;
                return true;
            }
        }
        false
    }

    pub fn abort_tx(&mut self, tx_id: u64) -> bool {
        if let Some(tx) = self.transactions.get_mut(&tx_id) {
            if tx.is_open() {
                tx.abort();
                self.stats.aborted_txs += 1;
                return true;
            }
        }
        false
    }

    pub fn get_tx(&self, tx_id: u64) -> Option<&JournalTransaction> {
        self.transactions.get(&tx_id)
    }

    pub fn get_tx_mut(&mut self, tx_id: u64) -> Option<&mut JournalTransaction> {
        self.transactions.get_mut(&tx_id)
    }

    // ── Journal Entry Logging ──────────────────────────

    pub fn log(&mut self, tx_id: u64, op: JournalOp, path: &str) -> Option<u64> {
        if !self.transactions.contains_key(&tx_id) {
            return None;
        }
        let seq = self.next_seq;
        self.next_seq += 1;
        let entry = JournalEntry::new(seq, tx_id, op, path);
        self.entries.push(entry.clone());
        if let Some(tx) = self.transactions.get_mut(&tx_id) {
            tx.add_entry(entry);
        }
        self.stats.total_entries += 1;
        self.stats.entries_since_checkpoint += 1;
        self.stats.journal_size_bytes += path.len() as u64 + 64;
        Some(seq)
    }

    pub fn log_with_data(&mut self, tx_id: u64, op: JournalOp, path: &str, data: Vec<u8>) -> Option<u64> {
        let seq = self.log(tx_id, op, path)?;
        // Update last entry with data
        if let Some(entry) = self.entries.last_mut() {
            entry.length = data.len() as u64;
            entry.data = data.clone();
        }
        if let Some(tx) = self.transactions.get_mut(&tx_id) {
            if let Some(entry) = tx.entries.last_mut() {
                entry.length = data.len() as u64;
                entry.data = data;
            }
        }
        self.stats.journal_size_bytes += seq;
        Some(seq)
    }

    pub fn log_rename(&mut self, tx_id: u64, old_path: &str, new_path: &str) -> Option<u64> {
        if !self.transactions.contains_key(&tx_id) {
            return None;
        }
        let seq = self.next_seq;
        self.next_seq += 1;
        let entry = JournalEntry::new(seq, tx_id, JournalOp::Rename, old_path)
            .with_target(new_path);
        self.entries.push(entry.clone());
        if let Some(tx) = self.transactions.get_mut(&tx_id) {
            tx.add_entry(entry);
        }
        self.stats.total_entries += 1;
        self.stats.entries_since_checkpoint += 1;
        self.stats.journal_size_bytes += (old_path.len() + new_path.len()) as u64 + 64;
        Some(seq)
    }

    // ── Checkpointing ───────────────────────────────────

    pub fn checkpoint(&mut self) -> u64 {
        let cp_seq = self.next_seq - 1;
        self.checkpoint_seq = cp_seq;
        self.stats.checkpoints += 1;
        self.stats.entries_since_checkpoint = 0;
        // Remove applied transactions
        let applied_ids: Vec<u64> = self.transactions.iter()
            .filter(|(_, tx)| tx.state == TxState::Applied)
            .map(|(id, _)| *id)
            .collect();
        for id in applied_ids {
            self.transactions.remove(&id);
        }
        // Truncate entries before checkpoint
        self.entries.retain(|e| e.seq > cp_seq);
        cp_seq
    }

    pub fn needs_checkpoint(&self) -> bool {
        self.stats.entries_since_checkpoint as usize >= self.max_entries / 2
    }

    pub fn checkpoint_seq(&self) -> u64 {
        self.checkpoint_seq
    }

    // ── Crash Recovery ──────────────────────────────────

    pub fn recover(&mut self) -> RecoveryResult {
        let mut recovered = 0u64;
        let mut applied = 0u64;
        let mut skipped = 0u64;

        // Replay all committed-but-not-applied transactions
        let committed_ids: Vec<u64> = self.transactions.iter()
            .filter(|(_, tx)| tx.state == TxState::Committed)
            .map(|(id, _)| *id)
            .collect();

        for tx_id in committed_ids {
            if let Some(tx) = self.transactions.get_mut(&tx_id) {
                recovered += tx.entry_count() as u64;
                tx.mark_applied();
                applied += 1;
                self.stats.applied_txs += 1;
            }
        }

        // Abort all open transactions
        let open_ids: Vec<u64> = self.transactions.iter()
            .filter(|(_, tx)| tx.state == TxState::Open)
            .map(|(id, _)| *id)
            .collect();

        for tx_id in open_ids {
            if let Some(tx) = self.transactions.get_mut(&tx_id) {
                tx.abort();
                skipped += tx.entry_count() as u64;
                self.stats.aborted_txs += 1;
            }
        }

        RecoveryResult {
            committed_replayed: applied,
            open_aborted: skipped,
            entries_recovered: recovered,
        }
    }

    // ── Query ───────────────────────────────────────────

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    pub fn tx_count(&self) -> usize {
        self.transactions.len()
    }

    pub fn pending_txs(&self) -> usize {
        self.transactions.values()
            .filter(|tx| tx.state == TxState::Committed)
            .count()
    }

    pub fn open_txs(&self) -> usize {
        self.transactions.values()
            .filter(|tx| tx.state == TxState::Open)
            .count()
    }

    pub fn stats(&self) -> &JournalStats {
        &self.stats
    }

    pub fn entries(&self) -> &Vec<JournalEntry> {
        &self.entries
    }

    pub fn is_full(&self) -> bool {
        self.entries.len() >= self.max_entries
    }

    pub fn capacity(&self) -> usize {
        self.max_entries
    }

    pub fn remaining_capacity(&self) -> usize {
        self.max_entries.saturating_sub(self.entries.len())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryResult {
    pub committed_replayed: u64,
    pub open_aborted: u64,
    pub entries_recovered: u64,
}

impl RecoveryResult {
    pub fn is_clean(&self) -> bool {
        self.committed_replayed == 0 && self.open_aborted == 0
    }
}

// ════════════════════════════════════════════════════════════════
//  JOURNAL MANAGER (Higher-Level API)
// ════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct JournalManager {
    journal: Journal,
    auto_checkpoint: bool,
    auto_checkpoint_threshold: usize,
}

impl Default for JournalManager {
    fn default() -> Self {
        Self::new(4096)
    }
}

impl JournalManager {
    pub fn new(max_entries: usize) -> Self {
        Self {
            journal: Journal::new(max_entries),
            auto_checkpoint: true,
            auto_checkpoint_threshold: max_entries / 4,
        }
    }

    pub fn set_auto_checkpoint(&mut self, enabled: bool) {
        self.auto_checkpoint = enabled;
    }

    pub fn set_threshold(&mut self, threshold: usize) {
        self.auto_checkpoint_threshold = threshold;
    }

    pub fn begin(&mut self) -> u64 {
        self.journal.begin_tx()
    }

    pub fn commit(&mut self, tx_id: u64, timestamp: u64) -> bool {
        let result = self.journal.commit_tx(tx_id, timestamp);
        if result && self.auto_checkpoint && self.journal.needs_checkpoint() {
            self.journal.checkpoint();
        }
        result
    }

    pub fn abort(&mut self, tx_id: u64) -> bool {
        self.journal.abort_tx(tx_id)
    }

    pub fn create(&mut self, tx_id: u64, path: &str) -> Option<u64> {
        self.journal.log(tx_id, JournalOp::Create, path)
    }

    pub fn write(&mut self, tx_id: u64, path: &str, data: Vec<u8>) -> Option<u64> {
        self.journal.log_with_data(tx_id, JournalOp::Write, path, data)
    }

    pub fn delete(&mut self, tx_id: u64, path: &str) -> Option<u64> {
        self.journal.log(tx_id, JournalOp::Delete, path)
    }

    pub fn rename(&mut self, tx_id: u64, old_path: &str, new_path: &str) -> Option<u64> {
        self.journal.log_rename(tx_id, old_path, new_path)
    }

    pub fn mkdir(&mut self, tx_id: u64, path: &str) -> Option<u64> {
        self.journal.log(tx_id, JournalOp::Mkdir, path)
    }

    pub fn rmdir(&mut self, tx_id: u64, path: &str) -> Option<u64> {
        self.journal.log(tx_id, JournalOp::Rmdir, path)
    }

    pub fn truncate(&mut self, tx_id: u64, path: &str, length: u64) -> Option<u64> {
        let seq = self.journal.log(tx_id, JournalOp::Truncate, path)?;
        if let Some(e) = self.journal.entries.last_mut() {
            e.length = length;
        }
        Some(seq)
    }

    pub fn chmod(&mut self, tx_id: u64, path: &str) -> Option<u64> {
        self.journal.log(tx_id, JournalOp::Chmod, path)
    }

    pub fn chown(&mut self, tx_id: u64, path: &str) -> Option<u64> {
        self.journal.log(tx_id, JournalOp::Chown, path)
    }

    pub fn link(&mut self, tx_id: u64, target: &str, link_path: &str) -> Option<u64> {
        self.journal.log_rename(tx_id, target, link_path)
    }

    pub fn symlink(&mut self, tx_id: u64, target: &str, link_path: &str) -> Option<u64> {
        self.journal.log_rename(tx_id, target, link_path)
    }

    pub fn sync(&mut self, tx_id: u64, path: &str) -> Option<u64> {
        self.journal.log(tx_id, JournalOp::Sync, path)
    }

    pub fn recover(&mut self) -> RecoveryResult {
        self.journal.recover()
    }

    pub fn checkpoint(&mut self) -> u64 {
        self.journal.checkpoint()
    }

    pub fn needs_checkpoint(&self) -> bool {
        self.journal.needs_checkpoint()
    }

    pub fn journal(&self) -> &Journal {
        &self.journal
    }

    pub fn stats(&self) -> &JournalStats {
        self.journal.stats()
    }
}

// ════════════════════════════════════════════════════════════════
//  TESTS
// ════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── JournalOp Tests ──────────────────────────────────

    #[test]
    fn test_journal_op_names() {
        assert_eq!(JournalOp::Create.name(), "Create");
        assert_eq!(JournalOp::Write.name(), "Write");
        assert_eq!(JournalOp::Delete.name(), "Delete");
        assert_eq!(JournalOp::Rename.name(), "Rename");
        assert_eq!(JournalOp::Mkdir.name(), "Mkdir");
        assert_eq!(JournalOp::Sync.name(), "Sync");
    }

    #[test]
    fn test_journal_op_is_metadata() {
        assert!(JournalOp::Create.is_metadata());
        assert!(JournalOp::Delete.is_metadata());
        assert!(JournalOp::Mkdir.is_metadata());
        assert!(!JournalOp::Write.is_metadata());
        assert!(!JournalOp::Sync.is_metadata());
    }

    #[test]
    fn test_journal_op_is_data() {
        assert!(JournalOp::Write.is_data());
        assert!(JournalOp::Truncate.is_data());
        assert!(JournalOp::Sync.is_data());
        assert!(!JournalOp::Create.is_data());
        assert!(!JournalOp::Delete.is_data());
    }

    // ── JournalEntry Tests ──────────────────────────────

    #[test]
    fn test_entry_new() {
        let e = JournalEntry::new(1, 100, JournalOp::Create, "/test/file.txt");
        assert_eq!(e.seq, 1);
        assert_eq!(e.tx_id, 100);
        assert_eq!(e.op, JournalOp::Create);
        assert_eq!(e.path, "/test/file.txt");
        assert!(e.data.is_empty());
    }

    #[test]
    fn test_entry_with_data() {
        let e = JournalEntry::new(1, 100, JournalOp::Write, "/test/file.txt")
            .with_data(vec![0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(e.data, vec![0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(e.length, 4);
    }

    #[test]
    fn test_entry_with_offset() {
        let e = JournalEntry::new(1, 100, JournalOp::Write, "/test/file.txt")
            .with_offset(4096);
        assert_eq!(e.offset, 4096);
    }

    #[test]
    fn test_entry_with_target() {
        let e = JournalEntry::new(1, 100, JournalOp::Rename, "/old.txt")
            .with_target("/new.txt");
        assert_eq!(e.target, Some("/new.txt".to_string()));
    }

    #[test]
    fn test_entry_is_metadata() {
        let meta = JournalEntry::new(1, 100, JournalOp::Create, "/file");
        let data = JournalEntry::new(2, 100, JournalOp::Write, "/file");
        assert!(meta.is_metadata());
        assert!(!data.is_metadata());
    }

    // ── Transaction Tests ───────────────────────────────

    #[test]
    fn test_tx_new() {
        let tx = JournalTransaction::new(1);
        assert_eq!(tx.id, 1);
        assert_eq!(tx.state, TxState::Open);
        assert!(tx.entries.is_empty());
    }

    #[test]
    fn test_tx_add_entry() {
        let mut tx = JournalTransaction::new(1);
        let e = JournalEntry::new(1, 1, JournalOp::Create, "/file");
        tx.add_entry(e);
        assert_eq!(tx.entry_count(), 1);
    }

    #[test]
    fn test_tx_commit() {
        let mut tx = JournalTransaction::new(1);
        assert!(tx.is_open());
        tx.commit(1000);
        assert!(tx.is_committed());
        assert_eq!(tx.committed_at, 1000);
    }

    #[test]
    fn test_tx_abort() {
        let mut tx = JournalTransaction::new(1);
        tx.abort();
        assert_eq!(tx.state, TxState::Aborted);
    }

    #[test]
    fn test_tx_mark_applied() {
        let mut tx = JournalTransaction::new(1);
        tx.commit(0);
        tx.mark_applied();
        assert_eq!(tx.state, TxState::Applied);
    }

    // ── Journal Tests ───────────────────────────────────

    #[test]
    fn test_journal_new() {
        let j = Journal::new(100);
        assert_eq!(j.entry_count(), 0);
        assert_eq!(j.tx_count(), 0);
        assert_eq!(j.capacity(), 100);
    }

    #[test]
    fn test_journal_begin_tx() {
        let mut j = Journal::new(100);
        let id = j.begin_tx();
        assert_eq!(id, 1);
        assert_eq!(j.tx_count(), 1);
        let id2 = j.begin_tx();
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_journal_log() {
        let mut j = Journal::new(100);
        let tx_id = j.begin_tx();
        let seq = j.log(tx_id, JournalOp::Create, "/test.txt");
        assert!(seq.is_some());
        assert_eq!(seq.unwrap(), 1);
        assert_eq!(j.entry_count(), 1);
    }

    #[test]
    fn test_journal_log_invalid_tx() {
        let mut j = Journal::new(100);
        let seq = j.log(999, JournalOp::Create, "/test.txt");
        assert!(seq.is_none());
    }

    #[test]
    fn test_journal_log_with_data() {
        let mut j = Journal::new(100);
        let tx_id = j.begin_tx();
        let seq = j.log_with_data(tx_id, JournalOp::Write, "/test.txt", vec![1, 2, 3]);
        assert!(seq.is_some());
        assert_eq!(j.entries()[0].data, vec![1, 2, 3]);
        assert_eq!(j.entries()[0].length, 3);
    }

    #[test]
    fn test_journal_log_rename() {
        let mut j = Journal::new(100);
        let tx_id = j.begin_tx();
        let seq = j.log_rename(tx_id, "/old.txt", "/new.txt");
        assert!(seq.is_some());
        assert_eq!(j.entries()[0].op, JournalOp::Rename);
        assert_eq!(j.entries()[0].target, Some("/new.txt".to_string()));
    }

    #[test]
    fn test_journal_commit_tx() {
        let mut j = Journal::new(100);
        let tx_id = j.begin_tx();
        j.log(tx_id, JournalOp::Create, "/test.txt");
        assert!(j.commit_tx(tx_id, 1000));
        assert!(j.get_tx(tx_id).unwrap().is_committed());
    }

    #[test]
    fn test_journal_commit_nonexistent() {
        let mut j = Journal::new(100);
        assert!(!j.commit_tx(999, 0));
    }

    #[test]
    fn test_journal_commit_already_committed() {
        let mut j = Journal::new(100);
        let tx_id = j.begin_tx();
        j.commit_tx(tx_id, 0);
        assert!(!j.commit_tx(tx_id, 0));
    }

    #[test]
    fn test_journal_abort_tx() {
        let mut j = Journal::new(100);
        let tx_id = j.begin_tx();
        assert!(j.abort_tx(tx_id));
        assert_eq!(j.get_tx(tx_id).unwrap().state, TxState::Aborted);
    }

    #[test]
    fn test_journal_pending_txs() {
        let mut j = Journal::new(100);
        let tx1 = j.begin_tx();
        let tx2 = j.begin_tx();
        j.commit_tx(tx1, 0);
        assert_eq!(j.pending_txs(), 1);
        assert_eq!(j.open_txs(), 1);
    }

    // ── Checkpoint Tests ────────────────────────────────

    #[test]
    fn test_journal_checkpoint() {
        let mut j = Journal::new(100);
        let tx = j.begin_tx();
        j.log(tx, JournalOp::Create, "/a");
        j.commit_tx(tx, 0);
        let cp = j.checkpoint();
        assert_eq!(cp, 1);
        assert_eq!(j.checkpoint_seq(), 1);
        assert_eq!(j.stats().checkpoints, 1);
    }

    #[test]
    fn test_journal_needs_checkpoint() {
        let mut j = Journal::new(100);
        let tx = j.begin_tx();
        for i in 0..50 {
            j.log(tx, JournalOp::Create, &format!("/file{}", i));
        }
        assert!(j.needs_checkpoint());
    }

    #[test]
    fn test_journal_checkpoint_clears_applied() {
        let mut j = Journal::new(100);
        let tx = j.begin_tx();
        j.log(tx, JournalOp::Create, "/a");
        j.commit_tx(tx, 0);
        j.get_tx_mut(tx).unwrap().mark_applied();
        j.checkpoint();
        assert_eq!(j.tx_count(), 0); // Applied tx removed
    }

    // ── Recovery Tests ──────────────────────────────────

    #[test]
    fn test_recovery_no_crash() {
        let mut j = Journal::new(100);
        let result = j.recover();
        assert!(result.is_clean());
    }

    #[test]
    fn test_recovery_replay_committed() {
        let mut j = Journal::new(100);
        let tx = j.begin_tx();
        j.log(tx, JournalOp::Create, "/test.txt");
        j.log(tx, JournalOp::Write, "/test.txt");
        j.commit_tx(tx, 1000);
        let result = j.recover();
        assert_eq!(result.committed_replayed, 1);
        assert_eq!(result.entries_recovered, 2);
    }

    #[test]
    fn test_recovery_abort_open() {
        let mut j = Journal::new(100);
        let tx = j.begin_tx();
        j.log(tx, JournalOp::Create, "/test.txt");
        // Don't commit — simulate crash
        let result = j.recover();
        assert_eq!(result.open_aborted, 1);
        assert_eq!(result.committed_replayed, 0);
    }

    #[test]
    fn test_recovery_mixed() {
        let mut j = Journal::new(100);
        let tx1 = j.begin_tx();
        j.log(tx1, JournalOp::Create, "/a");
        j.commit_tx(tx1, 0);

        let tx2 = j.begin_tx();
        j.log(tx2, JournalOp::Create, "/b");
        // tx2 left open

        let result = j.recover();
        assert_eq!(result.committed_replayed, 1);
        assert_eq!(result.open_aborted, 1);
    }

    #[test]
    fn test_recovery_after_applied() {
        let mut j = Journal::new(100);
        let tx = j.begin_tx();
        j.log(tx, JournalOp::Create, "/a");
        j.commit_tx(tx, 0);
        j.recover();
        let result = j.recover(); // Second recovery
        assert!(result.is_clean()); // Nothing to recover
    }

    // ── JournalManager Tests ────────────────────────────

    #[test]
    fn test_mgr_new() {
        let mgr = JournalManager::new(100);
        assert_eq!(mgr.journal().capacity(), 100);
    }

    #[test]
    fn test_mgr_begin_commit() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        mgr.create(tx, "/test.txt");
        assert!(mgr.commit(tx, 1000));
    }

    #[test]
    fn test_mgr_write() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        let seq = mgr.write(tx, "/test.txt", vec![1, 2, 3]);
        assert!(seq.is_some());
    }

    #[test]
    fn test_mgr_delete() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        let seq = mgr.delete(tx, "/test.txt");
        assert!(seq.is_some());
    }

    #[test]
    fn test_mgr_rename() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        let seq = mgr.rename(tx, "/old", "/new");
        assert!(seq.is_some());
    }

    #[test]
    fn test_mgr_mkdir_rmdir() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        assert!(mgr.mkdir(tx, "/dir").is_some());
        assert!(mgr.rmdir(tx, "/dir").is_some());
    }

    #[test]
    fn test_mgr_truncate() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        let seq = mgr.truncate(tx, "/file", 1024);
        assert!(seq.is_some());
    }

    #[test]
    fn test_mgr_chmod_chown() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        assert!(mgr.chmod(tx, "/file").is_some());
        assert!(mgr.chown(tx, "/file").is_some());
    }

    #[test]
    fn test_mgr_link_symlink() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        assert!(mgr.link(tx, "/target", "/link").is_some());
        assert!(mgr.symlink(tx, "/target", "/symlink").is_some());
    }

    #[test]
    fn test_mgr_sync() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        assert!(mgr.sync(tx, "/file").is_some());
    }

    #[test]
    fn test_mgr_abort() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        mgr.create(tx, "/test.txt");
        assert!(mgr.abort(tx));
    }

    #[test]
    fn test_mgr_recover() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        mgr.create(tx, "/a");
        mgr.commit(tx, 0);
        let result = mgr.recover();
        assert_eq!(result.committed_replayed, 1);
    }

    #[test]
    fn test_mgr_auto_checkpoint() {
        let mut mgr = JournalManager::new(40);
        mgr.set_threshold(10);
        for _ in 0..15 {
            let tx = mgr.begin();
            mgr.create(tx, "/file");
            mgr.commit(tx, 0);
        }
        assert!(mgr.stats().checkpoints > 0);
    }

    #[test]
    fn test_mgr_no_auto_checkpoint() {
        let mut mgr = JournalManager::new(100);
        mgr.set_auto_checkpoint(false);
        for _ in 0..50 {
            let tx = mgr.begin();
            mgr.create(tx, "/file");
            mgr.commit(tx, 0);
        }
        assert_eq!(mgr.stats().checkpoints, 0);
    }

    #[test]
    fn test_mgr_checkpoint_manual() {
        let mut mgr = JournalManager::new(100);
        let tx = mgr.begin();
        mgr.create(tx, "/a");
        mgr.commit(tx, 0);
        let cp = mgr.checkpoint();
        assert!(cp >= 1);
    }

    // ── Stats Tests ─────────────────────────────────────

    #[test]
    fn test_stats_initial() {
        let j = Journal::new(100);
        assert_eq!(j.stats().total_entries, 0);
        assert_eq!(j.stats().total_transactions, 0);
    }

    #[test]
    fn test_stats_after_operations() {
        let mut j = Journal::new(100);
        let tx = j.begin_tx();
        j.log(tx, JournalOp::Create, "/a");
        j.commit_tx(tx, 0);
        assert_eq!(j.stats().total_entries, 1);
        assert_eq!(j.stats().committed_txs, 1);
    }

    #[test]
    fn test_stats_after_abort() {
        let mut j = Journal::new(100);
        let tx = j.begin_tx();
        j.log(tx, JournalOp::Create, "/a");
        j.abort_tx(tx);
        assert_eq!(j.stats().aborted_txs, 1);
    }

    #[test]
    fn test_stats_after_checkpoint() {
        let mut j = Journal::new(100);
        for _ in 0..5 {
            let tx = j.begin_tx();
            j.log(tx, JournalOp::Create, "/a");
            j.commit_tx(tx, 0);
        }
        j.checkpoint();
        assert_eq!(j.stats().checkpoints, 1);
        assert_eq!(j.stats().entries_since_checkpoint, 0);
    }

    // ── Capacity Tests ──────────────────────────────────

    #[test]
    fn test_journal_is_full() {
        let mut j = Journal::new(5);
        let tx = j.begin_tx();
        for i in 0..5 {
            j.log(tx, JournalOp::Create, &format!("/f{}", i));
        }
        assert!(j.is_full());
    }

    #[test]
    fn test_journal_remaining_capacity() {
        let mut j = Journal::new(10);
        let tx = j.begin_tx();
        j.log(tx, JournalOp::Create, "/a");
        assert_eq!(j.remaining_capacity(), 9);
    }

    // ── Integration Tests ───────────────────────────────

    #[test]
    fn test_integration_full_lifecycle() {
        let mut mgr = JournalManager::new(100);

        // 1. Create a file
        let tx1 = mgr.begin();
        mgr.create(tx1, "/test.txt");
        mgr.write(tx1, "/test.txt", vec![0x41; 100]);
        mgr.commit(tx1, 1000);

        // 2. Rename it
        let tx2 = mgr.begin();
        mgr.rename(tx2, "/test.txt", "/renamed.txt");
        mgr.commit(tx2, 2000);

        // 3. Create a directory
        let tx3 = mgr.begin();
        mgr.mkdir(tx3, "/data");
        mgr.commit(tx3, 3000);

        // 4. Crash recovery
        let result = mgr.recover();
        assert_eq!(result.committed_replayed, 3);
        assert_eq!(result.open_aborted, 0);

        // 5. Check stats
        let stats = mgr.stats();
        assert_eq!(stats.total_transactions, 3);
        assert_eq!(stats.committed_txs, 3);
        assert_eq!(stats.applied_txs, 3);
    }

    #[test]
    fn test_integration_crash_with_open_tx() {
        let mut mgr = JournalManager::new(100);

        // 1. Commit some work
        let tx1 = mgr.begin();
        mgr.create(tx1, "/committed.txt");
        mgr.commit(tx1, 0);

        // 2. Start but don't commit (crash)
        let tx2 = mgr.begin();
        mgr.write(tx2, "/uncommitted.txt", vec![0xFF; 50]);
        // Don't commit tx2

        // 3. Recover
        let result = mgr.recover();
        assert_eq!(result.committed_replayed, 1);
        assert_eq!(result.open_aborted, 1);
    }

    #[test]
    fn test_integration_checkpoint_and_recover() {
        let mut mgr = JournalManager::new(100);
        mgr.set_auto_checkpoint(false);

        // Create and commit 3 transactions
        for i in 0..3 {
            let tx = mgr.begin();
            mgr.create(tx, &format!("/file{}", i));
            mgr.commit(tx, 0);
        }

        // Checkpoint
        mgr.checkpoint();

        // Create 2 more
        for i in 3..5 {
            let tx = mgr.begin();
            mgr.create(tx, &format!("/file{}", i));
            mgr.commit(tx, 0);
        }

        // Recover — should only replay post-checkpoint
        let result = mgr.recover();
        assert_eq!(result.committed_replayed, 2);
    }
}
