// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! IPC Endpoints and Messaging implementation (SC-003).

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use crate::capability::{Capability, CapabilityRights};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChannelId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub sender_badge: u64,
    pub payload: Vec<u8>,
    pub transferred_cap: Option<Capability>,
}

#[derive(Debug)]
pub struct Notification {
    pub bitmask: u64,
}

impl Notification {
    pub fn new() -> Self {
        Self { bitmask: 0 }
    }

    pub fn signal(&mut self, mask: u64) {
        self.bitmask |= mask;
    }

    pub fn poll(&mut self) -> u64 {
        let val = self.bitmask;
        self.bitmask = 0;
        val
    }
}

impl Default for Notification {
    fn default() -> Self {
        Self::new()
    }
}

pub struct KernelEndpoint {
    pub channel_id: ChannelId,
    queue: Vec<Message>,
    capacity: usize,
}

impl KernelEndpoint {
    pub fn new(channel_id: ChannelId, capacity: usize) -> Self {
        Self {
            channel_id,
            queue: Vec::new(),
            capacity,
        }
    }

    pub fn send(&mut self, sender_cap: &Capability, payload: Vec<u8>, transfer_cap: Option<Capability>) -> Result<(), String> {
        if !sender_cap.rights.send {
            return Err(String::from("Capability lacks SEND right"));
        }
        if self.queue.len() >= self.capacity {
            return Err(String::from("Endpoint queue full"));
        }
        self.queue.push(Message {
            sender_badge: sender_cap.badge,
            payload,
            transferred_cap: transfer_cap,
        });
        Ok(())
    }

    pub fn recv(&mut self, receiver_cap: &Capability) -> Result<Message, String> {
        if !receiver_cap.rights.recv {
            return Err(String::from("Capability lacks RECV right"));
        }
        if self.queue.is_empty() {
            return Err(String::from("No messages available in endpoint"));
        }
        Ok(self.queue.remove(0))
    }

    pub fn pending_count(&self) -> usize {
        self.queue.len()
    }
}

pub trait IpcEndpoint {
    fn channel(&self) -> ChannelId;
}

impl IpcEndpoint for KernelEndpoint {
    fn channel(&self) -> ChannelId {
        self.channel_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::CapabilityType;

    #[test]
    fn test_ipc_send_receive_roundtrip() {
        let mut ep = KernelEndpoint::new(ChannelId(1), 10);
        let send_cap = Capability::new(100, CapabilityType::Endpoint, CapabilityRights::all(), 555);
        let recv_cap = Capability::new(101, CapabilityType::Endpoint, CapabilityRights::all(), 0);

        let data = alloc::vec![1, 2, 3, 4];
        ep.send(&send_cap, data.clone(), None).expect("Send should succeed");
        assert_eq!(ep.pending_count(), 1);

        let msg = ep.recv(&recv_cap).expect("Recv should succeed");
        assert_eq!(msg.sender_badge, 555);
        assert_eq!(msg.payload, data);
        assert_eq!(ep.pending_count(), 0);
    }

    #[test]
    fn test_notification_bitmask() {
        let mut notif = Notification::new();
        notif.signal(0b0001);
        notif.signal(0b0100);
        assert_eq!(notif.poll(), 0b0101);
        assert_eq!(notif.poll(), 0); // Polling resets bitmask
    }
}
