//! Sandbox: Ausführung hinter dem Policy-Gate.
//! Reihenfolge VERBINDLICH: Policy-Check -> Ressourcen-Verbrauch -> Execution.
//! Fail-closed: jede abgelehnte Operation ist ein Fehler, keine Teil-Ausführung.

use crate::capability::CapabilitySet;
use crate::resources::{ResourceError, ResourceManager};
use crate::SandboxId;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub enum Operation {
    Compute { cpu_ms: u64 },
    Alloc { bytes: u64 },
    Free { bytes: u64 },
    Store { bytes: u64 },
    NetworkOut { bytes: u64, port: u16 },
    NetworkIn { bytes: u64, port: u16 },
    Syscall { name: String },
    Sign { hash: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExecOutcome {
    Computed { cpu_ms: u64 },
    Allocated { bytes: u64 },
    Freed { bytes: u64 },
    Stored { bytes: u64 },
    NetworkSent { bytes: u64, port: u16 },
    NetworkReceived { bytes: u64, port: u16 },
    Syscalled { name: String },
    Signed { signature: String },
}

#[derive(Debug)]
pub enum SandboxError {
    Denied { reason: String },
    Resource(ResourceError),
}

impl std::fmt::Display for SandboxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SandboxError::Denied { reason } => write!(f, "DENIED: {reason}"),
            SandboxError::Resource(e) => write!(f, "RESOURCE: {e}"),
        }
    }
}

impl From<ResourceError> for SandboxError {
    fn from(e: ResourceError) -> Self {
        SandboxError::Resource(e)
    }
}

pub struct Sandbox {
    pub id: SandboxId,
    pub capability: CapabilitySet,
}

impl Sandbox {
    pub fn new(id: &str, capability: CapabilitySet) -> Self {
        Self {
            id: id.to_string(),
            capability,
        }
    }

    /// Ausführungskette: Policy -> Resource -> Execute.
    pub fn execute(
        &self,
        op: Operation,
        resources: &mut ResourceManager,
    ) -> Result<ExecOutcome, SandboxError> {
        match op {
            Operation::Compute { cpu_ms } => {
                resources.consume_cpu(&self.id, cpu_ms, &self.capability)?;
                Ok(ExecOutcome::Computed { cpu_ms })
            }
            Operation::Alloc { bytes } => {
                resources.alloc_memory(&self.id, bytes, &self.capability)?;
                Ok(ExecOutcome::Allocated { bytes })
            }
            Operation::Free { bytes } => {
                resources.free_memory(&self.id, bytes)?;
                Ok(ExecOutcome::Freed { bytes })
            }
            Operation::Store { bytes } => {
                resources.consume_storage(&self.id, bytes, &self.capability)?;
                Ok(ExecOutcome::Stored { bytes })
            }
            Operation::NetworkOut { bytes, port } => {
                if !self.capability.network.allows_outbound(port) {
                    return Err(SandboxError::Denied {
                        reason: format!("outbound port {port} not allowed (deny by default)"),
                    });
                }
                resources.record_network_out(&self.id, bytes);
                Ok(ExecOutcome::NetworkSent { bytes, port })
            }
            Operation::NetworkIn { bytes, port } => {
                if !self.capability.network.allows_inbound(port) {
                    return Err(SandboxError::Denied {
                        reason: format!("inbound port {port} not allowed (deny by default)"),
                    });
                }
                resources.record_network_in(&self.id, bytes);
                Ok(ExecOutcome::NetworkReceived { bytes, port })
            }
            Operation::Syscall { name } => {
                if !self.capability.allows_syscall(&name) {
                    return Err(SandboxError::Denied {
                        reason: format!("syscall '{name}' not in allowlist (deny by default)"),
                    });
                }
                Ok(ExecOutcome::Syscalled { name })
            }
            Operation::Sign { hash } => {
                if !self.capability.crypto {
                    return Err(SandboxError::Denied {
                        reason: "crypto capability not granted".into(),
                    });
                }
                // Deterministische Pseudo-Signatur; der echte Keyring folgt in S16-S18
                // (keys verlassen die Keyring-Boundary NIE — KAI-CORE-RUNTIME-001 §7).
                let sig = Sha256::digest(format!("sig|{hash}").as_bytes());
                let signature: String = sig.iter().map(|b| format!("{b:02x}")).collect();
                Ok(ExecOutcome::Signed { signature })
            }
        }
    }
}
