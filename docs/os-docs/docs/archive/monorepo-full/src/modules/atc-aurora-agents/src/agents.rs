// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// 12 Agent implementations
use crate::agent_base::{Agent, AgentContext, AgentResponse};

macro_rules! impl_agent {
    ($id:expr, $name:ident, $role:expr, $caps:expr) => {
        pub struct $name;
        impl Agent for $name {
            fn id(&self) -> &str { $id }
            fn name(&self) -> &str { stringify!($name) }
            fn role(&self) -> &str { $role }
            fn capabilities(&self) -> Vec<String> { $caps.iter().map(|s| s.to_string()).collect() }
            fn execute(&self, ctx: AgentContext) -> AgentResponse {
                AgentResponse::success(&ctx.task_id,
                    &format!("[{}] Processing: {}", self.role(), &ctx.input[..ctx.input.len().min(50)])
                )
            }
        }
    };
}

impl_agent!("governance", GovernanceAgent, "Governance — proposal creation and voting", &["propose", "vote", "delegate"]);
impl_agent!("product", ProductAgent, "Product — roadmap and feature planning", &["plan", "prioritize", "spec"]);
impl_agent!("roadmap", RoadmapAgent, "Roadmap — sprint planning and milestones", &["plan_sprint", "track", "report"]);
impl_agent!("security", SecurityAgent, "Security — vulnerability scanning and audit", &["scan", "audit", "report"]);
impl_agent!("analytics", AnalyticsAgent, "Analytics — metrics and dashboards", &["collect", "analyze", "visualize"]);
impl_agent!("devops", DevOpsAgent, "DevOps — CI/CD and deployment", &["build", "deploy", "monitor"]);
impl_agent!("community", CommunityAgent, "Community — user engagement", &["respond", "moderate", "engage"]);
impl_agent!("treasury", TreasuryAgent, "Treasury — fund management", &["allocate", "report", "audit"]);
impl_agent!("audit", AuditAgent, "Audit — compliance verification", &["verify", "report", "flag"]);
impl_agent!("research", ResearchAgent, "Research — analysis and documentation", &["research", "document", "summarize"]);
impl_agent!("support", SupportAgent, "Support — user assistance", &["help", "guide", "escalate"]);
impl_agent!("bridge", BridgeAgent, "Bridge — cross-chain operations", &["relay", "verify", "settle"]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agents() {
        let gov = GovernanceAgent;
        assert_eq!(gov.id(), "governance");
        assert_eq!(gov.role(), "Governance — proposal creation and voting");
        assert_eq!(gov.capabilities().len(), 3);

        let ctx = AgentContext::new("task-1", "Create proposal for token burn");
        let resp = gov.execute(ctx);
        assert!(resp.success);
        assert!(resp.output.contains("Governance"));
    }

    #[test]
    fn test_all_agents() {
        let agents: Vec<Box<dyn Agent>> = vec![
            Box::new(GovernanceAgent), Box::new(ProductAgent), Box::new(RoadmapAgent),
            Box::new(SecurityAgent), Box::new(AnalyticsAgent), Box::new(DevOpsAgent),
            Box::new(CommunityAgent), Box::new(TreasuryAgent), Box::new(AuditAgent),
            Box::new(ResearchAgent), Box::new(SupportAgent), Box::new(BridgeAgent),
        ];
        assert_eq!(agents.len(), 12);
        for a in &agents {
            assert!(!a.id().is_empty());
            assert!(!a.capabilities().is_empty());
        }
    }
}
