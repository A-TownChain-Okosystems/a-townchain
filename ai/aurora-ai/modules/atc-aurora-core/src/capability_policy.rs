// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Aurora capability/policy boundary.
//
// Security invariants:
// - Model output is never an authorization source.
// - Authorization is evaluated before any tool/action executes.
// - Unknown identities, capabilities, scopes and ambiguous policies fail closed.
// - Missing approval pauses with ApprovalRequired; it is not silently converted to Allow.
// - A grant is bound to one request and its complete principal/capability/action/scope tuple.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionRequest {
    pub request_id: String,
    pub principal: String,
    pub capability: String,
    pub action: String,
    pub scope: String,
    /// Only the trusted approval subsystem may mint a grant. Callers cannot construct one
    /// from model output or a user-supplied boolean.
    pub approval: Option<ApprovalGrant>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApprovalGrant {
    request_id: String,
    principal: String,
    capability: String,
    action: String,
    scope: String,
    approver_id: String,
}

impl ApprovalGrant {
    /// Internal constructor used by the trusted approval subsystem after it authenticates
    /// and validates a human approval. It is not available to external callers.
    pub(crate) fn verified_for(request: &ActionRequest, approver_id: &str) -> Self {
        Self {
            request_id: request.request_id.clone(),
            principal: request.principal.clone(),
            capability: request.capability.clone(),
            action: request.action.clone(),
            scope: request.scope.clone(),
            approver_id: approver_id.into(),
        }
    }

    fn matches(&self, request: &ActionRequest) -> bool {
        !self.approver_id.is_empty()
            && self.request_id == request.request_id
            && self.principal == request.principal
            && self.capability == request.capability
            && self.action == request.action
            && self.scope == request.scope
    }

    pub fn approver_id(&self) -> &str {
        &self.approver_id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny,
    /// Execution must pause until a trusted, request-scoped approval is provided.
    ApprovalRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityPolicy {
    principal: String,
    capability: String,
    action: String,
    scope: String,
    approval_required: bool,
}

impl CapabilityPolicy {
    pub fn new(
        principal: &str,
        capability: &str,
        action: &str,
        scope: &str,
        approval_required: bool,
    ) -> Self {
        Self {
            principal: principal.into(),
            capability: capability.into(),
            action: action.into(),
            scope: scope.into(),
            approval_required,
        }
    }

    fn matches_request(&self, request: &ActionRequest) -> bool {
        request.principal == self.principal
            && request.capability == self.capability
            && request.action == self.action
            && request.scope == self.scope
    }

    pub fn evaluate(&self, request: &ActionRequest) -> PolicyDecision {
        if !self.matches_request(request) {
            return PolicyDecision::Deny;
        }

        match (&request.approval, self.approval_required) {
            (Some(grant), true) if grant.matches(request) => PolicyDecision::Allow,
            (Some(_), true) => PolicyDecision::Deny,
            (None, true) => PolicyDecision::ApprovalRequired,
            (Some(grant), false) if !grant.matches(request) => PolicyDecision::Deny,
            (_, false) => PolicyDecision::Allow,
        }
    }
}

#[derive(Debug, Default)]
pub struct CapabilityPolicyEngine {
    policies: Vec<CapabilityPolicy>,
}

impl CapabilityPolicyEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, policy: CapabilityPolicy) {
        self.policies.push(policy);
    }

    pub fn evaluate(&self, request: &ActionRequest) -> PolicyDecision {
        let mut matching = self
            .policies
            .iter()
            .filter(|policy| policy.matches_request(request));

        let Some(policy) = matching.next() else {
            return PolicyDecision::Deny;
        };

        // Conflicting/duplicate rules must never be resolved by registration order.
        if matching.next().is_some() {
            return PolicyDecision::Deny;
        }

        policy.evaluate(request)
    }

    pub fn policy_count(&self) -> usize {
        self.policies.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> ActionRequest {
        ActionRequest {
            request_id: "request:001".into(),
            principal: "agent:test".into(),
            capability: "chain.tx.submit".into(),
            action: "submit_transaction".into(),
            scope: "chain:testnet".into(),
            approval: None,
        }
    }

    fn policy(approval_required: bool) -> CapabilityPolicy {
        CapabilityPolicy::new(
            "agent:test",
            "chain.tx.submit",
            "submit_transaction",
            "chain:testnet",
            approval_required,
        )
    }

    #[test]
    fn missing_approval_pauses_instead_of_denies_or_allows() {
        assert_eq!(
            policy(true).evaluate(&request()),
            PolicyDecision::ApprovalRequired
        );
    }

    #[test]
    fn request_scoped_verified_approval_allows() {
        let mut request = request();
        request.approval = Some(ApprovalGrant::verified_for(&request, "human:reviewer"));
        assert_eq!(policy(true).evaluate(&request), PolicyDecision::Allow);
    }

    #[test]
    fn approval_for_another_request_is_terminal_deny() {
        let mut approved_request = request();
        approved_request.approval = Some(ApprovalGrant::verified_for(
            &approved_request,
            "human:reviewer",
        ));

        let mut replay = request();
        replay.request_id = "request:002".into();
        replay.approval = approved_request.approval;

        assert_eq!(policy(true).evaluate(&replay), PolicyDecision::Deny);
    }

    #[test]
    fn unknown_capability_fails_closed() {
        let mut request = request();
        request.capability = "chain.admin".into();
        assert_eq!(policy(true).evaluate(&request), PolicyDecision::Deny);
    }

    #[test]
    fn scope_mismatch_fails_closed() {
        let mut request = request();
        request.scope = "chain:mainnet".into();
        assert_eq!(policy(true).evaluate(&request), PolicyDecision::Deny);
    }

    #[test]
    fn duplicate_matching_policies_fail_closed() {
        let mut engine = CapabilityPolicyEngine::new();
        engine.register(policy(false));
        engine.register(policy(true));
        assert_eq!(engine.evaluate(&request()), PolicyDecision::Deny);
    }

    #[test]
    fn unknown_policy_fails_closed() {
        assert_eq!(
            CapabilityPolicyEngine::new().evaluate(&request()),
            PolicyDecision::Deny
        );
    }

    #[test]
    fn policy_without_approval_allows_exact_match() {
        assert_eq!(policy(false).evaluate(&request()), PolicyDecision::Allow);
    }
}
