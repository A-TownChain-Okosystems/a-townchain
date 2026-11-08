// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Order specification & lifecycle (ATC-9006).

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderStatus {
    Listed,
    Filled,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FillPolicy {
    FullOnly,
    Partial { max_fill: u128 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub order_id: u64,
    pub asset_id: u64,
    pub seller: String,
    pub price: u128,
    pub expiry_height: u64,
    pub nonce: u64,
    pub signature: Vec<u8>,
    pub fill_policy: FillPolicy,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderError {
    InvalidPrice,
    ExpiredOrder,
    NonceAlreadyUsed,
    OrderNotActive,
    PartialFillExceeded,
    InvalidSignature,
}

impl Order {
    pub fn new(
        order_id: u64,
        asset_id: u64,
        seller: impl Into<String>,
        price: u128,
        expiry_height: u64,
        nonce: u64,
        signature: Vec<u8>,
        fill_policy: FillPolicy,
        current_height: u64,
    ) -> Result<Self, OrderError> {
        if price == 0 {
            return Err(OrderError::InvalidPrice);
        }
        if expiry_height <= current_height {
            return Err(OrderError::ExpiredOrder);
        }
        if signature.is_empty() {
            return Err(OrderError::InvalidSignature);
        }
        Ok(Self {
            order_id,
            asset_id,
            seller: seller.into(),
            price,
            expiry_height,
            nonce,
            signature,
            fill_policy,
            status: OrderStatus::Listed,
        })
    }

    pub fn validate_execution(&self, current_height: u64) -> Result<(), OrderError> {
        if self.status != OrderStatus::Listed {
            return Err(OrderError::OrderNotActive);
        }
        if current_height >= self.expiry_height {
            return Err(OrderError::ExpiredOrder);
        }
        Ok(())
    }
}

/// Replay protection tracker for consumed seller nonces.
#[derive(Debug, Default)]
pub struct OrderNonceGuard {
    used_nonces: HashSet<(String, u64)>,
}

impl OrderNonceGuard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn consume_nonce(&mut self, seller: &str, nonce: u64) -> Result<(), OrderError> {
        let key = (seller.to_string(), nonce);
        if self.used_nonces.contains(&key) {
            Err(OrderError::NonceAlreadyUsed)
        } else {
            self.used_nonces.insert(key);
            Ok(())
        }
    }

    pub fn is_nonce_used(&self, seller: &str, nonce: u64) -> bool {
        self.used_nonces.contains(&(seller.to_string(), nonce))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_creation_and_expiry() {
        let order = Order::new(1, 200, "alice", 5000, 100, 1, vec![0xaa], FillPolicy::FullOnly, 10).unwrap();
        assert_eq!(order.status, OrderStatus::Listed);
        assert!(order.validate_execution(50).is_ok());
        assert_eq!(order.validate_execution(100), Err(OrderError::ExpiredOrder));
    }

    #[test]
    fn test_order_invalid_inputs() {
        assert_eq!(
            Order::new(1, 200, "alice", 0, 100, 1, vec![0xaa], FillPolicy::FullOnly, 10),
            Err(OrderError::InvalidPrice)
        );
        assert_eq!(
            Order::new(1, 200, "alice", 100, 10, 1, vec![0xaa], FillPolicy::FullOnly, 10),
            Err(OrderError::ExpiredOrder)
        );
        assert_eq!(
            Order::new(1, 200, "alice", 100, 100, 1, vec![], FillPolicy::FullOnly, 10),
            Err(OrderError::InvalidSignature)
        );
    }

    #[test]
    fn test_nonce_replay_guard() {
        let mut guard = OrderNonceGuard::new();
        assert!(guard.consume_nonce("alice", 1).is_ok());
        assert_eq!(guard.consume_nonce("alice", 1), Err(OrderError::NonceAlreadyUsed));
        assert!(guard.consume_nonce("alice", 2).is_ok());
        assert!(guard.is_nonce_used("alice", 1));
        assert!(!guard.is_nonce_used("bob", 1));
    }
}
