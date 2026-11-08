//! GlobusOS identity and wallet boundary.

mod account_lifecycle;
mod account_registry;
mod file_store;
mod ipc_key_provider;
mod key_provider;
mod keystore;
mod multi_account;
mod persistence;
mod secure_store;
mod service;
pub mod transaction_journal;

pub use account_lifecycle::{AccountLifecycleError, AccountUsername, RecoveryConfirmation};
pub use account_registry::{
    AccountDirectoryEntry, AccountDirectoryError, AccountDirectoryStore, FileAccountDirectory,
};
use bip39::{Language, Mnemonic};
pub use file_store::FileBlobStore;
use hmac::{Hmac, Mac};
pub use ipc_key_provider::{IdentityKeyIpcTransport, IpcIdentityKeyService};
pub use key_provider::{
    SecureKeyId, SecureKeyService, SecureKeyServiceError, ShivaCoreKeyProvider,
};
pub use keystore::{
    KeyId, KeyMetadata, KeyStore, KeystoreError, ProtectedKey, SignRequest, Signature,
};
pub use multi_account::{MultiAccountError, MultiAccountIdentityService};
pub use persistence::{
    CredentialRecord, IDENTITY_RECORD_VERSION, IdentityRecord, IdentityStore,
    InMemoryIdentityStore, PersistenceError,
};
pub use secure_store::{
    BlobStore, EncryptedBlobStore, IdentityKeyProvider, InMemoryBlobStore, SecureStoreError,
};
pub use service::{IdentityService, IdentityServiceError, LoginResult, RegistrationResult};
use sha2::{Digest, Sha256};
use zeroize::{Zeroize, Zeroizing};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(String);
impl UserId {
    pub fn new(value: impl Into<String>) -> Result<Self, IdentityError> {
        let value = value.into();
        if value.is_empty() || value.len() > 128 {
            return Err(IdentityError::InvalidUserId);
        }
        Ok(Self(value))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletAddress(String);
impl WalletAddress {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityBinding {
    pub user_id: UserId,
    pub wallet_address: WalletAddress,
    pub chain_id: u64,
    pub network: String,
    pub key_version: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountProfile {
    pub user_id: UserId,
    pub display_name: String,
    pub wallet_address: WalletAddress,
    pub binding: IdentityBinding,
}
impl AccountProfile {
    pub fn new(
        user_id: UserId,
        display_name: impl Into<String>,
        binding: IdentityBinding,
    ) -> Result<Self, IdentityError> {
        let display_name = display_name.into();
        if display_name.is_empty() || display_name.len() > 128 {
            return Err(IdentityError::InvalidDisplayName);
        }
        if binding.user_id != user_id {
            return Err(IdentityError::BindingMismatch);
        }
        Ok(Self {
            user_id,
            display_name,
            wallet_address: binding.wallet_address.clone(),
            binding,
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginState {
    LoggedOut,
    Authenticating,
    Authenticated,
    Locked,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentitySession {
    pub user_id: UserId,
    pub wallet_address: WalletAddress,
    pub state: LoginState,
    pub issued_at_unix: u64,
    pub expires_at_unix: u64,
}
impl IdentitySession {
    pub fn is_active(&self, now_unix: u64) -> bool {
        self.state == LoginState::Authenticated && now_unix < self.expires_at_unix
    }
    pub fn lock(&mut self) {
        self.state = LoginState::Locked;
    }
}
#[derive(Debug)]
pub struct RecoveryMaterial {
    seed: Zeroizing<Vec<u8>>,
}
impl RecoveryMaterial {
    pub fn from_seed(seed: Vec<u8>) -> Result<Self, IdentityError> {
        if seed.len() < 32 {
            return Err(IdentityError::WeakSeed);
        }
        Ok(Self {
            seed: Zeroizing::new(seed),
        })
    }
    pub fn seed_len(&self) -> usize {
        self.seed.len()
    }
}
impl Drop for RecoveryMaterial {
    fn drop(&mut self) {
        self.seed.zeroize();
    }
}
pub struct RecoveryPhrase(Zeroizing<String>);
impl RecoveryPhrase {
    pub fn expose(&self) -> &str {
        &self.0
    }
    pub fn word_count(&self) -> usize {
        self.0.split_whitespace().count()
    }
}
pub struct WalletCreation {
    pub recovery_phrase: RecoveryPhrase,
    pub wallet_address: WalletAddress,
    pub binding: IdentityBinding,
}
impl WalletCreation {
    pub fn recovery_phrase(&self) -> &str {
        self.recovery_phrase.expose()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityError {
    InvalidUserId,
    InvalidDisplayName,
    BindingMismatch,
    WeakSeed,
    InvalidWalletAddress,
    EmptyNetwork,
    InvalidMnemonic,
    WalletDerivationFailed,
}
pub fn create_wallet(
    user_id: UserId,
    chain_id: u64,
    network: impl Into<String>,
    key_version: u16,
) -> Result<WalletCreation, IdentityError> {
    let network = network.into();
    if network.is_empty() {
        return Err(IdentityError::EmptyNetwork);
    }
    let mnemonic = Mnemonic::generate_in(Language::English, 24)
        .map_err(|_| IdentityError::WalletDerivationFailed)?;
    let phrase = mnemonic.to_string();
    let seed = mnemonic.to_seed_normalized("");
    let private_key = derive_private_key(&seed)?;
    let public_key = derive_public_key(&private_key);
    let wallet_address = public_key_to_address(&public_key)?;
    let binding = IdentityBinding {
        user_id,
        wallet_address: wallet_address.clone(),
        chain_id,
        network,
        key_version,
    };
    Ok(WalletCreation {
        recovery_phrase: RecoveryPhrase(Zeroizing::new(phrase)),
        wallet_address,
        binding,
    })
}
pub fn restore_wallet(
    user_id: UserId,
    phrase: &str,
    chain_id: u64,
    network: impl Into<String>,
    key_version: u16,
) -> Result<IdentityBinding, IdentityError> {
    let network = network.into();
    if network.is_empty() {
        return Err(IdentityError::EmptyNetwork);
    }
    let mnemonic = Mnemonic::parse_in_normalized(Language::English, phrase)
        .map_err(|_| IdentityError::InvalidMnemonic)?;
    if mnemonic.word_count() != 24 {
        return Err(IdentityError::InvalidMnemonic);
    }
    let seed = mnemonic.to_seed_normalized("");
    let private_key = derive_private_key(&seed)?;
    let public_key = derive_public_key(&private_key);
    let wallet_address = public_key_to_address(&public_key)?;
    Ok(IdentityBinding {
        user_id,
        wallet_address,
        chain_id,
        network,
        key_version,
    })
}
fn derive_private_key(seed: &[u8]) -> Result<[u8; 32], IdentityError> {
    let mut mac = HmacSha256::new_from_slice(b"A-TownChain")
        .map_err(|_| IdentityError::WalletDerivationFailed)?;
    mac.update(seed);
    let output = mac.finalize().into_bytes();
    let mut key = [0u8; 32];
    key.copy_from_slice(&output);
    Ok(key)
}
fn derive_public_key(private_key: &[u8; 32]) -> [u8; 32] {
    Sha256::digest(private_key).into()
}
fn public_key_to_address(public_key: &[u8; 32]) -> Result<WalletAddress, IdentityError> {
    let step1 = Sha256::digest(public_key);
    let step2 = Sha256::digest(step1);
    let checksum = Sha256::digest(step2);
    let address = format!(
        "ATC{}{}",
        hex_upper(&step2[..14]),
        hex_upper(&checksum[..2])
    );
    if address.len() != 35 {
        return Err(IdentityError::InvalidWalletAddress);
    }
    Ok(WalletAddress(address))
}
fn hex_upper(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut output = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identity_rejects_empty_user_id() {
        assert_eq!(UserId::new("").unwrap_err(), IdentityError::InvalidUserId)
    }
    #[test]
    fn generated_wallet_has_24_words_and_atc_address() {
        let user = UserId::new("test-user").unwrap();
        let wallet = create_wallet(user, 600, "devnet", 1).unwrap();
        assert_eq!(wallet.recovery_phrase.word_count(), 24);
        assert_eq!(wallet.wallet_address.as_str().len(), 35);
        assert!(wallet.wallet_address.as_str().starts_with("ATC"))
    }
    #[test]
    fn wallet_restore_reproduces_address() {
        let user = UserId::new("test-user").unwrap();
        let wallet = create_wallet(user.clone(), 600, "devnet", 1).unwrap();
        let restored = restore_wallet(user, wallet.recovery_phrase(), 600, "devnet", 1).unwrap();
        assert_eq!(restored.wallet_address, wallet.wallet_address)
    }
    #[test]
    fn account_binding_is_checked() {
        let user = UserId::new("test-user").unwrap();
        let wallet = create_wallet(user.clone(), 600, "devnet", 1).unwrap();
        let profile = AccountProfile::new(user, "Test User", wallet.binding).unwrap();
        assert_eq!(profile.wallet_address, wallet.wallet_address)
    }
    #[test]
    fn session_expires_and_locks() {
        let user = UserId::new("test-user").unwrap();
        let wallet = create_wallet(user.clone(), 600, "devnet", 1).unwrap();
        let mut session = IdentitySession {
            user_id: user,
            wallet_address: wallet.wallet_address,
            state: LoginState::Authenticated,
            issued_at_unix: 100,
            expires_at_unix: 200,
        };
        assert!(session.is_active(199));
        assert!(!session.is_active(200));
        session.lock();
        assert!(!session.is_active(150))
    }
    #[test]
    fn weak_recovery_material_is_rejected() {
        assert_eq!(
            RecoveryMaterial::from_seed(vec![0; 16]).unwrap_err(),
            IdentityError::WeakSeed
        )
    }
}
