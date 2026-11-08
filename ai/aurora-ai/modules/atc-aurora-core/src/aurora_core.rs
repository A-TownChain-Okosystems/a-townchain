// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Central engine coordinator. Authorization uses the existing agent registry plus the
// fail-closed policy engine; this layer is not a substitute for GlobusOS/ShivaCore enforcement.

use crate::{
    ActionRequest, AgentRegistry, CapabilityPolicyEngine, ConfigManager, LlmRouter, ModelHub,
    PolicyDecision,
};

pub struct AuroraCore {
    pub model_hub: ModelHub,
    pub llm_router: LlmRouter,
    pub agent_registry: AgentRegistry,
    pub config: ConfigManager,
    pub capability_policy: CapabilityPolicyEngine,
    pub active: bool,
}

impl Default for AuroraCore {
    fn default() -> Self {
        Self::new()
    }
}

impl AuroraCore {
    pub fn new() -> Self {
        Self {
            model_hub: ModelHub::new(),
            llm_router: LlmRouter::new(),
            agent_registry: AgentRegistry::new(),
            config: ConfigManager::new(),
            capability_policy: CapabilityPolicyEngine::new(),
            active: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.active {
            return Err("AuroraCore already running".into());
        }
        self.active = true;
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if !self.active {
            return Err("AuroraCore not running".into());
        }
        self.active = false;
        Ok(())
    }

    /// Check the existing agent identity/capability registry before evaluating policy.
    /// The final OS/kernel service must independently enforce its own capabilities.
    pub fn authorize_action(&self, agent_id: &str, request: &ActionRequest) -> PolicyDecision {
        if !self.active || request.principal != agent_id {
            return PolicyDecision::Deny;
        }

        let Some(agent) = self.agent_registry.get_agent(agent_id) else {
            return PolicyDecision::Deny;
        };
        if !agent.is_active
            || !agent
                .capabilities
                .iter()
                .any(|cap| cap == &request.capability)
        {
            return PolicyDecision::Deny;
        }

        self.capability_policy.evaluate(request)
    }

    pub fn process_request(&self, agent_id: &str, prompt: &str) -> Result<String, String> {
        if !self.active {
            return Err("AuroraCore not active".into());
        }
        let agent = self
            .agent_registry
            .get_agent(agent_id)
            .ok_or_else(|| format!("Agent {} not registered", agent_id))?;
        if !agent.is_active {
            return Err(format!("Agent {} is inactive", agent_id));
        }

        let model = self.llm_router.route(prompt);
        let response = self.model_hub.inference(&model, prompt);
        Ok(response)
    }

    pub fn is_active(&self) -> bool {
        self.active
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CapabilityPolicy;

    fn request() -> ActionRequest {
        ActionRequest {
            request_id: "request:core-1".into(),
            principal: "agent:assistant".into(),
            capability: "filesystem.read".into(),
            action: "read_file".into(),
            scope: "workspace:project-a".into(),
            approval: None,
        }
    }

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

    #[test]
    fn process_request_rejects_inactive_agent() {
        let mut core = AuroraCore::new();
        core.start().unwrap();
        core.agent_registry
            .register("agent:assistant", "Assistant", "assistant", vec![])
            .unwrap();
        core.agent_registry
            .set_active("agent:assistant", false)
            .unwrap();
        assert!(core
            .process_request("agent:assistant", "hello")
            .unwrap_err()
            .contains("inactive"));
    }

    #[test]
    fn action_authorization_reuses_agent_registry_and_policy_engine() {
        let mut core = AuroraCore::new();
        core.start().unwrap();
        core.agent_registry
            .register(
                "agent:assistant",
                "Assistant",
                "assistant",
                vec!["filesystem.read".into()],
            )
            .unwrap();
        core.capability_policy.register(CapabilityPolicy::new(
            "agent:assistant",
            "filesystem.read",
            "read_file",
            "workspace:project-a",
            false,
        ));

        assert_eq!(
            core.authorize_action("agent:assistant", &request()),
            PolicyDecision::Allow
        );
        assert_eq!(
            core.authorize_action("agent:other", &request()),
            PolicyDecision::Deny
        );
    }

    #[test]
    fn action_authorization_denies_missing_agent_capability() {
        let mut core = AuroraCore::new();
        core.start().unwrap();
        core.agent_registry
            .register(
                "agent:assistant",
                "Assistant",
                "assistant",
                vec!["context.read".into()],
            )
            .unwrap();
        core.capability_policy.register(CapabilityPolicy::new(
            "agent:assistant",
            "filesystem.read",
            "read_file",
            "workspace:project-a",
            false,
        ));

        assert_eq!(
            core.authorize_action("agent:assistant", &request()),
            PolicyDecision::Deny
        );
    }
}
