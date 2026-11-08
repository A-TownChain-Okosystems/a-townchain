// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ─────────────────────────────────────────────────────────────────────────
// K-Sprint 14 — P2P-Consensus Foundation
// Kernel Layer | Chain-ID 658467
// Peer-Tabelle, Gossip-Protocol, P2P-Handshake (DID-basiert),
// Message-Types (Block/Tx/Vote/Ping/Pong), Chain-ID-658467 Validation.
// Baut auf K13 (TCP/IP) und K6 (DID) auf.
// ─────────────────────────────────────────────────────────────────────────

use alloc::vec;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use spin::Mutex;

use crate::tcpip::{Ipv4Packet, UdpPacket, TcpSegment, SocketManager, SocketId, TcpState, IP_PROTO_UDP, IP_PROTO_TCP};
use crate::net::{Ipv4Address, MacAddress, NetworkStack, NetworkError, ETH_TYPE_IPV4};

// ─── Chain-ID ───────────────────────────────────────────────────────────────

pub const CHAIN_ID: u32 = 658467;

// ─── P2P-Message-Types ──────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    Ping = 1,
    Pong = 2,
    Handshake = 3,
    HandshakeAck = 4,
    BlockAnnounce = 5,
    TxAnnounce = 6,
    Vote = 7,
    PeerList = 8,
    Bye = 9,
}

impl MessageType {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            1 => Some(MessageType::Ping),
            2 => Some(MessageType::Pong),
            3 => Some(MessageType::Handshake),
            4 => Some(MessageType::HandshakeAck),
            5 => Some(MessageType::BlockAnnounce),
            6 => Some(MessageType::TxAnnounce),
            7 => Some(MessageType::Vote),
            8 => Some(MessageType::PeerList),
            9 => Some(MessageType::Bye),
            _ => None,
        }
    }
}

// ─── P2P-Message ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct P2pMessage {
    pub msg_type: MessageType,
    pub chain_id: u32,
    pub sender_did: String,
    pub timestamp: u64,
    pub payload: Vec<u8>,
}

impl P2pMessage {
    pub fn new(msg_type: MessageType, sender_did: String, timestamp: u64, payload: Vec<u8>) -> Self {
        P2pMessage {
            msg_type,
            chain_id: CHAIN_ID,
            sender_did,
            timestamp,
            payload,
        }
    }

    /// Serialisiert: type[1] + chain_id[4] + did_len[2] + did + timestamp[8] + payload
    pub fn to_bytes(&self) -> Vec<u8> {
        let did_bytes = self.sender_did.as_bytes();
        let mut buf = Vec::with_capacity(1 + 4 + 2 + did_bytes.len() + 8 + self.payload.len());
        buf.push(self.msg_type as u8);
        buf.extend_from_slice(&self.chain_id.to_be_bytes());
        buf.extend_from_slice(&(did_bytes.len() as u16).to_be_bytes());
        buf.extend_from_slice(did_bytes);
        buf.extend_from_slice(&self.timestamp.to_be_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, P2pError> {
        if data.len() < 15 { return Err(P2pError::MessageTooShort); }
        let msg_type = MessageType::from_u8(data[0]).ok_or(P2pError::UnknownMessageType(data[0]))?;
        let chain_id = u32::from_be_bytes([data[1], data[2], data[3], data[4]]);
        if chain_id != CHAIN_ID { return Err(P2pError::WrongChainId(chain_id)); }
        let did_len = u16::from_be_bytes([data[5], data[6]]) as usize;
        if data.len() < 7 + did_len + 8 { return Err(P2pError::MessageTooShort); }
        let sender_did = String::from_utf8_lossy(&data[7..7 + did_len]).to_string();
        let timestamp = u64::from_be_bytes([
            data[7 + did_len], data[8 + did_len], data[9 + did_len], data[10 + did_len],
            data[11 + did_len], data[12 + did_len], data[13 + did_len], data[14 + did_len],
        ]);
        let payload = data[15 + did_len..].to_vec();
        Ok(P2pMessage { msg_type, chain_id, sender_did, timestamp, payload })
    }
}

// ─── P2P-Error ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum P2pError {
    MessageTooShort,
    UnknownMessageType(u8),
    WrongChainId(u32),
    PeerNotFound,
    PeerAlreadyConnected,
    HandshakeFailed(String),
    NotConnected,
    InvalidDID,
    BroadcastFailed,
}

// ─── Peer-Status ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeerStatus {
    Disconnected,
    Connecting,
    Connected,
    Banned,
}

// ─── Peer ──────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct Peer {
    pub id: u64,
    pub ip: Ipv4Address,
    pub port: u16,
    pub did: Option<String>,
    pub status: PeerStatus,
    pub last_seen: u64,
    pub bytes_sent: u64,
    pub bytes_recv: u64,
    pub messages_sent: u64,
    pub messages_recv: u64,
}

impl Peer {
    pub fn new(id: u64, ip: Ipv4Address, port: u16) -> Self {
        Peer {
            id, ip, port, did: None,
            status: PeerStatus::Disconnected,
            last_seen: 0, bytes_sent: 0, bytes_recv: 0,
            messages_sent: 0, messages_recv: 0,
        }
    }
}

// ─── Peer-Table ────────────────────────────────────────────────────────────────

pub struct PeerTable {
    peers: Mutex<BTreeMap<u64, Peer>>,
    by_ip_port: Mutex<BTreeMap<(Ipv4Address, u16), u64>>,
    next_id: Mutex<u64>,
    our_did: String,
    max_peers: usize,
}

impl PeerTable {
    pub fn new(our_did: String, max_peers: usize) -> Self {
        PeerTable {
            peers: Mutex::new(BTreeMap::new()),
            by_ip_port: Mutex::new(BTreeMap::new()),
            next_id: Mutex::new(1),
            our_did,
            max_peers,
        }
    }

    pub fn add_peer(&self, ip: Ipv4Address, port: u16) -> Result<u64, P2pError> {
        // Check if already known
        {
            let by_ip = self.by_ip_port.lock();
            if by_ip.contains_key(&(ip, port)) {
                return Err(P2pError::PeerAlreadyConnected);
            }
        }
        {
            let peers = self.peers.lock();
            if peers.len() >= self.max_peers {
                return Err(P2pError::PeerAlreadyConnected); // max reached
            }
        }

        let id = {
            let mut next = self.next_id.lock();
            let v = *next; *next += 1; v
        };

        let peer = Peer::new(id, ip, port);
        self.peers.lock().insert(id, peer);
        self.by_ip_port.lock().insert((ip, port), id);
        Ok(id)
    }

    pub fn remove_peer(&self, id: u64) -> bool {
        let mut peers = self.peers.lock();
        if let Some(peer) = peers.remove(&id) {
            self.by_ip_port.lock().remove(&(peer.ip, peer.port));
            true
        } else {
            false
        }
    }

    pub fn get_peer(&self, id: u64) -> Option<Peer> {
        self.peers.lock().get(&id).cloned()
    }

    pub fn find_by_addr(&self, ip: Ipv4Address, port: u16) -> Option<Peer> {
        let by_ip = self.by_ip_port.lock();
        by_ip.get(&(ip, port)).and_then(|&id| self.peers.lock().get(&id).cloned())
    }

    pub fn set_status(&self, id: u64, status: PeerStatus) {
        if let Some(peer) = self.peers.lock().get_mut(&id) {
            peer.status = status;
        }
    }

    pub fn set_did(&self, id: u64, did: String) {
        if let Some(peer) = self.peers.lock().get_mut(&id) {
            peer.did = Some(did);
        }
    }

    pub fn touch(&self, id: u64, timestamp: u64) {
        if let Some(peer) = self.peers.lock().get_mut(&id) {
            peer.last_seen = timestamp;
        }
    }

    pub fn record_sent(&self, id: u64, bytes: u64) {
        if let Some(peer) = self.peers.lock().get_mut(&id) {
            peer.bytes_sent += bytes;
            peer.messages_sent += 1;
        }
    }

    pub fn record_recv(&self, id: u64, bytes: u64) {
        if let Some(peer) = self.peers.lock().get_mut(&id) {
            peer.bytes_recv += bytes;
            peer.messages_recv += 1;
        }
    }

    pub fn peer_count(&self) -> usize {
        self.peers.lock().len()
    }

    pub fn connected_count(&self) -> usize {
        self.peers.lock().values().filter(|p| p.status == PeerStatus::Connected).count()
    }

    pub fn list_peers(&self) -> Vec<Peer> {
        self.peers.lock().values().cloned().collect()
    }

    pub fn our_did(&self) -> &str { &self.our_did }
}

// ─── Gossip-Protocol ────────────────────────────────────────────────────────

pub struct GossipProtocol {
    peers: Arc<PeerTable>,
    recv_queue: Mutex<Vec<(u64, P2pMessage)>>, // (peer_id, message)
}

impl GossipProtocol {
    pub fn new(peers: Arc<PeerTable>) -> Self {
        GossipProtocol {
            peers,
            recv_queue: Mutex::new(Vec::new()),
        }
    }

    /// Erzeugt eine Broadcast-Nachricht für alle verbundenen Peers.
    /// Liefert einen Vektor von (peer_id, serialized_message).
    pub fn broadcast(&self, msg: &P2pMessage) -> Vec<(u64, Vec<u8>)> {
        let peers = self.peers.list_peers();
        let serialized = msg.to_bytes();
        let mut result = Vec::new();

        for peer in peers {
            if peer.status == PeerStatus::Connected {
                self.peers.record_sent(peer.id, serialized.len() as u64);
                result.push((peer.id, serialized.clone()));
            }
        }
        result
    }

    /// Sendet eine Nachricht an einen spezifischen Peer.
    pub fn send_to(&self, peer_id: u64, msg: &P2pMessage) -> Result<Vec<u8>, P2pError> {
        let peer = self.peers.get_peer(peer_id).ok_or(P2pError::PeerNotFound)?;
        if peer.status != PeerStatus::Connected && peer.status != PeerStatus::Connecting {
            return Err(P2pError::NotConnected);
        }
        let serialized = msg.to_bytes();
        self.peers.record_sent(peer_id, serialized.len() as u64);
        Ok(serialized)
    }

    /// Verarbeitet eine empfangene Nachricht von einem Peer.
    pub fn handle_message(&self, peer_id: u64, data: &[u8], timestamp: u64) -> Result<(), P2pError> {
        let msg = P2pMessage::from_bytes(data)?;
        self.peers.record_recv(peer_id, data.len() as u64);
        self.peers.touch(peer_id, timestamp);

        match msg.msg_type {
            MessageType::Handshake => {
                // Lerne DID des Peers
                if !msg.sender_did.is_empty() {
                    self.peers.set_did(peer_id, msg.sender_did.clone());
                    self.peers.set_status(peer_id, PeerStatus::Connected);
                }
            }
            MessageType::Bye => {
                self.peers.set_status(peer_id, PeerStatus::Disconnected);
            }
            _ => {}
        }

        self.recv_queue.lock().push((peer_id, msg));
        Ok(())
    }

    /// Holt die nächste empfangene Nachricht aus der Queue.
    pub fn recv(&self) -> Option<(u64, P2pMessage)> {
        let mut queue = self.recv_queue.lock();
        queue.pop()
    }

    /// Anzahl der ungelesenen Nachrichten.
    pub fn pending_messages(&self) -> usize {
        self.recv_queue.lock().len()
    }

    /// Erzeugt einen Ping für einen Peer.
    pub fn make_ping(&self, timestamp: u64) -> P2pMessage {
        P2pMessage::new(MessageType::Ping, self.peers.our_did().to_string(), timestamp, vec![])
    }

    /// Erzeugt einen Pong (Antwort auf Ping).
    pub fn make_pong(&self, timestamp: u64, original_timestamp: u64) -> P2pMessage {
        P2pMessage::new(
            MessageType::Pong,
            self.peers.our_did().to_string(),
            timestamp,
            original_timestamp.to_be_bytes().to_vec(),
        )
    }

    /// Erzeugt einen Handshake für einen neuen Peer.
    pub fn make_handshake(&self, timestamp: u64, listen_port: u16) -> P2pMessage {
        let mut payload = Vec::new();
        payload.extend_from_slice(&listen_port.to_be_bytes());
        payload.extend_from_slice(&CHAIN_ID.to_be_bytes());
        P2pMessage::new(MessageType::Handshake, self.peers.our_did().to_string(), timestamp, payload)
    }

    /// Erzeugt einen Peer-List-Export (für Peer-Discovery).
    pub fn make_peer_list(&self, timestamp: u64) -> P2pMessage {
        let peers = self.peers.list_peers();
        let mut payload = Vec::new();
        payload.push(peers.len() as u8);
        for peer in peers {
            payload.extend_from_slice(&peer.ip.0);
            payload.extend_from_slice(&peer.port.to_be_bytes());
        }
        P2pMessage::new(MessageType::PeerList, self.peers.our_did().to_string(), timestamp, payload)
    }

    /// Verarbeitet eine empfangene Peer-List (fügt neue Peers hinzu).
    pub fn handle_peer_list(&self, data: &[u8]) -> Vec<u64> {
        if data.is_empty() { return vec![]; }
        let count = data[0] as usize;
        let mut new_ids = Vec::new();
        let mut offset = 1;
        for _ in 0..count {
            if offset + 6 > data.len() { break; }
            let mut ip = [0u8; 4];
            ip.copy_from_slice(&data[offset..offset + 4]);
            let port = u16::from_be_bytes([data[offset + 4], data[offset + 5]]);
            offset += 6;
            if let Ok(id) = self.peers.add_peer(Ipv4Address(ip), port) {
                new_ids.push(id);
            }
        }
        new_ids
    }
}

// ─── P2P-Node (Top-Level Integration) ───────────────────────────────────────

pub struct P2pNode {
    peers: Arc<PeerTable>,
    gossip: Arc<GossipProtocol>,
    listen_port: u16,
}

impl P2pNode {
    pub fn new(our_did: String, listen_port: u16, max_peers: usize) -> Self {
        let peers = Arc::new(PeerTable::new(our_did, max_peers));
        let gossip = Arc::new(GossipProtocol::new(peers.clone()));
        P2pNode { peers, gossip, listen_port }
    }

    /// Verbindet sich mit einem neuen Peer (sendet Handshake).
    pub fn connect_peer(&self, ip: Ipv4Address, port: u16, timestamp: u64) -> Result<(u64, Vec<u8>), P2pError> {
        let peer_id = self.peers.add_peer(ip, port)?;
        self.peers.set_status(peer_id, PeerStatus::Connecting);
        let handshake = self.gossip.make_handshake(timestamp, self.listen_port);
        let serialized = self.gossip.send_to(peer_id, &handshake)?;
        Ok((peer_id, serialized))
    }

    /// Verarbeitet einen eingehenden Handshake von einem Peer.
    pub fn handle_handshake(&self, peer_id: u64, data: &[u8], timestamp: u64) -> Result<Vec<u8>, P2pError> {
        self.gossip.handle_message(peer_id, data, timestamp)?;

        // Extrahiere DID aus Handshake
        let msg = P2pMessage::from_bytes(data)?;
        if !msg.sender_did.is_empty() {
            self.peers.set_did(peer_id, msg.sender_did);
        }

        // Sende HandshakeAck zurück
        let ack = P2pMessage::new(
            MessageType::HandshakeAck,
            self.peers.our_did().to_string(),
            timestamp,
            vec![],
        );
        self.peers.set_status(peer_id, PeerStatus::Connected);
        Ok(ack.to_bytes())
    }

    /// Pingt alle verbundenen Peers.
    pub fn ping_all(&self, timestamp: u64) -> Vec<(u64, Vec<u8>)> {
        let ping = self.gossip.make_ping(timestamp);
        self.gossip.broadcast(&ping)
    }

    /// Kündigt einen Block an alle Peers.
    pub fn announce_block(&self, block_hash: &[u8], block_height: u64, timestamp: u64) -> Vec<(u64, Vec<u8>)> {
        let mut payload = Vec::new();
        payload.extend_from_slice(block_hash);
        payload.extend_from_slice(&block_height.to_be_bytes());
        let msg = P2pMessage::new(MessageType::BlockAnnounce, self.peers.our_did().to_string(), timestamp, payload);
        self.gossip.broadcast(&msg)
    }

    /// Kündigt eine Transaktion an alle Peers.
    pub fn announce_tx(&self, tx_hash: &[u8], timestamp: u64) -> Vec<(u64, Vec<u8>)> {
        let msg = P2pMessage::new(
            MessageType::TxAnnounce,
            self.peers.our_did().to_string(),
            timestamp,
            tx_hash.to_vec(),
        );
        self.gossip.broadcast(&msg)
    }

    /// Trennt einen Peer.
    pub fn disconnect_peer(&self, peer_id: u64, timestamp: u64) -> Result<(), P2pError> {
        let bye = P2pMessage::new(MessageType::Bye, self.peers.our_did().to_string(), timestamp, vec![]);
        let _ = self.gossip.send_to(peer_id, &bye);
        self.peers.set_status(peer_id, PeerStatus::Disconnected);
        Ok(())
    }

    pub fn peers(&self) -> &Arc<PeerTable> { &self.peers }
    pub fn gossip(&self) -> &Arc<GossipProtocol> { &self.gossip }
    pub fn listen_port(&self) -> u16 { self.listen_port }
    pub fn peer_count(&self) -> usize { self.peers.peer_count() }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> P2pNode {
        P2pNode::new("did:shivacore:ed25519:test123".to_string(), 9000, 50)
    }

    // ── P2pMessage ──────────────────────────────────────────────────────────

    #[test]
    fn test_message_create() {
        let msg = P2pMessage::new(MessageType::Ping, "did:test".into(), 1000, vec![]);
        assert_eq!(msg.msg_type, MessageType::Ping);
        assert_eq!(msg.chain_id, CHAIN_ID);
        assert_eq!(msg.sender_did, "did:test");
        assert_eq!(msg.timestamp, 1000);
    }

    #[test]
    fn test_message_serialize_deserialize() {
        let msg = P2pMessage::new(
            MessageType::BlockAnnounce,
            "did:shivacore:ed25519:abc123".into(),
            9999,
            vec![0xDE, 0xAD, 0xBE, 0xEF],
        );
        let bytes = msg.to_bytes();
        let parsed = P2pMessage::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.msg_type, MessageType::BlockAnnounce);
        assert_eq!(parsed.chain_id, CHAIN_ID);
        assert_eq!(parsed.sender_did, "did:shivacore:ed25519:abc123");
        assert_eq!(parsed.timestamp, 9999);
        assert_eq!(parsed.payload, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_message_wrong_chain_id() {
        // Manually craft a message with wrong chain_id
        let mut bytes = vec![];
        bytes.push(1); // Ping
        bytes.extend_from_slice(&8888u32.to_be_bytes()); // wrong chain
        bytes.extend_from_slice(&0u16.to_be_bytes()); // empty DID
        bytes.extend_from_slice(&0u64.to_be_bytes()); // timestamp
        assert_eq!(P2pMessage::from_bytes(&bytes), Err(P2pError::WrongChainId(8888)));
    }

    #[test]
    fn test_message_too_short() {
        assert_eq!(P2pMessage::from_bytes(&[0; 5]), Err(P2pError::MessageTooShort));
    }

    #[test]
    fn test_message_unknown_type() {
        let mut bytes = vec![];
        bytes.push(99); // unknown type
        bytes.extend_from_slice(&CHAIN_ID.to_be_bytes());
        bytes.extend_from_slice(&0u16.to_be_bytes());
        bytes.extend_from_slice(&0u64.to_be_bytes());
        assert!(matches!(P2pMessage::from_bytes(&bytes), Err(P2pError::UnknownMessageType(99))));
    }

    #[test]
    fn test_all_message_types() {
        for t in [MessageType::Ping, MessageType::Pong, MessageType::Handshake,
                  MessageType::HandshakeAck, MessageType::BlockAnnounce,
                  MessageType::TxAnnounce, MessageType::Vote,
                  MessageType::PeerList, MessageType::Bye] {
            assert_eq!(MessageType::from_u8(t as u8), Some(t));
        }
    }

    // ── PeerTable ────────────────────────────────────────────────────────────

    #[test]
    fn test_add_peer() {
        let table = PeerTable::new("did:me".into(), 50);
        let id = table.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        assert_eq!(table.peer_count(), 1);
        let peer = table.get_peer(id).unwrap();
        assert_eq!(peer.ip, Ipv4Address::new(10, 0, 0, 2));
        assert_eq!(peer.port, 9000);
        assert_eq!(peer.status, PeerStatus::Disconnected);
    }

    #[test]
    fn test_add_duplicate_peer() {
        let table = PeerTable::new("did:me".into(), 50);
        table.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        assert_eq!(table.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000), Err(P2pError::PeerAlreadyConnected));
    }

    #[test]
    fn test_remove_peer() {
        let table = PeerTable::new("did:me".into(), 50);
        let id = table.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        assert!(table.remove_peer(id));
        assert_eq!(table.peer_count(), 0);
        assert!(!table.remove_peer(id));
    }

    #[test]
    fn test_find_by_addr() {
        let table = PeerTable::new("did:me".into(), 50);
        table.add_peer(Ipv4Address::new(192, 168, 1, 10), 8080).unwrap();
        let peer = table.find_by_addr(Ipv4Address::new(192, 168, 1, 10), 8080).unwrap();
        assert_eq!(peer.ip, Ipv4Address::new(192, 168, 1, 10));
    }

    #[test]
    fn test_set_status_and_did() {
        let table = PeerTable::new("did:me".into(), 50);
        let id = table.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        table.set_status(id, PeerStatus::Connected);
        table.set_did(id, "did:peer1".into());
        let peer = table.get_peer(id).unwrap();
        assert_eq!(peer.status, PeerStatus::Connected);
        assert_eq!(peer.did, Some("did:peer1".into()));
    }

    #[test]
    fn test_connected_count() {
        let table = PeerTable::new("did:me".into(), 50);
        let id1 = table.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        let id2 = table.add_peer(Ipv4Address::new(10, 0, 0, 3), 9000).unwrap();
        table.set_status(id1, PeerStatus::Connected);
        assert_eq!(table.connected_count(), 1);
        table.set_status(id2, PeerStatus::Connected);
        assert_eq!(table.connected_count(), 2);
    }

    #[test]
    fn test_max_peers() {
        let table = PeerTable::new("did:me".into(), 2);
        table.add_peer(Ipv4Address::new(10, 0, 0, 1), 9000).unwrap();
        table.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        assert_eq!(table.add_peer(Ipv4Address::new(10, 0, 0, 3), 9000), Err(P2pError::PeerAlreadyConnected));
    }

    #[test]
    fn test_stats_tracking() {
        let table = PeerTable::new("did:me".into(), 50);
        let id = table.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        table.record_sent(id, 100);
        table.record_sent(id, 50);
        table.record_recv(id, 200);
        let peer = table.get_peer(id).unwrap();
        assert_eq!(peer.bytes_sent, 150);
        assert_eq!(peer.bytes_recv, 200);
        assert_eq!(peer.messages_sent, 2);
        assert_eq!(peer.messages_recv, 1);
    }

    // ── GossipProtocol ────────────────────────────────────────────────────────

    #[test]
    fn test_gossip_broadcast() {
        let peers = Arc::new(PeerTable::new("did:me".into(), 50));
        let id1 = peers.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        let id2 = peers.add_peer(Ipv4Address::new(10, 0, 0, 3), 9000).unwrap();
        peers.set_status(id1, PeerStatus::Connected);
        peers.set_status(id2, PeerStatus::Connected);

        let gossip = GossipProtocol::new(peers.clone());
        let msg = P2pMessage::new(MessageType::Ping, "did:me".into(), 1000, vec![]);
        let result = gossip.broadcast(&msg);
        assert_eq!(result.len(), 2); // both connected peers
    }

    #[test]
    fn test_gossip_broadcast_only_connected() {
        let peers = Arc::new(PeerTable::new("did:me".into(), 50));
        let id1 = peers.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        let _id2 = peers.add_peer(Ipv4Address::new(10, 0, 0, 3), 9000).unwrap();
        peers.set_status(id1, PeerStatus::Connected);
        // id2 stays Disconnected

        let gossip = GossipProtocol::new(peers.clone());
        let msg = P2pMessage::new(MessageType::Ping, "did:me".into(), 1000, vec![]);
        let result = gossip.broadcast(&msg);
        assert_eq!(result.len(), 1); // only connected
    }

    #[test]
    fn test_gossip_send_to_specific() {
        let peers = Arc::new(PeerTable::new("did:me".into(), 50));
        let id = peers.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        peers.set_status(id, PeerStatus::Connected);

        let gossip = GossipProtocol::new(peers.clone());
        let msg = P2pMessage::new(MessageType::Pong, "did:me".into(), 1000, vec![]);
        let serialized = gossip.send_to(id, &msg).unwrap();
        assert!(!serialized.is_empty());
    }

    #[test]
    fn test_gossip_send_to_not_connected() {
        let peers = Arc::new(PeerTable::new("did:me".into(), 50));
        let id = peers.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        // Status = Disconnected

        let gossip = GossipProtocol::new(peers.clone());
        let msg = P2pMessage::new(MessageType::Ping, "did:me".into(), 1000, vec![]);
        assert_eq!(gossip.send_to(id, &msg), Err(P2pError::NotConnected));
    }

    #[test]
    fn test_gossip_handle_handshake() {
        let peers = Arc::new(PeerTable::new("did:me".into(), 50));
        let id = peers.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();

        let gossip = GossipProtocol::new(peers.clone());
        let handshake = P2pMessage::new(MessageType::Handshake, "did:peer".into(), 1000, vec![]);
        gossip.handle_message(id, &handshake.to_bytes(), 1000).unwrap();

        let peer = peers.get_peer(id).unwrap();
        assert_eq!(peer.status, PeerStatus::Connected);
        assert_eq!(peer.did, Some("did:peer".into()));
        assert_eq!(gossip.pending_messages(), 1);
    }

    #[test]
    fn test_gossip_handle_bye() {
        let peers = Arc::new(PeerTable::new("did:me".into(), 50));
        let id = peers.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        peers.set_status(id, PeerStatus::Connected);

        let gossip = GossipProtocol::new(peers.clone());
        let bye = P2pMessage::new(MessageType::Bye, "did:peer".into(), 2000, vec![]);
        gossip.handle_message(id, &bye.to_bytes(), 2000).unwrap();

        let peer = peers.get_peer(id).unwrap();
        assert_eq!(peer.status, PeerStatus::Disconnected);
    }

    #[test]
    fn test_gossip_make_ping_pong() {
        let peers = Arc::new(PeerTable::new("did:me".into(), 50));
        let gossip = GossipProtocol::new(peers);

        let ping = gossip.make_ping(1000);
        assert_eq!(ping.msg_type, MessageType::Ping);
        assert_eq!(ping.payload, vec![]);

        let pong = gossip.make_pong(1001, 1000);
        assert_eq!(pong.msg_type, MessageType::Pong);
        assert_eq!(u64::from_be_bytes(pong.payload[..8].try_into().unwrap()), 1000);
    }

    #[test]
    fn test_gossip_make_handshake() {
        let peers = Arc::new(PeerTable::new("did:me".into(), 50));
        let gossip = GossipProtocol::new(peers);
        let hs = gossip.make_handshake(1000, 9000);
        assert_eq!(hs.msg_type, MessageType::Handshake);
        let port = u16::from_be_bytes([hs.payload[0], hs.payload[1]]);
        let chain = u32::from_be_bytes([hs.payload[2], hs.payload[3], hs.payload[4], hs.payload[5]]);
        assert_eq!(port, 9000);
        assert_eq!(chain, CHAIN_ID);
    }

    // ── Peer-Discovery via PeerList ──────────────────────────────────────────

    #[test]
    fn test_peer_list_export_import() {
        let peers = Arc::new(PeerTable::new("did:me".into(), 50));
        peers.add_peer(Ipv4Address::new(10, 0, 0, 2), 9000).unwrap();
        peers.add_peer(Ipv4Address::new(10, 0, 0, 3), 9001).unwrap();

        let gossip = GossipProtocol::new(peers.clone());
        let list_msg = gossip.make_peer_list(1000);
        assert_eq!(list_msg.msg_type, MessageType::PeerList);
        assert_eq!(list_msg.payload[0], 2); // 2 peers

        // Import in a new table
        let peers2 = Arc::new(PeerTable::new("did:me2".into(), 50));
        let gossip2 = GossipProtocol::new(peers2.clone());
        let new_ids = gossip2.handle_peer_list(&list_msg.payload);
        assert_eq!(new_ids.len(), 2);
        assert_eq!(peers2.peer_count(), 2);
    }

    // ── P2pNode Integration ───────────────────────────────────────────────────

    #[test]
    fn test_p2p_node_connect() {
        let node = setup();
        let (peer_id, handshake_bytes) = node.connect_peer(
            Ipv4Address::new(10, 0, 0, 2), 9001, 1000,
        ).unwrap();
        assert!(!handshake_bytes.is_empty());
        assert_eq!(node.peers().get_peer(peer_id).unwrap().status, PeerStatus::Connecting);
    }

    #[test]
    fn test_p2p_node_handle_handshake() {
        let node = setup();
        let peer_id = node.peers().add_peer(Ipv4Address::new(10, 0, 0, 2), 9001).unwrap();

        let handshake = P2pMessage::new(
            MessageType::Handshake, "did:shivacore:ed25519:peerXYZ".into(), 1000, vec![]
        );
        let ack_bytes = node.handle_handshake(peer_id, &handshake.to_bytes(), 1000).unwrap();

        let ack = P2pMessage::from_bytes(&ack_bytes).unwrap();
        assert_eq!(ack.msg_type, MessageType::HandshakeAck);
        assert_eq!(node.peers().get_peer(peer_id).unwrap().status, PeerStatus::Connected);
        assert_eq!(node.peers().get_peer(peer_id).unwrap().did, Some("did:shivacore:ed25519:peerXYZ".into()));
    }

    #[test]
    fn test_p2p_node_ping_all() {
        let node = setup();
        let id1 = node.peers().add_peer(Ipv4Address::new(10, 0, 0, 2), 9001).unwrap();
        let id2 = node.peers().add_peer(Ipv4Address::new(10, 0, 0, 3), 9002).unwrap();
        node.peers().set_status(id1, PeerStatus::Connected);
        node.peers().set_status(id2, PeerStatus::Connected);

        let pings = node.ping_all(1000);
        assert_eq!(pings.len(), 2);
    }

    #[test]
    fn test_p2p_node_announce_block() {
        let node = setup();
        let id = node.peers().add_peer(Ipv4Address::new(10, 0, 0, 2), 9001).unwrap();
        node.peers().set_status(id, PeerStatus::Connected);

        let block_hash = [0xAA; 32];
        let announcements = node.announce_block(&block_hash, 42, 1000);
        assert_eq!(announcements.len(), 1);

        let msg = P2pMessage::from_bytes(&announcements[0].1).unwrap();
        assert_eq!(msg.msg_type, MessageType::BlockAnnounce);
        assert_eq!(&msg.payload[..32], &block_hash);
        let height = u64::from_be_bytes(msg.payload[32..40].try_into().unwrap());
        assert_eq!(height, 42);
    }

    #[test]
    fn test_p2p_node_announce_tx() {
        let node = setup();
        let id = node.peers().add_peer(Ipv4Address::new(10, 0, 0, 2), 9001).unwrap();
        node.peers().set_status(id, PeerStatus::Connected);

        let tx_hash = [0xBB; 32];
        let announcements = node.announce_tx(&tx_hash, 1000);
        assert_eq!(announcements.len(), 1);

        let msg = P2pMessage::from_bytes(&announcements[0].1).unwrap();
        assert_eq!(msg.msg_type, MessageType::TxAnnounce);
        assert_eq!(msg.payload, tx_hash.to_vec());
    }

    #[test]
    fn test_p2p_node_disconnect() {
        let node = setup();
        let id = node.peers().add_peer(Ipv4Address::new(10, 0, 0, 2), 9001).unwrap();
        node.peers().set_status(id, PeerStatus::Connected);

        node.disconnect_peer(id, 2000).unwrap();
        assert_eq!(node.peers().get_peer(id).unwrap().status, PeerStatus::Disconnected);
    }

    #[test]
    fn test_chain_id_is_658467() {
        assert_eq!(CHAIN_ID, 658467);
    }
}
