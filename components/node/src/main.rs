// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC-Node-Prozess (SCR-0112, F-139 Stufe 2): startet den Devnet-Bootstrap
//! und dient Chain-Access auf einer TCP-Adresse (Zeilen- und JSON-RPC).
//! Ehrlichkeit: Devnet-only, kein TLS, keine Authentisierung. Der Node treibt
//! seit SCR-0117 einen Konsens-Loop: jede Runde waehlt deterministisch einen
//! Proposer, sammelt signierte Votes bis 2/3-Quorum und finalisiert den Block.
//! Die Validator-Schluessel liegen im Devnet alle in diesem Prozess; der
//! Vote-Austausch zwischen Maschinen ist noch nicht im Gossip-Wire.

use atc_node::bootstrap::{devnet_boot, Genesis};
use atc_node::chain::hash_hex;
use atc_node::consensus::ConsensusEngine;
use atc_node::mempool::TxLedger;
use atc_node::rpc::{serve, ConsensusStatus, DevnetRpc};
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:39471".to_string());
    let genesis = Genesis::devnet();
    let (peers, boot_hash) = match devnet_boot(
        &genesis,
        &[(1, "atc-node-1".to_string()), (2, "atc-node-2".to_string())],
    ) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("devnet_boot fehlgeschlagen: {}", e);
            std::process::exit(1);
        }
    };
    eprintln!(
        "ATC-Node Devnet gestartet | Chain-ID {} | Boot-Hash {} | Peers {} | RPC auf {} (Zeile + JSON-RPC)",
        genesis.chain_id,
        hash_hex(&boot_hash),
        peers.len(),
        addr
    );
    let kette = std::sync::Arc::new(std::sync::Mutex::new(atc_node::chain::Chain::from_genesis(&genesis)));
    {
        let k = kette.lock().expect("Chain-Lock vergiftet");
        eprintln!(
            "Devnet-Kette: Hoehe {} | Best-Hash {} | Blockmodell SCR-0117/0118 (Konsens-Loop aktiv, Devnet)",
            k.height(),
            atc_node::chain::hash_hex(&k.best_hash())
        );
    }
    let gossip_addr = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "127.0.0.1:39472".to_string());
    eprintln!("Gossip-Dienst (Pull-Sync, SCR-0118): {}", gossip_addr);
    let gossip_kette = std::sync::Arc::clone(&kette);
    std::thread::spawn(move || {
        if let Err(e) = atc_node::gossip::serve_gossip(&gossip_addr, gossip_kette) {
            eprintln!("Gossip-Dienst beendet: {}", e);
        }
    });
    // Konsens-Loop (SCR-0117): finalisiert Runden im Takt.
    let status = Arc::new(Mutex::new(ConsensusStatus::default()));
    let konsens_kette = Arc::clone(&kette);
    let konsens_status = Arc::clone(&status);
    std::thread::spawn(move || {
        let mut engine = ConsensusEngine::devnet(3);
        let mut ledger = TxLedger::new();
        {
            let mut st = konsens_status.lock().expect("Status-Lock vergiftet");
            st.validators = engine.validator_set().active_validators().len();
            st.quorum_power = engine.required_power();
            st.height = konsens_kette
                .lock()
                .map(|k| k.height())
                .unwrap_or_default();
        }
        loop {
            std::thread::sleep(Duration::from_secs(2));
            let ergebnis = {
                let mut kette_locked = match konsens_kette.lock() {
                    Ok(k) => k,
                    Err(_) => {
                        eprintln!("Konsens: Chain-Lock vergiftet, Loop endet");
                        return;
                    }
                };
                engine.run_round(&mut kette_locked, &[], &mut ledger)
            };
            match ergebnis {
                Ok(out) if out.finalized => {
                    eprintln!(
                        "Konsens: Hoehe {} finalisiert | Proposer {} | Stimmen {:?} | Hash {}",
                        out.height,
                        out.proposer_id,
                        out.voter_ids,
                        hash_hex(&out.block_hash)
                    );
                    if let Ok(mut st) = konsens_status.lock() {
                        st.height = out.height;
                        st.finalized_rounds += 1;
                        st.last_voters = out.voter_ids.len();
                        st.last_hash = hash_hex(&out.block_hash);
                    }
                }
                Ok(out) => eprintln!(
                    "Konsens: Hoehe {} nicht finalisiert ({})",
                    out.height, out.reason
                ),
                Err(e) => eprintln!("Konsens-Fehler: {}", e),
            }
        }
    });

    serve(
        &addr,
        DevnetRpc::from_state(&genesis, &peers).with_consensus(status),
    )
}
