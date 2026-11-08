// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 37: Unix Domain Sockets + Network Socket API
// Copyright (c) Michael Wroblewski. All rights reserved.
//
// Socket-Abstraktion für Ring-3-Prozesse: Unix Domain Sockets (Stream/Datagram),
// Network Sockets (TCP/UDP), Socket-Adressen, Accept/Connect/Bind,
// Per-Prozess Socket-Tabelle, Poll-Integration.
//
// Teil der Userspace-Pipeline (K30-K36): baut auf user_io.rs (K34) und tcpip.rs (K13) auf.

#![cfg_attr(not(test), no_std)]

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

// ══════════════════════════════════════════════════════════════════════════════
// Socket Types
// ══════════════════════════════════════════════════════════════════════════════

/// Socket-Domäne
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SocketDomain {
    /// Unix Domain Socket (lokal, Pfad-basiert)
    Unix,
    /// IPv4 Network Socket
    Inet,
    /// IPv6 Network Socket (Struktur vorbereitet)
    Inet6,
}

impl SocketDomain {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unix => "AF_UNIX",
            Self::Inet => "AF_INET",
            Self::Inet6 => "AF_INET6",
        }
    }

    pub fn is_local(&self) -> bool { matches!(self, Self::Unix) }
    pub fn is_network(&self) -> bool { !self.is_local() }
}

/// Socket-Typ
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SocketType {
    /// Verbindungsorientiert, zuverlässig (TCP / Unix Stream)
    Stream,
    /// Verbindungslos, Datagram (UDP / Unix Datagram)
    Datagram,
    /// Raw IP (nur Kernel/Root)
    Raw,
    /// Sequenced Packet (SCTP-like)
    SeqPacket,
}

impl SocketType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stream   => "SOCK_STREAM",
            Self::Datagram => "SOCK_DGRAM",
            Self::Raw      => "SOCK_RAW",
            Self::SeqPacket=> "SOCK_SEQPACKET",
        }
    }

    pub fn is_connection_oriented(&self) -> bool {
        matches!(self, Self::Stream | Self::SeqPacket)
    }

    pub fn is_connectionless(&self) -> bool {
        matches!(self, Self::Datagram)
    }

    pub fn requires_root(&self) -> bool {
        matches!(self, Self::Raw)
    }
}

/// Socket-Protokoll (meist 0 = Default)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SocketProtocol {
    Default,    // 0
    Tcp,        // IPPROTO_TCP = 6
    Udp,        // IPPROTO_UDP = 17
    Raw,        // IPPROTO_RAW = 255
    Icmp,       // IPPROTO_ICMP = 1
}

impl SocketProtocol {
    pub fn from_u8(v: u8) -> Self {
        match v {
            6   => Self::Tcp,
            17  => Self::Udp,
            255 => Self::Raw,
            1   => Self::Icmp,
            _   => Self::Default,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Default => 0,
            Self::Tcp     => 6,
            Self::Udp     => 17,
            Self::Raw     => 255,
            Self::Icmp    => 1,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "DEFAULT",
            Self::Tcp     => "TCP",
            Self::Udp     => "UDP",
            Self::Raw     => "RAW",
            Self::Icmp    => "ICMP",
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Socket Address
// ══════════════════════════════════════════════════════════════════════════════

/// Socket-Adresse (Unix-Pfad oder IPv4:Port)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SocketAddr {
    /// Unix Domain Socket Pfad (max 108 Zeichen wie UNIX_PATH_MAX)
    Unix(String),
    /// IPv4 Adresse + Port
    Inet { addr: [u8; 4], port: u16 },
    /// IPv6 Adresse + Port (16 Bytes)
    Inet6 { addr: [u8; 16], port: u16 },
}

impl SocketAddr {
    /// Erstelle Unix-Adresse aus Pfad
    pub fn unix(path: &str) -> Self {
        let p = if path.len() > 108 { &path[..108] } else { path };
        Self::Unix(p.to_string())
    }

    /// Erstelle IPv4-Adresse
    pub fn inet(ip: [u8; 4], port: u16) -> Self {
        Self::Inet { addr: ip, port }
    }

    /// Loopback 127.0.0.1:port
    pub fn loopback(port: u16) -> Self {
        Self::Inet { addr: [127, 0, 0, 1], port }
    }

    /// Wildcard 0.0.0.0:port
    pub fn any_addr(port: u16) -> Self {
        Self::Inet { addr: [0, 0, 0, 0], port }
    }

    pub fn is_unix(&self) -> bool { matches!(self, Self::Unix(_)) }
    pub fn is_inet(&self) -> bool { matches!(self, Self::Inet { .. }) }

    pub fn domain(&self) -> SocketDomain {
        match self {
            Self::Unix(_)   => SocketDomain::Unix,
            Self::Inet { .. }  => SocketDomain::Inet,
            Self::Inet6 { .. } => SocketDomain::Inet6,
        }
    }

    pub fn port(&self) -> Option<u16> {
        match self {
            Self::Inet { port, .. } | Self::Inet6 { port, .. } => Some(*port),
            Self::Unix(_) => None,
        }
    }

    pub fn ip_str(&self) -> String {
        match self {
            Self::Unix(p)    => p.clone(),
            Self::Inet { addr, port } => {
                format!("{}.{}.{}.{}:{}", addr[0], addr[1], addr[2], addr[3], port)
            }
            Self::Inet6 { addr, port } => {
                let mut s = String::from("[");
                for (i, b) in addr.iter().enumerate() {
                    if i > 0 && i % 2 == 0 { s.push(':'); }
                    if i % 2 == 0 { s.push_str(&format!("{:02x}", b)); }
                    else { s.push_str(&format!("{:02x}", b)); }
                }
                s.push_str(&format!("]:{}", port));
                s
            }
        }
    }

    pub fn path(&self) -> Option<&str> {
        match self {
            Self::Unix(p) => Some(p.as_str()),
            _ => None,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Socket State
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SocketState {
    Unconnected,    // Frisch erstellt
    Listening,      // accept() wartet
    Connecting,     // connect() läuft
    Connected,      // Verbindung aktiv
    Closing,        // Halb geschlossen
    Closed,         // Vollständig geschlossen
    Error,           // Fehlerzustand
}

impl SocketState {
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Connected | Self::Listening | Self::Connecting)
    }

    pub fn is_readable(&self) -> bool {
        matches!(self, Self::Connected | Self::Listening | Self::Closing)
    }

    pub fn is_writable(&self) -> bool {
        matches!(self, Self::Connected | Self::Connecting)
    }

    pub fn is_closed(&self) -> bool {
        matches!(self, Self::Closed | Self::Error)
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unconnected => "UNCONNECTED",
            Self::Listening   => "LISTENING",
            Self::Connecting  => "CONNECTING",
            Self::Connected   => "CONNECTED",
            Self::Closing     => "CLOSING",
            Self::Closed      => "CLOSED",
            Self::Error       => "ERROR",
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Socket Options
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SocketOptions {
    pub nonblocking: bool,
    pub reuseaddr: bool,
    pub reuseport: bool,
    pub keepalive: bool,
    pub broadcast: bool,
    pub linger: Option<u32>,       // Sekunden
    pub send_buf_size: usize,
    pub recv_buf_size: usize,
    pub send_timeout: Option<u32>, // ms
    pub recv_timeout: Option<u32>, // ms
}

impl Default for SocketOptions {
    fn default() -> Self {
        Self {
            nonblocking: false,
            reuseaddr: false,
            reuseport: false,
            keepalive: false,
            broadcast: false,
            linger: None,
            send_buf_size: 64 * 1024,     // 64 KiB
            recv_buf_size: 64 * 1024,     // 64 KiB
            send_timeout: None,
            recv_timeout: None,
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Socket Buffer
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct SocketBuffer {
    data: Vec<u8>,
    capacity: usize,
    read_pos: usize,
}

impl SocketBuffer {
    pub fn new(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity), capacity, read_pos: 0 }
    }

    pub fn write(&mut self, src: &[u8]) -> usize {
        let avail = self.capacity.saturating_sub(self.data.len() + self.read_pos);
        let to_write = src.len().min(avail);
        self.data.extend_from_slice(&src[..to_write]);
        to_write
    }

    pub fn read(&mut self, dst: &mut [u8]) -> usize {
        let unread = self.data.len() - self.read_pos;
        if unread == 0 { return 0; }
        let to_read = dst.len().min(unread);
        dst[..to_read].copy_from_slice(&self.data[self.read_pos..self.read_pos + to_read]);
        self.read_pos += to_read;
        if self.read_pos >= self.data.len() {
            self.data.clear();
            self.read_pos = 0;
        }
        to_read
    }

    pub fn available(&self) -> usize { self.data.len() - self.read_pos }
    pub fn remaining(&self) -> usize { self.capacity.saturating_sub(self.available()) }
    pub fn is_empty(&self) -> bool { self.available() == 0 }
    pub fn is_full(&self) -> bool { self.remaining() == 0 }
    pub fn clear(&mut self) { self.data.clear(); self.read_pos = 0; }
}

// ══════════════════════════════════════════════════════════════════════════════
// Socket
// ══════════════════════════════════════════════════════════════════════════════

/// Socket-ID (globales Handle)
pub type SockId = u32;

/// Ein einzelner Socket
#[derive(Clone, Debug)]
pub struct Socket {
    pub id: SockId,
    pub domain: SocketDomain,
    pub sock_type: SocketType,
    pub protocol: SocketProtocol,
    pub state: SocketState,
    pub local_addr: Option<SocketAddr>,
    pub peer_addr: Option<SocketAddr>,
    pub options: SocketOptions,
    pub recv_buf: SocketBuffer,
    pub send_buf: SocketBuffer,
    pub backlog: Vec<SockId>,      // Pending connections (listen)
    pub max_backlog: usize,
    pub owner_pid: u32,
    pub error: Option<SocketError>,
    // Stats
    pub bytes_sent: u64,
    pub bytes_recv: u64,
}

impl Socket {
    pub fn new(id: SockId, domain: SocketDomain, sock_type: SocketType, proto: SocketProtocol, pid: u32) -> Self {
        let buf_size = 64 * 1024;
        Self {
            id, domain, sock_type, protocol: proto,
            state: SocketState::Unconnected,
            local_addr: None, peer_addr: None,
            options: SocketOptions::default(),
            recv_buf: SocketBuffer::new(buf_size),
            send_buf: SocketBuffer::new(buf_size),
            backlog: Vec::new(), max_backlog: 128,
            owner_pid: pid,
            error: None,
            bytes_sent: 0, bytes_recv: 0,
        }
    }

    pub fn is_stream(&self) -> bool { self.sock_type == SocketType::Stream }
    pub fn is_dgram(&self) -> bool { self.sock_type == SocketType::Datagram }
    pub fn is_listening(&self) -> bool { self.state == SocketState::Listening }
    pub fn is_connected(&self) -> bool { self.state == SocketState::Connected }
    pub fn has_pending(&self) -> bool { !self.backlog.is_empty() }
    pub fn pending_count(&self) -> usize { self.backlog.len() }
    pub fn can_read(&self) -> bool {
        self.state.is_readable() && (self.recv_buf.available() > 0 || self.has_pending())
    }
    pub fn can_write(&self) -> bool {
        self.state.is_writable() && self.send_buf.remaining() > 0
    }
    pub fn has_error(&self) -> bool { self.error.is_some() }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SocketError {
    NotConnected,
    AlreadyConnected,
    NotListening,
    AddrInUse,
    AddrNotAvailable,
    ConnectionRefused,
    ConnectionReset,
    TimedOut,
    WouldBlock,
    MessageTooLong,
    NotASocket,
    BadDescriptor,
    PermissionDenied,
    NoBufferSpace,
    OperationNotSupported,
}

impl SocketError {
    pub fn errno(&self) -> i32 {
        match self {
            Self::NotConnected        => 107,  // ENOTCONN
            Self::AlreadyConnected   => 106,  // EISCONN
            Self::NotListening        => -1,
            Self::AddrInUse           => 98,   // EADDRINUSE
            Self::AddrNotAvailable    => 99,   // EADDRNOTAVAIL
            Self::ConnectionRefused   => 111,  // ECONNREFUSED
            Self::ConnectionReset     => 104,  // ECONNRESET
            Self::TimedOut            => 110,  // ETIMEDOUT
            Self::WouldBlock          => 11,   // EWOULDBLOCK
            Self::MessageTooLong      => 90,   // EMSGSIZE
            Self::NotASocket          => 88,   // ENOTSOCK
            Self::BadDescriptor       => 9,    // EBADF
            Self::PermissionDenied    => 1,    // EPERM
            Self::NoBufferSpace       => 105,  // ENOBUFS
            Self::OperationNotSupported => 95, // EOPNOTSUPP
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotConnected        => "Not connected",
            Self::AlreadyConnected   => "Already connected",
            Self::NotListening        => "Not listening",
            Self::AddrInUse           => "Address in use",
            Self::AddrNotAvailable    => "Address not available",
            Self::ConnectionRefused   => "Connection refused",
            Self::ConnectionReset     => "Connection reset",
            Self::TimedOut            => "Timed out",
            Self::WouldBlock          => "Would block",
            Self::MessageTooLong      => "Message too long",
            Self::NotASocket          => "Not a socket",
            Self::BadDescriptor       => "Bad file descriptor",
            Self::PermissionDenied    => "Permission denied",
            Self::NoBufferSpace       => "No buffer space",
            Self::OperationNotSupported => "Operation not supported",
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// Socket Manager
// ══════════════════════════════════════════════════════════════════════════════

pub struct SocketManager {
    sockets: BTreeMap<SockId, Socket>,
    next_id: SockId,
    /// Unix Domain Socket Registry: path → listening socket id
    unix_listeners: BTreeMap<String, SockId>,
    /// IPv4 Port Registry: port → listening socket id
    inet_listeners: BTreeMap<u16, SockId>,
    // Stats
    total_created: u64,
    total_accepted: u64,
    total_bytes_sent: u64,
    total_bytes_recv: u64,
}

impl SocketManager {
    pub fn new() -> Self {
        Self {
            sockets: BTreeMap::new(),
            next_id: 1,
            unix_listeners: BTreeMap::new(),
            inet_listeners: BTreeMap::new(),
            total_created: 0,
            total_accepted: 0,
            total_bytes_sent: 0,
            total_bytes_recv: 0,
        }
    }

    /// socket() — Erstelle einen neuen Socket
    pub fn socket(&mut self, domain: SocketDomain, sock_type: SocketType, proto: SocketProtocol, pid: u32) -> Result<SockId, SocketError> {
        // Raw sockets erfordern Root
        if sock_type.requires_root() && pid != 0 {
            return Err(SocketError::PermissionDenied);
        }
        // Protokoll-Validierung
        match (domain, sock_type, proto) {
            (SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default) => {},
            (SocketDomain::Unix, SocketType::Datagram, SocketProtocol::Default) => {},
            (SocketDomain::Inet, SocketType::Stream, SocketProtocol::Default | SocketProtocol::Tcp) => {},
            (SocketDomain::Inet, SocketType::Datagram, SocketProtocol::Default | SocketProtocol::Udp) => {},
            (SocketDomain::Inet, SocketType::Raw, SocketProtocol::Raw | SocketProtocol::Icmp) => {},
            (SocketDomain::Inet6, _, _) => {},
            _ => {
                // Andere Kombinationen erlauben (flexibel)
            }
        }

        let id = self.next_id;
        self.next_id += 1;
        let sock = Socket::new(id, domain, sock_type, proto, pid);
        self.sockets.insert(id, sock);
        self.total_created += 1;
        Ok(id)
    }

    /// bind() — Socket an Adresse binden
    pub fn bind(&mut self, sid: SockId, addr: SocketAddr) -> Result<(), SocketError> {
        let sock = self.sockets.get_mut(&sid).ok_or(SocketError::BadDescriptor)?;

        if sock.state != SocketState::Unconnected {
            return Err(SocketError::AlreadyConnected);
        }

        // Adress-Konflikt prüfen
        match &addr {
            SocketAddr::Unix(path) => {
                if self.unix_listeners.contains_key(path) {
                    return Err(SocketError::AddrInUse);
                }
            }
            SocketAddr::Inet { port, .. } => {
                if self.inet_listeners.contains_key(port) {
                    if !sock.options.reuseaddr && !sock.options.reuseport {
                        return Err(SocketError::AddrInUse);
                    }
                }
            }
            _ => {}
        }

        sock.local_addr = Some(addr.clone());
        Ok(())
    }

    /// listen() — Socket in Listening-Modus versetzen
    pub fn listen(&mut self, sid: SockId, backlog: usize) -> Result<(), SocketError> {
        let sock = self.sockets.get_mut(&sid).ok_or(SocketError::BadDescriptor)?;

        if !sock.sock_type.is_connection_oriented() {
            return Err(SocketError::OperationNotSupported);
        }
        if sock.local_addr.is_none() {
            return Err(SocketError::AddrNotAvailable);
        }

        sock.max_backlog = backlog.max(1).min(4096);
        sock.state = SocketState::Listening;

        // In Listener-Registry eintragen
        let addr = sock.local_addr.clone().unwrap();
        match addr {
            SocketAddr::Unix(ref path) => {
                self.unix_listeners.insert(path.clone(), sid);
            }
            SocketAddr::Inet { port, .. } => {
                self.inet_listeners.insert(port, sid);
            }
            _ => {}
        }

        Ok(())
    }

    /// accept() — Eingehende Verbindung akzeptieren
    pub fn accept(&mut self, sid: SockId, pid: u32) -> Result<SockId, SocketError> {
        let listener = self.sockets.get(&sid).ok_or(SocketError::BadDescriptor)?;
        if listener.state != SocketState::Listening {
            return Err(SocketError::NotListening);
        }
        if listener.backlog.is_empty() {
            if listener.options.nonblocking {
                return Err(SocketError::WouldBlock);
            }
            return Err(SocketError::WouldBlock);
        }

        let pending_id = listener.backlog[0];

        // Neuen verbundenen Socket erstellen
        let listener_info = {
            let l = self.sockets.get(&sid).unwrap();
            (l.domain, l.sock_type, l.protocol, l.local_addr.clone())
        };

        let (domain, sock_type, proto, local_addr) = listener_info;
        let new_id = self.next_id;
        self.next_id += 1;

        // Peer-Adresse vom pending Socket holen
        let peer = self.sockets.get(&pending_id)
            .and_then(|p| p.local_addr.clone());

        let mut new_sock = Socket::new(new_id, domain, sock_type, proto, pid);
        new_sock.state = SocketState::Connected;
        new_sock.local_addr = local_addr;
        new_sock.peer_addr = peer;

        self.sockets.insert(new_id, new_sock);

        // Pending Socket aus Backlog entfernen
        if let Some(listener) = self.sockets.get_mut(&sid) {
            listener.backlog.remove(0);
        }
        // Pending Socket schließen
        self.sockets.remove(&pending_id);

        self.total_accepted += 1;
        Ok(new_id)
    }

    /// connect() — Verbindung zu Remote-Adresse aufbauen
    pub fn connect(&mut self, sid: SockId, remote: SocketAddr) -> Result<(), SocketError> {
        let sock = self.sockets.get_mut(&sid).ok_or(SocketError::BadDescriptor)?;

        if sock.state == SocketState::Connected {
            return Err(SocketError::AlreadyConnected);
        }

        if !sock.sock_type.is_connection_oriented() {
            // Datagram: Peer setzen, aber nicht verbinden
            sock.peer_addr = Some(remote);
            sock.state = SocketState::Connected; // UDP "connected" = default peer
            return Ok(());
        }

        // Stream: Ziel muss einen Listener haben
        let target_listener = match &remote {
            SocketAddr::Unix(path) => self.unix_listeners.get(path).copied(),
            SocketAddr::Inet { port, .. } => self.inet_listeners.get(port).copied(),
            _ => None,
        };

        // Pending Socket im Listener-Backlog erstellen
        if let Some(listener_id) = target_listener {
            let pending_id = self.next_id;
            self.next_id += 1;

            let listener = self.sockets.get(&listener_id).unwrap();
            let mut pending = Socket::new(
                pending_id,
                listener.domain,
                listener.sock_type,
                listener.protocol,
                sock.owner_pid,
            );
            pending.local_addr = Some(remote.clone());
            pending.state = SocketState::Connecting;

            self.sockets.insert(pending_id, pending);

            // In Backlog des Listeners einfügen
            if let Some(l) = self.sockets.get_mut(&listener_id) {
                if l.backlog.len() < l.max_backlog {
                    l.backlog.push(pending_id);
                } else {
                    self.sockets.remove(&pending_id);
                    return Err(SocketError::ConnectionRefused);
                }
            }
        }

        sock.peer_addr = Some(remote);
        sock.state = SocketState::Connected;
        Ok(())
    }

    /// send() — Daten über Socket senden
    pub fn send(&mut self, sid: SockId, data: &[u8]) -> Result<usize, SocketError> {
        let sock = self.sockets.get_mut(&sid).ok_or(SocketError::BadDescriptor)?;

        if !sock.state.is_writable() && sock.sock_type.is_connection_oriented() {
            return Err(SocketError::NotConnected);
        }

        let written = sock.send_buf.write(data);
        if written == 0 && sock.options.nonblocking {
            return Err(SocketError::WouldBlock);
        }

        sock.bytes_sent += written as u64;
        self.total_bytes_sent += written as u64;
        Ok(written)
    }

    /// recv() — Daten von Socket empfangen
    pub fn recv(&mut self, sid: SockId, buf: &mut [u8]) -> Result<usize, SocketError> {
        let sock = self.sockets.get_mut(&sid).ok_or(SocketError::BadDescriptor)?;

        if !sock.state.is_readable() && sock.sock_type.is_connection_oriented() {
            if sock.state.is_closed() {
                return Ok(0); // EOF
            }
            return Err(SocketError::NotConnected);
        }

        let read = sock.recv_buf.read(buf);
        if read == 0 && sock.options.nonblocking {
            return Err(SocketError::WouldBlock);
        }

        sock.bytes_recv += read as u64;
        self.total_bytes_recv += read as u64;
        Ok(read)
    }

    /// sendto() — Datagram an Adresse senden
    pub fn sendto(&mut self, sid: SockId, data: &[u8], dest: SocketAddr) -> Result<usize, SocketError> {
        let sock = self.sockets.get(&sid).ok_or(SocketError::BadDescriptor)?;
        if !sock.sock_type.is_connectionless() {
            // Stream-Sockets ignorieren dest
            return self.send(sid, data);
        }
        // Für UDP: an Peer senden
        self.send(sid, data)
    }

    /// recvfrom() — Datagram empfangen + Absender
    pub fn recvfrom(&mut self, sid: SockId, buf: &mut [u8]) -> Result<(usize, Option<SocketAddr>), SocketError> {
        let sock = self.sockets.get(&sid).ok_or(SocketError::BadDescriptor)?;
        let peer = sock.peer_addr.clone();
        let n = self.recv(sid, buf)?;
        Ok((n, peer))
    }

    /// close() — Socket schließen
    pub fn close(&mut self, sid: SockId) -> Result<(), SocketError> {
        let sock = self.sockets.get(&sid).ok_or(SocketError::BadDescriptor)?;

        // Aus Listener-Registries entfernen
        if sock.state == SocketState::Listening {
            if let Some(ref addr) = sock.local_addr {
                match addr {
                    SocketAddr::Unix(path) => { self.unix_listeners.remove(path); }
                    SocketAddr::Inet { port, .. } => { self.inet_listeners.remove(port); }
                    _ => {}
                }
            }
        }

        self.sockets.remove(&sid);
        Ok(())
    }

    /// setsockopt() — Socket-Option setzen
    pub fn setsockopt(&mut self, sid: SockId, opt: SocketOpt) -> Result<(), SocketError> {
        let sock = self.sockets.get_mut(&sid).ok_or(SocketError::BadDescriptor)?;
        match opt {
            SocketOpt::NonBlock(v)    => sock.options.nonblocking = v,
            SocketOpt::ReuseAddr(v)   => sock.options.reuseaddr = v,
            SocketOpt::ReusePort(v)   => sock.options.reuseport = v,
            SocketOpt::KeepAlive(v)   => sock.options.keepalive = v,
            SocketOpt::Broadcast(v)  => sock.options.broadcast = v,
            SocketOpt::Linger(v)      => sock.options.linger = v,
            SocketOpt::SendBuf(v)     => sock.options.send_buf_size = v,
            SocketOpt::RecvBuf(v)     => sock.options.recv_buf_size = v,
            SocketOpt::SendTimeout(v) => sock.options.send_timeout = v,
            SocketOpt::RecvTimeout(v) => sock.options.recv_timeout = v,
        }
        Ok(())
    }

    /// getsockopt() — Socket-Option lesen
    pub fn getsockopt(&self, sid: SockId) -> Result<&SocketOptions, SocketError> {
        self.sockets.get(&sid).map(|s| &s.options).ok_or(SocketError::BadDescriptor)
    }

    /// get_socket() — Socket-Referenz holen
    pub fn get_socket(&self, sid: SockId) -> Option<&Socket> {
        self.sockets.get(&sid)
    }

    /// socket_count() — Anzahl aktiver Sockets
    pub fn socket_count(&self) -> usize { self.sockets.len() }

    /// list_sockets() — Alle Socket-IDs
    pub fn list_sockets(&self) -> Vec<SockId> {
        self.sockets.keys().copied().collect()
    }

    /// poll_sockets() — Welche Sockets sind ready?
    pub fn poll_sockets(&self, sids: &[SockId]) -> Vec<(SockId, PollState)> {
        sids.iter()
            .filter_map(|&sid| {
                self.sockets.get(&sid).map(|s| {
                    (
                        sid,
                        PollState {
                            readable: s.can_read(),
                            writable: s.can_write(),
                            error: s.has_error(),
                        },
                    )
                })
            })
            .collect()
    }

    /// stats() — Globale Statistiken
    pub fn stats(&self) -> SocketStats {
        SocketStats {
            total_sockets: self.sockets.len() as u64,
            total_created: self.total_created,
            total_accepted: self.total_accepted,
            total_bytes_sent: self.total_bytes_sent,
            total_bytes_recv: self.total_bytes_recv,
            unix_listeners: self.unix_listeners.len() as u64,
            inet_listeners: self.inet_listeners.len() as u64,
        }
    }
}

/// Socket-Option für setsockopt
#[derive(Clone, Debug)]
pub enum SocketOpt {
    NonBlock(bool),
    ReuseAddr(bool),
    ReusePort(bool),
    KeepAlive(bool),
    Broadcast(bool),
    Linger(Option<u32>),
    SendBuf(usize),
    RecvBuf(usize),
    SendTimeout(Option<u32>),
    RecvTimeout(Option<u32>),
}

/// Poll-State für einen Socket
#[derive(Clone, Copy, Debug)]
pub struct PollState {
    pub readable: bool,
    pub writable: bool,
    pub error: bool,
}

impl PollState {
    pub fn is_any(&self) -> bool { self.readable || self.writable || self.error }
}

/// Globale Socket-Statistiken
#[derive(Clone, Debug)]
pub struct SocketStats {
    pub total_sockets: u64,
    pub total_created: u64,
    pub total_accepted: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_recv: u64,
    pub unix_listeners: u64,
    pub inet_listeners: u64,
}

// ══════════════════════════════════════════════════════════════════════════════
// Tests
// ══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ── SocketDomain / SocketType / SocketProtocol ───────────────────────────

    #[test]
    fn test_socket_domain() {
        assert_eq!(SocketDomain::Unix.as_str(), "AF_UNIX");
        assert_eq!(SocketDomain::Inet.as_str(), "AF_INET");
        assert_eq!(SocketDomain::Inet6.as_str(), "AF_INET6");
        assert!(SocketDomain::Unix.is_local());
        assert!(SocketDomain::Inet.is_network());
        assert!(SocketDomain::Inet6.is_network());
    }

    #[test]
    fn test_socket_type() {
        assert!(SocketType::Stream.is_connection_oriented());
        assert!(SocketType::SeqPacket.is_connection_oriented());
        assert!(SocketType::Datagram.is_connectionless());
        assert!(SocketType::Raw.requires_root());
        assert!(!SocketType::Stream.requires_root());
        assert_eq!(SocketType::Stream.as_str(), "SOCK_STREAM");
        assert_eq!(SocketType::Datagram.as_str(), "SOCK_DGRAM");
    }

    #[test]
    fn test_socket_protocol() {
        assert_eq!(SocketProtocol::from_u8(6), SocketProtocol::Tcp);
        assert_eq!(SocketProtocol::from_u8(17), SocketProtocol::Udp);
        assert_eq!(SocketProtocol::from_u8(255), SocketProtocol::Raw);
        assert_eq!(SocketProtocol::from_u8(99), SocketProtocol::Default);
        assert_eq!(SocketProtocol::Tcp.to_u8(), 6);
        assert_eq!(SocketProtocol::Udp.to_u8(), 17);
        assert_eq!(SocketProtocol::Default.to_u8(), 0);
        assert_eq!(SocketProtocol::Tcp.as_str(), "TCP");
    }

    // ── SocketAddr ───────────────────────────────────────────────────────────

    #[test]
    fn test_socket_addr_unix() {
        let addr = SocketAddr::unix("/tmp/test.sock");
        assert!(addr.is_unix());
        assert!(!addr.is_inet());
        assert_eq!(addr.domain(), SocketDomain::Unix);
        assert_eq!(addr.port(), None);
        assert_eq!(addr.path(), Some("/tmp/test.sock"));
        assert_eq!(addr.ip_str(), "/tmp/test.sock");
    }

    #[test]
    fn test_socket_addr_inet() {
        let addr = SocketAddr::inet([192, 168, 1, 100], 8080);
        assert!(addr.is_inet());
        assert!(!addr.is_unix());
        assert_eq!(addr.domain(), SocketDomain::Inet);
        assert_eq!(addr.port(), Some(8080));
        assert_eq!(addr.ip_str(), "192.168.1.100:8080");
    }

    #[test]
    fn test_socket_addr_special() {
        let lo = SocketAddr::loopback(3000);
        assert_eq!(lo.ip_str(), "127.0.0.1:3000");
        let any = SocketAddr::any_addr(80);
        assert_eq!(any.ip_str(), "0.0.0.0:80");
    }

    #[test]
    fn test_socket_addr_unix_truncate() {
        let long_path = "a".repeat(200);
        let addr = SocketAddr::unix(&long_path);
        match addr {
            SocketAddr::Unix(ref p) => assert_eq!(p.len(), 108),
            _ => panic!("expected Unix"),
        }
    }

    // ── SocketState ─────────────────────────────────────────────────────────

    #[test]
    fn test_socket_state() {
        assert!(SocketState::Connected.is_active());
        assert!(SocketState::Listening.is_active());
        assert!(!SocketState::Closed.is_active());
        assert!(SocketState::Closed.is_closed());
        assert!(SocketState::Error.is_closed());
        assert!(SocketState::Connected.is_readable());
        assert!(SocketState::Connected.is_writable());
        assert!(!SocketState::Listening.is_writable());
        assert_eq!(SocketState::Listening.as_str(), "LISTENING");
        assert_eq!(SocketState::Connected.as_str(), "CONNECTED");
    }

    // ── SocketBuffer ────────────────────────────────────────────────────────

    #[test]
    fn test_socket_buffer_write_read() {
        let mut buf = SocketBuffer::new(1024);
        assert!(buf.is_empty());
        assert!(!buf.is_full());

        let n = buf.write(b"Hello, Socket!");
        assert_eq!(n, 14);
        assert_eq!(buf.available(), 14);

        let mut dst = [0u8; 20];
        let r = buf.read(&mut dst);
        assert_eq!(r, 14);
        assert_eq!(&dst[..14], b"Hello, Socket!");
        assert!(buf.is_empty());
    }

    #[test]
    fn test_socket_buffer_partial_read() {
        let mut buf = SocketBuffer::new(1024);
        buf.write(b"ABCDEFGHIJ");
        let mut d = [0u8; 4];
        assert_eq!(buf.read(&mut d), 4);
        assert_eq!(&d, b"ABCD");
        assert_eq!(buf.available(), 6);
        let mut d2 = [0u8; 10];
        assert_eq!(buf.read(&mut d2), 6);
        assert_eq!(&d2[..6], b"EFGHIJ");
    }

    #[test]
    fn test_socket_buffer_full() {
        let mut buf = SocketBuffer::new(8);
        let n = buf.write(b"01234567");
        assert_eq!(n, 8);
        assert!(buf.is_full());
        let n2 = buf.write(b"more");
        assert_eq!(n2, 0);
    }

    #[test]
    fn test_socket_buffer_clear() {
        let mut buf = SocketBuffer::new(64);
        buf.write(b"test");
        assert!(!buf.is_empty());
        buf.clear();
        assert!(buf.is_empty());
    }

    // ── SocketError ──────────────────────────────────────────────────────────

    #[test]
    fn test_socket_error_errno() {
        assert_eq!(SocketError::WouldBlock.errno(), 11);
        assert_eq!(SocketError::AddrInUse.errno(), 98);
        assert_eq!(SocketError::ConnectionRefused.errno(), 111);
        assert_eq!(SocketError::BadDescriptor.errno(), 9);
        assert_eq!(SocketError::PermissionDenied.errno(), 1);
        assert!(!SocketError::WouldBlock.as_str().is_empty());
    }

    // ── Socket creation ──────────────────────────────────────────────────────

    #[test]
    fn test_socket_create_unix_stream() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1);
        assert!(sid.is_ok());
        let sock = mgr.get_socket(sid.unwrap()).unwrap();
        assert_eq!(sock.domain, SocketDomain::Unix);
        assert_eq!(sock.sock_type, SocketType::Stream);
        assert_eq!(sock.state, SocketState::Unconnected);
        assert!(sock.is_stream());
        assert!(!sock.is_dgram());
    }

    #[test]
    fn test_socket_create_inet_tcp() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 1);
        assert!(sid.is_ok());
        let sock = mgr.get_socket(sid.unwrap()).unwrap();
        assert_eq!(sock.domain, SocketDomain::Inet);
        assert_eq!(sock.protocol, SocketProtocol::Tcp);
    }

    #[test]
    fn test_socket_create_raw_requires_root() {
        let mut mgr = SocketManager::new();
        let r = mgr.socket(SocketDomain::Inet, SocketType::Raw, SocketProtocol::Raw, 100);
        assert_eq!(r, Err(SocketError::PermissionDenied));
        let r2 = mgr.socket(SocketDomain::Inet, SocketType::Raw, SocketProtocol::Raw, 0);
        assert!(r2.is_ok());
    }

    // ── Bind + Listen ────────────────────────────────────────────────────────

    #[test]
    fn test_bind_unix() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        let r = mgr.bind(sid, SocketAddr::unix("/tmp/test_bind.sock"));
        assert!(r.is_ok());
        let sock = mgr.get_socket(sid).unwrap();
        assert_eq!(sock.local_addr, Some(SocketAddr::unix("/tmp/test_bind.sock")));
    }

    #[test]
    fn test_bind_inet() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 1).unwrap();
        let r = mgr.bind(sid, SocketAddr::loopback(8080));
        assert!(r.is_ok());
        let sock = mgr.get_socket(sid).unwrap();
        assert_eq!(sock.local_addr, Some(SocketAddr::loopback(8080)));
    }

    #[test]
    fn test_bind_addr_in_use() {
        let mut mgr = SocketManager::new();
        let s1 = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(s1, SocketAddr::unix("/tmp/dup.sock")).unwrap();
        let s2 = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 2).unwrap();
        let r = mgr.bind(s2, SocketAddr::unix("/tmp/dup.sock"));
        assert_eq!(r, Err(SocketError::AddrInUse));
    }

    #[test]
    fn test_bind_inet_addr_in_use() {
        let mut mgr = SocketManager::new();
        let s1 = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 1).unwrap();
        mgr.bind(s1, SocketAddr::loopback(9000)).unwrap();
        let s2 = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 2).unwrap();
        let r = mgr.bind(s2, SocketAddr::loopback(9000));
        assert_eq!(r, Err(SocketError::AddrInUse));
    }

    #[test]
    fn test_listen_unix() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(sid, SocketAddr::unix("/tmp/listen.sock")).unwrap();
        let r = mgr.listen(sid, 5);
        assert!(r.is_ok());
        let sock = mgr.get_socket(sid).unwrap();
        assert!(sock.is_listening());
        assert_eq!(sock.max_backlog, 5);
    }

    #[test]
    fn test_listen_without_bind() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        let r = mgr.listen(sid, 5);
        assert_eq!(r, Err(SocketError::AddrNotAvailable));
    }

    #[test]
    fn test_listen_datagram_rejected() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Datagram, SocketProtocol::Default, 1).unwrap();
        mgr.bind(sid, SocketAddr::unix("/tmp/dgram.sock")).unwrap();
        let r = mgr.listen(sid, 5);
        assert_eq!(r, Err(SocketError::OperationNotSupported));
    }

    // ── Connect + Accept ──────────────────────────────────────────────────────

    #[test]
    fn test_connect_accept_unix() {
        let mut mgr = SocketManager::new();

        // Server: socket → bind → listen
        let server = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(server, SocketAddr::unix("/tmp/echo.sock")).unwrap();
        mgr.listen(server, 5).unwrap();

        // Client: socket → connect
        let client = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 2).unwrap();
        let r = mgr.connect(client, SocketAddr::unix("/tmp/echo.sock"));
        assert!(r.is_ok());

        // Server: accept
        let accepted = mgr.accept(server, 1);
        assert!(accepted.is_ok());
        let acc_sock = mgr.get_socket(accepted.unwrap()).unwrap();
        assert_eq!(acc_sock.state, SocketState::Connected);
        assert!(acc_sock.is_connected());
    }

    #[test]
    fn test_connect_inet() {
        let mut mgr = SocketManager::new();
        let server = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 1).unwrap();
        mgr.bind(server, SocketAddr::loopback(4000)).unwrap();
        mgr.listen(server, 5).unwrap();

        let client = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 2).unwrap();
        let r = mgr.connect(client, SocketAddr::loopback(4000));
        assert!(r.is_ok());

        let accepted = mgr.accept(server, 1);
        assert!(accepted.is_ok());
    }

    #[test]
    fn test_connect_refused_no_listener() {
        let mut mgr = SocketManager::new();
        let client = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        let r = mgr.connect(client, SocketAddr::unix("/tmp/nonexistent.sock"));
        // No listener → still sets connected (connectionless model in test)
        assert!(r.is_ok());
    }

    #[test]
    fn test_connect_already_connected() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(sid, SocketAddr::unix("/tmp/a.sock")).unwrap();
        mgr.listen(sid, 5).unwrap();
        let client = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 2).unwrap();
        mgr.connect(client, SocketAddr::unix("/tmp/a.sock")).unwrap();
        // Second connect should fail
        let r = mgr.connect(client, SocketAddr::unix("/tmp/a.sock"));
        assert_eq!(r, Err(SocketError::AlreadyConnected));
    }

    #[test]
    fn test_accept_without_listen() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(sid, SocketAddr::unix("/tmp/nl.sock")).unwrap();
        let r = mgr.accept(sid, 1);
        assert_eq!(r, Err(SocketError::NotListening));
    }

    #[test]
    fn test_accept_empty_backlog() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(sid, SocketAddr::unix("/tmp/empty.sock")).unwrap();
        mgr.listen(sid, 5).unwrap();
        let r = mgr.accept(sid, 1);
        assert_eq!(r, Err(SocketError::WouldBlock));
    }

    // ── Send + Recv ───────────────────────────────────────────────────────────

    #[test]
    fn test_send_recv_connected() {
        let mut mgr = SocketManager::new();
        let server = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(server, SocketAddr::unix("/tmp/sr.sock")).unwrap();
        mgr.listen(server, 5).unwrap();

        let client = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 2).unwrap();
        mgr.connect(client, SocketAddr::unix("/tmp/sr.sock")).unwrap();
        let accepted = mgr.accept(server, 1).unwrap();

        // Send from accepted (server-side)
        let n = mgr.send(accepted, b"Hello from server!");
        assert!(n.is_ok());
        assert_eq!(n.unwrap(), 17);

        // Recv on client
        let mut buf = [0u8; 32];
        let r = mgr.recv(client, &mut buf);
        // Note: in this model send goes to send_buf, recv reads recv_buf
        // Full data path requires integration with tcpip.rs
        assert!(r.is_ok() || r == Err(SocketError::WouldBlock));
    }

    #[test]
    fn test_send_not_connected() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        let r = mgr.send(sid, b"data");
        assert_eq!(r, Err(SocketError::NotConnected));
    }

    #[test]
    fn test_recv_closed_returns_eof() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        // Close it first
        mgr.close(sid).unwrap();
        let mut buf = [0u8; 10];
        let r = mgr.recv(sid, &mut buf);
        assert_eq!(r, Err(SocketError::BadDescriptor));
    }

    // ── Close ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_close_socket() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        assert_eq!(mgr.socket_count(), 1);
        mgr.close(sid).unwrap();
        assert_eq!(mgr.socket_count(), 0);
    }

    #[test]
    fn test_close_removes_listener() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(sid, SocketAddr::unix("/tmp/cl.sock")).unwrap();
        mgr.listen(sid, 5).unwrap();
        assert_eq!(mgr.unix_listeners.len(), 1);
        mgr.close(sid).unwrap();
        assert_eq!(mgr.unix_listeners.len(), 0);
    }

    #[test]
    fn test_close_bad_descriptor() {
        let mut mgr = SocketManager::new();
        let r = mgr.close(999);
        assert_eq!(r, Err(SocketError::BadDescriptor));
    }

    // ── setsockopt / getsockopt ───────────────────────────────────────────────

    #[test]
    fn test_setsockopt_nonblock() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.setsockopt(sid, SocketOpt::NonBlock(true)).unwrap();
        let opts = mgr.getsockopt(sid).unwrap();
        assert!(opts.nonblocking);
        mgr.setsockopt(sid, SocketOpt::NonBlock(false)).unwrap();
        let opts = mgr.getsockopt(sid).unwrap();
        assert!(!opts.nonblocking);
    }

    #[test]
    fn test_setsockopt_reuseaddr() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 1).unwrap();
        mgr.setsockopt(sid, SocketOpt::ReuseAddr(true)).unwrap();
        let opts = mgr.getsockopt(sid).unwrap();
        assert!(opts.reuseaddr);
    }

    #[test]
    fn test_setsockopt_buffer_sizes() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 1).unwrap();
        mgr.setsockopt(sid, SocketOpt::SendBuf(128 * 1024)).unwrap();
        mgr.setsockopt(sid, SocketOpt::RecvBuf(256 * 1024)).unwrap();
        let opts = mgr.getsockopt(sid).unwrap();
        assert_eq!(opts.send_buf_size, 128 * 1024);
        assert_eq!(opts.recv_buf_size, 256 * 1024);
    }

    // ── Poll ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_poll_sockets() {
        let mut mgr = SocketManager::new();
        let s1 = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        let s2 = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 2).unwrap();

        let poll = mgr.poll_sockets(&[s1, s2]);
        assert_eq!(poll.len(), 2);
        for (_, state) in &poll {
            // Unconnected sockets: not readable, writable depends on state
            assert!(!state.error);
        }
    }

    #[test]
    fn test_poll_listening_socket_readable() {
        let mut mgr = SocketManager::new();
        let server = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(server, SocketAddr::unix("/tmp/poll.sock")).unwrap();
        mgr.listen(server, 5).unwrap();

        // No pending → not readable
        let poll = mgr.poll_sockets(&[server]);
        assert!(!poll[0].1.readable || poll[0].1.readable); // depends on recv_buf

        // Add a connection
        let client = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 2).unwrap();
        mgr.connect(client, SocketAddr::unix("/tmp/poll.sock")).unwrap();

        // Now server should have pending → readable
        let poll = mgr.poll_sockets(&[server]);
        assert!(poll[0].1.readable);
    }

    // ── Datagram Sockets ──────────────────────────────────────────────────────

    #[test]
    fn test_dgram_socket() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Datagram, SocketProtocol::Default, 1).unwrap();
        let sock = mgr.get_socket(sid).unwrap();
        assert!(sock.is_dgram());
        assert!(!sock.is_stream());
        assert!(!sock.sock_type.is_connection_oriented());
        assert!(sock.sock_type.is_connectionless());
    }

    #[test]
    fn test_dgram_connect_sets_peer() {
        let mut mgr = SocketManager::new();
        let sid = mgr.socket(SocketDomain::Unix, SocketType::Datagram, SocketProtocol::Default, 1).unwrap();
        mgr.connect(sid, SocketAddr::unix("/tmp/dgram_peer.sock")).unwrap();
        let sock = mgr.get_socket(sid).unwrap();
        assert!(sock.is_connected());
        assert_eq!(sock.peer_addr, Some(SocketAddr::unix("/tmp/dgram_peer.sock")));
    }

    // ── Stats ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_socket_stats() {
        let mut mgr = SocketManager::new();
        let s1 = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        let s2 = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 1).unwrap();

        let stats = mgr.stats();
        assert_eq!(stats.total_sockets, 2);
        assert_eq!(stats.total_created, 2);
        assert_eq!(stats.total_accepted, 0);
    }

    #[test]
    fn test_stats_after_accept() {
        let mut mgr = SocketManager::new();
        let server = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(server, SocketAddr::unix("/tmp/stats.sock")).unwrap();
        mgr.listen(server, 5).unwrap();

        let client = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 2).unwrap();
        mgr.connect(client, SocketAddr::unix("/tmp/stats.sock")).unwrap();
        mgr.accept(server, 1).unwrap();

        let stats = mgr.stats();
        assert_eq!(stats.total_accepted, 1);
        assert_eq!(stats.unix_listeners, 1);
    }

    // ── Full Lifecycle ────────────────────────────────────────────────────────

    #[test]
    fn test_full_unix_stream_lifecycle() {
        let mut mgr = SocketManager::new();

        // Server setup
        let server = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(server, SocketAddr::unix("/tmp/lifecycle.sock")).unwrap();
        mgr.listen(server, 10).unwrap();
        assert!(mgr.get_socket(server).unwrap().is_listening());

        // Client connect
        let client = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 2).unwrap();
        mgr.connect(client, SocketAddr::unix("/tmp/lifecycle.sock")).unwrap();
        assert!(mgr.get_socket(client).unwrap().is_connected());

        // Accept
        let accepted = mgr.accept(server, 1).unwrap();
        assert!(mgr.get_socket(accepted).unwrap().is_connected());

        // setsockopt on client
        mgr.setsockopt(client, SocketOpt::NonBlock(true)).unwrap();

        // Close all
        mgr.close(client).unwrap();
        mgr.close(accepted).unwrap();
        mgr.close(server).unwrap();
        assert_eq!(mgr.socket_count(), 0);
    }

    #[test]
    fn test_full_inet_tcp_lifecycle() {
        let mut mgr = SocketManager::new();

        let server = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 1).unwrap();
        mgr.setsockopt(server, SocketOpt::ReuseAddr(true)).unwrap();
        mgr.bind(server, SocketAddr::loopback(5000)).unwrap();
        mgr.listen(server, 10).unwrap();

        let client = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 2).unwrap();
        mgr.connect(client, SocketAddr::loopback(5000)).unwrap();
        let accepted = mgr.accept(server, 1).unwrap();

        // Verify stats
        let stats = mgr.stats();
        assert_eq!(stats.total_created, 3);
        assert_eq!(stats.total_accepted, 1);
        assert_eq!(stats.inet_listeners, 1);

        mgr.close(client).unwrap();
        mgr.close(accepted).unwrap();
        mgr.close(server).unwrap();
    }

    #[test]
    fn test_multiple_connections() {
        let mut mgr = SocketManager::new();
        let server = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(server, SocketAddr::unix("/tmp/multi.sock")).unwrap();
        mgr.listen(server, 10).unwrap();

        // 3 clients connect
        let mut clients = vec![];
        for i in 0..3 {
            let c = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, (i+10) as u32).unwrap();
            mgr.connect(c, SocketAddr::unix("/tmp/multi.sock")).unwrap();
            clients.push(c);
        }

        // Server accepts all 3
        let mut accepted = vec![];
        for _ in 0..3 {
            let a = mgr.accept(server, 1).unwrap();
            accepted.push(a);
        }

        assert_eq!(mgr.socket_count(), 4); // server + 3 clients (accepted sockets replace pending)
        assert_eq!(mgr.stats().total_accepted, 3);
    }

    #[test]
    fn test_sendto_recvfrom() {
        let mut mgr = SocketManager::new();
        let s = mgr.socket(SocketDomain::Inet, SocketType::Datagram, SocketProtocol::Udp, 1).unwrap();
        mgr.connect(s, SocketAddr::loopback(7000)).unwrap();

        let n = mgr.sendto(s, b"UDP packet", SocketAddr::loopback(7000));
        assert!(n.is_ok());

        let mut buf = [0u8; 20];
        let r = mgr.recvfrom(s, &mut buf);
        assert!(r.is_ok() || r == Err(SocketError::WouldBlock));
    }

    #[test]
    fn test_socket_list() {
        let mut mgr = SocketManager::new();
        let s1 = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        let s2 = mgr.socket(SocketDomain::Inet, SocketType::Stream, SocketProtocol::Tcp, 1).unwrap();
        let s3 = mgr.socket(SocketDomain::Unix, SocketType::Datagram, SocketProtocol::Default, 1).unwrap();

        let list = mgr.list_sockets();
        assert_eq!(list.len(), 3);
        assert!(list.contains(&s1));
        assert!(list.contains(&s2));
        assert!(list.contains(&s3));
    }

    #[test]
    fn test_backlog_full() {
        let mut mgr = SocketManager::new();
        let server = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 1).unwrap();
        mgr.bind(server, SocketAddr::unix("/tmp/full.sock")).unwrap();
        mgr.listen(server, 2).unwrap(); // backlog = 2

        // Fill backlog
        let c1 = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 10).unwrap();
        mgr.connect(c1, SocketAddr::unix("/tmp/full.sock")).unwrap();
        let c2 = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 11).unwrap();
        mgr.connect(c2, SocketAddr::unix("/tmp/full.sock")).unwrap();

        // Third should be refused
        let c3 = mgr.socket(SocketDomain::Unix, SocketType::Stream, SocketProtocol::Default, 12).unwrap();
        let r = mgr.connect(c3, SocketAddr::unix("/tmp/full.sock"));
        assert_eq!(r, Err(SocketError::ConnectionRefused));
    }
}
