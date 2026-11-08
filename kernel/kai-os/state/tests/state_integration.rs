//! S10-S12 Integrationstests — Issue #105 Akzeptanzkriterien.
//! Deterministisch: kein RNG, keine Wall-Clock (REQ-ENG-002).

use kai_os_state::merkle;
use kai_os_state::snapshot::Snapshot;
use kai_os_state::state::{StateStore, Tx};
use kai_os_state::sync::{Block, SyncEngine, SyncError, GENESIS_PREV};
use std::collections::BTreeMap;

fn entries(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

/// Baut einen gültigen Block zum aktuellen Engine-State.
fn valid_block(engine: &SyncEngine, height: u64, txs: Vec<Tx>) -> Block {
    let mut scratch = engine.state.clone();
    scratch.apply_all(&txs);
    Block {
        height,
        prev_hash: engine.tip_block_hash.clone(),
        txs,
        state_root: scratch.state_root(),
    }
}

// ── AK 1: Merkle-Determinismus ──────────────────────────────────────────

#[test]
fn merkle_root_insertion_order_independent() {
    // Gleiche Einträge, unterschiedliche Einfüge-Order -> identischer Root
    let mut a = BTreeMap::new();
    a.insert("x".into(), "1".into());
    a.insert("y".into(), "2".into());
    a.insert("z".into(), "3".into());
    let mut b = BTreeMap::new();
    b.insert("z".into(), "3".into());
    b.insert("x".into(), "1".into());
    b.insert("y".into(), "2".into());
    assert_eq!(merkle::merkle_root(&a), merkle::merkle_root(&b));

    // Unterschiedlicher Wert -> anderer Root
    let mut c = a.clone();
    c.insert("y".into(), "99".into());
    assert_ne!(merkle::merkle_root(&a), merkle::merkle_root(&c));

    // Odd-Anzahl (3) funktioniert, Even (4) auch
    let mut d = a.clone();
    d.insert("w".into(), "4".into());
    assert_eq!(merkle::merkle_root(&d).len(), 64);
}

// ── AK 2: Inclusion-Proofs ─────────────────────────────────────────────

#[test]
fn inclusion_proof_valid_and_tamper_fails() {
    let e = entries(&[
        ("acct-alice", "100"),
        ("acct-bob", "200"),
        ("acct-carol", "300"),
    ]);
    let root = merkle::merkle_root(&e);

    for key in ["acct-alice", "acct-bob", "acct-carol"] {
        let proof = merkle::prove(&e, key).unwrap();
        let leaf = merkle::leaf_hash(key, e.get(key).unwrap());
        assert!(
            merkle::verify_proof(&leaf, &proof, &root),
            "proof für {key} muss gelten"
        );
    }

    // Manipulierter Leaf -> Proof scheitert
    let proof = merkle::prove(&e, "acct-alice").unwrap();
    let tampered = merkle::leaf_hash("acct-alice", "999999");
    assert!(!merkle::verify_proof(&tampered, &proof, &root));

    // Fremder Root -> scheitert
    let leaf = merkle::leaf_hash("acct-alice", "100");
    let other_root = merkle::merkle_root(&entries(&[("a", "1")]));
    assert!(!merkle::verify_proof(&leaf, &proof, &other_root));

    // Unbekannter Key -> kein Proof
    assert!(merkle::prove(&e, "unbekannt").is_none());
}

// ── AK 3: Block NUR mit passendem State-Root ───────────────────────────

#[test]
fn block_with_wrong_root_rejected_state_untouched() {
    let mut engine = SyncEngine::new(StateStore::new());
    let txs = vec![Tx::Set {
        key: "alice".into(),
        value: "100".into(),
    }];

    let mut bad = valid_block(&engine, 1, txs.clone());
    bad.state_root = "f".repeat(64); // gelogen
    assert!(matches!(
        engine.apply_block(&bad),
        Err(SyncError::RootMismatch { .. })
    ));

    // Atomicität: State unverändert
    assert_eq!(engine.tip_height, 0);
    assert!(engine.state.is_empty());
    assert_eq!(engine.tip_block_hash, GENESIS_PREV.to_string());

    // Korrekter Block geht durch
    let good = valid_block(&engine, 1, txs);
    engine.apply_block(&good).unwrap();
    assert_eq!(engine.state.get("alice"), Some(&"100".to_string()));
    assert_eq!(engine.tip_height, 1);
}

// ── AK 4: Strikte Höhen ─────────────────────────────────────────────────

#[test]
fn height_gap_duplicate_and_fork_rejected() {
    let mut engine = SyncEngine::new(StateStore::new());
    let b1 = valid_block(
        &engine,
        1,
        vec![Tx::Set {
            key: "a".into(),
            value: "1".into(),
        }],
    );
    engine.apply_block(&b1).unwrap();

    // Höhen-Sprung (1 -> 3)
    let b3 = valid_block(
        &engine,
        3,
        vec![Tx::Set {
            key: "c".into(),
            value: "3".into(),
        }],
    );
    assert!(matches!(
        engine.apply_block(&b3),
        Err(SyncError::HeightGap {
            expected: 2,
            got: 3
        })
    ));

    // Duplikat (gleicher Block nochmal)
    assert!(matches!(
        engine.apply_block(&b1),
        Err(SyncError::Duplicate { height: 1 })
    ));

    // Fork: gleiche Höhe, anderer Hash
    let mut fork = valid_block(
        &engine,
        1,
        vec![Tx::Set {
            key: "a".into(),
            value: "1".into(),
        }],
    );
    fork.state_root = "0".repeat(64);
    // Fork-Detection feuert vor Root-Check (Hash differiert durch state_root)
    assert!(matches!(
        engine.apply_block(&fork),
        Err(SyncError::Fork { height: 1, .. })
    ));
}

// ── AK 5: Prev-Hash-Kette ────────────────────────────────────────────────

#[test]
fn prev_hash_chain_continuity_enforced() {
    let mut engine = SyncEngine::new(StateStore::new());
    let b1 = valid_block(
        &engine,
        1,
        vec![Tx::Set {
            key: "a".into(),
            value: "1".into(),
        }],
    );
    engine.apply_block(&b1).unwrap();

    // Block 2 mit falschem prev_hash
    let mut wrong_prev = valid_block(
        &engine,
        2,
        vec![Tx::Set {
            key: "b".into(),
            value: "2".into(),
        }],
    );
    wrong_prev.prev_hash = "f".repeat(64);
    assert!(matches!(
        engine.apply_block(&wrong_prev),
        Err(SyncError::PrevMismatch { .. })
    ));
    assert_eq!(engine.tip_height, 1);

    // Korrekte Kette: 2, 3, 4
    let b2 = valid_block(
        &engine,
        2,
        vec![Tx::Set {
            key: "b".into(),
            value: "2".into(),
        }],
    );
    engine.apply_block(&b2).unwrap();
    let b3 = valid_block(&engine, 3, vec![Tx::Remove { key: "b".into() }]);
    engine.apply_block(&b3).unwrap();
    let b4 = valid_block(
        &engine,
        4,
        vec![Tx::Set {
            key: "c".into(),
            value: "4".into(),
        }],
    );
    let h4 = engine.apply_block(&b4).unwrap();

    assert_eq!(engine.tip_height, 4);
    assert_eq!(engine.block_hash_at(4), Some(&h4));
    assert_eq!(engine.state.get("b"), None); // Remove angewendet
    assert_eq!(engine.state_root(), engine.state.state_root());
}

// ── AK 6+7: Snapshot-Verifikation (nie vertrauen) ──────────────────────

#[test]
fn snapshot_verify_against_trusted_root() {
    let mut engine = SyncEngine::new(StateStore::new());
    engine
        .apply_block(&valid_block(
            &engine,
            1,
            vec![Tx::Set {
                key: "alice".into(),
                value: "100".into(),
            }],
        ))
        .unwrap();
    engine
        .apply_block(&valid_block(
            &engine,
            2,
            vec![Tx::Set {
                key: "bob".into(),
                value: "200".into(),
            }],
        ))
        .unwrap();

    // Snapshot erstellen
    let snap = Snapshot::from_state(engine.tip_height, &engine.tip_block_hash, &engine.state);
    let trusted_root = engine.state_root().clone();
    assert_eq!(snap.snapshot_id().len(), 64);

    // Ehrlicher Snapshot verifiziert
    snap.verify_against(2, &trusted_root).unwrap();

    // Manipulierter Eintrag -> Root stimmt nicht mehr -> Abweisung
    let mut tampered = snap.clone();
    tampered.entries[0].1 = "999999".to_string();
    assert!(matches!(
        tampered.verify_against(2, &trusted_root),
        Err(SyncError::SnapshotRootMismatch { .. })
    ));

    // Falsche Höhe -> Abweisung
    assert!(matches!(
        snap.verify_against(99, &trusted_root),
        Err(SyncError::SnapshotHeightMismatch { expected: 99, .. })
    ));

    // Vertrauens-Root stimmt nicht -> Abweisung
    let lie_root = "0".repeat(64);
    assert!(matches!(
        snap.verify_against(2, &lie_root),
        Err(SyncError::SnapshotRootMismatch { .. })
    ));
}

#[test]
fn snapshot_rebuild_state_matches_original() {
    let mut engine = SyncEngine::new(StateStore::new());
    engine
        .apply_block(&valid_block(
            &engine,
            1,
            vec![
                Tx::Set {
                    key: "x".into(),
                    value: "1".into(),
                },
                Tx::Set {
                    key: "y".into(),
                    value: "2".into(),
                },
            ],
        ))
        .unwrap();

    let snap = Snapshot::from_state(engine.tip_height, &engine.tip_block_hash, &engine.state);
    snap.verify_against(engine.tip_height, &engine.state_root())
        .unwrap();

    // Verifizierter Snapshot rekonstruiert einen identischen State
    let rebuilt = snap.to_state();
    assert_eq!(rebuilt.state_root(), engine.state_root());
    assert_eq!(rebuilt.get("x"), Some(&"1".to_string()));
    assert_eq!(rebuilt.get("y"), Some(&"2".to_string()));
}

// ── Chain-Replay: dieselben Blöcke -> derselbe Root (Determinismus) ────

#[test]
fn deterministic_replay_same_root() {
    let build = |mut e: SyncEngine| -> String {
        for i in 1..=5 {
            let txs = vec![
                Tx::Set {
                    key: format!("k{i}"),
                    value: format!("v{i}"),
                },
                if i % 2 == 0 {
                    Tx::Remove {
                        key: format!("k{}", i - 1),
                    }
                } else {
                    Tx::Set {
                        key: format!("alt{i}"),
                        value: "x".into(),
                    }
                },
            ];
            let b = valid_block(&e, i, txs);
            e.apply_block(&b).unwrap();
        }
        e.state_root()
    };
    let r1 = build(SyncEngine::new(StateStore::new()));
    let r2 = build(SyncEngine::new(StateStore::new()));
    assert_eq!(r1, r2); // identischer Replay -> identischer Root
    assert_eq!(r1.len(), 64);
}

// ── Leerer State / Genesis ─────────────────────────────────────────────

#[test]
fn empty_state_root_is_sha256_empty() {
    let state = StateStore::new();
    assert_eq!(state.state_root(), kai_os_state::merkle::EMPTY_ROOT);
}
