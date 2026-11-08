//! Node Supervisor: startet/überwacht Subsysteme, Crash-Restart-Policy
//! (deterministisch: fixe Backoffs aus Config, kein RNG — REQ-ENG-002),
//! Persistenz für Crash-Recovery (Daemon-Neustart stellt Zustand wieder her).
//!
//! Der Supervisor implementiert Consensus-INFRASTRUKTUR (Orchestrierung),
//! niemals Consensus-SEMANTIK (AD-008.4).

use crate::audit::AuditLogger;
use crate::health::{overall, HealthStatus};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug)]
pub struct SubsystemError(pub String);

impl std::fmt::Display for SubsystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "subsystem error: {}", self.0)
    }
}

pub trait Subsystem: Send {
    fn name(&self) -> &'static str;
    fn start(&mut self) -> Result<(), SubsystemError>;
    fn stop(&mut self) -> Result<(), SubsystemError>;
    fn health(&self) -> HealthStatus;
    /// Ein Fehler in `tick` wird als Crash-ähnlicher Ausfall behandelt.
    fn tick(&mut self) -> Result<(), SubsystemError>;
}

struct Entry {
    subsystem: Box<dyn Subsystem>,
    restarts: u32,
    last_health: HealthStatus,
    running: bool,
}

/// Persistierbarer Supervisor-Zustand (Crash-Recovery).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorState {
    pub subsystems: Vec<SubsystemStateEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemStateEntry {
    pub name: String,
    pub restarts: u32,
    pub last_health: String,
    pub running: bool,
}

pub struct Supervisor {
    entries: Vec<Entry>,
    max_restarts: u32,
    backoff_ms: u64,
}

impl Supervisor {
    pub fn new(max_restarts: u32, backoff_ms: u64) -> Self {
        Self {
            entries: Vec::new(),
            max_restarts,
            backoff_ms,
        }
    }

    pub fn register(&mut self, s: Box<dyn Subsystem>) {
        self.entries.push(Entry {
            subsystem: s,
            restarts: 0,
            last_health: HealthStatus::Healthy,
            running: false,
        });
    }

    /// Fail-closed Boot: scheitert ein Subsystem beim Start, werden bereits
    /// gestartete in umgekehrter Reihenfolge gestoppt und der Boot abgebrochen.
    pub fn start_all(&mut self, audit: &mut AuditLogger) -> Result<(), SubsystemError> {
        for i in 0..self.entries.len() {
            let (name, res) = {
                let e = &mut self.entries[i];
                (e.subsystem.name(), e.subsystem.start())
            };
            match res {
                Ok(()) => {
                    if let Err(err) = audit.record("supervisor", &format!("started {name}")) {
                        let _ = err; // Audit-Schreibfehler faulen nicht den Boot (FileSink optional)
                    }
                    self.entries[i].running = true;
                    self.entries[i].last_health = self.entries[i].subsystem.health();
                }
                Err(err) => {
                    let _ = audit.record("supervisor", &format!("BOOT FAILED at {name}: {err}"));
                    self.entries[i].last_health = HealthStatus::Unhealthy;
                    self.stop_started_reverse();
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    fn stop_started_reverse(&mut self) {
        for e in self.entries.iter_mut().rev() {
            if e.running {
                let _ = e.subsystem.stop();
                e.running = false;
            }
        }
    }

    /// Ein Tick über alle Subsysteme. Crash-Policy: Restart bis max_restarts
    /// (deterministischer Fix-Backoff), danach Unhealthy -> Node Degraded.
    pub fn tick_all(&mut self, audit: &mut AuditLogger) -> HealthStatus {
        for i in 0..self.entries.len() {
            let tick_failed = {
                let e = &mut self.entries[i];
                if !e.running {
                    continue;
                }
                e.subsystem.tick().is_err()
            };
            if tick_failed {
                let name = self.entries[i].subsystem.name();
                if self.entries[i].restarts < self.max_restarts {
                    let stop_ok = self.entries[i].subsystem.stop().is_ok();
                    let start_ok = self.entries[i].subsystem.start().is_ok();
                    self.entries[i].restarts += 1;
                    if self.backoff_ms > 0 {
                        std::thread::sleep(std::time::Duration::from_millis(self.backoff_ms));
                    }
                    let _ = audit.record(
                        "supervisor",
                        &format!(
                            "restart #{r} of {name} (stop_ok={stop_ok}, start_ok={start_ok})",
                            r = self.entries[i].restarts
                        ),
                    );
                    self.entries[i].running = start_ok;
                } else {
                    let _ = self.entries[i].subsystem.stop();
                    let _ = audit.record(
                        "supervisor",
                        &format!("{name} exceeded max_restarts -> UNHEALTHY"),
                    );
                    self.entries[i].running = false;
                }
            }
            self.entries[i].last_health = self.entries[i].subsystem.health();
        }
        let statuses: Vec<HealthStatus> = self.entries.iter().map(|e| e.last_health).collect();
        overall(&statuses)
    }

    pub fn overall_health(&self) -> HealthStatus {
        let statuses: Vec<HealthStatus> = self.entries.iter().map(|e| e.last_health).collect();
        overall(&statuses)
    }

    pub fn restart_count(&self, name: &str) -> Option<u32> {
        self.entries
            .iter()
            .find(|e| e.subsystem.name() == name)
            .map(|e| e.restarts)
    }

    /// Crash-Recovery: Zustand auf Disk persistieren.
    pub fn persist_state(&self, path: &Path) -> Result<(), std::io::Error> {
        let state = self.snapshot();
        let json = serde_json::to_string_pretty(&state).map_err(std::io::Error::other)?;
        std::fs::write(path, json)
    }

    pub fn snapshot(&self) -> SupervisorState {
        SupervisorState {
            subsystems: self
                .entries
                .iter()
                .map(|e| SubsystemStateEntry {
                    name: e.subsystem.name().to_string(),
                    restarts: e.restarts,
                    last_health: format!("{:?}", e.last_health),
                    running: e.running,
                })
                .collect(),
        }
    }

    /// Crash-Recovery: Zustand eines früheren Laufs laden und anwenden.
    pub fn restore(&mut self, state: &SupervisorState) {
        for s in &state.subsystems {
            if let Some(e) = self
                .entries
                .iter_mut()
                .find(|e| e.subsystem.name() == s.name)
            {
                e.restarts = s.restarts;
                e.running = s.running;
            }
        }
    }

    pub fn load_state(path: &Path) -> Result<SupervisorState, std::io::Error> {
        let raw = std::fs::read_to_string(path)?;
        serde_json::from_str(&raw).map_err(std::io::Error::other)
    }

    /// Shutdown-Reihenfolge: umgekehrte Start-Reihenfolge, nur laufende Subsysteme.
    pub fn shutdown_order(&self) -> Vec<String> {
        self.entries
            .iter()
            .rev()
            .filter(|e| e.running)
            .map(|e| e.subsystem.name().to_string())
            .collect()
    }

    /// Einzelnes Subsystem geordnet stoppen (graceful drain).
    pub fn stop_subsystem(&mut self, name: &str) -> Result<(), SubsystemError> {
        if let Some(e) = self
            .entries
            .iter_mut()
            .find(|e| e.subsystem.name() == name && e.running)
        {
            let res = e.subsystem.stop();
            e.running = false;
            res
        } else {
            Ok(())
        }
    }
}

/// Programmierbarer Stub für S01 (Subsysteme entstehen in S04-S12).
pub struct StubSubsystem {
    name: &'static str,
    fail_starts_remaining: u32,
    fail_ticks_remaining: u32,
    pub started: bool,
    pub ticks: u32,
}

impl StubSubsystem {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            fail_starts_remaining: 0,
            fail_ticks_remaining: 0,
            started: false,
            ticks: 0,
        }
    }
    pub fn failing_starts(mut self, n: u32) -> Self {
        self.fail_starts_remaining = n;
        self
    }
    pub fn failing_ticks(mut self, n: u32) -> Self {
        self.fail_ticks_remaining = n;
        self
    }
}

impl Subsystem for StubSubsystem {
    fn name(&self) -> &'static str {
        self.name
    }
    fn start(&mut self) -> Result<(), SubsystemError> {
        if self.fail_starts_remaining > 0 {
            self.fail_starts_remaining -= 1;
            return Err(SubsystemError(format!(
                "{}: simulated start failure",
                self.name
            )));
        }
        self.started = true;
        Ok(())
    }
    fn stop(&mut self) -> Result<(), SubsystemError> {
        self.started = false;
        Ok(())
    }
    fn health(&self) -> HealthStatus {
        if self.started {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        }
    }
    fn tick(&mut self) -> Result<(), SubsystemError> {
        self.ticks += 1;
        if self.fail_ticks_remaining > 0 {
            self.fail_ticks_remaining -= 1;
            return Err(SubsystemError(format!(
                "{}: simulated tick failure",
                self.name
            )));
        }
        Ok(())
    }
}
