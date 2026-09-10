// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Node-Konfiguration mit Chain-ID-Bindung (L3, Chain-ID 658467).

pub const CHAIN_ID: u64 = 658467;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeConfig {
    pub chain_id: u64,
    pub listen_addr: String,
    pub max_peers: usize,
}

impl NodeConfig {
    pub fn new(listen_addr: impl Into<String>, max_peers: usize) -> Self {
        NodeConfig { chain_id: CHAIN_ID, listen_addr: listen_addr.into(), max_peers }
    }

    pub fn valid(&self) -> bool {
        self.chain_id == CHAIN_ID && !self.listen_addr.is_empty() && self.max_peers > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_sind_valide() {
        let c = NodeConfig::new("/ip4/0.0.0.0/tcp/30303", 50);
        assert!(c.valid());
        assert_eq!(c.chain_id, 658467);
    }

    #[test]
    fn ungueltige_werte() {
        let mut c = NodeConfig::new("", 10);
        assert!(!c.valid());
        c.listen_addr = "/ip4/0.0.0.0/tcp/1".to_string();
        c.max_peers = 0;
        assert!(!c.valid());
    }
}
