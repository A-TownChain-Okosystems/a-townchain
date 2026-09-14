// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Devnet-RPC Stufe 2/3 with ATC-STD-600 Chain Identity representation.

use crate::bootstrap::Genesis;
use crate::peers::PeerTable;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

pub struct DevnetRpc {
    pub chain_id: String,
    pub boot_hash: u64,
    pub peer_count: usize,
}

impl DevnetRpc {
    pub fn from_state(genesis: &Genesis, peers: &PeerTable) -> Self {
        DevnetRpc { chain_id: genesis.chain_id.clone(), boot_hash: genesis.boot_hash(), peer_count: peers.len() }
    }

    pub fn answer(&self, req: &str) -> String {
        match req.trim() {
            "CHAIN_ID" => self.chain_id.clone(),
            "BOOT_HASH" => self.boot_hash.to_string(),
            "PEERS" => self.peer_count.to_string(),
            "PING" => "PONG".to_string(),
            other => format!("ERR: unbekannter Befehl {}", other),
        }
    }
}

pub fn serve(addr: &str, state: DevnetRpc) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        if let Ok(stream) = stream { handle(stream, &state)?; }
    }
    Ok(())
}

fn handle(stream: TcpStream, state: &DevnetRpc) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let resp = if line.trim().starts_with('{') { state.answer_json(&line) } else { state.answer(&line) };
    let mut w = stream;
    w.write_all(resp.as_bytes())?;
    w.write_all(b"\n")?;
    Ok(())
}

impl DevnetRpc {
    pub fn answer_json(&self, req: &str) -> String {
        let id = extract_between(req, "\"id\":", '}').and_then(|v| v.trim().parse::<u64>().ok()).unwrap_or(0);
        let method = extract_between(req, "\"method\":\"", '"').unwrap_or("");
        let result = match method {
            "chain_id" => self.chain_id.clone(),
            "boot_hash" => self.boot_hash.to_string(),
            "peers" => self.peer_count.to_string(),
            "ping" => "pong".to_string(),
            other => return format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"error\":{{\"code\":-32601,\"message\":\"method not found: {}\"}}}}", id, other),
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
    fn jsonrpc_unbekannte_methode() {
        let rpc = test_state();
        let r = rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"gib_nichts\",\"id\":3}");
        assert!(r.contains("-32601"));
    }
}
