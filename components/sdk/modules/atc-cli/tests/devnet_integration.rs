// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Cross-Repo-Integrationstest (SCR-0111, F-140): der SDK-Client (atc_cli)
//! spricht ueber echtes TCP mit dem ECHTEN atc-node-Crate aus dem Monorepo
//! (Pfadabhaengigkeit, keine Git-Revision) — kein Mock. Ehrlichkeit: Devnet-only, localhost, kein TLS.

use atc_cli::rpc_client::RpcClient;
use atc_node::bootstrap::{devnet_boot, Genesis};
use atc_node::rpc::{serve, DevnetRpc};
use std::net::TcpStream;
use std::time::Duration;

#[test]
fn sdk_spricht_mit_echtem_node() {
    let g = Genesis::devnet();
    let (peers, boot_hash) = devnet_boot(&g, &[(1, "addr1".to_string()), (2, "addr2".to_string())])
        .expect("devnet_boot fehlgeschlagen");
    let state = DevnetRpc::from_state(&g, &peers);

    // Freien Port suchen, dann Node-Dienst darauf starten
    let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("probe fehlgeschlagen");
    let port = probe.local_addr().expect("keine Adresse").port();
    drop(probe);
    std::thread::spawn(move || {
        let _ = serve(&format!("127.0.0.1:{}", port), state);
    });

    // Bis zu 5s auf Server-Bereitschaft warten
    let mut bereit = false;
    for _ in 0..50 {
        match TcpStream::connect(("127.0.0.1", port)) {
            Ok(s) => { drop(s); bereit = true; break; }
            Err(_) => std::thread::sleep(Duration::from_millis(100)),
        }
    }
    assert!(bereit, "Node-Dienst nicht bereit");

    let mut c = RpcClient::new(format!("127.0.0.1:{}", port));
    assert_eq!(c.chain_id().expect("chain_id"), g.chain_id, "SDK-Client muss die kanonische Chain-ID des echten Nodes lesen");
    assert_eq!(c.peers().expect("peers"), 2);
    // Kerninvariante: Boot-Hash aus der Node-Genesis == was der Client ueber RPC erhaelt
    let erwartet: String = boot_hash.iter().map(|b| format!("{:02x}", b)).collect();
    let vom_node = c.boot_hash().expect("boot_hash");
    assert_eq!(vom_node, erwartet, "Boot-Hash muss als voller 256-Bit-Hex ueber RPC kommen");
}
