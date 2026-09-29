// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Canonical A-TownChain L1 transaction construction and signing.
//!
//! This module defines the protocol-level transaction preimage. Cryptographic
//! primitives remain isolated in keys.rs.

use crate::keys::WalletKey;
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
    /// Economic value: canonical u128 representation.
    pub amount: u128,
    /// Economic fee price: canonical u128 representation.
    pub gas_price: u128,
    /// Resource limit, not an economic quantity.
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
    SigningFailure,
}

impl Transaction {
    pub fn signing_bytes(&self) -> Result<Vec<u8>, TxError> {
        if self.chain_id != NUMERIC_CHAIN_ID {
            return Err(TxError::InvalidChainId);
        }
        if self.sender_did.is_empty() {
            return Err(TxError::EmptySender);
        }

        let mut b = Vec::with_capacity(160 + self.payload.len());
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
        let mut b = Vec::with_capacity(160 + self.payload.len());
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
        key.sign(&self.signing_bytes()?)
            .map_err(|_| TxError::SigningFailure)
    }

    pub fn verify(&self, public_key: &[u8; 33], signature: &[u8; 64]) -> Result<(), TxError> {
        WalletKey::verify(public_key, &self.signing_bytes()?, signature)
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
        let key = WalletKey::from_private_key_bytes(&[7u8; 32]).unwrap();
        let tx = tx();
        let signature = tx.sign(&key).unwrap();
        assert!(tx.verify(&key.public_key(), &signature).is_ok());
    }

    #[test]
    fn wrong_chain_id_is_rejected_before_signing() {
        let mut tx = tx();
        tx.chain_id = 1;
        let key = WalletKey::from_private_key_bytes(&[7u8; 32]).unwrap();
        assert!(matches!(tx.sign(&key), Err(TxError::InvalidChainId)));
    }

    #[test]
    fn transaction_mutation_invalidates_signature() {
        let key = WalletKey::from_private_key_bytes(&[7u8; 32]).unwrap();
        let tx = tx();
        let signature = tx.sign(&key).unwrap();
        let mut altered = tx.clone();
        altered.amount += 1;
        assert!(altered.verify(&key.public_key(), &signature).is_err());
    }

    #[test]
    fn canonical_u128_vector_is_stable() {
        let key = WalletKey::from_private_key_bytes(&[7u8; 32]).unwrap();
        let tx = tx();
        let bytes = tx.signing_bytes().unwrap();

        assert_eq!(
            Sha256::digest(&bytes).as_slice(),
            &[
                0x86, 0x08, 0xd1, 0x53, 0x0c, 0xc0, 0x8d, 0xd0,
                0x22, 0x07, 0x93, 0xed, 0xc6, 0xb2, 0x40, 0x71,
                0x87, 0x8b, 0x36, 0xfc, 0xa0, 0x29, 0x9b, 0x25,
                0x34, 0xce, 0xf7, 0x6e, 0x79, 0xd3, 0x40, 0xbe
            ]
        );

        assert_eq!(
            key.public_key(),
            [
                0x02, 0x5c, 0xbd, 0xf0, 0x64, 0x6e, 0x5d, 0xb4,
                0xea, 0xa3, 0x98, 0xf3, 0x65, 0xf2, 0xea, 0x7a,
                0x0e, 0x3d, 0x41, 0x9b, 0x7e, 0x03, 0x30, 0xe3,
                0x9c, 0xe9, 0x2b, 0xdd, 0xed, 0xca, 0xc4, 0xf9,
                0xbc
            ]
        );

        assert_eq!(
            tx.sign(&key).unwrap(),
            [
                0x1c, 0x06, 0x61, 0xf2, 0xec, 0xc4, 0xcc, 0xfc,
                0xa7, 0x86, 0xe5, 0xa3, 0x7a, 0x38, 0x61, 0x91,
                0xa6, 0x2a, 0x30, 0x1c, 0xd2, 0x44, 0x9c, 0xe8,
                0x00, 0x89, 0xe3, 0x55, 0x22, 0x20, 0xcc, 0xfc,
                0x3a, 0x71, 0xcb, 0xb5, 0x90, 0x3b, 0xa6, 0xf5,
                0xdf, 0x68, 0x69, 0x0b, 0xc7, 0x8e, 0xd3, 0x89,
                0xec, 0x54, 0x8f, 0x4e, 0x9c, 0xee, 0xf3, 0x44,
                0xe3, 0x6a, 0xbc, 0x7b, 0x3b, 0x64, 0x3d, 0x78
            ]
        );
    }

    #[test]
    fn u128_max_values_roundtrip() {
        let mut tx = tx();
        tx.amount = u128::MAX;
        tx.gas_price = u128::MAX;

        let bytes = tx.signing_bytes().unwrap();
        assert_eq!(
            bytes.windows(16).filter(|w| *w == [0xff; 16]).count(),
            2
        );
    }
}
