//! Snapshot-Persistenz (G2-B, Issue #112).
//!
//! Zwei Integritätsmaßnahmen:
//! 1. **Write-Verify:** nach dem Speichern wird die Datei zurückgelesen und
//!    der State-Root neu berechnet — stimmt er nicht, ist das Schreiben
//!    gescheitert (fail-closed, kein blindes "fertig").
//! 2. **Load-Verify:** beim Laden wird der gespeicherte Root gegen den aus
//!    den Einträgen neu berechneten Root geprüft — Manipulation oder Korruption
//!    der Datei wird erkannt, bevor ein Zustand zurückgegeben wird.

use crate::state::{StateStore, Tx};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct PersistedSnapshot {
    height: u64,
    tip_block_hash: String,
    state_root: String,
    entries: BTreeMap<String, String>,
}

impl PersistedSnapshot {
    fn from_store(store: &StateStore, height: u64, tip_block_hash: &str) -> Self {
        Self {
            height,
            tip_block_hash: tip_block_hash.to_string(),
            state_root: store.state_root(),
            entries: store.entries().clone(),
        }
    }

    fn to_store(&self) -> StateStore {
        let mut store = StateStore::new();
        for (k, v) in &self.entries {
            store.apply(&Tx::Set {
                key: k.clone(),
                value: v.clone(),
            });
        }
        store
    }
}

/// Snapshot speichern — inkl. Write-Verify.
pub fn save_snapshot_store(
    store: &StateStore,
    height: u64,
    tip_block_hash: &str,
    path: &Path,
) -> io::Result<()> {
    let snap = PersistedSnapshot::from_store(store, height, tip_block_hash);
    let json = serde_json::to_string_pretty(&snap).map_err(io::Error::other)?;
    fs::write(path, json)?;
    // Write-Verify: zurücklesen, neu bauen, Root vergleichen
    let reread = load_snapshot_store(path)?;
    if reread.state_root() != store.state_root() {
        return Err(io::Error::other(
            "write-verify failed: state-root weicht nach Rücklesen ab",
        ));
    }
    Ok(())
}

/// Snapshot laden — inkl. Load-Verify (Tamper-Erkennung).
pub fn load_snapshot_store(path: &Path) -> io::Result<StateStore> {
    let raw = fs::read_to_string(path)?;
    let snap: PersistedSnapshot = serde_json::from_str(&raw).map_err(io::Error::other)?;
    let store = snap.to_store();
    if store.state_root() != snap.state_root {
        return Err(io::Error::other(format!(
            "load-verify failed: gespeicherter Root {} weicht ab",
            &snap.state_root[..16]
        )));
    }
    Ok(store)
}

/// Metadata-Reader (Höhe/Tip/Root ohne State-Rebuild).
pub fn snapshot_meta(path: &Path) -> io::Result<(u64, String, String)> {
    let raw = fs::read_to_string(path)?;
    let snap: PersistedSnapshot = serde_json::from_str(&raw).map_err(io::Error::other)?;
    Ok((snap.height, snap.tip_block_hash, snap.state_root))
}
