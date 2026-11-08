// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — P2P Gossip Bridge (K-Sprint 28)
//!
//! Verbindet genesis_bridge.rs (K27) mit atcnet.rs (K24) für:
//!   1. Block-Gossip: Neue Blöcke → BlockAnn an alle Peers
//!   2. Block-Sync: GetBlocks/Blocks für fehlende Blöcke
//!   3. Vote-Gossip: Konsens-Votes über das Netzwerk
//!   4. Chain-ID-Validierung auf Network-Ebene (658467)
//!   5. Mempool-Gossip: TxBroadcast Integration
//!   6. Peer-Height-Tracking: Automatische Sync-Erkennung

use alloc::format;
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use crate::atcnet::{
    self, AtcNetHandler, AtcNetError, PeerId,
    BlockAnnMsg, GetBlocksMsg, BlocksMsg, BlockData,
    serialize_block_ann, deserialize_block_ann,
    serialize_get_blocks, deserialize_get_blocks,
    serialize_tx_broadcast,
    MessageType, CHAIN_ID,
};
use crate::genesis_bridge::{
    GenesisBridge, BridgeBlock, BridgeBlockChain, BridgeChainError,
};

// === Gossip Error === //

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GossipError {
    NetworkError(AtcNetError),
    ChainError(BridgeChainError),
    NoPeers,
    InvalidChainId,
    BlockNotFound,
    InvalidMessage,
    HeightMismatch,
    AlreadyKnown,
    PeerNotConnected,
}

impl From<AtcNetError> for GossipError {
    fn from(e: AtcNetError) -> Self { GossipError::NetworkError(e) }
}

impl From<BridgeChainError> for GossipError {
    fn from(e: BridgeChainError) -> Self { GossipError::ChainError(e) }
}

// === Peer State Tracking === //

#[derive(Debug, Clone)]
pub struct PeerState {
    pub conn_id: u64,
    pub did: String,
    pub peer_id: PeerId,
    pub block_height: u64,
    pub genesis_hash: [u8; 32],
    pub chain_id: u32,
    pub last_seen: u64,
}

// === Gossip Bridge === //

/// Verbindet GenesisBridge mit AtcNetHandler
pub struct GossipBridge {
    pub bridge: GenesisBridge,
    pub net: AtcNetHandler,
    pub peers: BTreeMap<u64, PeerState>,
    /// Blöcke die wir von Peers gehört haben aber noch nicht haben
    pub pending_announcements: Vec<BlockAnnMsg>,
    /// Unsere selbst erstellten Blöcke (für Sync-Antworten)
    pub known_blocks: BTreeMap<u64, BridgeBlock>,
}

impl GossipBridge {
    /// Initialisiert die Gossip Bridge aus einer Genesis-Konfiguration
    pub fn init(
        config: &crate::genesis::GenesisConfig,
        self_peer_id: PeerId,
        self_did: String,
    ) -> Result<Self, GossipError> {
        let bridge = GenesisBridge::init_from_config(config)
            .map_err(|_| GossipError::InvalidChainId)?;

        let net = AtcNetHandler::new(self_peer_id, self_did);
        net.set_height(0);

        // Genesis-Block bekannt machen
        let genesis = bridge.chain.get_block(0).unwrap().clone();
        let mut known_blocks = BTreeMap::new();
        known_blocks.insert(0, genesis);

        Ok(GossipBridge {
            bridge,
            net,
            peers: BTreeMap::new(),
            pending_announcements: Vec::new(),
            known_blocks,
        })
    }

    // === Peer Management === //

    /// Verbindet zu einem Peer und startet Handshake
    pub fn connect_peer(
        &mut self,
        peer_id: PeerId,
        peer_did: String,
    ) -> Result<u64, GossipError> {
        let conn_id = self.net.connect(peer_id, peer_did.clone())?;

        let peer_state = PeerState {
            conn_id,
            did: peer_did,
            peer_id,
            block_height: 0,
            genesis_hash: [0u8; 32],
            chain_id: 0,
            last_seen: 0,
        };
        self.peers.insert(conn_id, peer_state);

        Ok(conn_id)
    }

    /// Schliesst den Handshake ab (Connecting → Connected)
    /// In Produktion: wird durch handle_message mit HandshakeMsg aufgerufen
    pub fn complete_handshake(&mut self, conn_id: u64, listen_port: u16) -> Result<(), GossipError> {
        // Sende unseren Handshake
        let _data = self.net.send_handshake(conn_id, listen_port)?;
        // Simuliere Empfang der Gegenseite (in Produktion: über TCP)
        // handle_handshake setzt state = Connected
        // Wir tun so als ob wir einen Handshake empfangen
        let fake_hs = crate::atcnet::serialize_handshake(&crate::atcnet::HandshakeMsg {
            protocol_version: crate::atcnet::PROTOCOL_VERSION,
            chain_id: crate::atcnet::CHAIN_ID,
            peer_id: self.peers.get(&conn_id).map(|p| p.peer_id).unwrap_or([0u8; 32]),
            peer_did: self.peers.get(&conn_id).map(|p| p.did.clone()).unwrap_or_default(),
            listen_port,
            current_height: 0,
        });
        let _ = self.net.handle_message(conn_id, &fake_hs);
        Ok(())
    }

    /// Trennt einen Peer
    pub fn disconnect_peer(&mut self, conn_id: u64) -> bool {
        self.peers.remove(&conn_id);
        self.net.disconnect(conn_id)
    }

    /// Anzahl verbundener Peers
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    /// Alle Peer-Connection-IDs
    pub fn peer_conn_ids(&self) -> Vec<u64> {
        self.peers.keys().cloned().collect()
    }

    /// Aktualisiert die Blockhöhe eines Peers
    pub fn update_peer_height(&mut self, conn_id: u64, height: u64) {
        if let Some(p) = self.peers.get_mut(&conn_id) {
            p.block_height = height;
        }
    }

    /// Beste Peer-Height (für Sync-Entscheidungen)
    pub fn best_peer_height(&self) -> Option<(u64, u64)> {
        self.peers.values()
            .max_by_key(|p| p.block_height)
            .map(|p| (p.conn_id, p.block_height))
    }

    // === Gap 1: Block-Gossip (BridgeBlock → BlockAnn) === //

    /// Broadcastet einen neuen Block an alle verbundenen Peers
    pub fn gossip_block(&mut self, block: &BridgeBlock) -> Result<usize, GossipError> {
        if block.chain_id != CHAIN_ID {
            return Err(GossipError::InvalidChainId);
        }

        let conn_ids = self.net.connection_ids();
        let mut sent = 0;

        for conn_id in conn_ids {
            let data = self.net.send_block_ann(
                conn_id,
                block.id,
                block.height,
                block.parent_hash,
            )?;
            // In Produktion: data wird über TCP gesendet
            sent += 1;

            // Peer-Höhe aktualisieren
            if let Some(p) = self.peers.get_mut(&conn_id) {
                p.block_height = block.height;
                p.last_seen = block.timestamp;
            }
        }

        // Block lokal speichern
        self.known_blocks.insert(block.height, block.clone());

        Ok(sent)
    }

    /// Propose + Gossip in einem Aufruf
    pub fn propose_and_gossip(
        &mut self,
        proposer_did: &str,
        timestamp: u64,
        tx_root: [u8; 32],
    ) -> Result<(BridgeBlock, usize), GossipError> {
        let block = self.bridge.propose_block(proposer_did, timestamp, tx_root)?;
        let peers = self.gossip_block(&block)?;
        Ok((block, peers))
    }

    // === Gap 2: Block-Sync (GetBlocks / Blocks) === //

    /// Sendet eine GetBlocks-Anfrage an einen Peer
    pub fn request_blocks(
        &mut self,
        conn_id: u64,
        from_height: u64,
        max_count: u16,
    ) -> Result<Vec<u8>, GossipError> {
        if !self.peers.contains_key(&conn_id) {
            return Err(GossipError::PeerNotConnected);
        }

        let msg = GetBlocksMsg { from_height, max_count };
        let data = serialize_get_blocks(&msg);
        // In Produktion: send data over TCP to conn_id
        Ok(data)
    }

    /// Fordert fehlende Blöcke vom besten Peer an
    pub fn sync_from_best_peer(&mut self) -> Result<Option<Vec<u8>>, GossipError> {
        let our_height = self.bridge.height();

        match self.best_peer_height() {
            Some((conn_id, peer_height)) if peer_height > our_height => {
                let needed = (peer_height - our_height) as u16;
                let data = self.request_blocks(conn_id, our_height + 1, needed.min(50))?;
                Ok(Some(data))
            }
            _ => Ok(None),
        }
    }

    /// Antwortet auf eine GetBlocks-Anfrage mit unseren Blöcken
    pub fn respond_get_blocks(
        &self,
        from_height: u64,
        max_count: u16,
    ) -> Result<BlocksMsg, GossipError> {
        let mut blocks = Vec::new();
        let count = max_count as usize;

        for h in from_height..(from_height + count as u64) {
            if let Some(block) = self.known_blocks.get(&h) {
                let block_data = serialize_bridge_block(block);
                blocks.push(BlockData {
                    height: block.height,
                    hash: block.id,
                    data: block_data,
                });
            } else {
                break;
            }
        }

        if blocks.is_empty() {
            return Err(GossipError::BlockNotFound);
        }

        Ok(BlocksMsg { blocks })
    }

    /// Verarbeitet empfangene Blöcke (BlocksMsg)
    pub fn process_blocks(&mut self, msg: &BlocksMsg) -> Result<usize, GossipError> {
        let mut added = 0;

        for bd in &msg.blocks {
            // Chain-ID aus Block-Daten prüfen
            if bd.data.len() < 4 {
                return Err(GossipError::InvalidMessage);
            }

            let block = deserialize_bridge_block(&bd.data)
                .ok_or(GossipError::InvalidMessage)?;

            if block.chain_id != CHAIN_ID {
                return Err(GossipError::InvalidChainId);
            }

            // Versuche Block zur Chain hinzuzufügen
            match self.bridge.chain.add_block(block.clone()) {
                Ok(()) => {
                    self.known_blocks.insert(block.height, block);
                    added += 1;
                }
                Err(BridgeChainError::BlockExists) | Err(BridgeChainError::InvalidHeight) => {
                    // Already known or out of order — skip
                }
                Err(e) => return Err(GossipError::ChainError(e)),
            }
        }

        // Höhe im Netzwerk-Handler aktualisieren
        self.net.set_height(self.bridge.height());

        Ok(added)
    }

    // === Gap 3: Vote-Gossip === //

    /// Vote-Nachricht (vereinfacht: 32 bytes vertex_id + 1 byte approve + 64 bytes sig)
    pub fn gossip_vote(
        &mut self,
        vertex_id: [u8; 32],
        approve: bool,
        signature: [u8; 64],
    ) -> Result<usize, GossipError> {
        let conn_ids = self.net.connection_ids();
        let mut vote_data = Vec::with_capacity(97);
        vote_data.extend_from_slice(&vertex_id);
        vote_data.push(if approve { 1 } else { 0 });
        vote_data.extend_from_slice(&signature);

        // Votes werden als TxBroadcast mit speziellem Prefix verschickt
        let tx_hash = simple_hash_local(&vote_data);
        let mut sent = 0;

        for conn_id in conn_ids {
            let msg_data = serialize_tx_broadcast(&crate::atcnet::TxBroadcastMsg {
                tx_hash,
                tx_data: vote_data.clone(),
            });
            // In Produktion: send msg_data to conn_id
            let _ = msg_data;
            sent += 1;
        }

        Ok(sent)
    }

    /// Verarbeitet einen empfangenen Vote
    pub fn process_vote(&self, data: &[u8]) -> Result<([u8; 32], bool, [u8; 64]), GossipError> {
        if data.len() != 97 {
            return Err(GossipError::InvalidMessage);
        }
        let mut vertex_id = [0u8; 32];
        vertex_id.copy_from_slice(&data[..32]);
        let approve = data[32] == 1;
        let mut signature = [0u8; 64];
        signature.copy_from_slice(&data[33..]);

        Ok((vertex_id, approve, signature))
    }

    // === Gap 4: Chain-ID-Validierung auf Network-Ebene === //

    /// Validiert dass ein Peer dieselbe Chain-ID hat
    pub fn validate_peer_chain(&self, conn_id: u64, chain_id: u32) -> Result<(), GossipError> {
        if chain_id != CHAIN_ID {
            return Err(GossipError::InvalidChainId);
        }
        if !self.peers.contains_key(&conn_id) {
            return Err(GossipError::PeerNotConnected);
        }
        Ok(())
    }

    /// Validiert eine eingehende BlockAnn gegen unsere Chain
    pub fn validate_block_ann(&self, ann: &BlockAnnMsg) -> Result<bool, GossipError> {
        // Höhe muss > 0 sein (Genesis wird nicht annonciert)
        if ann.block_height == 0 {
            return Ok(false);
        }
        // Prüfe ob wir den Block schon haben
        if self.known_blocks.contains_key(&ann.block_height) {
            return Ok(false); // Already known
        }
        Ok(true)
    }

    // === Gap 5: Mempool-Gossip === //

    /// Broadcastet eine Transaktion an alle Peers
    pub fn gossip_transaction(
        &mut self,
        tx_hash: [u8; 32],
        tx_data: Vec<u8>,
    ) -> usize {
        self.net.gossip_tx(tx_hash, tx_data).len()
    }

    // === Gap 6: Peer-Height-Tracking === //

    /// Verarbeitet eine eingehende BlockAnn und aktualisiert Peer-Height
    pub fn handle_block_ann(
        &mut self,
        conn_id: u64,
        ann: &BlockAnnMsg,
    ) -> Result<bool, GossipError> {
        // Peer-Height aktualisieren
        if let Some(p) = self.peers.get_mut(&conn_id) {
            if ann.block_height > p.block_height {
                p.block_height = ann.block_height;
            }
            p.last_seen = ann.block_height; // Vereinfacht
        }

        // Validiere die Announcement
        if !self.validate_block_ann(ann)? {
            return Ok(false); // Already known or invalid
        }

        // Zur Pending-Liste hinzufügen
        self.pending_announcements.push(ann.clone());

        // Sync nötig?
        if ann.block_height > self.bridge.height() {
            return Ok(true); // Need sync
        }

        Ok(false)
    }

    /// Verarbeitet eine eingehende Nachricht von einem Peer
    pub fn handle_peer_message(
        &mut self,
        conn_id: u64,
        data: &[u8],
    ) -> Result<Option<Vec<u8>>, GossipError> {
        if data.is_empty() {
            return Err(GossipError::InvalidMessage);
        }

        let msg_type = MessageType::from_byte(data[0]);

        match msg_type {
            Some(MessageType::BlockAnn) => {
                let ann = deserialize_block_ann(data)
                    .map_err(|_| GossipError::InvalidMessage)?;
                let needs_sync = self.handle_block_ann(conn_id, &ann)?;
                if needs_sync {
                    // Request missing blocks
                    let sync_data = self.request_blocks(
                        conn_id,
                        self.bridge.height() + 1,
                        50,
                    )?;
                    Ok(Some(sync_data))
                } else {
                    Ok(None)
                }
            }
            Some(MessageType::GetBlocks) => {
                let msg = deserialize_get_blocks(data)
                    .map_err(|_| GossipError::InvalidMessage)?;
                let response = self.respond_get_blocks(msg.from_height, msg.max_count)?;
                // Serialize BlocksMsg (simplified — in production: proper serializer)
                let mut buf = Vec::new();
                buf.push(MessageType::Blocks as u8);
                buf.extend_from_slice(&(response.blocks.len() as u32).to_le_bytes());
                for bd in &response.blocks {
                    buf.extend_from_slice(&bd.height.to_le_bytes());
                    buf.extend_from_slice(&bd.hash);
                    buf.extend_from_slice(&(bd.data.len() as u32).to_le_bytes());
                    buf.extend_from_slice(&bd.data);
                }
                Ok(Some(buf))
            }
            Some(MessageType::Blocks) => {
                // Parse BlocksMsg (simplified)
                if data.len() < 5 {
                    return Err(GossipError::InvalidMessage);
                }
                let count = u32::from_le_bytes(
                    data[1..5].try_into().unwrap()
                ) as usize;
                let mut blocks = Vec::new();
                let mut offset = 5;

                for _ in 0..count {
                    if offset + 40 > data.len() {
                        return Err(GossipError::InvalidMessage);
                    }
                    let height = u64::from_le_bytes(
                        data[offset..offset+8].try_into().unwrap()
                    );
                    let mut hash = [0u8; 32];
                    hash.copy_from_slice(&data[offset+8..offset+40]);
                    offset += 40;

                    if offset + 4 > data.len() {
                        return Err(GossipError::InvalidMessage);
                    }
                    let data_len = u32::from_le_bytes(
                        data[offset..offset+4].try_into().unwrap()
                    ) as usize;
                    offset += 4;

                    if offset + data_len > data.len() {
                        return Err(GossipError::InvalidMessage);
                    }
                    let block_data = data[offset..offset+data_len].to_vec();
                    offset += data_len;

                    blocks.push(BlockData { height, hash, data: block_data });
                }

                let msg = BlocksMsg { blocks };
                let added = self.process_blocks(&msg)?;
                if added > 0 {
                    // Re-gossip to other peers (relay)
                    Ok(None) // In production: gossip to other peers
                } else {
                    Ok(None)
                }
            }
            Some(MessageType::TxBroadcast) => {
                // Could be a vote or a real transaction
                if let Ok(tx_msg) = crate::atcnet::deserialize_tx_broadcast(data) {
                    if tx_msg.tx_data.len() == 97 {
                        // It's a vote
                        let _ = self.process_vote(&tx_msg.tx_data)?;
                        Ok(None)
                    } else {
                        // Regular transaction
                        Ok(None)
                    }
                } else {
                    Err(GossipError::InvalidMessage)
                }
            }
            Some(MessageType::Ping) => {
                // Update last_seen
                if let Some(p) = self.peers.get_mut(&conn_id) {
                    p.last_seen = 1;
                }
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    // === Status === //

    pub fn height(&self) -> u64 {
        self.bridge.height()
    }

    pub fn chain_id(&self) -> u32 {
        self.bridge.chain_id()
    }

    pub fn genesis_hash(&self) -> [u8; 32] {
        self.bridge.genesis_hash()
    }

    pub fn total_stake(&self) -> u64 {
        self.bridge.total_stake()
    }

    pub fn pending_sync_count(&self) -> usize {
        self.pending_announcements.len()
    }

    /// Statistik
    pub fn stats(&self) -> GossipStats {
        GossipStats {
            height: self.bridge.height(),
            peer_count: self.peer_count(),
            known_blocks: self.known_blocks.len(),
            pending_announcements: self.pending_announcements.len(),
            chain_id: self.bridge.chain_id(),
            total_stake: self.bridge.total_stake(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GossipStats {
    pub height: u64,
    pub peer_count: usize,
    pub known_blocks: usize,
    pub pending_announcements: usize,
    pub chain_id: u32,
    pub total_stake: u64,
}

// === Block Serialization (for network transfer) === //

pub fn serialize_bridge_block(block: &BridgeBlock) -> Vec<u8> {
    let mut buf = Vec::new();
    // Chain-ID (4 bytes)
    buf.extend_from_slice(&block.chain_id.to_le_bytes());
    // Height
    buf.extend_from_slice(&block.height.to_le_bytes());
    // Block ID
    buf.extend_from_slice(&block.id);
    // Parent hash
    buf.extend_from_slice(&block.parent_hash);
    // Proposer DID
    buf.extend_from_slice(&(block.proposer_did.len() as u16).to_le_bytes());
    buf.extend_from_slice(block.proposer_did.as_bytes());
    // Timestamp
    buf.extend_from_slice(&block.timestamp.to_le_bytes());
    // PoH hash
    buf.extend_from_slice(&block.poh_hash);
    // Tx root
    buf.extend_from_slice(&block.tx_root);
    // State root
    buf.extend_from_slice(&block.state_root);
    // Gas used
    buf.extend_from_slice(&block.gas_used.to_le_bytes());
    // Total fees
    buf.extend_from_slice(&block.total_fees.to_le_bytes());
    // Signature
    buf.extend_from_slice(&block.signature);
    // Validator set
    buf.extend_from_slice(&(block.validator_set.len() as u16).to_le_bytes());
    for (did, stake) in &block.validator_set {
        buf.extend_from_slice(&(did.len() as u16).to_le_bytes());
        buf.extend_from_slice(did.as_bytes());
        buf.extend_from_slice(&stake.to_le_bytes());
    }
    // Allocations
    buf.extend_from_slice(&(block.allocations.len() as u16).to_le_bytes());
    for (addr, amount) in &block.allocations {
        buf.extend_from_slice(&(addr.len() as u16).to_le_bytes());
        buf.extend_from_slice(addr.as_bytes());
        buf.extend_from_slice(&amount.to_le_bytes());
    }
    buf
}

pub fn deserialize_bridge_block(data: &[u8]) -> Option<BridgeBlock> {
    if data.len() < 4 + 8 + 32 + 32 + 2 + 8 + 32 + 32 + 32 + 8 + 8 + 64 + 2 {
        return None;
    }

    let mut offset = 0;
    let chain_id = u32::from_le_bytes(data[offset..offset+4].try_into().ok()?);
    offset += 4;
    let height = u64::from_le_bytes(data[offset..offset+8].try_into().ok()?);
    offset += 8;
    let mut id = [0u8; 32];
    id.copy_from_slice(&data[offset..offset+32]);
    offset += 32;
    let mut parent_hash = [0u8; 32];
    parent_hash.copy_from_slice(&data[offset..offset+32]);
    offset += 32;

    let did_len = u16::from_le_bytes(data[offset..offset+2].try_into().ok()?) as usize;
    offset += 2;
    if offset + did_len > data.len() { return None; }
    let proposer_did = String::from_utf8(data[offset..offset+did_len].to_vec()).ok()?;
    offset += did_len;

    let timestamp = u64::from_le_bytes(data[offset..offset+8].try_into().ok()?);
    offset += 8;
    let mut poh_hash = [0u8; 32];
    poh_hash.copy_from_slice(&data[offset..offset+32]);
    offset += 32;
    let mut tx_root = [0u8; 32];
    tx_root.copy_from_slice(&data[offset..offset+32]);
    offset += 32;
    let mut state_root = [0u8; 32];
    state_root.copy_from_slice(&data[offset..offset+32]);
    offset += 32;
    let gas_used = u64::from_le_bytes(data[offset..offset+8].try_into().ok()?);
    offset += 8;
    let total_fees = u64::from_le_bytes(data[offset..offset+8].try_into().ok()?);
    offset += 8;
    let mut signature = [0u8; 64];
    signature.copy_from_slice(&data[offset..offset+64]);
    offset += 64;

    // Validator set
    let vs_len = u16::from_le_bytes(data[offset..offset+2].try_into().ok()?) as usize;
    offset += 2;
    let mut validator_set = Vec::new();
    for _ in 0..vs_len {
        let dlen = u16::from_le_bytes(data[offset..offset+2].try_into().ok()?) as usize;
        offset += 2;
        let did = String::from_utf8(data[offset..offset+dlen].to_vec()).ok()?;
        offset += dlen;
        let stake = u64::from_le_bytes(data[offset..offset+8].try_into().ok()?);
        offset += 8;
        validator_set.push((did, stake));
    }

    // Allocations
    let al_len = u16::from_le_bytes(data[offset..offset+2].try_into().ok()?) as usize;
    offset += 2;
    let mut allocations = Vec::new();
    for _ in 0..al_len {
        let alen = u16::from_le_bytes(data[offset..offset+2].try_into().ok()?) as usize;
        offset += 2;
        let addr = String::from_utf8(data[offset..offset+alen].to_vec()).ok()?;
        offset += alen;
        let amount = u64::from_le_bytes(data[offset..offset+8].try_into().ok()?);
        offset += 8;
        allocations.push((addr, amount));
    }

    Some(BridgeBlock {
        id, height, parent_hash, proposer_did, timestamp,
        poh_hash, tx_root, state_root, gas_used, total_fees,
        signature, chain_id, validator_set, allocations,
    })
}

fn simple_hash_local(data: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut h: u64 = 0xcbf29ce484222325;
    for (i, &b) in data.iter().enumerate() {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
        let off = (i * 4) % 24;
        result[off..off+8].copy_from_slice(&h.to_le_bytes());
    }
    result
}

// === Tests === //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genesis::{GenesisConfig, GENESIS_CHAIN_ID, LockType, GenesisValidator, GenesisAllocation};

    fn dummy_pubkey(n: u8) -> [u8; 33] {
        let mut k = [0u8; 33]; k[0] = 0x02; k[1] = n; k
    }
    fn dummy_address(n: u8) -> String {
        format!("ATC{}", "a".repeat(30).chars().chain(core::iter::once((b'a' + n) as char)).collect::<String>())
    }
    fn dummy_did(n: u8) -> String { format!("did:shivacore:validator{}", n) }
    fn make_validator(n: u8, stake: u64) -> GenesisValidator {
        GenesisValidator { did: dummy_did(n), pubkey: dummy_pubkey(n), stake, address: dummy_address(n), commission: 500 }
    }
    fn make_test_config() -> GenesisConfig {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, 1726358400);
        for i in 1..=4u8 { config.add_validator(make_validator(i, 10000)).unwrap(); }
        for i in 1..=4u8 {
            config.add_allocation(GenesisAllocation { address: dummy_address(i), amount: 1_000_000_000, lock_type: LockType::None, lock_duration: 0 }).unwrap();
        }
        config.memo = "A-TownChain Mainnet Genesis".to_string();
        config
    }
    fn peer_id(n: u8) -> PeerId {
        let mut p = [0u8; 32]; p[0] = n; p
    }

    // === Init Tests === //

    #[test]
    fn test_gossip_bridge_init() {
        let config = make_test_config();
        let gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        assert_eq!(gb.chain_id(), 658467);
        assert_eq!(gb.height(), 0);
        assert_eq!(gb.peer_count(), 0);
        assert_eq!(gb.known_blocks.len(), 1); // Genesis
        assert_ne!(gb.genesis_hash(), [0u8; 32]);
    }

    #[test]
    fn test_gossip_bridge_init_invalid_config() {
        let config = GenesisConfig::new(9999, 100);
        assert!(GossipBridge::init(&config, peer_id(1), "did:shivacore:me".into()).is_err());
    }

    // === Peer Management === //

    #[test]
    fn test_connect_peer() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        assert_eq!(gb.peer_count(), 1);
        assert!(gb.peers.contains_key(&conn));
    }

    #[test]
    fn test_disconnect_peer() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        assert_eq!(gb.peer_count(), 1);

        gb.disconnect_peer(conn);
        assert_eq!(gb.peer_count(), 0);
    }

    #[test]
    fn test_connect_multiple_peers() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        for i in 2..=5u8 {
            gb.connect_peer(peer_id(i), dummy_did(i - 1)).unwrap();
        }
        assert_eq!(gb.peer_count(), 4);
    }

    #[test]
    fn test_update_peer_height() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();

        gb.update_peer_height(conn, 5);
        assert_eq!(gb.peers.get(&conn).unwrap().block_height, 5);
    }

    #[test]
    fn test_best_peer_height() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let c1 = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        let c2 = gb.connect_peer(peer_id(3), dummy_did(2)).unwrap();

        gb.update_peer_height(c1, 3);
        gb.update_peer_height(c2, 7);

        let best = gb.best_peer_height().unwrap();
        assert_eq!(best.0, c2);
        assert_eq!(best.1, 7);
    }

    // === Block-Gossip === //

    #[test]
    fn test_gossip_block_no_peers() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let block = gb.bridge.chain.get_block(0).unwrap().clone();
        let sent = gb.gossip_block(&block).unwrap();
        assert_eq!(sent, 0); // No peers
    }

    #[test]
    fn test_gossip_block_with_peers() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let c1 = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(c1, 9000).unwrap();
        let c2 = gb.connect_peer(peer_id(3), dummy_did(2)).unwrap();
        gb.complete_handshake(c2, 9000).unwrap();

        let block = gb.bridge.chain.get_block(0).unwrap().clone();
        let sent = gb.gossip_block(&block).unwrap();
        assert_eq!(sent, 2);
    }

    #[test]
    fn test_propose_and_gossip() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let c = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(c, 9000).unwrap();

        let proposer = gb.bridge.next_proposer().unwrap();
        let (block, peers) = gb.propose_and_gossip(&proposer, 2000, [0xAB; 32]).unwrap();

        assert_eq!(block.height, 1);
        assert_eq!(peers, 1);
        assert_eq!(gb.height(), 1);
    }

    #[test]
    fn test_gossip_block_wrong_chain_id() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let mut block = gb.bridge.chain.get_block(0).unwrap().clone();
        block.chain_id = 9999;
        assert_eq!(gb.gossip_block(&block), Err(GossipError::InvalidChainId));
    }

    // === Block Sync === //

    #[test]
    fn test_request_blocks() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(conn, 9000).unwrap();

        let data = gb.request_blocks(conn, 1, 10).unwrap();
        assert!(!data.is_empty());
        assert_eq!(data[0], MessageType::GetBlocks as u8);
    }

    #[test]
    fn test_request_blocks_peer_not_connected() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        assert_eq!(gb.request_blocks(999, 1, 10), Err(GossipError::PeerNotConnected));
    }

    #[test]
    fn test_respond_get_blocks_genesis() {
        let config = make_test_config();
        let gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let response = gb.respond_get_blocks(0, 1).unwrap();
        assert_eq!(response.blocks.len(), 1);
        assert_eq!(response.blocks[0].height, 0);
    }

    #[test]
    fn test_respond_get_blocks_not_found() {
        let config = make_test_config();
        let gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        assert_eq!(gb.respond_get_blocks(100, 1), Err(GossipError::BlockNotFound));
    }

    #[test]
    fn test_sync_from_best_peer() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(conn, 9000).unwrap();
        gb.update_peer_height(conn, 5);

        let result = gb.sync_from_best_peer().unwrap();
        assert!(result.is_some()); // Should request blocks
    }

    #[test]
    fn test_sync_not_needed() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.update_peer_height(conn, 0); // Same height as us

        let result = gb.sync_from_best_peer().unwrap();
        assert!(result.is_none()); // No sync needed
    }

    // === Block Serialization === //

    #[test]
    fn test_serialize_deserialize_block() {
        let config = make_test_config();
        let gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let block = gb.bridge.chain.get_block(0).unwrap().clone();

        let data = serialize_bridge_block(&block);
        assert!(!data.is_empty());

        let restored = deserialize_bridge_block(&data).unwrap();
        assert_eq!(restored.id, block.id);
        assert_eq!(restored.height, block.height);
        assert_eq!(restored.chain_id, block.chain_id);
        assert_eq!(restored.validator_set, block.validator_set);
        assert_eq!(restored.allocations, block.allocations);
    }

    #[test]
    fn test_serialize_deserialize_block_with_proposer() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let proposer = gb.bridge.next_proposer().unwrap();
        let block = gb.bridge.propose_block(&proposer, 2000, [0xAB; 32]).unwrap();

        let data = serialize_bridge_block(&block);
        let restored = deserialize_bridge_block(&data).unwrap();
        assert_eq!(restored.proposer_did, block.proposer_did);
        assert_eq!(restored.id, block.id);
    }

    #[test]
    fn test_deserialize_invalid_data() {
        assert!(deserialize_bridge_block(&[0; 10]).is_none());
        assert!(deserialize_bridge_block(&[]).is_none());
    }

    // === Process Blocks === //

    #[test]
    fn test_process_blocks_from_peer() {
        let config = make_test_config();

        // Node A: has blocks up to height 2
        let mut gb_a = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let p1 = gb_a.bridge.next_proposer().unwrap();
        let b1 = gb_a.bridge.propose_block(&p1, 2000, [0xAB; 32]).unwrap();
        gb_a.known_blocks.insert(1, b1.clone());
        let p2 = gb_a.bridge.next_proposer().unwrap();
        let b2 = gb_a.bridge.propose_block(&p2, 3000, [0xCD; 32]).unwrap();
        gb_a.known_blocks.insert(2, b2.clone());

        // Node B: only genesis
        let mut gb_b = GossipBridge::init(&config, peer_id(2), dummy_did(1)).unwrap();

        // Node A responds to GetBlocks(1, 2)
        let response = gb_a.respond_get_blocks(1, 2).unwrap();
        assert_eq!(response.blocks.len(), 2);

        // Node B processes the blocks
        let added = gb_b.process_blocks(&response).unwrap();
        assert_eq!(added, 2);
        assert_eq!(gb_b.height(), 2);
    }

    #[test]
    fn test_process_blocks_duplicate_ignored() {
        let config = make_test_config();
        let mut gb_a = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let p = gb_a.bridge.next_proposer().unwrap();
        let b1 = gb_a.bridge.propose_block(&p, 2000, [0xAB; 32]).unwrap();
        gb_a.known_blocks.insert(1, b1.clone());

        let mut gb_b = GossipBridge::init(&config, peer_id(2), dummy_did(1)).unwrap();
        let p = gb_b.bridge.next_proposer().unwrap();
        gb_b.bridge.propose_block(&p, 2000, [0xAB; 32]).unwrap(); // Same block
        gb_b.known_blocks.insert(1, gb_b.bridge.chain.get_block(1).unwrap().clone());

        let response = gb_a.respond_get_blocks(1, 1).unwrap();
        let added = gb_b.process_blocks(&response).unwrap();
        assert_eq!(added, 0); // Already known
    }

    // === Vote-Gossip === //

    #[test]
    fn test_gossip_vote() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let c1 = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(c1, 9000).unwrap();
        let c2 = gb.connect_peer(peer_id(3), dummy_did(2)).unwrap();
        gb.complete_handshake(c2, 9000).unwrap();

        let sent = gb.gossip_vote([0x42; 32], true, [0xAA; 64]).unwrap();
        assert_eq!(sent, 2);
    }

    #[test]
    fn test_gossip_vote_no_peers() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let sent = gb.gossip_vote([0x42; 32], true, [0xAA; 64]).unwrap();
        assert_eq!(sent, 0);
    }

    #[test]
    fn test_process_vote_valid() {
        let config = make_test_config();
        let gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let mut data = Vec::new();
        data.extend_from_slice(&[0x42; 32]);
        data.push(1); // approve
        data.extend_from_slice(&[0xAA; 64]);

        let (vertex_id, approve, sig) = gb.process_vote(&data).unwrap();
        assert_eq!(vertex_id, [0x42; 32]);
        assert!(approve);
        assert_eq!(sig, [0xAA; 64]);
    }

    #[test]
    fn test_process_vote_invalid_length() {
        let config = make_test_config();
        let gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        assert_eq!(gb.process_vote(&[0; 50]), Err(GossipError::InvalidMessage));
    }

    // === Chain-ID Validation === //

    #[test]
    fn test_validate_peer_chain_ok() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();

        assert!(gb.validate_peer_chain(conn, 658467).is_ok());
    }

    #[test]
    fn test_validate_peer_chain_wrong_id() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();

        assert_eq!(gb.validate_peer_chain(conn, 9999), Err(GossipError::InvalidChainId));
    }

    #[test]
    fn test_validate_block_ann_new() {
        let config = make_test_config();
        let gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let ann = BlockAnnMsg { block_hash: [0x01; 32], block_height: 1, prev_hash: [0u8; 32] };
        assert_eq!(gb.validate_block_ann(&ann), Ok(true));
    }

    #[test]
    fn test_validate_block_ann_known() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let p = gb.bridge.next_proposer().unwrap();
        gb.bridge.propose_block(&p, 2000, [0xAB; 32]).unwrap();
        let b1 = gb.bridge.chain.get_block(1).unwrap().clone();
        gb.known_blocks.insert(1, b1.clone());

        let block = gb.bridge.chain.get_block(1).unwrap();
        let ann = BlockAnnMsg { block_hash: block.id, block_height: 1, prev_hash: block.parent_hash };
        assert_eq!(gb.validate_block_ann(&ann), Ok(false)); // Already known
    }

    #[test]
    fn test_validate_block_ann_genesis_rejected() {
        let config = make_test_config();
        let gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        let ann = BlockAnnMsg { block_hash: [0x01; 32], block_height: 0, prev_hash: [0u8; 32] };
        assert_eq!(gb.validate_block_ann(&ann), Ok(false)); // Genesis not annonced
    }

    // === Mempool Gossip === //

    #[test]
    fn test_gossip_transaction() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let c1 = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(c1, 9000).unwrap();
        let c2 = gb.connect_peer(peer_id(3), dummy_did(2)).unwrap();
        gb.complete_handshake(c2, 9000).unwrap();

        let sent = gb.gossip_transaction([0x42; 32], vec![1, 2, 3, 4]);
        assert_eq!(sent, 2);
    }

    // === Peer Height Tracking === //

    #[test]
    fn test_handle_block_ann_updates_height() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(conn, 9000).unwrap();

        let ann = BlockAnnMsg { block_hash: [0x01; 32], block_height: 5, prev_hash: [0u8; 32] };
        let needs_sync = gb.handle_block_ann(conn, &ann).unwrap();

        assert!(needs_sync); // Peer is ahead
        assert_eq!(gb.peers.get(&conn).unwrap().block_height, 5);
        assert_eq!(gb.pending_sync_count(), 1);
    }

    #[test]
    fn test_handle_block_ann_no_sync_needed() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let p = gb.bridge.next_proposer().unwrap();
        gb.bridge.propose_block(&p, 2000, [0xAB; 32]).unwrap(); // Height 1
        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(conn, 9000).unwrap();

        let ann = BlockAnnMsg { block_hash: [0x01; 32], block_height: 1, prev_hash: [0u8; 32] };
        let needs_sync = gb.handle_block_ann(conn, &ann).unwrap();

        assert!(!needs_sync); // Same height
    }

    // === Stats === //

    #[test]
    fn test_gossip_stats() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let c = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(c, 9000).unwrap();

        let p = gb.bridge.next_proposer().unwrap();
        gb.bridge.propose_block(&p, 2000, [0xAB; 32]).unwrap();
        gb.known_blocks.insert(1, gb.bridge.chain.get_block(1).unwrap().clone());

        let stats = gb.stats();
        assert_eq!(stats.height, 1);
        assert_eq!(stats.peer_count, 1);
        assert_eq!(stats.chain_id, 658467);
        assert_eq!(stats.total_stake, 40000);
    }

    // === Multi-Node Scenario === //

    #[test]
    fn test_multi_node_block_propagation() {
        let config = make_test_config();

        // Node A
        let mut gb_a = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        // Node B
        let mut gb_b = GossipBridge::init(&config, peer_id(2), dummy_did(1)).unwrap();
        // Node C
        let mut gb_c = GossipBridge::init(&config, peer_id(3), dummy_did(2)).unwrap();

        // A connects to B and C
        let conn_ab = gb_a.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb_a.complete_handshake(conn_ab, 9000).unwrap();
        let conn_ac = gb_a.connect_peer(peer_id(3), dummy_did(2)).unwrap();
        gb_a.complete_handshake(conn_ac, 9000).unwrap();

        // A proposes a block and gossips
        let proposer = gb_a.bridge.next_proposer().unwrap();
        let (block, sent) = gb_a.propose_and_gossip(&proposer, 2000, [0xAB; 32]).unwrap();
        assert_eq!(sent, 2);
        assert_eq!(gb_a.height(), 1);

        // B and C receive the BlockAnn and need to sync
        let ann = BlockAnnMsg {
            block_hash: block.id,
            block_height: block.height,
            prev_hash: block.parent_hash,
        };

        let needs_b = gb_b.handle_block_ann(conn_ab, &ann).unwrap();
        let needs_c = gb_c.handle_block_ann(conn_ac, &ann).unwrap();
        assert!(needs_b);
        assert!(needs_c);

        // B requests blocks from A
        let response = gb_a.respond_get_blocks(1, 1).unwrap();
        let added_b = gb_b.process_blocks(&response).unwrap();
        assert_eq!(added_b, 1);
        assert_eq!(gb_b.height(), 1);

        // C also syncs
        let response2 = gb_a.respond_get_blocks(1, 1).unwrap();
        let added_c = gb_c.process_blocks(&response2).unwrap();
        assert_eq!(added_c, 1);
        assert_eq!(gb_c.height(), 1);

        // All nodes at height 1
        assert_eq!(gb_a.height(), gb_b.height());
        assert_eq!(gb_b.height(), gb_c.height());
    }

    #[test]
    fn test_multi_node_chain_convergence() {
        let config = make_test_config();

        let mut gb_a = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let mut gb_b = GossipBridge::init(&config, peer_id(2), dummy_did(1)).unwrap();
        let conn = gb_a.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb_a.complete_handshake(conn, 9000).unwrap();

        // A creates 3 blocks
        for i in 1..=3 {
            let proposer = gb_a.bridge.next_proposer().unwrap();
            let (block, _) = gb_a.propose_and_gossip(&proposer, 2000 + i * 100, [0xAB; 32]).unwrap();
        }
        assert_eq!(gb_a.height(), 3);

        // B syncs all blocks
        let response = gb_a.respond_get_blocks(1, 10).unwrap();
        let added = gb_b.process_blocks(&response).unwrap();
        assert_eq!(added, 3);
        assert_eq!(gb_b.height(), 3);

        // Both at same height
        assert_eq!(gb_a.height(), gb_b.height());
    }

    #[test]
    fn test_block_relay_to_other_peers() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        for i in 2..=4u8 {
            let c = gb.connect_peer(peer_id(i), dummy_did(i - 1)).unwrap();
            gb.complete_handshake(c, 9000).unwrap();
        }

        let block = gb.bridge.chain.get_block(0).unwrap().clone();
        let sent = gb.gossip_block(&block).unwrap();
        assert_eq!(sent, 3);
    }

    // === Known Blocks === //

    #[test]
    fn test_known_blocks_after_propose() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        assert_eq!(gb.known_blocks.len(), 1); // Genesis

        let p = gb.bridge.next_proposer().unwrap();
        let (_block, _) = gb.propose_and_gossip(&p, 2000, [0xAB; 32]).unwrap();

        assert_eq!(gb.known_blocks.len(), 2); // Genesis + block 1
        assert!(gb.known_blocks.contains_key(&1));
    }

    #[test]
    fn test_known_blocks_after_sync() {
        let config = make_test_config();
        let mut gb_a = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let mut gb_b = GossipBridge::init(&config, peer_id(2), dummy_did(1)).unwrap();

        // A creates 2 blocks
        for i in 1..=2 {
            let p = gb_a.bridge.next_proposer().unwrap();
            gb_a.bridge.propose_block(&p, 2000 + i, [0xAB; 32]).unwrap();
            gb_a.known_blocks.insert(i, gb_a.bridge.chain.get_block(i).unwrap().clone());
        }

        // B syncs
        let response = gb_a.respond_get_blocks(1, 5).unwrap();
        gb_b.process_blocks(&response).unwrap();

        assert_eq!(gb_b.known_blocks.len(), 3); // Genesis + 2 blocks // Genesis + 2 blocks
    }

    // === Error Handling === //

    #[test]
    fn test_peer_not_connected_for_request() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        assert_eq!(gb.request_blocks(999, 1, 10), Err(GossipError::PeerNotConnected));
    }

    #[test]
    fn test_process_blocks_wrong_chain_id() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();

        // Create a block with wrong chain ID
        let mut block = gb.bridge.chain.get_block(0).unwrap().clone();
        block.chain_id = 9999;
        block.height = 1;

        let msg = BlocksMsg {
            blocks: vec![BlockData {
                height: 1,
                hash: block.id,
                data: serialize_bridge_block(&block),
            }],
        };

        assert_eq!(gb.process_blocks(&msg), Err(GossipError::InvalidChainId));
    }

    // === Consensus Integration === //

    #[test]
    fn test_total_stake_available() {
        let config = make_test_config();
        let gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        assert_eq!(gb.total_stake(), 40000);
    }

    #[test]
    fn test_genesis_hash_consistent() {
        let config = make_test_config();
        let gb1 = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let gb2 = GossipBridge::init(&config, peer_id(2), dummy_did(1)).unwrap();

        // Same config → same genesis hash
        assert_eq!(gb1.genesis_hash(), gb2.genesis_hash());
    }

    #[test]
    fn test_pending_announcements_accumulate() {
        let config = make_test_config();
        let mut gb = GossipBridge::init(&config, peer_id(1), dummy_did(0)).unwrap();
        let conn = gb.connect_peer(peer_id(2), dummy_did(1)).unwrap();
        gb.complete_handshake(conn, 9000).unwrap();

        for h in 1..=5 {
            let ann = BlockAnnMsg { block_hash: [h as u8; 32], block_height: h, prev_hash: [0u8; 32] };
            gb.handle_block_ann(conn, &ann).unwrap();
        }

        assert_eq!(gb.pending_sync_count(), 5);
    }
}
