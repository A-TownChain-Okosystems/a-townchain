// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — Genesis Bridge (K-Sprint 27)
//!
//! Verbindet genesis.rs (K26) mit blockchain.rs (K18) und consensus.rs (K16).
//!
//! 6 Integration-Gaps geschlossen:
//!   1. GenesisBlock → Block Konverter (mit state_root + validator_set)
//!   2. PoH Seed mit echtem Genesis-Hash (statt [0x42;32])
//!   3. Validator Bulk-Init aus GenesisConfig
//!   4. State Root verbunden (GenesisState → Block.state_root)
//!   5. Chain-ID-Validierung in add_block (658467)
//!   6. Genesis Signatur-Verifikation

use alloc::format;
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::genesis::{
    GenesisConfig, GenesisBlock, GenesisState, GenesisBuilder, GenesisValidator,
    GenesisError, GENESIS_CHAIN_ID, LockType,
};

// === Minimal Block (compatible mit blockchain.rs) === //

/// Block-Struktur — kompatibel mit blockchain.rs::Block
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBlock {
    pub id: [u8; 32],
    pub height: u64,
    pub parent_hash: [u8; 32],
    pub proposer_did: String,
    pub timestamp: u64,
    pub poh_hash: [u8; 32],
    pub tx_root: [u8; 32],
    pub state_root: [u8; 32],
    pub gas_used: u64,
    pub total_fees: u64,
    pub signature: [u8; 64],
    pub chain_id: u32,
    pub validator_set: Vec<(String, u64)>,
    pub allocations: Vec<(String, u64)>,
}

impl BridgeBlock {
    /// Erzeugt einen Genesis-Block aus einer GenesisBlock-Konfiguration
    pub fn from_genesis(genesis: &GenesisBlock) -> Self {
        BridgeBlock {
            id: genesis.genesis_hash,
            height: 0,
            parent_hash: [0u8; 32],
            proposer_did: String::new(), // Genesis hat keinen Proposer
            timestamp: genesis.timestamp,
            poh_hash: genesis.genesis_hash, // PoH wird mit Genesis-Hash geseedt
            tx_root: [0u8; 32], // Genesis hat keine Txs
            state_root: genesis.state_root,
            gas_used: 0,
            total_fees: 0,
            signature: genesis.signature.unwrap_or([0u8; 64]),
            chain_id: genesis.chain_id,
            validator_set: genesis.validator_set.clone(),
            allocations: genesis.allocations.clone(),
        }
    }

    pub fn is_genesis(&self) -> bool {
        self.height == 0 && self.parent_hash == [0u8; 32]
    }
}

// === Minimal PoH (compatible mit consensus.rs::PohSequence) === //

/// Proof of History — kompatibel mit consensus.rs::PohSequence
pub struct BridgePoh {
    current_hash: [u8; 32],
    tick: u64,
    entries: Vec<BridgePohEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePohEntry {
    pub hash: [u8; 32],
    pub timestamp: u64,
    pub tick: u64,
}

impl BridgePoh {
    /// NEU: Wird mit dem Genesis-Hash geseedt (nicht mehr [0x42;32])
    pub fn new(genesis_hash: [u8; 32]) -> Self {
        BridgePoh {
            current_hash: genesis_hash,
            tick: 0,
            entries: Vec::new(),
        }
    }

    pub fn tick(&mut self, timestamp: u64) -> BridgePohEntry {
        let mut input = Vec::with_capacity(40);
        input.extend_from_slice(&self.current_hash);
        input.extend_from_slice(&self.tick.to_be_bytes());
        let new_hash = simple_hash(&input);

        let entry = BridgePohEntry {
            hash: new_hash,
            timestamp,
            tick: self.tick,
        };

        self.current_hash = new_hash;
        self.tick += 1;
        self.entries.push(entry.clone());
        entry
    }

    pub fn current_hash(&self) -> [u8; 32] {
        self.current_hash
    }

    pub fn tick_count(&self) -> u64 {
        self.tick
    }

    pub fn verify(start_hash: [u8; 32], entries: &[BridgePohEntry]) -> bool {
        let mut expected = start_hash;
        for entry in entries {
            let mut input = Vec::with_capacity(40);
            input.extend_from_slice(&expected);
            input.extend_from_slice(&entry.tick.to_be_bytes());
            let computed = simple_hash(&input);
            if computed != entry.hash {
                return false;
            }
            expected = entry.hash;
        }
        true
    }
}

// === Minimal Validator Registry (compatible mit consensus.rs::ValidatorRegistry) === //

#[derive(Debug, Clone)]
pub struct BridgeValidator {
    pub did: String,
    pub stake: u64,
    pub active: bool,
    pub pubkey: [u8; 33],
    pub commission: u16,
    pub votes_cast: u64,
    pub blocks_proposed: u64,
}

pub struct BridgeValidatorRegistry {
    validators: BTreeMap<String, BridgeValidator>,
    total_stake: u64,
}

impl BridgeValidatorRegistry {
    pub fn new() -> Self {
        BridgeValidatorRegistry {
            validators: BTreeMap::new(),
            total_stake: 0,
        }
    }

    /// NEU: Bulk-Initialisierung aus GenesisConfig
    pub fn from_genesis(config: &GenesisConfig) -> Result<Self, GenesisError> {
        config.validate()?;
        let mut registry = Self::new();
        for v in &config.validators {
            registry.register(v.clone());
        }
        Ok(registry)
    }

    pub fn register(&mut self, v: GenesisValidator) {
        self.total_stake += v.stake;
        self.validators.insert(v.did.clone(), BridgeValidator {
            did: v.did.clone(),
            stake: v.stake,
            active: true,
            pubkey: v.pubkey,
            commission: v.commission,
            votes_cast: 0,
            blocks_proposed: 0,
        });
    }

    pub fn deactivate(&mut self, did: &str) {
        if let Some(v) = self.validators.get_mut(did) {
            v.active = false;
        }
    }

    pub fn is_active(&self, did: &str) -> bool {
        self.validators.get(did).map(|v| v.active).unwrap_or(false)
    }

    pub fn get_stake(&self, did: &str) -> u64 {
        self.validators.get(did).map(|v| v.stake).unwrap_or(0)
    }

    pub fn total_stake(&self) -> u64 {
        self.total_stake
    }

    pub fn validator_count(&self) -> usize {
        self.validators.len()
    }

    pub fn active_count(&self) -> usize {
        self.validators.values().filter(|v| v.active).count()
    }

    /// Stake-weighted Proposer-Auswahl (PoH-seeded)
    pub fn select_proposer(&self, poh_hash: &[u8; 32]) -> Option<String> {
        let active: Vec<&BridgeValidator> = self.validators.values().filter(|v| v.active).collect();
        if active.is_empty() || self.total_stake == 0 {
            return None;
        }

        let hash_val = u64::from_be_bytes(poh_hash[..8].try_into().unwrap());
        let target = hash_val % self.total_stake;

        let mut acc: u64 = 0;
        for v in &active {
            acc += v.stake;
            if acc > target {
                return Some(v.did.clone());
            }
        }
        active.last().map(|v| v.did.clone())
    }
}

// === Minimal BlockChain (compatible mit blockchain.rs::BlockChain) === //

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeChainError {
    InvalidChainId,
    InvalidHeight,
    BlockExists,
    DuplicateBlock,
    GenesisExists,
    ParentNotFound,
    InvalidSignature,
}

#[derive(Clone)]
pub struct BridgeBlockChain {
    blocks: BTreeMap<u64, BridgeBlock>,
    by_hash: BTreeMap<[u8; 32], u64>,
    current_height: u64,
    genesis_hash: Option<[u8; 32]>,
    chain_id: u32,
}

impl BridgeBlockChain {
    pub fn new() -> Self {
        BridgeBlockChain {
            blocks: BTreeMap::new(),
            by_hash: BTreeMap::new(),
            current_height: 0,
            genesis_hash: None,
            chain_id: 0, // Wird bei add_genesis gesetzt
        }
    }

    /// NEU: Chain-ID wird beim Genesis-Block gesetzt
    pub fn add_genesis(&mut self, block: BridgeBlock) -> Result<(), BridgeChainError> {
        if block.height != 0 {
            return Err(BridgeChainError::InvalidHeight);
        }
        if self.blocks.contains_key(&0) {
            return Err(BridgeChainError::GenesisExists);
        }
        // NEU: Chain-ID-Validierung
        if block.chain_id != GENESIS_CHAIN_ID {
            return Err(BridgeChainError::InvalidChainId);
        }
        // NEU: Signatur-Verifikation
        if block.signature == [0u8; 64] {
            return Err(BridgeChainError::InvalidSignature);
        }

        self.chain_id = block.chain_id;
        self.genesis_hash = Some(block.id);
        self.by_hash.insert(block.id, 0);
        self.blocks.insert(0, block);
        self.current_height = 0;
        Ok(())
    }

    /// NEU: Chain-ID wird bei jedem Block validiert
    pub fn add_block(&mut self, block: BridgeBlock) -> Result<(), BridgeChainError> {
        if block.chain_id != self.chain_id {
            return Err(BridgeChainError::InvalidChainId);
        }
        if block.height != self.current_height + 1 {
            return Err(BridgeChainError::InvalidHeight);
        }
        if self.blocks.contains_key(&block.height) {
            return Err(BridgeChainError::BlockExists);
        }
        if self.by_hash.contains_key(&block.id) {
            return Err(BridgeChainError::DuplicateBlock);
        }
        // Parent muss existieren
        if !self.by_hash.contains_key(&block.parent_hash) {
            return Err(BridgeChainError::ParentNotFound);
        }

        self.by_hash.insert(block.id, block.height);
        self.blocks.insert(block.height, block);
        self.current_height += 1;
        Ok(())
    }

    pub fn get_block(&self, height: u64) -> Option<&BridgeBlock> {
        self.blocks.get(&height)
    }

    pub fn get_by_hash(&self, hash: &[u8; 32]) -> Option<&BridgeBlock> {
        self.by_hash.get(hash).and_then(|&h| self.blocks.get(&h))
    }

    pub fn current_height(&self) -> u64 {
        self.current_height
    }

    pub fn block_count(&self) -> usize {
        self.blocks.len()
    }

    pub fn last_block(&self) -> Option<&BridgeBlock> {
        self.blocks.get(&self.current_height)
    }

    pub fn chain_id(&self) -> u32 {
        self.chain_id
    }

    pub fn genesis_hash(&self) -> Option<[u8; 32]> {
        self.genesis_hash
    }
}

// === Genesis Bridge — Hauptkomponente === //

/// Verbindet GenesisConfig → BlockChain + ValidatorRegistry + PoH
pub struct GenesisBridge {
    pub chain: BridgeBlockChain,
    pub validators: BridgeValidatorRegistry,
    pub poh: BridgePoh,
    pub state: GenesisState,
    pub config_hash: [u8; 32],
}

impl GenesisBridge {
    /// Initialisiert die komplette Chain aus einer Genesis-Konfiguration
    pub fn init_from_config(config: &GenesisConfig) -> Result<Self, GenesisError> {
        // 1. Konfiguration validieren
        config.validate()?;

        // 2. Genesis-Block bauen und signieren
        let mut genesis_block = GenesisBuilder::build(config)?;
        // Simulierte Signatur (in Produktion: echte ECDSA/Ed25519)
        let signing_bytes = genesis_block.signing_bytes();
        GenesisBuilder::sign(&mut genesis_block, simple_signature(&signing_bytes))?;

        // 3. Genesis-Block verifizieren
        GenesisBuilder::verify(&genesis_block)?;

        // 4. Zu BridgeBlock konvertieren
        let bridge_block = BridgeBlock::from_genesis(&genesis_block);

        // 5. BlockChain initialisieren
        let mut chain = BridgeBlockChain::new();
        chain.add_genesis(bridge_block).map_err(|_| GenesisError::InvalidChainId)?;

        // 6. Validator Registry aus Konfiguration
        let validators = BridgeValidatorRegistry::from_genesis(config)?;

        // 7. PoH mit Genesis-Hash seeden
        let poh = BridgePoh::new(genesis_block.genesis_hash);
        let mut poh = poh;
        let genesis_tick = poh.tick(config.timestamp);

        // 8. Genesis State aufbauen
        let mut state = GenesisState::new();
        for v in &config.validators {
            state.add_validator(&v.did, v.stake);
        }
        for a in &config.allocations {
            state.add_balance(&a.address, a.amount);
        }

        Ok(GenesisBridge {
            chain,
            validators,
            poh,
            state,
            config_hash: genesis_block.genesis_hash,
        })
    }

    /// Erzeugt den nächsten Block (Post-Genesis)
    pub fn propose_block(
        &mut self,
        proposer_did: &str,
        timestamp: u64,
        tx_root: [u8; 32],
    ) -> Result<BridgeBlock, BridgeChainError> {
        let parent = self.chain.last_block().ok_or(BridgeChainError::ParentNotFound)?;
        let poh_entry = self.poh.tick(timestamp);
        let height = parent.height + 1;

        // Block-ID berechnen
        let mut bi = Vec::new();
        bi.extend_from_slice(&height.to_be_bytes());
        bi.extend_from_slice(&parent.id);
        bi.extend_from_slice(proposer_did.as_bytes());
        bi.extend_from_slice(&timestamp.to_be_bytes());
        bi.extend_from_slice(&poh_entry.hash);
        bi.extend_from_slice(&tx_root);
        let block_id = simple_hash(&bi);

        let block = BridgeBlock {
            id: block_id,
            height,
            parent_hash: parent.id,
            proposer_did: proposer_did.to_string(),
            timestamp,
            poh_hash: poh_entry.hash,
            tx_root,
            state_root: self.state.state_root(),
            gas_used: 0,
            total_fees: 0,
            signature: [0u8; 64], // In Produktion: Proposer signiert
            chain_id: self.chain.chain_id(),
            validator_set: vec![],
            allocations: vec![],
        };

        self.chain.add_block(block.clone())?;
        Ok(block)
    }

    /// Wählt den nächsten Proposer (stake-weighted, PoH-seeded)
    pub fn next_proposer(&self) -> Option<String> {
        self.validators.select_proposer(&self.poh.current_hash())
    }

    /// Gesamt-Stake aller aktiven Validator
    pub fn total_stake(&self) -> u64 {
        self.validators.total_stake()
    }

    /// Anzahl aktiver Validator
    pub fn active_validators(&self) -> usize {
        self.validators.active_count()
    }

    /// Chain-ID
    pub fn chain_id(&self) -> u32 {
        self.chain.chain_id()
    }

    /// Genesis-Hash
    pub fn genesis_hash(&self) -> [u8; 32] {
        self.config_hash
    }

    /// Aktueller State-Root
    pub fn state_root(&self) -> [u8; 32] {
        self.state.state_root()
    }

    /// Blockhöhe
    pub fn height(&self) -> u64 {
        self.chain.current_height()
    }
}

// === Helper Functions === //

fn simple_hash(data: &[u8]) -> [u8; 32] {
    let mut result = [0u8; 32];
    let mut h1: u64 = 0xcbf29ce484222325;
    let mut h2: u64 = 0x84222325cbf29ce4;
    for (i, &b) in data.iter().enumerate() {
        h1 ^= b as u64;
        h1 = h1.wrapping_mul(0x100000001b3);
        h2 = h2.wrapping_add(b as u64);
        h2 ^= h2 >> 33;
        let off = (i * 4) % 24;
        if i % 2 == 0 {
            result[off..off + 8].copy_from_slice(&h1.to_le_bytes());
        } else {
            result[off..off + 8].copy_from_slice(&h2.to_le_bytes());
        }
    }
    result
}

fn simple_signature(data: &[u8]) -> [u8; 64] {
    let h1 = simple_hash(data);
    let h2 = simple_hash(&h1);
    let mut sig = [0u8; 64];
    sig[..32].copy_from_slice(&h1);
    sig[32..].copy_from_slice(&h2);
    sig
}

// === Tests === //

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_pubkey(n: u8) -> [u8; 33] {
        let mut k = [0u8; 33];
        k[0] = 0x02;
        k[1] = n;
        k
    }

    fn dummy_address(n: u8) -> String {
        format!("ATC{}", "a".repeat(30).chars().chain(core::iter::once((b'a' + n) as char)).collect::<String>())
    }

    fn dummy_did(n: u8) -> String {
        format!("did:shivacore:validator{}", n)
    }

    fn make_validator(n: u8, stake: u64) -> GenesisValidator {
        GenesisValidator {
            did: dummy_did(n),
            pubkey: dummy_pubkey(n),
            stake,
            address: dummy_address(n),
            commission: 500,
        }
    }

    fn make_test_config() -> GenesisConfig {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, 1726358400);
        for i in 1..=4u8 {
            config.add_validator(make_validator(i, 10000)).unwrap();
        }
        for i in 1..=4u8 {
            config.add_allocation(crate::genesis::GenesisAllocation {
                address: dummy_address(i),
                amount: 1_000_000_000,
                lock_type: LockType::None,
                lock_duration: 0,
            }).unwrap();
        }
        config.memo = "A-TownChain Mainnet Genesis".to_string();
        config
    }

    // === Gap 1: GenesisBlock → BridgeBlock === //

    #[test]
    fn test_genesis_to_bridge_block_conversion() {
        let config = make_test_config();
        let mut genesis = GenesisBuilder::build(&config).unwrap();
        let sb = genesis.signing_bytes();
        GenesisBuilder::sign(&mut genesis, simple_signature(&sb)).unwrap();

        let bridge_block = BridgeBlock::from_genesis(&genesis);

        assert_eq!(bridge_block.height, 0);
        assert_eq!(bridge_block.chain_id, 658467);
        assert!(bridge_block.is_genesis());
        assert_eq!(bridge_block.state_root, genesis.state_root);
        assert_eq!(bridge_block.validator_set.len(), 4);
        assert_eq!(bridge_block.allocations.len(), 4);
        assert_ne!(bridge_block.signature, [0u8; 64]); // Signed
    }

    #[test]
    fn test_bridge_block_genesis_id_matches() {
        let config = make_test_config();
        let mut genesis = GenesisBuilder::build(&config).unwrap();
        GenesisBuilder::sign(&mut genesis, [0xAA; 64]).unwrap();

        let bridge_block = BridgeBlock::from_genesis(&genesis);
        assert_eq!(bridge_block.id, genesis.genesis_hash);
    }

    // === Gap 2: PoH seeded with Genesis-Hash === //

    #[test]
    fn test_poh_seeded_with_genesis_hash() {
        let config = make_test_config();
        let genesis_hash = config.genesis_hash();

        let mut poh = BridgePoh::new(genesis_hash);
        assert_eq!(poh.current_hash(), genesis_hash);

        let entry = poh.tick(1000);
        assert_ne!(entry.hash, genesis_hash); // Should advance
        assert_eq!(entry.tick, 0);
        assert_eq!(entry.timestamp, 1000);
    }

    #[test]
    fn test_poh_verify_from_genesis() {
        let config = make_test_config();
        let genesis_hash = config.genesis_hash();

        let mut poh = BridgePoh::new(genesis_hash);
        let entries: Vec<BridgePohEntry> = (0..5).map(|i| poh.tick(i * 1000)).collect();

        assert!(BridgePoh::verify(genesis_hash, &entries));
    }

    #[test]
    fn test_poh_verify_fails_with_wrong_seed() {
        let config = make_test_config();
        let genesis_hash = config.genesis_hash();
        let wrong_seed = [0x99; 32];

        let mut poh = BridgePoh::new(genesis_hash);
        let entries: Vec<BridgePohEntry> = (0..3).map(|i| poh.tick(i * 1000)).collect();

        assert!(!BridgePoh::verify(wrong_seed, &entries)); // Wrong seed → fails
    }

    #[test]
    fn test_poh_tick_count() {
        let mut poh = BridgePoh::new([0x42; 32]);
        for i in 0..10 {
            poh.tick(i * 100);
        }
        assert_eq!(poh.tick_count(), 10);
    }

    // === Gap 3: Validator Bulk-Init === //

    #[test]
    fn test_validator_registry_from_genesis() {
        let config = make_test_config();
        let registry = BridgeValidatorRegistry::from_genesis(&config).unwrap();

        assert_eq!(registry.validator_count(), 4);
        assert_eq!(registry.active_count(), 4);
        assert_eq!(registry.total_stake(), 40000);
    }

    #[test]
    fn test_validator_registry_deactivate() {
        let config = make_test_config();
        let mut registry = BridgeValidatorRegistry::from_genesis(&config).unwrap();

        registry.deactivate(&dummy_did(2));
        assert_eq!(registry.active_count(), 3);
        assert!(!registry.is_active(&dummy_did(2)));
        assert!(registry.is_active(&dummy_did(1)));
    }

    #[test]
    fn test_validator_registry_get_stake() {
        let config = make_test_config();
        let registry = BridgeValidatorRegistry::from_genesis(&config).unwrap();

        assert_eq!(registry.get_stake(&dummy_did(1)), 10000);
        assert_eq!(registry.get_stake(&dummy_did(4)), 10000);
        assert_eq!(registry.get_stake("did:shivacore:nonexistent"), 0);
    }

    #[test]
    fn test_validator_select_proposer() {
        let config = make_test_config();
        let registry = BridgeValidatorRegistry::from_genesis(&config).unwrap();

        let poh_hash = config.genesis_hash();
        let proposer = registry.select_proposer(&poh_hash);
        assert!(proposer.is_some());
        assert!(proposer.unwrap().starts_with("did:shivacore:validator"));
    }

    #[test]
    fn test_validator_select_proposer_deterministic() {
        let config = make_test_config();
        let registry = BridgeValidatorRegistry::from_genesis(&config).unwrap();

        let poh_hash = config.genesis_hash();
        let p1 = registry.select_proposer(&poh_hash);
        let p2 = registry.select_proposer(&poh_hash);
        assert_eq!(p1, p2); // Same PoH hash → same proposer
    }

    #[test]
    fn test_validator_select_proposer_different_hashes() {
        let config = make_test_config();
        let registry = BridgeValidatorRegistry::from_genesis(&config).unwrap();

        let h1 = [0x01; 32];
        let h2 = [0x02; 32];
        // Different PoH hashes may select different proposers
        let p1 = registry.select_proposer(&h1);
        let p2 = registry.select_proposer(&h2);
        assert!(p1.is_some());
        assert!(p2.is_some());
    }

    // === Gap 4: State Root verbunden === //

    #[test]
    fn test_state_root_in_genesis_block() {
        let config = make_test_config();
        let mut genesis = GenesisBuilder::build(&config).unwrap();
        GenesisBuilder::sign(&mut genesis, [0xAA; 64]).unwrap();

        let bridge_block = BridgeBlock::from_genesis(&genesis);
        // State root should be non-zero (computed from balances + validators)
        assert_ne!(bridge_block.state_root, [0u8; 32]);
    }

    #[test]
    fn test_state_root_deterministic() {
        let config1 = make_test_config();
        let config2 = make_test_config();

        let g1 = GenesisBuilder::build(&config1).unwrap();
        let g2 = GenesisBuilder::build(&config2).unwrap();

        assert_eq!(g1.state_root, g2.state_root);
    }

    #[test]
    fn test_state_root_changes_with_allocations() {
        let mut config1 = make_test_config();
        let mut config2 = make_test_config();
        config2.add_allocation(crate::genesis::GenesisAllocation {
            address: dummy_address(9),
            amount: 5_000_000_000,
            lock_type: LockType::None,
            lock_duration: 0,
        }).unwrap();

        let g1 = GenesisBuilder::build(&config1).unwrap();
        let g2 = GenesisBuilder::build(&config2).unwrap();

        assert_ne!(g1.state_root, g2.state_root);
    }

    // === Gap 5: Chain-ID-Validierung === //

    #[test]
    fn test_chain_id_in_genesis_block() {
        let config = make_test_config();
        let mut genesis = GenesisBuilder::build(&config).unwrap();
        GenesisBuilder::sign(&mut genesis, [0xAA; 64]).unwrap();

        let bridge_block = BridgeBlock::from_genesis(&genesis);
        assert_eq!(bridge_block.chain_id, 658467);
    }

    #[test]
    fn test_chain_rejects_wrong_chain_id() {
        let mut chain = BridgeBlockChain::new();
        let mut block = BridgeBlock {
            id: [0x01; 32],
            height: 0,
            parent_hash: [0u8; 32],
            proposer_did: String::new(),
            timestamp: 1000,
            poh_hash: [0u8; 32],
            tx_root: [0u8; 32],
            state_root: [0u8; 32],
            gas_used: 0,
            total_fees: 0,
            signature: [0xAA; 64],
            chain_id: 9999, // Wrong chain
            validator_set: vec![],
            allocations: vec![],
        };
        assert_eq!(chain.add_genesis(block), Err(BridgeChainError::InvalidChainId));
    }

    #[test]
    fn test_chain_rejects_unsigned_genesis() {
        let mut chain = BridgeBlockChain::new();
        let block = BridgeBlock {
            id: [0x01; 32],
            height: 0,
            parent_hash: [0u8; 32],
            proposer_did: String::new(),
            timestamp: 1000,
            poh_hash: [0u8; 32],
            tx_root: [0u8; 32],
            state_root: [0u8; 32],
            gas_used: 0,
            total_fees: 0,
            signature: [0u8; 64], // Unsigned!
            chain_id: 658467,
            validator_set: vec![],
            allocations: vec![],
        };
        assert_eq!(chain.add_genesis(block), Err(BridgeChainError::InvalidSignature));
    }

    #[test]
    fn test_chain_rejects_duplicate_genesis() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        let mut genesis = GenesisBuilder::build(&config).unwrap();
        let sb = genesis.signing_bytes();
        GenesisBuilder::sign(&mut genesis, simple_signature(&sb)).unwrap();
        let block2 = BridgeBlock::from_genesis(&genesis);

        let mut chain = bridge.chain.clone();
        let result = chain.add_genesis(block2);
        assert_eq!(result, Err(BridgeChainError::GenesisExists));
    }

    #[test]
    fn test_chain_accepts_valid_genesis() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        assert_eq!(bridge.chain_id(), 658467);
        assert_eq!(bridge.height(), 0);
        assert_eq!(bridge.chain.block_count(), 1);
    }

    // === Gap 6: Genesis Signatur-Verifikation === //

    #[test]
    fn test_genesis_signature_in_bridge_block() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        let genesis = bridge.chain.get_block(0).unwrap();
        assert_ne!(genesis.signature, [0u8; 64]); // Must be signed
    }

    #[test]
    fn test_unsigned_genesis_rejected_by_chain() {
        let config = make_test_config();
        let mut genesis = GenesisBuilder::build(&config).unwrap();
        // Don't sign

        let block = BridgeBlock::from_genesis(&genesis);
        let mut chain = BridgeBlockChain::new();
        assert_eq!(chain.add_genesis(block), Err(BridgeChainError::InvalidSignature));
    }

    // === Full Bridge Integration === //

    #[test]
    fn test_bridge_init_from_config() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        assert_eq!(bridge.chain_id(), 658467);
        assert_eq!(bridge.height(), 0);
        assert_eq!(bridge.active_validators(), 4);
        assert_eq!(bridge.total_stake(), 40000);
        assert_ne!(bridge.genesis_hash(), [0u8; 32]);
        assert_ne!(bridge.state_root(), [0u8; 32]);
    }

    #[test]
    fn test_bridge_poh_seeded_correctly() {
        let config = make_test_config();
        let genesis_hash = config.genesis_hash();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        // PoH should be seeded with genesis hash
        assert_eq!(bridge.poh.current_hash() != genesis_hash, true); // Advanced by init tick
        assert_eq!(bridge.poh.tick_count(), 1); // One tick from init
    }

    #[test]
    fn test_bridge_propose_block() {
        let config = make_test_config();
        let mut bridge = GenesisBridge::init_from_config(&config).unwrap();

        let proposer = bridge.next_proposer().unwrap();
        let block = bridge.propose_block(&proposer, 2000, [0xAB; 32]).unwrap();

        assert_eq!(block.height, 1);
        assert_eq!(block.chain_id, 658467);
        assert_eq!(block.parent_hash, bridge.genesis_hash());
        assert_eq!(block.proposer_did, proposer);
        assert_ne!(block.poh_hash, [0u8; 32]);
        assert_ne!(block.state_root, [0u8; 32]);
    }

    #[test]
    fn test_bridge_multi_blocks() {
        let config = make_test_config();
        let mut bridge = GenesisBridge::init_from_config(&config).unwrap();

        for i in 1..=5 {
            let proposer = bridge.next_proposer().unwrap();
            let block = bridge.propose_block(&proposer, 2000 + i * 100, [0xAB; 32]).unwrap();
            assert_eq!(block.height, i as u64);
        }

        assert_eq!(bridge.height(), 5);
        assert_eq!(bridge.chain.block_count(), 6); // Genesis + 5
    }

    #[test]
    fn test_bridge_chain_id_consistent_across_blocks() {
        let config = make_test_config();
        let mut bridge = GenesisBridge::init_from_config(&config).unwrap();

        for i in 1..=3 {
            let proposer = bridge.next_proposer().unwrap();
            let block = bridge.propose_block(&proposer, 2000 + i, [0xCD; 32]).unwrap();
            assert_eq!(block.chain_id, 658467);
        }
    }

    #[test]
    fn test_bridge_block_height_strict() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        // Try to add block at height 5 (should fail, height 1 expected)
        let genesis_hash = bridge.genesis_hash();
        let bad_block = BridgeBlock {
            id: [0xFF; 32],
            height: 5,
            parent_hash: genesis_hash,
            proposer_did: dummy_did(1),
            timestamp: 5000,
            poh_hash: [0u8; 32],
            tx_root: [0u8; 32],
            state_root: [0u8; 32],
            gas_used: 0,
            total_fees: 0,
            signature: [0xAA; 64],
            chain_id: 658467,
            validator_set: vec![],
            allocations: vec![],
        };
        let mut chain = bridge.chain.clone();
        assert_eq!(chain.add_block(bad_block), Err(BridgeChainError::InvalidHeight));
    }

    #[test]
    fn test_bridge_validator_set_in_genesis() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        let genesis = bridge.chain.get_block(0).unwrap();
        assert_eq!(genesis.validator_set.len(), 4);
        for (did, stake) in &genesis.validator_set {
            assert!(did.starts_with("did:shivacore:validator"));
            assert_eq!(*stake, 10000);
        }
    }

    #[test]
    fn test_bridge_allocations_in_genesis() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        let genesis = bridge.chain.get_block(0).unwrap();
        assert_eq!(genesis.allocations.len(), 4);
        for (addr, amount) in &genesis.allocations {
            assert!(addr.starts_with("ATC"));
            assert_eq!(*amount, 1_000_000_000);
        }
    }

    #[test]
    fn test_bridge_state_balances() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        assert_eq!(bridge.state.balances.len(), 4);
        for (_, balance) in &bridge.state.balances {
            assert_eq!(*balance, 1_000_000_000);
        }
    }

    #[test]
    fn test_bridge_state_validators() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        assert_eq!(bridge.state.validators.len(), 4);
        for (_, stake) in &bridge.state.validators {
            assert_eq!(*stake, 10000);
        }
    }

    #[test]
    fn test_bridge_next_proposer_is_active_validator() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        let proposer = bridge.next_proposer().unwrap();
        assert!(bridge.validators.is_active(&proposer));
    }

    #[test]
    fn test_bridge_init_with_10_validators() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, 1726358400);
        for i in 1..=10u8 {
            config.add_validator(make_validator(i, 50000)).unwrap();
        }
        for i in 1..=10u8 {
            config.add_allocation(crate::genesis::GenesisAllocation {
                address: dummy_address(i),
                amount: 1_000_000_000,
                lock_type: LockType::None,
                lock_duration: 0,
            }).unwrap();
        }
        config.memo = "10-validator test".to_string();

        let bridge = GenesisBridge::init_from_config(&config).unwrap();
        assert_eq!(bridge.active_validators(), 10);
        assert_eq!(bridge.total_stake(), 500000);
    }

    #[test]
    fn test_bridge_init_rejects_invalid_config() {
        let mut config = GenesisConfig::new(9999, 1726358400); // Wrong chain ID
        config.add_validator(make_validator(1, 10000)).unwrap();
        assert!(GenesisBridge::init_from_config(&config).is_err());
    }

    #[test]
    fn test_bridge_get_block_by_hash() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();

        let genesis_hash = bridge.genesis_hash();
        let block = bridge.chain.get_by_hash(&genesis_hash);
        assert!(block.is_some());
        assert_eq!(block.unwrap().height, 0);
    }

    #[test]
    fn test_bridge_clone_chain() {
        let config = make_test_config();
        let bridge = GenesisBridge::init_from_config(&config).unwrap();
        let chain_clone = bridge.chain.clone();
        // Clone is fine
        assert_eq!(chain_clone.block_count(), 1);
        assert_eq!(chain_clone.chain_id(), 658467);
    }

    #[test]
    fn test_bridge_poh_advance_on_propose() {
        let config = make_test_config();
        let mut bridge = GenesisBridge::init_from_config(&config).unwrap();

        let poh_before = bridge.poh.tick_count();
        let proposer = bridge.next_proposer().unwrap();
        bridge.propose_block(&proposer, 2000, [0xAB; 32]).unwrap();
        let poh_after = bridge.poh.tick_count();

        assert_eq!(poh_after, poh_before + 1);
    }

    #[test]
    fn test_bridge_state_root_advances_with_blocks() {
        let config = make_test_config();
        let mut bridge = GenesisBridge::init_from_config(&config).unwrap();

        let genesis_state = bridge.state_root();

        // Modify state
        bridge.state.add_balance("ATCnew", 500);
        let new_state = bridge.state_root();

        assert_ne!(genesis_state, new_state);
    }

    #[test]
    fn test_bridge_invalid_config_no_validators() {
        let mut config = GenesisConfig::new(GENESIS_CHAIN_ID, 1726358400);
        config.add_allocation(crate::genesis::GenesisAllocation {
            address: dummy_address(1),
            amount: 1000,
            lock_type: LockType::None,
            lock_duration: 0,
        }).unwrap();
        assert!(GenesisBridge::init_from_config(&config).is_err());
    }
}
