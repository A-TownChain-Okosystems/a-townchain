// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Deterministic A-TownChain L1 ledger state transitions.
//!
//! This crate owns ledger semantics only. Persistent storage, consensus,
//! transaction signing, and state-root hashing remain separate concerns.

use std::collections::BTreeMap;

pub type Address = String;
pub type Amount = u128;
pub type Nonce = u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub balance: Amount,
    pub nonce: Nonce,
}

impl Default for Account {
    fn default() -> Self {
        Self { balance: 0, nonce: 0 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transfer {
    pub from: Address,
    pub to: Address,
    pub amount: Amount,
    pub fee: Amount,
    pub nonce: Nonce,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEntry {
    pub sequence: u64,
    pub from: Address,
    pub to: Address,
    pub amount: Amount,
    pub fee: Amount,
    pub nonce: Nonce,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerError {
    EmptyAddress,
    InsufficientBalance,
    InvalidNonce { expected: Nonce, actual: Nonce },
    AmountOverflow,
    NonceOverflow,
    SequenceOverflow,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Ledger {
    accounts: BTreeMap<Address, Account>,
    journal: Vec<LedgerEntry>,
    next_sequence: u64,
}

impl Ledger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn account(&self, address: &str) -> Option<&Account> {
        self.accounts.get(address)
    }

    pub fn balance_of(&self, address: &str) -> Amount {
        self.accounts.get(address).map_or(0, |a| a.balance)
    }

    pub fn nonce_of(&self, address: &str) -> Nonce {
        self.accounts.get(address).map_or(0, |a| a.nonce)
    }

    pub fn journal(&self) -> &[LedgerEntry] {
        &self.journal
    }

    pub fn apply_transfer(&mut self, tx: Transfer) -> Result<(), LedgerError> {
        if tx.from.is_empty() || tx.to.is_empty() {
            return Err(LedgerError::EmptyAddress);
        }

        let debit = tx
            .amount
            .checked_add(tx.fee)
            .ok_or(LedgerError::AmountOverflow)?;

        let sender = self.accounts.get(&tx.from).cloned().unwrap_or_default();
        if tx.nonce != sender.nonce {
            return Err(LedgerError::InvalidNonce {
                expected: sender.nonce,
                actual: tx.nonce,
            });
        }
        if sender.balance < debit {
            return Err(LedgerError::InsufficientBalance);
        }

        let recipient = self.accounts.get(&tx.to).cloned().unwrap_or_default();
        let sender_nonce = sender
            .nonce
            .checked_add(1)
            .ok_or(LedgerError::NonceOverflow)?;
        let sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(LedgerError::SequenceOverflow)?;

        let (sender_balance, recipient_balance) = if tx.from == tx.to {
            // A self-transfer returns its amount to the same account; only
            // the fee changes the balance, while the nonce advances.
            (
                sender
                    .balance
                    .checked_sub(tx.fee)
                    .ok_or(LedgerError::InsufficientBalance)?,
                sender
                    .balance
                    .checked_sub(tx.fee)
                    .ok_or(LedgerError::InsufficientBalance)?,
            )
        } else {
            let recipient_balance = recipient
                .balance
                .checked_add(tx.amount)
                .ok_or(LedgerError::AmountOverflow)?;
            let sender_balance = sender
                .balance
                .checked_sub(debit)
                .ok_or(LedgerError::InsufficientBalance)?;
            (sender_balance, recipient_balance)
        };

        self.accounts.insert(
            tx.from.clone(),
            Account {
                balance: sender_balance,
                nonce: sender_nonce,
            },
        );
        if tx.from != tx.to {
            self.accounts.insert(
                tx.to.clone(),
                Account {
                    balance: recipient_balance,
                    nonce: recipient.nonce,
                },
            );
        }
        self.next_sequence = sequence;
        self.journal.push(LedgerEntry {
            sequence,
            from: tx.from,
            to: tx.to,
            amount: tx.amount,
            fee: tx.fee,
            nonce: tx.nonce,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn funded_ledger() -> Ledger {
        let mut ledger = Ledger::new();
        ledger.accounts.insert(
            "alice".into(),
            Account {
                balance: 100,
                nonce: 0,
            },
        );
        ledger
    }

    #[test]
    fn transfer_updates_balances_and_nonce() {
        let mut ledger = funded_ledger();
        ledger
            .apply_transfer(Transfer {
                from: "alice".into(),
                to: "bob".into(),
                amount: 30,
                fee: 2,
                nonce: 0,
            })
            .unwrap();

        assert_eq!(ledger.balance_of("alice"), 68);
        assert_eq!(ledger.balance_of("bob"), 30);
        assert_eq!(ledger.nonce_of("alice"), 1);
        assert_eq!(ledger.journal()[0].sequence, 1);
    }

    #[test]
    fn self_transfer_charges_only_fee() {
        let mut ledger = funded_ledger();
        ledger
            .apply_transfer(Transfer {
                from: "alice".into(),
                to: "alice".into(),
                amount: 30,
                fee: 2,
                nonce: 0,
            })
            .unwrap();

        assert_eq!(ledger.balance_of("alice"), 98);
        assert_eq!(ledger.nonce_of("alice"), 1);
    }

    #[test]
    fn insufficient_balance_is_atomic() {
        let mut ledger = funded_ledger();
        let before = ledger.clone();

        let result = ledger.apply_transfer(Transfer {
            from: "alice".into(),
            to: "bob".into(),
            amount: 99,
            fee: 2,
            nonce: 0,
        });

        assert_eq!(result, Err(LedgerError::InsufficientBalance));
        assert_eq!(ledger, before);
    }

    #[test]
    fn invalid_nonce_is_atomic() {
        let mut ledger = funded_ledger();
        let before = ledger.clone();

        let result = ledger.apply_transfer(Transfer {
            from: "alice".into(),
            to: "bob".into(),
            amount: 1,
            fee: 0,
            nonce: 1,
        });

        assert_eq!(
            result,
            Err(LedgerError::InvalidNonce {
                expected: 0,
                actual: 1
            })
        );
        assert_eq!(ledger, before);
    }

    #[test]
    fn u128_max_amount_is_checked() {
        let mut ledger = Ledger::new();
        ledger.accounts.insert(
            "alice".into(),
            Account {
                balance: u128::MAX,
                nonce: 0,
            },
        );

        ledger
            .apply_transfer(Transfer {
                from: "alice".into(),
                to: "bob".into(),
                amount: u128::MAX,
                fee: 0,
                nonce: 0,
            })
            .unwrap();

        assert_eq!(ledger.balance_of("alice"), 0);
        assert_eq!(ledger.balance_of("bob"), u128::MAX);
    }

    #[test]
    fn amount_plus_fee_overflow_is_rejected_without_mutation() {
        let mut ledger = funded_ledger();
        let before = ledger.clone();

        let result = ledger.apply_transfer(Transfer {
            from: "alice".into(),
            to: "bob".into(),
            amount: u128::MAX,
            fee: 1,
            nonce: 0,
        });

        assert_eq!(result, Err(LedgerError::AmountOverflow));
        assert_eq!(ledger, before);
    }

    #[test]
    fn deterministic_order_is_preserved() {
        let mut ledger = funded_ledger();
        ledger.accounts.insert(
            "carol".into(),
            Account {
                balance: 0,
                nonce: 0,
            },
        );

        ledger
            .apply_transfer(Transfer {
                from: "alice".into(),
                to: "bob".into(),
                amount: 10,
                fee: 1,
                nonce: 0,
            })
            .unwrap();
        ledger
            .apply_transfer(Transfer {
                from: "alice".into(),
                to: "carol".into(),
                amount: 5,
                fee: 1,
                nonce: 1,
            })
            .unwrap();

        assert_eq!(
            ledger.journal().iter().map(|e| e.sequence).collect::<Vec<_>>(),
            vec![1, 2]
        );
    }
}
