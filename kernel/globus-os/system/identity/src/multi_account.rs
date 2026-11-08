//! In-process coordinator for multiple independently authenticated identity accounts.
//!
//! This is an orchestration layer, not a durable account database. Callers must persist each
//! account through a separate durable IdentityStore and coordinate username-directory changes
//! with a journal/reconciliation protocol before claiming crash-consistent registration.
//! Wallet derivation remains delegated to the existing identity boundary and is not evidence of
//! conformance to the draft canonical atc-wallet BIP44 specification.

use std::collections::HashMap;

use super::{
    IdentityService, IdentityServiceError, IdentitySession, LoginResult, RegistrationResult,
    UserId, WalletCreation,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiAccountError {
    DuplicateAccount,
    AccountNotFound,
    Identity(IdentityServiceError),
}

impl From<IdentityServiceError> for MultiAccountError {
    fn from(value: IdentityServiceError) -> Self {
        Self::Identity(value)
    }
}

/// Keeps authentication state isolated by account identifier.
///
/// Each map entry owns its own credential hash and session. This coordinator is intentionally
/// in-memory: restart durability and atomic coordination with the username directory are separate
/// requirements and must not be inferred from this type.
#[derive(Default)]
pub struct MultiAccountIdentityService {
    accounts: HashMap<String, IdentityService>,
}

impl MultiAccountIdentityService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn account_count(&self) -> usize {
        self.accounts.len()
    }

    pub fn register(
        &mut self,
        user_id: UserId,
        display_name: impl Into<String>,
        credential: &[u8],
        chain_id: u64,
        network: impl Into<String>,
        key_version: u16,
    ) -> Result<(RegistrationResult, WalletCreation), MultiAccountError> {
        let key = user_id.as_str().to_owned();
        if self.accounts.contains_key(&key) {
            return Err(MultiAccountError::DuplicateAccount);
        }

        let mut account = IdentityService::new();
        let result = account.register(
            user_id,
            display_name,
            credential,
            chain_id,
            network,
            key_version,
        )?;
        // Publish the account only after registration succeeds.
        self.accounts.insert(key, account);
        Ok(result)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn restore(
        &mut self,
        user_id: UserId,
        display_name: impl Into<String>,
        phrase: &str,
        credential: &[u8],
        chain_id: u64,
        network: impl Into<String>,
        key_version: u16,
    ) -> Result<RegistrationResult, MultiAccountError> {
        let key = user_id.as_str().to_owned();
        if self.accounts.contains_key(&key) {
            return Err(MultiAccountError::DuplicateAccount);
        }

        let mut account = IdentityService::new();
        let result = account.restore(
            user_id,
            display_name,
            phrase,
            credential,
            chain_id,
            network,
            key_version,
        )?;
        self.accounts.insert(key, account);
        Ok(result)
    }

    pub fn login(
        &mut self,
        user_id: &UserId,
        credential: &[u8],
        now_unix: u64,
        ttl_seconds: u64,
    ) -> Result<LoginResult, MultiAccountError> {
        self.accounts
            .get_mut(user_id.as_str())
            .ok_or(MultiAccountError::AccountNotFound)?
            .login(credential, now_unix, ttl_seconds)
            .map_err(MultiAccountError::Identity)
    }

    pub fn logout(&mut self, user_id: &UserId) -> Result<(), MultiAccountError> {
        self.accounts
            .get_mut(user_id.as_str())
            .ok_or(MultiAccountError::AccountNotFound)?
            .logout();
        Ok(())
    }

    pub fn session(&self, user_id: &UserId, now_unix: u64) -> Option<&IdentitySession> {
        self.accounts
            .get(user_id.as_str())
            .and_then(|account| account.session(now_unix))
    }

    /// Remove an account from this in-memory coordinator and thereby revoke its local session.
    ///
    /// This does not delete durable identity records or the persistent username index. A caller
    /// must complete those operations transactionally before treating account deletion as durable.
    pub fn remove_account(&mut self, user_id: &UserId) -> Result<(), MultiAccountError> {
        self.accounts
            .remove(user_id.as_str())
            .map(|_| ())
            .ok_or(MultiAccountError::AccountNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accounts_have_independent_credentials_and_sessions() {
        let alice = UserId::new("alice").unwrap();
        let bob = UserId::new("bob").unwrap();
        let mut service = MultiAccountIdentityService::new();

        service
            .register(alice.clone(), "Alice", b"alice-secret", 658467, "devnet", 1)
            .unwrap();
        service
            .register(bob.clone(), "Bob", b"bob-secret", 658467, "devnet", 1)
            .unwrap();

        assert_eq!(service.account_count(), 2);
        assert!(service.login(&alice, b"alice-secret", 100, 60).is_ok());
        assert!(service.login(&bob, b"bob-secret", 100, 60).is_ok());
        assert!(service.login(&alice, b"bob-secret", 100, 60).is_err());

        service.logout(&alice).unwrap();
        assert!(service.session(&alice, 101).is_none());
        assert!(service.session(&bob, 101).is_some());
    }

    #[test]
    fn duplicate_account_is_rejected_without_replacing_existing_account() {
        let alice = UserId::new("alice").unwrap();
        let mut service = MultiAccountIdentityService::new();
        service
            .register(alice.clone(), "Alice", b"alice-secret", 658467, "devnet", 1)
            .unwrap();

        assert_eq!(
            service
                .register(
                    alice.clone(),
                    "Imposter",
                    b"other-secret",
                    658467,
                    "devnet",
                    1
                )
                .err()
                .unwrap(),
            MultiAccountError::DuplicateAccount
        );
        assert!(service.login(&alice, b"alice-secret", 100, 60).is_ok());
        assert_eq!(service.account_count(), 1);
    }

    #[test]
    fn removing_account_revokes_its_local_session() {
        let alice = UserId::new("alice").unwrap();
        let mut service = MultiAccountIdentityService::new();
        service
            .register(alice.clone(), "Alice", b"alice-secret", 658467, "devnet", 1)
            .unwrap();
        service.login(&alice, b"alice-secret", 100, 60).unwrap();

        service.remove_account(&alice).unwrap();
        assert_eq!(service.account_count(), 0);
        assert!(service.session(&alice, 101).is_none());
        assert_eq!(
            service.login(&alice, b"alice-secret", 101, 60).unwrap_err(),
            MultiAccountError::AccountNotFound
        );
    }
}
