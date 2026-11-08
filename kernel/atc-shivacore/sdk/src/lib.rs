#![no_std]
#![allow(dead_code)]

extern crate alloc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CapabilityId(pub u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChannelId(pub u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AttestationId(pub u64);

pub trait CapabilityResolver {
    fn resolve(&self, id: CapabilityId) -> bool;
}

pub trait IpcEndpoint {
    fn channel(&self) -> ChannelId;
}
