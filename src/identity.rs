// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! ATC-STD-600 Chain Identity implementation.
//! All validation is fail-closed. Deployment values are explicit inputs.

use atc_algorithm::hash::atc_hash;

pub const CHAIN_ID: &str = "atc";
pub const DEVNET_NETWORK_ID: &str = "devnet";
pub const PROTOCOL_VERSION: &str = "1.0.0";
pub const VM_VERSION: &str = "1.0.0";
pub const TX_DOMAIN: &str = "ATC-TX-DOMAIN";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainIdentity {
    pub chain_id: String,
    pub network_id: String,
    pub genesis_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContext {
    pub identity: ChainIdentity,
    pub protocol_version: String,
    pub vm_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransactionDomain {
    pub chain_id: String,
    pub network_id: String,
    pub protocol_version: String,
    pub transaction_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityError {
    EmptyField(&'static str),
    InvalidChainId(String),
    InvalidNetworkId(String),
    InvalidGenesisId(String),
    GenesisMismatch { configured: String, computed: String },
    ProtocolMismatch { expected: String, actual: String },
    VmMismatch { expected: String, actual: String },
}

impl ChainIdentity {
    pub fn validate(&self) -> Result<(), IdentityError> {
        if self.chain_id.is_empty() { return Err(IdentityError::EmptyField("chain_id")); }
        if self.network_id.is_empty() { return Err(IdentityError::EmptyField("network_id")); }
        if self.genesis_id.is_empty() { return Err(IdentityError::EmptyField("genesis_id")); }
        if self.chain_id != CHAIN_ID { return Err(IdentityError::InvalidChainId(self.chain_id.clone())); }
        if !matches!(self.network_id.as_str(), "devnet" | "testnet" | "mainnet") {
            return Err(IdentityError::InvalidNetworkId(self.network_id.clone()));
        }
        if self.genesis_id.len() != 64 || !self.genesis_id.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(IdentityError::InvalidGenesisId(self.genesis_id.clone()));
        }
        Ok(())
    }

    pub fn genesis_document_encoding(&self, protocol_version: &str, vm_version: &str) -> Vec<u8> {
        canonical_fields(&[
            ("chain_id", self.chain_id.as_str()),
            ("network_id", self.network_id.as_str()),
            ("protocol_version", protocol_version),
            ("vm_version", vm_version),
        ])
    }
}

impl RuntimeContext {
    pub fn validate(&self, expected_protocol: &str, expected_vm: &str) -> Result<(), IdentityError> {
        self.identity.validate()?;
        if self.protocol_version != expected_protocol {
            return Err(IdentityError::ProtocolMismatch { expected: expected_protocol.to_string(), actual: self.protocol_version.clone() });
        }
        if self.vm_version != expected_vm {
            return Err(IdentityError::VmMismatch { expected: expected_vm.to_string(), actual: self.vm_version.clone() });
        }
        Ok(())
    }
}

impl TransactionDomain {
    pub fn signing_bytes(&self, nonce: u64, sender: &str, recipient: &str, value: u64, fee: u64, payload: &[u8]) -> Vec<u8> {
        let payload_hex = hex_encode(payload);
        canonical_fields(&[
            ("domain", TX_DOMAIN),
            ("chain_id", self.chain_id.as_str()),
            ("network_id", self.network_id.as_str()),
            ("protocol_version", self.protocol_version.as_str()),
            ("transaction_type", self.transaction_type.as_str()),
            ("nonce", &nonce.to_string()),
            ("sender", sender),
            ("recipient", recipient),
            ("value", &value.to_string()),
            ("fee", &fee.to_string()),
            ("payload_hex", payload_hex.as_str()),
        ])
    }
}

/// Computes the deterministic Genesis identity over the canonical Genesis document.
/// The configured genesis_id is never part of its own preimage.
pub fn compute_genesis_id(chain_id: &str, network_id: &str, protocol_version: &str, vm_version: &str) -> String {
    let bytes = canonical_fields(&[
        ("chain_id", chain_id),
        ("network_id", network_id),
        ("protocol_version", protocol_version),
        ("vm_version", vm_version),
    ]);
    hex_encode(&atc_hash(&bytes))
}

pub fn verify_genesis_id(identity: &ChainIdentity, protocol_version: &str, vm_version: &str) -> Result<(), IdentityError> {
    identity.validate()?;
    let computed = compute_genesis_id(&identity.chain_id, &identity.network_id, protocol_version, vm_version);
    if identity.genesis_id != computed {
        return Err(IdentityError::GenesisMismatch { configured: identity.genesis_id.clone(), computed });
    }
    Ok(())
}

/// Length-prefixed field encoding: no ambiguous delimiter concatenation.
fn canonical_fields(fields: &[(&str, &str)]) -> Vec<u8> {
    let mut out = Vec::new();
    for (key, value) in fields {
        out.extend_from_slice(&(key.len() as u32).to_be_bytes());
        out.extend_from_slice(key.as_bytes());
        out.extend_from_slice(&(value.len() as u64).to_be_bytes());
        out.extend_from_slice(value.as_bytes());
    }
    out
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genesis_id_is_deterministic() {
        let a = compute_genesis_id(CHAIN_ID, DEVNET_NETWORK_ID, PROTOCOL_VERSION, VM_VERSION);
        let b = compute_genesis_id(CHAIN_ID, DEVNET_NETWORK_ID, PROTOCOL_VERSION, VM_VERSION);
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }

    #[test]
    fn identity_is_fail_closed() {
        let identity = ChainIdentity { chain_id: CHAIN_ID.into(), network_id: DEVNET_NETWORK_ID.into(), genesis_id: compute_genesis_id(CHAIN_ID, DEVNET_NETWORK_ID, PROTOCOL_VERSION, VM_VERSION) };
        assert!(verify_genesis_id(&identity, PROTOCOL_VERSION, VM_VERSION).is_ok());
        let mut bad = identity.clone();
        bad.network_id = "mainnet".into();
        assert!(verify_genesis_id(&bad, PROTOCOL_VERSION, VM_VERSION).is_err());
    }

    #[test]
    fn transaction_encoding_is_unambiguous() {
        let d = TransactionDomain { chain_id: CHAIN_ID.into(), network_id: DEVNET_NETWORK_ID.into(), protocol_version: PROTOCOL_VERSION.into(), transaction_type: "transfer".into() };
        let a = d.signing_bytes(1, "alice", "bob", 10, 1, b"ab");
        let b = d.signing_bytes(1, "alice", "bob", 10, 1, b"a\x00b");
        assert_ne!(a, b);
    }
}
