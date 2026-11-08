//! Resource Manager: Verbrauchs-Tracking + Quota-Enforcement pro Sandbox.
//! Deterministisch: kein Wall-Clock im Accounting — CPU wird als ms-Budget
//! pro Accounting-Periode (Tick) gebucht, das Zurücksetzen ist explizit.

use crate::capability::CapabilitySet;
use crate::SandboxId;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResourceUsage {
    pub cpu_ms_used: u64,
    pub memory_bytes_current: u64,
    pub storage_bytes_total: u64,
    pub network_bytes_out: u64,
    pub network_bytes_in: u64,
}

#[derive(Debug)]
pub enum ResourceError {
    QuotaExceeded {
        resource: &'static str,
        requested: u64,
        limit: u64,
        used: u64,
    },
    InvalidFree {
        attempted: u64,
        current: u64,
    },
}

impl std::fmt::Display for ResourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceError::QuotaExceeded {
                resource,
                requested,
                limit,
                used,
            } => write!(
                f,
                "{resource} quota exceeded: requested {requested}, used {used}, limit {limit}"
            ),
            ResourceError::InvalidFree { attempted, current } => write!(
                f,
                "invalid free: attempted {attempted}, current {current} (kein Negativsaldo)"
            ),
        }
    }
}

pub struct ResourceManager {
    usage: HashMap<SandboxId, ResourceUsage>,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self {
            usage: HashMap::new(),
        }
    }

    fn entry(&mut self, id: &str) -> &mut ResourceUsage {
        self.usage.entry(id.to_string()).or_default()
    }

    pub fn usage(&self, id: &str) -> ResourceUsage {
        self.usage.get(id).cloned().unwrap_or_default()
    }

    /// CPU-Budget innerhalb des aktuellen Ticks buchen.
    pub fn consume_cpu(
        &mut self,
        id: &str,
        ms: u64,
        cap: &CapabilitySet,
    ) -> Result<(), ResourceError> {
        let u = self.usage(id);
        let used = u.cpu_ms_used;
        let limit = cap.cpu_ms_per_tick;
        if used + ms > limit {
            return Err(ResourceError::QuotaExceeded {
                resource: "cpu",
                requested: ms,
                limit,
                used,
            });
        }
        self.entry(id).cpu_ms_used += ms;
        Ok(())
    }

    /// Speicher allozieren (aktueller Stand steigt, Peak wird über Quota begrenzt).
    pub fn alloc_memory(
        &mut self,
        id: &str,
        bytes: u64,
        cap: &CapabilitySet,
    ) -> Result<(), ResourceError> {
        let u = self.usage(id);
        let used = u.memory_bytes_current;
        let limit = cap.memory_bytes;
        if used + bytes > limit {
            return Err(ResourceError::QuotaExceeded {
                resource: "memory",
                requested: bytes,
                limit,
                used,
            });
        }
        self.entry(id).memory_bytes_current += bytes;
        Ok(())
    }

    /// Speicher freigeben — kein Negativsaldo (fail-closed).
    pub fn free_memory(&mut self, id: &str, bytes: u64) -> Result<(), ResourceError> {
        let current = self.usage(id).memory_bytes_current;
        if bytes > current {
            return Err(ResourceError::InvalidFree {
                attempted: bytes,
                current,
            });
        }
        self.entry(id).memory_bytes_current -= bytes;
        Ok(())
    }

    /// Storage wächst append-only — Quota begrenzt Gesamtsumme.
    pub fn consume_storage(
        &mut self,
        id: &str,
        bytes: u64,
        cap: &CapabilitySet,
    ) -> Result<(), ResourceError> {
        let u = self.usage(id);
        let used = u.storage_bytes_total;
        let limit = cap.storage_bytes;
        if used + bytes > limit {
            return Err(ResourceError::QuotaExceeded {
                resource: "storage",
                requested: bytes,
                limit,
                used,
            });
        }
        self.entry(id).storage_bytes_total += bytes;
        Ok(())
    }

    /// Netzwerk-Bytes buchen (Policy-Gate passiert davor in der Sandbox).
    pub fn record_network_out(&mut self, id: &str, bytes: u64) {
        self.entry(id).network_bytes_out += bytes;
    }

    pub fn record_network_in(&mut self, id: &str, bytes: u64) {
        self.entry(id).network_bytes_in += bytes;
    }

    /// Neuer Accounting-Tick: CPU-Budget wird zurückgesetzt (explizit, deterministisch).
    pub fn reset_tick(&mut self) {
        for u in self.usage.values_mut() {
            u.cpu_ms_used = 0;
        }
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new()
    }
}
