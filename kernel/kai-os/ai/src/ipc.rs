//! IPC-Gateway (KAI-CORE-RUNTIME-001 §7): standardisierte Schnittstelle mit
//! Identity / Auth / Schema / Replay-Schutz / Audit — KEIN Message-Queue.
//! Deny by default: unbekannte Agents und nicht freigegebene Nachrichtentypen
//! werden abgewiesen.

use crate::audit::AuditPipeline;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Nachrichtentypen mit Pflichtfeldern im Payload (fail-closed Schema).
pub const SCHEMAS: &[(&str, &[&str])] = &[
    ("propose_tx", &["action", "params"]),
    ("propose_decision", &["topic", "recommendation"]),
    ("query_state", &["key"]),
    ("register_model", &["model_id", "version"]),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub from_agent: String,
    pub msg_type: String,
    pub seq: u64,
    pub payload: serde_json::Value,
}

#[derive(Debug, PartialEq)]
pub enum IpcError {
    UnknownAgent(String),
    CapabilityDenied { agent: String, msg_type: String },
    Replay { seq: u64, last: u64 },
    SchemaViolation { msg_type: String, missing: String },
}

impl std::fmt::Display for IpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IpcError::UnknownAgent(a) => write!(f, "unknown agent '{a}' (deny by default)"),
            IpcError::CapabilityDenied { agent, msg_type } => {
                write!(
                    f,
                    "agent '{agent}' lacks capability for '{msg_type}' (deny by default)"
                )
            }
            IpcError::Replay { seq, last } => write!(f, "ipc replay: seq {seq} <= last {last}"),
            IpcError::SchemaViolation { msg_type, missing } => {
                write!(
                    f,
                    "schema violation for '{msg_type}': missing field '{missing}' (fail-closed)"
                )
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryReceipt {
    pub seq: u64,
    pub audit_seq: u64,
}

pub struct IpcGateway {
    /// Agent-ID -> erlaubte Nachrichtentypen.
    capabilities: HashMap<String, Vec<String>>,
    /// Agent-ID -> letzte Sequenznummer (Replay-Schutz).
    last_seq: HashMap<String, u64>,
    audit: AuditPipeline,
}

impl Default for IpcGateway {
    fn default() -> Self {
        Self::new()
    }
}

impl IpcGateway {
    pub fn new() -> Self {
        Self {
            capabilities: HashMap::new(),
            last_seq: HashMap::new(),
            audit: AuditPipeline::new(),
        }
    }

    /// Agent registrieren mit Capability-Set (explizit, deny by default).
    pub fn register_agent(&mut self, agent_id: &str, msg_types: &[&str]) {
        self.capabilities.insert(
            agent_id.to_string(),
            msg_types.iter().map(|s| s.to_string()).collect(),
        );
        self.audit
            .record("ipc-gateway", "agent_registered", agent_id);
    }

    pub fn audit(&self) -> &AuditPipeline {
        &self.audit
    }

    /// Nachrichten-Schema: Pflichtfelder prüfen (fail-closed).
    fn check_schema(msg: &IpcMessage) -> Result<(), IpcError> {
        let schema = SCHEMAS.iter().find(|(t, _)| *t == msg.msg_type);
        let (_, required) = match schema {
            Some(s) => s,
            None => return Ok(()), // unbekannter Typ wird vorher von Capability abgewiesen
        };
        for field in required.iter() {
            if msg.payload.get(*field).is_none() {
                return Err(IpcError::SchemaViolation {
                    msg_type: msg.msg_type.clone(),
                    missing: field.to_string(),
                });
            }
        }
        Ok(())
    }

    /// Nachricht zustellen: Identity -> Capability -> Replay -> Schema -> Audit.
    /// Reihenfolge verbindlich, fail-closed, jeder Schritt auditierbar.
    pub fn deliver(&mut self, msg: &IpcMessage) -> Result<DeliveryReceipt, IpcError> {
        // 1. Identity (deny by default)
        let caps = match self.capabilities.get(&msg.from_agent) {
            Some(c) => c,
            None => {
                self.audit
                    .record("ipc-gateway", "rejected_unknown_agent", &msg.from_agent);
                return Err(IpcError::UnknownAgent(msg.from_agent.clone()));
            }
        };
        // 2. Capability (deny by default)
        if !caps.contains(&msg.msg_type) {
            self.audit.record(
                "ipc-gateway",
                "rejected_capability",
                &format!("{}|{}", msg.from_agent, msg.msg_type),
            );
            return Err(IpcError::CapabilityDenied {
                agent: msg.from_agent.clone(),
                msg_type: msg.msg_type.clone(),
            });
        }
        // 3. Replay-Schutz: strikt monoton pro Agent
        let last = self.last_seq.get(&msg.from_agent).copied().unwrap_or(0);
        if msg.seq <= last {
            self.audit.record(
                "ipc-gateway",
                "rejected_replay",
                &format!("{}|{}", msg.from_agent, msg.seq),
            );
            return Err(IpcError::Replay { seq: msg.seq, last });
        }
        // 4. Schema-Validierung (fail-closed)
        if let Err(e) = Self::check_schema(msg) {
            self.audit.record(
                "ipc-gateway",
                "rejected_schema",
                &format!("{}|{}", msg.from_agent, msg.msg_type),
            );
            return Err(e);
        }
        // 5. Zustellung + Audit
        self.last_seq.insert(msg.from_agent.clone(), msg.seq);
        let payload = serde_json::to_string(&msg.payload).unwrap_or_default();
        let audit_seq = self.audit.record(&msg.from_agent, &msg.msg_type, &payload);
        Ok(DeliveryReceipt {
            seq: msg.seq,
            audit_seq,
        })
    }
}
