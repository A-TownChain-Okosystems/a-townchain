// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Chain-Access-Client (SCR-0110, F-140): minimaler JSON-RPC-2.0-Client
//! gegen den Devnet-RPC des atc-node (Methoden chain_id/boot_hash/peers/ping).
//! Ehrlichkeit: std::net, KEIN TLS, keine Wiederverwendung von Verbindungen,
//! keine Request-Retry-Logik, Trefferquote fuer Produktion unzureichend.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

pub struct RpcClient {
    addr: String,
    next_id: u64,
}

impl RpcClient {
    pub fn new(addr: impl Into<String>) -> Self {
        RpcClient { addr: addr.into(), next_id: 1 }
    }

    fn call(&mut self, method: &str) -> Result<String, String> {
        let id = self.next_id;
        self.next_id += 1;
        let req = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"{}\",\"id\":{}}}\n", method, id);
        let mut stream = TcpStream::connect(&self.addr)
            .map_err(|e| format!("connect {}: {}", self.addr, e))?;
        stream.set_read_timeout(Some(Duration::from_secs(5)))
            .map_err(|e| format!("timeout: {}", e))?;
        stream.write_all(req.as_bytes()).map_err(|e| format!("send: {}", e))?;
        let mut line = String::new();
        BufReader::new(stream).read_line(&mut line).map_err(|e| format!("recv: {}", e))?;
        extract_result(&line).ok_or_else(|| format!("kein result in: {}", line.trim()))
    }

    pub fn chain_id(&mut self) -> Result<u64, String> {
        self.call("chain_id")?.parse::<u64>().map_err(|e| format!("kein u64: {}", e))
    }
    pub fn boot_hash(&mut self) -> Result<u64, String> {
        self.call("boot_hash")?.parse::<u64>().map_err(|e| format!("kein u64: {}", e))
    }
    pub fn peers(&mut self) -> Result<usize, String> {
        self.call("peers")?.parse::<usize>().map_err(|e| format!("kein usize: {}", e))
    }
    pub fn ping(&mut self) -> Result<String, String> {
        self.call("ping")
    }
}

fn extract_result(resp: &str) -> Option<String> {
    let i = resp.find("\"result\":\"")? + "\"result\":\"".len();
    let rest = &resp[i..];
    let j = rest.find('"')?;
    Some(rest[..j].to_string())
}

fn extract_id(resp: &str) -> String {
    let i = resp.find("\"id\":").map(|i| i + "\"id\":".len()).unwrap_or(0);
    let rest = &resp[i..];
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() { "0".to_string() } else { digits }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::io::Write as _;

    /// Mock-Node: antwortet wie der atc-node Devnet-RPC (SCR-0109-Protokoll).
    fn mock_node(listener: TcpListener) {
        for s in listener.incoming() {
            let mut s = match s { Ok(s) => s, Err(_) => break };
            let mut reader = BufReader::new(s.try_clone().unwrap());
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let body = if line.contains("\"chain_id\"") {
                "658467"
            } else if line.contains("\"peers\"") {
                "2"
            } else if line.contains("\"boot_hash\"") {
                "1234567890"
            } else {
                "pong"
            };
            let resp = format!("{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":\"{}\"}}\n", extract_id(&line), body);
            s.write_all(resp.as_bytes()).unwrap();
            break; // ein Request pro Verbindung (wie atc-node)
        }
    }

    #[test]
    fn chain_access_roundtrip() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind fehlgeschlagen");
        let addr = listener.local_addr().expect("keine Adresse");
        std::thread::spawn(move || mock_node(listener));
        let mut c = RpcClient::new(format!("127.0.0.1:{}", addr.port()));
        assert_eq!(c.chain_id().expect("chain_id fehlgeschlagen"), 658467);

        // zweite Verbindung: peers (Mock nimmt nur einen Request je Instanz)
        let listener2 = TcpListener::bind("127.0.0.1:0").expect("bind2 fehlgeschlagen");
        let addr2 = listener2.local_addr().expect("keine Adresse2");
        std::thread::spawn(move || mock_node(listener2));
        let mut c2 = RpcClient::new(format!("127.0.0.1:{}", addr2.port()));
        assert_eq!(c2.peers().expect("peers fehlgeschlagen"), 2);
    }

    #[test]
    fn verbindungsfehler_ehrlich() {
        // Port 1 auf localhost ist ungenutzt -> connect schlaegt fehl
        let mut c = RpcClient::new("127.0.0.1:1");
        assert!(c.chain_id().is_err());
    }

    #[test]
    fn extract_helpers() {
        assert_eq!(extract_result("{\"id\":7,\"result\":\"42\"}").unwrap(), "42");
        assert!(extract_result("{\"id\":7,\"error\":{\"code\":-32601}}").is_none());
        assert_eq!(extract_id("{\"id\":42,\"result\":\"x\"}"), "42");
    }
}
