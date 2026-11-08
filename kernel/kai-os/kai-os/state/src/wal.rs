//! Write-Ahead-Log (G2-B, Issue #112) — Persistente Storage-Layer.
//!
//! WAL-Vertrag (Crash-Sicherheit):
//! 1. **WAL vor State:** jede Tx wird ZUERST in den Log geschrieben (inkl.
//!    `sync_data`), ERST DANACH in den StateStore angewendet — ein Absturz
//!    verliert niemals eine angewendete Änderung.
//! 2. **Hash-Kette:** jeder Record verketted den Hash des Vorgängers —
//!    Manipulation mitten im Log wird beim Replay fail-closed erkannt.
//! 3. **Torn Tail:** ein unvollständiger LETZTER Record (Absturz während des
//!    Schreibens) wird verworfen — das ist der Standard-WAL-Vertrag; Corruption
//!    MITTEN im Log ist ein Fehler.

use crate::state::Tx;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Eine WAL-Operation: normale Tx ODER Checkpoint-Marker.
/// Der Marker ist Teil der Hash-Kette — das WAL bleibt strikt append-only
/// (kein Truncate mehr, G2-D schliesst das G2-B-Fenster).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WalOp {
    Apply(Tx),
    /// Ab hier ist der Zustand als Snapshot gesichert (root verifizierbar).
    Checkpoint {
        height: u64,
        tip_block_hash: String,
        state_root: String,
    },
}

fn sha256_hex(input: &[u8]) -> String {
    let d = Sha256::digest(input);
    d.iter().map(|b| format!("{b:02x}")).collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalRecord {
    pub seq: u64,
    pub prev_hash: String,
    pub op: WalOp,
}

impl WalRecord {
    pub fn hash(&self) -> String {
        let payload = format!(
            "{}|{}|{}",
            self.seq,
            self.prev_hash,
            serde_json::to_string(&self.op).unwrap_or_default()
        );
        sha256_hex(payload.as_bytes())
    }
}

#[derive(Debug)]
pub enum WalError {
    Io(std::io::Error),
    /// Hash-Kette mitten im Log gebrochen — fail-closed, kein Blind-Replay.
    ChainBroken {
        line: usize,
        expected: String,
        got: String,
    },
    /// Record unverständlich (kein JSON) UND nicht der letzte — Corruption.
    MalformedRecord {
        line: usize,
    },
}

impl std::fmt::Display for WalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WalError::Io(e) => write!(f, "WAL I/O: {e}"),
            WalError::ChainBroken {
                line,
                expected,
                got,
            } => write!(
                f,
                "WAL-Chain gebrochen (Zeile {line}): erwartet {expected}, gefunden {got}"
            ),
            WalError::MalformedRecord { line } => write!(f, "WAL-Record unlesbar (Zeile {line})"),
        }
    }
}

/// Das WAL für einen StateStore.
pub struct WriteAheadLog {
    path: PathBuf,
    file: File,
    seq: u64,
    prev_hash: String,
}

pub const GENESIS_PREV: &str = "0000000000000000000000000000000000000000000000000000000000000000";

impl WriteAheadLog {
    /// WAL öffnen (erstellt bei Bedarf) und Konsistenz prüfen.
    pub fn open(path: &Path) -> Result<Self, WalError> {
        let (seq, prev_hash) = Self::scan_tail(path)?;
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(WalError::Io)?;
        Ok(Self {
            path: path.to_path_buf(),
            file,
            seq,
            prev_hash,
        })
    }

    /// Tx anlegen: WAL zuerst (durable), dann erst gilt sie als angewendet.
    pub fn append(&mut self, tx: &Tx) -> Result<u64, WalError> {
        self.append_op(WalOp::Apply(tx.clone()))
    }

    /// Beliebige Op anlegen (Apply oder Checkpoint-Marker) — derselbe Ketten-Vertrag.
    pub fn append_op(&mut self, op: WalOp) -> Result<u64, WalError> {
        self.seq += 1;
        let record = WalRecord {
            seq: self.seq,
            prev_hash: self.prev_hash.clone(),
            op,
        };
        let hash = record.hash();
        let line =
            serde_json::to_string(&record).map_err(|e| WalError::Io(std::io::Error::other(e)))?;
        writeln!(self.file, "{line}").map_err(WalError::Io)?;
        self.file.sync_data().map_err(WalError::Io)?; // DURABILITY vor Rückkehr
        self.prev_hash = hash;
        Ok(self.seq)
    }

    /// Ende der Kette (für Recovery-Prüfung gegen Snapshot).
    pub fn tip_hash(&self) -> &str {
        &self.prev_hash
    }

    pub fn seq(&self) -> u64 {
        self.seq
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Komplettes Log lesen: verifiziert die Hash-Kette komplett durch.
    /// Ein unvollständiger LETZTER Record (torn tail) wird verworfen.
    /// Rückgabe: alle Ops (inkl. Marker), Kettenende, Tip-Hash.
    pub fn replay(path: &Path) -> Result<(Vec<WalOp>, u64, String), WalError> {
        let file = File::open(path).map_err(WalError::Io)?;
        let reader = BufReader::new(file);
        let mut ops = Vec::new();
        let mut seq = 0u64;
        let mut prev_hash = GENESIS_PREV.to_string();
        let mut line_no = 0usize;
        for line in reader.lines() {
            let line = line.map_err(WalError::Io)?;
            line_no += 1;
            if line.trim().is_empty() {
                continue;
            }
            let record: WalRecord = match serde_json::from_str(&line) {
                Ok(r) => r,
                // Torn tail: unlesbarer Record NUR an letzter Stelle -> verwerfen.
                Err(_) => {
                    let is_last = Self::is_last_line(path, line_no);
                    if is_last {
                        return Ok((ops, seq, prev_hash)); // torn tail verworfen
                    }
                    return Err(WalError::MalformedRecord { line: line_no });
                }
            };
            // Kette prüfen: seq aufsteigend + prev_hash korrekt + eigener Hash stimmt
            if record.seq != seq + 1 {
                return Err(WalError::ChainBroken {
                    line: line_no,
                    expected: format!("seq {}", seq + 1),
                    got: format!("seq {}", record.seq),
                });
            }
            if record.prev_hash != prev_hash {
                return Err(WalError::ChainBroken {
                    line: line_no,
                    expected: prev_hash.clone(),
                    got: record.prev_hash.clone(),
                });
            }
            prev_hash = record.hash();
            seq = record.seq;
            ops.push(record.op);
        }
        Ok((ops, seq, prev_hash))
    }

    /// Beim Öffnen: letztes gültiges Kettenende ermitteln.
    fn scan_tail(path: &Path) -> Result<(u64, String), WalError> {
        if !path.exists() {
            return Ok((0, GENESIS_PREV.to_string()));
        }
        let (_, seq, prev_hash) = Self::replay(path)?;
        Ok((seq, prev_hash))
    }

    fn is_last_line(path: &Path, line_no: usize) -> bool {
        // Grobe Prüfung: Anzahl Zeilen zählen — torn tail liegt immer am Dateiende.
        match fs::File::open(path) {
            Ok(f) => BufReader::new(f).lines().count() <= line_no,
            Err(_) => false,
        }
    }
}

/// PersistentState: StateStore mit WAL-Garantie (WAL vor State).
pub struct PersistentState {
    pub store: crate::state::StateStore,
    wal: WriteAheadLog,
    state_path_hint: PathBuf,
}

/// Letzte verifizierte Zustandsgrenze (Rollback-Ziel, kein semantisches Undo).
#[derive(Debug, Clone, PartialEq)]
pub struct VerifiedBoundary {
    pub height: u64,
    pub tip_block_hash: String,
    pub state_root: String,
}

impl PersistentState {
    /// Recovery — atomar, OHNE Absturzfenster (G2-D):
    ///
    /// Marker-Protokoll (WAL strikt append-only, kein Truncate):
    /// 1. WAL komplett verifiziert lesen; LETZTEN Checkpoint-Marker suchen.
    /// 2. Marker vorhanden UND Snapshot lädt UND Roots stimmen überein
    ///    → Basis = Snapshot, nur Ops NACH dem Marker anwenden.
    /// 3. Sonst (kein Marker / Snapshot korrupt / Absturz vor Snapshot-Sync)
    ///    → Basis = Genesis, ALLE Apply-Ops von Anfang an replays.
    ///
    /// Beide Pfade sind korrekt; ein Absturz an JEDER Stelle des Checkpoints
    /// landet in einem der beiden korrekten Pfade — das G2-B-Fenster ist geschlossen.
    pub fn open(state_path: &Path, wal_path: &Path) -> Result<Self, WalError> {
        let (ops, _seq, _tip) = if wal_path.exists() {
            WriteAheadLog::replay(wal_path)?
        } else {
            (Vec::new(), 0, GENESIS_PREV.to_string())
        };
        // Letzten Marker suchen (Kette ist geordnet, letzter = relevantester)
        let marker = ops.iter().rev().find_map(|op| match op {
            WalOp::Checkpoint {
                height,
                tip_block_hash,
                state_root,
            } => Some((*height, tip_block_hash.clone(), state_root.clone())),
            _ => None,
        });
        let mut store = crate::state::StateStore::new();
        let mut applied_from = 0usize; // Index, ab dem Ops angewendet werden
        if let Some((_h, _t, marker_root)) = &marker {
            if let Ok(snapshot_store) = crate::persistence::load_snapshot_store(state_path) {
                if snapshot_store.state_root() == *marker_root {
                    store = snapshot_store;
                    // nur der Suffix nach dem LETZTEN Marker zaehlt
                    let last_marker_idx = ops
                        .iter()
                        .rposition(|op| matches!(op, WalOp::Checkpoint { .. }))
                        .unwrap_or(0);
                    applied_from = last_marker_idx + 1;
                }
            }
        }
        for op in &ops[applied_from..] {
            if let WalOp::Apply(tx) = op {
                store.apply(tx);
            }
        }
        let wal = WriteAheadLog::open(wal_path)?;
        Ok(Self {
            store,
            wal,
            state_path_hint: state_path.to_path_buf(),
        })
    }

    /// Crash-sicheres Anwenden: WAL zuerst (durable), dann State.
    pub fn apply(&mut self, tx: &Tx) -> Result<u64, WalError> {
        let seq = self.wal.append(tx)?;
        self.store.apply(tx);
        Ok(seq)
    }

    /// Checkpoint — ATOMAR per Marker-Protokoll (G2-D):
    /// 1. Marker-Record in den WAL (durable, Teil der Hash-Kette)
    /// 2. Snapshot auf Disk schreiben (write-verified)
    ///
    /// Kein Truncate: das WAL bleibt append-only. Ein Absturz zwischen 1 und 2
    /// fueht beim Reopen zum Genesis-Replay (korrekt, nur laenger); danach
    /// greift der Snapshot-Pfad. Es gibt KEIN korruptes Zwischenreich mehr.
    pub fn checkpoint(
        &mut self,
        height: u64,
        tip_block_hash: &str,
        state_path: &Path,
    ) -> Result<(), WalError> {
        let root = self.store.state_root();
        self.wal.append_op(WalOp::Checkpoint {
            height,
            tip_block_hash: tip_block_hash.to_string(),
            state_root: root,
        })?;
        crate::persistence::save_snapshot_store(&self.store, height, tip_block_hash, state_path)
            .map_err(WalError::Io)?;
        Ok(())
    }

    /// Pfad der Snapshot-Datei (für Sync-Daemon, der auf derselben Basis persistiert).
    pub fn state_path_hint(&self) -> &Path {
        &self.state_path_hint
    }

    /// Rollback = SNAPSHOT-RETURN, nie semantische Umkehr (G2-D).
    /// Gibt die letzte verifizierte Zustandsgrenze zurueck — der Aufrufer
    /// entscheidet, ob er auf sie zurueckkehrt (Neustart von dort) statt
    /// angewendete Konsens-Effekte "rueckgaengig" zu machen.
    pub fn last_verified_boundary(&self) -> VerifiedBoundary {
        let meta = std::fs::File::open(&self.state_path_hint)
            .ok()
            .and_then(|_| crate::persistence::snapshot_meta(&self.state_path_hint).ok());
        match meta {
            Some((height, tip, root)) => VerifiedBoundary {
                height,
                tip_block_hash: tip,
                state_root: root,
            },
            None => VerifiedBoundary {
                height: 0,
                tip_block_hash: String::new(),
                state_root: crate::state::StateStore::new().state_root(),
            },
        }
    }
}
