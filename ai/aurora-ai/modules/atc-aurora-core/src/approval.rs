// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// In-memory approval reference implementation. It does not perform I/O or authenticate users.
// Only an authorized host adapter may mint VerifiedApproval after authenticating the human.

use std::collections::HashMap;

use crate::capability_policy::{ActionRequest, ApprovalGrant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalError {
    EmptyRequestId,
    EmptyApprover,
    RequestAlreadyPending,
    RequestNotPending,
    RequestAlreadyCarriesApproval,
}

#[derive(Debug)]
pub struct VerifiedApproval {
    request_id: String,
    approver_id: String,
}

impl VerifiedApproval {
    /// Seal created only inside the crate by the future authenticated host adapter.
    /// This constructor remains unused in production until that adapter is integrated.
    #[allow(dead_code)]
    pub(crate) fn from_authenticated_host(
        request_id: &str,
        approver_id: &str,
    ) -> Result<Self, ApprovalError> {
        if request_id.is_empty() {
            return Err(ApprovalError::EmptyRequestId);
        }
        if approver_id.is_empty() {
            return Err(ApprovalError::EmptyApprover);
        }
        Ok(Self {
            request_id: request_id.into(),
            approver_id: approver_id.into(),
        })
    }
}

#[derive(Debug, Default)]
pub struct ApprovalEngine {
    pending: HashMap<String, ActionRequest>,
}

impl ApprovalEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn request_approval(&mut self, request: ActionRequest) -> Result<(), ApprovalError> {
        if request.request_id.is_empty() {
            return Err(ApprovalError::EmptyRequestId);
        }
        if request.approval.is_some() {
            return Err(ApprovalError::RequestAlreadyCarriesApproval);
        }
        if self.pending.contains_key(&request.request_id) {
            return Err(ApprovalError::RequestAlreadyPending);
        }

        self.pending.insert(request.request_id.clone(), request);
        Ok(())
    }

    /// Consumes a sealed approval token. External code cannot construct this token directly.
    pub fn approve(
        &mut self,
        confirmation: VerifiedApproval,
    ) -> Result<ApprovalGrant, ApprovalError> {
        let request = self
            .pending
            .remove(&confirmation.request_id)
            .ok_or(ApprovalError::RequestNotPending)?;
        Ok(ApprovalGrant::verified_for(
            &request,
            &confirmation.approver_id,
        ))
    }

    pub fn reject(&mut self, request_id: &str) -> bool {
        self.pending.remove(request_id).is_some()
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> ActionRequest {
        ActionRequest {
            request_id: "request:approval-1".into(),
            principal: "agent:assistant".into(),
            capability: "filesystem.write".into(),
            action: "write_file".into(),
            scope: "workspace:project-a".into(),
            approval: None,
        }
    }

    #[test]
    fn verified_approval_is_one_shot_and_bound_to_request() {
        let mut engine = ApprovalEngine::new();
        engine.request_approval(request()).unwrap();

        let confirmation =
            VerifiedApproval::from_authenticated_host("request:approval-1", "human:owner").unwrap();
        let grant = engine.approve(confirmation).unwrap();

        assert_eq!(grant.approver_id(), "human:owner");
        assert_eq!(engine.pending_count(), 0);
        let replay =
            VerifiedApproval::from_authenticated_host("request:approval-1", "human:owner").unwrap();
        assert_eq!(
            engine.approve(replay),
            Err(ApprovalError::RequestNotPending)
        );
    }

    #[test]
    fn rejects_duplicate_pending_request() {
        let mut engine = ApprovalEngine::new();
        engine.request_approval(request()).unwrap();
        assert_eq!(
            engine.request_approval(request()),
            Err(ApprovalError::RequestAlreadyPending)
        );
    }

    #[test]
    fn reject_removes_pending_request() {
        let mut engine = ApprovalEngine::new();
        engine.request_approval(request()).unwrap();
        assert!(engine.reject("request:approval-1"));
        assert_eq!(engine.pending_count(), 0);
        assert!(!engine.reject("request:approval-1"));
    }
}
