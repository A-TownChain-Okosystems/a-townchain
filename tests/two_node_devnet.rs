// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Zwei-Node-Devnet-Smoke (SCR-0113, F-139): zwei lebende Node-Dienste
//! (je eigener Prozess-Thread, eigene Genesis-Instanz, eigene Peer-Tabelle)
//! teilen dieselbe Genesis-Definition. Ein Client verifiziert ueber ECHTE
//! TCP-Sockets, dass beide Nodes Chain-ID und Boot-Hash identisch liefern —
//! die Devnet-Kerninvariante. Ehrlichkeit: Simulation von zwei Prozessen
//! via Threads, kein Docker, kein Gossip zwischen den Nodes.

use atc_node::bootstrap::{devnet_boot, Genesis};
use atc_node::rpc::{serve, DevnetRpc};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

fn start_node() -> (u16, u64) {
    let g = Genesis::devnet();
    let (peers, boot_hash) =
        devnet_boot(&g, &[(1, "atc-node-1".to_string()), (2, "atc-node-2".to_string())])
            .expect("devnet_boot fehlgeschlagen");
    let state = DevnetRpc::from_state(&g, &peers);
    let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("probe fehlgeschlagen");
    let port = probe.local_addr().expect("keine Adresse").port();
    drop(probe);
    let addr = format!("127.0.0.1:{}", port);
    std::thread::spawn(move || {
        let _ = serve(&addr, state);
    });
    (port, boot_hash)
}

fn query(port: u16, cmd: &str) -> String {
    for _ in 0..50 {
        if let Ok(mut s) = TcpStream::connect(("127.0.0.1", port)) {
            s.write_all(format!("{}\n", cmd).as_bytes()).expect("send fehlgeschlagen");
            let mut line = String::new();
            BufReader::new(s).read_line(&mut line).expect("keine Antwort");
            return line.trim().to_string();
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    panic!("Node auf Port {} nicht erreichbar", port);
}

#[test]
fn zwei_node_devnet_gleiche_genesis() {
    let (p1, h1) = start_node();
    let (p2, h2) = start_node();
    assert_eq!(h1, h2, "beide Nodes muessen denselben Genesis-Boot-Hash haben");

    // Beide Nodes leben und antworten ueber echte Sockets
    assert_eq!(query(p1, "CHAIN_ID"), "658467");
    assert_eq!(query(p2, "CHAIN_ID"), "658467");
    assert_eq!(query(p1, "BOOT_HASH"), h1.to_string());
    assert_eq!(query(p2, "BOOT_HASH"), h2.to_string());

    // Beide sprechen auch JSON-RPC (SCR-0109)
    assert!(query(p1, "{\"method\":\"chain_id\",\"id\":1}").contains("658467"));
    assert!(query(p2, "{\"method\":\"ping\",\"id\":2}").contains("pong"));
}

#[test]
fn zwei_ketten_gleiche_genesis_gleiche_bloecke() {
    let g = Genesis::devnet();
    let mut k1 = atc_node::chain::Chain::from_genesis(&g);
    let mut k2 = atc_node::chain::Chain::from_genesis(&g);
    for p in ["tx-a", "tx-b", "tx-c"] {
        k1.produce(p).expect("produce k1");
        k2.produce(p).expect("produce k2");
    }
    assert_eq!(k1.best_hash(), k2.best_hash(), "beide Node-Instanzen muessen identische Ketten bauen");
    assert_eq!(k1.height(), 3);
    assert!(k1.verify().is_ok());
    assert!(k2.verify().is_ok());
}

#[test]
fn zwei_nodes_syncen_kette_ueber_gossip() {
    let g = Genesis::devnet();
    let mut a = atc_node::chain::Chain::from_genesis(&g);
    for p in ["tx-1", "tx-2", "tx-3", "tx-4"] {
        a.produce(p).expect("produce a");
    }
    let ziel = a.best_hash();
    let shared = std::sync::Arc::new(std::sync::Mutex::new(a));
    let handler = std::sync::Arc::clone(&shared);
    let probe = std::net::TcpListener::bind("127.0.0.1:0").expect("probe fehlgeschlagen");
    let port = probe.local_addr().expect("keine Adresse").port();
    drop(probe);
    std::thread::spawn(move || {
        let _ = atc_node::gossip::serve_gossip(&format!("127.0.0.1:{}", port), handler);
    });
    for _ in 0..50 {
        if TcpStream::connect(("127.0.0.1", port)).is_ok() {
            break;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    let mut b = atc_node::chain::Chain::from_genesis(&g);
    let rep = atc_node::gossip::sync_pull(&mut b, &format!("127.0.0.1:{}", port)).expect("sync fehlgeschlagen");
    assert!(rep.adopted, "Adoption erwartet: {}", rep.grund);
    assert_eq!(b.height(), 4);
    assert_eq!(b.best_hash(), ziel, "beide Nodes muessen dieselbe Kettenspitze haben");
    assert!(b.verify().is_ok());
}
