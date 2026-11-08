// Central engine coordinator
use std::collections::HashMap;
use crate::{ModelHub, LlmRouter, AgentRegistry, ConfigManager};

pub struct AuroraCore {
    pub model_hub: ModelHub,
    pub llm_router: LlmRouter,
    pub agent_registry: AgentRegistry,
    pub config: ConfigManager,
    pub active: bool,
}

impl AuroraCore {
    pub fn new() -> Self {
        Self {
            model_hub: ModelHub::new(),
            llm_router: LlmRouter::new(),
            agent_registry: AgentRegistry::new(),
            config: ConfigManager::new(),
            active: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.active { return Err("AuroraCore already running".into()); }
        self.active = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if !self.active { return Err("AuroraCore not running".into()); }
        self.active = false;
        Ok(())
    }

    pub fn process_request(&self, agent_id: &str, prompt: &str) -> Result<String, String> {
        if !self.active { return Err("AuroraCore not active".into()); }
        self.agent_registry.get_agent(agent_id)
            .ok_or_else(|| format!("Agent {} not registered", agent_id))?;
        let model = self.llm_router.route(prompt);
        let response = self.model_hub.inference(&model, prompt);
        Ok(response)
    }

    pub fn is_active(&self) -> bool { self.active }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aurora_core_lifecycle() {
        let mut core = AuroraCore::new();
        assert!(!core.is_active());
        assert!(core.start().is_ok());
        assert!(core.is_active());
        assert!(core.start().is_err());
        assert!(core.stop().is_ok());
        assert!(!core.is_active());
    }

    #[test]
    fn test_process_request_inactive() {
        let core = AuroraCore::new();
        assert!(core.process_request("test", "hello").is_err());
    }
}
