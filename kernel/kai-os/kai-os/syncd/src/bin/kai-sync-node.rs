//! kai-sync-node — spawnbarer Sync-Knoten für Multi-Node-DR-Tests (#113).
//!
//! Zwei Rollen, beide mit eigenem PersistentState (WAL + Snapshot):
//!   source: wendet N deterministische Txs an, checkpointt, synced zum Peer
//!   sink:   lauscht, verifiziert Snapshots (Root-Gleichheit), persisted, ackt
//!
//! Protokoll-Ausgabe auf stdout (zeilengepuffert, sofort geflusht):
//!   LISTEN <addr>            (sink: gebundene Adresse)
//!   APPLIED <root>           (source: eigener Zustand nach Txs, vor Sync)
//!   SYNC-OK <root>           (beide: verifizierter, identischer Root)
//!   SYNC-FAILED <grund>      (source: Partition — Retry-Puffer erschöpft)

use kai_os_state::state::Tx;
use kai_os_state::wal::PersistentState;
use kai_os_syncd::{persist_verified, receive_and_verify, send_and_await_ack, SyncAck};
use std::io::Write;
use std::net::TcpListener;
use std::path::PathBuf;
use std::time::Duration;

fn say(line: &str) {
    let mut out = std::io::stdout().lock();
    let _ = writeln!(out, "{line}");
    let _ = out.flush(); // Piping: explizit flushen, kein Block-Puffer
}

/// Deterministische Txs — feste Keys/Values, kein RNG, keine Uhr (REQ-ENG-002).
fn deterministic_txs(n: u64) -> Vec<Tx> {
    (0..n)
        .map(|i| Tx::Set {
            key: format!("node/tx-{i:04}"),
            value: format!("value-{i:04}"),
        })
        .collect()
}

struct Args {
    role: String,
    dir: PathBuf,
    addr: Option<String>,
    peer: Option<String>,
    apply: u64,
    retries: u32,
    delay_ms: u64,
}

fn parse_args() -> Args {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut a = Args {
        role: String::new(),
        dir: PathBuf::new(),
        addr: None,
        peer: None,
        apply: 0,
        retries: 0,
        delay_ms: 0,
    };
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--role" => a.role = argv.get(i + 1).cloned().unwrap_or_default(),
            "--dir" => a.dir = PathBuf::from(argv.get(i + 1).cloned().unwrap_or_default()),
            "--addr" => a.addr = argv.get(i + 1).cloned(),
            "--peer" => a.peer = argv.get(i + 1).cloned(),
            "--apply" => a.apply = argv.get(i + 1).and_then(|v| v.parse().ok()).unwrap_or(0),
            "--retries" => a.retries = argv.get(i + 1).and_then(|v| v.parse().ok()).unwrap_or(0),
            "--delay-ms" => a.delay_ms = argv.get(i + 1).and_then(|v| v.parse().ok()).unwrap_or(0),
            _ => {}
        }
        i += 2;
    }
    a
}

fn main() {
    let args = parse_args();
    let state_path = args.dir.join("state.json");
    let wal_path = args.dir.join("wal.log");
    let mut ps = match PersistentState::open(&state_path, &wal_path) {
        Ok(ps) => ps,
        Err(e) => {
            say(&format!("BOOT-FAILED {e}"));
            std::process::exit(2);
        }
    };

    match args.role.as_str() {
        "source" => {
            // 1. Deterministische Txs anwenden (WAL zuerst — Crash-sicher)
            for tx in deterministic_txs(args.apply) {
                if let Err(e) = ps.apply(&tx) {
                    say(&format!("BOOT-FAILED {e}"));
                    std::process::exit(2);
                }
            }
            // 2. Checkpoint: Zustand ist ab hier verifizierbar gesichert
            if let Err(e) = ps.checkpoint(1, "kai-sync-node", &state_path) {
                say(&format!("BOOT-FAILED {e}"));
                std::process::exit(2);
            }
            say(&format!("APPLIED {}", ps.store.state_root()));
            // 3. Orchestrierungs-Fenster für Tests (Absturz-Punkt steuerbar)
            if args.delay_ms > 0 {
                std::thread::sleep(Duration::from_millis(args.delay_ms));
            }
            // 4. Sync mit Retry (Partition = feindliche Umgebung)
            let peer = match &args.peer {
                Some(p) => p.clone(),
                None => {
                    say("SYNC-FAILED kein --peer");
                    std::process::exit(1);
                }
            };
            for attempt in 0..=args.retries {
                match kai_os_network::tcp::TcpPeer::connect(&peer, kai_os_syncd::SYNC_TIMEOUT) {
                    Ok(mut sock) => {
                        match send_and_await_ack(&mut sock, &ps.store, 1, "kai-sync-node") {
                            Ok(()) => {
                                say(&format!("SYNC-OK {}", ps.store.state_root()));
                                std::process::exit(0);
                            }
                            Err(e) => {
                                say(&format!("SYNC-FAILED versuch-{attempt}: {e}"));
                            }
                        }
                    }
                    Err(e) => {
                        say(&format!("SYNC-FAILED connect-{attempt}: {e}"));
                    }
                }
                if attempt < args.retries {
                    std::thread::sleep(Duration::from_millis(200));
                }
            }
            std::process::exit(1);
        }
        "sink" => {
            let bind = args.addr.clone().unwrap_or_else(|| "127.0.0.1:0".into());
            let listener = match TcpListener::bind(&bind) {
                Ok(l) => l,
                Err(e) => {
                    say(&format!("BOOT-FAILED {e}"));
                    std::process::exit(2);
                }
            };
            say(&format!(
                "LISTEN {}",
                listener
                    .local_addr()
                    .map(|a| a.to_string())
                    .unwrap_or_default()
            ));
            // Genau EINEN erfolgreichen Sync abfertigen (deterministische Orchestrierung)
            for stream in listener.incoming() {
                let Ok(stream) = stream else { continue };
                let mut peer = match kai_os_network::tcp::TcpPeer::from_stream(
                    stream,
                    kai_os_syncd::SYNC_TIMEOUT,
                ) {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                let snap = match receive_and_verify(&mut peer) {
                    Ok(s) => s,
                    Err(e) => {
                        say(&format!("VERIFY-FAILED {e}"));
                        continue; // abgewiesen — kein Teilzustand, weiter lauschen
                    }
                };
                if let Err(e) = persist_verified(&mut ps, &snap) {
                    say(&format!("VERIFY-FAILED {e}"));
                    continue;
                }
                let ack = SyncAck {
                    state_root: ps.store.state_root(),
                };
                if peer
                    .send(&serde_json::to_vec(&ack).unwrap_or_default())
                    .is_err()
                {
                    continue;
                }
                say(&format!("SYNC-OK {}", ps.store.state_root()));
                std::process::exit(0);
            }
            say("SYNC-FAILED listener-end");
            std::process::exit(1);
        }
        _ => {
            say("USAGE --role source|sink --dir <d> [--peer a] [--addr a] [--apply n] [--retries r] [--delay-ms ms]");
            std::process::exit(2);
        }
    }
}
