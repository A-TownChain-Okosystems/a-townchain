// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ─────────────────────────────────────────────────────────────────────────
// K-Sprint 16 — Konsens-Mechanismus (ATC-04 DAG + PoH)
// Kernel Layer | Chain-ID 658467
// DAG-Struktur, Proof of History, Validator-Voting, Finality.
// Baut auf K6 (DID), K14 (P2P), K15 (Security) auf.
// ─────────────────────────────────────────────────────────────────────────

use alloc::vec;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use spin::Mutex;

// ═══════════════════════════════════════════════════════════════════════════
// 1. Proof of History (PoH) — sequenzielle Hash-Kette für Zeitordnung
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct PohEntry {
    pub hash: [u8; 32],
    pub timestamp: u64,
    pub tick: u64,
}

pub struct PohSequence {
    current_hash: Mutex<[u8; 32]>,
    tick: Mutex<u64>,
    entries: Mutex<Vec<PohEntry>>,
}

impl PohSequence {
    pub fn new(genesis_hash: [u8; 32]) -> Self {
        PohSequence {
            current_hash: Mutex::new(genesis_hash),
            tick: Mutex::new(0),
            entries: Mutex::new(Vec::new()),
        }
    }

    /// Erzeugt einen neuen PoH-Tick (Hash der Vorgänger-Hash + Tick-Nummer).
    pub fn tick(&self, timestamp: u64) -> PohEntry {
        let mut hash = self.current_hash.lock();
        let mut tick = self.tick.lock();

        let mut input = Vec::with_capacity(40);
        input.extend_from_slice(&*hash);
        input.extend_from_slice(&tick.to_be_bytes());
        let new_hash = crate::security::simple_hash(&input);

        let entry = PohEntry {
            hash: new_hash,
            timestamp,
            tick: *tick,
        };

        *hash = new_hash;
        *tick += 1;
        self.entries.lock().push(entry.clone());
        entry
    }

    /// Erzeugt einen PoH-Eintrag mit Referenz auf ein Event (z.B. Tx-Hash).
    pub fn record(&self, timestamp: u64, event_hash: &[u8; 32]) -> PohEntry {
        let mut hash = self.current_hash.lock();
        let mut tick = self.tick.lock();

        let mut input = Vec::with_capacity(72);
        input.extend_from_slice(&*hash);
        input.extend_from_slice(&tick.to_be_bytes());
        input.extend_from_slice(event_hash);
        let new_hash = crate::security::simple_hash(&input);

        let entry = PohEntry {
            hash: new_hash,
            timestamp,
            tick: *tick,
        };

        *hash = new_hash;
        *tick += 1;
        self.entries.lock().push(entry.clone());
        entry
    }

    pub fn current_hash(&self) -> [u8; 32] { *self.current_hash.lock() }
    pub fn tick_count(&self) -> u64 { *self.tick.lock() }
    pub fn entries(&self) -> Vec<PohEntry> { self.entries.lock().clone() }

    /// Verifiziert eine PoH-Sequenz ab einem Start-Hash.
    pub fn verify(start_hash: [u8; 32], entries: &[PohEntry]) -> bool {
        let mut expected = start_hash;
        for entry in entries {
            let mut input = Vec::with_capacity(40);
            input.extend_from_slice(&expected);
            input.extend_from_slice(&entry.tick.to_be_bytes());
            let computed = crate::security::simple_hash(&input);
            if computed != entry.hash { return false; }
            expected = entry.hash;
        }
        true
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 2. DAG-Vertex (Event) — ATC-04
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VertexType {
    Genesis,
    Transaction,
    Checkpoint,
}

#[derive(Clone, Debug)]
pub struct DagVertex {
    pub id: [u8; 32],
    pub vertex_type: VertexType,
    pub parents: Vec<[u8; 32]>,    // Referenzen auf Vorgänger (DAG!)
    pub creator_did: String,
    pub timestamp: u64,
    pub poh_hash: [u8; 32],        // Proof of History Verknüpfung
    pub payload_hash: [u8; 32],    // Hash der Transaktion/des Events
    pub signature: [u8; 64],       // Ed25519 Signatur des Creators
    pub confirmed: bool,
    pub confirmation_votes: u32,
}

impl DagVertex {
    pub fn new(
        vertex_type: VertexType,
        parents: Vec<[u8; 32]>,
        creator_did: String,
        timestamp: u64,
        poh_hash: [u8; 32],
        payload_hash: [u8; 32],
        signature: [u8; 64],
    ) -> Self {
        // ID = hash(poh_hash + payload_hash + parents + creator)
        let mut input = Vec::new();
        input.extend_from_slice(&poh_hash);
        input.extend_from_slice(&payload_hash);
        for p in &parents { input.extend_from_slice(p); }
        input.extend_from_slice(creator_did.as_bytes());
        let id = crate::security::simple_hash(&input);

        DagVertex {
            id, vertex_type, parents, creator_did,
            timestamp, poh_hash, payload_hash, signature,
            confirmed: false, confirmation_votes: 0,
        }
    }

    pub fn genesis(creator_did: String, timestamp: u64, poh_hash: [u8; 32]) -> Self {
        let payload_hash = [0u8; 32];
        let sig = [0u8; 64];
        let mut v = DagVertex::new(VertexType::Genesis, vec![], creator_did, timestamp, poh_hash, payload_hash, sig);
        v.confirmed = true;
        v
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 3. DAG-Struktur — ATC-04
// ═══════════════════════════════════════════════════════════════════════════

pub struct Dag {
    vertices: Mutex<BTreeMap<[u8; 32], DagVertex>>,
    tips: Mutex<Vec<[u8; 32]>>,  // Unbestätigte Spitzen des DAG
    genesis_id: Mutex<Option<[u8; 32]>>,
}

impl Dag {
    pub fn new() -> Self {
        Dag {
            vertices: Mutex::new(BTreeMap::new()),
            tips: Mutex::new(Vec::new()),
            genesis_id: Mutex::new(None),
        }
    }

    pub fn add_vertex(&self, vertex: DagVertex) -> Result<(), ConsensusError> {
        let mut vertices = self.vertices.lock();
        let mut tips = self.tips.lock();

        // Genesis
        if vertex.vertex_type == VertexType::Genesis {
            *self.genesis_id.lock() = Some(vertex.id);
        } else {
            // Prüfe dass alle Parents existieren
            for parent_id in &vertex.parents {
                if !vertices.contains_key(parent_id) {
                    return Err(ConsensusError::ParentNotFound(*parent_id));
                }
            }
            // Entferne Parents aus Tips (sie sind jetzt "verbunden")
            tips.retain(|t| !vertex.parents.contains(t));
        }

        // Neuer Vertex ist ein neuer Tip
        tips.push(vertex.id);
        vertices.insert(vertex.id, vertex);
        Ok(())
    }

    pub fn get_vertex(&self, id: &[u8; 32]) -> Option<DagVertex> {
        self.vertices.lock().get(id).cloned()
    }

    /// Liefert alle direkten Nachfolger eines Vertex.
    pub fn get_children(&self, parent_id: &[u8; 32]) -> Vec<DagVertex> {
        let vertices = self.vertices.lock();
        vertices.values()
            .filter(|v| v.parents.contains(parent_id))
            .cloned()
            .collect()
    }

    /// Liefert alle Tips (unbestätigte Spitzen).
    pub fn get_tips(&self) -> Vec<[u8; 32]> {
        self.tips.lock().clone()
    }

    /// Anzahl der Vertices im DAG.
    pub fn vertex_count(&self) -> usize {
        self.vertices.lock().len()
    }

    /// Anzahl der Tips.
    pub fn tip_count(&self) -> usize {
        self.tips.lock().len()
    }

    /// Topologische Sortierung (BFS ab Genesis).
    pub fn topological_order(&self) -> Vec<DagVertex> {
        let vertices = self.vertices.lock();
        let genesis = match *self.genesis_id.lock() {
            Some(id) => id,
            None => return Vec::new(),
        };

        let mut result = Vec::new();
        let mut visited = alloc::collections::BTreeSet::new();
        let mut queue = alloc::collections::VecDeque::new();

        visited.insert(genesis);
        queue.push_back(genesis);

        while let Some(id) = queue.pop_front() {
            if let Some(v) = vertices.get(&id) {
                result.push(v.clone());
                // Finde Kinder
                for child in vertices.values() {
                    if child.parents.contains(&id) && !visited.contains(&child.id) {
                        visited.insert(child.id);
                        queue.push_back(child.id);
                    }
                }
            }
        }

        result
    }

    /// Bestätigt einen Vertex (erreicht Supermajority).
    pub fn confirm_vertex(&self, id: &[u8; 32]) {
        if let Some(v) = self.vertices.lock().get_mut(id) {
            v.confirmed = true;
        }
    }

    /// Anzahl der bestätigten Vertices.
    pub fn confirmed_count(&self) -> usize {
        self.vertices.lock().values().filter(|v| v.confirmed).count()
    }

    /// Erzeugt einen Merkle-ähnlichen Hash über alle Tips (für Checkpoints).
    pub fn tips_hash(&self) -> [u8; 32] {
        let tips = self.tips.lock();
        let mut input = Vec::new();
        for t in tips.iter() {
            input.extend_from_slice(t);
        }
        crate::security::simple_hash(&input)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 4. Validator-Registry
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct Validator {
    pub did: String,
    pub stake: u64,
    pub active: bool,
    pub votes_cast: u64,
    pub blocks_proposed: u64,
}

pub struct ValidatorRegistry {
    validators: Mutex<BTreeMap<String, Validator>>,
    total_stake: Mutex<u64>,
}

impl ValidatorRegistry {
    pub fn new() -> Self {
        ValidatorRegistry {
            validators: Mutex::new(BTreeMap::new()),
            total_stake: Mutex::new(0),
        }
    }

    pub fn register(&self, did: String, stake: u64) {
        let mut total = self.total_stake.lock();
        let v = Validator { did: did.clone(), stake, active: true, votes_cast: 0, blocks_proposed: 0 };
        *total += stake;
        self.validators.lock().insert(did, v);
    }

    pub fn deactivate(&self, did: &str) {
        let mut validators = self.validators.lock();
        if let Some(v) = validators.get_mut(did) {
            v.active = false;
        }
    }

    pub fn is_active(&self, did: &str) -> bool {
        self.validators.lock().get(did).map(|v| v.active).unwrap_or(false)
    }

    pub fn get_stake(&self, did: &str) -> u64 {
        self.validators.lock().get(did).map(|v| v.stake).unwrap_or(0)
    }

    pub fn total_stake(&self) -> u64 { *self.total_stake.lock() }
    pub fn validator_count(&self) -> usize { self.validators.lock().len() }
    pub fn active_count(&self) -> usize { self.validators.lock().values().filter(|v| v.active).count() }

    /// Wählt den nächsten Proposer basierend auf Stake (VRF-ähnlich, simplified).
    pub fn select_proposer(&self, poh_hash: &[u8; 32]) -> Option<String> {
        let validators = self.validators.lock();
        let active: Vec<&Validator> = validators.values().filter(|v| v.active).collect();
        if active.is_empty() { return None; }

        // Simplified: Hash mod total_stake → weighted selection
        let total = *self.total_stake.lock();
        let mut input = poh_hash.to_vec();
        let hash_val = u64::from_be_bytes(
            crate::security::simple_hash(&input)[..8].try_into().unwrap()
        );
        let target = hash_val % total;

        let mut acc: u64 = 0;
        for v in &active {
            acc += v.stake;
            if acc > target {
                return Some(v.did.clone());
            }
        }
        active.last().map(|v| v.did.clone())
    }

    pub fn record_vote(&self, did: &str) {
        if let Some(v) = self.validators.lock().get_mut(did) {
            v.votes_cast += 1;
        }
    }

    pub fn record_proposal(&self, did: &str) {
        if let Some(v) = self.validators.lock().get_mut(did) {
            v.blocks_proposed += 1;
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 5. Voting & Finality
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Clone, Debug)]
pub struct Vote {
    pub vertex_id: [u8; 32],
    pub voter_did: String,
    pub timestamp: u64,
    pub approve: bool,
    pub signature: [u8; 64],
}

pub struct VotePool {
    votes: Mutex<BTreeMap<[u8; 32], Vec<Vote>>>,  // vertex_id → votes
    validators: Arc<ValidatorRegistry>,
    finality_threshold: f64,  // z.B. 0.667 für 2/3 Supermajority
}

impl VotePool {
    pub fn new(validators: Arc<ValidatorRegistry>, threshold: f64) -> Self {
        VotePool {
            votes: Mutex::new(BTreeMap::new()),
            validators,
            finality_threshold: threshold,
        }
    }

    pub fn cast_vote(&self, vote: Vote) {
        self.validators.record_vote(&vote.voter_did);
        self.votes.lock().entry(vote.vertex_id).or_insert_with(Vec::new).push(vote);
    }

    /// Prüft ob ein Vertex Finalität erreicht hat.
    pub fn is_final(&self, vertex_id: &[u8; 32]) -> bool {
        let votes = self.votes.lock();
        let vertex_votes = match votes.get(vertex_id) {
            Some(v) => v,
            None => return false,
        };

        let total_stake = self.validators.total_stake();
        if total_stake == 0 { return false; }

        let approving_stake: u64 = vertex_votes.iter()
            .filter(|v| v.approve)
            .map(|v| self.validators.get_stake(&v.voter_did))
            .sum();

        let threshold_stake = (total_stake as f64 * self.finality_threshold) as u64;
        approving_stake >= threshold_stake
    }

    /// Anzahl der Votes für einen Vertex.
    pub fn vote_count(&self, vertex_id: &[u8; 32]) -> usize {
        self.votes.lock().get(vertex_id).map(|v| v.len()).unwrap_or(0)
    }

    /// Anzahl der zustimmenden Votes.
    pub fn approve_count(&self, vertex_id: &[u8; 32]) -> usize {
        self.votes.lock().get(vertex_id)
            .map(|v| v.iter().filter(|vote| vote.approve).count())
            .unwrap_or(0)
    }

    /// Anzahl der ablehnenden Votes.
    pub fn reject_count(&self, vertex_id: &[u8; 32]) -> usize {
        self.votes.lock().get(vertex_id)
            .map(|v| v.iter().filter(|vote| !vote.approve).count())
            .unwrap_or(0)
    }

    /// Liefert alle Vertices, die Finalität erreicht haben.
    pub fn finalized_vertices(&self) -> Vec<[u8; 32]> {
        self.votes.lock().keys()
            .filter(|id| self.is_final(id))
            .copied()
            .collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// 6. Consensus-Engine (Top-Level)
// ═══════════════════════════════════════════════════════════════════════════

pub struct ConsensusEngine {
    pub dag: Arc<Dag>,
    pub poh: Arc<PohSequence>,
    pub validators: Arc<ValidatorRegistry>,
    pub votes: Arc<VotePool>,
    our_did: String,
}

impl ConsensusEngine {
    pub fn new(our_did: String, genesis_hash: [u8; 32]) -> Self {
        let dag = Arc::new(Dag::new());
        let poh = Arc::new(PohSequence::new(genesis_hash));
        let validators = Arc::new(ValidatorRegistry::new());
        let votes = Arc::new(VotePool::new(validators.clone(), 0.667));

        ConsensusEngine { dag, poh, validators, votes, our_did }
    }

    /// Initialisiert den DAG mit Genesis-Vertex.
    pub fn init_genesis(&self, timestamp: u64) -> Result<[u8; 32], ConsensusError> {
        let poh_entry = self.poh.tick(timestamp);
        let genesis = DagVertex::genesis(self.our_did.clone(), timestamp, poh_entry.hash);
        let genesis_id = genesis.id;
        self.dag.add_vertex(genesis)?;
        Ok(genesis_id)
    }

    /// Erzeugt einen neuen Vertex (Transaktion) und fügt ihn zum DAG hinzu.
    pub fn propose_vertex(
        &self,
        payload_hash: [u8; 32],
        timestamp: u64,
        signature: [u8; 64],
    ) -> Result<[u8; 32], ConsensusError> {
        let parents = self.dag.get_tips();
        let poh_entry = self.poh.record(timestamp, &payload_hash);

        let vertex = DagVertex::new(
            VertexType::Transaction,
            parents,
            self.our_did.clone(),
            timestamp,
            poh_entry.hash,
            payload_hash,
            signature,
        );

        let id = vertex.id;
        self.dag.add_vertex(vertex)?;
        self.validators.record_proposal(&self.our_did);
        Ok(id)
    }

    /// Stimmt über einen Vertex ab.
    pub fn vote(&self, vertex_id: [u8; 32], timestamp: u64, approve: bool, signature: [u8; 64]) {
        let vote = Vote {
            vertex_id,
            voter_did: self.our_did.clone(),
            timestamp,
            approve,
            signature,
        };
        self.votes.cast_vote(vote);

        // Wenn final → confirm im DAG
        if self.votes.is_final(&vertex_id) {
            self.dag.confirm_vertex(&vertex_id);
        }
    }

    /// Verarbeitet eine eingehende Stimme eines anderen Validators.
    pub fn handle_vote(&self, vote: Vote) {
        let vote_clone = vote.clone();
        self.votes.cast_vote(vote_clone.clone());
        if self.votes.is_final(&vote_clone.vertex_id) {
            self.dag.confirm_vertex(&vote.vertex_id);
        }
    }

    /// Wählt den nächsten Proposer.
    pub fn next_proposer(&self) -> Option<String> {
        self.validators.select_proposer(&self.poh.current_hash())
    }

    /// Fork-Choice: liefert den schweresten Pfad ab Genesis.
    pub fn fork_choice(&self) -> Vec<[u8; 32]> {
        let mut path = Vec::new();
        let genesis = match *self.dag.genesis_id.lock() {
            Some(id) => id,
            None => return path,
        };

        let mut current = genesis;
        path.push(current);

        loop {
            let children = self.dag.get_children(&current);
            if children.is_empty() { break; }

            // Wähle das Kind mit den meisten Bestätigungs-Votes
            let best = children.iter()
                .max_by_key(|v| self.votes.vote_count(&v.id))
                .map(|v| v.id);

            match best {
                Some(id) => {
                    path.push(id);
                    current = id;
                }
                None => break,
            }
        }
        path
    }

    pub fn our_did(&self) -> &str { &self.our_did }
    pub fn dag(&self) -> &Arc<Dag> { &self.dag }
    pub fn poh(&self) -> &Arc<PohSequence> { &self.poh }
    pub fn validators(&self) -> &Arc<ValidatorRegistry> { &self.validators }
    pub fn votes(&self) -> &Arc<VotePool> { &self.votes }
}

// ═══════════════════════════════════════════════════════════════════════════
// Errors
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusError {
    ParentNotFound([u8; 32]),
    GenesisAlreadyExists,
    VertexAlreadyExists,
    NoActiveValidators,
    InvalidVote,
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> ConsensusEngine {
        ConsensusEngine::new("did:validator1".into(), [0x42; 32])
    }

    // ── Proof of History ─────────────────────────────────────────────────────

    #[test]
    fn test_poh_tick_advances_hash() {
        let poh = PohSequence::new([0x42; 32]);
        let e1 = poh.tick(1000);
        let e2 = poh.tick(2000);
        assert_ne!(e1.hash, e2.hash);
        assert_eq!(e1.tick, 0);
        assert_eq!(e2.tick, 1);
    }

    #[test]
    fn test_poh_record_event() {
        let poh = PohSequence::new([0x42; 32]);
        let event_hash = [0xAA; 32];
        let entry = poh.record(1000, &event_hash);
        assert_eq!(entry.tick, 0);
        assert_ne!(entry.hash, [0x42; 32]); // changed
    }

    #[test]
    fn test_poh_verify_valid() {
        let genesis = [0x42; 32];
        let poh = PohSequence::new(genesis);
        poh.tick(1000);
        poh.tick(2000);
        poh.tick(3000);
        let entries = poh.entries();
        assert!(PohSequence::verify(genesis, &entries));
    }

    #[test]
    fn test_poh_verify_invalid() {
        let genesis = [0x42; 32];
        let mut poh = PohSequence::new(genesis);
        poh.tick(1000);
        poh.tick(2000);
        let mut entries = poh.entries();
        entries[1].hash = [0xFF; 32]; // tamper
        assert!(!PohSequence::verify(genesis, &entries));
    }

    #[test]
    fn test_poh_tick_count() {
        let poh = PohSequence::new([0; 32]);
        assert_eq!(poh.tick_count(), 0);
        poh.tick(1000);
        poh.tick(2000);
        assert_eq!(poh.tick_count(), 2);
    }

    // ── DAG ──────────────────────────────────────────────────────────────────

    #[test]
    fn test_dag_add_genesis() {
        let engine = setup();
        let genesis_id = engine.init_genesis(1000).unwrap();
        assert_eq!(engine.dag().vertex_count(), 1);
        assert_eq!(engine.dag().tip_count(), 1);
        let genesis = engine.dag().get_vertex(&genesis_id).unwrap();
        assert!(genesis.confirmed);
        assert_eq!(genesis.vertex_type, VertexType::Genesis);
    }

    #[test]
    fn test_dag_add_vertex() {
        let engine = setup();
        engine.init_genesis(1000).unwrap();

        let payload = [0x11; 32];
        let sig = [0u8; 64];
        let id = engine.propose_vertex(payload, 2000, sig).unwrap();
        assert_eq!(engine.dag().vertex_count(), 2);
        assert_eq!(engine.dag().tip_count(), 1); // genesis consumed, new tip

        let vertex = engine.dag().get_vertex(&id).unwrap();
        assert_eq!(vertex.vertex_type, VertexType::Transaction);
        assert!(!vertex.confirmed);
        assert_eq!(vertex.parents.len(), 1); // references genesis
    }

    #[test]
    fn test_dag_multiple_parents() {
        let engine = setup();
        let gen = engine.init_genesis(1000).unwrap();

        // Zwei Vertices parallel zu Genesis (manual parents for parallelism)
        let poh1 = engine.poh().record(1100, &[0x11; 32]);
        let v1_vert = DagVertex::new(VertexType::Transaction, vec![gen], engine.our_did().to_string(), 1100, poh1.hash, [0x11; 32], [0; 64]);
        engine.dag().add_vertex(v1_vert.clone()).unwrap();
        let v1 = v1_vert.id;
        let poh2 = engine.poh().record(1200, &[0x22; 32]);
        let v2_vert = DagVertex::new(VertexType::Transaction, vec![gen], engine.our_did().to_string(), 1200, poh2.hash, [0x22; 32], [0; 64]);
        engine.dag().add_vertex(v2_vert.clone()).unwrap();
        let v2 = v2_vert.id;
        assert_eq!(engine.dag().tip_count(), 2);

        // Dritter Vertex referenziert beide Tips
        let parents = engine.dag().get_tips();
        let poh_entry = engine.poh().record(1300, &[0x33; 32]);
        let v3 = DagVertex::new(
            VertexType::Transaction, parents,
            "did:v1".into(), 1300, poh_entry.hash, [0x33; 32], [0; 64],
        );
        let v3_id = v3.id;
        engine.dag().add_vertex(v3).unwrap();
        assert_eq!(engine.dag().tip_count(), 1); // both consumed
        assert_eq!(engine.dag().get_vertex(&v3_id).unwrap().parents.len(), 2);
    }

    #[test]
    fn test_dag_parent_not_found() {
        let dag = Dag::new();
        let v = DagVertex::new(
            VertexType::Transaction, vec![[0xFF; 32]],
            "did:x".into(), 1000, [0; 32], [0; 32], [0; 64],
        );
        assert_eq!(dag.add_vertex(v), Err(ConsensusError::ParentNotFound([0xFF; 32])));
    }

    #[test]
    fn test_dag_topological_order() {
        let engine = setup();
        engine.init_genesis(1000).unwrap();
        engine.propose_vertex([0x11; 32], 1100, [0; 64]).unwrap();
        engine.propose_vertex([0x22; 32], 1200, [0; 64]).unwrap();

        let order = engine.dag().topological_order();
        assert_eq!(order.len(), 3);
        assert_eq!(order[0].vertex_type, VertexType::Genesis); // genesis first
    }

    #[test]
    fn test_dag_get_children() {
        let engine = setup();
        let gen = engine.init_genesis(1000).unwrap();
        engine.propose_vertex([0x11; 32], 1100, [0; 64]).unwrap();
        engine.propose_vertex([0x22; 32], 1200, [0; 64]).unwrap();

        let children = engine.dag().get_children(&gen);
        assert_eq!(children.len(), 1);
    }

    #[test]
    fn test_dag_tips_hash() {
        let engine = setup();
        engine.init_genesis(1000).unwrap();
        engine.propose_vertex([0x11; 32], 1100, [0; 64]).unwrap();
        let h1 = engine.dag().tips_hash();
        engine.propose_vertex([0x22; 32], 1200, [0; 64]).unwrap();
        let h2 = engine.dag().tips_hash();
        assert_ne!(h1, h2); // different tips → different hash
    }

    // ── Validator-Registry ───────────────────────────────────────────────────

    #[test]
    fn test_validator_register() {
        let reg = ValidatorRegistry::new();
        reg.register("did:v1".into(), 1000);
        assert_eq!(reg.validator_count(), 1);
        assert_eq!(reg.total_stake(), 1000);
        assert!(reg.is_active("did:v1"));
        assert_eq!(reg.get_stake("did:v1"), 1000);
    }

    #[test]
    fn test_validator_deactivate() {
        let reg = ValidatorRegistry::new();
        reg.register("did:v1".into(), 1000);
        reg.deactivate("did:v1");
        assert!(!reg.is_active("did:v1"));
        assert_eq!(reg.active_count(), 0);
    }

    #[test]
    fn test_validator_select_proposer() {
        let reg = ValidatorRegistry::new();
        reg.register("did:v1".into(), 100);
        reg.register("did:v2".into(), 200);
        reg.register("did:v3".into(), 300);

        let proposer = reg.select_proposer(&[0x42; 32]);
        assert!(proposer.is_some());
        let did = proposer.unwrap();
        assert!(did.starts_with("did:v"));
    }

    #[test]
    fn test_validator_select_no_active() {
        let reg = ValidatorRegistry::new();
        assert!(reg.select_proposer(&[0; 32]).is_none());
    }

    #[test]
    fn test_validator_stats() {
        let reg = ValidatorRegistry::new();
        reg.register("did:v1".into(), 500);
        reg.record_vote("did:v1");
        reg.record_vote("did:v1");
        reg.record_proposal("did:v1");
        let validators = reg.validators.lock();
        let v = validators.get("did:v1").unwrap();
        assert_eq!(v.votes_cast, 2);
        assert_eq!(v.blocks_proposed, 1);
    }

    // ── Voting & Finality ─────────────────────────────────────────────────────

    #[test]
    fn test_vote_cast() {
        let validators = Arc::new(ValidatorRegistry::new());
        validators.register("did:v1".into(), 100);
        validators.register("did:v2".into(), 100);
        validators.register("did:v3".into(), 100);
        let pool = VotePool::new(validators, 0.667);

        let vertex_id = [0x42; 32];
        pool.cast_vote(Vote {
            vertex_id, voter_did: "did:v1".into(), timestamp: 1000, approve: true, signature: [0; 64],
        });
        assert_eq!(pool.vote_count(&vertex_id), 1);
        assert_eq!(pool.approve_count(&vertex_id), 1);
        assert_eq!(pool.reject_count(&vertex_id), 0);
    }

    #[test]
    fn test_finality_two_thirds() {
        let validators = Arc::new(ValidatorRegistry::new());
        validators.register("did:v1".into(), 100);
        validators.register("did:v2".into(), 100);
        validators.register("did:v3".into(), 100);
        let pool = VotePool::new(validators.clone(), 0.667);

        let vertex_id = [0x42; 32];

        // 1/3 approve → not final
        pool.cast_vote(Vote { vertex_id, voter_did: "did:v1".into(), timestamp: 1000, approve: true, signature: [0; 64] });
        assert!(!pool.is_final(&vertex_id));

        // 2/3 approve → final (>= 66.7%)
        pool.cast_vote(Vote { vertex_id, voter_did: "did:v2".into(), timestamp: 2000, approve: true, signature: [0; 64] });
        assert!(pool.is_final(&vertex_id));
    }

    #[test]
    fn test_finality_rejected_votes() {
        let validators = Arc::new(ValidatorRegistry::new());
        validators.register("did:v1".into(), 100);
        validators.register("did:v2".into(), 100);
        validators.register("did:v3".into(), 100);
        let pool = VotePool::new(validators, 0.667);

        let vertex_id = [0x42; 32];
        pool.cast_vote(Vote { vertex_id, voter_did: "did:v1".into(), timestamp: 1000, approve: true, signature: [0; 64] });
        pool.cast_vote(Vote { vertex_id, voter_did: "did:v2".into(), timestamp: 2000, approve: false, signature: [0; 64] });

        assert_eq!(pool.approve_count(&vertex_id), 1);
        assert_eq!(pool.reject_count(&vertex_id), 1);
        assert!(!pool.is_final(&vertex_id));
    }

    #[test]
    fn test_vote_no_votes() {
        let validators = Arc::new(ValidatorRegistry::new());
        let pool = VotePool::new(validators, 0.667);
        assert!(!pool.is_final(&[0x42; 32]));
        assert_eq!(pool.vote_count(&[0x42; 32]), 0);
    }

    // ── Consensus-Engine Integration ──────────────────────────────────────────

    #[test]
    fn test_engine_full_workflow() {
        let engine = setup();

        // 1. Init Genesis
        let gen = engine.init_genesis(1000).unwrap();

        // 2. Register validators
        engine.validators().register("did:validator1".into(), 100);
        engine.validators().register("did:validator2".into(), 100);
        engine.validators().register("did:validator3".into(), 100);

        // 3. Propose a vertex
        let v_id = engine.propose_vertex([0x11; 32], 2000, [0; 64]).unwrap();
        assert!(!engine.dag().get_vertex(&v_id).unwrap().confirmed);

        // 4. Vote (2/3 majority)
        engine.handle_vote(Vote { vertex_id: v_id, voter_did: "did:validator2".into(), timestamp: 2100, approve: true, signature: [0; 64] });
        engine.handle_vote(Vote { vertex_id: v_id, voter_did: "did:validator3".into(), timestamp: 2200, approve: true, signature: [0; 64] });

        // 5. Should be confirmed
        assert!(engine.dag().get_vertex(&v_id).unwrap().confirmed);
        assert_eq!(engine.dag().confirmed_count(), 2); // genesis + v1
    }

    #[test]
    fn test_engine_fork_choice() {
        let engine = setup();
        engine.init_genesis(1000).unwrap();

        // Two parallel branches
        let v1 = engine.propose_vertex([0x11; 32], 1100, [0; 64]).unwrap();
        let v2 = engine.propose_vertex([0x22; 32], 1200, [0; 64]).unwrap();

        // Vote on v1 more
        engine.validators().register("did:voter".into(), 100);
        engine.handle_vote(Vote { vertex_id: v1, voter_did: "did:voter".into(), timestamp: 1500, approve: true, signature: [0; 64] });

        let path = engine.fork_choice();
        assert!(!path.is_empty());
        assert_eq!(path[0], engine.dag().genesis_id.lock().unwrap());
    }

    #[test]
    fn test_engine_next_proposer() {
        let engine = setup();
        engine.validators().register("did:v1".into(), 100);
        engine.validators().register("did:v2".into(), 200);

        let proposer = engine.next_proposer();
        assert!(proposer.is_some());
    }

    #[test]
    fn test_engine_next_proposer_no_validators() {
        let engine = setup();
        assert!(engine.next_proposer().is_none());
    }

    #[test]
    fn test_engine_propose_updates_tips() {
        let engine = setup();
        engine.init_genesis(1000).unwrap();

        let v1 = engine.propose_vertex([0x11; 32], 1100, [0; 64]).unwrap();
        assert_eq!(engine.dag().tip_count(), 1);

        let v2 = engine.propose_vertex([0x22; 32], 1200, [0; 64]).unwrap();
        assert_eq!(engine.dag().tip_count(), 1); // v1 consumed

        // Both should exist
        assert!(engine.dag().get_vertex(&v1).is_some());
        assert!(engine.dag().get_vertex(&v2).is_some());
    }

    #[test]
    fn test_chain_id_constant() {
        assert_eq!(crate::p2p::CHAIN_ID, 658467);
    }
}
