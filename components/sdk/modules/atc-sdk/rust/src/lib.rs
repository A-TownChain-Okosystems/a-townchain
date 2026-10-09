//! Canonical, transport-neutral A-TownChain client primitives.
//! Consensus rules remain owned by A-TownChain/ATC-VM; this crate only
//! serializes protocol data, performs RPC calls, and validates responses.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("invalid endpoint: {0}")]
    InvalidEndpoint(String),
    #[error("RPC transport error: {0}")]
    Transport(String),
    #[error("RPC error {code}: {message}")]
    Rpc { code: i64, message: String },
    #[error("invalid response: {0}")]
    InvalidResponse(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RpcRequest<P> {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: P,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: Option<T>,
    pub error: Option<RpcError>,
}

impl<T> RpcResponse<T> {
    pub fn into_result(self) -> Result<T, SdkError> {
        if let Some(error) = self.error {
            return Err(SdkError::Rpc { code: error.code, message: error.message });
        }
        self.result.ok_or_else(|| SdkError::InvalidResponse("missing result".into()))
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub endpoint: Url,
}

impl ClientConfig {
    pub fn new(endpoint: &str) -> Result<Self, SdkError> {
        let url = Url::parse(endpoint)
            .map_err(|e| SdkError::InvalidEndpoint(e.to_string()))?;
        match url.scheme() {
            "http" | "https" | "ws" | "wss" => Ok(Self { endpoint: url }),
            scheme => Err(SdkError::InvalidEndpoint(format!("unsupported scheme: {scheme}"))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AtcClient {
    config: ClientConfig,
}

impl AtcClient {
    pub fn new(endpoint: &str) -> Result<Self, SdkError> {
        Ok(Self { config: ClientConfig::new(endpoint)? })
    }

    pub fn endpoint(&self) -> &Url {
        &self.config.endpoint
    }

    pub fn request<P: Serialize>(&self, id: u64, method: impl Into<String>, params: P) -> RpcRequest<P> {
        RpcRequest {
            jsonrpc: "2.0".into(),
            id,
            method: method.into(),
            params,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChainInfo {
    pub chain_id: String,
    pub network: String,
    pub protocol_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Balance {
    pub address: String,
    pub amount: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_endpoint_scheme() {
        assert!(ClientConfig::new("ftp://node.example").is_err());
    }

    #[test]
    fn builds_json_rpc_request() {
        let client = AtcClient::new("https://node.example").unwrap();
        let request = client.request(1, "chain_getInfo", serde_json::json!([]));
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "chain_getInfo");
    }

    #[test]
    fn rpc_response_is_fail_closed() {
        let response = RpcResponse::<ChainInfo> {
            jsonrpc: "2.0".into(),
            id: 1,
            result: None,
            error: None,
        };
        assert!(matches!(response.into_result(), Err(SdkError::InvalidResponse(_))));
    }
}
