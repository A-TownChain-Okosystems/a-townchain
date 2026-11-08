//! Deterministic socket lifecycle and policy enforcement.

use crate::{NetworkPolicy, Protocol, SocketId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SocketState {
    Created,
    Bound,
    Listening,
    Connected,
    Closed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Socket {
    pub id: SocketId,
    pub protocol: Protocol,
    pub state: SocketState,
}

#[derive(Debug, Default)]
pub struct SocketTable {
    next: u64,
    sockets: Vec<Socket>,
}

impl SocketTable {
    pub fn new() -> Self {
        Self {
            next: 1,
            sockets: Vec::new(),
        }
    }
    pub fn create(&mut self, protocol: Protocol, policy: NetworkPolicy) -> Option<SocketId> {
        if policy == NetworkPolicy::Disabled {
            return None;
        }
        let id = SocketId(self.next);
        self.next = self.next.checked_add(1)?;
        self.sockets.push(Socket {
            id,
            protocol,
            state: SocketState::Created,
        });
        Some(id)
    }
    pub fn transition(&mut self, id: SocketId, next: SocketState) -> bool {
        let Some(socket) = self.sockets.iter_mut().find(|s| s.id == id) else {
            return false;
        };
        let valid = matches!(
            (socket.state, next),
            (SocketState::Created, SocketState::Bound)
                | (SocketState::Bound, SocketState::Listening)
                | (SocketState::Bound, SocketState::Connected)
                | (SocketState::Created, SocketState::Connected)
                | (_, SocketState::Closed)
        );
        if valid {
            socket.state = next;
        }
        valid
    }
    pub fn get(&self, id: SocketId) -> Option<&Socket> {
        self.sockets.iter().find(|s| s.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disabled_policy_denies_creation() {
        let mut t = SocketTable::new();
        assert!(t.create(Protocol::Tcp, NetworkPolicy::Disabled).is_none());
    }
    #[test]
    fn lifecycle_is_checked() {
        let mut t = SocketTable::new();
        let id = t.create(Protocol::Tcp, NetworkPolicy::Normal).unwrap();
        assert!(!t.transition(id, SocketState::Listening));
        assert!(t.transition(id, SocketState::Bound));
        assert!(t.transition(id, SocketState::Listening));
    }
}
