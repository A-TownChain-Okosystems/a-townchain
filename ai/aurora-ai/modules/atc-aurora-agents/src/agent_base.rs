// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Base agent traits and structures
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AgentContext {
    pub task_id: String,
    pub input: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AgentResponse {
    pub task_id: String,
    pub output: String,
    pub success: bool,
    pub metadata: HashMap<String, String>,
}

pub trait Agent: Send + Sync {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn role(&self) -> &str;
    fn capabilities(&self) -> Vec<String>;
    fn execute(&self, ctx: AgentContext) -> AgentResponse;
}

impl AgentContext {
    pub fn new(task_id: &str, input: &str) -> Self {
        Self { task_id: task_id.into(), input: input.into(), metadata: HashMap::new() }
    }
    pub fn with_meta(mut self, key: &str, val: &str) -> Self {
        self.metadata.insert(key.into(), val.into());
        self
    }
}

impl AgentResponse {
    pub fn success(task_id: &str, output: &str) -> Self {
        Self { task_id: task_id.into(), output: output.into(), success: true, metadata: HashMap::new() }
    }
    pub fn failure(task_id: &str, error: &str) -> Self {
        Self { task_id: task_id.into(), output: error.into(), success: false, metadata: HashMap::new() }
    }
}
