// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! Canonical A-TownChain L1 transaction authorization.
//! Protocol definition is owned by the standards/L2 contract; this is its wallet implementation.

use k256::ecdsa::{
    signature::hazmat::{PrehashSigner, PrehashVerifier},
    Signature, SigningKey, VerifyingKey,
};
use sha2::{Digest, Sha256};

pub const CHAIN_ID: u64 = 658467;
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
    pub amount: u128,
    pub gas_price: u128,
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

fn put_bytes(out: &mut Vec<u8>, value: &[u8]) {
    assert!(value.len() <= u32::MAX as usize);
    out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    out.extend_from_slice(value);
}

impl Transaction {
    pub fn signing_bytes(&self) -> Result<Vec<u8>, TxError> {
        if self.chain_id != CHAIN_ID {
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

    pub fn digest(&self) -> Result<[u8; 32], TxError> {
        Ok(Sha256::digest(self.signing_bytes()?).into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn tx() -> Transaction {
        Transaction {
            nonce: 7,
            sender: vec![2; 33],
            recipient: vec![3; 33],
            amount: u128::MAX,
            gas_price: u128::MAX - 1,
            gas_limit: 1000,
            nonce: 7,
            timestamp: 1_700_000_000,
            payload: b"ATC-V2".to_vec(),
            poh_hash: [9u8; 32],
        }
    }

    #[test]
    fn u128_is_fixed_16_byte_big_endian() {
        let bytes = signing_preimage(&tx()).unwrap();
        assert!(bytes.windows(16).any(|w| w == u128::MAX.to_be_bytes()));
    }

    #[test]
    fn v2_roundtrip_and_low_s() {
        let key = SigningKey::from_bytes((&[7u8; 32]).into()).unwrap();
        let sig = sign(&tx(), &key).unwrap();
        assert!(sig.normalize_s().is_none());
        assert!(verify(&tx(), &key.verifying_key(), &sig).is_ok());
    }

    #[test]
    fn legacy_domain_is_not_used() {
        assert_ne!(TX_DOMAIN_V2, b"ATC-TX-DOMAIN");
    }
}
