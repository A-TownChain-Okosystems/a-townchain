//! G2-B (Issue #112): WAL + Snapshot-Persistenz + Crash-Recovery.
//! Echter Dateisystem-Test — der "Absturz" ist ein echter Drop,
//! die Recovery liest von Disk. Deterministisch (REQ-ENG-002).

use kai_os_state::persistence::{load_snapshot_store, save_snapshot_store, snapshot_meta};
use kai_os_state::state::{StateStore, Tx};
use kai_os_state::wal::{PersistentState, WalError, WriteAheadLog};
use std::path::PathBuf;

fn test_dir(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("kai-g2b-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn sample_txs() -> Vec<Tx> {
    vec![
        Tx::Set {
            key: "chain/name".into(),
            value: "a-townchain".into(),
        },
        Tx::Set {
            key: "chain/id".into(),
            value: "658467".into(),
        },
        Tx::Remove {
            key: "chain/name".into(),
        },
        Tx::Set {
            key: "chain/name".into(),
            value: "a-townchain-v2".into(),
        },
    ]
}

// ── AK: Crash-Recovery — Drop -> Reopen -> identischer Root ────────────

#[test]
fn crash_recovery_via_wal_replay() {
    let dir = test_dir("recovery");
    let state_path = dir.join("state.json");
    let wal_path = dir.join("wal.log");

    // Run 1: Tx-Log anwenden (WAL zuerst, durable), dann ABSTURZ
    let root_before = {
        let mut ps = PersistentState::open(&state_path, &wal_path).unwrap();
        for tx in sample_txs() {
            ps.apply(&tx).unwrap();
        }
        ps.store.state_root()
    };
    // Absturz: drop — nichts bleibt im Speicher

    // Run 2: Reopen von Disk — Recovery durch Replay
    let ps = PersistentState::open(&state_path, &wal_path).unwrap();
    assert_eq!(
        ps.store.state_root(),
        root_before,
        "Replay muss identischen Root ergeben"
    );
    assert_eq!(ps.store.get("chain/id"), Some(&"658467".to_string()));
    assert_eq!(
        ps.store.get("chain/name"),
        Some(&"a-townchain-v2".to_string())
    );
    assert_eq!(ps.store.get("chain/removed"), None);
}

// ── AK: WAL allein — Append + Replay sind zueinander konsistent ─────────

#[test]
fn wal_append_then_replay_yields_all_txs() {
    let dir = test_dir("append-replay");
    let wal_path = dir.join("wal.log");
    let mut wal = WriteAheadLog::open(&wal_path).unwrap();
    for tx in sample_txs() {
        wal.append(&tx).unwrap();
    }
    let (ops, seq, _tip) = WriteAheadLog::replay(&wal_path).unwrap();
    let txs: Vec<Tx> = ops
        .into_iter()
        .filter_map(|op| match op {
            kai_os_state::wal::WalOp::Apply(tx) => Some(tx),
            _ => None,
        })
        .collect();
    assert_eq!(txs, sample_txs());
    assert_eq!(seq, 4);
}

// ── AK: Torn Tail — unvollständiger LETZTER Record wird verworfen ──────

#[test]
fn wal_torn_tail_discarded() {
    let dir = test_dir("torn-tail");
    let wal_path = dir.join("wal.log");
    let mut wal = WriteAheadLog::open(&wal_path).unwrap();
    for tx in sample_txs() {
        wal.append(&tx).unwrap();
    }
    // Absturz mitten im Schreiben simulieren: letzte Zeile abschneiden
    let content = std::fs::read_to_string(&wal_path).unwrap();
    let mut lines: Vec<&str> = content.lines().collect();
    lines.pop();
    let torn: String = lines.join("\n");
    std::fs::write(&wal_path, torn).unwrap();
    // Ein OEFFNEN nach dem Absturz: 3 gueltige Records, torn tail verworfen
    let reopened = WriteAheadLog::open(&wal_path).unwrap();
    assert_eq!(reopened.seq(), 3);
}

// ── AK: Corruption MITTEN im Log — fail-closed, kein Blind-Replay ─────

#[test]
fn wal_midfile_corruption_fail_closed() {
    let dir = test_dir("midfile");
    let wal_path = dir.join("wal.log");
    let mut wal = WriteAheadLog::open(&wal_path).unwrap();
    for tx in sample_txs() {
        wal.append(&tx).unwrap();
    }
    // Mittlere Zeile manipulieren (Seq-Nummer umbiegen)
    let content = std::fs::read_to_string(&wal_path).unwrap();
    let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
    lines[1] = lines[1].replace("\"seq\":2", "\"seq\":99");
    std::fs::write(&wal_path, lines.join("\n")).unwrap();

    let result = WriteAheadLog::replay(&wal_path);
    assert!(
        matches!(result, Err(WalError::ChainBroken { .. })),
        "Corruption mid-file muss fail-closed sein"
    );
}

// ── AK: Snapshot-Persistenz mit Tamper-Erkennung ────────────────────────

#[test]
fn snapshot_save_load_and_tamper_detection() {
    let dir = test_dir("snapshot");
    let path = dir.join("state.json");
    let mut store = StateStore::new();
    for tx in sample_txs() {
        store.apply(&tx);
    }
    save_snapshot_store(&store, 42, "tip-abc", &path).unwrap();

    // Roundtrip: identischer Root
    let loaded = load_snapshot_store(&path).unwrap();
    assert_eq!(loaded.state_root(), store.state_root());
    let (height, tip, root) = snapshot_meta(&path).unwrap();
    assert_eq!((height, tip.as_str()), (42, "tip-abc"));
    assert_eq!(root, store.state_root());

    // Manipulation: einen Eintrag in der Datei veraendern
    let raw = std::fs::read_to_string(&path).unwrap();
    let tampered = raw.replace("658467", "999999");
    std::fs::write(&path, tampered).unwrap();
    let result = load_snapshot_store(&path);
    assert!(
        result.is_err(),
        "manipuliertes Snapshot muss erkannt werden (load-verify)"
    );
}

// ── AK: Checkpoint konsolidiert — danach Crash mit leerem WAL ──────────

#[test]
fn checkpoint_then_crash_recovers_from_snapshot() {
    let dir = test_dir("checkpoint");
    let state_path = dir.join("state.json");
    let wal_path = dir.join("wal.log");

    let root_before = {
        let mut ps = PersistentState::open(&state_path, &wal_path).unwrap();
        for tx in sample_txs() {
            ps.apply(&tx).unwrap();
        }
        ps.checkpoint(7, "tip-007", &state_path).unwrap();
        ps.store.state_root()
    };
    // WAL enthaelt jetzt den Marker (append-only, kein Truncate); Absturz
    // Reopen: Snapshot wird geladen, Ops nach dem Marker replays
    let mut ps = PersistentState::open(&state_path, &wal_path).unwrap();
    assert_eq!(ps.store.state_root(), root_before);
    // Und: weitere Txs nach dem Checkpoint landen im frischen WAL
    ps.apply(&Tx::Set {
        key: "post/ckpt".into(),
        value: "1".into(),
    })
    .unwrap();
    drop(ps);
    let ps2 = PersistentState::open(&state_path, &wal_path).unwrap();
    assert_eq!(ps2.store.get("post/ckpt"), Some(&"1".to_string()));
}

// ── G2-D: Marker-Protokoll — kein Absturzfenster mehr ──────────────────

#[test]
fn checkpoint_crash_before_snapshot_sync_still_recovers() {
    let dir = test_dir("marker-crash");
    let state_path = dir.join("state.json");
    let wal_path = dir.join("wal.log");

    let root_before = {
        let mut ps = PersistentState::open(&state_path, &wal_path).unwrap();
        for tx in sample_txs() {
            ps.apply(&tx).unwrap();
        }
        // Checkpoint anstossen — dann Absturz NACH dem Marker, VOR dem Snapshot-Sync simulieren:
        ps.checkpoint(5, "tip-005", &state_path).unwrap();
        // Snapshot wieder entfernen = "Sync nie abgeschlossen"
        std::fs::remove_file(&state_path).unwrap();
        ps.store.state_root()
    };
    // Reopen: Marker vorhanden, Snapshot fehlt -> GENESIS-REPLAY (korrekter Pfad 3)
    let ps = PersistentState::open(&state_path, &wal_path).unwrap();
    assert_eq!(
        ps.store.state_root(),
        root_before,
        "Marker ohne Snapshot muss ueber Genesis-Replay korrekt sein"
    );
}

#[test]
fn rollback_boundary_is_last_verified_snapshot() {
    let dir = test_dir("boundary");
    let state_path = dir.join("state.json");
    let wal_path = dir.join("wal.log");

    let mut ps = PersistentState::open(&state_path, &wal_path).unwrap();
    for tx in sample_txs() {
        ps.apply(&tx).unwrap();
    }
    ps.checkpoint(9, "tip-009", &state_path).unwrap();
    let boundary = ps.last_verified_boundary();
    assert_eq!(boundary.height, 9);
    assert_eq!(boundary.tip_block_hash, "tip-009");
    assert_eq!(boundary.state_root, ps.store.state_root());

    // Weitere Txs aendern den Zustand — aber NICHT die verifizierte Grenze:
    // Rollback-Ziel bleibt der letzte Checkpoint (kein semantisches Undo)
    ps.apply(&Tx::Set {
        key: "post/boundary".into(),
        value: "x".into(),
    })
    .unwrap();
    let boundary2 = ps.last_verified_boundary();
    assert_eq!(boundary2.height, 9, "Grenze bleibt beim letzten Checkpoint");
    assert_eq!(boundary2.state_root, boundary.state_root);

    // Und der Genesis-Fall: ohne jeden Checkpoint ist die Grenze der leere Zustand
    let fresh = PersistentState::open(&dir.join("none.json"), &dir.join("none.log")).unwrap();
    let b0 = fresh.last_verified_boundary();
    assert_eq!(b0.height, 0);
    assert_eq!(b0.state_root, StateStore::new().state_root());
}

#[test]
fn double_checkpoint_recovery_uses_last_marker() {
    let dir = test_dir("double-marker");
    let state_path = dir.join("state.json");
    let wal_path = dir.join("wal.log");

    let root_final = {
        let mut ps = PersistentState::open(&state_path, &wal_path).unwrap();
        for tx in &sample_txs()[..2] {
            ps.apply(tx).unwrap();
        }
        ps.checkpoint(1, "tip-1", &state_path).unwrap();
        for tx in &sample_txs()[2..] {
            ps.apply(tx).unwrap();
        }
        ps.checkpoint(2, "tip-2", &state_path).unwrap();
        // Txs NACH dem letzten Checkpoint (Suffix, der replayed werden muss)
        ps.apply(&Tx::Set {
            key: "final/tx".into(),
            value: "v".into(),
        })
        .unwrap();
        ps.store.state_root()
    };
    // Reopen muss Snapshot#2 + Suffix-Nachspielen kombinieren — KEIN Doppel-Apply
    let ps = PersistentState::open(&state_path, &wal_path).unwrap();
    assert_eq!(ps.store.state_root(), root_final);
    assert_eq!(ps.store.get("final/tx"), Some(&"v".to_string()));
    // Und: alle Vorgaenger-Keys sind da (kein Tx ging verloren)
    assert_eq!(ps.store.get("chain/id"), Some(&"658467".to_string()));
}
