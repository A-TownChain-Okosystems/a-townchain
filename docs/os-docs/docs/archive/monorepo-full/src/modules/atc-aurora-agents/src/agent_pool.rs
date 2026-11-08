// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Agent Pool — manages all 12 agents
use std::collections::HashMap;
use crate::agent_base::{Agent, AgentContext, AgentResponse};
use crate::agents::*;

pub struct AgentPool {
    agents: HashMap<String, Box<dyn Agent>>,
}

impl AgentPool {
    pub fn new() -> Self {
        let mut pool = Self { agents: HashMap::new() };
        pool.register(Box::new(GovernanceAgent));
        pool.register(Box::new(ProductAgent));
        pool.register(Box::new(RoadmapAgent));
        pool.register(Box::new(SecurityAgent));
        pool.register(Box::new(AnalyticsAgent));
        pool.register(Box::new(DevOpsAgent));
        pool.register(Box::new(CommunityAgent));
        pool.register(Box::new(TreasuryAgent));
        pool.register(Box::new(AuditAgent));
        pool.register(Box::new(ResearchAgent));
        pool.register(Box::new(SupportAgent));
        pool.register(Box::new(BridgeAgent));
        pool
    }

    pub fn register(&mut self, agent: Box<dyn Agent>) {
        let id = agent.id().to_string();
        self.agents.insert(id, agent);
    }

    pub fn execute(&self, agent_id: &str, ctx: AgentContext) -> Result<AgentResponse, String> {
        let agent = self.agents.get(agent_id)
            .ok_or_else(|| format!("Agent '{}' not found", agent_id))?;
        Ok(agent.execute(ctx))
    }

    pub fn list(&self) -> Vec<(&str, &str, &str)> {
        self.agents.values()
            .map(|a| (a.id(), a.name(), a.role()))
            .collect()
    }

    pub fn count(&self) -> usize { self.agents.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool() {
        let pool = AgentPool::new();
        assert_eq!(pool.count(), 12);
        let ctx = AgentContext::new("task-1", "test");
        assert!(pool.execute("governance", ctx).is_ok());
        assert!(pool.execute("nonexistent", AgentContext::new("x", "y")).is_err());
    }
}
