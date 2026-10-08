// Copyright (c) 2026 Michael Wroblewski — Apache-2.0
//! Network interfaces & protocol handling for mining jobs.

use crate::miner::{MiningJob, MiningResult, MiningError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkMessage {
    GetWorkRequest { miner_id: String },
    GetWorkResponse { job: Option<MiningJob> },
    SubmitProofRequest { result: MiningResult },
    SubmitProofResponse { accepted: bool },
}

pub struct NetworkHandler {
    node_id: String,
}

impl NetworkHandler {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
        }
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn process_proof(&self, result: &MiningResult, target: &[u8; 32]) -> Result<bool, MiningError> {
        if result.digest <= *target {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_handler_proof_verification() {
        let handler = NetworkHandler::new("node-01");
        assert_eq!(handler.node_id(), "node-01");

        let target = [0x0fu8; 32];
        let valid_result = MiningResult {
            job_id: 1,
            nonce: 42,
            digest: [0x01u8; 32],
        };
        let invalid_result = MiningResult {
            job_id: 1,
            nonce: 43,
            digest: [0xffu8; 32],
        };

        assert_eq!(handler.process_proof(&valid_result, &target), Ok(true));
        assert_eq!(handler.process_proof(&invalid_result, &target), Ok(false));
    }

    #[test]
    fn test_network_message_serialization_roundtrip() {
        let msg = NetworkMessage::GetWorkRequest { miner_id: "miner-1".into() };
        if let NetworkMessage::GetWorkRequest { miner_id } = msg {
            assert_eq!(miner_id, "miner-1");
        } else {
            panic!("Unexpected message variant");
        }
    }
}
