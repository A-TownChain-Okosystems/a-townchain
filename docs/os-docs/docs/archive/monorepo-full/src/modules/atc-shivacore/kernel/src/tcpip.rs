// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ─────────────────────────────────────────────────────────────────────────
// K-Sprint 13 — TCP/IP-Layer
// Kernel Layer | Chain-ID 658467
// IPv4, UDP, TCP, Routing-Table, Socket-Abstraktion.
// Baut auf K12 (Ethernet + ARP + NetworkDevice) auf.
// ─────────────────────────────────────────────────────────────────────────

use alloc::vec;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use spin::Mutex;

use crate::net::{
    MacAddress, Ipv4Address, EthernetFrame, ETH_TYPE_IPV4,
    NetworkDevice, NetworkError, NetworkStack,
};

// ─── Protokoll-Nummern ─────────────────────────────────────────────────────

pub const IP_PROTO_ICMP: u8 = 1;
pub const IP_PROTO_TCP: u8 = 6;
pub const IP_PROTO_UDP: u8 = 17;

// ═══════════════════════════════════════════════════════════════════════════
// IPv4-Packet
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq)]
pub struct Ipv4Packet {
    pub version: u8,
    pub ihl: u8,
    pub dscp: u8,
    pub ecn: u8,
    pub total_length: u16,
    pub identification: u16,
    pub flags: u8,
    pub fragment_offset: u16,
    pub ttl: u8,
    pub protocol: u8,
    pub header_checksum: u16,
    pub src_ip: Ipv4Address,
    pub dst_ip: Ipv4Address,
    pub payload: Vec<u8>,
}

impl Ipv4Packet {
    pub fn new(src: Ipv4Address, dst: Ipv4Address, protocol: u8, payload: Vec<u8>) -> Self {
        let total_length = (20 + payload.len()) as u16;
        Ipv4Packet {
            version: 4, ihl: 5, dscp: 0, ecn: 0,
            total_length, identification: 0, flags: 0,
            fragment_offset: 0, ttl: 64, protocol,
            header_checksum: 0, src_ip: src, dst_ip: dst, payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(20 + self.payload.len());
        buf.push((self.version << 4) | self.ihl);
        buf.push((self.dscp << 2) | self.ecn);
        buf.extend_from_slice(&self.total_length.to_be_bytes());
        buf.extend_from_slice(&self.identification.to_be_bytes());
        let flags_frag = ((self.flags as u16) << 13) | (self.fragment_offset & 0x1FFF);
        buf.extend_from_slice(&flags_frag.to_be_bytes());
        buf.push(self.ttl);
        buf.push(self.protocol);
        buf.extend_from_slice(&self.header_checksum.to_be_bytes());
        buf.extend_from_slice(&self.src_ip.0);
        buf.extend_from_slice(&self.dst_ip.0);
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, NetworkError> {
        if data.len() < 20 { return Err(NetworkError::PacketTooShort); }
        let version = data[0] >> 4;
        let ihl = data[0] & 0x0F;
        if version != 4 { return Err(NetworkError::UnsupportedProtocol); }
        let dscp = data[1] >> 2;
        let ecn = data[1] & 0x03;
        let total_length = u16::from_be_bytes([data[2], data[3]]);
        let identification = u16::from_be_bytes([data[4], data[5]]);
        let flags_frag = u16::from_be_bytes([data[6], data[7]]);
        let flags = (flags_frag >> 13) as u8;
        let fragment_offset = flags_frag & 0x1FFF;
        let ttl = data[8];
        let protocol = data[9];
        let header_checksum = u16::from_be_bytes([data[10], data[11]]);
        let mut src_ip = [0u8; 4];
        src_ip.copy_from_slice(&data[12..16]);
        let mut dst_ip = [0u8; 4];
        dst_ip.copy_from_slice(&data[16..20]);
        let header_len = (ihl as usize) * 4;
        if data.len() < header_len { return Err(NetworkError::PacketTooShort); }
        let payload = data[header_len..].to_vec();

        Ok(Ipv4Packet {
            version, ihl, dscp, ecn, total_length, identification,
            flags, fragment_offset, ttl, protocol, header_checksum,
            src_ip: Ipv4Address(src_ip), dst_ip: Ipv4Address(dst_ip), payload,
        })
    }

    pub fn calculate_checksum(header: &[u8]) -> u16 {
        let mut sum: u32 = 0;
        let mut i = 0;
        while i + 1 < header.len() {
            sum += u16::from_be_bytes([header[i], header[i + 1]]) as u32;
            i += 2;
        }
        if i < header.len() { sum += (header[i] as u32) << 8; }
        while sum >> 16 != 0 { sum = (sum & 0xFFFF) + (sum >> 16); }
        !(sum as u16)
    }

    pub fn with_checksum(mut self) -> Self {
        let mut header = self.to_bytes();
        header[10] = 0; header[11] = 0;
        self.header_checksum = Self::calculate_checksum(&header[..20]);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UDP-Packet
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq)]
pub struct UdpPacket {
    pub src_port: u16,
    pub dst_port: u16,
    pub length: u16,
    pub checksum: u16,
    pub payload: Vec<u8>,
}

impl UdpPacket {
    pub fn new(src_port: u16, dst_port: u16, payload: Vec<u8>) -> Self {
        UdpPacket { src_port, dst_port, length: (8 + payload.len()) as u16, checksum: 0, payload }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8 + self.payload.len());
        buf.extend_from_slice(&self.src_port.to_be_bytes());
        buf.extend_from_slice(&self.dst_port.to_be_bytes());
        buf.extend_from_slice(&self.length.to_be_bytes());
        buf.extend_from_slice(&self.checksum.to_be_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, NetworkError> {
        if data.len() < 8 { return Err(NetworkError::PacketTooShort); }
        Ok(UdpPacket {
            src_port: u16::from_be_bytes([data[0], data[1]]),
            dst_port: u16::from_be_bytes([data[2], data[3]]),
            length: u16::from_be_bytes([data[4], data[5]]),
            checksum: u16::from_be_bytes([data[6], data[7]]),
            payload: data[8..].to_vec(),
        })
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TCP-Segment
// ═══════════════════════════════════════════════════════════════════════════

pub const TCP_FIN: u8 = 0x01;
pub const TCP_SYN: u8 = 0x02;
pub const TCP_RST: u8 = 0x04;
pub const TCP_PSH: u8 = 0x08;
pub const TCP_ACK: u8 = 0x10;
pub const TCP_URG: u8 = 0x20;

#[derive(Clone, Debug, PartialEq)]
pub struct TcpSegment {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq_num: u32,
    pub ack_num: u32,
    pub data_offset: u8,
    pub flags: u8,
    pub window_size: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
    pub payload: Vec<u8>,
}

impl TcpSegment {
    pub fn new(src_port: u16, dst_port: u16, seq: u32, ack: u32, flags: u8, payload: Vec<u8>) -> Self {
        TcpSegment {
            src_port, dst_port, seq_num: seq, ack_num: ack,
            data_offset: 5, flags, window_size: 65535,
            checksum: 0, urgent_ptr: 0, payload,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(20 + self.payload.len());
        buf.extend_from_slice(&self.src_port.to_be_bytes());
        buf.extend_from_slice(&self.dst_port.to_be_bytes());
        buf.extend_from_slice(&self.seq_num.to_be_bytes());
        buf.extend_from_slice(&self.ack_num.to_be_bytes());
        buf.push((self.data_offset << 4) | 0);
        buf.push(self.flags);
        buf.extend_from_slice(&self.window_size.to_be_bytes());
        buf.extend_from_slice(&self.checksum.to_be_bytes());
        buf.extend_from_slice(&self.urgent_ptr.to_be_bytes());
        buf.extend_from_slice(&self.payload);
        buf
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, NetworkError> {
        if data.len() < 20 { return Err(NetworkError::PacketTooShort); }
        let data_offset = data[12] >> 4;
        let header_len = (data_offset as usize) * 4;
        let payload = if data.len() > header_len { data[header_len..].to_vec() } else { Vec::new() };
        Ok(TcpSegment {
            src_port: u16::from_be_bytes([data[0], data[1]]),
            dst_port: u16::from_be_bytes([data[2], data[3]]),
            seq_num: u32::from_be_bytes([data[4], data[5], data[6], data[7]]),
            ack_num: u32::from_be_bytes([data[8], data[9], data[10], data[11]]),
            data_offset, flags: data[13],
            window_size: u16::from_be_bytes([data[14], data[15]]),
            checksum: u16::from_be_bytes([data[16], data[17]]),
            urgent_ptr: u16::from_be_bytes([data[18], data[19]]),
            payload,
        })
    }

    pub fn is_syn(&self) -> bool { self.flags & TCP_SYN != 0 }
    pub fn is_ack(&self) -> bool { self.flags & TCP_ACK != 0 }
    pub fn is_fin(&self) -> bool { self.flags & TCP_FIN != 0 }
    pub fn is_rst(&self) -> bool { self.flags & TCP_RST != 0 }
}

// ═══════════════════════════════════════════════════════════════════════════
// Routing-Table
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq)]
pub struct Route {
    pub network: Ipv4Address,
    pub prefix_len: u8,
    pub gateway: Option<Ipv4Address>,
    pub interface: String,
    pub metric: u32,
}

pub struct RoutingTable {
    routes: Mutex<Vec<Route>>,
}

impl RoutingTable {
    pub fn new() -> Self { RoutingTable { routes: Mutex::new(Vec::new()) } }

    pub fn add(&self, route: Route) {
        let mut routes = self.routes.lock();
        routes.push(route);
        routes.sort_by_key(|r| r.metric);
    }

    pub fn lookup(&self, dst: Ipv4Address) -> Option<Route> {
        let routes = self.routes.lock();
        let mut best: Option<&Route> = None;
        let mut best_len: u8 = 0;
        for route in routes.iter() {
            if Self::matches(dst, route.network, route.prefix_len) {
                if route.prefix_len >= best_len {
                    best = Some(route);
                    best_len = route.prefix_len;
                }
            }
        }
        best.cloned()
    }

    fn matches(ip: Ipv4Address, network: Ipv4Address, prefix: u8) -> bool {
        if prefix == 0 { return true; }
        let mask = if prefix >= 32 { 0xFFFFFFFFu32 } else { !((1u32 << (32 - prefix)) - 1) };
        let ip_u32 = u32::from_be_bytes(ip.0);
        let net_u32 = u32::from_be_bytes(network.0);
        (ip_u32 & mask) == (net_u32 & mask)
    }

    pub fn route_count(&self) -> usize { self.routes.lock().len() }
    pub fn clear(&self) { self.routes.lock().clear(); }
}

// ═══════════════════════════════════════════════════════════════════════════
// Socket-Abstraktion
// ═══════════════════════════════════════════════════════════════════════════

pub type SocketId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcpState {
    Closed, Listen, SynSent, SynReceived, Established,
    FinWait1, FinWait2, CloseWait, LastAck, TimeWait,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UdpSocket {
    pub id: SocketId,
    pub local_ip: Ipv4Address,
    pub local_port: u16,
    pub remote_ip: Option<Ipv4Address>,
    pub remote_port: Option<u16>,
    pub recv_queue: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TcpSocket {
    pub id: SocketId,
    pub local_ip: Ipv4Address,
    pub local_port: u16,
    pub remote_ip: Option<Ipv4Address>,
    pub remote_port: Option<u16>,
    pub state: TcpState,
    pub seq_num: u32,
    pub ack_num: u32,
    pub recv_queue: Vec<Vec<u8>>,
}

// ═══════════════════════════════════════════════════════════════════════════
// Socket-Manager
// ═══════════════════════════════════════════════════════════════════════════

pub struct SocketManager {
    udp_sockets: Mutex<BTreeMap<SocketId, UdpSocket>>,
    tcp_sockets: Mutex<BTreeMap<SocketId, TcpSocket>>,
    next_id: Mutex<SocketId>,
}

impl SocketManager {
    pub fn new() -> Self {
        SocketManager {
            udp_sockets: Mutex::new(BTreeMap::new()),
            tcp_sockets: Mutex::new(BTreeMap::new()),
            next_id: Mutex::new(1),
        }
    }

    fn alloc_id(&self) -> SocketId {
        let mut id = self.next_id.lock();
        let v = *id; *id += 1; v
    }

    // ── UDP ──────────────────────────────────────────────────────────────

    pub fn udp_bind(&self, local_ip: Ipv4Address, local_port: u16) -> SocketId {
        let id = self.alloc_id();
        self.udp_sockets.lock().insert(id, UdpSocket {
            id, local_ip, local_port,
            remote_ip: None, remote_port: None, recv_queue: Vec::new(),
        });
        id
    }

    pub fn udp_connect(&self, id: SocketId, remote_ip: Ipv4Address, remote_port: u16) -> Result<(), NetworkError> {
        let mut sockets = self.udp_sockets.lock();
        let socket = sockets.get_mut(&id).ok_or(NetworkError::ArpResolutionFailed)?;
        socket.remote_ip = Some(remote_ip);
        socket.remote_port = Some(remote_port);
        Ok(())
    }

    pub fn udp_send(&self, id: SocketId, data: &[u8], stack: &NetworkStack) -> Result<(), NetworkError> {
        let sockets = self.udp_sockets.lock();
        let socket = sockets.get(&id).ok_or(NetworkError::ArpResolutionFailed)?;
        let remote_ip = socket.remote_ip.ok_or(NetworkError::ArpResolutionFailed)?;
        let remote_port = socket.remote_port.ok_or(NetworkError::ArpResolutionFailed)?;
        let udp = UdpPacket::new(socket.local_port, remote_port, data.to_vec());
        let ip = Ipv4Packet::new(socket.local_ip, remote_ip, IP_PROTO_UDP, udp.to_bytes()).with_checksum();
        let dst_mac = stack.resolve_mac(remote_ip).ok_or(NetworkError::ArpResolutionFailed)?;
        let frame = EthernetFrame::new(dst_mac, stack.device.mac_address(), ETH_TYPE_IPV4, ip.to_bytes());
        stack.device.send_frame(&frame.to_bytes())
    }

    pub fn udp_recv(&self, id: SocketId) -> Result<Vec<u8>, NetworkError> {
        let mut sockets = self.udp_sockets.lock();
        let socket = sockets.get_mut(&id).ok_or(NetworkError::ArpResolutionFailed)?;
        socket.recv_queue.pop().ok_or(NetworkError::NoFrameAvailable)
    }

    pub fn handle_udp(&self, _src_ip: Ipv4Address, _src_port: u16, dst_port: u16, payload: Vec<u8>) {
        let mut sockets = self.udp_sockets.lock();
        for socket in sockets.values_mut() {
            if socket.local_port == dst_port {
                socket.recv_queue.insert(0, payload.clone());
            }
        }
    }

    pub fn udp_close(&self, id: SocketId) { self.udp_sockets.lock().remove(&id); }

    // ── TCP ──────────────────────────────────────────────────────────────

    pub fn tcp_bind(&self, local_ip: Ipv4Address, local_port: u16) -> SocketId {
        let id = self.alloc_id();
        self.tcp_sockets.lock().insert(id, TcpSocket {
            id, local_ip, local_port, remote_ip: None, remote_port: None,
            state: TcpState::Listen, seq_num: 1000, ack_num: 0, recv_queue: Vec::new(),
        });
        id
    }

    pub fn tcp_connect(&self, id: SocketId, remote_ip: Ipv4Address, remote_port: u16) -> Result<TcpState, NetworkError> {
        let mut sockets = self.tcp_sockets.lock();
        let socket = sockets.get_mut(&id).ok_or(NetworkError::ArpResolutionFailed)?;
        socket.remote_ip = Some(remote_ip);
        socket.remote_port = Some(remote_port);
        socket.state = TcpState::SynSent;
        Ok(socket.state)
    }

    pub fn tcp_state(&self, id: SocketId) -> Option<TcpState> {
        self.tcp_sockets.lock().get(&id).map(|s| s.state)
    }

    pub fn handle_tcp(&self, src_ip: Ipv4Address, src_port: u16, dst_port: u16, seg: &TcpSegment) {
        let mut sockets = self.tcp_sockets.lock();
        let matching_id: Option<SocketId> = sockets.values()
            .find(|s| s.local_port == dst_port).map(|s| s.id);

        if let Some(id) = matching_id {
            let socket = sockets.get_mut(&id).unwrap();
            match socket.state {
                TcpState::Listen => {
                    if seg.is_syn() {
                        socket.remote_ip = Some(src_ip);
                        socket.remote_port = Some(src_port);
                        socket.ack_num = seg.seq_num.wrapping_add(1);
                        socket.state = TcpState::SynReceived;
                    }
                }
                TcpState::SynSent => {
                    if seg.is_syn() && seg.is_ack() {
                        socket.ack_num = seg.seq_num.wrapping_add(1);
                        socket.seq_num = seg.ack_num;
                        socket.state = TcpState::Established;
                    }
                }
                TcpState::Established => {
                    if seg.is_fin() {
                        socket.ack_num = seg.seq_num.wrapping_add(1);
                        socket.state = TcpState::CloseWait;
                    } else if !seg.payload.is_empty() {
                        socket.recv_queue.insert(0, seg.payload.clone());
                        socket.ack_num = seg.seq_num.wrapping_add(seg.payload.len() as u32);
                    }
                }
                TcpState::FinWait1 => {
                    if seg.is_ack() { socket.state = TcpState::FinWait2; }
                    if seg.is_fin() { socket.state = TcpState::TimeWait; }
                }
                TcpState::LastAck => {
                    if seg.is_ack() { socket.state = TcpState::Closed; }
                }
                _ => {}
            }
        }
    }

    pub fn tcp_recv(&self, id: SocketId) -> Result<Vec<u8>, NetworkError> {
        let mut sockets = self.tcp_sockets.lock();
        let socket = sockets.get_mut(&id).ok_or(NetworkError::ArpResolutionFailed)?;
        socket.recv_queue.pop().ok_or(NetworkError::NoFrameAvailable)
    }

    pub fn tcp_close(&self, id: SocketId) {
        let mut sockets = self.tcp_sockets.lock();
        if let Some(socket) = sockets.get_mut(&id) { socket.state = TcpState::Closed; }
        sockets.remove(&id);
    }

    pub fn socket_count(&self) -> (usize, usize) {
        (self.udp_sockets.lock().len(), self.tcp_sockets.lock().len())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// IpStack — verbindet NetworkStack + Routing + Sockets
// ═══════════════════════════════════════════════════════════════════════════

pub struct IpStack {
    net: Arc<NetworkStack>,
    routing: Arc<RoutingTable>,
    sockets: Arc<SocketManager>,
}

impl IpStack {
    pub fn new(net: Arc<NetworkStack>) -> Self {
        IpStack {
            net, routing: Arc::new(RoutingTable::new()), sockets: Arc::new(SocketManager::new()),
        }
    }

    pub fn handle_ipv4(&self, packet: &Ipv4Packet) {
        if packet.dst_ip != self.net.our_ip() && !packet.dst_ip.is_broadcast() { return; }
        match packet.protocol {
            IP_PROTO_UDP => {
                if let Ok(udp) = UdpPacket::from_bytes(&packet.payload) {
                    self.sockets.handle_udp(packet.src_ip, udp.src_port, udp.dst_port, udp.payload);
                }
            }
            IP_PROTO_TCP => {
                if let Ok(tcp) = TcpSegment::from_bytes(&packet.payload) {
                    self.sockets.handle_tcp(packet.src_ip, tcp.src_port, tcp.dst_port, &tcp);
                }
            }
            _ => {}
        }
    }

    pub fn handle_frame(&self, data: &[u8], timestamp: u64) -> Result<(), NetworkError> {
        let frame = EthernetFrame::from_bytes(data)?;
        match frame.ethertype {
            crate::net::ETH_TYPE_ARP => self.net.handle_frame(data, timestamp),
            ETH_TYPE_IPV4 => {
                let packet = Ipv4Packet::from_bytes(&frame.payload)?;
                self.handle_ipv4(&packet);
                Ok(())
            }
            _ => Err(NetworkError::UnsupportedProtocol),
        }
    }

    pub fn routing(&self) -> &Arc<RoutingTable> { &self.routing }
    pub fn sockets(&self) -> &Arc<SocketManager> { &self.sockets }
    pub fn net(&self) -> &Arc<NetworkStack> { &self.net }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::net::{LoopbackDevice, NetworkStack};

    // ── IPv4 ────────────────────────────────────────────────────────────────

    #[test]
    fn test_ipv4_create() {
        let pkt = Ipv4Packet::new(
            Ipv4Address::new(10, 0, 0, 1), Ipv4Address::new(10, 0, 0, 2),
            IP_PROTO_UDP, vec![0xDE, 0xAD],
        );
        assert_eq!(pkt.version, 4);
        assert_eq!(pkt.ihl, 5);
        assert_eq!(pkt.protocol, IP_PROTO_UDP);
        assert_eq!(pkt.ttl, 64);
        assert_eq!(pkt.total_length, 22);
    }

    #[test]
    fn test_ipv4_serialize_deserialize() {
        let pkt = Ipv4Packet::new(
            Ipv4Address::new(192, 168, 1, 1), Ipv4Address::new(192, 168, 1, 2),
            IP_PROTO_TCP, vec![0x01, 0x02, 0x03],
        );
        let bytes = pkt.to_bytes();
        assert_eq!(bytes.len(), 23);
        let parsed = Ipv4Packet::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.src_ip, pkt.src_ip);
        assert_eq!(parsed.dst_ip, pkt.dst_ip);
        assert_eq!(parsed.protocol, IP_PROTO_TCP);
        assert_eq!(parsed.payload, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn test_ipv4_checksum() {
        let pkt = Ipv4Packet::new(
            Ipv4Address::new(10, 0, 0, 1), Ipv4Address::new(10, 0, 0, 2),
            IP_PROTO_UDP, vec![],
        ).with_checksum();
        assert_ne!(pkt.header_checksum, 0);
        let mut bytes = pkt.to_bytes();
        bytes[10] = 0; bytes[11] = 0;
        let computed = Ipv4Packet::calculate_checksum(&bytes[..20]);
        assert_eq!(computed, pkt.header_checksum);
    }

    #[test]
    fn test_ipv4_too_short() {
        assert_eq!(Ipv4Packet::from_bytes(&[0u8; 10]), Err(NetworkError::PacketTooShort));
    }

    // ── UDP ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_udp_create() {
        let udp = UdpPacket::new(1234, 5678, vec![0x42]);
        assert_eq!(udp.src_port, 1234);
        assert_eq!(udp.dst_port, 5678);
        assert_eq!(udp.length, 9);
    }

    #[test]
    fn test_udp_serialize_deserialize() {
        let udp = UdpPacket::new(1000, 2000, vec![0xDE, 0xAD, 0xBE, 0xEF]);
        let bytes = udp.to_bytes();
        assert_eq!(bytes.len(), 12);
        let parsed = UdpPacket::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.src_port, 1000);
        assert_eq!(parsed.dst_port, 2000);
        assert_eq!(parsed.payload, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    }

    #[test]
    fn test_udp_too_short() {
        assert_eq!(UdpPacket::from_bytes(&[0; 4]), Err(NetworkError::PacketTooShort));
    }

    // ── TCP ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_tcp_create() {
        let seg = TcpSegment::new(80, 443, 1000, 2000, TCP_SYN | TCP_ACK, vec![]);
        assert_eq!(seg.src_port, 80);
        assert_eq!(seg.dst_port, 443);
        assert!(seg.is_syn());
        assert!(seg.is_ack());
        assert!(!seg.is_fin());
    }

    #[test]
    fn test_tcp_serialize_deserialize() {
        let seg = TcpSegment::new(8080, 9090, 42, 99, TCP_ACK, vec![0x01, 0x02]);
        let bytes = seg.to_bytes();
        assert_eq!(bytes.len(), 22);
        let parsed = TcpSegment::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.src_port, 8080);
        assert_eq!(parsed.dst_port, 9090);
        assert_eq!(parsed.seq_num, 42);
        assert_eq!(parsed.ack_num, 99);
        assert!(parsed.is_ack());
        assert_eq!(parsed.payload, vec![0x01, 0x02]);
    }

    #[test]
    fn test_tcp_flags() {
        assert!(TcpSegment::new(1, 2, 0, 0, TCP_SYN, vec![]).is_syn());
        assert!(TcpSegment::new(1, 2, 0, 0, TCP_FIN | TCP_ACK, vec![]).is_fin());
        assert!(TcpSegment::new(1, 2, 0, 0, TCP_RST, vec![]).is_rst());
    }

    #[test]
    fn test_tcp_too_short() {
        assert_eq!(TcpSegment::from_bytes(&[0; 10]), Err(NetworkError::PacketTooShort));
    }

    // ── Routing-Table ────────────────────────────────────────────────────────

    #[test]
    fn test_routing_exact() {
        let table = RoutingTable::new();
        table.add(Route {
            network: Ipv4Address::new(192, 168, 1, 0), prefix_len: 24,
            gateway: None, interface: "eth0".into(), metric: 100,
        });
        let route = table.lookup(Ipv4Address::new(192, 168, 1, 100)).unwrap();
        assert_eq!(route.prefix_len, 24);
        assert_eq!(route.interface, "eth0");
    }

    #[test]
    fn test_routing_longest_prefix() {
        let table = RoutingTable::new();
        table.add(Route { network: Ipv4Address::new(0,0,0,0), prefix_len: 0, gateway: Some(Ipv4Address::new(10,0,0,1)), interface: "eth0".into(), metric: 200 });
        table.add(Route { network: Ipv4Address::new(192,168,0,0), prefix_len: 16, gateway: None, interface: "eth1".into(), metric: 100 });
        table.add(Route { network: Ipv4Address::new(192,168,1,0), prefix_len: 24, gateway: None, interface: "eth2".into(), metric: 50 });
        let route = table.lookup(Ipv4Address::new(192, 168, 1, 50)).unwrap();
        assert_eq!(route.prefix_len, 24);
        assert_eq!(route.interface, "eth2");
    }

    #[test]
    fn test_routing_default_route() {
        let table = RoutingTable::new();
        table.add(Route { network: Ipv4Address::new(0,0,0,0), prefix_len: 0, gateway: Some(Ipv4Address::new(10,0,0,1)), interface: "eth0".into(), metric: 300 });
        let route = table.lookup(Ipv4Address::new(8, 8, 8, 8)).unwrap();
        assert_eq!(route.prefix_len, 0);
    }

    #[test]
    fn test_routing_no_match() {
        let table = RoutingTable::new();
        table.add(Route { network: Ipv4Address::new(192,168,1,0), prefix_len: 24, gateway: None, interface: "eth0".into(), metric: 100 });
        assert!(table.lookup(Ipv4Address::new(10, 0, 0, 1)).is_none());
    }

    // ── Socket-Manager: UDP ──────────────────────────────────────────────────

    #[test]
    fn test_udp_bind() {
        let mgr = SocketManager::new();
        let _id = mgr.udp_bind(Ipv4Address::new(10, 0, 0, 1), 8080);
        let (udp, tcp) = mgr.socket_count();
        assert_eq!(udp, 1);
        assert_eq!(tcp, 0);
    }

    #[test]
    fn test_udp_connect() {
        let mgr = SocketManager::new();
        let id = mgr.udp_bind(Ipv4Address::new(10, 0, 0, 1), 8080);
        mgr.udp_connect(id, Ipv4Address::new(10, 0, 0, 2), 9090).unwrap();
    }

    #[test]
    fn test_udp_recv_empty() {
        let mgr = SocketManager::new();
        let id = mgr.udp_bind(Ipv4Address::new(10, 0, 0, 1), 8080);
        assert_eq!(mgr.udp_recv(id), Err(NetworkError::NoFrameAvailable));
    }

    #[test]
    fn test_udp_handle_incoming() {
        let mgr = SocketManager::new();
        let id = mgr.udp_bind(Ipv4Address::new(10, 0, 0, 1), 8080);
        mgr.handle_udp(Ipv4Address::new(10, 0, 0, 2), 9090, 8080, vec![0xDE, 0xAD]);
        let data = mgr.udp_recv(id).unwrap();
        assert_eq!(data, vec![0xDE, 0xAD]);
    }

    #[test]
    fn test_udp_close() {
        let mgr = SocketManager::new();
        let id = mgr.udp_bind(Ipv4Address::new(10, 0, 0, 1), 8080);
        mgr.udp_close(id);
        let (udp, _) = mgr.socket_count();
        assert_eq!(udp, 0);
    }

    // ── Socket-Manager: TCP ──────────────────────────────────────────────────

    #[test]
    fn test_tcp_listen() {
        let mgr = SocketManager::new();
        let id = mgr.tcp_bind(Ipv4Address::new(10, 0, 0, 1), 80);
        assert_eq!(mgr.tcp_state(id), Some(TcpState::Listen));
    }

    #[test]
    fn test_tcp_connect_syn_sent() {
        let mgr = SocketManager::new();
        let id = mgr.tcp_bind(Ipv4Address::new(10, 0, 0, 1), 80);
        let state = mgr.tcp_connect(id, Ipv4Address::new(10, 0, 0, 2), 443).unwrap();
        assert_eq!(state, TcpState::SynSent);
    }

    #[test]
    fn test_tcp_handshake_listen_to_syn_received() {
        let mgr = SocketManager::new();
        let id = mgr.tcp_bind(Ipv4Address::new(10, 0, 0, 1), 80);
        let syn = TcpSegment::new(443, 80, 1000, 0, TCP_SYN, vec![]);
        mgr.handle_tcp(Ipv4Address::new(10, 0, 0, 2), 443, 80, &syn);
        assert_eq!(mgr.tcp_state(id), Some(TcpState::SynReceived));
    }

    #[test]
    fn test_tcp_data_reception() {
        let mgr = SocketManager::new();
        let id = mgr.tcp_bind(Ipv4Address::new(10, 0, 0, 1), 80);
        {
            let mut sockets = mgr.tcp_sockets.lock();
            if let Some(s) = sockets.get_mut(&id) {
                s.state = TcpState::Established;
                s.remote_ip = Some(Ipv4Address::new(10, 0, 0, 2));
                s.remote_port = Some(443);
            }
        }
        let data = TcpSegment::new(443, 80, 1001, 2001, TCP_ACK, vec![0x48, 0x49]);
        mgr.handle_tcp(Ipv4Address::new(10, 0, 0, 2), 443, 80, &data);
        let received = mgr.tcp_recv(id).unwrap();
        assert_eq!(received, vec![0x48, 0x49]);
    }

    #[test]
    fn test_tcp_fin_closes() {
        let mgr = SocketManager::new();
        let id = mgr.tcp_bind(Ipv4Address::new(10, 0, 0, 1), 80);
        {
            let mut sockets = mgr.tcp_sockets.lock();
            if let Some(s) = sockets.get_mut(&id) {
                s.state = TcpState::Established;
                s.remote_ip = Some(Ipv4Address::new(10, 0, 0, 2));
                s.remote_port = Some(443);
            }
        }
        let fin = TcpSegment::new(443, 80, 1000, 1000, TCP_FIN | TCP_ACK, vec![]);
        mgr.handle_tcp(Ipv4Address::new(10, 0, 0, 2), 443, 80, &fin);
        assert_eq!(mgr.tcp_state(id), Some(TcpState::CloseWait));
    }

    #[test]
    fn test_tcp_close() {
        let mgr = SocketManager::new();
        let id = mgr.tcp_bind(Ipv4Address::new(10, 0, 0, 1), 80);
        mgr.tcp_close(id);
        assert_eq!(mgr.tcp_state(id), None);
    }

    // ── IpStack Integration ──────────────────────────────────────────────────

    #[test]
    fn test_ipstack_handle_udp() {
        let dev = Arc::new(LoopbackDevice::new("lo0"));
        let net = Arc::new(NetworkStack::new(dev, Ipv4Address::new(10, 0, 0, 1)));
        let stack = IpStack::new(net);
        let sock_id = stack.sockets().udp_bind(Ipv4Address::new(10, 0, 0, 1), 8080);
        let udp = UdpPacket::new(9090, 8080, vec![0x42, 0x43]);
        let ip = Ipv4Packet::new(Ipv4Address::new(10, 0, 0, 2), Ipv4Address::new(10, 0, 0, 1), IP_PROTO_UDP, udp.to_bytes());
        stack.handle_ipv4(&ip);
        let data = stack.sockets().udp_recv(sock_id).unwrap();
        assert_eq!(data, vec![0x42, 0x43]);
    }

    #[test]
    fn test_ipstack_ignores_other_dst() {
        let dev = Arc::new(LoopbackDevice::new("lo0"));
        let net = Arc::new(NetworkStack::new(dev, Ipv4Address::new(10, 0, 0, 1)));
        let stack = IpStack::new(net);
        let sock_id = stack.sockets().udp_bind(Ipv4Address::new(10, 0, 0, 1), 8080);
        let udp = UdpPacket::new(9090, 8080, vec![0x42]);
        let ip = Ipv4Packet::new(Ipv4Address::new(10, 0, 0, 2), Ipv4Address::new(10, 0, 0, 99), IP_PROTO_UDP, udp.to_bytes());
        stack.handle_ipv4(&ip);
        assert_eq!(stack.sockets().udp_recv(sock_id), Err(NetworkError::NoFrameAvailable));
    }

    #[test]
    fn test_ipstack_handle_tcp_syn() {
        let dev = Arc::new(LoopbackDevice::new("lo0"));
        let net = Arc::new(NetworkStack::new(dev, Ipv4Address::new(10, 0, 0, 1)));
        let stack = IpStack::new(net);
        let sock_id = stack.sockets().tcp_bind(Ipv4Address::new(10, 0, 0, 1), 80);
        let tcp = TcpSegment::new(443, 80, 1000, 0, TCP_SYN, vec![]);
        let ip = Ipv4Packet::new(Ipv4Address::new(10, 0, 0, 2), Ipv4Address::new(10, 0, 0, 1), IP_PROTO_TCP, tcp.to_bytes());
        stack.handle_ipv4(&ip);
        assert_eq!(stack.sockets().tcp_state(sock_id), Some(TcpState::SynReceived));
    }

    #[test]
    fn test_ipstack_full_frame_dispatch() {
        let dev = Arc::new(LoopbackDevice::new("lo0"));
        let net = Arc::new(NetworkStack::new(dev, Ipv4Address::new(10, 0, 0, 1)));
        let stack = IpStack::new(net);
        let sock_id = stack.sockets().udp_bind(Ipv4Address::new(10, 0, 0, 1), 5000);
        let udp = UdpPacket::new(4000, 5000, vec![0xAA, 0xBB]);
        let ip = Ipv4Packet::new(Ipv4Address::new(10, 0, 0, 2), Ipv4Address::new(10, 0, 0, 1), IP_PROTO_UDP, udp.to_bytes());
        let frame = EthernetFrame::new(stack.net().device.mac_address(), MacAddress::new(0x11, 0x22, 0x33, 0x44, 0x55, 0x66), ETH_TYPE_IPV4, ip.to_bytes());
        stack.handle_frame(&frame.to_bytes(), 0).unwrap();
        let data = stack.sockets().udp_recv(sock_id).unwrap();
        assert_eq!(data, vec![0xAA, 0xBB]);
    }
}
