// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Devnet-RPC Stufe 2 (SCR-0108, F-140): minimaler Chain-Access-Dienst.
//! Zeilenprotokoll (ein Befehl pro Zeile) ueber TCP — CHAIN_ID, BOOT_HASH,
//! PEERS, PING. Ehrlichkeit: KEIN JSON-RPC, keine Authentisierung, keine
//! TLS-Verschluesselung (Devnet-only!), ein Request pro Verbindung.
//! JSON-RPC-Subset und Node-zu-Node-Gossip folgen in Stufe 3.

use crate::bootstrap::Genesis;
use crate::peers::PeerTable;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

/// Schnappschuss des Devnet-Zustands fuer den Zugriffsdienst (read-only).
pub struct DevnetRpc {
    pub chain_id: u64,
    pub boot_hash: u64,
    pub peer_count: usize,
}

impl DevnetRpc {
    pub fn from_state(genesis: &Genesis, peers: &PeerTable) -> Self {
        DevnetRpc {
            chain_id: genesis.chain_id,
            boot_hash: genesis.boot_hash(),
            peer_count: peers.len(),
        }
    }

    /// Beantwortet einen Zeilen-Befehl deterministisch.
    pub fn answer(&self, req: &str) -> String {
        match req.trim() {
            "CHAIN_ID" => self.chain_id.to_string(),
            "BOOT_HASH" => self.boot_hash.to_string(),
            "PEERS" => self.peer_count.to_string(),
            "PING" => "PONG".to_string(),
            other => format!("ERR: unbekannter Befehl {}", other),
        }
    }
}

/// Dauer-Dienst: nimmt Verbindungen an und beantwortet je eine Zeile.
pub fn serve(addr: &str, state: DevnetRpc) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle(stream, &state)?;
        }
    }
    Ok(())
}

fn handle(stream: TcpStream, state: &DevnetRpc) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut w = stream;
    w.write_all(state.answer(&line).as_bytes())?;
    w.write_all(b"\n")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::{devnet_boot, Genesis};

    fn test_state() -> DevnetRpc {
        let g = Genesis::devnet();
        let (peers, _) = devnet_boot(&g, &[(1, "addr1".to_string()), (2, "addr2".to_string())])
            .expect("devnet_boot fehlgeschlagen");
        DevnetRpc::from_state(&g, &peers)
    }

    #[test]
    fn rpc_antworten_deterministisch() {
        let rpc = test_state();
        assert_eq!(rpc.answer("CHAIN_ID"), "658467");
        assert_eq!(rpc.answer("PEERS"), "2");
        assert_eq!(rpc.answer("PING"), "PONG");
        assert!(!rpc.answer("BOOT_HASH").is_empty());
        assert!(rpc.answer("FOO").starts_with("ERR"));
        // Determinismus: gleicher Zustand -> gleiche Antworten
        assert_eq!(rpc.answer("CHAIN_ID"), test_state().answer("CHAIN_ID"));
    }

    #[test]
    fn tcp_socket_roundtrip() {
        let rpc = test_state();
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind fehlgeschlagen");
        let addr = listener.local_addr().expect("keine lokale Adresse");
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    let st = DevnetRpc {
                        chain_id: rpc.chain_id,
                        boot_hash: rpc.boot_hash,
                        peer_count: rpc.peer_count,
                    };
                    if handle(stream, &st).is_err() {
                        break;
                    }
                }
            }
        });
        let mut c = TcpStream::connect(addr).expect("connect fehlgeschlagen");
        c.write_all(b"CHAIN_ID\n").expect("send fehlgeschlagen");
        let mut resp = String::new();
        BufReader::new(c).read_line(&mut resp).expect("keine Antwort");
        assert_eq!(resp.trim(), "658467");
    }
}
