//! Health-Monitor: aggregierter Komponenten-Status (Heartbeat/Liveness).

use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

impl HealthStatus {
    /// Schlechtester Status gewinnt (worst-of aggregation).
    pub fn worst(a: HealthStatus, b: HealthStatus) -> HealthStatus {
        use HealthStatus::*;
        match (a, b) {
            (Unhealthy, _) | (_, Unhealthy) => Unhealthy,
            (Degraded, _) | (_, Degraded) => Degraded,
            _ => Healthy,
        }
    }
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn overall(statuses: &[HealthStatus]) -> HealthStatus {
    statuses
        .iter()
        .copied()
        .fold(HealthStatus::Healthy, HealthStatus::worst)
}
