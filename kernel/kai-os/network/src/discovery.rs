//! Peer Discovery: ANNOUNCE / PING-PONG / PEER_LIST — deterministisch, dedupliziert.

use crate::peer::{PeerInfo, PeerStore};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscoveryMessage {
    Announce {
        info: PeerInfo,
    },
    Ping {
        from_peer_id: String,
        tick: u64,
    },
    Pong {
        from_peer_id: String,
        tick: u64,
    },
    PeerList {
        from_peer_id: String,
        peers: Vec<PeerInfo>,
    },
}

pub struct Discovery {
    pub self_peer_id: String,
    store: PeerStore,
}

impl Discovery {
    pub fn new(self_peer_id: &str) -> Self {
        Self {
            self_peer_id: self_peer_id.into(),
            store: PeerStore::new(),
        }
    }

    pub fn store(&self) -> &PeerStore {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut PeerStore {
        &mut self.store
    }

    /// Eingehende Discovery-Nachricht verarbeiten; Antwort falls erforderlich.
    pub fn handle(&mut self, msg: DiscoveryMessage, now_tick: u64) -> Option<DiscoveryMessage> {
        match msg {
            DiscoveryMessage::Announce { info } => {
                let mut info = info;
                info.last_seen_tick = now_tick;
                self.store.upsert(info); // Dedup über PeerId (upsert)
                None
            }
            DiscoveryMessage::Ping {
                from_peer_id: _,
                tick,
            } => Some(DiscoveryMessage::Pong {
                from_peer_id: self.self_peer_id.clone(),
                tick,
            }),
            DiscoveryMessage::Pong { .. } => None,
            DiscoveryMessage::PeerList {
                from_peer_id: _,
                peers,
            } => {
                for mut p in peers {
                    p.last_seen_tick = now_tick;
                    self.store.upsert(p);
                }
                None
            }
        }
    }

    /// Eigenen PeerList versenden (deterministisch sortiert).
    pub fn peer_list(&self) -> Vec<PeerInfo> {
        self.store.all().into_iter().cloned().collect()
    }
}
