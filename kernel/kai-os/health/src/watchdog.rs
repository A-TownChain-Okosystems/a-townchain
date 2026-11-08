//! Tick-basierter Watchdog: Running -> Stale -> DeclaredDead.
//! Recovery NUR durch echten Herzschlag; Restart nur für DeclaredDead.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceState {
    Running,
    Stale,
    DeclaredDead,
}

#[derive(Debug, PartialEq)]
pub enum WatchdogError {
    UnknownService(String),
    DuplicateService(String),
    IllegalRestart {
        service: String,
        state: ServiceState,
    },
    IllegalHeartbeat {
        service: String,
        state: ServiceState,
    },
}

impl std::fmt::Display for WatchdogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WatchdogError::UnknownService(s) => write!(f, "unknown service '{s}' (fail-closed)"),
            WatchdogError::DuplicateService(s) => write!(f, "service '{s}' already registered"),
            WatchdogError::IllegalRestart { service, state } => {
                write!(
                    f,
                    "restart denied for '{service}' in state {state:?} (only DeclaredDead)"
                )
            }
            WatchdogError::IllegalHeartbeat { service, state } => {
                write!(f, "heartbeat denied for '{service}' in state {state:?} (dead services need restart)")
            }
        }
    }
}

struct ServiceEntry {
    state: ServiceState,
    last_heartbeat: u64,
    stale_after: u64, // Ticks ohne Herzschlag -> Stale
    dead_after: u64,  // weitere Ticks ohne Herzschlag -> DeclaredDead
    restarts: u64,
}

pub struct Watchdog {
    now_tick: u64,
    services: BTreeMap<String, ServiceEntry>,
}

impl Watchdog {
    pub fn new(start_tick: u64) -> Self {
        Self {
            now_tick: start_tick,
            services: BTreeMap::new(),
        }
    }

    pub fn now_tick(&self) -> u64 {
        self.now_tick
    }

    /// Service registrieren — ab sofort beobachtet (fail-closed bei Duplikat).
    pub fn register(
        &mut self,
        service_id: &str,
        stale_after: u64,
        dead_after: u64,
    ) -> Result<(), WatchdogError> {
        if self.services.contains_key(service_id) {
            return Err(WatchdogError::DuplicateService(service_id.to_string()));
        }
        self.services.insert(
            service_id.to_string(),
            ServiceEntry {
                state: ServiceState::Running,
                last_heartbeat: self.now_tick,
                stale_after,
                dead_after,
                restarts: 0,
            },
        );
        Ok(())
    }

    /// Echter Herzschlag: bestätigt Liveness; Stale -> Running.
    /// DeclaredDead-Services heilen sich NICHT selbst — sie brauchen einen Restart.
    pub fn heartbeat(&mut self, service_id: &str) -> Result<(), WatchdogError> {
        let entry = self
            .services
            .get_mut(service_id)
            .ok_or_else(|| WatchdogError::UnknownService(service_id.to_string()))?;
        match entry.state {
            ServiceState::DeclaredDead => Err(WatchdogError::IllegalHeartbeat {
                service: service_id.to_string(),
                state: entry.state,
            }),
            ServiceState::Stale | ServiceState::Running => {
                entry.state = ServiceState::Running;
                entry.last_heartbeat = self.now_tick;
                Ok(())
            }
        }
    }

    /// Zeit um einen Tick fortschalten und Zustände neu bewerten.
    /// Fail-closed: vergangene Zeit kann Zustände nur VERSCHLECHTERN, nie heilen.
    pub fn tick(&mut self) {
        let now = self.now_tick + 1;
        self.now_tick = now;
        for entry in self.services.values_mut() {
            let missed = now.saturating_sub(entry.last_heartbeat);
            if entry.state == ServiceState::Running && missed > entry.stale_after {
                entry.state = ServiceState::Stale;
            }
            if entry.state == ServiceState::Stale && missed > entry.stale_after + entry.dead_after {
                entry.state = ServiceState::DeclaredDead;
            }
        }
    }

    pub fn state(&self, service_id: &str) -> Result<ServiceState, WatchdogError> {
        self.services
            .get(service_id)
            .map(|e| e.state)
            .ok_or_else(|| WatchdogError::UnknownService(service_id.to_string()))
    }

    /// Deterministischer Report für CLI/Supervisor: (id, state, restarts) sortiert.
    pub fn report(&self) -> Vec<(String, ServiceState, u64)> {
        self.services
            .iter()
            .map(|(id, e)| (id.clone(), e.state, e.restarts))
            .collect()
    }

    pub fn restart_count(&self, service_id: &str) -> Result<u64, WatchdogError> {
        self.services
            .get(service_id)
            .map(|e| e.restarts)
            .ok_or_else(|| WatchdogError::UnknownService(service_id.to_string()))
    }

    /// Restart NUR für DeclaredDead (Supervisor-Kopplung): Zähler hoch,
    /// Zustand zurück auf Running mit frischem Herzschlag.
    pub fn restart(&mut self, service_id: &str) -> Result<u64, WatchdogError> {
        let entry = self
            .services
            .get_mut(service_id)
            .ok_or_else(|| WatchdogError::UnknownService(service_id.to_string()))?;
        if entry.state != ServiceState::DeclaredDead {
            return Err(WatchdogError::IllegalRestart {
                service: service_id.to_string(),
                state: entry.state,
            });
        }
        entry.restarts += 1;
        entry.state = ServiceState::Running;
        entry.last_heartbeat = self.now_tick;
        Ok(entry.restarts)
    }
}
