//! GlobusOS application SDK facade.
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WalletHandle {
    pub address: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityHandle {
    pub id: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServiceHandle {
    pub name: String,
}
pub trait IdentityProvider {
    fn identity(&self) -> IdentityHandle;
}
pub trait WalletProvider {
    fn wallet(&self) -> WalletHandle;
}
pub trait ServiceProvider {
    fn service(&self, name: &str) -> Option<ServiceHandle>;
}
