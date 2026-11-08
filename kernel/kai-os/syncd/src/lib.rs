//! Sync-Daemon (Issue #113, GATE-KAI-001 Kat. 12) — Multi-Node State-Sync.
//!
//! Prinzip (Kat. 11, vererbt): **Verifikation statt Vertrauen.**
//! Der Sink baut den empfangenen Snapshot selbst neu auf, berechnet den
//! State-Root und vergleicht — ein abweichender Root wird ABGEWIESEN,
//! nie übernommen. Nach bestandener Prüfung persistiert der Sink über
//! sein eigenes Marker-Protokoll (G2-D).
//!
//! Disaster Recovery: Absturz eines Knotens verliert nichts — beide Seiten
//! recovern aus ihrem eigenen WAL/Snapshot und konvergieren beim Re-Sync.

use kai_os_network::tcp::TcpPeer;
use kai_os_state::state::{StateStore, Tx};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const SYNC_TIMEOUT: Duration = Duration::from_secs(5);

/// Snapshot-Nachricht über die Leitung: Einträge + Root (zu verifizieren).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncSnapshot {
    pub height: u64,
    pub tip_block_hash: String,
    pub state_root: String,
    /// Sortierte Einträge (BTreeMap-Reihenfolge — deterministisch).
    pub entries: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncAck {
    pub state_root: String,
}

#[derive(Debug)]
pub enum SyncError {
    Io(std::io::Error),
    Tcp(kai_os_network::tcp::TcpError),
    /// Übermittelter Root stimmt nicht mit dem aus den Einträgen
    /// neu berechneten Root überein — abgewiesen (Verifikation statt Vertrauen).
    RootMismatch {
        declared: String,
        computed: String,
    },
    /// Ack-Root weicht vom eigenen Root ab.
    AckMismatch {
        expected: String,
        got: String,
    },
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncError::Io(e) => write!(f, "io: {e}"),
            SyncError::Tcp(e) => write!(f, "tcp: {e}"),
            SyncError::RootMismatch { declared, computed } => write!(
                f,
                "root-mismatch: deklariert {declared}, berechnet {computed} — abgewiesen"
            ),
            SyncError::AckMismatch { expected, got } => {
                write!(f, "ack-mismatch: erwartet {expected}, erhalten {got}")
            }
        }
    }
}

impl From<kai_os_network::tcp::TcpError> for SyncError {
    fn from(e: kai_os_network::tcp::TcpError) -> Self {
        SyncError::Tcp(e)
    }
}

/// Snapshot aus einem Store bauen (Einträge sortiert, Root vom Original).
pub fn snapshot_from_store(store: &StateStore, height: u64, tip_block_hash: &str) -> SyncSnapshot {
    SyncSnapshot {
        height,
        tip_block_hash: tip_block_hash.to_string(),
        state_root: store.state_root(),
        entries: store
            .entries()
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect(),
    }
}

/// Verifikation auf der Sink-Seite: Neuaufbau + Root-Vergleich.
/// Rückgabe: der verifizierte Store (nur bei Übereinstimmung).
pub fn verify_snapshot(snap: &SyncSnapshot) -> Result<StateStore, SyncError> {
    let mut rebuilt = StateStore::new();
    for (k, v) in &snap.entries {
        rebuilt.apply(&Tx::Set {
            key: k.clone(),
            value: v.clone(),
        });
    }
    let computed = rebuilt.state_root();
    if computed != snap.state_root {
        return Err(SyncError::RootMismatch {
            declared: snap.state_root.clone(),
            computed,
        });
    }
    Ok(rebuilt)
}

/// Sink: einen Snapshot empfangen, verifizieren, acken.
/// Liefert den VERIFIZIERTEN Store (oder Fehler — kein Teilzustand).
pub fn receive_and_verify(peer: &mut TcpPeer) -> Result<SyncSnapshot, SyncError> {
    let bytes = peer.receive()?;
    let snap: SyncSnapshot =
        serde_json::from_slice(&bytes).map_err(|_| SyncError::RootMismatch {
            declared: "unparsebar".into(),
            computed: "-".into(),
        })?;
    let verified = verify_snapshot(&snap)?;
    let _ = verified; // Verifikation bestanden — Store wird vom Aufrufer übernommen
    Ok(snap)
}

/// Sink: verifizierten Zustand über PersistentState persistieren (Marker-Protokoll, G2-D).
pub fn persist_verified(
    ps: &mut kai_os_state::wal::PersistentState,
    snap: &SyncSnapshot,
) -> Result<(), SyncError> {
    ps.store = verify_snapshot(snap)?;
    let state_path = ps.state_path_hint().to_path_buf();
    ps.checkpoint(snap.height, &snap.tip_block_hash, &state_path)
        .map_err(|e| SyncError::Io(std::io::Error::other(e.to_string())))?;
    Ok(())
}

/// Source: Snapshot senden, Ack empfangen und gegen eigenen Root prüfen.
pub fn send_and_await_ack(
    peer: &mut TcpPeer,
    store: &StateStore,
    height: u64,
    tip: &str,
) -> Result<(), SyncError> {
    let snap = snapshot_from_store(store, height, tip);
    peer.send(&serde_json::to_vec(&snap).map_err(|e| SyncError::Io(std::io::Error::other(e)))?)?;
    let ack_bytes = peer.receive()?;
    let ack: SyncAck = serde_json::from_slice(&ack_bytes).map_err(|_| SyncError::AckMismatch {
        expected: store.state_root(),
        got: "unparsebar".into(),
    })?;
    if ack.state_root != store.state_root() {
        return Err(SyncError::AckMismatch {
            expected: store.state_root(),
            got: ack.state_root,
        });
    }
    Ok(())
}
