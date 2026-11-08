//! Capability-aware IPC primitives used by GlobusOS.
//!
//! `no_std` + `alloc`: Dieser Crate wird sowohl vom Kernel (x86_64-unknown-none)
//! als auch von Userspace-Crates genutzt — ausschließlich alloc-Typen.
#![no_std]

extern crate alloc;

pub mod channel;
pub use channel::{ChannelRegistry, IpcError};

use alloc::string::String;
use alloc::vec::Vec;

/// An IPC endpoint identifier representing a communication socket or service target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Endpoint(pub u64);

/// Header metadata attached to every IPC message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageHeader {
    /// Target endpoint for the message.
    pub endpoint: Endpoint,
    /// Operation code identifying the request or response type.
    pub opcode: u32,
    /// Declared byte length of the message payload.
    pub payload_len: u32,
}

/// A complete IPC message consisting of a header and a binary payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// Metadata header for this message.
    pub header: MessageHeader,
    /// Binary body payload of the message.
    pub payload: Vec<u8>,
}

impl Message {
    /// Creates a new IPC message for the target endpoint with the given opcode and payload.
    pub fn new(endpoint: Endpoint, opcode: u32, payload: Vec<u8>) -> Self {
        Self {
            header: MessageHeader {
                endpoint,
                opcode,
                payload_len: payload.len() as u32,
            },
            payload,
        }
    }

    /// Validates that the payload size does not exceed `MAX_IPC_PAYLOAD` and matches `header.payload_len`.
    pub fn validate(&self) -> Result<(), IpcError> {
        if self.payload.len() > MAX_IPC_PAYLOAD
            || self.header.payload_len as usize != self.payload.len()
        {
            return Err(IpcError::PayloadTooLarge);
        }
        Ok(())
    }
}

/// Operation codes for identity management IPC requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum IdentityOpcode {
    /// Register a new user identity.
    Register = 0x0900,
    /// Authenticate and log in an existing user.
    Login = 0x0901,
    /// Log out the currently authenticated user session.
    Logout = 0x0902,
    /// Lock the active session.
    Lock = 0x0903,
    /// Initiate account or identity recovery.
    Recover = 0x0904,
    /// Load an identity cryptographic key.
    LoadKey = 0x0905,
}

impl IdentityOpcode {
    /// Converts the identity opcode enum variant to its raw `u32` value.
    pub const fn as_u32(self) -> u32 {
        self as u32
    }
}

/// Opaque handle identifying an identity key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IdentityKeyHandle(pub u64);

/// Request structure to load an identity key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentityKeyLoadRequest {
    /// Target key handle to load.
    pub key_handle: IdentityKeyHandle,
    /// Security token authorizing the key load capability.
    pub capability_token: u64,
}

/// Response structure for an identity key load request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentityKeyLoadResponse {
    /// Identifier assigned to the initiated key loading operation.
    pub operation_id: u64,
}

/// Request message parameters to register a new identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityRegisterMessage {
    /// Unique identifier for the registering user.
    pub user_id: String,
    /// Human-readable display name for the user profile.
    pub display_name: String,
    /// Blockchain chain ID associated with the user identity.
    pub chain_id: u64,
    /// Target network name (e.g., "devnet", "mainnet").
    pub network: String,
    /// Duration in seconds for which session tokens remain valid.
    pub session_ttl_seconds: u64,
}

/// Response returned after successfully registering an identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityRegisterResponse {
    /// Unique identifier of the newly registered user.
    pub user_id: String,
    /// Generated wallet address associated with the registered identity.
    pub wallet_address: String,
    /// Chain ID on which the identity was registered.
    pub chain_id: u64,
    /// Target network name of the registration.
    pub network: String,
    /// Indicates whether explicit recovery phrase confirmation is required.
    pub recovery_confirmation_required: bool,
}

/// Opaque handle holding an authentication token or secret.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AuthenticationHandle(pub u128);

impl core::fmt::Debug for AuthenticationHandle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("AuthenticationHandle(REDACTED)")
    }
}

/// Request message to log in an existing user identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentityLoginMessage {
    /// User identifier requesting login.
    pub user_id: String,
    /// Opaque handle containing authentication credentials.
    pub authentication_handle: AuthenticationHandle,
}

/// Response containing active session information for a logged-in user.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentitySessionResponse {
    /// Authenticated user identifier.
    pub user_id: String,
    /// Wallet address associated with the active session.
    pub wallet_address: String,
    /// Expiration timestamp of the session in Unix epoch seconds.
    pub expires_at_unix: u64,
}

/// Operation codes for wallet-related IPC service calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum WalletOpcode {
    /// Query public identity details for a wallet.
    GetPublicIdentity = 0x1000,
    /// Provision a new wallet keypair.
    Provision = 0x1001,
    /// Sign a message payload using a wallet key.
    Sign = 0x1002,
    /// Lock access to the wallet.
    Lock = 0x1003,
    /// Delete a wallet or revoke its credentials.
    Delete = 0x1004,
}

impl WalletOpcode {
    /// Converts the wallet opcode enum variant to its raw `u32` value.
    pub const fn as_u32(self) -> u32 {
        self as u32
    }
}

/// Information describing a wallet's public identity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletIdentityMessage {
    /// User identifier owning the wallet.
    pub user_id: String,
    /// Public wallet address.
    pub wallet_address: String,
    /// Associated blockchain chain identifier.
    pub chain_id: u64,
    /// Associated network name.
    pub network: String,
}

/// Parameters for a request to sign data with a wallet key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletSignMessage {
    /// User identifier requesting signature.
    pub user_id: String,
    /// Identifier of the cryptographic key to use for signing.
    pub key_id: String,
    /// Domain tag or context string for signature verification.
    pub domain: String,
    /// Raw byte payload to be signed.
    pub message: Vec<u8>,
}

/// Result returned after a successful signing operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalletSignResponse {
    /// Name of the cryptographic signature algorithm used.
    pub algorithm: String,
    /// Resulting signature bytes.
    pub signature: Vec<u8>,
    /// SHA-256 or digest hash of the original message that was signed.
    pub message_digest: [u8; 32],
}

/// Maximum allowable byte length for an IPC message payload (1 MiB).
pub const MAX_IPC_PAYLOAD: usize = 1024 * 1024;

/// Validates whether a message payload byte slice complies with `MAX_IPC_PAYLOAD`.
pub fn validate_payload(payload: &[u8]) -> bool {
    payload.len() <= MAX_IPC_PAYLOAD
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use alloc::vec;

    #[test]
    fn opcode_values_are_stable() {
        assert_eq!(IdentityOpcode::Register.as_u32(), 0x0900);
        assert_eq!(IdentityOpcode::LoadKey.as_u32(), 0x0905);
        assert_eq!(WalletOpcode::Sign.as_u32(), 0x1002);
    }

    #[test]
    fn key_request_contains_only_opaque_handles() {
        let r = IdentityKeyLoadRequest {
            key_handle: IdentityKeyHandle(7),
            capability_token: 9,
        };
        assert_eq!(r.key_handle, IdentityKeyHandle(7));
        assert_eq!(r.capability_token, 9);
    }

    #[test]
    fn registration_excludes_recovery_phrase() {
        let r = IdentityRegisterResponse {
            user_id: "user".into(),
            wallet_address: "ATC00000000000000000000000000000000000".into(),
            chain_id: 1,
            network: "devnet".into(),
            recovery_confirmation_required: true,
        };
        assert!(r.recovery_confirmation_required);
    }

    #[test]
    fn oversized_ipc_is_rejected() {
        assert!(!validate_payload(&vec![0; MAX_IPC_PAYLOAD + 1]));
    }

    #[test]
    fn message_header_cannot_lie_about_payload_size() {
        let mut m = Message::new(Endpoint(1), 7, vec![1, 2, 3]);
        m.header.payload_len = 2;
        assert_eq!(m.validate(), Err(IpcError::PayloadTooLarge));
    }

    #[test]
    fn authentication_handle_debug_is_redacted() {
        assert_eq!(
            format!("{:?}", AuthenticationHandle(42)),
            "AuthenticationHandle(REDACTED)"
        );
    }
}
