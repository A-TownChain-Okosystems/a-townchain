// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Agent Registry — stable identities, duplicate protection and durable registry snapshots.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityMetadata {
    pub name: String,
    pub description: String,
    /// Informational classification only; authorization is enforced by CapabilityPolicyEngine.
    pub risk_level: String,
}

impl CapabilityMetadata {
    pub fn new(name: &str, description: &str, risk_level: &str) -> Self {
        Self {
            name: name.trim().into(),
            description: description.trim().into(),
            risk_level: risk_level.trim().into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub role: String,
    pub capabilities: Vec<String>,
    pub capability_metadata: Vec<CapabilityMetadata>,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    EmptyId,
    InvalidId(String),
    EmptyName,
    EmptyRole,
    DuplicateId(String),
    EmptyCapability,
    DuplicateCapability(String),
    InvalidCapabilityMetadata(String),
    AgentNotFound(String),
    Persistence(String),
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AgentRegistry {
    agents: HashMap<String, AgentInfo>,
}

impl AgentRegistry {
    fn valid_id(id: &str) -> bool {
        !id.is_empty()
            && id.as_bytes()[0].is_ascii_alphanumeric()
            && id.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'_' | b'-' | b'.')
            })
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// Generates a collision-resistant UUIDv4 once. Callers must persist the returned ID
    /// and reuse it after restart; a new process/run must not silently mint a replacement.
    pub fn register_generated(
        &mut self,
        name: &str,
        role: &str,
        caps: Vec<String>,
    ) -> Result<String, RegistryError> {
        let id = format!("agent:{}", Uuid::new_v4());
        self.register(&id, name, role, caps)?;
        Ok(id)
    }

    /// Register an agent with an explicit stable ID. IDs are never silently overwritten.
    pub fn register(
        &mut self,
        id: &str,
        name: &str,
        role: &str,
        caps: Vec<String>,
    ) -> Result<(), RegistryError> {
        let metadata = caps
            .iter()
            .map(|cap| CapabilityMetadata::new(cap, "", "unspecified"))
            .collect();
        self.register_with_metadata(id, name, role, metadata)
    }

    pub fn register_with_metadata(
        &mut self,
        id: &str,
        name: &str,
        role: &str,
        metadata: Vec<CapabilityMetadata>,
    ) -> Result<(), RegistryError> {
        let id = id.trim();
        let name = name.trim();
        let role = role.trim();
        if id.is_empty() {
            return Err(RegistryError::EmptyId);
        }
        if !Self::valid_id(id) {
            return Err(RegistryError::InvalidId(id.into()));
        }
        if name.is_empty() {
            return Err(RegistryError::EmptyName);
        }
        if role.is_empty() {
            return Err(RegistryError::EmptyRole);
        }
        if self.agents.contains_key(id) {
            return Err(RegistryError::DuplicateId(id.into()));
        }

        let mut seen = HashSet::new();
        let mut capabilities = Vec::with_capacity(metadata.len());
        for item in &metadata {
            if item.name.trim().is_empty() {
                return Err(RegistryError::EmptyCapability);
            }
            if !seen.insert(item.name.trim().to_owned()) {
                return Err(RegistryError::DuplicateCapability(item.name.clone()));
            }
            if item.name != item.name.trim()
                || item.description != item.description.trim()
                || item.risk_level != item.risk_level.trim()
            {
                return Err(RegistryError::InvalidCapabilityMetadata(item.name.clone()));
            }
            capabilities.push(item.name.clone());
        }

        self.agents.insert(
            id.into(),
            AgentInfo {
                id: id.into(),
                name: name.into(),
                role: role.into(),
                capabilities,
                capability_metadata: metadata,
                is_active: true,
            },
        );
        Ok(())
    }

    /// Mark an agent inactive without deleting its identity or audit attribution.
    pub fn revoke(&mut self, id: &str) -> Result<(), RegistryError> {
        self.set_active(id, false)
    }

    /// Retire an identity without deleting its tombstone, preventing accidental ID reuse.
    pub fn unregister(&mut self, id: &str) -> Result<AgentInfo, RegistryError> {
        self.set_active(id, false)?;
        self.agents
            .get(id)
            .cloned()
            .ok_or_else(|| RegistryError::AgentNotFound(id.into()))
    }

    pub fn get_agent(&self, id: &str) -> Option<&AgentInfo> {
        self.agents.get(id)
    }

    pub fn is_active(&self, id: &str) -> bool {
        self.agents.get(id).is_some_and(|agent| agent.is_active)
    }

    pub fn list_active(&self) -> Vec<&AgentInfo> {
        let mut agents: Vec<_> = self.agents.values().filter(|a| a.is_active).collect();
        agents.sort_by(|left, right| left.id.cmp(&right.id));
        agents
    }

    pub fn count(&self) -> usize {
        self.agents.len()
    }

    pub fn set_active(&mut self, id: &str, active: bool) -> Result<(), RegistryError> {
        let agent = self
            .agents
            .get_mut(id)
            .ok_or_else(|| RegistryError::AgentNotFound(id.into()))?;
        agent.is_active = active;
        Ok(())
    }

    /// Persist IDs and revocation state. The caller controls the path, permissions and
    /// backup policy; production deployments should use an access-controlled durable store.
    pub fn save_json<P: AsRef<Path>>(&self, path: P) -> Result<(), RegistryError> {
        let path = path.as_ref();
        let bytes = serde_json::to_vec_pretty(self)
            .map_err(|error| RegistryError::Persistence(error.to_string()))?;
        let temp_path = path.with_extension(format!("tmp-{}", Uuid::new_v4()));
        fs::write(&temp_path, bytes)
            .map_err(|error| RegistryError::Persistence(error.to_string()))?;
        fs::rename(&temp_path, path).map_err(|error| {
            let _ = fs::remove_file(&temp_path);
            RegistryError::Persistence(error.to_string())
        })
    }

    pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Self, RegistryError> {
        let bytes =
            fs::read(path).map_err(|error| RegistryError::Persistence(error.to_string()))?;
        let registry: Self = serde_json::from_slice(&bytes)
            .map_err(|error| RegistryError::Persistence(error.to_string()))?;
        for (key, agent) in &registry.agents {
            if key != &agent.id || !Self::valid_id(key) {
                return Err(RegistryError::Persistence(
                    "registry snapshot contains an invalid or mismatched agent identity".into(),
                ));
            }
        }
        Ok(registry)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn registers_capabilities_and_metadata() {
        let mut registry = AgentRegistry::new();
        registry
            .register_with_metadata(
                "agent:research-1",
                "Research Agent",
                "research",
                vec![CapabilityMetadata::new(
                    "web.search",
                    "Search approved public sources",
                    "low",
                )],
            )
            .unwrap();
        let agent = registry.get_agent("agent:research-1").unwrap();
        assert_eq!(agent.capabilities, vec!["web.search"]);
        assert_eq!(agent.capability_metadata[0].risk_level, "low");
    }

    #[test]
    fn duplicate_id_is_rejected_without_overwrite() {
        let mut registry = AgentRegistry::new();
        registry
            .register("agent:one", "First", "research", vec![])
            .unwrap();
        assert_eq!(
            registry.register("agent:one", "Second", "coding", vec![]),
            Err(RegistryError::DuplicateId("agent:one".into()))
        );
        assert_eq!(registry.get_agent("agent:one").unwrap().name, "First");
        assert_eq!(registry.count(), 1);
    }

    #[test]
    fn generated_ids_are_unique_and_have_agent_namespace() {
        let mut registry = AgentRegistry::new();
        let first = registry
            .register_generated("First", "research", vec![])
            .unwrap();
        let second = registry
            .register_generated("Second", "coding", vec![])
            .unwrap();
        assert!(first.starts_with("agent:"));
        assert!(second.starts_with("agent:"));
        assert_ne!(first, second);
        assert!(registry.is_active(&first));
        assert!(registry.is_active(&second));
    }

    #[test]
    fn concurrent_registration_produces_distinct_ids() {
        let registry = Arc::new(Mutex::new(AgentRegistry::new()));
        let workers: Vec<_> = (0..16)
            .map(|index| {
                let registry = Arc::clone(&registry);
                thread::spawn(move || {
                    registry
                        .lock()
                        .unwrap()
                        .register_generated(&format!("agent-{index}"), "worker", vec![])
                        .unwrap()
                })
            })
            .collect();
        let ids: HashSet<_> = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect();
        assert_eq!(ids.len(), 16);
    }

    #[test]
    fn persistence_preserves_identity_and_revocation_across_reload() {
        let path =
            std::env::temp_dir().join(format!("aurora-agent-registry-{}.json", Uuid::new_v4()));
        let mut registry = AgentRegistry::new();
        let id = registry
            .register_generated("Persistent", "worker", vec![])
            .unwrap();
        registry.revoke(&id).unwrap();
        registry.save_json(&path).unwrap();

        let restored = AgentRegistry::load_json(&path).unwrap();
        let _ = fs::remove_file(&path);
        assert_eq!(restored.get_agent(&id).unwrap().id, id);
        assert!(!restored.is_active(&id));
        assert_eq!(restored.count(), 1);
    }

    #[test]
    fn retired_ids_cannot_be_reused_and_invalid_id_syntax_is_rejected() {
        let mut registry = AgentRegistry::new();
        registry
            .register("agent:retired", "Retired", "worker", vec![])
            .unwrap();
        let retired = registry.unregister("agent:retired").unwrap();
        assert!(!retired.is_active);
        assert_eq!(
            registry.register("agent:retired", "Replacement", "worker", vec![]),
            Err(RegistryError::DuplicateId("agent:retired".into()))
        );
        assert_eq!(
            registry.register("agent with spaces", "Invalid", "worker", vec![]),
            Err(RegistryError::InvalidId("agent with spaces".into()))
        );
    }

    #[test]
    fn invalid_identity_and_duplicate_capabilities_are_rejected() {
        let mut registry = AgentRegistry::new();
        assert_eq!(
            registry.register(" ", "Agent", "role", vec![]),
            Err(RegistryError::EmptyId)
        );
        assert_eq!(
            registry.register(
                "agent:dup",
                "Agent",
                "role",
                vec!["read".into(), "read".into()]
            ),
            Err(RegistryError::DuplicateCapability("read".into()))
        );
        assert_eq!(registry.count(), 0);
    }

    #[test]
    fn active_listing_is_deterministic_and_unknown_agent_errors() {
        let mut registry = AgentRegistry::new();
        registry.register("agent:z", "Z", "role", vec![]).unwrap();
        registry.register("agent:a", "A", "role", vec![]).unwrap();
        registry.revoke("agent:z").unwrap();
        assert_eq!(registry.list_active()[0].id, "agent:a");
        assert!(!registry.is_active("agent:z"));
        assert_eq!(
            registry.set_active("missing", true),
            Err(RegistryError::AgentNotFound("missing".into()))
        );
    }
}
