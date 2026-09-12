// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Devnet-Bootstrap Stufe 1 (SCR-0106, F-139): Genesis-Definition mit
//! Validierung, deterministischem Boot-Hash und Peer-Join ueber die
//! PeerTable. Ehrlichkeit: kein echtes Netzwerk-Socket, kein RPC, keine
//! Kryptographie (FNV-1a 64-bit als dokumentierter MVP-Platzhalter);
//! config/devnet/genesis.json ist das erklaerte Devnet-Artefakt; die
//! serde-File-Bindung (SCR-0114) erzwingt Code-Genesis == File-Genesis in CI.

use crate::config::CHAIN_ID;
use crate::peers::PeerTable;
use atc_algorithm::hash::atc_hash;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct Genesis {
    pub chain_id: u64,
    pub chain_name: String,
    pub genesis_height: u64,
    pub initial_peers: Vec<String>,
    pub state_root: String,
}

impl Genesis {
    /// Devnet-Genesis — spiegelt config/devnet/genesis.json (Chain-ID 658467).
    pub fn devnet() -> Self {
        Genesis {
            chain_id: CHAIN_ID,
            chain_name: "A-TownChain Devnet".to_string(),
            genesis_height: 0,
            initial_peers: vec!["atc-node-1".to_string(), "atc-node-2".to_string()],
            state_root: "0".repeat(64),
        }
    }

    /// Laedt die Genesis aus einer Datei (serde-Bindung, SCR-0114).
    /// Ehrlichkeit: keine Schema-Pruefung ueber die Feldtypen hinaus.
    pub fn from_file(path: &str) -> Result<Self, String> {
        let inhalt = std::fs::read_to_string(path).map_err(|e| format!("lesen {}: {}", path, e))?;
        serde_json::from_str(&inhalt).map_err(|e| format!("parsen {}: {}", path, e))
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.chain_id != CHAIN_ID {
            return Err(format!("Chain-ID {} verletzt Bindung {}", self.chain_id, CHAIN_ID));
        }
        if self.genesis_height != 0 {
            return Err(format!("Genesis-Height muss 0 sein, ist {}", self.genesis_height));
        }
        if self.chain_name.is_empty() {
            return Err("Chain-Name fehlt".to_string());
        }
        if self.initial_peers.len() < 2 {
            return Err("Devnet braucht mindestens 2 Initial-Peers".to_string());
        }
        Ok(())
    }

    /// Deterministischer Boot-Hash ueber kanonische Feldverkettung.
    /// ATC-HASH-001 TownHash-256, 64-Bit-Traversal (SCR-0120).
    /// Ehrlich: nicht kryptoanalysiert; Traversal verkuerzt den Digest.
    pub fn boot_hash(&self) -> u64 {
        townhash_u64(&format!(
            "{}|{}|{}|{}|{}",
            self.chain_id,
            self.chain_name,
            self.genesis_height,
            self.initial_peers.join(","),
            self.state_root
        ))
    }
}

/// Devnet-Boot: Genesis validieren, Peers anmelden und verifizieren,
/// deterministischen Boot-Hash zurueckgeben (Devnet-Gate Stufe 1).
pub fn devnet_boot(genesis: &Genesis, peers: &[(u64, String)]) -> Result<(PeerTable, u64), String> {
    genesis.validate()?;
    let mut table = PeerTable::new();
    for (id, addr) in peers {
        if !table.add(*id, addr.clone()) {
            return Err(format!("Peer {} schon angemeldet", id));
        }
        if !table.verify(*id) {
            return Err(format!("Peer {} nicht verifizierbar", id));
        }
    }
    Ok((table, genesis.boot_hash()))
}

pub(crate) fn townhash_u64(data: &str) -> u64 {
    // SCR-0120: ATC-HASH-001 (TownHash-256) aus atc-algorithm, rev-gepinnt.
    // Ehrlichkeit: 64-Bit-Traversal des 32-Byte-Digests (Devnet-Feldbreite);
    /// ATC-HASH-001 ist nicht kryptoanalysiert — Mainnet-Gate bleibt F-067.
    let digest = atc_hash(data.as_bytes());
    u64::from_le_bytes([
        digest[0], digest[1], digest[2], digest[3],
        digest[4], digest[5], digest[6], digest[7],
    ])
}

#[cfg(test)]
mod tests {
    #[test]
    fn genesis_file_bindung_ist_erzwungen() {
        let g = Genesis::from_file("config/devnet/genesis.json").expect("genesis.json lesbar");
        assert_eq!(g, Genesis::devnet(), "genesis.json und Code-Genesis duerfen nicht driften");
        g.validate().expect("File-Genesis valide");
        assert_eq!(g.boot_hash(), Genesis::devnet().boot_hash());
    }

    use super::*;

    #[test]
    fn devnet_genesis_valide() {
        let g = Genesis::devnet();
        assert!(g.validate().is_ok());
        assert_eq!(g.chain_id, 658467);
        assert_eq!(g.genesis_height, 0);
    }

    #[test]
    fn falsche_chain_id_abgelehnt() {
        let mut g = Genesis::devnet();
        g.chain_id = 1;
        assert!(g.validate().is_err());
    }

    #[test]
    fn zu_wenige_peers_abgelehnt() {
        let mut g = Genesis::devnet();
        g.initial_peers = vec!["allein".to_string()];
        assert!(g.validate().is_err());
    }

    #[test]
    fn boot_hash_deterministisch() {
        assert_eq!(Genesis::devnet().boot_hash(), Genesis::devnet().boot_hash());
    }

    #[test]
    fn zwei_nodes_gleiche_genesis_gleicher_hash() {
        let g = Genesis::devnet();
        let peers1 = vec![(1, "addr1".to_string()), (2, "addr2".to_string())];
        let peers2 = vec![(1, "addr1".to_string()), (2, "addr2".to_string())];
        let (t1, h1) = devnet_boot(&g, &peers1).expect("Boot 1 fehlgeschlagen");
        let (t2, h2) = devnet_boot(&g, &peers2).expect("Boot 2 fehlgeschlagen");
        assert_eq!(h1, h2, "Nodes mit gleicher Genesis muessen denselben Boot-Hash haben");
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
