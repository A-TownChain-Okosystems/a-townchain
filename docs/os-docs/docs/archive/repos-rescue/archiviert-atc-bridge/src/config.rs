// Bridge configuration
pub struct BridgeConfig {
    pub source_chain_id: u64,
    pub target_chain_id: u64,
    pub validator_threshold: usize,
    pub fee_rate_bps: u64,
    pub min_transfer: u64,
}
impl BridgeConfig {
    pub fn atc_eth() -> Self {
        Self { source_chain_id: 9000, target_chain_id: 1, validator_threshold: 2, fee_rate_bps: 30, min_transfer: 100 }
    }
}
