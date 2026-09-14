// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Devnet bootstrap and ATC-STD-600 Chain Identity binding.

use crate::identity::{compute_genesis_id, verify_genesis_id, ChainIdentity, DEVNET_NETWORK_ID, PROTOCOL_VERSION, VM_VERSION, CHAIN_ID};
use crate::peers::PeerTable;
use atc_algorithm::hash::atc_hash;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Genesis {
    pub chain_id: String,
    pub chain_name: String,
    pub network_id: String,
    pub genesis_id: String,
    pub genesis_height: u64,
    pub initial_peers: Vec<String>,
    pub state_root: String,
    pub protocol_version: String,
    pub vm_version: String,
}

impl Genesis {
    pub fn devnet() -> Self {
        let genesis_id = compute_genesis_id(CHAIN_ID, DEVNET_NETWORK_ID, PROTOCOL_VERSION, VM_VERSION);
        Genesis {
            chain_id: CHAIN_ID.to_string(),
            chain_name: "A-TownChain Devnet".to_string(),
            network_id: DEVNET_NETWORK_ID.to_string(),
            genesis_id,
            genesis_height: 0,
            initial_peers: vec!["atc-node-1".to_string(), "atc-node-2".to_string()],
            state_root: "0".repeat(64),
            protocol_version: PROTOCOL_VERSION.to_string(),
            vm_version: VM_VERSION.to_string(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path).map_err(|e| format!("lesen {}: {}", path, e))?;
        serde_json::from_str(&content).map_err(|e| format!("parsen {}: {}", path, e))
    }

    pub fn identity(&self) -> ChainIdentity {
        ChainIdentity { chain_id: self.chain_id.clone(), network_id: self.network_id.clone(), genesis_id: self.genesis_id.clone() }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.genesis_height != 0 { return Err(format!("Genesis-Height muss 0 sein, ist {}", self.genesis_height)); }
        if self.chain_name.is_empty() { return Err("Chain-Name fehlt".into()); }
        if self.initial_peers.len() < 2 { return Err("Devnet braucht mindestens 2 Initial-Peers".into()); }
        verify_genesis_id(&self.identity(), &self.protocol_version, &self.vm_version)
            .map_err(|e| format!("Chain Identity ungueltig: {:?}", e))?;
        if self.state_root.len() != 64 || !self.state_root.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("state_root muss ein 32-Byte-Hex-Digest sein".into());
        }
        Ok(())
    }

    pub fn boot_hash(&self) -> u64 {
        let digest = atc_hash(&self.canonical_boot_encoding());
        u64::from_le_bytes([digest[0], digest[1], digest[2], digest[3], digest[4], digest[5], digest[6], digest[7]])
    }

    fn canonical_boot_encoding(&self) -> Vec<u8> {
        let peers = serde_json::to_string(&self.initial_peers).expect("Vec<String> serialization cannot fail");
        let fields = [
            ("chain_id", self.chain_id.as_str()),
            ("chain_name", self.chain_name.as_str()),
            ("network_id", self.network_id.as_str()),
            ("genesis_id", self.genesis_id.as_str()),
            ("genesis_height", self.genesis_height.to_string().as_str()),
            ("initial_peers", peers.as_str()),
            ("state_root", self.state_root.as_str()),
            ("protocol_version", self.protocol_version.as_str()),
            ("vm_version", self.vm_version.as_str()),
        ];
        let mut out = Vec::new();
        for (key, value) in fields {
            out.extend_from_slice(&(key.len() as u32).to_be_bytes());
            out.extend_from_slice(key.as_bytes());
            out.extend_from_slice(&(value.len() as u64).to_be_bytes());
            out.extend_from_slice(value.as_bytes());
        }
        out
    }
}

pub fn devnet_boot(genesis: &Genesis, peers: &[(u64, String)]) -> Result<(PeerTable, u64), String> {
    genesis.validate()?;
    let mut table = PeerTable::new();
    for (id, addr) in peers {
        if !table.add(*id, addr.clone()) { return Err(format!("Peer {} schon angemeldet", id)); }
        if !table.verify(*id) { return Err(format!("Peer {} nicht verifizierbar", id)); }
    }
    Ok((table, genesis.boot_hash()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_file_bindung_ist_erzwungen() {
        let g = Genesis::from_file("config/devnet/genesis.json").expect("genesis.json lesbar");
        assert_eq!(g, Genesis::devnet(), "genesis.json und Code-Genesis duerfen nicht driften");
        g.validate().expect("File-Genesis valide");
    }

    #[test]
    fn devnet_genesis_valide() {
        let g = Genesis::devnet();
        assert!(g.validate().is_ok());
        assert_eq!(g.chain_id, "atc");
        assert_eq!(g.network_id, "devnet");
        assert_eq!(g.genesis_height, 0);
        assert_eq!(g.genesis_id.len(), 64);
    }

    #[test]
    fn identity_mismatch_is_rejected() {
        let mut g = Genesis::devnet();
        g.network_id = "mainnet".into();
        assert!(g.validate().is_err());
    }

    #[test]
    fn genesis_id_mismatch_is_rejected() {
        let mut g = Genesis::devnet();
        g.genesis_id = "0".repeat(64);
        assert!(g.validate().is_err());
    }

    #[test]
    fn zwei_nodes_gleiche_genesis_gleicher_hash() {
        let g = Genesis::devnet();
        let peers1 = vec![(1, "addr1".to_string()), (2, "addr2".to_string())];
        let peers2 = vec![(1, "addr1".to_string()), (2, "addr2".to_string())];
        let (t1, h1) = devnet_boot(&g, &peers1).expect("Boot 1 fehlgeschlagen");
        let (t2, h2) = devnet_boot(&g, &peers2).expect("Boot 2 fehlgeschlagen");
        assert_eq!(h1, h2);
        assert_eq!(t1.len(), 2);
        assert_eq!(t2.len(), 2);
    }

    #[test]
    fn duplikat_peer_abgelehnt() {
        let g = Genesis::devnet();
        let peers = vec![(1, "addr1".to_string()), (1, "dup".to_string())];
        assert!(devnet_boot(&g, &peers).is_err());
    }
}
