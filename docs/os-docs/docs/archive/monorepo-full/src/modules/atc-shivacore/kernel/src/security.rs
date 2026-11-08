// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ─────────────────────────────────────────────────────────────────────────
// K-Sprint 15 — Security Layer
// Kernel Layer | Chain-ID 658467
// Multi-Sig (ATC-18), Audit-Log, Peer-Reputation, Rate-Limiting, Secure-Channel.
// Baut auf K3a (Capabilities), K6 (DID), K6b (Ed25519), K14 (P2P) auf.
// ─────────────────────────────────────────────────────────────────────────

use alloc::format;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use spin::Mutex;

// ═══════════════════════════════════════════════════════════════════════════
// 1. Multi-Signature Auth (ATC-18)
// ═══════════════════════════════════════════════════════════════════════════

/// Ein m-of-n Multi-Sig Antrag.
#[derive(Clone, Debug)]
pub struct MultiSigProposal {
    pub id: u64,
    pub description: String,
    pub data_hash: [u8; 32],   // Hash der zu authorisierenden Daten
    pub required_sigs: u32,     // m
    pub total_signers: u32,     // n
    pub signatures: Vec<(String, [u8; 64])>,  // (DID, Ed25519-Signatur)
    pub created_at: u64,
    pub executed: bool,
}

impl MultiSigProposal {
    pub fn new(id: u64, description: String, data_hash: [u8; 32], required: u32, total: u32, created_at: u64) -> Self {
        MultiSigProposal {
            id, description, data_hash,
            required_sigs: required, total_signers: total,
            signatures: Vec::new(), created_at: created_at, executed: false,
        }
    }

    pub fn sign(&mut self, did: String, signature: [u8; 64]) -> Result<(), SecurityError> {
        if self.executed { return Err(SecurityError::AlreadyExecuted); }
        if self.signatures.len() >= self.total_signers as usize {
            return Err(SecurityError::TooManySignatures);
        }
        // Prüfe ob DID bereits signiert hat
        if self.signatures.iter().any(|(d, _)| *d == did) {
            return Err(SecurityError::AlreadySigned(did));
        }
        self.signatures.push((did, signature));
        Ok(())
    }

    pub fn is_ready(&self) -> bool {
        self.signatures.len() as u32 >= self.required_sigs
    }

    pub fn remaining_sigs(&self) -> u32 {
        self.required_sigs.saturating_sub(self.signatures.len() as u32)
    }

    pub fn execute(&mut self) -> Result<(), SecurityError> {
        if !self.is_ready() {
            return Err(SecurityError::InsufficientSignatures {
                have: self.signatures.len() as u32,
                required: self.required_sigs,
            });
        }
        if self.executed { return Err(SecurityError::AlreadyExecuted); }
        self.executed = true;
        Ok(())
    }
}

/// Verwaltet alle Multi-Sig-Anträge.
pub struct MultiSigManager {
    proposals: Mutex<BTreeMap<u64, MultiSigProposal>>,
    next_id: Mutex<u64>,
}

impl MultiSigManager {
    pub fn new() -> Self {
        MultiSigManager { proposals: Mutex::new(BTreeMap::new()), next_id: Mutex::new(1) }
    }

    pub fn create(&self, description: String, data_hash: [u8; 32], required: u32, total: u32, now: u64) -> u64 {
        let id = { let mut n = self.next_id.lock(); let v = *n; *n += 1; v };
        self.proposals.lock().insert(id, MultiSigProposal::new(id, description, data_hash, required, total, now));
        id
    }

    pub fn sign(&self, proposal_id: u64, did: String, signature: [u8; 64]) -> Result<(), SecurityError> {
        let mut proposals = self.proposals.lock();
        let proposal = proposals.get_mut(&proposal_id).ok_or(SecurityError::ProposalNotFound)?;
        proposal.sign(did, signature)
    }

    pub fn execute(&self, proposal_id: u64) -> Result<(), SecurityError> {
        let mut proposals = self.proposals.lock();
        let proposal = proposals.get_mut(&proposal_id).ok_or(SecurityError::ProposalNotFound)?;
        proposal.execute()
    }

    pub fn get(&self, proposal_id: u64) -> Option<MultiSigProposal> {
        self.proposals.lock().get(&proposal_id).cloned()
    }

    pub fn count(&self) -> usize { self.proposals.lock().len() }
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. Audit-Log (Tamper-Evident)
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct AuditEntry {
    pub seq: u64,
    pub timestamp: u64,
    pub actor_did: String,
    pub action: String,
    pub resource: String,
    pub result: AuditResult,
    pub prev_hash: [u8; 32],
    pub entry_hash: [u8; 32],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuditResult {
    Allowed,
    Denied,
    Error,
}

pub struct AuditLog {
    entries: Mutex<Vec<AuditEntry>>,
    last_hash: Mutex<[u8; 32]>,
}

impl AuditLog {
    pub fn new() -> Self {
        AuditLog { entries: Mutex::new(Vec::new()), last_hash: Mutex::new([0u8; 32]) }
    }

    pub fn log(&self, timestamp: u64, actor_did: String, action: String, resource: String, result: AuditResult) {
        let mut entries = self.entries.lock();
        let seq = entries.len() as u64 + 1;
        let prev_hash = *self.last_hash.lock();

        // Einfacher Hash: seq + timestamp + did + action + resource + prev_hash
        let mut hasher_input = Vec::new();
        hasher_input.extend_from_slice(&seq.to_be_bytes());
        hasher_input.extend_from_slice(&timestamp.to_be_bytes());
        hasher_input.extend_from_slice(actor_did.as_bytes());
        hasher_input.extend_from_slice(action.as_bytes());
        hasher_input.extend_from_slice(resource.as_bytes());
        hasher_input.extend_from_slice(&prev_hash);
        let entry_hash = simple_hash(&hasher_input);

        *self.last_hash.lock() = entry_hash;
        entries.push(AuditEntry {
            seq, timestamp, actor_did, action, resource, result, prev_hash, entry_hash,
        });
    }

    pub fn verify_chain(&self) -> bool {
        let entries = self.entries.lock();
        let mut expected_prev: [u8; 32] = [0u8; 32];
        for entry in entries.iter() {
            if entry.prev_hash != expected_prev { return false; }
            // Recompute hash
            let mut hasher_input = Vec::new();
            hasher_input.extend_from_slice(&entry.seq.to_be_bytes());
            hasher_input.extend_from_slice(&entry.timestamp.to_be_bytes());
            hasher_input.extend_from_slice(entry.actor_did.as_bytes());
            hasher_input.extend_from_slice(entry.action.as_bytes());
            hasher_input.extend_from_slice(entry.resource.as_bytes());
            hasher_input.extend_from_slice(&entry.prev_hash);
            let recomputed = simple_hash(&hasher_input);
            if recomputed != entry.entry_hash { return false; }
            expected_prev = entry.entry_hash;
        }
        true
    }

    pub fn entries(&self) -> Vec<AuditEntry> { self.entries.lock().clone() }
    pub fn count(&self) -> usize { self.entries.lock().len() }

    /// Filtert Einträge nach Actor.
    pub fn filter_by_actor(&self, did: &str) -> Vec<AuditEntry> {
        self.entries.lock().iter().filter(|e| e.actor_did == did).cloned().collect()
    }

    /// Filtert nach Result.
    pub fn filter_by_result(&self, result: AuditResult) -> Vec<AuditEntry> {
        self.entries.lock().iter().filter(|e| e.result == result).cloned().collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. Peer-Reputation
// ═══════════════════════════════════════════════════════════════════════════

pub const REPUTATION_MAX: i32 = 100;
pub const REPUTATION_MIN: i32 = -100;
pub const BAN_THRESHOLD: i32 = -50;

#[derive(Clone, Debug)]
pub struct PeerReputation {
    pub peer_id: u64,
    pub score: i32,
    pub good_actions: u32,
    pub bad_actions: u32,
    pub banned: bool,
    pub last_updated: u64,
}

pub struct ReputationSystem {
    peers: Mutex<BTreeMap<u64, PeerReputation>>,
}

impl ReputationSystem {
    pub fn new() -> Self { ReputationSystem { peers: Mutex::new(BTreeMap::new()) } }

    pub fn register(&self, peer_id: u64) {
        self.peers.lock().entry(peer_id).or_insert(PeerReputation {
            peer_id, score: 0, good_actions: 0, bad_actions: 0, banned: false, last_updated: 0,
        });
    }

    pub fn reward(&self, peer_id: u64, amount: i32, timestamp: u64) {
        let mut peers = self.peers.lock();
        if let Some(r) = peers.get_mut(&peer_id) {
            r.score = (r.score + amount).min(REPUTATION_MAX);
            r.good_actions += 1;
            r.last_updated = timestamp;
            if r.banned && r.score > BAN_THRESHOLD { r.banned = false; }
        }
    }

    pub fn penalize(&self, peer_id: u64, amount: i32, timestamp: u64) {
        let mut peers = self.peers.lock();
        if let Some(r) = peers.get_mut(&peer_id) {
            r.score = (r.score - amount).max(REPUTATION_MIN);
            r.bad_actions += 1;
            r.last_updated = timestamp;
            if r.score <= BAN_THRESHOLD { r.banned = true; }
        }
    }

    pub fn is_banned(&self, peer_id: u64) -> bool {
        self.peers.lock().get(&peer_id).map(|r| r.banned).unwrap_or(false)
    }

    pub fn score(&self, peer_id: u64) -> i32 {
        self.peers.lock().get(&peer_id).map(|r| r.score).unwrap_or(0)
    }

    pub fn get(&self, peer_id: u64) -> Option<PeerReputation> {
        self.peers.lock().get(&peer_id).cloned()
    }

    pub fn peer_count(&self) -> usize { self.peers.lock().len() }
    pub fn banned_count(&self) -> usize { self.peers.lock().values().filter(|r| r.banned).count() }

    pub fn unban(&self, peer_id: u64) {
        if let Some(r) = self.peers.lock().get_mut(&peer_id) { r.banned = false; r.score = 0; }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. Rate-Limiting (Token Bucket)
// ═══════════════════════════════════════════════════════════════════════════

pub struct TokenBucket {
    capacity: u32,
    tokens: Mutex<u32>,
    refill_rate: u32, // tokens per second
    last_refill: Mutex<u64>, // nanoseconds
}

impl TokenBucket {
    pub fn new(capacity: u32, refill_rate: u32, now: u64) -> Self {
        TokenBucket {
            capacity, tokens: Mutex::new(capacity), refill_rate,
            last_refill: Mutex::new(now),
        }
    }

    pub fn try_consume(&self, now: u64, count: u32) -> bool {
        let mut tokens = self.tokens.lock();
        let mut last = self.last_refill.lock();

        // Refill based on elapsed time
        let elapsed_ns = now.saturating_sub(*last);
        let elapsed_secs = elapsed_ns / 1_000_000_000;
        let refill = (elapsed_secs as u32) * self.refill_rate;
        if refill > 0 {
            *tokens = (*tokens + refill).min(self.capacity);
            *last = now;
        }

        if *tokens >= count {
            *tokens -= count;
            true
        } else {
            false
        }
    }

    pub fn available(&self, now: u64) -> u32 {
        let mut tokens = self.tokens.lock();
        let mut last = self.last_refill.lock();
        let elapsed_ns = now.saturating_sub(*last);
        let elapsed_secs = elapsed_ns / 1_000_000_000;
        let refill = (elapsed_secs as u32) * self.refill_rate;
        (*tokens + refill).min(self.capacity)
    }
}

pub struct RateLimiter {
    buckets: Mutex<BTreeMap<u64, TokenBucket>>,
    capacity: u32,
    refill_rate: u32,
}

impl RateLimiter {
    pub fn new(capacity: u32, refill_rate: u32) -> Self {
        RateLimiter { buckets: Mutex::new(BTreeMap::new()), capacity, refill_rate }
    }

    pub fn allow(&self, peer_id: u64, now: u64) -> bool {
        let mut buckets = self.buckets.lock();
        let bucket = buckets.entry(peer_id).or_insert_with(|| TokenBucket::new(self.capacity, self.refill_rate, now));
        bucket.try_consume(now, 1)
    }

    pub fn peer_count(&self) -> usize { self.buckets.lock().len() }
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. Secure-Channel (Encrypted Messaging Stub)
// ═══════════════════════════════════════════════════════════════════════════

/// Simplified Secure-Channel: XOR-basierte Verschluesselung für Tests.
/// In Produktion würde dies XChaCha20-Poly1305 oder AES-GCM verwenden.
#[derive(Clone, Debug)]
pub struct SecureChannel {
    pub peer_did: String,
    pub session_key: [u8; 32],
    pub established: bool,
    pub messages_sent: u64,
    pub messages_recv: u64,
}

impl SecureChannel {
    pub fn new(peer_did: String, session_key: [u8; 32]) -> Self {
        SecureChannel { peer_did, session_key, established: true, messages_sent: 0, messages_recv: 0 }
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Vec<u8> {
        self.messages_sent += 1;
        // Simplified: XOR with key (repeating). Production: AEAD.
        let mut ciphertext = Vec::with_capacity(plaintext.len());
        for (i, byte) in plaintext.iter().enumerate() {
            ciphertext.push(byte ^ self.session_key[i % 32]);
        }
        ciphertext
    }

    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        self.messages_recv += 1;
        // XOR is symmetric
        self.encrypt(ciphertext) // reuse same logic
    }

    pub fn is_established(&self) -> bool { self.established }
    pub fn close(&mut self) { self.established = false; }
}

pub struct SecureChannelManager {
    channels: Mutex<BTreeMap<String, SecureChannel>>,
}

impl SecureChannelManager {
    pub fn new() -> Self { SecureChannelManager { channels: Mutex::new(BTreeMap::new()) } }

    pub fn establish(&self, peer_did: String, session_key: [u8; 32]) -> bool {
        let channel = SecureChannel::new(peer_did.clone(), session_key);
        self.channels.lock().insert(peer_did, channel);
        true
    }

    pub fn send(&self, peer_did: &str, plaintext: &[u8]) -> Result<Vec<u8>, SecurityError> {
        let mut channels = self.channels.lock();
        let channel = channels.get_mut(peer_did).ok_or(SecurityError::ChannelNotEstablished)?;
        if !channel.is_established() { return Err(SecurityError::ChannelNotEstablished); }
        Ok(channel.encrypt(plaintext))
    }

    pub fn recv(&self, peer_did: &str, ciphertext: &[u8]) -> Result<Vec<u8>, SecurityError> {
        let mut channels = self.channels.lock();
        let channel = channels.get_mut(peer_did).ok_or(SecurityError::ChannelNotEstablished)?;
        if !channel.is_established() { return Err(SecurityError::ChannelNotEstablished); }
        Ok(channel.decrypt(ciphertext))
    }

    pub fn close(&self, peer_did: &str) {
        if let Some(c) = self.channels.lock().get_mut(peer_did) { c.close(); }
    }

    pub fn channel_count(&self) -> usize { self.channels.lock().len() }
    pub fn has_channel(&self, peer_did: &str) -> bool { self.channels.lock().contains_key(peer_did) }
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. Security-Manager (Top-Level Integration)
// ═══════════════════════════════════════════════════════════════════════════

pub struct SecurityManager {
    pub multisig: Arc<MultiSigManager>,
    pub audit: Arc<AuditLog>,
    pub reputation: Arc<ReputationSystem>,
    pub rate_limiter: Arc<RateLimiter>,
    pub channels: Arc<SecureChannelManager>,
}

impl SecurityManager {
    pub fn new() -> Self {
        SecurityManager {
            multisig: Arc::new(MultiSigManager::new()),
            audit: Arc::new(AuditLog::new()),
            reputation: Arc::new(ReputationSystem::new()),
            rate_limiter: Arc::new(RateLimiter::new(100, 10)), // 100 tokens, 10/sec refill
            channels: Arc::new(SecureChannelManager::new()),
        }
    }

    /// Prüft ob ein Peer kommunizieren darf (nicht gebannt + Rate-Limit).
    pub fn check_peer(&self, peer_id: u64, now: u64) -> bool {
        if self.reputation.is_banned(peer_id) { return false; }
        self.rate_limiter.allow(peer_id, now)
    }

    /// Loggt eine Sicherheitsaktion.
    pub fn audit_log(&self, timestamp: u64, actor: String, action: String, resource: String, result: AuditResult) {
        self.audit.log(timestamp, actor, action, resource, result);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Hilfsfunktionen
// ═══════════════════════════════════════════════════════════════════════════

/// Vereinfachter Hash (FNV-1a 32-bit, auf 32 Bytes aufgefüllt).
/// Produktion würde SHA-256 verwenden.
pub fn simple_hash(data: &[u8]) -> [u8; 32] {
    let mut hash: u32 = 0x811c9dc5;
    for &byte in data {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(0x01000193);
    }
    let mut result = [0u8; 32];
    result[..4].copy_from_slice(&hash.to_be_bytes());
    // Rest mit FNV über vorangegangene Bytes füllen
    for i in (4..32).step_by(4) {
        hash ^= result[i - 4] as u32;
        hash = hash.wrapping_mul(0x01000193);
        result[i..i+4].copy_from_slice(&hash.to_be_bytes());
    }
    result
}

// ═══════════════════════════════════════════════════════════════════════════
// Errors
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityError {
    AlreadyExecuted,
    AlreadySigned(String),
    TooManySignatures,
    InsufficientSignatures { have: u32, required: u32 },
    ProposalNotFound,
    ChannelNotEstablished,
    PeerBanned,
    RateLimited,
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Multi-Sig (ATC-18) ─────────────────────────────────────────────────

    #[test]
    fn test_multisig_create() {
        let mgr = MultiSigManager::new();
        let id = mgr.create("Transfer 100 ATC".into(), [0x42; 32], 3, 5, 1000);
        let p = mgr.get(id).unwrap();
        assert_eq!(p.required_sigs, 3);
        assert_eq!(p.total_signers, 5);
        assert!(!p.is_ready());
        assert_eq!(p.remaining_sigs(), 3);
    }

    #[test]
    fn test_multisig_sign_and_execute() {
        let mgr = MultiSigManager::new();
        let id = mgr.create("Update protocol".into(), [0xAA; 32], 2, 3, 1000);

        // 2 Signaturen
        mgr.sign(id, "did:node1".into(), [1u8; 64]).unwrap();
        assert!(!mgr.get(id).unwrap().is_ready());

        mgr.sign(id, "did:node2".into(), [2u8; 64]).unwrap();
        assert!(mgr.get(id).unwrap().is_ready());
        assert_eq!(mgr.get(id).unwrap().remaining_sigs(), 0);

        // Execute
        mgr.execute(id).unwrap();
        assert!(mgr.get(id).unwrap().executed);
    }

    #[test]
    fn test_multisig_duplicate_signer() {
        let mgr = MultiSigManager::new();
        let id = mgr.create("Test".into(), [0; 32], 2, 3, 1000);
        mgr.sign(id, "did:node1".into(), [1u8; 64]).unwrap();
        assert_eq!(
            mgr.sign(id, "did:node1".into(), [1u8; 64]),
            Err(SecurityError::AlreadySigned("did:node1".into()))
        );
    }

    #[test]
    fn test_multisig_insufficient() {
        let mgr = MultiSigManager::new();
        let id = mgr.create("Test".into(), [0; 32], 3, 5, 1000);
        mgr.sign(id, "did:node1".into(), [1u8; 64]).unwrap();
        mgr.sign(id, "did:node2".into(), [2u8; 64]).unwrap();
        assert_eq!(
            mgr.execute(id),
            Err(SecurityError::InsufficientSignatures { have: 2, required: 3 })
        );
    }

    #[test]
    fn test_multisig_already_executed() {
        let mgr = MultiSigManager::new();
        let id = mgr.create("Test".into(), [0; 32], 1, 1, 1000);
        mgr.sign(id, "did:node1".into(), [1u8; 64]).unwrap();
        mgr.execute(id).unwrap();
        assert_eq!(mgr.execute(id), Err(SecurityError::AlreadyExecuted));
    }

    #[test]
    fn test_multisig_too_many_signatures() {
        let mgr = MultiSigManager::new();
        let id = mgr.create("Test".into(), [0; 32], 1, 2, 1000);
        mgr.sign(id, "did:node1".into(), [1u8; 64]).unwrap();
        mgr.sign(id, "did:node2".into(), [2u8; 64]).unwrap();
        assert_eq!(
            mgr.sign(id, "did:node3".into(), [3u8; 64]),
            Err(SecurityError::TooManySignatures)
        );
    }

    #[test]
    fn test_multisig_not_found() {
        let mgr = MultiSigManager::new();
        assert_eq!(mgr.sign(999, "did:x".into(), [0; 64]), Err(SecurityError::ProposalNotFound));
        assert_eq!(mgr.execute(999), Err(SecurityError::ProposalNotFound));
    }

    // ── Audit-Log ──────────────────────────────────────────────────────────

    #[test]
    fn test_audit_log_basic() {
        let log = AuditLog::new();
        log.log(1000, "did:admin".into(), "write".into(), "/etc/config".into(), AuditResult::Allowed);
        log.log(2000, "did:guest".into(), "read".into(), "/etc/shadow".into(), AuditResult::Denied);

        assert_eq!(log.count(), 2);
        let entries = log.entries();
        assert_eq!(entries[0].seq, 1);
        assert_eq!(entries[0].result, AuditResult::Allowed);
        assert_eq!(entries[1].seq, 2);
        assert_eq!(entries[1].result, AuditResult::Denied);
    }

    #[test]
    fn test_audit_log_tamper_evident() {
        let log = AuditLog::new();
        log.log(1000, "did:a".into(), "action1".into(), "res1".into(), AuditResult::Allowed);
        log.log(2000, "did:b".into(), "action2".into(), "res2".into(), AuditResult::Denied);
        assert!(log.verify_chain());
    }

    #[test]
    fn test_audit_log_filter_by_actor() {
        let log = AuditLog::new();
        log.log(1000, "did:alice".into(), "read".into(), "/file1".into(), AuditResult::Allowed);
        log.log(2000, "did:bob".into(), "write".into(), "/file2".into(), AuditResult::Allowed);
        log.log(3000, "did:alice".into(), "delete".into(), "/file3".into(), AuditResult::Denied);

        let alice_entries = log.filter_by_actor("did:alice");
        assert_eq!(alice_entries.len(), 2);
    }

    #[test]
    fn test_audit_log_filter_by_result() {
        let log = AuditLog::new();
        log.log(1000, "did:a".into(), "read".into(), "res".into(), AuditResult::Allowed);
        log.log(2000, "did:b".into(), "write".into(), "res".into(), AuditResult::Denied);
        log.log(3000, "did:c".into(), "read".into(), "res".into(), AuditResult::Allowed);

        let denied = log.filter_by_result(AuditResult::Denied);
        assert_eq!(denied.len(), 1);
        let allowed = log.filter_by_result(AuditResult::Allowed);
        assert_eq!(allowed.len(), 2);
    }

    #[test]
    fn test_audit_log_empty() {
        let log = AuditLog::new();
        assert_eq!(log.count(), 0);
        assert!(log.verify_chain()); // empty chain is valid
    }

    // ── Peer-Reputation ─────────────────────────────────────────────────────

    #[test]
    fn test_reputation_register() {
        let rep = ReputationSystem::new();
        rep.register(1);
        assert_eq!(rep.peer_count(), 1);
        assert_eq!(rep.score(1), 0);
        assert!(!rep.is_banned(1));
    }

    #[test]
    fn test_reputation_reward() {
        let rep = ReputationSystem::new();
        rep.register(1);
        rep.reward(1, 10, 1000);
        assert_eq!(rep.score(1), 10);
        rep.reward(1, 20, 2000);
        assert_eq!(rep.score(1), 30);
        assert_eq!(rep.get(1).unwrap().good_actions, 2);
    }

    #[test]
    fn test_reputation_penalize_and_ban() {
        let rep = ReputationSystem::new();
        rep.register(1);
        rep.penalize(1, 30, 1000);
        assert_eq!(rep.score(1), -30);
        assert!(!rep.is_banned(1));

        rep.penalize(1, 30, 2000);
        assert_eq!(rep.score(1), -60);
        assert!(rep.is_banned(1)); // below -50
    }

    #[test]
    fn test_reputation_max_min() {
        let rep = ReputationSystem::new();
        rep.register(1);
        rep.reward(1, 200, 1000);
        assert_eq!(rep.score(1), REPUTATION_MAX); // capped at 100

        rep.penalize(1, 300, 2000);
        assert_eq!(rep.score(1), REPUTATION_MIN); // capped at -100
    }

    #[test]
    fn test_reputation_unban() {
        let rep = ReputationSystem::new();
        rep.register(1);
        rep.penalize(1, 60, 1000);
        assert!(rep.is_banned(1));

        rep.unban(1);
        assert!(!rep.is_banned(1));

        // Reward above threshold should also unban
        rep.penalize(1, 60, 2000);
        assert!(rep.is_banned(1));
        rep.reward(1, 30, 3000);
        assert!(!rep.is_banned(1)); // score now -30, above -50
    }

    #[test]
    fn test_reputation_banned_count() {
        let rep = ReputationSystem::new();
        rep.register(1);
        rep.register(2);
        rep.register(3);
        rep.penalize(1, 60, 1000);
        rep.penalize(2, 60, 1000);
        assert_eq!(rep.banned_count(), 2);
    }

    #[test]
    fn test_reputation_unregistered() {
        let rep = ReputationSystem::new();
        assert_eq!(rep.score(999), 0);
        assert!(!rep.is_banned(999));
    }

    // ── Rate-Limiting ────────────────────────────────────────────────────────

    #[test]
    fn test_rate_limiter_allow_initial() {
        let rl = RateLimiter::new(10, 5);
        assert!(rl.allow(1, 0));
        assert!(rl.allow(1, 0));
        assert!(rl.allow(1, 0));
    }

    #[test]
    fn test_rate_limiter_deplete() {
        let rl = RateLimiter::new(3, 1);
        assert!(rl.allow(1, 0));
        assert!(rl.allow(1, 0));
        assert!(rl.allow(1, 0));
        assert!(!rl.allow(1, 0)); // depleted
    }

    #[test]
    fn test_rate_limiter_refill() {
        let rl = RateLimiter::new(2, 1);
        rl.allow(1, 0);
        rl.allow(1, 0);
        assert!(!rl.allow(1, 0)); // empty
        assert!(!rl.allow(1, 500_000_000)); // 0.5s → no refill yet
        assert!(rl.allow(1, 1_000_000_000)); // 1s → 1 token refilled
    }

    #[test]
    fn test_rate_limiter_multiple_peers() {
        let rl = RateLimiter::new(2, 1);
        rl.allow(1, 0);
        rl.allow(1, 0);
        assert!(!rl.allow(1, 0)); // peer 1 depleted
        assert!(rl.allow(2, 0));  // peer 2 has own bucket
    }

    // ── Secure-Channel ──────────────────────────────────────────────────────

    #[test]
    fn test_secure_channel_encrypt_decrypt() {
        let mgr = SecureChannelManager::new();
        let key = [0x42u8; 32];
        mgr.establish("did:peer1".into(), key);

        let plaintext = b"Hello, secure world!";
        let ciphertext = mgr.send("did:peer1", plaintext).unwrap();
        assert_ne!(&ciphertext[..], &plaintext[..]);

        let decrypted = mgr.recv("did:peer1", &ciphertext).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_secure_channel_not_established() {
        let mgr = SecureChannelManager::new();
        assert_eq!(mgr.send("did:unknown", b"test"), Err(SecurityError::ChannelNotEstablished));
    }

    #[test]
    fn test_secure_channel_close() {
        let mgr = SecureChannelManager::new();
        mgr.establish("did:peer1".into(), [0; 32]);
        assert!(mgr.has_channel("did:peer1"));
        mgr.close("did:peer1");
        assert_eq!(mgr.send("did:peer1", b"test"), Err(SecurityError::ChannelNotEstablished));
    }

    #[test]
    fn test_secure_channel_stats() {
        let mgr = SecureChannelManager::new();
        mgr.establish("did:peer1".into(), [0xAA; 32]);
        mgr.establish("did:peer2".into(), [0xBB; 32]);
        assert_eq!(mgr.channel_count(), 2);
    }

    // ── Security-Manager Integration ────────────────────────────────────────

    #[test]
    fn test_security_manager_check_peer() {
        let sm = SecurityManager::new();
        sm.reputation.register(1);

        // Not banned, rate limit allows
        assert!(sm.check_peer(1, 0));

        // Ban the peer
        sm.reputation.penalize(1, 60, 1000);
        assert!(!sm.check_peer(1, 2000)); // banned
    }

    #[test]
    fn test_security_manager_rate_limit() {
        let sm = SecurityManager::new();
        sm.reputation.register(1);

        // Deplete rate limit (100 tokens)
        for _ in 0..100 {
            assert!(sm.check_peer(1, 0));
        }
        assert!(!sm.check_peer(1, 0)); // rate limited
    }

    #[test]
    fn test_security_manager_audit() {
        let sm = SecurityManager::new();
        sm.audit_log(1000, "did:admin".into(), "grant".into(), "cap:123".into(), AuditResult::Allowed);
        sm.audit_log(2000, "did:guest".into(), "access".into(), "/secret".into(), AuditResult::Denied);
        assert_eq!(sm.audit.count(), 2);
        assert!(sm.audit.verify_chain());
    }

    #[test]
    fn test_security_manager_full_workflow() {
        let sm = SecurityManager::new();
        sm.reputation.register(1);

        // 1. Peer passes security check
        assert!(sm.check_peer(1, 0));

        // 2. Create Multi-Sig proposal
        let pid = sm.multisig.create("Upgrade protocol".into(), [0x42; 32], 2, 3, 1000);

        // 3. Collect signatures
        sm.multisig.sign(pid, "did:node1".into(), [1u8; 64]).unwrap();
        sm.multisig.sign(pid, "did:node2".into(), [2u8; 64]).unwrap();

        // 4. Execute
        sm.multisig.execute(pid).unwrap();

        // 5. Audit the whole operation
        sm.audit_log(2000, "did:system".into(), "multisig_execute".into(), format!("proposal:{}", pid), AuditResult::Allowed);

        // 6. Establish secure channel with peer
        sm.channels.establish("did:peer1".into(), [0xFF; 32]);
        let ct = sm.channels.send("did:peer1", b"sensitive data").unwrap();
        let pt = sm.channels.recv("did:peer1", &ct).unwrap();
        assert_eq!(pt, b"sensitive data");

        // 7. Verify audit trail
        assert!(sm.audit.verify_chain());
    }

    // ── simple_hash ─────────────────────────────────────────────────────────

    #[test]
    fn test_simple_hash_deterministic() {
        let h1 = simple_hash(b"test");
        let h2 = simple_hash(b"test");
        assert_eq!(h1, h2);
    }

    #[test]
    fn test_simple_hash_different_inputs() {
        let h1 = simple_hash(b"hello");
        let h2 = simple_hash(b"world");
        assert_ne!(h1, h2);
    }

    #[test]
    fn test_simple_hash_size() {
        let h = simple_hash(b"test");
        assert_eq!(h.len(), 32);
    }
}
