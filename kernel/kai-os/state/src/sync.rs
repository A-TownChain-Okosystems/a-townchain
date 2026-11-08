//! SyncEngine: strikte, fail-close Block-Anwendung.
//! Ein Block wird NUR akzeptiert, wenn:
//! 1. Höhe == lokale Höhe + 1 (keine Lücken, keine Sprünge)
//! 2. prev_hash == Hash des letzten Blocks (Chain-Kontinuität)
//! 3. recomputeter State-Root == claimed state_root (NIEMALS vertrauen)
//!
//! Atomicität: ungültige Blöcke verändern den State nicht.

use crate::state::{StateStore, Tx};
use serde::{Deserialize, Serialize};

/// Prev-Hash von Block 1 (Genesis-Anfang).
pub const GENESIS_PREV: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Block {
    pub height: u64,
    pub prev_hash: String,
    pub txs: Vec<Tx>,
    pub state_root: String,
}

impl Block {
    /// Deterministischer Block-Hash.
    pub fn block_hash(&self) -> String {
        let txs: Vec<String> = self
            .txs
            .iter()
            .map(|t| match t {
                Tx::Set { key, value } => format!("set:{key}={value}"),
                Tx::Remove { key } => format!("rm:{key}"),
            })
            .collect();
        crate::sha_hex(
            format!(
                "{}|{}|{}|{}",
                self.height,
                self.prev_hash,
                txs.join(";"),
                self.state_root
            )
            .as_bytes(),
        )
    }
}

#[derive(Debug)]
pub enum SyncError {
    HeightGap {
        expected: u64,
        got: u64,
    },
    Duplicate {
        height: u64,
    },
    Fork {
        height: u64,
        known: String,
        got: String,
    },
    PrevMismatch {
        expected: String,
        got: String,
    },
    RootMismatch {
        claimed: String,
        computed: String,
    },
    SnapshotHeightMismatch {
        expected: u64,
        got: u64,
    },
    SnapshotRootMismatch {
        claimed: String,
        computed: String,
    },
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncError::HeightGap { expected, got } => {
                write!(f, "height gap: expected {expected}, got {got}")
            }
            SyncError::Duplicate { height } => write!(f, "duplicate block at height {height}"),
            SyncError::Fork { height, known, got } => {
                write!(f, "fork at {height}: known {known}, got {got}")
            }
            SyncError::PrevMismatch { expected, got } => {
                write!(f, "prev mismatch: expected {expected}, got {got}")
            }
            SyncError::RootMismatch { claimed, computed } => {
                write!(f, "state root mismatch: claimed {claimed}, computed {computed} (block NOT trusted)")
            }
            SyncError::SnapshotHeightMismatch { expected, got } => write!(
                f,
                "snapshot height mismatch: expected {expected}, got {got}"
            ),
            SyncError::SnapshotRootMismatch { claimed, computed } => {
                write!(
                    f,
                    "snapshot root mismatch: claimed {claimed}, computed {computed}"
                )
            }
        }
    }
}

pub struct SyncEngine {
    pub tip_height: u64,
    pub tip_block_hash: String,
    /// Höhe -> Block-Hash (für Fork-Detection).
    applied: std::collections::BTreeMap<u64, String>,
    pub state: StateStore,
}

impl SyncEngine {
    /// Engine mit bestehendem (verifiziertem) Start-State.
    pub fn new(state: StateStore) -> Self {
        Self {
            tip_height: 0,
            tip_block_hash: GENESIS_PREV.to_string(),
            applied: std::collections::BTreeMap::new(),
            state,
        }
    }

    pub fn state_root(&self) -> String {
        self.state.state_root()
    }

    /// Block anwenden — strikt, verifizierend, atomar.
    pub fn apply_block(&mut self, block: &Block) -> Result<String, SyncError> {
        // Fork-Detection vor Höhen-Check: bekannter Block mit anderem Hash.
        if let Some(known) = self.applied.get(&block.height) {
            if *known != block.block_hash() {
                return Err(SyncError::Fork {
                    height: block.height,
                    known: known.clone(),
                    got: block.block_hash(),
                });
            }
            return Err(SyncError::Duplicate {
                height: block.height,
            });
        }
        // Strikte Höhe.
        if block.height != self.tip_height + 1 {
            return Err(SyncError::HeightGap {
                expected: self.tip_height + 1,
                got: block.height,
            });
        }
        // Prev-Hash-Kette.
        if block.prev_hash != self.tip_block_hash {
            return Err(SyncError::PrevMismatch {
                expected: self.tip_block_hash.clone(),
                got: block.prev_hash.clone(),
            });
        }
        // Atomic: Scratch-State, Commit erst nach Root-Verifikation.
        let mut scratch = self.state.clone();
        scratch.apply_all(&block.txs);
        let computed = scratch.state_root();
        if computed != block.state_root {
            return Err(SyncError::RootMismatch {
                claimed: block.state_root.clone(),
                computed,
            });
        }
        // Commit.
        self.state = scratch;
        self.tip_height = block.height;
        self.tip_block_hash = block.block_hash();
        self.applied.insert(block.height, block.block_hash());
        Ok(self.tip_block_hash.clone())
    }

    pub fn block_hash_at(&self, height: u64) -> Option<&String> {
        self.applied.get(&height)
    }
}
