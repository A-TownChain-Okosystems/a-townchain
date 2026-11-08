// Agent Registry — manages registered AI agents
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub role: String,
    pub capabilities: Vec<String>,
    pub is_active: bool,
}

pub struct AgentRegistry {
    agents: HashMap<String, AgentInfo>,
}

impl AgentRegistry {
    pub fn new() -> Self { Self { agents: HashMap::new() } }

    pub fn register(&mut self, id: &str, name: &str, role: &str, caps: Vec<String>) {
        self.agents.insert(id.into(), AgentInfo {
            id: id.into(), name: name.into(), role: role.into(),
            capabilities: caps, is_active: true,
        });
    }

    pub fn unregister(&mut self, id: &str) { self.agents.remove(id); }
    pub fn get_agent(&self, id: &str) -> Option<&AgentInfo> { self.agents.get(id) }
    pub fn list_active(&self) -> Vec<&AgentInfo> {
        self.agents.values().filter(|a| a.is_active).collect()
    }
    pub fn count(&self) -> usize { self.agents.len() }
    pub fn set_active(&mut self, id: &str, active: bool) {
        if let Some(a) = self.agents.get_mut(id) { a.is_active = active; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry() {
        let mut reg = AgentRegistry::new();
        reg.register("governance", "Governance Agent", "governance", vec!["propose".into()]);
        assert_eq!(reg.count(), 1);
        assert!(reg.get_agent("governance").is_some());
        reg.set_active("governance", false);
        assert_eq!(reg.list_active().len(), 0);
    }
}
