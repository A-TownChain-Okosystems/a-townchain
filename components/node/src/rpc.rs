// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Devnet-RPC Stufe 2/3 with ATC-STD-600 Chain Identity representation.

use crate::bootstrap::Genesis;
use crate::did::DidRegistry;
use crate::peers::PeerTable;
use std::io::{BufRead, BufReader, Read, Write};
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};

const MAX_RPC_REQUEST_BYTES: usize = 8 * 1024;

/// Laufender Konsens-Zustand (SCR-0117), vom Node-Treiber aktualisiert.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConsensusStatus {
    /// Zuletzt finalisierte Blockhoehe.
    pub height: u64,
    /// Anzahl erfolgreicher Runden.
    pub finalized_rounds: u64,
    pub validators: usize,
    /// Benoetigte Voting-Power fuer 2/3-Quorum.
    pub quorum_power: u64,
    /// Stimmen der letzten finalisierten Runde.
    pub last_voters: usize,
    /// Hash des letzten finalisierten Blocks (hex).
    pub last_hash: String,
}

pub struct DevnetRpc {
    pub chain_id: String,
    /// 256-Bit-Bindungs-Hash der Genesis, hex (kein 64-Bit-Traversal).
    pub boot_hash: String,
    pub peer_count: usize,
    /// DID-Registry (SEC-P0-004) — lesbar ueber `DID <did>`.
    pub registry: DidRegistry,
    /// Konsens-Status, geteilt mit dem Runden-Treiber.
    pub consensus: Arc<Mutex<ConsensusStatus>>,
}

impl DevnetRpc {
    pub fn from_state(genesis: &Genesis, peers: &PeerTable) -> Self {
        DevnetRpc {
            chain_id: genesis.chain_id.clone(),
            boot_hash: crate::chain::hash_hex(&genesis.boot_hash256()),
            peer_count: peers.len(),
            registry: DidRegistry::new(),
            consensus: Arc::new(Mutex::new(ConsensusStatus::default())),
        }
    }

    /// Konsens-Status anbinden (geteilter Zustand mit dem Runden-Treiber).
    pub fn with_consensus(mut self, status: Arc<Mutex<ConsensusStatus>>) -> Self {
        self.consensus = status;
        self
    }

    fn consensus_snapshot(&self) -> ConsensusStatus {
        self.consensus
            .lock()
            .map(|s| s.clone())
            .unwrap_or_default()
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
            "BOOT_HASH" => self.boot_hash.clone(),
            "PEERS" => self.peer_count.to_string(),
            "PING" => "PONG".to_string(),
            "DID_COUNT" => self.registry.len().to_string(),
            "DID_EPOCH" => self.registry.epoch().to_string(),
            "HEIGHT" => self.consensus_snapshot().height.to_string(),
            "BEST_HASH" => {
                let h = self.consensus_snapshot().last_hash;
                if h.is_empty() { "-".to_string() } else { h }
            }
            "CONSENSUS" => {
                let c = self.consensus_snapshot();
                format!(
                    "height={} finalized_rounds={} validators={} quorum_power={} last_voters={} last_hash={}",
                    c.height,
                    c.finalized_rounds,
                    c.validators,
                    c.quorum_power,
                    c.last_voters,
                    if c.last_hash.is_empty() { "-" } else { &c.last_hash }
                )
            }
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
            "boot_hash" => self.boot_hash.clone(),
            "peers" => self.peer_count.to_string(),
            "ping" => "pong".to_string(),
            "did_count" => self.registry.len().to_string(),
            "height" => self.consensus_snapshot().height.to_string(),
            "consensus" => {
                let c = self.consensus_snapshot();
                format!(
                    "height={} rounds={} validators={} quorum={}",
                    c.height, c.finalized_rounds, c.validators, c.quorum_power
                )
            }
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
    fn boot_hash_ist_256_bit_hex_und_nicht_trunkiert() {
        let genesis = Genesis::devnet();
        let rpc = DevnetRpc::from_state(&genesis, &PeerTable::new());
        let wert = rpc.answer("BOOT_HASH");
        assert_eq!(wert.len(), 64, "Boot-Hash muss 32 Byte breit sein: {}", wert);
        assert!(wert.bytes().all(|b| b.is_ascii_hexdigit()), "kein Hex: {}", wert);
        assert_eq!(wert, crate::chain::hash_hex(&genesis.boot_hash256()));
        // Ein 64-Bit-Traversal waere eine reine Zahl und damit kein Bindungs-Hash.
        assert!(wert.parse::<u64>().is_err(), "Boot-Hash darf nicht als Zahl lesbar sein");
        // JSON-RPC liefert denselben Digest.
        let json = rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"boot_hash\",\"id\":3}");
        assert!(json.contains(&wert), "JSON-RPC weicht ab: {}", json);
    }

    #[test]
    fn konsens_status_ueber_rpc() {
        use std::sync::{Arc, Mutex};
        let genesis = Genesis::devnet();
        let peers = PeerTable::new();
        let status = Arc::new(Mutex::new(ConsensusStatus {
            height: 7,
            finalized_rounds: 7,
            validators: 3,
            quorum_power: 2,
            last_voters: 3,
            last_hash: "ab12".to_string(),
        }));
        let rpc = DevnetRpc::from_state(&genesis, &peers).with_consensus(Arc::clone(&status));
        assert_eq!(rpc.answer("HEIGHT"), "7");
        assert_eq!(rpc.answer("BEST_HASH"), "ab12");
        let antwort = rpc.answer("CONSENSUS");
        assert!(antwort.contains("height=7"), "unerwartet: {antwort}");
        assert!(antwort.contains("quorum_power=2"), "unerwartet: {antwort}");
        assert!(antwort.contains("last_hash=ab12"), "unerwartet: {antwort}");
        assert!(rpc.answer_json("{\"jsonrpc\":\"2.0\",\"method\":\"height\",\"id\":1}").contains("\"7\""));

        // Treiber aktualisiert den Stand, RPC liest ihn sofort.
        status.lock().unwrap().height = 9;
        assert_eq!(rpc.answer("HEIGHT"), "9");
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
