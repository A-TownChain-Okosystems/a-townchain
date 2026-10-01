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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub nonce: u64,
    pub sender: Vec<u8>,
    pub recipient: Vec<u8>,
    pub amount: u128,
    pub fee: u128,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxError {
    InvalidChainId,
    InvalidSignature,
    InvalidKey,
}

fn field(out: &mut Vec<u8>, key: &[u8], value: &[u8]) {
    out.extend_from_slice(&(key.len() as u32).to_be_bytes());
    out.extend_from_slice(key);
    out.extend_from_slice(&(value.len() as u64).to_be_bytes());
    out.extend_from_slice(value);
}

pub fn signing_preimage(tx: &Transaction) -> Result<Vec<u8>, TxError> {
    if CHAIN_ID != 658467 {
        return Err(TxError::InvalidChainId);
    }
    let mut out = Vec::new();
    field(&mut out, b"domain", TX_DOMAIN_V2);
    field(&mut out, b"chain_id", &CHAIN_ID.to_be_bytes());
    field(&mut out, b"nonce", &tx.nonce.to_be_bytes());
    field(&mut out, b"sender", &tx.sender);
    field(&mut out, b"recipient", &tx.recipient);
    field(&mut out, b"amount", &tx.amount.to_be_bytes());
    field(&mut out, b"fee", &tx.fee.to_be_bytes());
    field(&mut out, b"payload", &tx.payload);
    Ok(out)
}

pub fn digest(tx: &Transaction) -> Result<[u8; 32], TxError> {
    Ok(Sha256::digest(signing_preimage(tx)?).into())
}

pub fn sign(tx: &Transaction, key: &SigningKey) -> Result<Signature, TxError> {
    let digest = digest(tx)?;
    let mut sig = key.sign_prehash(&digest).map_err(|_| TxError::InvalidKey)?;
    if let Some(low_s) = sig.normalize_s() {
        sig = low_s;
    }
    Ok(sig)
}

pub fn verify(tx: &Transaction, key: &VerifyingKey, sig: &Signature) -> Result<(), TxError> {
    if sig.normalize_s().is_some() {
        return Err(TxError::InvalidSignature);
    }
    key.verify_prehash(&digest(tx)?, sig)
        .map_err(|_| TxError::InvalidSignature)
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
            fee: u128::MAX - 1,
            payload: b"ATC-V2".to_vec(),
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
