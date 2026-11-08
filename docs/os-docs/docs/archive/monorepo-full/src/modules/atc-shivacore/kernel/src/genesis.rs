// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Genesis Block Configuration (K-Sprint 26)
//!
//! Implementiert Issue #71: Genesis Block — Konfiguration & Signierung (Chain-ID 658467)
//!
//! Definiert:
//!   - GenesisConfig: Chain-ID, initiale Validator, Token-Allokationen, Parameter
//!   - GenesisBlock: Der Genesis-Block mit State Root, Validator Set, Allokationen
//!   - GenesisBuilder: Erstellt und validiert den Genesis-Block
//!   - GenesisState: Initialer Zustand (Balances, Validator Set, Contract Deployments)
//!
//! Integriert mit:
//!   - atcnet::CHAIN_ID (658467)
//!   - did::DID für Validator-Identitäten
//!   - ats1000::Pid für Prozess-Zuordnung

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

/// Chain-ID für A-TownChain Mainnet
pub const GENESIS_CHAIN_ID: u32 = 658467;

/// Genesis-Zeitstempel (Unix epoch seconds)
pub const GENESIS_TIMESTAMP: u64 = 1726358400; // 2024-09-15T00:00:00Z (symbolisch)

/// Block-Zeit in Sekunden
pub const DEFAULT_BLOCK_TIME: u64 = 3;

/// Maximale Transaktionen pro Block
pub const DEFAULT_MAX_TXS: u16 = 500;

/// Maximale Blockgrösse (1 MB)
pub const MAX_BLOCK_SIZE: usize = 1024 * 1024;

/// Minimale Validator-Stake (1000 ATC)
pub const MIN_VALIDATOR_STAKE: u64 = 1000;

/// Konsens-Schwellwert (2/3+1)
pub const CONSENSUS_THRESHOLD: f64 = 0.667;

/// SHA-256-ähnliche Hash-Funktion (deterministisch, für Tests)
/// In Produktion: echte SHA-3 oder BLAKE3
fn genesis_hash(data: &[u8]) -> [u8; 32] {
    // Simple FNV-1a based hash for testing (not cryptographically secure)
    let mut result = [0u8; 32];
    let mut h1: u64 = 0xcbf29ce484222325;
    let mut h2: u64 = 0x84222325cbf29ce4;
    for (i, &b) in data.iter().enumerate() {
        h1 ^= b as u64;
        h1 = h1.wrapping_mul(0x100000001b3);
        h2 = h2.wrapping_add(b as u64);
        h2 ^= h2 >> 33;
        if i % 8 == 0 {
            let off = (i % 32) & !7;
            result[off..off+8].copy_from_slice(&h1.to_le_bytes());
        } else if i % 8 == 4 {
            let off = ((i + 4) % 32) & !7;
            result[off..off+8].copy_from_slice(&h2.to_le_bytes());
        }
    }
    result
}

// === Genesis Configuration === //

/// Validator-Eintrag in der Genesis-Konfiguration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisValidator {
    /// DID des Validators (did:shivacore:...)
    pub did: String,
    /// ECDSA-Pubkey (komprimiert, 33 Bytes)
    pub pubkey: [u8; 33],
    /// Initialer Stake (in ATC, kleinste Einheit)
    pub stake: u64,
    /// Validator-Adresse (ATC-Präfix + 32 Zeichen)
    pub address: String,
    /// Commission-Rate (in Basispunkten, 0-10000)
    pub commission: u16,
}

/// Token-Allokation in der Genesis-Konfiguration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisAllocation {
    /// Empfänger-Adresse (ATC-Präfix)
    pub address: String,
    /// Betrag (in kleinster Einheit, 1 ATC = 10^8)
    pub amount: u64,
    /// Lock-Type: None, Vesting, TimeLock
    pub lock_type: LockType,
    /// Lock-Dauer in Blöcken (bei Vesting/TimeLock)
    pub lock_duration: u64,
}

/// Lock-Typ für Token-Allokationen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LockType {
    /// Keine Sperre — sofort verfügbar
    None,
    /// Vesting — schrittweise Freigabe
    Vesting,
    /// TimeLock — vollständige Freigabe nach Dauer
    TimeLock,
}

/// Konsens-Parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusParams {
    /// Block-Zeit in Sekunden
    pub block_time: u64,
    /// Maximale Transaktionen pro Block
    pub max_txs_per_block: u16,
    /// Konsens-Schwellwert (Anteil der Stake)
    pub threshold: u64, // in Basispunkten (6667 = 66.67%)
    /// Slash-Faktor für byzantinische Validator (in Basispunkten)
    pub slash_factor: u16,
    /// Maximal erlaubte Blockgrösse
    pub max_block_size: usize,
}

impl Default for ConsensusParams {
    fn default() -> Self {
        Self {
            block_time: DEFAULT_BLOCK_TIME,
            max_txs_per_block: DEFAULT_MAX_TXS,
            threshold: (CONSENSUS_THRESHOLD * 10000.0) as u64,
            slash_factor: 1000, // 10%
            max_block_size: MAX_BLOCK_SIZE,
        }
    }
}

/// Netzwerk-Parameter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkParams {
    /// Chain-ID
    pub chain_id: u32,
    /// Netzwerk-Magic (für P2P-Identifikation)
    pub network_magic: u32,
    /// Standard-Port für P2P
    pub p2p_port: u16,
    /// Standard-Port für RPC
    pub rpc_port: u16,
    /// Maximale Peers
    pub max_peers: u16,
    /// ATCNet-Protokoll-Version
    pub protocol_version: u8,
}

impl Default for NetworkParams {
    fn default() -> Self {
        Self {
            chain_id: GENESIS_CHAIN_ID,
            network_magic: 0x0A0C23A0, // abgeleitet von Chain-ID 658467 (0x0A0C23, Byte-Rotation)
            p2p_port: 9000,
            rpc_port: 9001,
            max_peers: 50,
            protocol_version: 1,
        }
    }
}

/// Vollständige Genesis-Konfiguration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisConfig {
    /// Chain-ID (658467)
    pub chain_id: u32,
    /// Genesis-Zeitstempel
    pub timestamp: u64,
    /// Initiale Validator
    pub validators: Vec<GenesisValidator>,
    /// Initiale Token-Allokationen
    pub allocations: Vec<GenesisAllocation>,
    /// Konsens-Parameter
    pub consensus: ConsensusParams,
    /// Netzwerk-Parameter
    pub network: NetworkParams,
    /// Community-Memo (freier Text, max 256 Bytes)
    pub memo: String,
}

impl GenesisConfig {
    pub fn new(chain_id: u32, timestamp: u64) -> Self {
        Self {
            chain_id,
            timestamp,
            validators: Vec::new(),
            allocations: Vec::new(),
            consensus: ConsensusParams::default(),
            network: NetworkParams { chain_id, ..Default::default() },
            memo: String::new(),
        }
    }

    /// Fügt einen Validator hinzu
    pub fn add_validator(&mut self, validator: GenesisValidator) -> Result<(), GenesisError> {
        if validator.stake < MIN_VALIDATOR_STAKE {
            return Err(GenesisError::StakeTooLow);
        }
        if self.validators.iter().any(|v| v.did == validator.did) {
            return Err(GenesisError::DuplicateValidator);
        }
        if self.validators.iter().any(|v| v.pubkey == validator.pubkey) {
            return Err(GenesisError::DuplicatePubkey);
        }
        if !validator.address.starts_with("ATC") {
            return Err(GenesisError::InvalidAddress);
        }
        self.validators.push(validator);
        Ok(())
    }

    /// Fügt eine Token-Allokation hinzu
    pub fn add_allocation(&mut self, alloc: GenesisAllocation) -> Result<(), GenesisError> {
        if alloc.amount == 0 {
            return Err(GenesisError::InvalidAmount);
        }
        if !alloc.address.starts_with("ATC") {
            return Err(GenesisError::InvalidAddress);
        }
        self.allocations.push(alloc);
        Ok(())
    }

    /// Validiert die gesamte Konfiguration
    pub fn validate(&self) -> Result<(), GenesisError> {
        if self.chain_id != GENESIS_CHAIN_ID {
            return Err(GenesisError::InvalidChainId);
        }
        if self.validators.is_empty() {
            return Err(GenesisError::NoValidators);
        }
        if self.validators.len() < 4 {
            return Err(GenesisError::TooFewValidators);
        }
        if self.validators.len() > 100 {
            return Err(GenesisError::TooManyValidators);
        }
        // Check total stake
        let total_stake: u64 = self.validators.iter().map(|v| v.stake).sum();
        if total_stake < MIN_VALIDATOR_STAKE * self.validators.len() as u64 {
            return Err(GenesisError::InsufficientTotalStake);
        }
        // Check allocations sum
        let total_alloc: u64 = self.allocations.iter().map(|a| a.amount).sum();
        if total_alloc == 0 {
            return Err(GenesisError::NoAllocations);
        }
        // Check for duplicate addresses in allocations
        let mut seen = alloc::collections::BTreeSet::new();
        for a in &self.allocations {
            if !seen.insert(a.address.as_str()) {
                return Err(GenesisError::DuplicateAllocation);
            }
        }
        // Validator DIDs must be unique (checked in add_validator, but double-check)
        let mut did_set = alloc::collections::BTreeSet::new();
        for v in &self.validators {
            if !did_set.insert(v.did.as_str()) {
                return Err(GenesisError::DuplicateValidator);
            }
        }
        // Memo max 256 bytes
        if self.memo.len() > 256 {
            return Err(GenesisError::MemoTooLong);
        }
        Ok(())
    }

    /// Serialisiert die Konfiguration zu Bytes (für Hash-Berechnung)
    pub fn to_hash_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.chain_id.to_le_bytes());
        buf.extend_from_slice(&self.timestamp.to_le_bytes());
        // Validators (sorted by DID for determinism)
        let mut sorted_validators = self.validators.clone();
        sorted_validators.sort_by(|a, b| a.did.cmp(&b.did));
        buf.extend_from_slice(&(sorted_validators.len() as u32).to_le_bytes());
        for v in &sorted_validators {
            let did_bytes = v.did.as_bytes();
            buf.extend_from_slice(&(did_bytes.len() as u16).to_le_bytes());
            buf.extend_from_slice(did_bytes);
            buf.extend_from_slice(&v.pubkey);
            buf.extend_from_slice(&v.stake.to_le_bytes());
            let addr_bytes = v.address.as_bytes();
            buf.extend_from_slice(&(addr_bytes.len() as u16).to_le_bytes());
            buf.extend_from_slice(addr_bytes);
            buf.extend_from_slice(&v.commission.to_le_bytes());
        }
        // Allocations (sorted by address)
        let mut sorted_allocs = self.allocations.clone();
        sorted_allocs.sort_by(|a, b| a.address.cmp(&b.address));
        buf.extend_from_slice(&(sorted_allocs.len() as u32).to_le_bytes());
        for a in &sorted_allocs {
            let addr_bytes = a.address.as_bytes();
            buf.extend_from_slice(&(addr_bytes.len() as u16).to_le_bytes());
            buf.extend_from_slice(addr_bytes);
            buf.extend_from_slice(&a.amount.to_le_bytes());
            buf.push(a.lock_type as u8);
            buf.extend_from_slice(&a.lock_duration.to_le_bytes());
        }
        // Consensus params
        buf.extend_from_slice(&self.consensus.block_time.to_le_bytes());
        buf.extend_from_slice(&self.consensus.max_txs_per_block.to_le_bytes());
        buf.extend_from_slice(&self.consensus.threshold.to_le_bytes());
        buf.extend_from_slice(&self.consensus.slash_factor.to_le_bytes());
        buf.extend_from_slice(&(self.consensus.max_block_size as u32).to_le_bytes());
        // Network params
        buf.extend_from_slice(&self.network.chain_id.to_le_bytes());
        buf.extend_from_slice(&self.network.network_magic.to_le_bytes());
        buf.extend_from_slice(&self.network.p2p_port.to_le_bytes());
        buf.extend_from_slice(&self.network.rpc_port.to_le_bytes());
        buf.extend_from_slice(&self.network.max_peers.to_le_bytes());
        buf.push(self.network.protocol_version);
        // Memo
        let memo_bytes = self.memo.as_bytes();
        buf.extend_from_slice(&(memo_bytes.len() as u16).to_le_bytes());
        buf.extend_from_slice(memo_bytes);
        buf
    }

    /// Berechnet den Genesis-Hash
    pub fn genesis_hash(&self) -> [u8; 32] {
        genesis_hash(&self.to_hash_bytes())
    }
}

// === Genesis Block === //

/// Der Genesis-Block (Height 0)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisBlock {
    /// Blockhöhe (immer 0)
    pub height: u64,
    /// Chain-ID
    pub chain_id: u32,
    /// Genesis-Hash (Hash der Konfiguration)
    pub genesis_hash: [u8; 32],
    /// State Root (Hash des Initialzustands)
    pub state_root: [u8; 32],
    /// Previous-Hash (immer Null für Genesis)
    pub prev_hash: [u8; 32],
    /// Zeitstempel
    pub timestamp: u64,
    /// Validator-Set (DID -> Stake)
    pub validator_set: Vec<(String, u64)>,
    /// Initiale Allokationen (Adresse -> Betrag)
    pub allocations: Vec<(String, u64)>,
    /// Konsens-Parameter (serialisiert)
    pub consensus_params: Vec<u8>,
    /// Netzwerk-Parameter (serialisiert)
    pub network_params: Vec<u8>,
    /// Memo
    pub memo: String,
    /// Signatur des Genesis-Blocks (von der Genesis-Autorität)
    pub signature: Option<[u8; 64]>,
}

impl GenesisBlock {
    /// Block-ID = Genesis-Hash
    pub fn block_id(&self) -> [u8; 32] {
        self.genesis_hash
    }

    /// Ist dies ein Genesis-Block?
    pub fn is_genesis(&self) -> bool {
        self.height == 0 && self.prev_hash == [0u8; 32]
    }

    /// Serialisiert den Block für die Signatur
    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.height.to_le_bytes());
        buf.extend_from_slice(&self.chain_id.to_le_bytes());
        buf.extend_from_slice(&self.genesis_hash);
        buf.extend_from_slice(&self.state_root);
        buf.extend_from_slice(&self.prev_hash);
        buf.extend_from_slice(&self.timestamp.to_le_bytes());
        // Validator set (sorted)
        let mut vs = self.validator_set.clone();
        vs.sort_by(|a, b| a.0.cmp(&b.0));
        buf.extend_from_slice(&(vs.len() as u32).to_le_bytes());
        for (did, stake) in &vs {
            buf.extend_from_slice(&(did.len() as u16).to_le_bytes());
            buf.extend_from_slice(did.as_bytes());
            buf.extend_from_slice(&stake.to_le_bytes());
        }
        // Allocations (sorted)
        let mut al = self.allocations.clone();
        al.sort_by(|a, b| a.0.cmp(&b.0));
        buf.extend_from_slice(&(al.len() as u32).to_le_bytes());
        for (addr, amount) in &al {
            buf.extend_from_slice(&(addr.len() as u16).to_le_bytes());
            buf.extend_from_slice(addr.as_bytes());
            buf.extend_from_slice(&amount.to_le_bytes());
        }
        buf.extend_from_slice(&self.consensus_params);
        buf.extend_from_slice(&self.network_params);
        let memo_bytes = self.memo.as_bytes();
        buf.extend_from_slice(&(memo_bytes.len() as u16).to_le_bytes());
        buf.extend_from_slice(memo_bytes);
        buf
    }

    /// Berechnet den Block-Hash (über alle Felder inkl. Signatur)
    pub fn block_hash(&self) -> [u8; 32] {
        let mut data = self.signing_bytes();
        if let Some(sig) = &self.signature {
            data.extend_from_slice(sig);
        }
        genesis_hash(&data)
    }
}

// === Genesis State === //

/// Initialer Zustand nach Genesis
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenesisState {
    /// Token-Balances (Adresse -> Betrag)
    pub balances: BTreeMap<String, u64>,
    /// Validator-Set (DID -> Stake)
    pub validators: BTreeMap<String, u64>,
    /// Nonce-Counter (Adresse -> Nonce)
    pub nonces: BTreeMap<String, u64>,
    /// Deployed Contracts (Adresse -> Code-Hash)
    pub contracts: BTreeMap<String, [u8; 32]>,
}

impl GenesisState {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
            validators: BTreeMap::new(),
            nonces: BTreeMap::new(),
            contracts: BTreeMap::new(),
        }
    }

    /// Fügt eine Balance hinzu
    pub fn add_balance(&mut self, address: &str, amount: u64) {
        *self.balances.entry(address.to_string()).or_insert(0) += amount;
    }

    /// Fügt einen Validator hinzu
    pub fn add_validator(&mut self, did: &str, stake: u64) {
        self.validators.insert(did.to_string(), stake);
    }

    /// Berechnet den State Root (Hash über alle State-Daten)
    pub fn state_root(&self) -> [u8; 32] {
        let mut buf = Vec::new();
        // Balances (sorted by BTreeMap order)
        buf.extend_from_slice(&(self.balances.len() as u32).to_le_bytes());
        for (addr, amount) in &self.balances {
            buf.extend_from_slice(&(addr.len() as u16).to_le_bytes());
            buf.extend_from_slice(addr.as_bytes());
            buf.extend_from_slice(&amount.to_le_bytes());
        }
        // Validators
        buf.extend_from_slice(&(self.validators.len() as u32).to_le_bytes());
        for (did, stake) in &self.validators {
            buf.extend_from_slice(&(did.len() as u16).to_le_bytes());
            buf.extend_from_slice(did.as_bytes());
            buf.extend_from_slice(&stake.to_le_bytes());
        }
        // Nonces
        buf.extend_from_slice(&(self.nonces.len() as u32).to_le_bytes());
        for (addr, nonce) in &self.nonces {
            buf.extend_from_slice(&(addr.len() as u16).to_le_bytes());
            buf.extend_from_slice(addr.as_bytes());
            buf.extend_from_slice(&nonce.to_le_bytes());
        }
        // Contracts
        buf.extend_from_slice(&(self.contracts.len() as u32).to_le_bytes());
        for (addr, hash) in &self.contracts {
            buf.extend_from_slice(&(addr.len() as u16).to_le_bytes());
            buf.extend_from_slice(addr.as_bytes());
            buf.extend_from_slice(hash);
        }
        genesis_hash(&buf)
    }
}

// === Genesis Builder === //

/// Fehler beim Genesis-Block
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenesisError {
    InvalidChainId,
    NoValidators,
    TooFewValidators,
    TooManyValidators,
    StakeTooLow,
    InsufficientTotalStake,
    DuplicateValidator,
    DuplicatePubkey,
    DuplicateAllocation,
    NoAllocations,
    InvalidAmount,
    InvalidAddress,
    MemoTooLong,
    NotSigned,
    AlreadySigned,
    InvalidSignature,
}

/// Baut den Genesis-Block aus einer Konfiguration
pub struct GenesisBuilder;

impl GenesisBuilder {
    /// Erstellt den Genesis-Block aus der Konfiguration
    pub fn build(config: &GenesisConfig) -> Result<GenesisBlock, GenesisError> {
        // Validiere die Konfiguration
        config.validate()?;

        // Erstelle den Initialzustand
        let mut state = GenesisState::new();

        // Füge Validator-Stakes hinzu
        for v in &config.validators {
            state.add_validator(&v.did, v.stake);
        }

        // Füge Allokationen hinzu
        for a in &config.allocations {
            state.add_balance(&a.address, a.amount);
        }

        // Berechne State Root
        let state_root = state.state_root();

        // Berechne Genesis-Hash
        let genesis_hash = config.genesis_hash();

        // Validator-Set für den Block
        let validator_set: Vec<(String, u64)> = config.validators
            .iter()
            .map(|v| (v.did.clone(), v.stake))
            .collect();

        // Allokationen für den Block
        let allocations: Vec<(String, u64)> = config.allocations
            .iter()
            .map(|a| (a.address.clone(), a.amount))
            .collect();

        // Konsens-Parameter serialisieren
        let consensus_params = serialize_consensus(&config.consensus);

        // Netzwerk-Parameter serialisieren
        let network_params = serialize_network(&config.network);

        Ok(GenesisBlock {
            height: 0,
            chain_id: config.chain_id,
            genesis_hash,
            state_root,
            prev_hash: [0u8; 32],
            timestamp: config.timestamp,
            validator_set,
            allocations,
            consensus_params,
            network_params,
            memo: config.memo.clone(),
            signature: None,
        })
    }

    /// Signiert den Genesis-Block
    pub fn sign(block: &mut GenesisBlock, signature: [u8; 64]) -> Result<(), GenesisError> {
        if block.signature.is_some() {
            return Err(GenesisError::AlreadySigned);
        }
        block.signature = Some(signature);
        Ok(())
    }

    /// Validiert einen signierten Genesis-Block
    pub fn verify(block: &GenesisBlock) -> Result<(), GenesisError> {
        if block.signature.is_none() {
            return Err(GenesisError::NotSigned);
        }
        if block.chain_id != GENESIS_CHAIN_ID {
            return Err(GenesisError::InvalidChainId);
        }
        if !block.is_genesis() {
            return Err(GenesisError::InvalidChainId);
        }
        if block.validator_set.is_empty() {
            return Err(GenesisError::NoValidators);
        }
        if block.allocations.is_empty() {
            return Err(GenesisError::NoAllocations);
        }
        Ok(())
    }

    /// Exportiert den Genesis-Block als JSON-ähnlichen String
    pub fn export_json(block: &GenesisBlock) -> String {
        let mut s = String::new();
        s.push_str("{\n");
        s.push_str(&format!("  \"chain_id\": {},\n", block.chain_id));
        s.push_str(&format!("  \"height\": {},\n", block.height));
        s.push_str(&format!("  \"timestamp\": {},\n", block.timestamp));
        s.push_str(&format!("  \"genesis_hash\": \"0x{}\",\n", hex_string(&block.genesis_hash)));
        s.push_str(&format!("  \"state_root\": \"0x{}\",\n", hex_string(&block.state_root)));
        s.push_str(&format!("  \"prev_hash\": \"0x{}\",\n", hex_string(&block.prev_hash)));
        s.push_str("  \"validators\": [\n");
        for (i, (did, stake)) in block.validator_set.iter().enumerate() {
            s.push_str(&format!("    {{\"did\": \"{}\", \"stake\": {}}}", did, stake));
            if i + 1 < block.validator_set.len() { s.push(','); }
            s.push('\n');
        }
        s.push_str("  ],\n");
        s.push_str("  \"allocations\": [\n");
        for (i, (addr, amount)) in block.allocations.iter().enumerate() {
            s.push_str(&format!("    {{\"address\": \"{}\", \"amount\": {}}}", addr, amount));
            if i + 1 < block.allocations.len() { s.push(','); }
            s.push('\n');
        }
        s.push_str("  ],\n");
        s.push_str(&format!("  \"memo\": \"{}\"\n", block.memo));
        s.push_str("}");
        s
    }
}

// === Serialization helpers === //

fn serialize_consensus(c: &ConsensusParams) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&c.block_time.to_le_bytes());
    buf.extend_from_slice(&c.max_txs_per_block.to_le_bytes());
    buf.extend_from_slice(&c.threshold.to_le_bytes());
    buf.extend_from_slice(&c.slash_factor.to_le_bytes());
    buf.extend_from_slice(&(c.max_block_size as u32).to_le_bytes());
    buf
}

fn serialize_network(n: &NetworkParams) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&n.chain_id.to_le_bytes());
    buf.extend_from_slice(&n.network_magic.to_le_bytes());
    buf.extend_from_slice(&n.p2p_port.to_le_bytes());
    buf.extend_from_slice(&n.rpc_port.to_le_bytes());
    buf.extend_from_slice(&n.max_peers.to_le_bytes());
    buf.push(n.protocol_version);
    buf
}

fn hex_string(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

// === Tests === //

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_pubkey(n: u8) -> [u8; 33] {
        let mut k = [0u8; 33];
        k[0] = 0x02; // compressed
        k[1] = n;
        k
    }

    fn dummy_address(n: u8) -> String {
        format!("ATC{}", "a".repeat(31).chars().chain(core::iter::once((b'a' + n) as char)).collect::<String>())
    }

    fn dummy_did(n: u8) -> String {
        format!("did:shivacore:validator{}", n)
    }

    fn make_test_validator(n: u8, stake: u64) -> GenesisValidator {
        GenesisValidator {
            did: dummy_did(n),
            pubkey: dummy_pubkey(n),
            stake,
            address: dummy_address(n),
            commission: 500, // 5%
        }
    }

    fn make_test_config() -> GenesisConfig {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        // 4 validators (minimum)
        for i in 1..=4u8 {
            config.add_validator(make_test_validator(i, 10000)).unwrap();
        }
        // Allocations
        for i in 1..=4u8 {
            config.add_allocation(GenesisAllocation {
                address: dummy_address(i),
                amount: 1_000_000_000, // 10 ATC
                lock_type: LockType::None,
                lock_duration: 0,
            }).unwrap();
        }
        config.memo = "A-TownChain Mainnet Genesis".to_string();
        config
    }

    // === Config Tests === //

    #[test]
    fn test_genesis_config_creation() {
        let config = make_test_config();
        assert_eq!(config.chain_id, 658467);
        assert_eq!(config.validators.len(), 4);
        assert_eq!(config.allocations.len(), 4);
        assert_eq!(config.memo, "A-TownChain Mainnet Genesis");
    }

    #[test]
    fn test_genesis_config_validate_ok() {
        let config = make_test_config();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_genesis_config_wrong_chain_id() {
        let mut config = make_test_config();
        config.chain_id = 9999;
        assert_eq!(config.validate(), Err(GenesisError::InvalidChainId));
    }

    #[test]
    fn test_genesis_config_no_validators() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        config.add_allocation(GenesisAllocation {
            address: dummy_address(1), amount: 1000, lock_type: LockType::None, lock_duration: 0,
        }).unwrap();
        assert_eq!(config.validate(), Err(GenesisError::NoValidators));
    }

    #[test]
    fn test_genesis_config_too_few_validators() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        config.add_validator(make_test_validator(1, 10000)).unwrap();
        config.add_validator(make_test_validator(2, 10000)).unwrap();
        config.add_validator(make_test_validator(3, 10000)).unwrap();
        config.add_allocation(GenesisAllocation {
            address: dummy_address(1), amount: 1000, lock_type: LockType::None, lock_duration: 0,
        }).unwrap();
        assert_eq!(config.validate(), Err(GenesisError::TooFewValidators)); // < 4
    }

    #[test]
    fn test_genesis_config_duplicate_validator() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        config.add_validator(make_test_validator(1, 10000)).unwrap();
        let result = config.add_validator(make_test_validator(1, 20000)); // Same DID
        assert_eq!(result, Err(GenesisError::DuplicateValidator));
    }

    #[test]
    fn test_genesis_config_duplicate_pubkey() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        let mut v1 = make_test_validator(1, 10000);
        let mut v2 = make_test_validator(2, 10000);
        v1.pubkey = dummy_pubkey(5);
        v2.pubkey = dummy_pubkey(5); // Same pubkey
        config.add_validator(v1).unwrap();
        assert_eq!(config.add_validator(v2), Err(GenesisError::DuplicatePubkey));
    }

    #[test]
    fn test_genesis_config_stake_too_low() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        let v = make_test_validator(1, 500); // < MIN_VALIDATOR_STAKE (1000)
        assert_eq!(config.add_validator(v), Err(GenesisError::StakeTooLow));
    }

    #[test]
    fn test_genesis_config_invalid_address() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        let mut v = make_test_validator(1, 10000);
        v.address = "BTC1abc...".to_string(); // Wrong prefix
        assert_eq!(config.add_validator(v), Err(GenesisError::InvalidAddress));
    }

    #[test]
    fn test_genesis_config_no_allocations() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        for i in 1..=4u8 {
            config.add_validator(make_test_validator(i, 10000)).unwrap();
        }
        assert_eq!(config.validate(), Err(GenesisError::NoAllocations));
    }

    #[test]
    fn test_genesis_config_duplicate_allocation() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        for i in 1..=4u8 {
            config.add_validator(make_test_validator(i, 10000)).unwrap();
        }
        config.add_allocation(GenesisAllocation {
            address: dummy_address(1), amount: 1000, lock_type: LockType::None, lock_duration: 0,
        }).unwrap();
        config.add_allocation(GenesisAllocation {
            address: dummy_address(1), amount: 2000, lock_type: LockType::None, lock_duration: 0, // Same address
        }).unwrap();
        assert_eq!(config.validate(), Err(GenesisError::DuplicateAllocation));
    }

    #[test]
    fn test_genesis_config_memo_too_long() {
        let mut config = make_test_config();
        config.memo = "x".repeat(257);
        assert_eq!(config.validate(), Err(GenesisError::MemoTooLong));
    }

    #[test]
    fn test_genesis_hash_deterministic() {
        let config1 = make_test_config();
        let config2 = make_test_config();
        assert_eq!(config1.genesis_hash(), config2.genesis_hash());
    }

    #[test]
    fn test_genesis_hash_changes_with_config() {
        let mut config1 = make_test_config();
        let mut config2 = make_test_config();
        config2.add_validator(make_test_validator(5, 10000)).unwrap();
        config2.add_allocation(GenesisAllocation {
            address: dummy_address(5), amount: 1000, lock_type: LockType::None, lock_duration: 0,
        }).unwrap();
        assert_ne!(config1.genesis_hash(), config2.genesis_hash());
    }

    // === Block Tests === //

    #[test]
    fn test_genesis_block_creation() {
        let config = make_test_config();
        let block = GenesisBuilder::build(&config).unwrap();

        assert_eq!(block.height, 0);
        assert_eq!(block.chain_id, 658467);
        assert!(block.is_genesis());
        assert_eq!(block.prev_hash, [0u8; 32]);
        assert_eq!(block.validator_set.len(), 4);
        assert_eq!(block.allocations.len(), 4);
        assert!(block.signature.is_none());
    }

    #[test]
    fn test_genesis_block_state_root() {
        let config = make_test_config();
        let block = GenesisBuilder::build(&config).unwrap();
        // State root should be non-zero
        assert_ne!(block.state_root, [0u8; 32]);
    }

    #[test]
    fn test_genesis_block_genesis_hash() {
        let config = make_test_config();
        let block = GenesisBuilder::build(&config).unwrap();
        assert_eq!(block.genesis_hash, config.genesis_hash());
    }

    #[test]
    fn test_genesis_block_signing() {
        let config = make_test_config();
        let mut block = GenesisBuilder::build(&config).unwrap();

        let sig = [0xAA; 64];
        GenesisBuilder::sign(&mut block, sig).unwrap();
        assert_eq!(block.signature, Some(sig));

        // Double-sign should fail
        let result = GenesisBuilder::sign(&mut block, [0xBB; 64]);
        assert_eq!(result, Err(GenesisError::AlreadySigned));
    }

    #[test]
    fn test_genesis_block_verify_unsigned() {
        let config = make_test_config();
        let block = GenesisBuilder::build(&config).unwrap();
        assert_eq!(GenesisBuilder::verify(&block), Err(GenesisError::NotSigned));
    }

    #[test]
    fn test_genesis_block_verify_signed() {
        let config = make_test_config();
        let mut block = GenesisBuilder::build(&config).unwrap();
        GenesisBuilder::sign(&mut block, [0xAA; 64]).unwrap();
        assert!(GenesisBuilder::verify(&block).is_ok());
    }

    #[test]
    fn test_genesis_block_block_hash() {
        let config = make_test_config();
        let mut block = GenesisBuilder::build(&config).unwrap();
        let hash1 = block.block_hash();

        GenesisBuilder::sign(&mut block, [0xBB; 64]).unwrap();
        let hash2 = block.block_hash();

        // Hash should change after signing
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_genesis_block_signing_bytes_deterministic() {
        let config = make_test_config();
        let block1 = GenesisBuilder::build(&config).unwrap();
        let block2 = GenesisBuilder::build(&config).unwrap();
        assert_eq!(block1.signing_bytes(), block2.signing_bytes());
    }

    // === State Tests === //

    #[test]
    fn test_genesis_state_balances() {
        let mut state = GenesisState::new();
        state.add_balance("ATCaaa", 1000);
        state.add_balance("ATCaaa", 500); // Add to existing
        state.add_balance("ATCbbb", 2000);

        assert_eq!(state.balances.get("ATCaaa"), Some(&1500));
        assert_eq!(state.balances.get("ATCbbb"), Some(&2000));
    }

    #[test]
    fn test_genesis_state_validators() {
        let mut state = GenesisState::new();
        state.add_validator("did:shivacore:v1", 10000);
        state.add_validator("did:shivacore:v2", 20000);

        assert_eq!(state.validators.len(), 2);
        assert_eq!(state.validators.get("did:shivacore:v1"), Some(&10000));
    }

    #[test]
    fn test_genesis_state_root_deterministic() {
        let mut state1 = GenesisState::new();
        state1.add_balance("ATCaaa", 1000);
        state1.add_validator("did:shivacore:v1", 10000);

        let mut state2 = GenesisState::new();
        state2.add_balance("ATCaaa", 1000);
        state2.add_validator("did:shivacore:v1", 10000);

        assert_eq!(state1.state_root(), state2.state_root());
    }

    #[test]
    fn test_genesis_state_root_changes_with_data() {
        let mut state1 = GenesisState::new();
        state1.add_balance("ATCaaa", 1000);

        let mut state2 = GenesisState::new();
        state2.add_balance("ATCaaa", 2000); // Different amount

        assert_ne!(state1.state_root(), state2.state_root());
    }

    // === Export Tests === //

    #[test]
    fn test_genesis_export_json() {
        let config = make_test_config();
        let block = GenesisBuilder::build(&config).unwrap();
        let json = GenesisBuilder::export_json(&block);

        assert!(json.contains("\"chain_id\": 658467"));
        assert!(json.contains("\"height\": 0"));
        assert!(json.contains("\"validators\":"));
        assert!(json.contains("\"allocations\":"));
        assert!(json.contains("A-TownChain Mainnet Genesis"));
    }

    #[test]
    fn test_genesis_export_json_has_hash() {
        let config = make_test_config();
        let block = GenesisBuilder::build(&config).unwrap();
        let json = GenesisBuilder::export_json(&block);

        assert!(json.contains("\"genesis_hash\": \"0x"));
        assert!(json.contains("\"state_root\": \"0x"));
    }

    // === Consensus Params === //

    #[test]
    fn test_consensus_params_defaults() {
        let cp = ConsensusParams::default();
        assert_eq!(cp.block_time, 3);
        assert_eq!(cp.max_txs_per_block, 500);
        assert_eq!(cp.threshold, (CONSENSUS_THRESHOLD * 10000.0) as u64); // 66.67%
        assert_eq!(cp.slash_factor, 1000); // 10%
        assert_eq!(cp.max_block_size, MAX_BLOCK_SIZE);
    }

    #[test]
    fn test_network_params_defaults() {
        let np = NetworkParams::default();
        assert_eq!(np.chain_id, 658467);
        assert_eq!(np.p2p_port, 9000);
        assert_eq!(np.rpc_port, 9001);
        assert_eq!(np.max_peers, 50);
        assert_eq!(np.protocol_version, 1);
    }

    // === Integration with atcnet === //

    #[test]
    fn test_genesis_chain_id_matches_atcnet() {
        assert_eq!(GENESIS_CHAIN_ID, crate::atcnet::CHAIN_ID);
    }

    #[test]
    fn test_genesis_protocol_version_matches_atcnet() {
        let np = NetworkParams::default();
        assert_eq!(np.protocol_version, crate::atcnet::PROTOCOL_VERSION);
    }

    // === LockType Tests === //

    #[test]
    fn test_lock_type_variants() {
        assert_ne!(LockType::None as u8, LockType::Vesting as u8);
        assert_ne!(LockType::Vesting as u8, LockType::TimeLock as u8);
        assert_ne!(LockType::None as u8, LockType::TimeLock as u8);
    }

    #[test]
    fn test_vesting_allocation() {
        let mut config = make_test_config();
        config.add_allocation(GenesisAllocation {
            address: dummy_address(9),
            amount: 5_000_000_000, // 50 ATC
            lock_type: LockType::Vesting,
            lock_duration: 1000, // 1000 blocks
        }).unwrap();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_timelock_allocation() {
        let mut config = make_test_config();
        config.add_allocation(GenesisAllocation {
            address: dummy_address(8),
            amount: 10_000_000_000, // 100 ATC
            lock_type: LockType::TimeLock,
            lock_duration: 5000, // 5000 blocks
        }).unwrap();
        assert!(config.validate().is_ok());
    }

    // === Constants Tests === //

    #[test]
    fn test_genesis_constants() {
        assert_eq!(GENESIS_CHAIN_ID, 658467);
        assert_eq!(DEFAULT_BLOCK_TIME, 3);
        assert_eq!(DEFAULT_MAX_TXS, 500);
        assert_eq!(MIN_VALIDATOR_STAKE, 1000);
        assert_eq!(MAX_BLOCK_SIZE, 1024 * 1024);
    }

    // === Large Validator Set === //

    #[test]
    fn test_genesis_with_10_validators() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        for i in 1..=10u8 {
            config.add_validator(make_test_validator(i, 50000)).unwrap();
        }
        for i in 1..=10u8 {
            config.add_allocation(GenesisAllocation {
                address: dummy_address(i),
                amount: 1_000_000_000,
                lock_type: LockType::None,
                lock_duration: 0,
            }).unwrap();
        }
        assert!(config.validate().is_ok());

        let block = GenesisBuilder::build(&config).unwrap();
        assert_eq!(block.validator_set.len(), 10);
    }

    #[test]
    fn test_genesis_too_many_validators() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, GENESIS_TIMESTAMP);
        for i in 1..=101u8 {
            let v = GenesisValidator {
                did: format!("did:shivacore:v{}", i),
                pubkey: dummy_pubkey(i),
                stake: 10000,
                address: format!("ATC{}", "v".repeat(31)),
                commission: 500,
            };
            config.add_validator(v).unwrap();
        }
        assert_eq!(config.validate(), Err(GenesisError::TooManyValidators));
    }
}
