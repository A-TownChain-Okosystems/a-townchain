//! Stateful identity-service orchestration.
//!
//! Credential verification uses Argon2id with a unique random salt. The service keeps
//! only the PHC password hash in memory; plaintext credentials are never retained.
//! Recovery phrases remain outside the persistent profile/session model.

use super::{
    AccountProfile, CredentialRecord, IDENTITY_RECORD_VERSION, IdentityBinding, IdentityError,
    IdentityRecord, IdentitySession, IdentityStore, LoginState, PersistenceError, UserId,
    WalletAddress, WalletCreation, create_wallet, restore_wallet,
};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityServiceError {
    Identity(IdentityError),
    Persistence(PersistenceError),
    AlreadyRegistered,
    NotRegistered,
    InvalidCredentials,
    CredentialHashFailed,
}

impl From<IdentityError> for IdentityServiceError {
    fn from(value: IdentityError) -> Self {
        Self::Identity(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrationResult {
    pub profile: AccountProfile,
    pub wallet_address: WalletAddress,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginResult {
    pub session: IdentitySession,
}

pub struct IdentityService {
    profile: Option<AccountProfile>,
    credential_hash: Option<String>,
    session: Option<IdentitySession>,
}

impl IdentityService {
    pub const fn new() -> Self {
        Self {
            profile: None,
            credential_hash: None,
            session: None,
        }
    }

    pub fn register(
        &mut self,
        user_id: UserId,
        display_name: impl Into<String>,
        credential: &[u8],
        chain_id: u64,
        network: impl Into<String>,
        key_version: u16,
    ) -> Result<(RegistrationResult, WalletCreation), IdentityServiceError> {
        if self.profile.is_some() {
            return Err(IdentityServiceError::AlreadyRegistered);
        }
        let credential_hash = hash_credential(credential)?;
        let wallet = create_wallet(user_id, chain_id, network, key_version)?;
        let profile = AccountProfile::new(
            wallet.binding.user_id.clone(),
            display_name,
            wallet.binding.clone(),
        )?;
        let result = RegistrationResult {
            wallet_address: profile.wallet_address.clone(),
            profile: profile.clone(),
        };
        self.credential_hash = Some(credential_hash);
        self.profile = Some(profile);
        self.session = None;
        Ok((result, wallet))
    }

    // Restore inherently carries the full recovery request; grouping the 8 fields
    // into a params struct is tracked design debt.
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
    ) -> Result<RegistrationResult, IdentityServiceError> {
        if self.profile.is_some() {
            return Err(IdentityServiceError::AlreadyRegistered);
        }
        let credential_hash = hash_credential(credential)?;
        let binding = restore_wallet(user_id, phrase, chain_id, network, key_version)?;
        let profile = AccountProfile::new(binding.user_id.clone(), display_name, binding)?;
        let result = RegistrationResult {
            wallet_address: profile.wallet_address.clone(),
            profile: profile.clone(),
        };
        self.credential_hash = Some(credential_hash);
        self.profile = Some(profile);
        self.session = None;
        Ok(result)
    }

    /// Export only durable identity state. Active sessions are intentionally excluded.
    pub fn snapshot(&self) -> Result<IdentityRecord, IdentityServiceError> {
        let profile = self
            .profile
            .clone()
            .ok_or(IdentityServiceError::NotRegistered)?;
        let hash = self
            .credential_hash
            .clone()
            .ok_or(IdentityServiceError::InvalidCredentials)?;
        Ok(IdentityRecord::new(
            profile,
            CredentialRecord::argon2id(hash),
        ))
    }

    /// Persist the current identity. The store validates before committing.
    pub fn persist<S: IdentityStore>(&self, store: &mut S) -> Result<(), IdentityServiceError> {
        store
            .save(&self.snapshot()?)
            .map_err(IdentityServiceError::Persistence)
    }

    /// Load the durable identity and invalidate any pre-existing session.
    pub fn load_from_store<S: IdentityStore>(
        &mut self,
        store: &S,
    ) -> Result<(), IdentityServiceError> {
        let record = store
            .load()
            .map_err(IdentityServiceError::Persistence)?
            .ok_or(IdentityServiceError::Persistence(
                PersistenceError::NotFound,
            ))?;
        self.load_record(record)
    }

    /// Export and validate durable identity state without persisting a session.
    pub fn load_record(&mut self, record: IdentityRecord) -> Result<(), IdentityServiceError> {
        record
            .validate()
            .map_err(IdentityServiceError::Persistence)?;
        if record.version != IDENTITY_RECORD_VERSION {
            return Err(IdentityServiceError::Persistence(
                PersistenceError::UnsupportedVersion(record.version),
            ));
        }
        self.profile = Some(record.profile);
        self.credential_hash = Some(record.credential.phc_hash);
        self.session = None;
        Ok(())
    }

    pub fn login(
        &mut self,
        credential: &[u8],
        now_unix: u64,
        ttl_seconds: u64,
    ) -> Result<LoginResult, IdentityServiceError> {
        let profile = self
            .profile
            .as_ref()
            .ok_or(IdentityServiceError::NotRegistered)?;
        let stored = self
            .credential_hash
            .as_deref()
            .ok_or(IdentityServiceError::InvalidCredentials)?;
        verify_credential(stored, credential)?;
        let session = IdentitySession {
            user_id: profile.user_id.clone(),
            wallet_address: profile.wallet_address.clone(),
            state: LoginState::Authenticated,
            issued_at_unix: now_unix,
            expires_at_unix: now_unix.saturating_add(ttl_seconds),
        };
        self.session = Some(session.clone());
        Ok(LoginResult { session })
    }

    pub fn logout(&mut self) {
        self.session = None;
    }
    pub fn lock(&mut self) {
        if let Some(session) = &mut self.session {
            session.lock();
        }
    }
    pub fn session(&self, now_unix: u64) -> Option<&IdentitySession> {
        self.session.as_ref().filter(|s| s.is_active(now_unix))
    }
    pub fn profile(&self) -> Option<&AccountProfile> {
        self.profile.as_ref()
    }
    pub fn binding(&self) -> Option<&IdentityBinding> {
        self.profile.as_ref().map(|p| &p.binding)
    }
}

fn hash_credential(credential: &[u8]) -> Result<String, IdentityServiceError> {
    if credential.is_empty() {
        return Err(IdentityServiceError::InvalidCredentials);
    }
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(credential, &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| IdentityServiceError::CredentialHashFailed)
}

fn verify_credential(stored: &str, credential: &[u8]) -> Result<(), IdentityServiceError> {
    if credential.is_empty() {
        return Err(IdentityServiceError::InvalidCredentials);
    }
    let parsed = PasswordHash::new(stored).map_err(|_| IdentityServiceError::InvalidCredentials)?;
    Argon2::default()
        .verify_password(credential, &parsed)
        .map_err(|_| IdentityServiceError::InvalidCredentials)
}

impl Default for IdentityService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryIdentityStore;

    #[test]
    fn register_login_lock_logout_flow() {
        let user = UserId::new("alice").unwrap();
        let mut service = IdentityService::new();
        let (registered, wallet) = service
            .register(user, "Alice", b"local-credential", 600, "devnet", 1)
            .unwrap();
        assert_eq!(registered.wallet_address, wallet.wallet_address);
        let login = service.login(b"local-credential", 100, 300).unwrap();
        assert_eq!(login.session.state, LoginState::Authenticated);
        assert!(service.session(200).is_some());
        service.lock();
        assert!(service.session(200).is_none());
        service.logout();
        assert!(service.session(200).is_none());
    }

    #[test]
    fn wrong_credential_is_denied() {
        let user = UserId::new("alice").unwrap();
        let mut service = IdentityService::new();
        service
            .register(user, "Alice", b"correct", 600, "devnet", 1)
            .unwrap();
        assert_eq!(
            service.login(b"wrong", 100, 300).unwrap_err(),
            IdentityServiceError::InvalidCredentials
        );
    }

    #[test]
    fn credential_hash_uses_unique_salts() {
        let first = hash_credential(b"same-secret").unwrap();
        let second = hash_credential(b"same-secret").unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn recovery_restores_wallet_identity() {
        let user = UserId::new("alice").unwrap();
        let mut source = IdentityService::new();
        let (_, wallet) = source
            .register(user.clone(), "Alice", b"credential", 600, "devnet", 1)
            .unwrap();
        let phrase = wallet.recovery_phrase().to_owned();
        let mut restored = IdentityService::new();
        let result = restored
            .restore(user, "Alice", &phrase, b"credential", 600, "devnet", 1)
            .unwrap();
        assert_eq!(result.wallet_address, wallet.wallet_address);
    }

    #[test]
    fn persistence_round_trip_invalidates_session() {
        let user = UserId::new("alice").unwrap();
        let mut source = IdentityService::new();
        source
            .register(user, "Alice", b"credential", 600, "devnet", 1)
            .unwrap();
        source.login(b"credential", 100, 300).unwrap();
        let mut store = InMemoryIdentityStore::new();
        source.persist(&mut store).unwrap();
        let mut restored = IdentityService::new();
        restored.load_from_store(&store).unwrap();
        assert!(restored.session(150).is_none());
        assert!(restored.login(b"credential", 200, 300).is_ok());
    }

    #[test]
    fn malformed_record_is_rejected() {
        let user = UserId::new("alice").unwrap();
        let mut source = IdentityService::new();
        source
            .register(user, "Alice", b"credential", 600, "devnet", 1)
            .unwrap();
        let mut record = source.snapshot().unwrap();
        record.credential.scheme = "sha256";
        let mut restored = IdentityService::new();
        assert_eq!(
            restored.load_record(record).unwrap_err(),
            IdentityServiceError::Persistence(PersistenceError::UnsupportedCredentialScheme)
        );
    }
}
