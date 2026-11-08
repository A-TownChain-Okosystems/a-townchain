// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — ATCNet Protocol Handler (K-Sprint 24)
//!
//! Implementiert ATC-01 Core Node Protocol auf dem TCP/IP-Layer.
//! Nutzt die bestehende net.rs (NetworkDevice) und tcpip.rs (TCP/IP) Infrastruktur.
//!
//! Protokoll-Nachrichten:
//!   HANDSHAKE    — Peer-Begrüssung mit DID + Chain-ID
//!   PEER_LIST    — Austausch der bekannten Peers
//!   BLOCK_ANN    — Block-Ankündigung (Hash + Höhe)
//!   TX_BROADCAST — Transaktions-Weiterleitung
//!   PING / PONG  — Keep-Alive
//!   GET_BLOCKS   — Block-Anforderung
//!   BLOCKS       — Block-Antwort
//!   GET_MEMPOOL  — Mempool-Anforderung
//!   MEMPOOL      — Mempool-Antwort
//!
//! Alle Nachrichten sind长度-präfix (4 Bytes LE) + Typ-Byte + Payload.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::sync::atomic::{AtomicU64, Ordering};

/// Chain-ID für das A-TownChain Mainnet
pub const CHAIN_ID: u32 = 658467;

/// ATCNet Protokoll-Version
pub const PROTOCOL_VERSION: u8 = 1;

/// Maximale Nachrichtengrösse (1 MB)
pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024;

/// Peer-ID (32 Bytes — ECDSA komprimiert oder Hash)
pub type PeerId = [u8; 32];

/// Nachrichtentypen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    Handshake = 0x01,
    PeerList = 0x02,
    BlockAnn = 0x03,
    TxBroadcast = 0x04,
    Ping = 0x05,
    Pong = 0x06,
    GetBlocks = 0x07,
    Blocks = 0x08,
    GetMempool = 0x09,
    Mempool = 0x0A,
}

impl MessageType {
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            0x01 => Some(Self::Handshake),
            0x02 => Some(Self::PeerList),
            0x03 => Some(Self::BlockAnn),
            0x04 => Some(Self::TxBroadcast),
            0x05 => Some(Self::Ping),
            0x06 => Some(Self::Pong),
            0x07 => Some(Self::GetBlocks),
            0x08 => Some(Self::Blocks),
            0x09 => Some(Self::GetMempool),
            0x0A => Some(Self::Mempool),
            _ => None,
        }
    }
}

/// Verbindungs-Zustand
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnState {
    /// Verbindung geöffnet, warte auf Handshake
    Connecting,
    /// Handshake gesendet, warte auf Antwort
    Handshaking,
    /// Authentifiziert, bereit für Nachrichten
    Connected,
    /// Verbindung geschlossen
    Disconnected,
}

/// Verbindung zu einem Peer
#[derive(Debug, Clone)]
pub struct PeerConnection {
    pub conn_id: u64,
    pub peer_id: PeerId,
    pub peer_did: String,
    pub state: ConnState,
    pub chain_id: u32,
    pub protocol_version: u8,
    pub last_seen: u64,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
}

/// ATCNet Protokoll-Fehler
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtcNetError {
    /// Unbekannter Nachrichtentyp
    UnknownMessageType,
    /// Nachricht zu gross
    MessageTooLarge,
    /// Ungültige Nachricht (falsches Format)
    InvalidMessage,
    /// Chain-ID stimmt nicht überein
    ChainIdMismatch,
    /// Protokoll-Version nicht kompatibel
    VersionMismatch,
    /// Peer nicht verbunden
    NotConnected,
    /// Peer schon bekannt
    AlreadyConnected,
    /// Verbindung nicht gefunden
    ConnectionNotFound,
    /// Handshake fehlgeschlagen
    HandshakeFailed,
    /// Buffer zu klein
    BufferTooSmall,
}

// === Nachricht-Strukturen === //

/// Handshake-Nachricht
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HandshakeMsg {
    pub protocol_version: u8,
    pub chain_id: u32,
    pub peer_id: PeerId,
    pub peer_did: String,
    pub listen_port: u16,
    pub current_height: u64,
}

/// Block-Ankündigung
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockAnnMsg {
    pub block_hash: [u8; 32],
    pub block_height: u64,
    pub prev_hash: [u8; 32],
}

/// Transaktions-Broadcast
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxBroadcastMsg {
    pub tx_hash: [u8; 32],
    pub tx_data: Vec<u8>,
}

/// Ping-Nachricht
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PingMsg {
    pub timestamp: u64,
    pub nonce: u64,
}

/// Pong-Nachricht
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PongMsg {
    pub timestamp: u64,
    pub nonce: u64,
}

/// Block-Anforderung
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBlocksMsg {
    pub from_height: u64,
    pub max_count: u16,
}

/// Block-Antwort
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlocksMsg {
    pub blocks: Vec<BlockData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockData {
    pub height: u64,
    pub hash: [u8; 32],
    pub data: Vec<u8>,
}

/// Peer-Liste
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerListMsg {
    pub peers: Vec<PeerEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerEntry {
    pub peer_id: PeerId,
    pub address: String,
    pub port: u16,
}

// === Serializer / Deserializer === //

/// Serialisiert eine Handshake-Nachricht
pub fn serialize_handshake(msg: &HandshakeMsg) -> Vec<u8> {
    let did_bytes = msg.peer_did.as_bytes();
    let did_len = did_bytes.len() as u16;
    let mut buf = vec![
        MessageType::Handshake as u8,
        msg.protocol_version,
    ];
    buf.extend_from_slice(&msg.chain_id.to_le_bytes());
    buf.extend_from_slice(&msg.peer_id);
    buf.extend_from_slice(&did_len.to_le_bytes());
    buf.extend_from_slice(did_bytes);
    buf.extend_from_slice(&msg.listen_port.to_le_bytes());
    buf.extend_from_slice(&msg.current_height.to_le_bytes());
    buf
}

/// Deserialisiert eine Handshake-Nachricht
pub fn deserialize_handshake(data: &[u8]) -> Result<HandshakeMsg, AtcNetError> {
    if data.len() < 2 + 4 + 32 + 2 + 2 + 8 {
        return Err(AtcNetError::InvalidMessage);
    }
    let protocol_version = data[0];
    let chain_id = u32::from_le_bytes(data[1..5].try_into().unwrap());
    let peer_id: PeerId = data[5..37].try_into().unwrap();
    let did_len = u16::from_le_bytes(data[37..39].try_into().unwrap()) as usize;
    if data.len() < 39 + did_len + 2 + 8 {
        return Err(AtcNetError::InvalidMessage);
    }
    let peer_did = String::from_utf8(data[39..39 + did_len].to_vec())
        .map_err(|_| AtcNetError::InvalidMessage)?;
    let offset = 39 + did_len;
    let listen_port = u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap());
    let current_height = u64::from_le_bytes(data[offset + 2..offset + 10].try_into().unwrap());

    Ok(HandshakeMsg {
        protocol_version,
        chain_id,
        peer_id,
        peer_did,
        listen_port,
        current_height,
    })
}

/// Serialisiert eine Block-Ankündigung
pub fn serialize_block_ann(msg: &BlockAnnMsg) -> Vec<u8> {
    let mut buf = vec![MessageType::BlockAnn as u8];
    buf.extend_from_slice(&msg.block_hash);
    buf.extend_from_slice(&msg.block_height.to_le_bytes());
    buf.extend_from_slice(&msg.prev_hash);
    buf
}

/// Deserialisiert eine Block-Ankündigung
pub fn deserialize_block_ann(data: &[u8]) -> Result<BlockAnnMsg, AtcNetError> {
    if data.len() != 32 + 8 + 32 {
        return Err(AtcNetError::InvalidMessage);
    }
    Ok(BlockAnnMsg {
        block_hash: data[0..32].try_into().unwrap(),
        block_height: u64::from_le_bytes(data[32..40].try_into().unwrap()),
        prev_hash: data[40..72].try_into().unwrap(),
    })
}

/// Serialisiert einen Ping
pub fn serialize_ping(msg: &PingMsg) -> Vec<u8> {
    let mut buf = vec![MessageType::Ping as u8];
    buf.extend_from_slice(&msg.timestamp.to_le_bytes());
    buf.extend_from_slice(&msg.nonce.to_le_bytes());
    buf
}

/// Deserialisiert einen Ping
pub fn deserialize_ping(data: &[u8]) -> Result<PingMsg, AtcNetError> {
    if data.len() != 16 {
        return Err(AtcNetError::InvalidMessage);
    }
    Ok(PingMsg {
        timestamp: u64::from_le_bytes(data[0..8].try_into().unwrap()),
        nonce: u64::from_le_bytes(data[8..16].try_into().unwrap()),
    })
}

/// Serialisiert einen Pong
pub fn serialize_pong(msg: &PongMsg) -> Vec<u8> {
    let mut buf = vec![MessageType::Pong as u8];
    buf.extend_from_slice(&msg.timestamp.to_le_bytes());
    buf.extend_from_slice(&msg.nonce.to_le_bytes());
    buf
}

/// Deserialisiert einen Pong
pub fn deserialize_pong(data: &[u8]) -> Result<PongMsg, AtcNetError> {
    if data.len() != 16 {
        return Err(AtcNetError::InvalidMessage);
    }
    Ok(PongMsg {
        timestamp: u64::from_le_bytes(data[0..8].try_into().unwrap()),
        nonce: u64::from_le_bytes(data[8..16].try_into().unwrap()),
    })
}

/// Serialisiert eine Tx-Broadcast
pub fn serialize_tx_broadcast(msg: &TxBroadcastMsg) -> Vec<u8> {
    let mut buf = vec![MessageType::TxBroadcast as u8];
    buf.extend_from_slice(&msg.tx_hash);
    buf.extend_from_slice(&(msg.tx_data.len() as u32).to_le_bytes());
    buf.extend_from_slice(&msg.tx_data);
    buf
}

/// Deserialisiert eine Tx-Broadcast
pub fn deserialize_tx_broadcast(data: &[u8]) -> Result<TxBroadcastMsg, AtcNetError> {
    if data.len() < 32 + 4 {
        return Err(AtcNetError::InvalidMessage);
    }
    let tx_hash: [u8; 32] = data[0..32].try_into().unwrap();
    let tx_len = u32::from_le_bytes(data[32..36].try_into().unwrap()) as usize;
    if data.len() != 36 + tx_len || tx_len > MAX_MESSAGE_SIZE {
        return Err(AtcNetError::InvalidMessage);
    }
    Ok(TxBroadcastMsg {
        tx_hash,
        tx_data: data[36..36 + tx_len].to_vec(),
    })
}

/// Serialisiert eine Peer-Liste
pub fn serialize_peer_list(msg: &PeerListMsg) -> Vec<u8> {
    let mut buf = vec![MessageType::PeerList as u8];
    buf.extend_from_slice(&(msg.peers.len() as u16).to_le_bytes());
    for peer in &msg.peers {
        buf.extend_from_slice(&peer.peer_id);
        let addr_bytes = peer.address.as_bytes();
        buf.extend_from_slice(&(addr_bytes.len() as u16).to_le_bytes());
        buf.extend_from_slice(addr_bytes);
        buf.extend_from_slice(&peer.port.to_le_bytes());
    }
    buf
}

/// Deserialisiert eine Peer-Liste
pub fn deserialize_peer_list(data: &[u8]) -> Result<PeerListMsg, AtcNetError> {
    if data.len() < 2 {
        return Err(AtcNetError::InvalidMessage);
    }
    let count = u16::from_le_bytes(data[0..2].try_into().unwrap()) as usize;
    let mut offset = 2;
    let mut peers = Vec::new();
    for _ in 0..count {
        if offset + 32 + 2 > data.len() {
            return Err(AtcNetError::InvalidMessage);
        }
        let peer_id: PeerId = data[offset..offset + 32].try_into().unwrap();
        offset += 32;
        let addr_len = u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap()) as usize;
        offset += 2;
        if offset + addr_len + 2 > data.len() {
            return Err(AtcNetError::InvalidMessage);
        }
        let address = String::from_utf8(data[offset..offset + addr_len].to_vec())
            .map_err(|_| AtcNetError::InvalidMessage)?;
        offset += addr_len;
        let port = u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap());
        offset += 2;
        peers.push(PeerEntry { peer_id, address, port });
    }
    Ok(PeerListMsg { peers })
}

/// Serialisiert eine GetBlocks-Anforderung
pub fn serialize_get_blocks(msg: &GetBlocksMsg) -> Vec<u8> {
    let mut buf = vec![MessageType::GetBlocks as u8];
    buf.extend_from_slice(&msg.from_height.to_le_bytes());
    buf.extend_from_slice(&msg.max_count.to_le_bytes());
    buf
}

/// Deserialisiert eine GetBlocks-Anforderung
pub fn deserialize_get_blocks(data: &[u8]) -> Result<GetBlocksMsg, AtcNetError> {
    if data.len() != 10 {
        return Err(AtcNetError::InvalidMessage);
    }
    Ok(GetBlocksMsg {
        from_height: u64::from_le_bytes(data[0..8].try_into().unwrap()),
        max_count: u16::from_le_bytes(data[8..10].try_into().unwrap()),
    })
}

// === ATCNet Protocol Handler === //

/// Der Protokoll-Handler verwaltet alle Peer-Verbindungen und
/// behandelt ein- und ausgehende Nachrichten.
pub struct AtcNetHandler {
    /// Alle aktiven Verbindungen
    connections: BTreeMap<u64, PeerConnection>,
    /// Bekannte Peers (Peer-ID -> letzte bekannte Adresse)
    known_peers: BTreeMap<PeerId, String>,
    /// Eigene Peer-ID
    self_peer_id: PeerId,
    /// Eigene DID
    self_did: String,
    /// Nächste Verbindungs-ID
    next_conn_id: AtomicU64,
    /// Eigene Blockhöhe
    current_height: AtomicU64,
    /// Anzahl gesendeter Nachrichten
    messages_sent: AtomicU64,
    /// Anzahl empfangener Nachrichten
    messages_recv: AtomicU64,
}

impl AtcNetHandler {
    pub fn new(self_peer_id: PeerId, self_did: String) -> Self {
        Self {
            connections: BTreeMap::new(),
            known_peers: BTreeMap::new(),
            self_peer_id,
            self_did,
            next_conn_id: AtomicU64::new(1),
            current_height: AtomicU64::new(0),
            messages_sent: AtomicU64::new(0),
            messages_recv: AtomicU64::new(0),
        }
    }

    /// Erstellt eine neue Verbindung zu einem Peer
    pub fn connect(&mut self, peer_id: PeerId, peer_did: String) -> Result<u64, AtcNetError> {
        if self.connections.values().any(|c| c.peer_id == peer_id && c.state == ConnState::Connected) {
            return Err(AtcNetError::AlreadyConnected);
        }
        let conn_id = self.next_conn_id.fetch_add(1, Ordering::SeqCst);
        let conn = PeerConnection {
            conn_id,
            peer_id,
            peer_did,
            state: ConnState::Connecting,
            chain_id: CHAIN_ID,
            protocol_version: PROTOCOL_VERSION,
            last_seen: 0,
            bytes_sent: 0,
            bytes_recv: 0,
        };
        self.connections.insert(conn_id, conn);
        Ok(conn_id)
    }

    /// Sendet einen Handshake auf einer Verbindung
    pub fn send_handshake(&mut self, conn_id: u64, listen_port: u16) -> Result<Vec<u8>, AtcNetError> {
        let conn = self.connections.get(&conn_id).ok_or(AtcNetError::ConnectionNotFound)?;
        if conn.state != ConnState::Connecting {
            return Err(AtcNetError::HandshakeFailed);
        }
        // State -> Handshaking
        if let Some(c) = self.connections.get_mut(&conn_id) {
            c.state = ConnState::Handshaking;
        }
        let msg = HandshakeMsg {
            protocol_version: PROTOCOL_VERSION,
            chain_id: CHAIN_ID,
            peer_id: self.self_peer_id,
            peer_did: self.self_did.clone(),
            listen_port,
            current_height: self.current_height.load(Ordering::SeqCst),
        };
        let data = serialize_handshake(&msg);
        self.messages_sent.fetch_add(1, Ordering::SeqCst);
        if let Some(c) = self.connections.get_mut(&conn_id) {
            c.bytes_sent += data.len() as u64;
        }
        Ok(data)
    }

    /// Verarbeitet eine eingehende Nachricht
    pub fn handle_message(&mut self, conn_id: u64, data: &[u8]) -> Result<Option<Vec<u8>>, AtcNetError> {
        if data.is_empty() {
            return Err(AtcNetError::InvalidMessage);
        }
        if data.len() > MAX_MESSAGE_SIZE {
            return Err(AtcNetError::MessageTooLarge);
        }

        let msg_type = MessageType::from_byte(data[0]).ok_or(AtcNetError::UnknownMessageType)?;
        let payload = &data[1..];

        self.messages_recv.fetch_add(1, Ordering::SeqCst);
        if let Some(c) = self.connections.get_mut(&conn_id) {
            c.bytes_recv += data.len() as u64;
            c.last_seen = self.current_height.load(Ordering::SeqCst);
        }

        match msg_type {
            MessageType::Handshake => self.handle_handshake(conn_id, payload),
            MessageType::Ping => self.handle_ping(payload),
            MessageType::Pong => self.handle_pong(conn_id, payload),
            MessageType::BlockAnn => self.handle_block_ann(conn_id, payload),
            MessageType::TxBroadcast => self.handle_tx_broadcast(conn_id, payload),
            MessageType::GetBlocks => self.handle_get_blocks(conn_id, payload),
            MessageType::PeerList => self.handle_peer_list(conn_id, payload),
            MessageType::GetMempool => Ok(None),
            MessageType::Mempool => Ok(None),
            MessageType::Blocks => Ok(None),
        }
    }

    fn handle_handshake(&mut self, conn_id: u64, payload: &[u8]) -> Result<Option<Vec<u8>>, AtcNetError> {
        let msg = deserialize_handshake(payload)?;

        // Chain-ID prüfen
        if msg.chain_id != CHAIN_ID {
            if let Some(c) = self.connections.get_mut(&conn_id) {
                c.state = ConnState::Disconnected;
            }
            return Err(AtcNetError::ChainIdMismatch);
        }

        // Protokoll-Version prüfen
        if msg.protocol_version != PROTOCOL_VERSION {
            if let Some(c) = self.connections.get_mut(&conn_id) {
                c.state = ConnState::Disconnected;
            }
            return Err(AtcNetError::VersionMismatch);
        }

        // Verbindung aktualisieren
        if let Some(c) = self.connections.get_mut(&conn_id) {
            c.peer_id = msg.peer_id;
            c.peer_did = msg.peer_did.clone();
            c.chain_id = msg.chain_id;
            c.protocol_version = msg.protocol_version;
            c.state = ConnState::Connected;
            c.last_seen = msg.current_height;
        }

        // Peer bekannt machen
        self.known_peers.insert(msg.peer_id, format!("peer:{}", conn_id));

        // Antwort: eigener Handshake
        let response = HandshakeMsg {
            protocol_version: PROTOCOL_VERSION,
            chain_id: CHAIN_ID,
            peer_id: self.self_peer_id,
            peer_did: self.self_did.clone(),
            listen_port: 9000,
            current_height: self.current_height.load(Ordering::SeqCst),
        };
        let resp_data = serialize_handshake(&response);
        self.messages_sent.fetch_add(1, Ordering::SeqCst);
        Ok(Some(resp_data))
    }

    fn handle_ping(&self, payload: &[u8]) -> Result<Option<Vec<u8>>, AtcNetError> {
        let ping = deserialize_ping(payload)?;
        let pong = PongMsg {
            timestamp: ping.timestamp,
            nonce: ping.nonce,
        };
        Ok(Some(serialize_pong(&pong)))
    }

    fn handle_pong(&self, _conn_id: u64, _payload: &[u8]) -> Result<Option<Vec<u8>>, AtcNetError> {
        // Pong bestätigt Connectivity — keine Antwort nötig
        Ok(None)
    }

    fn handle_block_ann(&mut self, conn_id: u64, payload: &[u8]) -> Result<Option<Vec<u8>>, AtcNetError> {
        let msg = deserialize_block_ann(payload)?;
        // Wenn der Block höher als unsere aktuelle Höhe ist, anfordern
        let our_height = self.current_height.load(Ordering::SeqCst);
        if msg.block_height > our_height {
            let request = GetBlocksMsg {
                from_height: our_height + 1,
                max_count: 100,
            };
            return Ok(Some(serialize_get_blocks(&request)));
        }
        Ok(None)
    }

    fn handle_tx_broadcast(&self, _conn_id: u64, payload: &[u8]) -> Result<Option<Vec<u8>>, AtcNetError> {
        let _msg = deserialize_tx_broadcast(payload)?;
        // Tx an Mempool weiterleiten (hier: nur akzeptieren)
        Ok(None)
    }

    fn handle_get_blocks(&self, _conn_id: u64, payload: &[u8]) -> Result<Option<Vec<u8>>, AtcNetError> {
        let request = deserialize_get_blocks(payload)?;
        // Dummy: leere Block-Liste (in echt: aus blockchain.rs holen)
        let _ = request;
        Ok(None)
    }

    fn handle_peer_list(&mut self, _conn_id: u64, payload: &[u8]) -> Result<Option<Vec<u8>>, AtcNetError> {
        let msg = deserialize_peer_list(payload)?;
        for peer in &msg.peers {
            self.known_peers.insert(peer.peer_id, format!("{}:{}", peer.address, peer.port));
        }
        Ok(None)
    }

    /// Sendet einen Ping
    pub fn send_ping(&mut self, conn_id: u64, nonce: u64) -> Result<Vec<u8>, AtcNetError> {
        let conn = self.connections.get(&conn_id).ok_or(AtcNetError::ConnectionNotFound)?;
        if conn.state != ConnState::Connected {
            return Err(AtcNetError::NotConnected);
        }
        let ping = PingMsg {
            timestamp: self.current_height.load(Ordering::SeqCst),
            nonce,
        };
        let data = serialize_ping(&ping);
        self.messages_sent.fetch_add(1, Ordering::SeqCst);
        if let Some(c) = self.connections.get_mut(&conn_id) {
            c.bytes_sent += data.len() as u64;
        }
        Ok(data)
    }

    /// Sendet eine Block-Ankündigung
    pub fn send_block_ann(&mut self, conn_id: u64, block_hash: [u8; 32], block_height: u64, prev_hash: [u8; 32]) -> Result<Vec<u8>, AtcNetError> {
        let conn = self.connections.get(&conn_id).ok_or(AtcNetError::ConnectionNotFound)?;
        if conn.state != ConnState::Connected {
            return Err(AtcNetError::NotConnected);
        }
        self.current_height.store(block_height, Ordering::SeqCst);
        let msg = BlockAnnMsg { block_hash, block_height, prev_hash };
        let data = serialize_block_ann(&msg);
        self.messages_sent.fetch_add(1, Ordering::SeqCst);
        if let Some(c) = self.connections.get_mut(&conn_id) {
            c.bytes_sent += data.len() as u64;
        }
        Ok(data)
    }

    /// Sendet eine Tx-Broadcast an alle verbundenen Peers (Gossip)
    pub fn gossip_tx(&mut self, tx_hash: [u8; 32], tx_data: Vec<u8>) -> Vec<(u64, Vec<u8>)> {
        let msg = TxBroadcastMsg { tx_hash, tx_data };
        let serialized = serialize_tx_broadcast(&msg);
        let mut results = Vec::new();
        let connected_ids: Vec<u64> = self.connections.iter()
            .filter(|(_, c)| c.state == ConnState::Connected)
            .map(|(id, _)| *id)
            .collect();
        for conn_id in connected_ids {
            self.messages_sent.fetch_add(1, Ordering::SeqCst);
            if let Some(c) = self.connections.get_mut(&conn_id) {
                c.bytes_sent += serialized.len() as u64;
            }
            results.push((conn_id, serialized.clone()));
        }
        results
    }

    /// Trennt eine Verbindung
    pub fn disconnect(&mut self, conn_id: u64) -> bool {
        if let Some(c) = self.connections.get_mut(&conn_id) {
            c.state = ConnState::Disconnected;
            true
        } else {
            false
        }
    }

    /// Setzt die aktuelle Blockhöhe
    pub fn set_height(&self, height: u64) {
        self.current_height.store(height, Ordering::SeqCst);
    }

    /// Anzahl aktiver Verbindungen
    pub fn connection_count(&self) -> usize {
        self.connections.values().filter(|c| c.state == ConnState::Connected).count()
    }

    /// Alle Verbindungs-IDs
    pub fn connection_ids(&self) -> Vec<u64> {
        self.connections.keys().cloned().collect()
    }

    /// Statistik
    pub fn stats(&self) -> AtcNetStats {
        AtcNetStats {
            total_connections: self.connections.len(),
            connected: self.connection_count(),
            known_peers: self.known_peers.len(),
            messages_sent: self.messages_sent.load(Ordering::SeqCst),
            messages_recv: self.messages_recv.load(Ordering::SeqCst),
            current_height: self.current_height.load(Ordering::SeqCst),
        }
    }
}

/// ATCNet Statistik
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtcNetStats {
    pub total_connections: usize,
    pub connected: usize,
    pub known_peers: usize,
    pub messages_sent: u64,
    pub messages_recv: u64,
    pub current_height: u64,
}

// === ats1000 NetworkStack Trait Implementation === //

use crate::ats1000::NetworkStack;

impl NetworkStack for AtcNetHandler {
    fn connect(&mut self, peer_node_id: &[u8; 32]) -> Option<u64> {
        let peer_id: PeerId = *peer_node_id;
        let did = format!("did:shivacore:{}", hex_encode(&peer_id[..8]));
        self.connect(peer_id, did).ok()
    }

    fn send(&mut self, conn: u64, msg: &[u8]) -> bool {
        self.handle_message(conn, msg).is_ok()
    }

    fn recv(&mut self, conn: u64, buf: &mut [u8]) -> u64 {
        // In echter Implementierung: aus einem Empfangsbuffer lesen
        // Hier: simuliert — gibt 0 zurück (keine Daten)
        let _ = (conn, buf);
        0
    }
}

/// Hex-Encoder (minimal, kein externes Crate)
fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_peer_id(n: u8) -> PeerId {
        let mut id = [0u8; 32];
        id[0] = n;
        id
    }

    // === Serialisierung-Tests === //

    #[test]
    fn test_handshake_roundtrip() {
        let msg = HandshakeMsg {
            protocol_version: 1,
            chain_id: CHAIN_ID,
            peer_id: dummy_peer_id(1),
            peer_did: "did:shivacore:abcd".to_string(),
            listen_port: 9000,
            current_height: 42,
        };
        let serialized = serialize_handshake(&msg);
        let deserialized = deserialize_handshake(&serialized[1..]).unwrap();
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_block_ann_roundtrip() {
        let msg = BlockAnnMsg {
            block_hash: [0xAA; 32],
            block_height: 100,
            prev_hash: [0xBB; 32],
        };
        let serialized = serialize_block_ann(&msg);
        let deserialized = deserialize_block_ann(&serialized[1..]).unwrap();
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_ping_pong_roundtrip() {
        let ping = PingMsg { timestamp: 12345, nonce: 999 };
        let serialized = serialize_ping(&ping);
        let deserialized = deserialize_ping(&serialized[1..]).unwrap();
        assert_eq!(ping, deserialized);

        let pong = PongMsg { timestamp: 12345, nonce: 999 };
        let serialized = serialize_pong(&pong);
        let deserialized = deserialize_pong(&serialized[1..]).unwrap();
        assert_eq!(pong, deserialized);
    }

    #[test]
    fn test_tx_broadcast_roundtrip() {
        let msg = TxBroadcastMsg {
            tx_hash: [0xCC; 32],
            tx_data: vec![1, 2, 3, 4, 5],
        };
        let serialized = serialize_tx_broadcast(&msg);
        let deserialized = deserialize_tx_broadcast(&serialized[1..]).unwrap();
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_get_blocks_roundtrip() {
        let msg = GetBlocksMsg { from_height: 100, max_count: 50 };
        let serialized = serialize_get_blocks(&msg);
        let deserialized = deserialize_get_blocks(&serialized[1..]).unwrap();
        assert_eq!(msg, deserialized);
    }

    #[test]
    fn test_peer_list_roundtrip() {
        let msg = PeerListMsg {
            peers: vec![
                PeerEntry { peer_id: dummy_peer_id(1), address: "10.0.0.1".to_string(), port: 9000 },
                PeerEntry { peer_id: dummy_peer_id(2), address: "10.0.0.2".to_string(), port: 9000 },
            ],
        };
        let serialized = serialize_peer_list(&msg);
        let deserialized = deserialize_peer_list(&serialized[1..]).unwrap();
        assert_eq!(msg, deserialized);
    }

    // === Handler-Tests === //

    #[test]
    fn test_handler_connect() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        assert!(conn > 0);
        assert_eq!(h.connection_count(), 0); // Not connected yet (Connecting state)
    }

    #[test]
    fn test_handler_connect_duplicate_rejected() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        // Second connect with same peer_id should fail IF first is Connected
        // But first is still Connecting, so second succeeds
        let conn2 = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string());
        assert!(conn2.is_ok()); // OK because first is in Connecting, not Connected
    }

    #[test]
    fn test_handler_handshake() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();

        // Send handshake
        let hs_data = h.send_handshake(conn, 9000).unwrap();
        assert_eq!(hs_data[0], MessageType::Handshake as u8);

        // Simulate peer handshake response
        let peer_hs = HandshakeMsg {
            protocol_version: PROTOCOL_VERSION,
            chain_id: CHAIN_ID,
            peer_id: dummy_peer_id(1),
            peer_did: "did:shivacore:peer1".to_string(),
            listen_port: 9001,
            current_height: 10,
        };
        let peer_data = serialize_handshake(&peer_hs);
        let response = h.handle_message(conn, &peer_data).unwrap();
        assert!(response.is_some()); // Should respond with our handshake

        // Now connected
        assert_eq!(h.connection_count(), 1);
    }

    #[test]
    fn test_handler_handshake_wrong_chain() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();

        let peer_hs = HandshakeMsg {
            protocol_version: PROTOCOL_VERSION,
            chain_id: 9999, // Wrong chain
            peer_id: dummy_peer_id(1),
            peer_did: "did:shivacore:peer1".to_string(),
            listen_port: 9001,
            current_height: 10,
        };
        let peer_data = serialize_handshake(&peer_hs);
        let result = h.handle_message(conn, &peer_data);
        assert_eq!(result, Err(AtcNetError::ChainIdMismatch));
        assert_eq!(h.connection_count(), 0); // Disconnected
    }

    #[test]
    fn test_handler_handshake_wrong_version() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();

        let peer_hs = HandshakeMsg {
            protocol_version: 99, // Wrong version
            chain_id: CHAIN_ID,
            peer_id: dummy_peer_id(1),
            peer_did: "did:shivacore:peer1".to_string(),
            listen_port: 9001,
            current_height: 10,
        };
        let peer_data = serialize_handshake(&peer_hs);
        let result = h.handle_message(conn, &peer_data);
        assert_eq!(result, Err(AtcNetError::VersionMismatch));
    }

    #[test]
    fn test_handler_ping_pong() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();

        // Manually set to Connected
        if let Some(c) = h.connections.get_mut(&conn) {
            c.state = ConnState::Connected;
        }

        // Send ping
        let ping_data = h.send_ping(conn, 12345).unwrap();
        assert_eq!(ping_data[0], MessageType::Ping as u8);

        // Handle incoming ping -> should get pong
        let response = h.handle_message(conn, &ping_data).unwrap();
        assert!(response.is_some());
        assert_eq!(response.unwrap()[0], MessageType::Pong as u8);
    }

    #[test]
    fn test_handler_block_ann_triggers_get_blocks() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        if let Some(c) = h.connections.get_mut(&conn) {
            c.state = ConnState::Connected;
        }

        // Block at height 100 (we're at 0)
        let block = BlockAnnMsg {
            block_hash: [0xFF; 32],
            block_height: 100,
            prev_hash: [0xEE; 32],
        };
        let block_data = serialize_block_ann(&block);
        let response = h.handle_message(conn, &block_data).unwrap();
        assert!(response.is_some()); // Should request blocks
        assert_eq!(response.unwrap()[0], MessageType::GetBlocks as u8);
    }

    #[test]
    fn test_handler_block_ann_no_request_if_synced() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        if let Some(c) = h.connections.get_mut(&conn) {
            c.state = ConnState::Connected;
        }
        h.set_height(200);

        // Block at height 100 (we're at 200)
        let block = BlockAnnMsg {
            block_hash: [0xFF; 32],
            block_height: 100,
            prev_hash: [0xEE; 32],
        };
        let block_data = serialize_block_ann(&block);
        let response = h.handle_message(conn, &block_data).unwrap();
        assert!(response.is_none()); // No request needed
    }

    #[test]
    fn test_handler_gossip_tx() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());

        // Connect 3 peers
        for i in 1..=3 {
            let conn = h.connect(dummy_peer_id(i), format!("did:shivacore:peer{}", i)).unwrap();
            if let Some(c) = h.connections.get_mut(&conn) {
                c.state = ConnState::Connected;
            }
        }
        assert_eq!(h.connection_count(), 3);

        let results = h.gossip_tx([0xDD; 32], vec![1, 2, 3]);
        assert_eq!(results.len(), 3); // Sent to all 3
        for (_, data) in &results {
            assert_eq!(data[0], MessageType::TxBroadcast as u8);
        }
    }

    #[test]
    fn test_handler_disconnect() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        assert!(h.disconnect(conn));
        // Disconnected state
        assert_eq!(h.connection_count(), 0);
    }

    #[test]
    fn test_handler_stats() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());

        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        h.send_handshake(conn, 9000).unwrap();

        let stats = h.stats();
        assert_eq!(stats.total_connections, 1);
        assert_eq!(stats.connected, 0); // Still handshaking
        assert_eq!(stats.messages_sent, 1);
    }

    #[test]
    fn test_handler_peer_list_updates_known_peers() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        if let Some(c) = h.connections.get_mut(&conn) {
            c.state = ConnState::Connected;
        }

        let peer_list = PeerListMsg {
            peers: vec![
                PeerEntry { peer_id: dummy_peer_id(10), address: "10.0.0.10".to_string(), port: 9000 },
                PeerEntry { peer_id: dummy_peer_id(11), address: "10.0.0.11".to_string(), port: 9000 },
            ],
        };
        let data = serialize_peer_list(&peer_list);
        h.handle_message(conn, &data).unwrap();

        let stats = h.stats();
        assert_eq!(stats.known_peers, 2); // Two new peers learned
    }

    #[test]
    fn test_unknown_message_type() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        let result = h.handle_message(conn, &[0xFF, 0x00]); // Unknown type
        assert_eq!(result, Err(AtcNetError::UnknownMessageType));
    }

    #[test]
    fn test_message_too_large() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        let large = vec![0u8; MAX_MESSAGE_SIZE + 1];
        let result = h.handle_message(conn, &large);
        assert_eq!(result, Err(AtcNetError::MessageTooLarge));
    }

    #[test]
    fn test_empty_message_rejected() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        let result = h.handle_message(conn, &[]);
        assert_eq!(result, Err(AtcNetError::InvalidMessage));
    }

    #[test]
    fn test_send_to_disconnected_rejected() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let conn = h.connect(dummy_peer_id(1), "did:shivacore:peer1".to_string()).unwrap();
        // Still Connecting, not Connected
        let result = h.send_ping(conn, 1);
        assert_eq!(result, Err(AtcNetError::NotConnected));
    }

    #[test]
    fn test_send_to_unknown_connection() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let result = h.send_ping(999, 1);
        assert_eq!(result, Err(AtcNetError::ConnectionNotFound));
    }

    #[test]
    fn test_connection_ids() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let c1 = h.connect(dummy_peer_id(1), "d1".to_string()).unwrap();
        let c2 = h.connect(dummy_peer_id(2), "d2".to_string()).unwrap();
        let ids = h.connection_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&c1));
        assert!(ids.contains(&c2));
    }

    #[test]
    fn test_message_type_from_byte() {
        assert_eq!(MessageType::from_byte(0x01), Some(MessageType::Handshake));
        assert_eq!(MessageType::from_byte(0x05), Some(MessageType::Ping));
        assert_eq!(MessageType::from_byte(0x0A), Some(MessageType::Mempool));
        assert_eq!(MessageType::from_byte(0xFF), None);
    }

    #[test]
    fn test_ats1000_network_stack_trait() {
        let mut h = AtcNetHandler::new(dummy_peer_id(0), "did:shivacore:self".to_string());
        let peer_id = dummy_peer_id(1);
        let conn = NetworkStack::connect(&mut h, &peer_id).unwrap();
        assert!(conn > 0);
    }

    #[test]
    fn test_invalid_handshake_too_short() {
        let result = deserialize_handshake(&[1, 2]); // Way too short
        assert_eq!(result, Err(AtcNetError::InvalidMessage));
    }

    #[test]
    fn test_invalid_block_ann_wrong_length() {
        let result = deserialize_block_ann(&[0; 50]); // Wrong length
        assert_eq!(result, Err(AtcNetError::InvalidMessage));
    }

    #[test]
    fn test_invalid_ping_wrong_length() {
        let result = deserialize_ping(&[0; 10]); // Wrong length
        assert_eq!(result, Err(AtcNetError::InvalidMessage));
    }

    #[test]
    fn test_tx_broadcast_too_large() {
        let mut data = vec![0xCC; 32];
        data.extend_from_slice(&(MAX_MESSAGE_SIZE as u32 + 1).to_le_bytes());
        data.extend_from_slice(&vec![0; 100]);
        let result = deserialize_tx_broadcast(&data);
        assert_eq!(result, Err(AtcNetError::InvalidMessage));
    }

    #[test]
    fn test_peer_list_empty() {
        let msg = PeerListMsg { peers: vec![] };
        let serialized = serialize_peer_list(&msg);
        let deserialized = deserialize_peer_list(&serialized[1..]).unwrap();
        assert_eq!(deserialized.peers.len(), 0);
    }

    #[test]
    fn test_protocol_constants() {
        assert_eq!(CHAIN_ID, 658467);
        assert_eq!(PROTOCOL_VERSION, 1);
        assert_eq!(MAX_MESSAGE_SIZE, 1024 * 1024);
    }
}
