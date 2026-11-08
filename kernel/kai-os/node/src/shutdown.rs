//! Geordnetes Herunterfahren: Subsysteme stoppen in UMKREHRTER Start-Reihenfolge
//! (drain), Fehler werden im Report als `forced` vermerkt, dann force.

use crate::supervisor::Supervisor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownReport {
    pub stopped_in_order: Vec<String>,
    pub forced: Vec<String>,
    pub clean: bool,
}

impl ShutdownReport {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_default()
    }
}

pub struct ShutdownCoordinator;

impl ShutdownCoordinator {
    /// Graceful Shutdown: umgekehrte Start-Reihenfolge (LIFO-Drain).
    pub fn shutdown_all(sup: &mut Supervisor) -> ShutdownReport {
        let mut stopped = Vec::new();
        let mut forced = Vec::new();
        let order = sup.shutdown_order();
        for name in &order {
            match sup.stop_subsystem(name) {
                Ok(()) => stopped.push(name.clone()),
                Err(_) => forced.push(name.clone()),
            }
        }
        let clean = forced.is_empty();
        ShutdownReport {
            stopped_in_order: stopped,
            forced,
            clean,
        }
    }
}
