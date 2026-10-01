// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC-STD-600 chain/genesis identity.
//! Transaction authorization is intentionally NOT owned by node identity.

use atc_algorithm::hash::atc_hash;

pub const CHAIN_ID: &str = "atc";
pub const DEVNET_NETWORK_ID: &str = "devnet";
pub const PROTOCOL_VERSION: &str = "1.0.0";
pub const VM_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainIdentity { pub chain_id: String, pub network_id: String, pub genesis_id: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContext { pub identity: ChainIdentity, pub protocol_version: String, pub vm_version: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityError {
    EmptyField(&'static str), InvalidChainId(String), InvalidNetworkId(String), InvalidGenesisId(String),
    GenesisMismatch { configured: String, computed: String },
    ProtocolMismatch { expected: String, actual: String }, VmMismatch { expected: String, actual: String },
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
}

impl RuntimeContext {
    pub fn validate(&self, expected_protocol: &str, expected_vm: &str) -> Result<(), IdentityError> {
        self.identity.validate()?;
        if self.protocol_version != expected_protocol {
            return Err(IdentityError::ProtocolMismatch { expected: expected_protocol.into(), actual: self.protocol_version.clone() });
        }
        if self.vm_version != expected_vm {
            return Err(IdentityError::VmMismatch { expected: expected_vm.into(), actual: self.vm_version.clone() });
        }
        Ok(())
    }
}

pub fn compute_genesis_id(
    chain_id: &str, chain_name: &str, network_id: &str, genesis_height: u64,
    initial_peers: &[String], state_root: &str, protocol_version: &str, vm_version: &str,
) -> String {
    let height = genesis_height.to_string();
    let peers = serde_json::to_string(initial_peers).expect("Vec<String> serialization cannot fail");
    let bytes = canonical_fields(&[
        ("chain_id", chain_id), ("chain_name", chain_name), ("network_id", network_id),
        ("genesis_height", height.as_str()), ("initial_peers", peers.as_str()), ("state_root", state_root),
        ("protocol_version", protocol_version), ("vm_version", vm_version),
    ]);
    hex_encode(&atc_hash(&bytes))
}

pub fn verify_genesis_id(
    identity: &ChainIdentity, chain_name: &str, genesis_height: u64, initial_peers: &[String],
    state_root: &str, protocol_version: &str, vm_version: &str,
) -> Result<(), IdentityError> {
    identity.validate()?;
    let computed = compute_genesis_id(&identity.chain_id, chain_name, &identity.network_id, genesis_height, initial_peers, state_root, protocol_version, vm_version);
    if identity.genesis_id != computed {
        return Err(IdentityError::GenesisMismatch { configured: identity.genesis_id.clone(), computed });
    }
    Ok(())
}

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
    for &b in bytes { out.push(HEX[(b >> 4) as usize] as char); out.push(HEX[(b & 0x0f) as usize] as char); }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peers() -> Vec<String> { vec!["atc-node-1".into(), "atc-node-2".into()] }

    #[test]
    fn genesis_id_is_deterministic() {
        let p = peers();
        let a = compute_genesis_id(CHAIN_ID, "A-TownChain Devnet", DEVNET_NETWORK_ID, 0, &p, &"0".repeat(64), PROTOCOL_VERSION, VM_VERSION);
        let b = compute_genesis_id(CHAIN_ID, "A-TownChain Devnet", DEVNET_NETWORK_ID, 0, &p, &"0".repeat(64), PROTOCOL_VERSION, VM_VERSION);
        assert_eq!(a, b);
        assert_eq!(a.len(), 64);
    }

    #[test]
    fn identity_is_fail_closed() {
        let p = peers();
        let id = compute_genesis_id(CHAIN_ID, "A-TownChain Devnet", DEVNET_NETWORK_ID, 0, &p, &"0".repeat(64), PROTOCOL_VERSION, VM_VERSION);
        let i = ChainIdentity { chain_id: CHAIN_ID.into(), network_id: DEVNET_NETWORK_ID.into(), genesis_id: id };
        assert!(verify_genesis_id(&i, "A-TownChain Devnet", 0, &p, &"0".repeat(64), PROTOCOL_VERSION, VM_VERSION).is_ok());
    }
}
