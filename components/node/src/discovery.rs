// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Peer Discovery & Seed-Bootstrap fuer atc-node (SCR-0100).
//! Ermoeglicht automatische Peer-Erkennung und Peer-Exchange ueber TCP.

use std::collections::HashSet;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoveryMessage {
    Ping { node_id: u64 },
    Pong { node_id: u64 },
    GetPeers,
    PeersResponse { peers: Vec<(u64, String)> },
}

impl DiscoveryMessage {
    pub fn serialize(&self) -> String {
        match self {
            DiscoveryMessage::Ping { node_id } => format!("PING {}", node_id),
            DiscoveryMessage::Pong { node_id } => format!("PONG {}", node_id),
            DiscoveryMessage::GetPeers => "GET_PEERS".to_string(),
            DiscoveryMessage::PeersResponse { peers } => {
                let items: Vec<String> = peers.iter().map(|(id, addr)| format!("{}:{}", id, addr)).collect();
                format!("PEERS {}", items.join(","))
            }
        }
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let trimmed = s.trim();
        if let Some(rest) = trimmed.strip_prefix("PING ") {
            let id: u64 = rest.parse().map_err(|e| format!("Ungueltige Ping ID: {}", e))?;
            Ok(DiscoveryMessage::Ping { node_id: id })
        } else if let Some(rest) = trimmed.strip_prefix("PONG ") {
            let id: u64 = rest.parse().map_err(|e| format!("Ungueltige Pong ID: {}", e))?;
            Ok(DiscoveryMessage::Pong { node_id: id })
        } else if trimmed == "GET_PEERS" {
            Ok(DiscoveryMessage::GetPeers)
        } else if let Some(rest) = trimmed.strip_prefix("PEERS ") {
            if rest.is_empty() {
                return Ok(DiscoveryMessage::PeersResponse { peers: Vec::new() });
            }
            let mut peers = Vec::new();
            for item in rest.split(',') {
                if item.is_empty() { continue; }
                let parts: Vec<&str> = item.splitn(2, ':').collect();
                if parts.len() != 2 {
                    return Err(format!("Ungueltiges Peer-Format: {}", item));
                }
                let id: u64 = parts[0].parse().map_err(|e| format!("Ungueltige Peer ID: {}", e))?;
                peers.push((id, parts[1].to_string()));
            }
            Ok(DiscoveryMessage::PeersResponse { peers })
        } else {
            Err(format!("Unbekannte Discovery-Nachricht: {}", trimmed))
        }
    }
}

pub struct PeerDiscovery {
    pub node_id: u64,
    pub self_addr: String,
    known_peers: Vec<(u64, String)>,
    seeds: HashSet<String>,
}

impl PeerDiscovery {
    pub fn new(node_id: u64, self_addr: impl Into<String>) -> Self {
        PeerDiscovery {
            node_id,
            self_addr: self_addr.into(),
            known_peers: Vec::new(),
            seeds: HashSet::new(),
        }
    }

    pub fn add_seed(&mut self, addr: impl Into<String>) {
        self.seeds.insert(addr.into());
    }

    pub fn register_peer(&mut self, id: u64, addr: impl Into<String>) -> bool {
        let addr_str = addr.into();
        if id == self.node_id {
            return false; // Selbst-Ausschluss
        }
        if self.known_peers.iter().any(|(p_id, _)| *p_id == id) {
            return false; // Duplikat
        }
        self.known_peers.push((id, addr_str));
        true
    }

    pub fn known_peers(&self) -> &[(u64, String)] {
        &self.known_peers
    }

    pub fn handle_message(&mut self, msg: DiscoveryMessage) -> Option<DiscoveryMessage> {
        match msg {
            DiscoveryMessage::Ping { node_id } => {
                Some(DiscoveryMessage::Pong { node_id: self.node_id })
            }
            DiscoveryMessage::GetPeers => {
                let mut all = vec![(self.node_id, self.self_addr.clone())];
                all.extend(self.known_peers.clone());
                Some(DiscoveryMessage::PeersResponse { peers: all })
            }
            DiscoveryMessage::PeersResponse { peers } => {
                for (id, addr) in peers {
                    self.register_peer(id, addr);
                }
                None
            }
            DiscoveryMessage::Pong { .. } => None,
        }
    }

    pub fn exchange_peers(&mut self, target_addr: &str) -> Result<usize, String> {
        let mut stream = TcpStream::connect(target_addr)
            .map_err(|e| format!("Verbindung zu Seed {} fehlgeschlagen: {}", target_addr, e))?;
        stream.set_read_timeout(Some(Duration::from_secs(3)))
            .map_err(|e| format!("Timeout-Fehler: {}", e))?;

        let req = DiscoveryMessage::GetPeers.serialize();
        writeln!(stream, "{}", req).map_err(|e| format!("Sende-Fehler: {}", e))?;

        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        reader.read_line(&mut line).map_err(|e| format!("Empfangs-Fehler: {}", e))?;

        let msg = DiscoveryMessage::parse(&line)?;
        let before = self.known_peers.len();
        self.handle_message(msg);
        Ok(self.known_peers.len() - before)
    }
}

pub fn serve_discovery(addr: &str, mut discovery: PeerDiscovery) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    for stream in listener.incoming() {
        if let Ok(mut s) = stream {
            let mut reader = BufReader::new(s.try_clone()?);
            let mut line = String::new();
            if reader.read_line(&mut line).is_ok() {
                if let Ok(msg) = DiscoveryMessage::parse(&line) {
                    if let Some(resp) = discovery.handle_message(msg) {
                        let _ = writeln!(s, "{}", resp.serialize());
                    }
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_serialization_roundtrip() {
        let ping = DiscoveryMessage::Ping { node_id: 42 };
        let s = ping.serialize();
        assert_eq!(s, "PING 42");
        assert_eq!(DiscoveryMessage::parse(&s).unwrap(), ping);

        let peers = DiscoveryMessage::PeersResponse {
            peers: vec![(1, "127.0.0.1:39471".into()), (2, "127.0.0.1:39472".into())],
        };
        let s_peers = peers.serialize();
        assert_eq!(DiscoveryMessage::parse(&s_peers).unwrap(), peers);
    }

    #[test]
    fn register_peer_duplikat_und_self_exclusion() {
        let mut disc = PeerDiscovery::new(10, "127.0.0.1:8000");
        assert!(!disc.register_peer(10, "127.0.0.1:8000"), "Selbst-Ausschluss erfordert false");
        assert!(disc.register_peer(11, "127.0.0.1:8001"));
        assert!(!disc.register_peer(11, "127.0.0.1:8001"), "Duplikat muss abgewiesen werden");
        assert_eq!(disc.known_peers().len(), 1);
    }

    #[test]
    fn peer_exchange_roundtrip_over_tcp() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let seed_addr = format!("127.0.0.1:{}", port);

        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let msg = DiscoveryMessage::parse(&line).unwrap();
            assert_eq!(msg, DiscoveryMessage::GetPeers);

            let resp = DiscoveryMessage::PeersResponse {
                peers: vec![(100, "127.0.0.1:9000".into()), (101, "127.0.0.1:9001".into())],
            };
            writeln!(stream, "{}", resp.serialize()).unwrap();
        });

        let mut disc = PeerDiscovery::new(1, "127.0.0.1:7000");
        let added = disc.exchange_peers(&seed_addr).unwrap();
        assert_eq!(added, 2);
        assert_eq!(disc.known_peers().len(), 2);
        handle.join().unwrap();
    }
}
