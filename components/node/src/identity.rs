// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! ATC-STD-600 Chain Identity implementation.
//! Genesis identity covers the complete canonical Genesis document.

use atc_algorithm::hash::atc_hash;

pub const CHAIN_ID: &str = "atc";
pub const DEVNET_NETWORK_ID: &str = "devnet";
pub const PROTOCOL_VERSION: &str = "1.0.0";
pub const VM_VERSION: &str = "1.0.0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainIdentity { pub chain_id: String, pub network_id: String, pub genesis_id: String }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeContext { pub identity: ChainIdentity, pub protocol_version: String, pub vm_version: String }

/// Genesis identity = HASH(CANONICAL_ENCODE(genesis_document)).
/// The genesis_id itself is excluded from its own preimage.
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
    if identity.genesis_id != computed { return Err(IdentityError::GenesisMismatch { configured: identity.genesis_id.clone(), computed }); }
    Ok(())
}

fn canonical_fields(fields: &[(&str, &str)]) -> Vec<u8> {
    let mut out = Vec::new();
    for (key, value) in fields {
        out.extend_from_slice(&(key.len() as u32).to_be_bytes()); out.extend_from_slice(key.as_bytes());
        out.extend_from_slice(&(value.len() as u64).to_be_bytes()); out.extend_from_slice(value.as_bytes());
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
    #[test] fn genesis_id_is_deterministic() { let p=peers(); let a=compute_genesis_id(CHAIN_ID,"A-TownChain Devnet",DEVNET_NETWORK_ID,0,&p,"0".repeat(64),PROTOCOL_VERSION,VM_VERSION); let b=compute_genesis_id(CHAIN_ID,"A-TownChain Devnet",DEVNET_NETWORK_ID,0,&p,"0".repeat(64),PROTOCOL_VERSION,VM_VERSION); assert_eq!(a,b); assert_eq!(a.len(),64); }
    #[test] fn identity_is_fail_closed() { let p=peers(); let id=compute_genesis_id(CHAIN_ID,"A-TownChain Devnet",DEVNET_NETWORK_ID,0,&p,"0".repeat(64),PROTOCOL_VERSION,VM_VERSION); let i=ChainIdentity{chain_id:CHAIN_ID.into(),network_id:DEVNET_NETWORK_ID.into(),genesis_id:id}; assert!(verify_genesis_id(&i,"A-TownChain Devnet",0,&p,"0".repeat(64),PROTOCOL_VERSION,VM_VERSION).is_ok()); }
    #[test] fn transaction_encoding_is_unambiguous() { let d=TransactionDomain{chain_id:CHAIN_ID.into(),network_id:DEVNET_NETWORK_ID.into(),protocol_version:PROTOCOL_VERSION.into(),transaction_type:"transfer".into()}; assert_ne!(d.signing_bytes(1,"alice","bob",10,1,b"ab"),d.signing_bytes(1,"alice","bob",10,1,b"a\0b")); }
}
