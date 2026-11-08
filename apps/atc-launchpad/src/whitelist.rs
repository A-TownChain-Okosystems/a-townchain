// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Whitelist Module — Allowlist-Verwaltung, Tier-Limits & Revokation (LP-WL-001).

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhitelistError {
    EntryNotFound,
    EntryExpired,
    AddressRevoked,
    ExceedsTierLimit,
    InvalidProof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhitelistEntry {
    pub address: u64,
    pub tier: u8,
    pub limit: u128,
    pub expiry_slot: u64,
}

pub struct WhitelistManager {
    entries: HashMap<u64, WhitelistEntry>,
    revoked: HashSet<u64>,
    merkle_root: [u8; 32],
}

impl WhitelistManager {
    pub fn new(merkle_root: [u8; 32]) -> Self {
        Self {
            entries: HashMap::new(),
            revoked: HashSet::new(),
            merkle_root,
        }
    }

    pub fn merkle_root(&self) -> &[u8; 32] {
        &self.merkle_root
    }

    pub fn add_entry(&mut self, entry: WhitelistEntry) {
        self.entries.insert(entry.address, entry);
    }

    pub fn revoke(&mut self, address: u64) {
        self.revoked.insert(address);
    }

    pub fn is_revoked(&self, address: u64) -> bool {
        self.revoked.contains(&address)
    }

    pub fn verify_and_authorize(
        &self,
        address: u64,
        requested_amount: u128,
        current_slot: u64,
    ) -> Result<&WhitelistEntry, WhitelistError> {
        if self.is_revoked(address) {
            return Err(WhitelistError::AddressRevoked);
        }

        let entry = self
            .entries
            .get(&address)
            .ok_or(WhitelistError::EntryNotFound)?;

        if current_slot > entry.expiry_slot {
            return Err(WhitelistError::EntryExpired);
        }

        if requested_amount > entry.limit {
            return Err(WhitelistError::ExceedsTierLimit);
        }

        Ok(entry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitelist_authorization_and_revocation() {
        let root = [0u8; 32];
        let mut wm = WhitelistManager::new(root);
        wm.add_entry(WhitelistEntry {
            address: 101,
            tier: 1,
            limit: 500,
            expiry_slot: 200,
        });

        // Valid authorization
        assert!(wm.verify_and_authorize(101, 300, 150).is_ok());

        // Exceeds tier limit
        assert_eq!(
            wm.verify_and_authorize(101, 600, 150),
            Err(WhitelistError::ExceedsTierLimit)
        );

        // Expired
        assert_eq!(
            wm.verify_and_authorize(101, 100, 250),
            Err(WhitelistError::EntryExpired)
        );

        // Revocation overrides validity
        wm.revoke(101);
        assert_eq!(
            wm.verify_and_authorize(101, 100, 150),
            Err(WhitelistError::AddressRevoked)
        );
    }

    #[test]
    fn test_whitelist_entry_not_found() {
        let root = [0u8; 32];
        let wm = WhitelistManager::new(root);
        assert_eq!(
            wm.verify_and_authorize(999, 10, 100),
            Err(WhitelistError::EntryNotFound)
        );
    }
}
