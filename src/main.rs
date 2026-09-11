// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC-Node-Prozess (SCR-0112, F-139 Stufe 2): startet den Devnet-Bootstrap
//! und dient Chain-Access auf einer TCP-Adresse (Zeilen- und JSON-RPC).
//! Ehrlichkeit: Devnet-only, kein TLS, keine Authentisierung, keine
//! Node-zu-Node-Gossip, keine Blockproduktion; Logs nach stderr.

use atc_node::bootstrap::{devnet_boot, Genesis};
use atc_node::rpc::{serve, DevnetRpc};

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
        boot_hash,
        peers.len(),
        addr
    );
    serve(&addr, DevnetRpc::from_state(&genesis, &peers))
}
