// Copyright (c) 2026 A-TownChain-Okosystems — Apache-2.0
//! ATC-STD-600 execution context and fail-closed gate.

pub const CHAIN_ID: &str = "atc";
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainContext {
    pub chain_id: String,
    pub network_id: String,
    pub genesis_id: String,
    pub protocol_version: String,
    pub vm_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextError {
    InvalidChainId,
    InvalidNetworkId,
    InvalidGenesisId,
    ProtocolMismatch,
    VmMismatch,
    GenesisMismatch,
}

impl ChainContext {
    pub fn validate(&self, expected_protocol: &str, expected_vm: &str) -> Result<(), ContextError> {
        if self.chain_id != CHAIN_ID { return Err(ContextError::InvalidChainId); }
        if !matches!(self.network_id.as_str(), "devnet" | "testnet" | "mainnet") { return Err(ContextError::InvalidNetworkId); }
        if self.genesis_id.len() != 64 || !self.genesis_id.bytes().all(|b| b.is_ascii_hexdigit()) { return Err(ContextError::InvalidGenesisId); }
        if self.protocol_version != expected_protocol { return Err(ContextError::ProtocolMismatch); }
        if self.vm_version != expected_vm { return Err(ContextError::VmMismatch); }
        Ok(())
    }

    pub fn verify_genesis_id(&self, computed: &str) -> Result<(), ContextError> {
        if self.genesis_id != computed { return Err(ContextError::GenesisMismatch); }
        Ok(())
    }
}

/// State transitions MUST call this gate before VM execution.
pub fn execution_gate(context: &ChainContext, computed_genesis_id: &str, expected_protocol: &str, expected_vm: &str) -> Result<(), ContextError> {
    context.validate(expected_protocol, expected_vm)?;
    context.verify_genesis_id(computed_genesis_id)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> ChainContext {
        ChainContext {
            chain_id: "atc".into(),
            network_id: "devnet".into(),
            genesis_id: "a".repeat(64),
            protocol_version: "1.0.0".into(),
            vm_version: "1.0.0".into(),
        }
    }

    #[test]
    fn valid_context_passes() {
        assert!(execution_gate(&ctx(), &"a".repeat(64), "1.0.0", "1.0.0").is_ok());
    }

    #[test]
    fn identity_mismatch_fails_closed() {
        let mut c = ctx();
        c.network_id = "mainnet".into();
        assert!(execution_gate(&c, &"a".repeat(64), "1.0.0", "1.0.0").is_err());
    }

    #[test]
    fn genesis_mismatch_fails_closed() {
        assert!(execution_gate(&ctx(), &"b".repeat(64), "1.0.0", "1.0.0").is_err());
    }
}
