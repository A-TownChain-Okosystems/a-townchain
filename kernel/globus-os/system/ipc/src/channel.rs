//! Bounded deterministic IPC channels.

use crate::{Endpoint, MAX_IPC_PAYLOAD, Message, validate_payload};
use alloc::collections::{BTreeMap, VecDeque};

/// Errors that can occur during IPC channel operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IpcError {
    /// The message payload exceeds the maximum allowed IPC size.
    PayloadTooLarge,
    /// The target endpoint channel queue is full.
    QueueFull,
    /// The specified target endpoint was not found in the channel registry.
    EndpointNotFound,
}

/// A registry for managing IPC communication channels between endpoints.
#[derive(Debug)]
pub struct ChannelRegistry {
    capacity: usize,
    queues: BTreeMap<Endpoint, VecDeque<Message>>,
}

impl ChannelRegistry {
    /// Creates a new `ChannelRegistry` with the specified per-endpoint queue capacity.
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: if capacity < 1 { 1 } else { capacity },
            queues: BTreeMap::new(),
        }
    }

    /// Registers a new endpoint in the registry.
    pub fn register(&mut self, endpoint: Endpoint) -> bool {
        self.queues.entry(endpoint).or_default();
        true
    }

    /// Sends an IPC message to the destination endpoint specified in the message header.
    pub fn send(&mut self, message: Message) -> Result<(), IpcError> {
        if message.payload.len() > MAX_IPC_PAYLOAD || !validate_payload(&message.payload) {
            return Err(IpcError::PayloadTooLarge);
        }
        let queue = self
            .queues
            .get_mut(&message.header.endpoint)
            .ok_or(IpcError::EndpointNotFound)?;
        if queue.len() >= self.capacity {
            return Err(IpcError::QueueFull);
        }
        queue.push_back(message);
        Ok(())
    }

    /// Receives a message from the queue of the specified endpoint, if available.
    pub fn receive(&mut self, endpoint: Endpoint) -> Result<Option<Message>, IpcError> {
        self.queues
            .get_mut(&endpoint)
            .map(|q| q.pop_front())
            .ok_or(IpcError::EndpointNotFound)
    }

    /// Returns the number of queued messages for the specified endpoint.
    pub fn len(&self, endpoint: Endpoint) -> Result<usize, IpcError> {
        self.queues
            .get(&endpoint)
            .map(VecDeque::len)
            .ok_or(IpcError::EndpointNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn fifo_is_deterministic() {
        let mut r = ChannelRegistry::new(2);
        let e = Endpoint(1);
        r.register(e);
        r.send(Message::new(e, 1, vec![1])).unwrap();
        r.send(Message::new(e, 2, vec![2])).unwrap();
        assert_eq!(r.receive(e).unwrap().unwrap().header.opcode, 1);
        assert_eq!(r.receive(e).unwrap().unwrap().header.opcode, 2);
    }

    #[test]
    fn capacity_is_enforced() {
        let mut r = ChannelRegistry::new(1);
        let e = Endpoint(2);
        r.register(e);
        r.send(Message::new(e, 1, vec![])).unwrap();
        assert_eq!(r.send(Message::new(e, 2, vec![])), Err(IpcError::QueueFull));
    }
}
