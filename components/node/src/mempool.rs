// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Mempool mit Signatur-, DID-Bindungs- und Nonce-Pruefung.
//!
//! SEC-P0-001: Die kanonische secp256k1-Transaktion wird hier erstmals in einem
//! Node-Pfad tatsaechlich verifiziert (`verify_bound`: Signatur + DID-Bindung).
//! SEC-P0-008: Nonce-Ordnung je Sender verhindert Wiedereinreichung und Replay;
//! aufgenommene IDs bleiben auch nach der Block-Entnahme gesperrt.

use std::collections::{HashMap, HashSet};

use atc_wallet::tx::{Transaction, TxError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MempoolError {
    InvalidChainId,
    InvalidSignature,
    SenderNotBound,
    NonceGap { expected: u64 },
    NonceTooLow { expected: u64 },
    DuplicateTransaction,
    Full,
    /// DID ist der Registry unbekannt (strikter Modus).
    UnknownDid,
    /// Mitgelieferter Pubkey ist nicht der autoritative Schluessel der DID.
    KeyMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxEnvelope {
    pub tx: Transaction,
    pub public_key: [u8; 33],
    pub signature: [u8; 64],
}

pub struct Mempool {
    capacity: usize,
    policy: IdentityPolicy,
    expected_nonce: HashMap<String, u64>,
    entries: Vec<TxEnvelope>,
    seen_ids: HashSet<[u8; 32]>,
}

impl Mempool {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            policy: IdentityPolicy::permissive(),
            expected_nonce: HashMap::new(),
            entries: Vec::new(),
            seen_ids: HashSet::new(),
        }
    }

    /// Mempool mit Identitaets-Policy (z. B. strikte DID-Registry).
    pub fn with_policy(capacity: usize, policy: IdentityPolicy) -> Self {
        let mut pool = Self::new(capacity);
        pool.policy = policy;
        pool
    }

    pub fn policy(&self) -> &IdentityPolicy {
        &self.policy
    }

    pub fn policy_mut(&mut self) -> &mut IdentityPolicy {
        &mut self.policy
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn contains(&self, id: &[u8; 32]) -> bool {
        self.seen_ids.contains(id)
    }

    pub fn expected_nonce(&self, sender_did: &str) -> Option<u64> {
        self.expected_nonce.get(sender_did).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TxEnvelope> {
        self.entries.iter()
    }

    /// Nimmt eine Transaktion nur auf, wenn Signatur, DID-Bindung, Chain-ID,
    /// Nonce-Ordnung und Kapazitaet stimmen.
    pub fn admit(&mut self, env: TxEnvelope) -> Result<[u8; 32], MempoolError> {
        let id = env.tx.id().map_err(|_| MempoolError::InvalidChainId)?;
        if self.seen_ids.contains(&id) {
            return Err(MempoolError::DuplicateTransaction);
        }
        self.policy.verify(&env)?;
        if self.entries.len() >= self.capacity {
            return Err(MempoolError::Full);
        }
        // Erste Transaktion eines Senders setzt den Nonce-Baseline.
        let expected = self
            .expected_nonce
            .get(&env.tx.sender_did)
            .copied()
            .unwrap_or(env.tx.nonce);
        if env.tx.nonce < expected {
            return Err(MempoolError::NonceTooLow { expected });
        }
        if env.tx.nonce > expected {
            return Err(MempoolError::NonceGap { expected });
        }
        self.expected_nonce
            .insert(env.tx.sender_did.clone(), expected.saturating_add(1));
        self.seen_ids.insert(id);
        self.entries.push(env);
        Ok(id)
    }

    /// Block-Assembly: entnimmt alle Transaktionen in Aufnahmereihenfolge.
    /// IDs und Nonce-Zaehler bleiben erhalten, Replay wird weiter abgelehnt.
    pub fn take_all(&mut self) -> Vec<TxEnvelope> {
        std::mem::take(&mut self.entries)
    }

    pub fn remove(&mut self, id: &[u8; 32]) -> bool {
        if let Some(pos) = self.entries.iter().position(|e| e.tx.id().ok() == Some(*id)) {
            self.entries.remove(pos);
            return true;
        }
        false
    }

    /// Nach Block-Import: Nonce-Zaehler eines Senders auf den gueltigen Stand heben.
    pub fn advance_nonce(&mut self, sender_did: &str, next_nonce: u64) {
        let cur = self.expected_nonce.get(sender_did).copied().unwrap_or(0);
        if next_nonce > cur {
            self.expected_nonce.insert(sender_did.to_string(), next_nonce);
        }
    }
}


/// Kettenweites Ledger fuer Transaktions-Nonces und Replay-Schutz.
///
/// SEC-P0-001: Blockproduktion und Block-Import pruefen jede Transaktion
/// (Signatur, DID-Bindung, Chain-ID) und die Nonce-Ordnung, bevor ein Block
/// angehaengt wird. Die Pruefung veraendert keinen Zustand; erst `commit_block`
/// uebernimmt die Transaktionen nach erfolgreichem Anhang.
#[derive(Debug, Clone, Default)]
pub struct TxLedger {
    policy: IdentityPolicy,
    expected_nonce: HashMap<String, u64>,
    seen_ids: HashSet<[u8; 32]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerError {
    EmptyBlock,
    InvalidChainId,
    InvalidSignature,
    SenderNotBound,
    DuplicateTransaction,
    NonceGap { expected: u64 },
    NonceTooLow { expected: u64 },
    UnknownDid,
    KeyMismatch,
}

impl TxLedger {
    pub fn new() -> Self {
        Self::default()
    }

    /// Ledger mit Identitaets-Policy (strikte DID-Registry).
    pub fn with_policy(policy: IdentityPolicy) -> Self {
        Self { policy, ..Self::default() }
    }

    pub fn policy_mut(&mut self) -> &mut IdentityPolicy {
        &mut self.policy
    }

    pub fn expected_nonce(&self, sender: &str) -> Option<u64> {
        self.expected_nonce.get(sender).copied()
    }

    pub fn contains(&self, id: &[u8; 32]) -> bool {
        self.seen_ids.contains(id)
    }

    /// Prueft eine vollstaendige Tx-Folge gegen den Ledger-Zustand.
    /// Rein lesend: bei Fehler bleibt der Ledger unveraendert.
    pub fn check_block(&self, txs: &[TxEnvelope]) -> Result<Vec<[u8; 32]>, LedgerError> {
        if txs.is_empty() {
            return Err(LedgerError::EmptyBlock);
        }
        let mut nonces = self.expected_nonce.clone();
        let mut ids = self.seen_ids.clone();
        let mut out = Vec::with_capacity(txs.len());

        for env in txs {
            let id = env.tx.id().map_err(|_| LedgerError::InvalidChainId)?;
            if ids.contains(&id) {
                return Err(LedgerError::DuplicateTransaction);
            }
            self.policy.verify(env).map_err(|e| match e {
                MempoolError::SenderNotBound => LedgerError::SenderNotBound,
                MempoolError::InvalidChainId => LedgerError::InvalidChainId,
                MempoolError::UnknownDid => LedgerError::UnknownDid,
                MempoolError::KeyMismatch => LedgerError::KeyMismatch,
                _ => LedgerError::InvalidSignature,
            })?;
            let expected = *nonces.get(&env.tx.sender_did).unwrap_or(&env.tx.nonce);
            if env.tx.nonce < expected {
                return Err(LedgerError::NonceTooLow { expected });
            }
            if env.tx.nonce > expected {
                return Err(LedgerError::NonceGap { expected });
            }
            nonces.insert(env.tx.sender_did.clone(), expected.saturating_add(1));
            ids.insert(id);
            out.push(id);
        }
        Ok(out)
    }

    /// Uebernimmt die Transaktionen eines angehaengten Blocks.
    pub fn commit_block(&mut self, txs: &[TxEnvelope]) -> Result<Vec<[u8; 32]>, LedgerError> {
        let ids = self.check_block(txs)?;
        for env in txs {
            let expected = *self.expected_nonce.get(&env.tx.sender_did).unwrap_or(&env.tx.nonce);
            self.expected_nonce
                .insert(env.tx.sender_did.clone(), expected.saturating_add(1));
        }
        for id in &ids {
            self.seen_ids.insert(*id);
        }
        Ok(ids)
    }

    /// Nach externem Block-Import: Nonce-Zaehler eines Senders nachziehen.
    pub fn advance_nonce(&mut self, sender: &str, next_nonce: u64) {
        self.expected_nonce.insert(sender.to_string(), next_nonce);
    }
}

/// Identitaets-Policy: entscheidet, ob der Verifier dem mitgelieferten Pubkey
/// glaubt (self-certifying) oder ihn gegen die DID-Registry aufloest.
#[derive(Debug, Clone, Default)]
pub struct IdentityPolicy {
    registry: crate::did::DidRegistry,
    require_registered: bool,
}

impl IdentityPolicy {
    /// Ohne Registry: DID == abgeleitete Adresse des Pubkeys (bisheriges Verhalten).
    pub fn permissive() -> Self {
        Self::default()
    }

    /// Mit Registry: nur registrierte DIDs, Schluessel muss der aktuelle sein.
    pub fn strict(registry: crate::did::DidRegistry) -> Self {
        Self { registry, require_registered: true }
    }

    pub fn registry(&self) -> &crate::did::DidRegistry {
        &self.registry
    }

    pub fn registry_mut(&mut self) -> &mut crate::did::DidRegistry {
        &mut self.registry
    }

    pub fn is_strict(&self) -> bool {
        self.require_registered
    }

    /// Signaturpruefung nach Policy. Rotierte DIDs sind zulaessig, solange der
    /// eingereichte Schluessel der aktuelle Registry-Schluessel ist.
    pub fn verify(&self, env: &TxEnvelope) -> Result<(), MempoolError> {
        if !self.require_registered {
            return verify_envelope(env);
        }
        let current = self
            .registry
            .current_key(&env.tx.sender_did)
            .map_err(|_| MempoolError::UnknownDid)?;
        if current != env.public_key {
            return Err(MempoolError::KeyMismatch);
        }
        env.tx
            .verify(&env.public_key, &env.signature)
            .map_err(|_| MempoolError::InvalidSignature)
    }
}

/// Gemeinsame Transaktions-Pruefung fuer Mempool und Ledger:
/// Signatur und DID-Bindung ueber die kanonische secp256k1-Transaktion.
pub fn verify_envelope(env: &TxEnvelope) -> Result<(), MempoolError> {
    env.tx
        .verify_bound(&env.public_key, &env.signature)
        .map_err(|e| match e {
            atc_wallet::tx::TxError::SenderNotBound => MempoolError::SenderNotBound,
            atc_wallet::tx::TxError::InvalidChainId => MempoolError::InvalidChainId,
            _ => MempoolError::InvalidSignature,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use atc_wallet::keys::WalletKey;
    use atc_wallet::tx::{TxType, NUMERIC_CHAIN_ID};

    fn tx_for(key: &WalletKey, nonce: u64) -> Transaction {
        Transaction {
            chain_id: NUMERIC_CHAIN_ID,
            tx_type: TxType::Transfer,
            sender_did: key.address(),
            recipient_did: Some("ATC:1111111111111111111111111111111111111111".into()),
            amount: 100,
            gas_price: 1,
            gas_limit: 1000,
            nonce,
            timestamp: 1_700_000_000,
            payload: Vec::new(),
            poh_hash: [9u8; 32],
        }
    }

    fn envelope(key: &WalletKey, tx: Transaction) -> TxEnvelope {
        let signature = tx.sign(key).unwrap();
        TxEnvelope { tx, public_key: key.public_key(), signature }
    }

    #[test]
    fn gtueltige_tx_wird_aufgenommen() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let mut pool = Mempool::new(10);
        let tx = tx_for(&key, 0);
        assert!(pool.admit(envelope(&key, tx)).is_ok());
        assert_eq!(pool.len(), 1);
        assert_eq!(pool.expected_nonce(&key.address()), Some(1));
    }

    #[test]
    fn fremde_did_wird_abgelehnt() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let mut pool = Mempool::new(10);
        let mut tx = tx_for(&key, 0);
        tx.sender_did = "ATC:2222222222222222222222222222222222222222".into();
        assert_eq!(pool.admit(envelope(&key, tx)), Err(MempoolError::SenderNotBound));
    }

    #[test]
    fn falsche_signatur_wird_abgelehnt() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let other = WalletKey::from_seed([8u8; 32]).unwrap();
        let mut pool = Mempool::new(10);
        let tx = tx_for(&key, 0);
        let mut env = envelope(&key, tx);
        env.signature = other.sign_digest([1u8; 32]).serialize_compact();
        assert_eq!(pool.admit(env), Err(MempoolError::InvalidSignature));
    }

    #[test]
    fn nonce_luecke_wird_abgelehnt() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let mut pool = Mempool::new(10);
        assert!(pool.admit(envelope(&key, tx_for(&key, 0))).is_ok());
        assert_eq!(
            pool.admit(envelope(&key, tx_for(&key, 2))),
            Err(MempoolError::NonceGap { expected: 1 })
        );
    }

    #[test]
    fn doppelte_einreichung_wird_abgelehnt() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let mut pool = Mempool::new(10);
        let tx = tx_for(&key, 0);
        assert!(pool.admit(envelope(&key, tx.clone())).is_ok());
        assert_eq!(pool.admit(envelope(&key, tx)), Err(MempoolError::DuplicateTransaction));
    }

    #[test]
    fn replay_nach_block_entnahme_wird_abgelehnt() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let mut pool = Mempool::new(10);
        let tx = tx_for(&key, 0);
        assert!(pool.admit(envelope(&key, tx.clone())).is_ok());
        let block = pool.take_all();
        assert_eq!(block.len(), 1);
        assert!(pool.is_empty());
        // Wiedereinreichung derselben Tx nach Block-Assembly
        assert_eq!(pool.admit(envelope(&key, tx)), Err(MempoolError::DuplicateTransaction));
    }

    #[test]
    fn nonce_zu_niedrig_nach_fortschritt() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let mut pool = Mempool::new(10);
        pool.advance_nonce(&key.address(), 5);
        assert_eq!(
            pool.admit(envelope(&key, tx_for(&key, 4))),
            Err(MempoolError::NonceTooLow { expected: 5 })
        );
        assert!(pool.admit(envelope(&key, tx_for(&key, 5))).is_ok());
    }

    #[test]
    fn kapazitaet_wird_durchgesetzt() {
        let mut pool = Mempool::new(1);
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        assert!(pool.admit(envelope(&key, tx_for(&key, 0))).is_ok());
        let key2 = WalletKey::from_seed([9u8; 32]).unwrap();
        assert_eq!(pool.admit(envelope(&key2, tx_for(&key2, 0))), Err(MempoolError::Full));
    }

    #[test]
    fn falsche_chain_id_wird_abgelehnt() {
        let key = WalletKey::from_seed([7u8; 32]).unwrap();
        let mut pool = Mempool::new(10);
        let mut tx = tx_for(&key, 0);
        tx.chain_id = 1;
        // Signieren lehnt falsche Chain-IDs bereits ab (fail-closed)
        assert!(tx.sign(&key).is_err());
        // Einreichung wird ebenfalls abgelehnt, noch vor der Signaturpruefung
        let env = TxEnvelope { tx, public_key: key.public_key(), signature: [0u8; 64] };
        assert_eq!(pool.admit(env), Err(MempoolError::InvalidChainId));
    }
}
