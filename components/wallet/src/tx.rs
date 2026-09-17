// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! ATC-STD-600 transaction domain and deterministic signing preimage.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};

pub const TX_DOMAIN: &str = "ATC-TX-DOMAIN";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionDomain {
    pub chain_id: String,
    pub network_id: String,
    pub protocol_version: String,
    pub transaction_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaction {
    pub nonce: u64,
    pub sender: Vec<u8>,
    pub recipient: Vec<u8>,
    pub value: u64,
    pub fee: u64,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxError { InvalidDomain, InvalidSignature }

impl TransactionDomain {
    pub fn validate(&self) -> Result<(), TxError> {
        if self.chain_id != "atc" || !matches!(self.network_id.as_str(), "devnet" | "testnet" | "mainnet") || self.protocol_version.is_empty() || self.transaction_type.is_empty() {
            return Err(TxError::InvalidDomain);
        }
        Ok(())
    }

    pub fn signing_preimage(&self, tx: &Transaction) -> Result<Vec<u8>, TxError> {
        self.validate()?;
        let fields: [(&str, &[u8]); 11] = [
            ("domain", TX_DOMAIN.as_bytes()),
            ("chain_id", self.chain_id.as_bytes()),
            ("network_id", self.network_id.as_bytes()),
            ("protocol_version", self.protocol_version.as_bytes()),
            ("transaction_type", self.transaction_type.as_bytes()),
            ("nonce", &tx.nonce.to_be_bytes()),
            ("sender", &tx.sender),
            ("recipient", &tx.recipient),
            ("value", &tx.value.to_be_bytes()),
            ("fee", &tx.fee.to_be_bytes()),
            ("payload", &tx.payload),
        ];
        Ok(canonical_fields(&fields))
    }

    pub fn signing_digest(&self, tx: &Transaction) -> Result<[u8; 32], TxError> {
        Ok(Sha256::digest(self.signing_preimage(tx)?).into())
    }

    pub fn sign(&self, tx: &Transaction, key: &SigningKey) -> Result<Signature, TxError> {
        Ok(key.sign(&self.signing_digest(tx)?))
    }

    pub fn verify(&self, tx: &Transaction, public_key: &VerifyingKey, signature: &Signature) -> Result<(), TxError> {
        public_key.verify(&self.signing_digest(tx)?, signature).map_err(|_| TxError::InvalidSignature)
    }
}

fn canonical_fields(fields: &[(&str, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    for (key, value) in fields {
        out.extend_from_slice(&(key.len() as u32).to_be_bytes());
        out.extend_from_slice(key.as_bytes());
        out.extend_from_slice(&(value.len() as u64).to_be_bytes());
        out.extend_from_slice(value);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn domain() -> TransactionDomain {
        TransactionDomain { chain_id: "atc".into(), network_id: "devnet".into(), protocol_version: "1.0.0".into(), transaction_type: "transfer".into() }
    }

    fn tx() -> Transaction {
        Transaction { nonce: 1, sender: vec![1; 32], recipient: vec![2; 32], value: 100, fee: 1, payload: b"hello".to_vec() }
    }

    #[test]
    fn signature_roundtrip() {
        let d = domain();
        let t = tx();
        let key = SigningKey::from_bytes(&[7u8; 32]);
        let sig = d.sign(&t, &key).unwrap();
        assert!(d.verify(&t, &key.verifying_key(), &sig).is_ok());
    }

    #[test]
    fn network_replay_domain_isolation() {
        let t = tx();
        let key = SigningKey::from_bytes(&[7u8; 32]);
        let sig = domain().sign(&t, &key).unwrap();
        let mut other = domain();
        other.network_id = "mainnet".into();
        assert!(other.verify(&t, &key.verifying_key(), &sig).is_err());
    }

    #[test]
    fn payload_is_part_of_authenticated_data() {
        let d = domain();
        let key = SigningKey::from_bytes(&[7u8; 32]);
        let sig = d.sign(&tx(), &key).unwrap();
        let mut altered = tx();
        altered.payload.push(0);
        assert!(d.verify(&altered, &key.verifying_key(), &sig).is_err());
    }
}
