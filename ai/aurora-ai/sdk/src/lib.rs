//! Aurora-AI SDK boundary. AI execution remains outside deterministic consensus.
use serde::{Deserialize,Serialize};
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]pub struct AgentId(pub String);
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]pub struct ToolCall{pub tool:String,pub input:serde_json::Value}
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]pub struct ModelRequest{pub agent:AgentId,pub input:String}
#[derive(Debug,Clone,Serialize,Deserialize,PartialEq,Eq)]pub struct ModelResponse{pub output:String}
pub trait ModelProvider{fn generate(&self,request:ModelRequest)->Result<ModelResponse,String>;}
