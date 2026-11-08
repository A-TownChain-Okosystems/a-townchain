//! Stable, versioned userspace ABI exposed by ShivaCore.
//!
//! This crate contains only wire-safe identifiers and request/response types.
//! Kernel implementations remain private to ShivaCore.

#![no_std]

pub use globus_ipc::{Endpoint, Message, MessageHeader};

pub const ABI_MAJOR: u16 = 1;
pub const ABI_MINOR: u16 = 0;
pub const ABI_VERSION: u32 = ((ABI_MAJOR as u32) << 16) | ABI_MINOR as u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Syscall {
    Yield = 0x0001,
    IpcSend = 0x0010,
    IpcReceive = 0x0011,
    CapabilityQuery = 0x0020,
    HandleClose = 0x0030,
    MonotonicTime = 0x0040,
}

impl Syscall {
    pub const fn id(self) -> u16 {
        self as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CapabilityHandle(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ObjectHandle(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapabilityRight {
    Read,
    Write,
    Execute,
    Grant,
    Revoke,
    Inspect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AbiError {
    InvalidSyscall = 1,
    InvalidHandle = 2,
    PermissionDenied = 3,
    InvalidPayload = 4,
    EndpointUnavailable = 5,
    ResourceExhausted = 6,
    AbiVersionMismatch = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbiHandshake {
    pub major: u16,
    pub minor: u16,
}

impl AbiHandshake {
    pub const CURRENT: Self = Self {
        major: ABI_MAJOR,
        minor: ABI_MINOR,
    };

    pub const fn compatible(self) -> bool {
        self.major == ABI_MAJOR
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpcSendRequest {
    pub endpoint: Endpoint,
    pub capability: CapabilityHandle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IpcReceiveRequest {
    pub endpoint: Endpoint,
    pub capability: CapabilityHandle,
    pub max_payload: u32,
}

/// Stable 32-byte digest exchanged across the kernel/service boundary.
pub type Hash32 = [u8; 32];

/// Deterministic compatibility hash used by existing Service-Space records.
///
/// This is deliberately kept as a contract primitive so callers do not import
/// the kernel security implementation. It is not a cryptographic hash.
pub fn simple_hash(data: &[u8]) -> Hash32 {
    let mut hash: u32 = 0x811c9dc5;
    for &byte in data {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(0x01000193);
    }
    let mut result = [0u8; 32];
    result[..4].copy_from_slice(&hash.to_be_bytes());
    for i in (4..32).step_by(4) {
        hash ^= result[i - 4] as u32;
        hash = hash.wrapping_mul(0x01000193);
        result[i..i + 4].copy_from_slice(&hash.to_be_bytes());
    }
    result
}

/// Public capability rights representation. Kernel storage remains private.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CapabilityRights(pub u8);

impl CapabilityRights {
    pub const NONE: Self = Self(0);
    pub const READ: Self = Self(1);
    pub const WRITE: Self = Self(2);
    pub const EXECUTE: Self = Self(4);
    pub const DELEGATE: Self = Self(8);
    pub const INSPECT: Self = Self(16);

    pub const fn contains(self, required: Self) -> bool {
        self.0 & required.0 == required.0
    }

    pub const fn bits(self) -> u8 {
        self.0
    }
}

/// Stable boot phases visible to services without exposing KernelState.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum KernelBootPhase {
    Heap = 0,
    Capabilities = 1,
    Processes = 2,
    Scheduler = 3,
    Ipc = 4,
    FileSystem = 5,
    Security = 6,
    Ready = 7,
}

pub const MAX_SYSCALL_PAYLOAD: usize = 1024 * 1024;

pub const fn validate_abi_version(version: u32) -> bool {
    (version >> 16) as u16 == ABI_MAJOR
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abi_version_is_stable() {
        assert_eq!(ABI_VERSION, 0x0001_0000);
        assert!(validate_abi_version(0x0001_ffff));
        assert!(!validate_abi_version(0x0002_0000));
    }

    #[test]
    fn syscall_ids_are_stable() {
        assert_eq!(Syscall::IpcSend.id(), 0x0010);
        assert_eq!(Syscall::MonotonicTime.id(), 0x0040);
    }

    #[test]
    fn simple_hash_is_stable() {
        assert_eq!(simple_hash(b"abc"), simple_hash(b"abc"));
        assert_ne!(simple_hash(b"abc"), simple_hash(b"abd"));
    }

    #[test]
    fn capability_rights_are_monotonic() {
        let rights = CapabilityRights::READ;
        assert!(rights.contains(CapabilityRights::READ));
        assert!(!rights.contains(CapabilityRights::WRITE));
    }

    #[test]
    fn handshake_accepts_minor_changes() {
        assert!(
            AbiHandshake {
                major: 1,
                minor: 99
            }
            .compatible()
        );
        assert!(!AbiHandshake { major: 2, minor: 0 }.compatible());
    }
}
