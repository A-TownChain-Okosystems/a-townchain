//! S25-S26 End-to-End-Integrationstests — Issue #111 Akzeptanzkriterien.
//! Deterministisch: feste Seeds, fester Tx-Log, keine Wall-Clock (REQ-ENG-002).

use kai_os_integration::{boot, handoff_message, verify_handoff, BootConfig, HANDOFF_CHAIN_ID};
use kai_os_keyring::keyring::verify;
use kai_os_state::state::Tx;

fn config() -> BootConfig {
    BootConfig {
        node_seed: [11u8; 32],
        agent_seed: [77u8; 32],
        txs: vec![
            Tx::Set {
                key: "chain/name".into(),
                value: "a-townchain".into(),
            },
            Tx::Set {
                key: "chain/id".into(),
                value: "658467".into(),
            },
            Tx::Set {
                key: "agent/1/caps".into(),
                value: "propose_tx,query_state".into(),
            },
        ],
    }
}

// ── AK 3: End-to-End — alle Invarianten nach dem Boot ───────────────────

#[test]
fn end_to_end_boot_all_invariants() {
    let (sys, report) = boot(&config()).unwrap();

    // Alle 9 Schritte erfolgreich
    assert!(report.ok(), "boot report: {report:?}");
    assert_eq!(report.steps.len(), 9);
    let names: Vec<&str> = report.steps.iter().map(|(n, _)| *n).collect();
    assert_eq!(
        names,
        vec![
            "lifecycle",
            "keyring",
            "state",
            "snapshot",
            "sandbox",
            "ipc",
            "proposal",
            "signature",
            "health"
        ]
    );

    // Invarianten quer über alle Crates:
    // Lifecycle RUNNING
    assert_eq!(
        sys.lifecycle.current(),
        kai_os_node::lifecycle::State::Running
    );
    // State-Root == Snapshot-Root (die zwei Berechnungen müssen übereinstimmen)
    assert_eq!(sys.state_root, sys.snapshot.state_root);
    // Tx-Log vollständig angewendet
    assert_eq!(sys.state.get("chain/id"), Some(&"658467".to_string()));
    // Proposal am VM-Rand, nicht ausgeführt (AI may propose, ATVM executes)
    assert_eq!(
        sys.proposals.get(&sys.proposal_id).map(|p| p.state),
        Some(kai_os_ai::proposal::ProposalState::HandedToVM)
    );
    // Signatur über die Proposal-ID verifizierbar mit dem Node-Public-Key
    assert!(verify_handoff(
        &sys.handoff_public_key,
        &handoff_message(HANDOFF_CHAIN_ID, &sys.proposal_id),
        &sys.proposal_signature
    ));
    // Services laufen
    assert_eq!(
        sys.watchdog.state("node-daemon").unwrap(),
        kai_os_health::watchdog::ServiceState::Running
    );
    // Audit-Ketten beider AI-Komponenten intakt
    assert!(sys.ipc.audit().verify());
    assert!(sys.proposals.audit().verify());
    // Health-Snapshot vollständig (3 Services)
    assert_eq!(sys.gossip.snapshot("self").len(), 3);
}

// ── AK 4: Replay-Determinismus ─────────────────────────────────────────

#[test]
fn replay_determinism_identical_everything() {
    let (s1, r1) = boot(&config()).unwrap();
    let (s2, r2) = boot(&config()).unwrap();

    // Identische kritische Werte
    assert_eq!(s1.state_root, s2.state_root);
    assert_eq!(s1.proposal_id, s2.proposal_id);
    assert_eq!(
        s1.proposal_signature.to_vec(),
        s2.proposal_signature.to_vec()
    );
    assert_eq!(s1.snapshot.snapshot_id(), s2.snapshot.snapshot_id());
    assert_eq!(
        s1.keyring.public_key("node-key").unwrap(),
        s2.keyring.public_key("node-key").unwrap()
    );
    // Boot-Reports identisch (alle Schritte, alle Details)
    let d1: Vec<_> = r1.steps.iter().map(|(n, d)| (*n, d.clone())).collect();
    let d2: Vec<_> = r2.steps.iter().map(|(n, d)| (*n, d.clone())).collect();
    assert_eq!(d1, d2);
}

// ── AK 5: Crash-Recovery durch deterministisches Replay ────────────────

#[test]
fn crash_recovery_via_tx_log_replay() {
    // Boot 1: System entsteht, Tx-Log ist die Mutationsquelle
    let (s1, _) = boot(&config()).unwrap();
    let root1 = s1.state_root.clone();

    // Absturz: System verwerfen (Drop — nichts bleibt zurück)
    drop(s1);

    // Boot 2: identischer Tx-Log → identischer Zustand
    let (s2, _) = boot(&config()).unwrap();
    assert_eq!(s2.state_root, root1);
    assert_eq!(
        s2.state.get("agent/1/caps"),
        Some(&"propose_tx,query_state".to_string())
    );

    // Und ein ANDERER Tx-Log ergibt einen ANDEREN Root (keine Fake-Recovery)
    let mut cfg_other = config();
    cfg_other.txs.push(Tx::Set {
        key: "extra".into(),
        value: "1".into(),
    });
    let (s3, _) = boot(&cfg_other).unwrap();
    assert_ne!(s3.state_root, root1);
}

// ── AK 6: Seed-Isolation ───────────────────────────────────────────────

#[test]
fn seed_isolation_changes_identity() {
    let (s1, _) = boot(&config()).unwrap();
    let mut cfg2 = config();
    cfg2.node_seed = [99u8; 32];
    let (s2, _) = boot(&cfg2).unwrap();

    // Anderer Node-Seed → anderer Public-Key, andere Proposal-Signatur
    assert_ne!(
        s1.keyring.public_key("node-key").unwrap(),
        s2.keyring.public_key("node-key").unwrap()
    );
    assert_ne!(
        s1.proposal_signature.to_vec(),
        s2.proposal_signature.to_vec()
    );
    // State-Root unverändert (Seeds beeinflussen NICHT den State — saubere Trennung)
    assert_eq!(s1.state_root, s2.state_root);
}

#[test]
fn handoff_signatur_ist_domain_separiert_nicht_roh() {
    let (sys, _report) = boot(&config()).unwrap();
    // Die rohe Proposal-ID ist NICHT die signierte Nachricht:
    assert!(!verify_handoff(&sys.handoff_public_key, sys.proposal_id.as_bytes(), &sys.proposal_signature),
        "Handoff-Signatur darf nicht ueber die rohe ID gelten (Layer-Trennung)");
    // Die domain-separierte Nachricht schon:
    assert!(verify_handoff(&sys.handoff_public_key,
        &handoff_message(HANDOFF_CHAIN_ID, &sys.proposal_id), &sys.proposal_signature));
    // Ed25519-Transport-Schluessel darf die Handoff-Signatur NICHT verifizieren:
    let ed_public = sys.keyring.public_key("node-key").unwrap();
    assert!(!verify(&ed_public, &handoff_message(HANDOFF_CHAIN_ID, &sys.proposal_id), &sys.proposal_signature),
        "Ed25519 darf keine secp256k1-Handoff-Signatur bestaetigen");
    assert!(handoff_message(HANDOFF_CHAIN_ID, "x").starts_with(b"ATC-KAI-HANDOFF-V1"));
}
