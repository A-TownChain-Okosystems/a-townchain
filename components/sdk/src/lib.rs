//! A-TownChain SDK protocol primitives.
use atc_ledger::Amount;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub address: String,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub nonce: u64,
    pub amount: Amount,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockRef {
    pub height: u64,
    pub hash: String,
}

pub fn validate_nonce(actual: u64, expected: u64) -> bool {
    actual == expected
}

pub fn validate_amount(amount: Amount) -> bool {
    amount > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_validation_supports_u128_max() {
        assert!(validate_amount(u128::MAX));
    }

    #[test]
    fn zero_amount_is_invalid() {
        assert!(!validate_amount(0));
    }
}
