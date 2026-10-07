// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Canonical A-TownChain monetary policy.
//!
//! Consensus parameters:
//! - maximum supply: 360,000,000 ATC
//! - target block time: 360 seconds
//! - halving interval: 360,000 blocks
//! - 36 halvings
//! - block-height driven; timestamps are not consensus inputs
//!
//! The initial subsidy is 500 ATC/block. Thirty-six emission epochs
//! (epochs 0..35) are active. At height 12,960,000 the subsidy becomes zero.
//! The final emission block receives any remaining integer-denominated supply
//! so the maximum supply is reached exactly without exceeding the cap.

pub const ATC_BASE_UNITS: u128 = 1_000_000_000_000_000_000;
pub const MAX_SUPPLY: u128 = 360_000_000 * ATC_BASE_UNITS;
pub const TARGET_BLOCK_TIME_SECS: u64 = 360;
pub const HALVING_INTERVAL_BLOCKS: u64 = 360_000;
pub const MAX_HALVINGS: u32 = 36;
pub const INITIAL_SUBSIDY: u128 = 500 * ATC_BASE_UNITS;
pub const FINAL_EMISSION_BLOCK: u64 = HALVING_INTERVAL_BLOCKS * (MAX_HALVINGS as u64) - 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonetaryPolicy;

impl MonetaryPolicy {
    pub const fn epoch(height: u64) -> u32 {
        let epoch = height / HALVING_INTERVAL_BLOCKS;
        if epoch >= MAX_HALVINGS as u64 { MAX_HALVINGS } else { epoch as u32 }
    }

    pub const fn raw_subsidy(height: u64) -> u128 {
        let epoch = Self::epoch(height);
        if epoch >= MAX_HALVINGS { return 0; }
        INITIAL_SUBSIDY >> epoch
    }

    pub const fn subsidy(height: u64, issued_before_block: u128) -> u128 {
        if issued_before_block >= MAX_SUPPLY { return 0; }
        let remaining = MAX_SUPPLY - issued_before_block;
        let raw = Self::raw_subsidy(height);
        if height == FINAL_EMISSION_BLOCK && raw > 0 { return remaining; }
        if raw < remaining { raw } else { remaining }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn protocol_constants_are_fixed() {
        assert_eq!(MAX_SUPPLY, 360_000_000 * ATC_BASE_UNITS);
        assert_eq!(TARGET_BLOCK_TIME_SECS, 360);
        assert_eq!(HALVING_INTERVAL_BLOCKS, 360_000);
        assert_eq!(MAX_HALVINGS, 36);
        assert_eq!(FINAL_EMISSION_BLOCK, 12_959_999);
    }
    #[test]
    fn epoch_boundaries_are_height_only() {
        assert_eq!(MonetaryPolicy::epoch(0), 0);
        assert_eq!(MonetaryPolicy::epoch(359_999), 0);
        assert_eq!(MonetaryPolicy::epoch(360_000), 1);
        assert_eq!(MonetaryPolicy::epoch(720_000), 2);
        assert_eq!(MonetaryPolicy::epoch(12_599_999), 34);
        assert_eq!(MonetaryPolicy::epoch(12_600_000), 35);
        assert_eq!(MonetaryPolicy::epoch(12_960_000), 36);
    }
    #[test]
    fn subsidy_halves_at_each_boundary() {
        assert_eq!(MonetaryPolicy::raw_subsidy(0), 500 * ATC_BASE_UNITS);
        assert_eq!(MonetaryPolicy::raw_subsidy(360_000), 250 * ATC_BASE_UNITS);
        assert_eq!(MonetaryPolicy::raw_subsidy(720_000), 125 * ATC_BASE_UNITS);
        assert_eq!(MonetaryPolicy::raw_subsidy(12_600_000), 14_551_915_228);
        assert_eq!(MonetaryPolicy::raw_subsidy(12_960_000), 0);
    }
    #[test]
    fn supply_cap_is_fail_closed() {
        assert_eq!(MonetaryPolicy::subsidy(0, MAX_SUPPLY), 0);
        assert_eq!(MonetaryPolicy::subsidy(0, MAX_SUPPLY - 10), 10);
    }
    #[test]
    fn final_emission_block_closes_integer_rounding_dust() {
        let mut issued = 0u128;
        for height in 0..=FINAL_EMISSION_BLOCK {
            let reward = MonetaryPolicy::subsidy(height, issued);
            issued += reward;
        }
        assert_eq!(issued, MAX_SUPPLY);
        assert_eq!(MonetaryPolicy::subsidy(FINAL_EMISSION_BLOCK + 1, issued), 0);
    }
}