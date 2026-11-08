// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! Syscall ABI and Dispatcher implementation (SC-004).

extern crate alloc;
use alloc::vec::Vec;
use crate::capability::{Capability, CapabilityId};
use crate::ipc::KernelEndpoint;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SyscallOp {
    Yield,
    Call,
    Reply,
    Recv,
    Send,
    MintCap,
    RevokeCap,
    MapMemory,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SyscallResult {
    Success(u64),
    PermissionDenied,
    InvalidCapability,
    ChannelClosed,
    OutOfMemory,
    InvalidArgument,
}

pub struct SyscallDispatcher;

impl SyscallDispatcher {
    pub fn dispatch_send(
        endpoint: &mut KernelEndpoint,
        cap: &Capability,
        payload: Vec<u8>,
    ) -> SyscallResult {
        if cap.id.0 == 0 {
            return SyscallResult::InvalidCapability;
        }
        match endpoint.send(cap, payload, None) {
            Ok(()) => SyscallResult::Success(0),
            Err(e) if e.contains("lacks SEND") => SyscallResult::PermissionDenied,
            Err(_) => SyscallResult::InvalidArgument,
        }
    }

    pub fn dispatch_recv(
        endpoint: &mut KernelEndpoint,
        cap: &Capability,
    ) -> (SyscallResult, Option<Vec<u8>>) {
        if cap.id.0 == 0 {
            return (SyscallResult::InvalidCapability, None);
        }
        match endpoint.recv(cap) {
            Ok(msg) => (SyscallResult::Success(msg.sender_badge), Some(msg.payload)),
            Err(e) if e.contains("lacks RECV") => (SyscallResult::PermissionDenied, None),
            Err(_) => (SyscallResult::InvalidArgument, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::{CapabilityRights, CapabilityType};
    use crate::ipc::ChannelId;

    #[test]
    fn test_syscall_dispatch_send_recv() {
        let mut ep = KernelEndpoint::new(ChannelId(1), 5);
        let cap = Capability::new(10, CapabilityType::Endpoint, CapabilityRights::all(), 42);

        let res = SyscallDispatcher::dispatch_send(&mut ep, &cap, alloc::vec![10, 20, 30]);
        assert_eq!(res, SyscallResult::Success(0));

        let (res_recv, payload) = SyscallDispatcher::dispatch_recv(&mut ep, &cap);
        assert_eq!(res_recv, SyscallResult::Success(42));
        assert_eq!(payload, Some(alloc::vec![10, 20, 30]));
    }
}
