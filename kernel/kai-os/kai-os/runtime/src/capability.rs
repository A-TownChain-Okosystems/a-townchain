//! Capability Policy — deny by default.
//! Ein Agent erhält NIE unbegrenzte Ressourcen; jede Freigabe ist explizit.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    /// Erlaubte Ports für ausgehende Verbindungen (leer = komplett verboten).
    pub allow_outbound: Vec<u16>,
    /// Erlaubte Ports für eingehende Verbindungen (leer = komplett verboten).
    pub allow_inbound: Vec<u16>,
}

impl NetworkPolicy {
    pub fn closed() -> Self {
        Self {
            allow_outbound: Vec::new(),
            allow_inbound: Vec::new(),
        }
    }
    pub fn allows_outbound(&self, port: u16) -> bool {
        self.allow_outbound.contains(&port)
    }
    pub fn allows_inbound(&self, port: u16) -> bool {
        self.allow_inbound.contains(&port)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySet {
    pub cpu_ms_per_tick: u64,
    pub memory_bytes: u64,
    pub storage_bytes: u64,
    pub network: NetworkPolicy,
    /// Syscall-Allowlist (leer = alle verboten).
    pub syscalls: Vec<String>,
    /// Kryptografische Capability (Signieren) — nur mit expliziter Freigabe.
    pub crypto: bool,
}

#[derive(Debug)]
pub struct CapabilityError(pub String);

impl std::fmt::Display for CapabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "capability invalid: {}", self.0)
    }
}

impl CapabilitySet {
    /// Fail-closed: Quotas müssen > 0 sein; Netz/Syscalls sind ohne
    /// explizite Freigabe geschlossen.
    pub fn validate(&self) -> Result<(), CapabilityError> {
        if self.cpu_ms_per_tick == 0 {
            return Err(CapabilityError("cpu_ms_per_tick must be > 0".into()));
        }
        if self.memory_bytes == 0 {
            return Err(CapabilityError("memory_bytes must be > 0".into()));
        }
        if self.storage_bytes == 0 {
            return Err(CapabilityError("storage_bytes must be > 0".into()));
        }
        Ok(())
    }

    /// Deny-by-default-Basis-Capability: keine Syscalls, kein Netz, kein Crypto.
    pub fn builder() -> CapabilityBuilder {
        CapabilityBuilder::default()
    }

    pub fn allows_syscall(&self, name: &str) -> bool {
        self.syscalls.iter().any(|s| s == name)
    }
}

pub struct CapabilityBuilder {
    cpu_ms_per_tick: u64,
    memory_bytes: u64,
    storage_bytes: u64,
    network: NetworkPolicy,
    syscalls: Vec<String>,
    crypto: bool,
}

impl Default for CapabilityBuilder {
    fn default() -> Self {
        Self {
            cpu_ms_per_tick: 100,
            memory_bytes: 64 * 1024 * 1024,
            storage_bytes: 1024 * 1024 * 1024,
            network: NetworkPolicy::closed(),
            syscalls: Vec::new(),
            crypto: false,
        }
    }
}

impl CapabilityBuilder {
    pub fn cpu_ms(mut self, ms: u64) -> Self {
        self.cpu_ms_per_tick = ms;
        self
    }
    pub fn memory(mut self, bytes: u64) -> Self {
        self.memory_bytes = bytes;
        self
    }
    pub fn storage(mut self, bytes: u64) -> Self {
        self.storage_bytes = bytes;
        self
    }
    pub fn allow_outbound(mut self, port: u16) -> Self {
        self.network.allow_outbound.push(port);
        self
    }
    pub fn allow_inbound(mut self, port: u16) -> Self {
        self.network.allow_inbound.push(port);
        self
    }
    pub fn allow_syscall(mut self, name: &str) -> Self {
        self.syscalls.push(name.into());
        self
    }
    pub fn allow_crypto(mut self) -> Self {
        self.crypto = true;
        self
    }

    /// Baut und validiert (fail-closed).
    pub fn build(self) -> Result<CapabilitySet, CapabilityError> {
        let cs = CapabilitySet {
            cpu_ms_per_tick: self.cpu_ms_per_tick,
            memory_bytes: self.memory_bytes,
            storage_bytes: self.storage_bytes,
            network: self.network,
            syscalls: self.syscalls,
            crypto: self.crypto,
        };
        cs.validate()?;
        Ok(cs)
    }
}
