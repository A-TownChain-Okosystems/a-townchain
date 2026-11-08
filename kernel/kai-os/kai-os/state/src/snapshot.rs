//! Snapshot: vollständiger State-Dump mit Höhe, Tip-Hash und State-Root.
//! Ein Snapshot wird NIE vertraut — verify() rechnet den Root neu.

use crate::merkle::{merkle_root, EMPTY_ROOT};
use crate::state::StateStore;
use crate::sync::SyncError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub height: u64,
    pub tip_block_hash: String,
    pub state_root: String,
    pub entries: Vec<(String, String)>,
}

impl Snapshot {
    pub fn from_state(height: u64, tip_block_hash: &str, state: &StateStore) -> Self {
        Self {
            height,
            tip_block_hash: tip_block_hash.into(),
            state_root: state.state_root(),
            entries: state
                .entries()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        }
    }

    /// Deterministische Snapshot-ID.
    pub fn snapshot_id(&self) -> String {
        crate::sha_hex(
            format!(
                "{}|{}|{}",
                self.height, self.tip_block_hash, self.state_root
            )
            .as_bytes(),
        )
    }

    /// Verifikation gegen einen VERTRAUENSWÜRDIGEN Root (z. B. aus Chain-Header).
    /// Fail-closed: Root stimmt nicht überein → Fehler. Der Snapshot wird nie geglaubt.
    pub fn verify_against(
        &self,
        trusted_height: u64,
        trusted_state_root: &str,
    ) -> Result<(), SyncError> {
        if self.height != trusted_height {
            return Err(SyncError::SnapshotHeightMismatch {
                expected: trusted_height,
                got: self.height,
            });
        }
        let entries: BTreeMap<String, String> = self.entries.clone().into_iter().collect();
        let computed = if entries.is_empty() {
            EMPTY_ROOT.to_string()
        } else {
            merkle_root(&entries)
        };
        if computed != self.state_root {
            return Err(SyncError::SnapshotRootMismatch {
                claimed: self.state_root.clone(),
                computed,
            });
        }
        if computed != trusted_state_root {
            return Err(SyncError::SnapshotRootMismatch {
                claimed: computed,
                computed: trusted_state_root.to_string(),
            });
        }
        Ok(())
    }

    /// Verifizierten Snapshot in einen State überführen (nur nach verify_against!).
    pub fn to_state(&self) -> StateStore {
        let mut state = StateStore::new();
        for (k, v) in &self.entries {
            state.apply(&crate::state::Tx::Set {
                key: k.clone(),
                value: v.clone(),
            });
        }
        state
    }
}
