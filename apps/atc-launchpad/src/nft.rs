// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! NFT Launchpad Module — Collection-Konfiguration, Per-Wallet Limits & Minting (LP-NFT-001).

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NftError {
    MaxSupplyReached,
    PerWalletLimitExceeded,
    InsufficientPayment,
    MetadataMismatch,
    ZeroQuantity,
}

#[derive(Debug, Clone)]
pub struct NftCollectionConfig {
    pub collection_id: String,
    pub max_supply: u32,
    pub mint_price: u128,
    pub per_wallet_limit: u32,
    pub metadata_root: [u8; 32],
}

pub struct NftLaunchpad {
    config: NftCollectionConfig,
    total_minted: u32,
    user_mints: HashMap<u64, u32>,
}

impl NftLaunchpad {
    pub fn new(config: NftCollectionConfig) -> Self {
        Self {
            config,
            total_minted: 0,
            user_mints: HashMap::new(),
        }
    }

    pub fn config(&self) -> &NftCollectionConfig {
        &self.config
    }

    pub fn total_minted(&self) -> u32 {
        self.total_minted
    }

    pub fn user_mint_count(&self, user: u64) -> u32 {
        self.user_mints.get(&user).copied().unwrap_or(0)
    }

    pub fn mint(&mut self, user: u64, quantity: u32, payment: u128) -> Result<Vec<u32>, NftError> {
        if quantity == 0 {
            return Err(NftError::ZeroQuantity);
        }

        let required_payment = self.config.mint_price.saturating_mul(quantity as u128);
        if payment < required_payment {
            return Err(NftError::InsufficientPayment);
        }

        if self.total_minted.saturating_add(quantity) > self.config.max_supply {
            return Err(NftError::MaxSupplyReached);
        }

        let user_current = self.user_mint_count(user);
        if user_current.saturating_add(quantity) > self.config.per_wallet_limit {
            return Err(NftError::PerWalletLimitExceeded);
        }

        let mut token_ids = Vec::with_capacity(quantity as usize);
        for i in 1..=quantity {
            token_ids.push(self.total_minted + i);
        }

        self.total_minted += quantity;
        *self.user_mints.entry(user).or_insert(0) += quantity;

        Ok(token_ids)
    }

    pub fn verify_reveal(&self, metadata_hash: &[u8; 32]) -> bool {
        &self.config.metadata_root == metadata_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nft_minting_and_limits() {
        let config = NftCollectionConfig {
            collection_id: "atc-dragons".to_string(),
            max_supply: 5,
            mint_price: 100,
            per_wallet_limit: 2,
            metadata_root: [7u8; 32],
        };
        let mut nft = NftLaunchpad::new(config);

        // Mint 2 NFTs for user 1 (success)
        let ids = nft.mint(1, 2, 200).unwrap();
        assert_eq!(ids, vec![1, 2]);
        assert_eq!(nft.total_minted(), 2);

        // Exceed wallet limit for user 1
        assert_eq!(nft.mint(1, 1, 100), Err(NftError::PerWalletLimitExceeded));

        // Insufficient payment
        assert_eq!(nft.mint(2, 1, 50), Err(NftError::InsufficientPayment));

        // User 2 mints 2, User 3 mints 1 (reaches max supply 5)
        assert_eq!(nft.mint(2, 2, 200).unwrap(), vec![3, 4]);
        assert_eq!(nft.mint(3, 1, 100).unwrap(), vec![5]);
        assert_eq!(nft.total_minted(), 5);

        // Max supply reached
        assert_eq!(nft.mint(4, 1, 100), Err(NftError::MaxSupplyReached));

        // Reveal verification
        assert!(nft.verify_reveal(&[7u8; 32]));
        assert!(!nft.verify_reveal(&[0u8; 32]));
    }
}
