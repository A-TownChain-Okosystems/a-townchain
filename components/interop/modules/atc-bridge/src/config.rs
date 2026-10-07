// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Bridge configuration

pub struct BridgeConfig {
    pub source_chain_id: u64,
    pub target_chain_id: u64,
    pub validator_threshold: usize,
    pub fee_rate_bps: u64,
    pub min_transfer: u128,
}

impl BridgeConfig {
    pub fn atc_eth() -> Self {
        Self {
            source_chain_id: 658467,
            target_chain_id: 1,
            validator_threshold: 2,
            fee_rate_bps: 30,
            min_transfer: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atc_eth_uses_canonical_economic_amount_width() {
        assert_eq!(BridgeConfig::atc_eth().min_transfer, 100u128);
    }
}
