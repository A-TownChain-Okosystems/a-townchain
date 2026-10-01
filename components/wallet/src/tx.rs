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
pub enum TxType { Transfer = 0, Stake = 1, Unstake = 2, Contract = 3 }

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
pub enum TxError { InvalidChainId, EmptySender, InvalidSignature }

fn put_bytes(out: &mut Vec<u8>, value: &[u8]) {
    assert!(value.len() <= u32::MAX as usize);
    out.extend_from_slice(&(value.len() as u32).to_be_bytes());
    out.extend_from_slice(value);
}

impl Transaction {
    pub fn signing_bytes(&self) -> Result<Vec<u8>, TxError> {
        if self.chain_id != CHAIN_ID { return Err(TxError::InvalidChainId); }
        if self.sender_did.is_empty() { return Err(TxError::EmptySender); }
        let mut b = Vec::with_capacity(128 + self.payload.len());
        b.extend_from_slice(TX_DOMAIN_V2);
        b.extend_from_slice(&self.chain_id.to_be_bytes());
        b.push(self.tx_type as u8);
        put_bytes(&mut b, self.sender_did.as_bytes());
        match &self.recipient_did {
            Some(v) => { b.push(1); put_bytes(&mut b, v.as_bytes()); }
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

pub fn sign(tx: &Transaction, key: &SigningKey) -> Result<Signature, TxError> {
    let digest = tx.digest()?;
    let mut sig: Signature =
        key.sign_prehash(&digest).map_err(|_| TxError::InvalidSignature)?;
    if let Some(low_s) = sig.normalize_s() { sig = low_s; }
    Ok(sig)
}

pub fn verify(tx: &Transaction, key: &VerifyingKey, sig: &Signature) -> Result<(), TxError> {
    if sig.normalize_s().is_some() { return Err(TxError::InvalidSignature); }
    key.verify_prehash(&tx.digest()?, sig).map_err(|_| TxError::InvalidSignature)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tx() -> Transaction {
        Transaction {
            chain_id: CHAIN_ID,
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
    fn canonical_vector_digest() {
        assert_eq!(
            hex::encode(tx().signing_bytes().unwrap()),
            "4154432d54582d444f4d41494e2d563200000000000a0c23000000000a4154432d73656e646572010000000d4154432d726563697069656e74000000000000000000000000000000640000000000000000000000000000000100000000000003e80000000000000007000000006553f1000000000568656c6c6f0909090909090909090909090909090909090909090909090909090909090909"
        );
        assert_eq!(hex::encode(tx().digest().unwrap()), "8608d1530c0c8dd02207903ec6b24071878b36fca0299b2534cef76e79d340be");
    }

    #[test]
    fn u128_is_fixed_16_byte_big_endian() {
        let mut t = tx();
        t.amount = u128::MAX;
        assert_eq!(t.signing_bytes().unwrap().windows(16).filter(|w| *w == u128::MAX.to_be_bytes()).count(), 1);
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
