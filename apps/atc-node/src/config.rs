// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Node-Konfiguration gemaess ATC-STD-600.

pub const CHAIN_ID: &str = "atc";
pub const NETWORK_ID: &str = "devnet";
pub const PROTOCOL_VERSION: &str = "1.0.0";
pub const VM_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeConfig {
    pub chain_id: String,
    pub network_id: String,
    pub listen_addr: String,
    pub max_peers: usize,
}

impl NodeConfig {
    pub fn new(listen_addr: impl Into<String>, max_peers: usize) -> Self {
        NodeConfig { chain_id: CHAIN_ID.into(), network_id: NETWORK_ID.into(), listen_addr: listen_addr.into(), max_peers }
    }

    pub fn valid(&self) -> bool {
        self.chain_id == CHAIN_ID
            && self.network_id == NETWORK_ID
            && !self.listen_addr.is_empty()
            && self.max_peers > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_sind_valide() {
        let c = NodeConfig::new("/ip4/0.0.0.0/tcp/30303", 50);
        assert!(c.valid());
        assert_eq!(c.chain_id, "atc");
        assert_eq!(c.network_id, "devnet");
    }

    #[test]
    fn fremdes_netzwerk_wird_abgelehnt() {
        let mut c = NodeConfig::new("/ip4/0.0.0.0/tcp/30303", 50);
        c.network_id = "mainnet".into();
        assert!(!c.valid());
    }
}
