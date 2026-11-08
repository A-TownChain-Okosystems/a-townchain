// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Devnet-RPC Stufe 2/3 with ATC-STD-600 Chain Identity representation.

use crate::bootstrap::Genesis;
use crate::did::DidRegistry;
use crate::peers::PeerTable;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

const MAX_RPC_REQUEST_BYTES: usize = 8 * 1024;

pub struct DevnetRpc {
    pub chain_id: String,
    pub boot_hash: u64,
    pub peer_count: usize,
    /// DID-Registry (SEC-P0-004) — lesbar ueber `DID <did>`.
    pub registry: DidRegistry,
}

impl DevnetRpc {
    pub fn from_state(genesis: &Genesis, peers: &PeerTable) -> Self {
        DevnetRpc {
            chain_id: genesis.chain_id.clone(),
            boot_hash: genesis.boot_hash(),
            peer_count: peers.len(),
            registry: DidRegistry::new(),
        }
    }

    /// Registry anbinden (Persistenz + Aufloesung ueber RPC).
    pub fn with_registry(mut self, registry: DidRegistry) -> Self {
        self.registry = registry;
        self
    }

    pub fn answer(&self, req: &str) -> String {
        let cmd = req.trim();
        if let Some(did) = cmd.strip_prefix("DID ") {
            let did = did.trim();
            return match self.registry.resolve(did) {
                Ok(doc) => format!(
                    "version={} keys={} epoch={} current={}",
                    doc.version,
                    doc.keys.len(),
                    self.registry.epoch(),
                    doc.current_key()
                        .map(|k| crate::did::hex_lower(&k))
                        .unwrap_or_else(|| "-".to_string()),
                ),
                Err(_) => "ERR: Unbekannte DID".to_string(),
            };
        }
        match cmd {
            "CHAIN_ID" => self.chain_id.clone(),
            "BOOT_HASH" => self.boot_hash.to_string(),
            "PEERS" => self.peer_count.to_string(),
            "PING" => "PONG".to_string(),
            "DID_COUNT" => self.registry.len().to_string(),
            "DID_EPOCH" => self.registry.epoch().to_string(),
            other => format!("ERR: unbekannter Befehl {}", other),
        }
    }
}

pub fn serve(addr: &str, state: DevnetRpc) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle(stream, &state) {
                    eprintln!("RPC-Verbindung beendet: {}", e);
                }
            }
            Err(e) => {
                eprintln!("RPC-Accept fehlgeschlagen: {}", e);
            }
        }
    }
    Ok(())
}

fn handle(stream: TcpStream, state: &DevnetRpc) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    let bytes_read = reader.take((MAX_RPC_REQUEST_BYTES + 1) as u64).read_line(&mut line)?;
    if bytes_read == 0 {
        return Ok(());
    }
    if bytes_read > MAX_RPC_REQUEST_BYTES || !line.ends_with('\n') {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "RPC request too large or missing newline",
        ));
    }
    let resp = if line.trim().starts_with('{') { state.answer_json(&line) } else { state.answer(&line) };
    let mut w = stream;
    w.write_all(resp.as_bytes())?;
    w.write_all(b"
")?;
    Ok(())
}

impl DevnetRpc {
    pub fn answer_json(&self, req: &str) -> String {
        // ID steht nicht immer vor einem Komma; auch "...,\"id\":7}" (letztes Feld) ist valider JSON-RPC.
        let id = match extract_between(req, "\"id\":", ',').or_else(|| extract_between(req, "\"id\":", '}')) {
            Some(raw) => match raw.trim().trim_end_matches('}').parse::<u64>() {
                Ok(id) => id,
                Err(_) => return "{\"jsonrpc\":\"2.0\",\"id\":null,\"error\":{\"code\":-32600,\"message\":\"invalid request\"}}".to_string(),
            },
            None => return "{\"jsonrpc\":\"2.0\",\"id\":null,\"error\":{\"code\":-32600,\"message\":\"invalid request\"}}".to_string(),
        };
        let method = match extract_between(req, "\"method\":\"", '"') {
            Some(method) if !method.is_empty() => method,
            _ => return format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":-32600,\"message\":\"invalid request\"}}}}", id),
        };
        let result = match method {
            "chain_id" => self.chain_id.clone(),
            "boot_hash" => self.boot_hash.to_string(),
            "peers" => self.peer_count.to_string(),
            "ping" => "pong".to_string(),
            "did_count" => self.registry.len().to_string(),
            "did_epoch" => self.registry.epoch().to_string(),
            _ => return format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":-32601,\"message\":\"method not found\"}}}}", id),
        };
        format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":\"{}\"}}", id, result)

    }
}

fn extract_between<'a>(s: &'a str, start: &str, end: char) -> Option<&'a str> {
    let i = s.find(start)? + start.len();
    let rest = &s[i..];
    let j = rest.find(end)?;
    Some(&rest[..j])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap::{devnet_boot, Genesis};

    fn test_state() -> DevnetRpc {
        let g = Genesis::devnet();
        let (peers, _) = devnet_boot(&g, &[(1, "addr1".to_string()), (2, "addr2".to_string())]).unwrap();
        DevnetRpc::from_state(&g, &peers)
    }

    #[test]
    fn rpc_antworten_deterministisch() {
        let rpc = test_state();
        assert_eq!(rpc.answer("CHAIN_ID"), "atc");
        assert_eq!(rpc.answer("PEERS"), "2");
        assert_eq!(rpc.answer("PING"), "PONG");
        assert!(!rpc.answer("BOOT_HASH").is_empty());
    }

    #[test]
    fn jsonrpc_antworten() {
        let rpc = test_state();
        let r = rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"chain_id\",\"id\":7}");
        assert!(r.contains("\"id\":7"));
        assert!(r.contains("\"result\":\"atc\""));
    }

    #[test]
    fn jsonrpc_fehlende_id_wird_abgelehnt() {
        let rpc = test_state();
        let r = rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"ping\"}");
        assert!(r.contains("-32600"));
        assert!(r.contains("\"id\":null"));
    }

    #[test]
    fn jsonrpc_ungueltige_id_wird_abgelehnt() {
        let rpc = test_state();
        let r = rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"ping\",\"id\":\"bad\"}");
        assert!(r.contains("-32600"));
        assert!(r.contains("\"id\":null"));
    }

    #[test]
    fn rpc_loest_dids_auf() {
        use atc_wallet::keys::WalletKey;
        let key = WalletKey::from_seed([5u8; 32]).unwrap();
        let mut reg = DidRegistry::new();
        let did = reg.register(key.public_key()).unwrap();
        let rpc = test_state().with_registry(reg);

        let antwort = rpc.answer(&format!("DID {}", did));
        assert!(antwort.contains("version=1"), "Antwort: {}", antwort);
        assert!(antwort.contains("keys=1"));
        assert!(antwort.contains(&crate::did::hex_lower(&key.public_key())));
        assert_eq!(rpc.answer("DID_COUNT"), "1");
        assert_eq!(rpc.answer("DID ATC:gibtsnicht"), "ERR: Unbekannte DID");
        assert!(rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"did_count\",\"id\":1}").contains("\"result\":\"1\""));
    }

    #[test]
    fn jsonrpc_unbekannte_methode() {
        let rpc = test_state();
        let r = rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"gib_nichts\",\"id\":3}");
        assert!(r.contains("-32601"));
        assert!(r.contains("\"message\":\"method not found\""));
        assert!(!r.contains("gib_nichts"));
    }
}
