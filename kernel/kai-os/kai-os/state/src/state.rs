//! StateStore: deterministischer Key-Value-State (BTreeMap) + Tx-Modell.

use crate::merkle::merkle_root;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tx {
    Set { key: String, value: String },
    Remove { key: String },
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct StateStore {
    entries: BTreeMap<String, String>,
}

impl StateStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, tx: &Tx) {
        match tx {
            Tx::Set { key, value } => {
                self.entries.insert(key.clone(), value.clone());
            }
            Tx::Remove { key } => {
                self.entries.remove(key);
            }
        }
    }

    pub fn apply_all(&mut self, txs: &[Tx]) {
        for tx in txs {
            self.apply(tx);
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.entries
    }

    /// Deterministischer State-Root (Merkle).
    pub fn state_root(&self) -> String {
        merkle_root(&self.entries)
    }
}
