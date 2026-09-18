//! A-TownChain SDK protocol primitives.
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Account { pub address: String, pub nonce: u64 }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction { pub from: String, pub to: String, pub nonce: u64, pub amount: u64, pub payload: Vec<u8> }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockRef { pub height: u64, pub hash: String }
pub fn validate_nonce(actual: u64, expected: u64) -> bool { actual == expected }
