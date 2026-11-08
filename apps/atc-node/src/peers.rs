// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Peer-Tabelle mit Lifecycle (Peer-Lifecycle: Connected/Verified/Banned).

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PeerState {
    Connected,
    Verified,
    Banned,
}

#[derive(Debug, Clone)]
pub struct Peer {
    pub id: u64,
    pub addr: String,
    pub state: PeerState,
}

#[derive(Default)]
pub struct PeerTable {
    peers: Vec<Peer>,
}

impl PeerTable {
    pub fn new() -> Self {
        PeerTable::default()
    }

    pub fn add(&mut self, id: u64, addr: impl Into<String>) -> bool {
        if self.peers.iter().any(|p| p.id == id) {
            return false;
        }
        self.peers.push(Peer { id, addr: addr.into(), state: PeerState::Connected });
        true
    }

    pub fn verify(&mut self, id: u64) -> bool {
        if let Some(p) = self.peers.iter_mut().find(|p| p.id == id) {
            p.state = PeerState::Verified;
            true
        } else {
            false
        }
    }

    pub fn ban(&mut self, id: u64) -> bool {
        if let Some(p) = self.peers.iter_mut().find(|p| p.id == id) {
            p.state = PeerState::Banned;
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, id: u64) -> bool {
        let before = self.peers.len();
        self.peers.retain(|p| p.id != id);
        before != self.peers.len()
    }

    pub fn is_banned(&self, id: u64) -> bool {
        self.peers.iter().any(|p| p.id == id && p.state == PeerState::Banned)
    }

    pub fn len(&self) -> usize {
        self.peers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle() {
        let mut t = PeerTable::new();
        assert!(t.add(1, "addr1"));
        assert!(!t.add(1, "dup"));
        assert!(t.verify(1));
        assert!(t.ban(1));
        assert!(t.is_banned(1));
        assert!(!t.is_banned(2));
        assert!(t.remove(1));
        assert!(!t.remove(1));
        assert!(t.is_empty());
    }

    #[test]
    fn unbekannte_ids() {
        let mut t = PeerTable::new();
        assert!(!t.verify(9));
        assert!(!t.ban(9));
        assert!(!t.remove(9));
    }
}
