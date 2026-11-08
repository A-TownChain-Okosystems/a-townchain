//! Proposal-Registry — die KI darf vorschlagen, niemals ausführen.
//! Lifecycle: Proposed -> Specified -> HandedToVM.
//! Deterministische Proposal-IDs; kein Execution-Pfad in dieser Crate.

use crate::audit::AuditPipeline;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProposalState {
    Proposed,
    Specified,
    HandedToVM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub agent: String,
    pub kind: String,
    pub payload: serde_json::Value,
    pub state: ProposalState,
}

#[derive(Debug, PartialEq)]
pub enum ProposalError {
    UnknownProposal(String),
    InvalidTransition {
        from: ProposalState,
        to: ProposalState,
    },
}

impl std::fmt::Display for ProposalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalError::UnknownProposal(id) => write!(f, "unknown proposal '{id}'"),
            ProposalError::InvalidTransition { from, to } => {
                write!(f, "invalid proposal transition {from:?} -> {to:?}")
            }
        }
    }
}

pub struct ProposalRegistry {
    proposals: BTreeMap<String, Proposal>,
    audit: AuditPipeline,
}

impl Default for ProposalRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ProposalRegistry {
    pub fn new() -> Self {
        Self {
            proposals: BTreeMap::new(),
            audit: AuditPipeline::new(),
        }
    }

    /// AI-Vorschlag einreichen — deterministische ID, Lifecycle-Start Proposed.
    /// WICHTIG: Diese Methode verändert KEINEN Chain-State; Vorschläge sind
    /// Daten, keine Ausführungen (AD-008 §7).
    pub fn submit(&mut self, agent: &str, kind: &str, payload: serde_json::Value) -> String {
        let payload_str = serde_json::to_string(&payload).unwrap_or_default();
        let id = crate::sha_hex(format!("{agent}|{kind}|{payload_str}").as_bytes());
        // Idempotenz: identischer Vorschlag -> identische ID, kein Duplikat.
        if !self.proposals.contains_key(&id) {
            self.proposals.insert(
                id.clone(),
                Proposal {
                    id: id.clone(),
                    agent: agent.into(),
                    kind: kind.into(),
                    payload,
                    state: ProposalState::Proposed,
                },
            );
            self.audit.record(agent, "proposal_submitted", &id);
        }
        id
    }

    pub fn get(&self, id: &str) -> Option<&Proposal> {
        self.proposals.get(id)
    }

    pub fn all(&self) -> Vec<&Proposal> {
        self.proposals.values().collect()
    }

    /// Lifecycle-Übergang Proposed -> Specified (ATCLang-Kanonisierung).
    pub fn mark_specified(&mut self, id: &str) -> Result<(), ProposalError> {
        let p = self
            .proposals
            .get_mut(id)
            .ok_or_else(|| ProposalError::UnknownProposal(id.to_string()))?;
        if p.state != ProposalState::Proposed {
            return Err(ProposalError::InvalidTransition {
                from: p.state,
                to: ProposalState::Specified,
            });
        }
        p.state = ProposalState::Specified;
        self.audit
            .record("proposal-registry", "proposal_specified", id);
        Ok(())
    }

    /// Lifecycle-Übergang Specified -> HandedToVM (Übergabe an die ATVM-Grenze).
    /// Danach ist die Verantwortung beim VM-/Consensus-Stack — hier endet die AI-Runtime.
    pub fn mark_handed_to_vm(&mut self, id: &str) -> Result<(), ProposalError> {
        let p = self
            .proposals
            .get_mut(id)
            .ok_or_else(|| ProposalError::UnknownProposal(id.to_string()))?;
        if p.state != ProposalState::Specified {
            return Err(ProposalError::InvalidTransition {
                from: p.state,
                to: ProposalState::HandedToVM,
            });
        }
        p.state = ProposalState::HandedToVM;
        self.audit
            .record("proposal-registry", "proposal_handed_to_vm", id);
        Ok(())
    }

    pub fn audit(&self) -> &AuditPipeline {
        &self.audit
    }
}
