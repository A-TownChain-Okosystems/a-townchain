// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Canonical A-TownChain L1 transaction construction and signing.
//!
//! This module uses exactly the ATC-TX-DOMAIN-V2 byte layout accepted by the
//! Rust L1 kernel. No legacy TransactionDomain or ATC-TX-DOMAIN encoding remains.

use crate::keys::WalletKey;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

pub const NUMERIC_CHAIN_ID: u64 = 658467;
pub const TX_DOMAIN_V2: &[u8] = b"ATC-TX-DOMAIN-V2";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TxType {
    Transfer = 0,
    Stake = 1,
    Unstake = 2,
    Contract = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub chain_id: u64,
    pub tx_type: TxType,
    pub sender_did: String,
    pub recipient_did: Option<String>,
    pub amount: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub nonce: u64,
    pub timestamp: u64,
    pub payload: Vec<u8>,
    pub poh_hash: [u8; 32],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxError {
    InvalidChainId,
    EmptySender,
    InvalidSignature,
}

impl Transaction {
    pub fn signing_bytes(&self) -> Result<Vec<u8>, TxError> {
        if self.chain_id != NUMERIC_CHAIN_ID {
            return Err(TxError::InvalidChainId);
        }
        if self.sender_did.is_empty() {
            return Err(TxError::EmptySender);
        }

        let mut b = Vec::with_capacity(128 + self.payload.len());
        b.extend_from_slice(TX_DOMAIN_V2);
        b.extend_from_slice(&self.chain_id.to_be_bytes());
        b.push(self.tx_type as u8);
        put_bytes(&mut b, self.sender_did.as_bytes());

        match &self.recipient_did {
            Some(value) => {
                b.push(1);
                put_bytes(&mut b, value.as_bytes());
            }
            None => b.push(0),
        }

        b.extend_from_slice(&self.amount.to_be_bytes());
        b.extend_from_slice(&self.gas_price.to_be_bytes());
        b.extend_from_slice(&self.gas_limit.to_be_bytes());
        b.extend_from_slice(&self.nonce.to_be_bytes());
        b.extend_from_slice(&self.timestamp.to_be_bytes());
        put_bytes(&mut b, &self.payload);
        b.extend_from_slice(&self.poh_hash);
        Ok(b)
    }

    pub fn id(&self, _signature: &[u8; 64]) -> Result<[u8; 32], TxError> {
        let mut b = Vec::with_capacity(128 + self.payload.len());
        b.extend_from_slice(b"ATC-TX-ID-V2");
        b.extend_from_slice(&self.chain_id.to_be_bytes());
        b.push(self.tx_type as u8);
        put_bytes(&mut b, self.sender_did.as_bytes());
        match &self.recipient_did {
            Some(value) => {
                b.push(1);
                put_bytes(&mut b, value.as_bytes());
            }
            None => b.push(0),
        }
        b.extend_from_slice(&self.amount.to_be_bytes());
        b.extend_from_slice(&self.gas_price.to_be_bytes());
        b.extend_from_slice(&self.gas_limit.to_be_bytes());
        b.extend_from_slice(&self.nonce.to_be_bytes());
        b.extend_from_slice(&self.timestamp.to_be_bytes());
        put_bytes(&mut b, &self.payload);
        b.extend_from_slice(&self.poh_hash);
        Ok(Sha256::digest(b).into())
    }

    pub fn sign(&self, key: &WalletKey) -> Result<[u8; 64], TxError> {
        Ok(key.sign(&self.signing_bytes()?).to_bytes())
    }

    pub fn verify(&self, public_key: &[u8; 32], signature: &[u8; 64]) -> Result<(), TxError> {
        let key = VerifyingKey::from_bytes(public_key).map_err(|_| TxError::InvalidSignature)?;
        key.verify(&self.signing_bytes()?, &Signature::from_bytes(signature))
            .map_err(|_| TxError::InvalidSignature)
    }
}

fn put_bytes(out: &mut Vec<u8>, value: &[u8]) {
    out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    out.extend_from_slice(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tx() -> Transaction {
        Transaction {
            chain_id: NUMERIC_CHAIN_ID,
            tx_type: TxType::Transfer,
            sender_did: "ATC-sender".into(),
            recipient_did: Some("ATC-recipient".into()),
            amount: 100,
            gas_price: 1,
            gas_limit: 1000,
            nonce: 7,
            timestamp: 1_700_000_000,
            payload: b"hello".to_vec(),
            poh_hash: [9u8; 32],
        }
    }

    #[test]
    fn l1_signature_roundtrip() {
        let key = WalletKey::from_seed([7u8; 32]);
        let tx = tx();
        let signature = tx.sign(&key).unwrap();
        assert!(tx.verify(&key.public_key(), &signature).is_ok());
    }

    #[test]
    fn wrong_chain_id_is_rejected_before_signing() {
        let mut tx = tx();
        tx.chain_id = 1;
        let key = WalletKey::from_seed([7u8; 32]);
        assert!(matches!(tx.sign(&key), Err(TxError::InvalidChainId)));
    }

    #[test]
    fn transaction_mutation_invalidates_signature() {
        let key = WalletKey::from_seed([7u8; 32]);
        let tx = tx();
        let signature = tx.sign(&key).unwrap();
        let mut altered = tx.clone();
        altered.amount += 1;
        assert!(altered.verify(&key.public_key(), &signature).is_err());
    }
}
