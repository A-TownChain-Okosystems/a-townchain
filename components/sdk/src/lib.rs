//! A-TownChain SDK protocol primitives.
//! Canonical client-side types. Protocol authority remains in L2.

use serde::{Deserialize, Serialize};

pub const CHAIN_ID: u64 = 658467;
pub const TX_DOMAIN_V2: &str = "ATC-TX-DOMAIN-V2";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account {
    pub address: String,
    pub nonce: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub chain_id: u64,
    pub from: String,
    pub to: String,
    pub nonce: u64,
    pub amount: u128,
    pub fee: u128,
    pub payload: Vec<u8>,
}

pub fn validate_nonce(actual: u64, expected: u64) -> bool {
    actual == expected
}

pub fn validate_amount(amount: u128) -> bool {
    amount > 0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockRef {
    pub height: u64,
    pub hash: String,
}
