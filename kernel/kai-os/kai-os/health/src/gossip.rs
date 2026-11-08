//! Health-Gossip: deterministisches Merge von Gesundheitszuständen.
//! Regel: höherer Tick gewinnt; bei Gleichstand gewinnt der LOKALE Eintrag
//! (verhindert Oszillation). Snapshots sind deterministisch sortiert und
//! über die P2P-Discovery (Issue #104) transportfähig.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Stale,
    Dead,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeHealth {
    pub node_id: String,
    pub status: HealthStatus,
    pub tick: u64,
}

#[derive(Default)]
pub struct HealthGossip {
    self_id: String,
    /// Beobachter-Node -> (Beobachteter Node -> Eintrag).
    /// Eigenbeobachtungen leben unter self_id.
    observations: BTreeMap<String, BTreeMap<String, NodeHealth>>,
}

impl HealthGossip {
    pub fn new(self_id: &str) -> Self {
        Self {
            self_id: self_id.to_string(),
            observations: BTreeMap::new(),
        }
    }

    /// Eigene Beobachtung einpflegen (überschreibt den alten eigenen Eintrag).
    pub fn observe(&mut self, node_id: &str, status: HealthStatus, tick: u64) {
        self.observations
            .entry(self.self_id.clone())
            .or_default()
            .insert(
                node_id.to_string(),
                NodeHealth {
                    node_id: node_id.to_string(),
                    status,
                    tick,
                },
            );
    }

    /// Remote-Snapshot mergen (von einem Peer empfangen). Regel:
    /// höherer Tick gewinnt; Gleichstand -> der eigene Eintrag bleibt.
    /// Rückgabe: Anzahl übernommener Fremd-Einträge.
    pub fn merge(&mut self, observer: &str, remote: &[NodeHealth]) -> usize {
        let target = self.observations.entry(observer.to_string()).or_default();
        let mut adopted = 0;
        for r in remote {
            match target.get(&r.node_id) {
                Some(local) if local.tick >= r.tick => {} // lokal gewinnt (auch bei Gleichstand)
                _ => {
                    target.insert(r.node_id.clone(), r.clone());
                    adopted += 1;
                }
            }
        }
        adopted
    }

    /// Deterministisch sortierter Snapshot der Beobachtungen EINES Beobachters.
    pub fn snapshot(&self, observer: &str) -> Vec<NodeHealth> {
        self.observations
            .get(observer)
            .map(|m| m.values().cloned().collect())
            .unwrap_or_default()
    }

    /// Aggregierte Sicht auf einen Node: schlimmster beobachteter Zustand
    /// bei höchstem Tick (fail-closed konservativ).
    pub fn consensus_view(&self, node_id: &str) -> Option<NodeHealth> {
        let mut best: Option<NodeHealth> = None;
        for observer_map in self.observations.values() {
            if let Some(entry) = observer_map.get(node_id) {
                best = Some(match best {
                    None => entry.clone(),
                    Some(b) => {
                        if entry.tick > b.tick
                            || (entry.tick == b.tick && severity(entry.status) > severity(b.status))
                        {
                            entry.clone()
                        } else {
                            b
                        }
                    }
                });
            }
        }
        best
    }
}

/// Mapping Watchdog -> Gossip (einheitliche Terminologie).
impl From<crate::watchdog::ServiceState> for HealthStatus {
    fn from(s: crate::watchdog::ServiceState) -> Self {
        match s {
            crate::watchdog::ServiceState::Running => HealthStatus::Healthy,
            crate::watchdog::ServiceState::Stale => HealthStatus::Stale,
            crate::watchdog::ServiceState::DeclaredDead => HealthStatus::Dead,
        }
    }
}

fn severity(s: HealthStatus) -> u8 {
    match s {
        HealthStatus::Healthy => 0,
        HealthStatus::Stale => 1,
        HealthStatus::Dead => 2,
    }
}
