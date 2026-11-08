use crate::{
    AgentId, ApprovalId, ApprovalState, AuroraError, AuroraRequest, CapabilityId, Decision,
    ModelId, PolicyId, RequestStatus, RiskLevel, SkillId, ToolId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityResult {
    pub decision: Decision,
    pub reason: String,
    pub policy_id: Option<PolicyId>,
    pub approval_id: Option<ApprovalId>,
    pub risk: RiskLevel,
}

pub trait Capability {
    fn id(&self) -> &CapabilityId;
    fn evaluate(&self, principal: &str) -> CapabilityResult;
}

pub trait Policy {
    fn id(&self) -> &PolicyId;
    fn evaluate(&self, capability: &CapabilityId, principal: &str) -> Decision;
}

pub trait Approval {
    fn request(&self, reason: &str) -> Result<ApprovalId, AuroraError>;
    fn status(&self, approval_id: &ApprovalId) -> ApprovalState;
}

pub trait Agent {
    fn id(&self) -> &AgentId;
    fn execute(&self, request: &AuroraRequest) -> Result<AgentResult, AuroraError>;
}

pub trait Skill {
    fn id(&self) -> &SkillId;
}

pub trait Tool {
    fn id(&self) -> &ToolId;
    fn execute(&self, request: &ToolRequest) -> Result<ToolResult, AuroraError>;
}

pub trait Model {
    fn id(&self) -> &ModelId;
    fn infer(&self, request: &ModelRequest) -> Result<ModelResult, AuroraError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentResult {
    pub status: RequestStatus,
    pub output: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolRequest {
    pub tool_id: ToolId,
    pub request_id: crate::RequestId,
    pub input: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolResult {
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRequest {
    pub model_id: ModelId,
    pub request_id: crate::RequestId,
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelResult {
    pub output: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionRequest {
    pub request_id: crate::RequestId,
    pub capability: CapabilityId,
    pub input: String,
}

pub trait Runtime {
    fn execute(&self, request: &ExecutionRequest) -> Result<ExecutionResult, AuroraError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    pub status: RequestStatus,
    pub output: Option<String>,
}
