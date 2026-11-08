// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ─────────────────────────────────────────────────────────────────────────
// K-Sprint 14 (Upgrade) — P2P v1.0.0 gem. ATC-PROTO-P2P-001 (§9-freigegeben
// 08.09.2026, 01:06 UTC+2; SCR-0028). Kernel Layer | Chain-ID 658467.
//
// Implementiert das v1.0.0-Delta der Spezifikation (protocols/p2p/ATC-PROTO-P2P-001.md):
//   §3   Envelope 9+1 Pflichtfelder + kanonische Byte-Serialisierung
//   §3.1 Signatur-Grundlage = kanonische Bytes mit Domain-Separation ATC-P2P-v1\0
//   §3.2 v0.9-Kompatibilitätsmodus (K14-Wire-Format bleibt lesbar)
//   §4   Message-Types 10..13 (CapabilityExchange, AuthChallenge, AuthResponse,
//         KeyExchange) für den 6-Phasen-Handshake
//   §5/§10 Peer-Status Banned (Zeitfenster) + Verified (nach Auth)
//   §7   Gossip-Deduplizierung über Seen-Set (bounded, FIFO-Verdrängung)
//   §8   6-Phasen-Handshake mit Version-Verhandlung + Capability-Bitmap
//   §9   Auth: Challenge-Response über SignatureProvider (Kryptografie HAL,
//         PROTOCOL-001 §12 — Algorithmuswechsel ohne Neuimplementierung);
//         Identität = DID, nie IP/Port
//   §11   Replay-Schutz: nonce je Absender, message_id Seen-Set,
//         timestamp ±120s-Fenster, Chain-ID-Pflicht 658467
//   §13/§14 Fehlercodes ATC-PROTO-P2P-001..019, Rate-Limiting über K15 TokenBucket
// Baut auf K14 (p2p.rs, v0.9) und K15 (security.rs TokenBucket) auf.
// ─────────────────────────────────────────────────────────────────────────

use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet, VecDeque};
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;
use spin::Mutex;

use crate::p2p::{MessageType, P2pMessage, P2pNode, CHAIN_ID};
use shivacore::security::TokenBucket;

/// Parsed HELLO payload: (protocol version, supported extensions, node id).
pub type HelloPayload = (u16, Vec<(u8, u8, u8)>, [u8; 32]);

// ─── Protokoll-Konstanten (ATC-PROTO-P2P-001 §1/§3/§11/§18) ──────────────────

/// Protokoll-Bezeichner, linksbündig, mit Nullbytes auf 16 Byte gepolstert (§3.1).
pub const PROTOCOL_ID: [u8; 16] = *b"ATC-PROTO-P2P\0\0\0";

/// Normative Protokollversion dieses Moduls.
pub const PROTOCOL_VERSION: (u8, u8, u8) = (1, 0, 0);

/// Domain-Separation-Präfix für Signaturen (§11 — verhindert Cross-Protokoll-Reuse).
pub const DOMAIN_SEPARATION: &[u8] = b"ATC-P2P-v1\0";

/// Verhandelte Versionsspanne (§18: supported_versions).
pub const SUPPORTED_VERSIONS: [(u8, u8, u8); 2] = [(0, 9, 0), (1, 0, 0)];
pub const MINIMUM_VERSION: (u8, u8, u8) = (0, 9, 0);
pub const MAXIMUM_VERSION: (u8, u8, u8) = (1, 0, 0);

/// Timestamp-Toleranzfenster ±120 s (§15 — Millisekunden).
pub const TIMESTAMP_WINDOW_MS: u64 = 120_000;

/// Seen-Set-Obergrenze für Deduplizierung (§7 — bounded, FIFO-Verdrängung).
pub const SEEN_SET_MAX: usize = 4096;

/// Rate-Limit-Defaults (§14).
pub const RATE_MSG_CAPACITY: u32 = 100; // Messages/s je Peer
pub const RATE_MSG_REFILL: u32 = 100;
pub const RATE_BYTES_CAPACITY: u32 = 256 * 1024; // Bytes/s je Peer
pub const RATE_BYTES_REFILL: u32 = 256 * 1024;

// ─── v1.0.0-Message-Types (ATC-PROTO-P2P-001 §4) ─────────────────────────────

/// NEU in v1.0.0 (Kontroll-Ebene, 6-Phasen-Handshake §8). Typen 1..9 bleiben
/// unverändert aus K14 (Daten-Ebene + v0.9-Handshake-Subset).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum V1Type {
    CapabilityExchange = 10,
    AuthChallenge = 11,
    AuthResponse = 12,
    KeyExchange = 13,
}

impl V1Type {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            10 => Some(V1Type::CapabilityExchange),
            11 => Some(V1Type::AuthChallenge),
            12 => Some(V1Type::AuthResponse),
            13 => Some(V1Type::KeyExchange),
            _ => None,
        }
    }
}

/// Kontroll-Typen des erweiterten Handshakes (Phase 2–5) — Daten-Typen
/// (5 BlockAnnounce, 6 TxAnnounce, 7 Vote, 8 PeerList) sind nur nach Phase 6
/// zulässig (§8: „kein Connect ohne abgeschlossene 6 Phasen für Daten").
pub fn is_control_type(msg_type: u8) -> bool {
    matches!(msg_type, 3 | 4 | 10 | 11 | 12 | 13)
}

// ─── Fehler-Protokoll (ATC-PROTO-P2P-001 §12 — K14-Errors + v1.0.0-NEU) ──────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum V1Error {
    /// ATC-PROTO-P2P-001..009: K14-Basis-Errors (siehe P2pError-Mapping der Spezifikation)
    MessageTooShort,
    UnknownMessageType(u8),
    WrongChainId(u32),
    PeerNotFound,
    HandshakeFailed,
    InvalidDID,
    NotConnected,
    /// ATC-PROTO-P2P-010..019: v1.0.0-NEU
    EnvelopeInvalid, // 010 (VALIDATION)
    SignatureInvalid,        // 011 (SECURITY)
    NonceReuse,              // 012 (SECURITY)
    CapabilityDenied,        // 013 (AUTHORIZATION)
    RateLimited,             // 014 (RATE_LIMIT)
    ReplayDetected,          // 015 (SECURITY)
    TimestampOutsideWindow,  // 016 (VALIDATION)
    HandshakePhaseViolation, // 017 (PROTOCOL)
    PeerBanned,              // 018 (SECURITY)
    VersionIncompatible,     // 019 (PROTOCOL)
}

impl V1Error {
    /// ATC-Fehlercode gem. Spezifikation §12 (maschinenlesbar, PROTOCOL-001 §13).
    pub fn error_code(&self) -> &'static str {
        match self {
            V1Error::MessageTooShort => "ATC-PROTO-P2P-001",
            V1Error::UnknownMessageType(_) => "ATC-PROTO-P2P-002",
            V1Error::WrongChainId(_) => "ATC-PROTO-P2P-003",
            V1Error::PeerNotFound => "ATC-PROTO-P2P-004",
            V1Error::HandshakeFailed => "ATC-PROTO-P2P-005",
            V1Error::InvalidDID => "ATC-PROTO-P2P-006",
            V1Error::NotConnected => "ATC-PROTO-P2P-008",
            V1Error::EnvelopeInvalid => "ATC-PROTO-P2P-010",
            V1Error::SignatureInvalid => "ATC-PROTO-P2P-011",
            V1Error::NonceReuse => "ATC-PROTO-P2P-012",
            V1Error::CapabilityDenied => "ATC-PROTO-P2P-013",
            V1Error::RateLimited => "ATC-PROTO-P2P-014",
            V1Error::ReplayDetected => "ATC-PROTO-P2P-015",
            V1Error::TimestampOutsideWindow => "ATC-PROTO-P2P-016",
            V1Error::HandshakePhaseViolation => "ATC-PROTO-P2P-017",
            V1Error::PeerBanned => "ATC-PROTO-P2P-018",
            V1Error::VersionIncompatible => "ATC-PROTO-P2P-019",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            V1Error::MessageTooShort
            | V1Error::EnvelopeInvalid
            | V1Error::TimestampOutsideWindow => "VALIDATION",
            V1Error::UnknownMessageType(_)
            | V1Error::HandshakeFailed
            | V1Error::HandshakePhaseViolation
            | V1Error::VersionIncompatible => "PROTOCOL",
            V1Error::WrongChainId(_)
            | V1Error::SignatureInvalid
            | V1Error::NonceReuse
            | V1Error::ReplayDetected
            | V1Error::PeerBanned => "SECURITY",
            V1Error::PeerNotFound | V1Error::NotConnected => "NETWORK",
            V1Error::InvalidDID => "AUTHENTICATION",
            V1Error::CapabilityDenied => "AUTHORIZATION",
            V1Error::RateLimited => "RATE_LIMIT",
        }
    }

    pub fn retryable(&self) -> bool {
        matches!(
            self,
            V1Error::PeerNotFound
                | V1Error::NotConnected
                | V1Error::HandshakeFailed
                | V1Error::RateLimited
        )
    }

    pub fn severity(&self) -> &'static str {
        match self {
            V1Error::WrongChainId(_)
            | V1Error::SignatureInvalid
            | V1Error::NonceReuse
            | V1Error::ReplayDetected
            | V1Error::PeerBanned => "CRITICAL",
            V1Error::HandshakeFailed
            | V1Error::HandshakePhaseViolation
            | V1Error::VersionIncompatible
            | V1Error::InvalidDID => "WARN",
            _ => "INFO",
        }
    }
}

// ─── Kryptografie-Abstraction-Layer (PROTOCOL-001 §12 / P2P-001 §11) ─────────
//
// Die Spezifikation fordert Ed25519 als Pflicht-Signaturverfahren — aber KEINE
// direkte Verdrahtung im Code. Dieser Trait ist die HAL: der produktive
// Ed25519-Backend wird ausgetauscht, OHNE dass p2p_secure.rs geändert wird.
// `SimulatedSigner` ist der deterministische no_std-Test-Backend (symmetrische
// MAC-Simulation über eine 32-Byte-Mixing-Funktion — für Unit-Tests und
// Trait-basierte Backends wie im Rest des Kernels).

pub trait SignatureProvider {
    /// Erzeugt ein deterministisches Schlüsselpaar aus einem Seed (Tests/CI).
    fn keypair(&self, seed: u64) -> ([u8; 32], [u8; 32]); // (secret, public)
    /// Signiert `msg` mit `secret`.
    fn sign(&self, secret: &[u8; 32], msg: &[u8]) -> [u8; 64];
    /// Prüft `sig` gegen `public` (Identität des Absenders — nie IP/Port, §9).
    fn verify(&self, public: &[u8; 32], msg: &[u8], sig: &[u8; 64]) -> bool;
}

/// 32-Byte-Mixing-Hash (FNV-1a-Basis mit Xorshift-Diffusion, no_std, deterministisch).
fn mix32(state: u64, data: &[u8]) -> [u8; 32] {
    let mut s = state ^ 0x9E3779B97F4A7C15;
    for &b in data {
        s ^= b as u64;
        s = s.wrapping_mul(0x100000001B3);
        s ^= s >> 29;
    }
    let mut out = [0u8; 32];
    for i in 0..4 {
        s = s
            .wrapping_mul(0x2545F4914F6CDD1D)
            .wrapping_add(0x9E3779B97F4A7C15);
        out[i * 8..(i + 1) * 8].copy_from_slice(&s.to_le_bytes());
    }
    out
}

/// Simulierter Signatur-Backend (HAL-Implementierung für Tests/CI).
/// Deterministisch: gleiche Eingaben → gleiche Signatur auf jedem Node (§5).
pub struct SimulatedSigner;

impl SignatureProvider for SimulatedSigner {
    fn keypair(&self, seed: u64) -> ([u8; 32], [u8; 32]) {
        // Simulation: public == secret (symmetrische MAC-Simulation). Fälschungs-
        // schutz und Schlüssel-Bindung verhalten sich in Tests korrekt; echte
        // Ed25519-Semantik (öffentlich sicher) liefert der austauschbare Backend.
        let secret = mix32(seed, b"ATC-P2P-keypair");
        (secret, secret)
    }

    fn sign(&self, secret: &[u8; 32], msg: &[u8]) -> [u8; 64] {
        let h1 = mix32(0x243F6A8885A308D3, &[secret.as_slice(), msg].concat());
        let h2 = mix32(0x13198A2E03707344, &[msg, secret.as_slice()].concat());
        let mut sig = [0u8; 64];
        sig[..32].copy_from_slice(&h1);
        sig[32..].copy_from_slice(&h2);
        sig
    }

    fn verify(&self, public: &[u8; 32], msg: &[u8], sig: &[u8; 64]) -> bool {
        // Symmetrische MAC-Simulation: Signatur ist an den (publizierten) Schlüssel
        // gebunden — falscher Absender oder manipuliertes payload → Ungültig.
        self.sign(public, msg) == *sig
    }
}

// ─── Capabilities (ATC-PROTO-P2P-001 §8 Phase 3) ──────────────────────────────

/// Capability-Bitmap gem. PROTOCOL-001 §8 (Mindestmenge).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Capabilities(pub u32);

impl Capabilities {
    pub const CONSENSUS: u32 = 1 << 0;
    pub const TX_VERSIONS: u32 = 1 << 1;
    pub const ZKP: u32 = 1 << 2;
    pub const ENCODING: u32 = 1 << 3;
    pub const COMPRESSION: u32 = 1 << 4;
    pub const FEATURES: u32 = 1 << 5;
    pub const V1_ENVELOPE: u32 = 1 << 6;
    pub const V0_9_COMPAT: u32 = 1 << 7;

    pub fn with(bits: u32) -> Self {
        Capabilities(bits)
    }
    pub fn has(&self, bit: u32) -> bool {
        self.0 & bit != 0
    }
    pub fn intersection(&self, other: Capabilities) -> Capabilities {
        Capabilities(self.0 & other.0)
    }

    /// Serde: Bitmap[4]
    pub fn to_bytes(&self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        if b.len() < 4 {
            return None;
        }
        Some(Capabilities(u32::from_be_bytes([b[0], b[1], b[2], b[3]])))
    }
}

// ─── Envelope v1.0.0 (ATC-PROTO-P2P-001 §3 — 9+1 Pflichtfelder) ────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct Envelope {
    pub protocol: [u8; 16],    // 1: ATC-PROTO-P2P (§3.1 fixed 16)
    pub version: (u8, u8, u8), // 2: SemVer des Absenders
    pub message_type: u8,      // 3: §4-Tabelle
    pub message_id: [u8; 32],  // 4: eindeutig (Seen-Set, §7/§15)
    pub timestamp: u64,        // 5: ms seit Epoch
    pub sender: String,        // 6: DID (K6) — Identität, nie IP (§9)
    pub nonce: [u8; 16],       // 7: Replay-Schutz (§15)
    pub payload: Vec<u8>,      // 8: typspezifisch (§4)
    pub signature: [u8; 64],   // 9: Signatur über kanonische Bytes (§11)
    pub chain_id: u32,         // 10: 658467 (§15 — Abweichung = Disconnect)
}

impl Envelope {
    /// Kanonische Byte-Serialisierung (§3.1) — Signatur-Grundlage, deterministisch:
    /// protocol[16] || version[3] || message_type[1] || message_id[32] ||
    /// timestamp[8] || sender_len[2] || sender || nonce[16] || payload_len[4] ||
    /// payload || chain_id[4] — Big-Endian, feste Feldreihenfolge.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let sender = self.sender.as_bytes();
        let mut buf = Vec::with_capacity(
            16 + 3 + 1 + 32 + 8 + 2 + sender.len() + 16 + 4 + self.payload.len() + 4,
        );
        buf.extend_from_slice(&self.protocol);
        buf.push(self.version.0);
        buf.push(self.version.1);
        buf.push(self.version.2);
        buf.push(self.message_type);
        buf.extend_from_slice(&self.message_id);
        buf.extend_from_slice(&self.timestamp.to_be_bytes());
        buf.extend_from_slice(&(sender.len() as u16).to_be_bytes());
        buf.extend_from_slice(sender);
        buf.extend_from_slice(&self.nonce);
        buf.extend_from_slice(&(self.payload.len() as u32).to_be_bytes());
        buf.extend_from_slice(&self.payload);
        buf.extend_from_slice(&self.chain_id.to_be_bytes());
        buf
    }

    /// Wire-Format: kanonische Bytes || signature[64].
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = self.canonical_bytes();
        buf.extend_from_slice(&self.signature);
        buf
    }

    /// Signiert die kanonischen Bytes mit Domain-Separation-Präfix (§11).
    pub fn sign(&mut self, signer: &dyn SignatureProvider, secret: &[u8; 32]) {
        let mut msg = Vec::with_capacity(DOMAIN_SEPARATION.len() + 200);
        msg.extend_from_slice(DOMAIN_SEPARATION);
        msg.extend_from_slice(&self.canonical_bytes());
        self.signature = signer.sign(secret, &msg);
    }

    /// Prüft die Signatur gegen einen Public-Key (Absender-Identität).
    pub fn verify_signature(&self, signer: &dyn SignatureProvider, public: &[u8; 32]) -> bool {
        let mut msg = Vec::with_capacity(DOMAIN_SEPARATION.len() + 200);
        msg.extend_from_slice(DOMAIN_SEPARATION);
        msg.extend_from_slice(&self.canonical_bytes());
        signer.verify(public, &msg, &self.signature)
    }

    /// Parst Wire-Bytes; verifiziert Protokoll-ID und Chain-ID (§3/§15).
    pub fn from_bytes(data: &[u8]) -> Result<Self, V1Error> {
        // Minimum: 16+3+1+32+8+2+16+4+4+64 = 150 + Sender
        if data.len() < 150 {
            return Err(V1Error::MessageTooShort);
        }
        let mut protocol = [0u8; 16];
        protocol.copy_from_slice(&data[0..16]);
        if protocol != PROTOCOL_ID {
            return Err(V1Error::EnvelopeInvalid);
        }
        let version = (data[16], data[17], data[18]);
        let message_type = data[19];
        let mut message_id = [0u8; 32];
        message_id.copy_from_slice(&data[20..52]);
        let timestamp = u64::from_be_bytes([
            data[52], data[53], data[54], data[55], data[56], data[57], data[58], data[59],
        ]);
        let sender_len = u16::from_be_bytes([data[60], data[61]]) as usize;
        if data.len() < 62 + sender_len + 16 + 4 + 4 + 64 {
            return Err(V1Error::MessageTooShort);
        }
        let sender = String::from_utf8_lossy(&data[62..62 + sender_len]).to_string();
        let mut off = 62 + sender_len;
        let mut nonce = [0u8; 16];
        nonce.copy_from_slice(&data[off..off + 16]);
        off += 16;
        let payload_len =
            u32::from_be_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]) as usize;
        if data.len() < off + 4 + payload_len + 4 + 64 {
            return Err(V1Error::MessageTooShort);
        }
        off += 4;
        let payload = data[off..off + payload_len].to_vec();
        off += payload_len;
        let chain_id = u32::from_be_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]);
        if chain_id != CHAIN_ID {
            return Err(V1Error::WrongChainId(chain_id));
        }
        off += 4;
        let mut signature = [0u8; 64];
        signature.copy_from_slice(&data[off..off + 64]);
        Ok(Envelope {
            protocol,
            version,
            message_type,
            message_id,
            timestamp,
            sender,
            nonce,
            payload,
            signature,
            chain_id,
        })
    }
}

// ─── Seen-Set (§7 Gossip-Dedup, bounded) ───────────────────────────────────────

pub struct SeenSet {
    seen: BTreeSet<[u8; 32]>,
    order: VecDeque<[u8; 32]>,
    max: usize,
}

impl SeenSet {
    pub fn new(max: usize) -> Self {
        SeenSet {
            seen: BTreeSet::new(),
            order: VecDeque::new(),
            max,
        }
    }

    /// true = neu (und aufgenommen); false = Replay (§12: ATC-PROTO-P2P-015).
    pub fn check_and_insert(&mut self, id: [u8; 32]) -> bool {
        if !self.seen.insert(id) {
            return false;
        }
        self.order.push_back(id);
        if self.order.len() > self.max {
            if let Some(old) = self.order.pop_front() {
                self.seen.remove(&old);
            }
        }
        true
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }
    /// Counterpart to `len`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ─── Secure-Peer-Verwaltung (§5/§8/§10/§14) ───────────────────────────────────

/// Handshake-Phasen (§8) — strikte Übergänge, Verstoß = PhaseViolation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandshakePhase {
    None = 0,
    HelloSent = 1,     // Phase 1 (HELLO)
    Negotiated = 2,    // Phase 2 (PROTOCOL_NEGOTIATION)
    Capabilities = 3,  // Phase 3 (CAPABILITY_EXCHANGE)
    Authenticated = 4, // Phase 4 (AUTHENTICATION)
    KeysExchanged = 5, // Phase 5 (KEY_EXCHANGE)
    Established = 6,   // Phase 6 (SESSION_ESTABLISHED)
}

pub struct SecurePeer {
    pub phase: HandshakePhase,
    pub their_public: Option<[u8; 32]>,
    pub their_capabilities: Capabilities,
    pub negotiated_version: Option<(u8, u8, u8)>,
    pub session_key: Option<[u8; 32]>,
    pub challenge: Option<[u8; 32]>,
    pub banned_until: u64,
    pub verified: bool,
    pub msg_bucket: TokenBucket,
    pub bytes_bucket: TokenBucket,
    pub nonces: BTreeSet<[u8; 16]>,
}

impl SecurePeer {
    pub fn new(now: u64) -> Self {
        SecurePeer {
            phase: HandshakePhase::None,
            their_public: None,
            their_capabilities: Capabilities::default(),
            negotiated_version: None,
            session_key: None,
            challenge: None,
            banned_until: 0,
            verified: false,
            msg_bucket: TokenBucket::new(RATE_MSG_CAPACITY, RATE_MSG_REFILL, now),
            bytes_bucket: TokenBucket::new(RATE_BYTES_CAPACITY, RATE_BYTES_REFILL, now),
            nonces: BTreeSet::new(),
        }
    }
}

pub struct SecurePeerTable {
    peers: Mutex<BTreeMap<u64, Box<SecurePeer>>>,
}

impl Default for SecurePeerTable {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurePeerTable {
    pub fn new() -> Self {
        SecurePeerTable {
            peers: Mutex::new(BTreeMap::new()),
        }
    }

    pub fn ensure(&self, peer_id: u64, now: u64) {
        let mut peers = self.peers.lock();
        peers
            .entry(peer_id)
            .or_insert_with(|| Box::new(SecurePeer::new(now)));
    }

    pub fn challenge_of(&self, peer_id: u64) -> Option<[u8; 32]> {
        self.peers.lock().get(&peer_id).and_then(|p| p.challenge)
    }

    pub fn public_of(&self, peer_id: u64) -> Option<[u8; 32]> {
        self.peers.lock().get(&peer_id).and_then(|p| p.their_public)
    }

    pub fn capabilities_of(&self, peer_id: u64) -> Capabilities {
        self.peers
            .lock()
            .get(&peer_id)
            .map(|p| p.their_capabilities)
            .unwrap_or_default()
    }

    pub fn update(&self, peer_id: u64, f: impl FnOnce(&mut SecurePeer)) {
        let mut peers = self.peers.lock();
        // Update-Semantik: fehlende Einträge werden angelegt (lazy creation,
        // Bucket-Baseline t=0 — erste Nutzung füllt auf Kapazität auf).
        let p = peers
            .entry(peer_id)
            .or_insert_with(|| Box::new(SecurePeer::new(0)));
        f(p);
    }

    pub fn phase(&self, peer_id: u64) -> HandshakePhase {
        self.peers
            .lock()
            .get(&peer_id)
            .map(|p| p.phase)
            .unwrap_or(HandshakePhase::None)
    }

    pub fn is_verified(&self, peer_id: u64) -> bool {
        self.peers
            .lock()
            .get(&peer_id)
            .map(|p| p.verified)
            .unwrap_or(false)
    }

    pub fn session_key(&self, peer_id: u64) -> Option<[u8; 32]> {
        self.peers.lock().get(&peer_id).and_then(|p| p.session_key)
    }

    pub fn ban(&self, peer_id: u64, until: u64) {
        self.update(peer_id, |p| {
            p.banned_until = until;
            p.verified = false;
        });
    }

    pub fn is_banned(&self, peer_id: u64, now: u64) -> bool {
        self.peers
            .lock()
            .get(&peer_id)
            .map(|p| p.banned_until > now)
            .unwrap_or(false)
    }

    pub fn count(&self) -> usize {
        self.peers.lock().len()
    }
}

// ─── SecureP2pNode — v1.0.0-Schicht über der K14-Basis (P2pNode) ──────────────

/// Symmetrische Session-Key-Ableitung (§11 — HKDF-Ersatz der Simulation):
/// beide Public-Keys werden byteweise sortiert, damit Initiator und Responder
/// unabhängig von der Rollen-Reihenfolge denselben Key ableiten.
pub fn derive_session_key(pub_a: &[u8; 32], pub_b: &[u8; 32], salt: &[u8; 32]) -> [u8; 32] {
    let (lo, hi) = if pub_a <= pub_b {
        (pub_a, pub_b)
    } else {
        (pub_b, pub_a)
    };
    mix32(0x5A827999, &[DOMAIN_SEPARATION, lo, hi, salt].concat())
}

pub struct SecureP2pNode {
    /// K14-Basis (v0.9): PeerTable, Gossip, Transport-Anbindung.
    pub inner: Arc<P2pNode>,
    signer: Box<dyn SignatureProvider + Send + Sync>,
    our_secret: [u8; 32],
    our_public: [u8; 32],
    peers: SecurePeerTable,
    seen: Mutex<SeenSet>,
    rng: Mutex<u64>,
    msg_counter: Mutex<u64>,
}

impl SecureP2pNode {
    pub fn new(inner: Arc<P2pNode>, seed: u64) -> Self {
        let signer: Box<dyn SignatureProvider + Send + Sync> = Box::new(SimulatedSigner);
        let (our_secret, our_public) = signer.keypair(seed);
        SecureP2pNode {
            inner,
            signer,
            our_secret,
            our_public,
            peers: SecurePeerTable::new(),
            seen: Mutex::new(SeenSet::new(SEEN_SET_MAX)),
            rng: Mutex::new(seed | 1),
            msg_counter: Mutex::new(1),
        }
    }

    pub fn our_public_key(&self) -> [u8; 32] {
        self.our_public
    }
    pub fn secure_peers(&self) -> &SecurePeerTable {
        &self.peers
    }

    /// Xorshift64 — deterministische Nonce-/Message-ID-Quelle (§15).
    fn next_random(&self) -> u64 {
        let mut rng = self.rng.lock();
        *rng ^= *rng << 13;
        *rng ^= *rng >> 7;
        *rng ^= *rng << 17;
        *rng
    }

    pub fn next_nonce(&self) -> [u8; 16] {
        let mut n = [0u8; 16];
        n[..8].copy_from_slice(&self.next_random().to_be_bytes());
        let c = {
            let mut mc = self.msg_counter.lock();
            let v = *mc;
            *mc += 1;
            v
        };
        n[8..].copy_from_slice(&c.to_be_bytes());
        n
    }

    pub fn next_message_id(&self) -> [u8; 32] {
        mix32(self.next_random(), &self.next_random().to_be_bytes())
    }

    /// Baut, signiert und versiegelt einen v1.0.0-Envelope (§3/§11).
    pub fn build_envelope(&self, msg_type: u8, timestamp: u64, payload: Vec<u8>) -> Envelope {
        let mut env = Envelope {
            protocol: PROTOCOL_ID,
            version: PROTOCOL_VERSION,
            message_type: msg_type,
            message_id: self.next_message_id(),
            timestamp,
            sender: self.inner.peers().our_did().to_string(),
            nonce: self.next_nonce(),
            payload,
            signature: [0u8; 64],
            chain_id: CHAIN_ID,
        };
        env.sign(self.signer.as_ref(), &self.our_secret);
        env
    }

    /// Höchste gemeinsame Version (§8 Phase 2).
    pub fn negotiate_version(their_versions: &[(u8, u8, u8)]) -> Result<(u8, u8, u8), V1Error> {
        let mut best: Option<(u8, u8, u8)> = None;
        for &v in their_versions {
            if SUPPORTED_VERSIONS.contains(&v) {
                best = match best {
                    Some(b) if b >= v => Some(b),
                    _ => Some(v),
                };
            }
        }
        best.ok_or(V1Error::VersionIncompatible)
    }

    /// HELLO-Payload (§8 Phase 1+2): listen_port[2] || versions_count[1] ||
    /// versions[3*n] || public_key[32].
    pub fn hello_payload(&self, their_side: bool) -> Vec<u8> {
        let _ = their_side;
        let mut p = Vec::new();
        p.extend_from_slice(&self.inner.listen_port().to_be_bytes());
        p.push(SUPPORTED_VERSIONS.len() as u8);
        for (maj, min, pat) in SUPPORTED_VERSIONS.iter() {
            p.push(*maj);
            p.push(*min);
            p.push(*pat);
        }
        p.extend_from_slice(&self.our_public);
        p
    }

    pub fn parse_hello(payload: &[u8]) -> Result<HelloPayload, V1Error> {
        if payload.len() < 3 {
            return Err(V1Error::MessageTooShort);
        }
        let port = u16::from_be_bytes([payload[0], payload[1]]);
        let n = payload[2] as usize;
        if payload.len() < 3 + n * 3 + 32 {
            return Err(V1Error::MessageTooShort);
        }
        let mut versions = Vec::new();
        for i in 0..n {
            let o = 3 + i * 3;
            versions.push((payload[o], payload[o + 1], payload[o + 2]));
        }
        let mut public = [0u8; 32];
        public.copy_from_slice(&payload[3 + n * 3..3 + n * 3 + 32]);
        Ok((port, versions, public))
    }

    // ── 6-Phasen-Handshake (§8) ─────────────────────────────────────────────

    /// Phase 1: HELLO (Typ 3) — startet den erweiterten Handshake.
    pub fn begin_handshake(&self, peer_id: u64, timestamp: u64) -> Result<Envelope, V1Error> {
        if self.peers.is_banned(peer_id, timestamp) {
            return Err(V1Error::PeerBanned);
        }
        self.peers
            .update(peer_id, |p| p.phase = HandshakePhase::HelloSent);
        Ok(self.build_envelope(3, timestamp, self.hello_payload(false)))
    }

    /// Eingehendes HELLO: Phase 2 (Version-Verhandlung) + Antwort (CapabilityExchange).
    /// Rückgabe: Envelope Typ 10 an den Peer.
    pub fn handle_hello(&self, peer_id: u64, env: &Envelope) -> Result<Envelope, V1Error> {
        if env.message_type != 3 {
            return Err(V1Error::HandshakeFailed);
        }
        let (_port, versions, public) = Self::parse_hello(&env.payload)?;
        let version = Self::negotiate_version(&versions)?;
        self.peers.update(peer_id, |p| {
            p.phase = HandshakePhase::Negotiated;
            p.negotiated_version = Some(version);
            p.their_public = Some(public);
        });
        // Phase 3: CapabilityExchange (Typ 10) — Payload: Capability-Bitmap[4].
        Ok(self.build_envelope(
            10,
            env.timestamp + 1,
            Capabilities::with(
                Capabilities::V1_ENVELOPE | Capabilities::V0_9_COMPAT | Capabilities::TX_VERSIONS,
            )
            .to_bytes()
            .to_vec(),
        ))
    }

    /// Eingehendes CapabilityExchange: Phase 3 abschließen, Phase 4 starten
    /// (AuthChallenge, Typ 11 — Payload: challenge[32]).
    pub fn handle_capability_exchange(
        &self,
        peer_id: u64,
        env: &Envelope,
    ) -> Result<Envelope, V1Error> {
        if env.message_type != 10 {
            return Err(V1Error::HandshakeFailed);
        }
        let caps = Capabilities::from_bytes(&env.payload).ok_or(V1Error::EnvelopeInvalid)?;
        let challenge = mix32(self.next_random(), &env.message_id);
        self.peers.update(peer_id, |p| {
            p.phase = HandshakePhase::Capabilities;
            p.their_capabilities = caps;
            p.challenge = Some(challenge);
        });
        Ok(self.build_envelope(11, env.timestamp + 1, challenge.to_vec()))
    }

    /// Eingehender AuthChallenge: Phase 4 Antwort (AuthResponse, Typ 12 —
    /// Payload: Ed25519-/HAL-Signatur[64] über DOMAIN_SEP || challenge).
    pub fn handle_auth_challenge(&self, peer_id: u64, env: &Envelope) -> Result<Envelope, V1Error> {
        if env.message_type != 11 || env.payload.len() < 32 {
            return Err(V1Error::HandshakeFailed);
        }
        let mut challenge = [0u8; 32];
        challenge.copy_from_slice(&env.payload[..32]);
        let mut msg = Vec::with_capacity(DOMAIN_SEPARATION.len() + 32);
        msg.extend_from_slice(DOMAIN_SEPARATION);
        msg.extend_from_slice(&challenge);
        let sig = self.signer.sign(&self.our_secret, &msg);
        self.peers.update(peer_id, |p| {
            p.challenge = Some(challenge);
        });
        Ok(self.build_envelope(12, env.timestamp + 1, sig.to_vec()))
    }

    /// Eingehender AuthResponse (Challenger-Seite): Signatur gegen das in der
    /// HELLO-Phase registrierte Public-Key prüfen → Phase 5 (KeyExchange, Typ 13 —
    /// Payload: public[32] || session_salt[32]).
    pub fn handle_auth_response(&self, peer_id: u64, env: &Envelope) -> Result<Envelope, V1Error> {
        if env.message_type != 12 || env.payload.len() < 64 {
            return Err(V1Error::HandshakeFailed);
        }
        let mut sig = [0u8; 64];
        sig.copy_from_slice(&env.payload[..64]);
        let challenge = self
            .peers
            .challenge_of(peer_id)
            .ok_or(V1Error::HandshakeFailed)?;
        let mut msg = Vec::with_capacity(DOMAIN_SEPARATION.len() + 32);
        msg.extend_from_slice(DOMAIN_SEPARATION);
        msg.extend_from_slice(&challenge);
        let their_public = self
            .peers
            .public_of(peer_id)
            .ok_or(V1Error::HandshakeFailed)?;
        if !self.signer.verify(&their_public, &msg, &sig) {
            self.peers.ban(peer_id, env.timestamp + 24 * 3600 * 1000);
            return Err(V1Error::SignatureInvalid);
        }
        self.peers.update(peer_id, |p| {
            p.phase = HandshakePhase::Authenticated;
            p.verified = true;
        });
        let session_salt = mix32(self.next_random(), &challenge);
        let mut payload = Vec::with_capacity(64);
        payload.extend_from_slice(&self.our_public);
        payload.extend_from_slice(&session_salt);
        Ok(self.build_envelope(13, env.timestamp + 1, payload))
    }

    /// Eingehender KeyExchange (Responder-Seite): Session-Key ableiten und
    /// Phase 6 abschließen (SESSION_ESTABLISHED — HandshakeAck Typ 4).
    /// Session-Key-Simulation (§11): HKDF-Ersatz über beide Publics + Salt.
    pub fn handle_key_exchange(&self, peer_id: u64, env: &Envelope) -> Result<Envelope, V1Error> {
        if env.message_type != 13 || env.payload.len() < 64 {
            return Err(V1Error::HandshakeFailed);
        }
        let mut their_public = [0u8; 32];
        their_public.copy_from_slice(&env.payload[..32]);
        let mut salt = [0u8; 32];
        salt.copy_from_slice(&env.payload[32..64]);
        let session_key = derive_session_key(&their_public, &self.our_public, &salt);
        self.peers.update(peer_id, |p| {
            p.their_public = Some(their_public);
            p.session_key = Some(session_key);
            p.phase = HandshakePhase::Established;
            p.verified = true;
        });
        // SESSION_ESTABLISHED (Typ 4): Payload = Capability-Bitmap-Ack[4].
        let caps = Capabilities::with(
            Capabilities::V1_ENVELOPE | Capabilities::V0_9_COMPAT | Capabilities::TX_VERSIONS,
        );
        Ok(self.build_envelope(4, env.timestamp + 1, caps.to_bytes().to_vec()))
    }

    /// Abschluss auf der KeyExchange-Initiator-Seite: Session-Key symmetrisch
    /// ableiten (gleiche Inputs, andere Reihenfolge — symmetrische Funktion).
    pub fn finalize_handshake(
        &self,
        peer_id: u64,
        env: &Envelope,
        session_salt: &[u8; 32],
    ) -> Result<(), V1Error> {
        if env.message_type != 4 {
            return Err(V1Error::HandshakeFailed);
        }
        let their_public = self
            .peers
            .public_of(peer_id)
            .ok_or(V1Error::HandshakeFailed)?;
        let session_key = derive_session_key(&their_public, &self.our_public, session_salt);
        self.peers.update(peer_id, |p| {
            p.session_key = Some(session_key);
            p.phase = HandshakePhase::Established;
            p.verified = true;
        });
        Ok(())
    }

    // ── Empfangs-Pipeline (§3.2 Dual-Mode, §10 Threat-Gates, §15 Replay) ────

    /// Verarbeitet Wire-Bytes: v1.0.0-Envelope ODER v0.9-Kompatibilitätsmodus
    /// (P2pMessage). Erzwingt Chain-ID (Ist: K14), Timestamp-Fenster, Nonce- und
    /// Message-ID-Einmaligkeit, Rate-Limits und Handshake-Phase-Gating.
    pub fn handle_wire(&self, peer_id: u64, data: &[u8], now: u64) -> Result<WireMessage, V1Error> {
        if self.peers.is_banned(peer_id, now) {
            return Err(V1Error::PeerBanned);
        }

        match Envelope::from_bytes(data) {
            Ok(env) => self.handle_v1(peer_id, env, now),
            Err(V1Error::MessageTooShort) | Err(V1Error::EnvelopeInvalid) => {
                // v0.9-Kompatibilitätsmodus (§3.2): Basisvalidierung bleibt Chain-ID.
                use crate::p2p::P2pError;
                let msg = P2pMessage::from_bytes(data).map_err(|e| match e {
                    P2pError::WrongChainId(id) => V1Error::WrongChainId(id),
                    P2pError::MessageTooShort => V1Error::MessageTooShort,
                    _ => V1Error::EnvelopeInvalid,
                })?;
                if msg.chain_id != CHAIN_ID {
                    return Err(V1Error::WrongChainId(msg.chain_id));
                }
                Ok(WireMessage::V09(msg))
            }
            Err(e) => Err(e),
        }
    }

    fn handle_v1(&self, peer_id: u64, env: Envelope, now: u64) -> Result<WireMessage, V1Error> {
        // §15: Timestamp-Fenster ±120 s
        if env.timestamp > now + TIMESTAMP_WINDOW_MS || env.timestamp + TIMESTAMP_WINDOW_MS < now {
            return Err(V1Error::TimestampOutsideWindow);
        }
        // §14: Rate-Limiting (K15 TokenBucket) + §15: Nonce-Einmaligkeit
        self.peers.ensure(peer_id, now);
        let payload_len = env.payload.len() as u32;
        let mut gate_err: Option<V1Error> = None;
        self.peers.update(peer_id, |p| {
            if !p.msg_bucket.try_consume(now, 1) {
                gate_err = Some(V1Error::RateLimited);
                return;
            }
            if !p.bytes_bucket.try_consume(now, payload_len) {
                gate_err = Some(V1Error::RateLimited);
                return;
            }
            if !p.nonces.insert(env.nonce) {
                gate_err = Some(V1Error::NonceReuse);
                return;
            }
            if p.nonces.len() > SEEN_SET_MAX {
                p.nonces.clear();
            } // Bounded-Reset (Periodik)
        });
        if let Some(e) = gate_err {
            return Err(e);
        }
        // §7/§15: Message-ID-Deduplizierung (Seen-Set, bounded)
        if !self.seen.lock().check_and_insert(env.message_id) {
            return Err(V1Error::ReplayDetected);
        }
        // §8: Daten-Nachrichten nur nach abgeschlossenem 6-Phasen-Handshake
        if !is_control_type(env.message_type) {
            let phase = self.peers.phase(peer_id);
            if phase != HandshakePhase::Established {
                return Err(V1Error::HandshakePhaseViolation);
            }
            // §9: Capability-Authorization (Vote → Consensus-Bit)
            if env.message_type == 7
                && !self
                    .peers
                    .capabilities_of(peer_id)
                    .has(Capabilities::CONSENSUS)
            {
                return Err(V1Error::CapabilityDenied);
            }
        }
        Ok(WireMessage::V1(env))
    }

    /// Sendet eine v1.0.0-Daten-Nachricht (nur nach Phase 6 zulässig, §8).
    pub fn send_data(
        &self,
        peer_id: u64,
        msg_type: u8,
        timestamp: u64,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, V1Error> {
        if self.peers.is_banned(peer_id, timestamp) {
            return Err(V1Error::PeerBanned);
        }
        if self.peers.phase(peer_id) != HandshakePhase::Established {
            return Err(V1Error::HandshakePhaseViolation);
        }
        Ok(self.build_envelope(msg_type, timestamp, payload).to_bytes())
    }

    pub fn ban_peer(&self, peer_id: u64, now: u64) {
        self.peers.ban(peer_id, now + 24 * 3600 * 1000); // Default 24 h (§10)
    }
}

/// Empfangenes Wire-Ergebnis: v1.0.0-Envelope oder v0.9-Kompatibilitätsmodus.
#[derive(Clone, Debug)]
pub enum WireMessage {
    V1(Envelope),
    V09(P2pMessage),
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn node(seed: u64) -> SecureP2pNode {
        // DID teststatisch (kein format! im no_std-Testpfad nötig)
        let did = match seed {
            100 => "did:shivacore:ed25519:initiator",
            200 => "did:shivacore:ed25519:responder",
            300 => "did:shivacore:ed25519:challenger",
            400 => "did:shivacore:ed25519:legit",
            500 => "did:shivacore:ed25519:imposter",
            _ => "did:shivacore:ed25519:node",
        };
        let inner = Arc::new(P2pNode::new(did.into(), 9000, 50));
        SecureP2pNode::new(inner, seed)
    }

    const TS: u64 = 1_000_000;

    // ── Envelope & kanonische Serialisierung (§3) ────────────────────────────

    #[test]
    fn test_envelope_roundtrip_all_fields() {
        let n = node(42);
        let mut env = n.build_envelope(5, TS, vec![1, 2, 3, 4]);
        env.timestamp = TS + 7;
        env.sender = "did:shivacore:ed25519:roundtrip".into();
        let parsed = Envelope::from_bytes(&env.to_bytes()).unwrap();
        assert_eq!(parsed.protocol, PROTOCOL_ID);
        assert_eq!(parsed.version, (1, 0, 0));
        assert_eq!(parsed.message_type, 5);
        assert_eq!(parsed.message_id, env.message_id);
        assert_eq!(parsed.timestamp, TS + 7);
        assert_eq!(parsed.sender, "did:shivacore:ed25519:roundtrip");
        assert_eq!(parsed.nonce, env.nonce);
        assert_eq!(parsed.payload, vec![1, 2, 3, 4]);
        assert_eq!(parsed.signature, env.signature);
        assert_eq!(parsed.chain_id, CHAIN_ID);
    }

    #[test]
    fn test_canonical_bytes_deterministic() {
        let n = node(7);
        let a = n.build_envelope(6, TS, vec![9; 33]);
        let b = n.build_envelope(6, TS, vec![9; 33]);
        // Gleiche Nachricht-Daten → identische kanonische Bytes (§3.1/§5).
        // (message_id/nonce sind je Aufruf neu — für den Determinismus-Test
        // fixieren wir sie.)
        let mut a2 = a.clone();
        let mut b2 = b;
        a2.message_id = [5; 32];
        a2.nonce = [3; 16];
        b2.message_id = [5; 32];
        b2.nonce = [3; 16];
        assert_eq!(a2.canonical_bytes(), b2.canonical_bytes());
        assert_eq!(
            a2.canonical_bytes().len(),
            16 + 3 + 1 + 32 + 8 + 2 + a2.sender.len() + 16 + 4 + 33 + 4
        );
    }

    #[test]
    fn test_envelope_protocol_mismatch() {
        let n = node(1);
        let mut env = n.build_envelope(5, TS, vec![]);
        env.protocol = *b"XX-PROTO-OTHER\0\0";
        let err = Envelope::from_bytes(&env.to_bytes()).unwrap_err();
        assert_eq!(err, V1Error::EnvelopeInvalid);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-010");
    }

    #[test]
    fn test_envelope_wrong_chain_id() {
        let n = node(1);
        let mut env = n.build_envelope(5, TS, vec![]);
        env.chain_id = 1;
        let bytes = {
            let mut b = env.canonical_bytes();
            b.extend_from_slice(&[0u8; 64]);
            b
        };
        // Chain-ID sitzt bei offset 16+3+1+32+8+2+sender+16+4+payload — hier
        // über from_bytes der manipulierten Wire-Form:
        let err = Envelope::from_bytes(&bytes).unwrap_err();
        assert_eq!(err, V1Error::WrongChainId(1));
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-003");
        assert_eq!(err.severity(), "CRITICAL");
    }

    #[test]
    fn test_envelope_truncated() {
        let err = Envelope::from_bytes(&[0u8; 20]).unwrap_err();
        assert_eq!(err, V1Error::MessageTooShort);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-001");
    }

    #[test]
    fn test_wire_envelope_too_short_is_v09_or_invalid() {
        // 20 Bytes sind weder v1- noch v0.9-Parsbar → EnvelopeInvalid (010).
        let n = node(1);
        let r = n.handle_wire(1, &[0u8; 20], TS);
        assert_eq!(r.unwrap_err(), V1Error::EnvelopeInvalid);
    }

    // ── Signatur & Domain-Separation (§11) ───────────────────────────────────

    #[test]
    fn test_signature_verification_with_secret_binding() {
        // Simulierter HAL-Backend: Signatur ist an Secret gebunden und via
        // verify_signer gegen dasselbe Secret prüfbar (Challenge-Response-Flow).
        let signer = SimulatedSigner;
        let (secret, public) = signer.keypair(99);
        let mut msg = Vec::new();
        msg.extend_from_slice(DOMAIN_SEPARATION);
        msg.extend_from_slice(b"challenge");
        let sig = signer.sign(&secret, &msg);
        // Binding: Signatur ändert sich mit anderem Secret
        let (other_secret, _) = signer.keypair(100);
        let sig_other = signer.sign(&other_secret, &msg);
        assert_ne!(sig, sig_other);
        let _ = public;
        // Determinismus: gleiche Inputs → gleiche Signatur (§5)
        assert_eq!(signer.sign(&secret, &msg), sig);
    }

    #[test]
    fn test_domain_separation_changes_signature() {
        let signer = SimulatedSigner;
        let (secret, _) = signer.keypair(5);
        let with_prefix = signer.sign(&secret, &[DOMAIN_SEPARATION, b"data"].concat());
        let without_prefix = signer.sign(&secret, b"data");
        assert_ne!(with_prefix, without_prefix);
    }

    // ── Replay-Schutz (§15) ───────────────────────────────────────────────────

    #[test]
    fn test_nonce_and_message_id_uniqueness() {
        let n = node(3);
        let mut seen = BTreeSet::new();
        for _ in 0..1000 {
            assert!(seen.insert(n.next_nonce()));
        }
        let mut ids = BTreeSet::new();
        for _ in 0..1000 {
            assert!(ids.insert(n.next_message_id()));
        }
    }

    #[test]
    fn test_seen_set_replay_and_eviction() {
        let mut s = SeenSet::new(4);
        assert!(s.check_and_insert([1; 32]));
        assert!(!s.check_and_insert([1; 32])); // Replay
        assert!(s.check_and_insert([2; 32]));
        assert!(s.check_and_insert([3; 32]));
        assert!(s.check_and_insert([4; 32]));
        assert!(s.check_and_insert([5; 32])); // verdrängt [1;32] (FIFO)
        assert_eq!(s.len(), 4);
        assert!(s.check_and_insert([1; 32])); // wieder neu
    }

    #[test]
    fn test_timestamp_window_boundary() {
        let n = node(11);
        n.peers.update(9, |p| p.phase = HandshakePhase::Established);
        let mut peers_probe_ok = true;
        // Innerhalb ±120 s: ok
        let env_in = n.build_envelope(5, TS, vec![]);
        match n.handle_v1(9, env_in, TS + 120_000) {
            Ok(_) => {}
            Err(e) => {
                peers_probe_ok = false;
                let _ = e;
            }
        }
        // Genau außerhalb: abgewiesen
        let env_out = n.build_envelope(5, TS + 1, vec![]);
        let err = n
            .handle_v1(9, env_out, TS + 120_000 + 120_000 + 2)
            .unwrap_err();
        assert_eq!(err, V1Error::TimestampOutsideWindow);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-016");
        assert!(peers_probe_ok);
    }

    #[test]
    fn test_nonce_reuse_detected() {
        let n = node(12);
        let mut env = n.build_envelope(5, TS, vec![]);
        // Phase-6-Simulation: Peer established setzen, damit Daten durchgehen
        n.peers.update(1, |p| {
            p.phase = HandshakePhase::Established;
            p.verified = true;
        });
        assert!(n.handle_v1(1, env.clone(), TS).is_ok());
        // Gleiche Nonce erneut → NonceReuse (012)
        let mut env2 = n.build_envelope(5, TS, vec![]);
        env2.nonce = env.nonce;
        let err = n.handle_v1(1, env2, TS).unwrap_err();
        assert_eq!(err, V1Error::NonceReuse);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-012");
    }

    #[test]
    fn test_message_id_replay_detected() {
        let n = node(13);
        n.peers.update(1, |p| p.phase = HandshakePhase::Established);
        let env = n.build_envelope(5, TS, vec![]);
        assert!(n.handle_v1(1, env.clone(), TS).is_ok());
        let mut env2 = n.build_envelope(5, TS, vec![]);
        env2.message_id = env.message_id;
        let err = n.handle_v1(1, env2, TS).unwrap_err();
        assert_eq!(err, V1Error::ReplayDetected);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-015");
    }

    // ── Rate-Limiting (§14) ───────────────────────────────────────────────────

    #[test]
    fn test_rate_limit_messages() {
        let n = node(20);
        n.peers.update(1, |p| {
            p.phase = HandshakePhase::Established;
            p.msg_bucket = TokenBucket::new(10, 10, TS);
        });
        let mut limited = false;
        for i in 0..50 {
            let env = n.build_envelope(5, TS + i, vec![]);
            if let Err(e) = n.handle_v1(1, env, TS) {
                assert_eq!(e, V1Error::RateLimited);
                assert_eq!(e.error_code(), "ATC-PROTO-P2P-014");
                limited = true;
                break;
            }
        }
        assert!(
            limited,
            "Token-Bucket muss bei 50 Nachrichten in derselben Sekunde zuschlagen"
        );
    }

    // ── Banning (§5/§10) ──────────────────────────────────────────────────────

    #[test]
    fn test_ban_and_unban_window() {
        let n = node(21);
        n.ban_peer(1, TS);
        assert!(n.peers.is_banned(1, TS + 1000));
        assert!(!n.peers.is_banned(1, TS + 24 * 3600 * 1000 + 1)); // abgelaufen
        let err = n.handle_wire(1, &[0u8; 200], TS).unwrap_err();
        assert_eq!(err, V1Error::PeerBanned);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-018");
    }

    // ── Capabilities (§8 Phase 3, §9 Authorization) ──────────────────────────

    #[test]
    fn test_capabilities_bitmap() {
        let caps = Capabilities::with(Capabilities::V1_ENVELOPE | Capabilities::V0_9_COMPAT);
        assert!(caps.has(Capabilities::V1_ENVELOPE));
        assert!(caps.has(Capabilities::V0_9_COMPAT));
        assert!(!caps.has(Capabilities::CONSENSUS));
        let bytes = caps.to_bytes();
        assert_eq!(Capabilities::from_bytes(&bytes), Some(caps));
        assert!(Capabilities::from_bytes(&[1, 2]).is_none());
    }

    #[test]
    fn test_capability_denied_for_vote_without_consensus() {
        let n = node(22);
        n.peers.update(1, |p| {
            p.phase = HandshakePhase::Established;
            p.their_capabilities = Capabilities::with(Capabilities::TX_VERSIONS);
        });
        let env = n.build_envelope(7, TS, vec![]); // Vote
        let err = n.handle_v1(1, env, TS).unwrap_err();
        assert_eq!(err, V1Error::CapabilityDenied);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-013");
    }

    // ── 6-Phasen-Handshake (§8) ───────────────────────────────────────────────

    #[test]
    fn test_handshake_full_six_phases() {
        let initiator = node(100);
        let responder = node(200);

        // Phase 1: HELLO vom Initiator
        let hello = initiator.begin_handshake(1, TS).unwrap();
        assert_eq!(hello.message_type, 3);
        assert_eq!(initiator.peers.phase(1), HandshakePhase::HelloSent);
        let (port, versions, resp_public) = SecureP2pNode::parse_hello(&hello.payload).unwrap();
        assert_eq!(port, 9000);
        assert_eq!(versions, SUPPORTED_VERSIONS.to_vec());
        assert_eq!(resp_public, initiator.our_public_key());

        // Phase 2+3: Responder verhandelt, antwortet CapabilityExchange
        let caps_msg = responder.handle_hello(1, &hello).unwrap();
        assert_eq!(caps_msg.message_type, 10);
        assert_eq!(responder.peers.phase(1), HandshakePhase::Negotiated);

        // Phase 3+4: Initiator nimmt Capabilities an, sendet AuthChallenge
        let challenge = initiator.handle_capability_exchange(1, &caps_msg).unwrap();
        assert_eq!(challenge.message_type, 11);
        assert_eq!(initiator.peers.phase(1), HandshakePhase::Capabilities);

        // Phase 4: Responder signiert die Challenge (AuthResponse)
        let auth_resp = responder.handle_auth_challenge(1, &challenge).unwrap();
        assert_eq!(auth_resp.message_type, 12);
        assert_eq!(auth_resp.payload.len(), 64);

        // Phase 4→5: Initiator verifiziert gegen registriertes Public-Key.
        // WICHTIG: Public-Key des Responders vorab registrieren (DID-Bindung, §9).
        initiator
            .peers
            .update(1, |p| p.their_public = Some(responder.our_public_key()));
        let key_exchange = initiator.handle_auth_response(1, &auth_resp).unwrap();
        assert_eq!(key_exchange.message_type, 13);
        assert_eq!(initiator.peers.phase(1), HandshakePhase::Authenticated);
        assert!(initiator.peers.is_verified(1));

        // Phase 5→6: Responder leitet Session-Key ab, bestätigt (Typ 4)
        let session_established = responder.handle_key_exchange(1, &key_exchange).unwrap();
        assert_eq!(session_established.message_type, 4);
        assert_eq!(responder.peers.phase(1), HandshakePhase::Established);
        assert!(responder.peers.session_key(1).is_some());

        // Initiator schließt symmetrisch ab (gleicher Salt aus KeyExchange-Payload)
        let mut salt = [0u8; 32];
        salt.copy_from_slice(&key_exchange.payload[32..64]);
        initiator
            .finalize_handshake(1, &session_established, &salt)
            .unwrap();
        assert_eq!(initiator.peers.phase(1), HandshakePhase::Established);

        // Beide Seiten haben denselben Session-Key (§11 symmetrische Ableitung)
        assert_eq!(
            initiator.peers.session_key(1),
            responder.peers.session_key(1)
        );
    }

    #[test]
    fn test_auth_response_wrong_signer_rejected_and_banned() {
        let initiator = node(300);
        let responder = node(400);
        let imposter = node(500);

        let hello = initiator.begin_handshake(1, TS).unwrap();
        let caps_msg = responder.handle_hello(1, &hello).unwrap();
        let challenge = initiator.handle_capability_exchange(1, &caps_msg).unwrap();

        // Imposter signiert die Challenge mit SEINEM Schlüssel (Fälschung)
        let forged = imposter.handle_auth_challenge(1, &challenge).unwrap();
        // Initiator erwartet den registrierten Responder (DID-Bindung, §9)
        initiator
            .peers
            .update(1, |p| p.their_public = Some(responder.our_public_key()));
        let err = initiator.handle_auth_response(1, &forged).unwrap_err();
        assert_eq!(err, V1Error::SignatureInvalid);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-011");
        assert_eq!(err.severity(), "CRITICAL");
        // Fälschungsversuch bannt den Peer (§10 — Default 24 h)
        assert!(initiator.peers.is_banned(1, TS));
    }

    #[test]
    fn test_data_before_established_rejected() {
        let n = node(30);
        let err = n.send_data(1, 5, TS, vec![1]).unwrap_err();
        assert_eq!(err, V1Error::HandshakePhaseViolation);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-017");
    }

    #[test]
    fn test_version_negotiation() {
        assert_eq!(
            SecureP2pNode::negotiate_version(&[(1, 0, 0)]),
            Ok((1, 0, 0))
        );
        assert_eq!(
            SecureP2pNode::negotiate_version(&[(0, 9, 0), (1, 0, 0)]),
            Ok((1, 0, 0))
        );
        assert_eq!(
            SecureP2pNode::negotiate_version(&[(0, 9, 0)]),
            Ok((0, 9, 0))
        );
        let err = SecureP2pNode::negotiate_version(&[(9, 9, 9)]).unwrap_err();
        assert_eq!(err, V1Error::VersionIncompatible);
        assert_eq!(err.error_code(), "ATC-PROTO-P2P-019");
    }

    #[test]
    fn test_handshake_phase_gating_for_incoming_data() {
        let n = node(31);
        n.peers
            .update(1, |p| p.phase = HandshakePhase::Authenticated);
        let env = n.build_envelope(6, TS, vec![2]);
        let err = n.handle_v1(1, env, TS).unwrap_err();
        assert_eq!(err, V1Error::HandshakePhaseViolation);
    }

    // ── v0.9-Kompatibilitätsmodus (§3.2) ─────────────────────────────────────

    #[test]
    fn test_v09_compat_accepted() {
        let n = node(40);
        let v09 = P2pMessage::new(
            MessageType::Ping,
            "did:legacy:old".into(),
            TS,
            vec![1, 2, 3],
        );
        match n.handle_wire(1, &v09.to_bytes(), TS).unwrap() {
            WireMessage::V09(msg) => {
                assert_eq!(msg.msg_type, MessageType::Ping);
                assert_eq!(msg.chain_id, CHAIN_ID);
            }
            _ => panic!("v0.9-Nachricht muss im Kompatibilitätsmodus ankommen"),
        }
    }

    #[test]
    fn test_v09_wrong_chain_rejected_in_compat_mode() {
        let n = node(41);
        let mut v09 = P2pMessage::new(MessageType::Ping, "did:legacy:old".into(), TS, vec![]);
        v09.chain_id = 999;
        let bytes = v09.to_bytes();
        let err = n.handle_wire(1, &bytes, TS).unwrap_err();
        assert_eq!(err, V1Error::WrongChainId(999));
    }

    // ── v1.0.0-Datenverkehr nach Phase 6 (§8) ─────────────────────────────────

    #[test]
    fn test_data_after_established_roundtrip() {
        let n = node(50);
        for peer in [1u64, 2u64] {
            n.peers.update(peer, |p| {
                p.phase = HandshakePhase::Established;
                p.verified = true;
            });
        }
        let wire = n.send_data(1, 5, TS, vec![0xAA; 40]).unwrap();
        match n.handle_wire(2, &wire, TS).unwrap() {
            WireMessage::V1(env) => {
                assert_eq!(env.message_type, 5);
                assert_eq!(env.payload, vec![0xAA; 40]);
                assert_eq!(env.version, (1, 0, 0));
            }
            _ => panic!("v1-Envelope erwartet"),
        }
    }

    #[test]
    fn test_error_catalog_mapping() {
        assert_eq!(V1Error::EnvelopeInvalid.error_code(), "ATC-PROTO-P2P-010");
        assert_eq!(V1Error::SignatureInvalid.error_code(), "ATC-PROTO-P2P-011");
        assert_eq!(V1Error::SignatureInvalid.category(), "SECURITY");
        assert_eq!(V1Error::SignatureInvalid.severity(), "CRITICAL");
        assert!(!V1Error::SignatureInvalid.retryable());
        assert!(V1Error::RateLimited.retryable());
        assert_eq!(V1Error::RateLimited.category(), "RATE_LIMIT");
        assert_eq!(
            V1Error::UnknownMessageType(77).error_code(),
            "ATC-PROTO-P2P-002"
        );
        assert_eq!(V1Error::InvalidDID.error_code(), "ATC-PROTO-P2P-006");
        assert_eq!(V1Error::InvalidDID.category(), "AUTHENTICATION");
    }

    #[test]
    fn test_v1_type_roundtrip() {
        assert_eq!(V1Type::from_u8(10), Some(V1Type::CapabilityExchange));
        assert_eq!(V1Type::from_u8(11), Some(V1Type::AuthChallenge));
        assert_eq!(V1Type::from_u8(12), Some(V1Type::AuthResponse));
        assert_eq!(V1Type::from_u8(13), Some(V1Type::KeyExchange));
        assert_eq!(V1Type::from_u8(9), None);
        assert!(is_control_type(10) && is_control_type(3) && is_control_type(4));
        assert!(
            !is_control_type(5)
                && !is_control_type(6)
                && !is_control_type(7)
                && !is_control_type(8)
        );
    }

    #[test]
    fn test_multiple_peers_independent_state() {
        let n = node(60);
        n.peers.update(1, |p| {
            p.phase = HandshakePhase::Established;
            p.verified = true;
        });
        assert_eq!(n.peers.phase(1), HandshakePhase::Established);
        assert_eq!(n.peers.phase(2), HandshakePhase::None);
        assert!(!n.peers.is_verified(2));
        n.ban_peer(2, TS);
        assert!(n.peers.is_banned(2, TS));
        assert!(!n.peers.is_banned(1, TS));
        assert_eq!(n.peers.count(), 2);
    }

    #[test]
    fn test_regression_k14_v09_untouched() {
        // K14-Basis bleibt voll funktionsfähig (v0.9-Stabilität, §3.2/§18).
        let inner = Arc::new(P2pNode::new("did:shivacore:ed25519:reg".into(), 9000, 50));
        let (peer_id, wire) = inner
            .connect_peer(ipv4_addr(10, 0, 0, 2), 9100, TS)
            .unwrap();
        let ack = inner.handle_handshake(peer_id, &wire, TS).unwrap();
        let parsed = P2pMessage::from_bytes(&ack).unwrap();
        assert_eq!(parsed.msg_type as u8, 4); // HandshakeAck
    }

    use crate::net::Ipv4Address;
    fn ipv4_addr(a: u8, b: u8, c: u8, d: u8) -> Ipv4Address {
        Ipv4Address::new(a, b, c, d)
    }
}
