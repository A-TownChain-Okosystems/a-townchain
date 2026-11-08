// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Settlement interface & adapter implementation (ATC-9007).

use std::collections::HashMap;
use crate::order::{Order, OrderError, OrderNonceGuard, OrderStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettlementError {
    OrderError(OrderError),
    InsufficientBalance,
    AssetNotFound,
    UnauthorizedOwnership,
    ArithmeticOverflow,
    InvalidFeeRate,
}

impl From<OrderError> for SettlementError {
    fn from(err: OrderError) -> Self {
        SettlementError::OrderError(err)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeeSplit {
    pub net_seller_amount: u128,
    pub royalty_amount: u128,
    pub protocol_fee_amount: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettlementEvent {
    pub order_id: u64,
    pub buyer: String,
    pub seller: String,
    pub asset_id: u64,
    pub total_price: u128,
    pub net_to_seller: u128,
    pub royalty: u128,
    pub protocol_fee: u128,
}

/// Trait defining the settlement adapter requirements.
pub trait SettlementAdapter {
    fn validate_order(&self, order: &Order, current_height: u64) -> Result<(), SettlementError>;
    fn execute_payment(&mut self, buyer: &str, seller: &str, amount: u128) -> Result<(), SettlementError>;
    fn transfer_ownership(&mut self, asset_id: u64, from: &str, to: &str) -> Result<(), SettlementError>;
    fn split_fees(&self, amount: u128, royalty_bps: u16, fee_bps: u16) -> Result<FeeSplit, SettlementError>;
}

/// Default in-memory implementation of `SettlementAdapter`.
pub struct DefaultSettlementAdapter {
    pub balances: HashMap<String, u128>,
    pub asset_owners: HashMap<u64, String>,
    pub fee_collector: String,
    pub royalty_collector: String,
}

impl DefaultSettlementAdapter {
    pub fn new(fee_collector: impl Into<String>, royalty_collector: impl Into<String>) -> Self {
        Self {
            balances: HashMap::new(),
            asset_owners: HashMap::new(),
            fee_collector: fee_collector.into(),
            royalty_collector: royalty_collector.into(),
        }
    }

    pub fn set_balance(&mut self, account: &str, amount: u128) {
        self.balances.insert(account.to_string(), amount);
    }

    pub fn get_balance(&self, account: &str) -> u128 {
        *self.balances.get(account).unwrap_or(&0)
    }

    pub fn set_asset_owner(&mut self, asset_id: u64, owner: &str) {
        self.asset_owners.insert(asset_id, owner.to_string());
    }

    pub fn get_asset_owner(&self, asset_id: u64) -> Option<&String> {
        self.asset_owners.get(&asset_id)
    }
}

impl SettlementAdapter for DefaultSettlementAdapter {
    fn validate_order(&self, order: &Order, current_height: u64) -> Result<(), SettlementError> {
        order.validate_execution(current_height)?;
        let owner = self.asset_owners.get(&order.asset_id).ok_or(SettlementError::AssetNotFound)?;
        if owner != &order.seller {
            return Err(SettlementError::UnauthorizedOwnership);
        }
        Ok(())
    }

    fn split_fees(&self, amount: u128, royalty_bps: u16, fee_bps: u16) -> Result<FeeSplit, SettlementError> {
        if royalty_bps as u32 + fee_bps as u32 > 10000 {
            return Err(SettlementError::InvalidFeeRate);
        }

        let royalty_amount = amount.checked_mul(royalty_bps as u128).ok_or(SettlementError::ArithmeticOverflow)? / 10000;
        let protocol_fee_amount = amount.checked_mul(fee_bps as u128).ok_or(SettlementError::ArithmeticOverflow)? / 10000;

        let total_deductions = royalty_amount.checked_add(protocol_fee_amount).ok_or(SettlementError::ArithmeticOverflow)?;
        let net_seller_amount = amount.checked_sub(total_deductions).ok_or(SettlementError::ArithmeticOverflow)?;

        Ok(FeeSplit {
            net_seller_amount,
            royalty_amount,
            protocol_fee_amount,
        })
    }

    fn execute_payment(&mut self, buyer: &str, seller: &str, amount: u128) -> Result<(), SettlementError> {
        let buyer_bal = self.get_balance(buyer);
        if buyer_bal < amount {
            return Err(SettlementError::InsufficientBalance);
        }
        self.balances.insert(buyer.to_string(), buyer_bal - amount);

        let seller_bal = self.get_balance(seller);
        let new_seller_bal = seller_bal.checked_add(amount).ok_or(SettlementError::ArithmeticOverflow)?;
        self.balances.insert(seller.to_string(), new_seller_bal);

        Ok(())
    }

    fn transfer_ownership(&mut self, asset_id: u64, from: &str, to: &str) -> Result<(), SettlementError> {
        let current_owner = self.asset_owners.get(&asset_id).ok_or(SettlementError::AssetNotFound)?;
        if current_owner != from {
            return Err(SettlementError::UnauthorizedOwnership);
        }
        self.asset_owners.insert(asset_id, to.to_string());
        Ok(())
    }
}

/// Settlement Engine orchestrating atomic trade executions.
pub struct SettlementEngine<A: SettlementAdapter> {
    pub adapter: A,
    pub nonce_guard: OrderNonceGuard,
    pub events: Vec<SettlementEvent>,
}

impl<A: SettlementAdapter> SettlementEngine<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            adapter,
            nonce_guard: OrderNonceGuard::new(),
            events: Vec::new(),
        }
    }

    pub fn settle_order(
        &mut self,
        order: &mut Order,
        buyer: &str,
        royalty_bps: u16,
        fee_bps: u16,
        current_height: u64,
    ) -> Result<SettlementEvent, SettlementError> {
        self.adapter.validate_order(order, current_height)?;
        self.nonce_guard.consume_nonce(&order.seller, order.nonce)?;

        let fee_split = self.adapter.split_fees(order.price, royalty_bps, fee_bps)?;

        self.adapter.execute_payment(buyer, &order.seller, fee_split.net_seller_amount)?;
        if fee_split.royalty_amount > 0 {
            self.adapter.execute_payment(buyer, "royalty_vault", fee_split.royalty_amount)?;
        }
        if fee_split.protocol_fee_amount > 0 {
            self.adapter.execute_payment(buyer, "protocol_fee_vault", fee_split.protocol_fee_amount)?;
        }

        self.adapter.transfer_ownership(order.asset_id, &order.seller, buyer)?;

        order.status = OrderStatus::Filled;

        let event = SettlementEvent {
            order_id: order.order_id,
            buyer: buyer.to_string(),
            seller: order.seller.clone(),
            asset_id: order.asset_id,
            total_price: order.price,
            net_to_seller: fee_split.net_seller_amount,
            royalty: fee_split.royalty_amount,
            protocol_fee: fee_split.protocol_fee_amount,
        };
        self.events.push(event.clone());

        Ok(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::order::FillPolicy;

    #[test]
    fn test_fee_splitting() {
        let adapter = DefaultSettlementAdapter::new("treasury", "creator");
        let split = adapter.split_fees(10_000, 250, 100).unwrap(); // 2.5% royalty, 1.0% fee
        assert_eq!(split.royalty_amount, 250);
        assert_eq!(split.protocol_fee_amount, 100);
        assert_eq!(split.net_seller_amount, 9650);
    }

    #[test]
    fn test_fee_splitting_invalid_rate() {
        let adapter = DefaultSettlementAdapter::new("treasury", "creator");
        assert_eq!(adapter.split_fees(10_000, 6000, 5000), Err(SettlementError::InvalidFeeRate));
    }

    #[test]
    fn test_successful_settlement() {
        let mut adapter = DefaultSettlementAdapter::new("treasury", "creator");
        adapter.set_balance("bob", 10_000);
        adapter.set_asset_owner(500, "alice");

        let mut engine = SettlementEngine::new(adapter);

        let mut order = Order::new(10, 500, "alice", 5000, 100, 1, vec![0x11], FillPolicy::FullOnly, 10).unwrap();

        let event = engine.settle_order(&mut order, "bob", 200, 100, 15).unwrap();

        assert_eq!(event.net_to_seller, 4850);
        assert_eq!(event.royalty, 100);
        assert_eq!(event.protocol_fee, 50);
        assert_eq!(order.status, OrderStatus::Filled);
        assert_eq!(engine.adapter.get_asset_owner(500), Some(&"bob".to_string()));
        assert_eq!(engine.adapter.get_balance("bob"), 5000);
        assert_eq!(engine.adapter.get_balance("alice"), 4850);
    }

    #[test]
    fn test_settlement_insufficient_buyer_balance() {
        let mut adapter = DefaultSettlementAdapter::new("treasury", "creator");
        adapter.set_balance("bob", 100);
        adapter.set_asset_owner(500, "alice");

        let mut engine = SettlementEngine::new(adapter);
        let mut order = Order::new(10, 500, "alice", 5000, 100, 1, vec![0x11], FillPolicy::FullOnly, 10).unwrap();

        assert_eq!(
            engine.settle_order(&mut order, "bob", 200, 100, 15),
            Err(SettlementError::InsufficientBalance)
        );
    }

    #[test]
    fn test_settlement_unauthorized_asset_owner() {
        let mut adapter = DefaultSettlementAdapter::new("treasury", "creator");
        adapter.set_balance("bob", 10_000);
        adapter.set_asset_owner(500, "charlie"); // owned by charlie, but order claims alice

        let mut engine = SettlementEngine::new(adapter);
        let mut order = Order::new(10, 500, "alice", 5000, 100, 1, vec![0x11], FillPolicy::FullOnly, 10).unwrap();

        assert_eq!(
            engine.settle_order(&mut order, "bob", 200, 100, 15),
            Err(SettlementError::UnauthorizedOwnership)
        );
    }

    #[test]
    fn test_settlement_duplicate_nonce_prevention() {
        let mut adapter = DefaultSettlementAdapter::new("treasury", "creator");
        adapter.set_balance("bob", 20_000);
        adapter.set_asset_owner(500, "alice");
        adapter.set_asset_owner(501, "alice");

        let mut engine = SettlementEngine::new(adapter);

        let mut order1 = Order::new(10, 500, "alice", 5000, 100, 1, vec![0x11], FillPolicy::FullOnly, 10).unwrap();
        let mut order2 = Order::new(11, 501, "alice", 5000, 100, 1, vec![0x22], FillPolicy::FullOnly, 10).unwrap(); // Same nonce (1)

        assert!(engine.settle_order(&mut order1, "bob", 100, 100, 15).is_ok());
        assert_eq!(
            engine.settle_order(&mut order2, "bob", 100, 100, 15),
            Err(SettlementError::OrderError(OrderError::NonceAlreadyUsed))
        );
    }
}
