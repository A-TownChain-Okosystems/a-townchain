// Copyright (c) 2026 Michael Wroblewski — Apache-2.0

use atc_launchpad::nft::{NftCollectionConfig, NftError, NftLaunchpad};
use atc_launchpad::sale::{Sale, SaleConfig, SaleError, SaleState};
use atc_launchpad::vesting::{VestingError, VestingManager, VestingSchedule};
use atc_launchpad::whitelist::{WhitelistEntry, WhitelistError, WhitelistManager};

#[test]
fn test_integration_whitelisted_presale_to_vesting() {
    // 1. Whitelist Setup
    let root = [1u8; 32];
    let mut wm = WhitelistManager::new(root);
    wm.add_entry(WhitelistEntry {
        address: 42,
        tier: 1,
        limit: 1000,
        expiry_slot: 100,
    });

    // Verify user 42 is eligible for 500 tokens
    let entry = wm.verify_and_authorize(42, 500, 50).unwrap();
    assert_eq!(entry.limit, 1000);

    // 2. Sale Setup
    let sale_config = SaleConfig {
        hard_cap: 5000,
        soft_cap: 1000,
        min_contribution: 50,
        max_contribution_per_user: 1000,
        start_slot: 10,
        end_slot: 100,
    };
    let mut sale = Sale::new(sale_config);
    sale.transition_to(SaleState::Configured).unwrap();
    sale.transition_to(SaleState::WhitelistOpen).unwrap();

    // User 42 contributes 500 tokens
    sale.contribute_at_slot(42, 500, 50).unwrap();
    assert_eq!(sale.user_contribution(42), 500);

    // User 99 contributes 1000 tokens (reaches soft cap & ends)
    sale.contribute_at_slot(99, 1000, 60).unwrap();
    sale.transition_to(SaleState::Ended).unwrap();
    sale.transition_to(SaleState::Claimable).unwrap();

    // 3. Vesting Setup for User 42's purchased tokens
    let schedule = VestingSchedule {
        total_allocation: 500,
        start_slot: 100,
        cliff_slot: 120,
        duration_slots: 100,
        interval_slots: 20,
    };
    let mut vm = VestingManager::new(schedule).unwrap();
    vm.set_beneficiary_allocation(42, 500);

    // Check vesting claim at cliff (slot 120 -> 20 slots passed out of 100 = 20% = 100)
    assert_eq!(vm.calculate_claimable(42, 120), 100);
    assert_eq!(vm.claim(42, 120), Ok(100));
    assert_eq!(vm.calculate_claimable(42, 120), 0);
}

#[test]
fn test_integration_nft_launchpad_flow() {
    let config = NftCollectionConfig {
        collection_id: "atc-genesis".to_string(),
        max_supply: 10,
        mint_price: 250,
        per_wallet_limit: 3,
        metadata_root: [9u8; 32],
    };
    let mut nft_sale = NftLaunchpad::new(config);

    // User 1 mints 2 NFTs
    let minted = nft_sale.mint(1, 2, 500).unwrap();
    assert_eq!(minted, vec![1, 2]);

    // User 2 tries to mint 4 NFTs (exceeds per_wallet_limit of 3)
    assert_eq!(
        nft_sale.mint(2, 4, 1000),
        Err(NftError::PerWalletLimitExceeded)
    );

    // User 2 mints 3 NFTs successfully
    let minted2 = nft_sale.mint(2, 3, 750).unwrap();
    assert_eq!(minted2, vec![3, 4, 5]);

    assert_eq!(nft_sale.total_minted(), 5);
}

#[test]
fn test_integration_unsuccessful_sale_refund_flow() {
    let sale_config = SaleConfig {
        hard_cap: 10000,
        soft_cap: 5000,
        min_contribution: 100,
        max_contribution_per_user: 2000,
        start_slot: 100,
        end_slot: 500,
    };
    let mut sale = Sale::new(sale_config);
    sale.transition_to(SaleState::Configured).unwrap();
    sale.transition_to(SaleState::PublicSale).unwrap();

    // Contributor 1 deposits 1000 (below soft_cap 5000)
    sale.contribute_at_slot(1, 1000, 200).unwrap();
    assert_eq!(sale.total(), 1000);

    // Sale ends without reaching soft cap
    sale.transition_to(SaleState::Ended).unwrap();
    assert_eq!(sale.transition_to(SaleState::Settled), Ok(()));

    // Refund contributor 1
    let refunded = sale.refund(1).unwrap();
    assert_eq!(refunded, 1000);
    assert_eq!(sale.refund(1), Err(SaleError::NothingToRefund));
}

#[test]
fn test_integration_vesting_full_lifecycle_invariant() {
    let schedule = VestingSchedule {
        total_allocation: 10000,
        start_slot: 1000,
        cliff_slot: 1000,
        duration_slots: 500,
        interval_slots: 50,
    };
    let mut vm = VestingManager::new(schedule).unwrap();
    vm.set_beneficiary_allocation(88, 10000);

    for slot in (1000..=1600).step_by(25) {
        assert!(vm.check_invariants(88, slot));
        let _ = vm.claim(88, slot);
        assert!(vm.check_invariants(88, slot));
    }

    let state = vm.get_beneficiary_state(88).unwrap();
    assert_eq!(state.claimed_amount, 10000);
    assert_eq!(vm.calculate_claimable(88, 2000), 0);
    assert_eq!(vm.claim(88, 2000), Err(VestingError::NothingToClaim));
}

#[test]
fn test_integration_whitelist_revocation_prevents_sale_entry() {
    let mut wm = WhitelistManager::new([0u8; 32]);
    wm.add_entry(WhitelistEntry {
        address: 7,
        tier: 2,
        limit: 2000,
        expiry_slot: 300,
    });

    wm.revoke(7);
    let auth_res = wm.verify_and_authorize(7, 500, 100);
    assert_eq!(auth_res, Err(WhitelistError::AddressRevoked));
}
