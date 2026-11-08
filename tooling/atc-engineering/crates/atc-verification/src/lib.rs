//! Verification foundation. PASS is only emitted from an explicit verifier.

use atc_core::{CoreError, Result};
use atc_graph::DependencyGraph;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerificationStatus {
    Pass,
    Fail,
    Blocked,
    NotExecuted,
    NotApplicable,
}

impl VerificationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Blocked => "BLOCKED",
            Self::NotExecuted => "NOT_EXECUTED",
            Self::NotApplicable => "NOT_APPLICABLE",
        }
    }
    pub fn is_pass(self) -> bool {
        matches!(self, Self::Pass)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationResult {
    pub check_id: String,
    pub status: VerificationStatus,
    pub evidence_ref: Option<String>,
    pub details: String,
}

impl VerificationResult {
    pub fn pass(check_id: &str, evidence_ref: &str, details: &str) -> Result<Self> {
        if check_id.trim().is_empty() || evidence_ref.trim().is_empty() {
            return Err(CoreError::MissingEvidence(
                "PASS requires check id and evidence reference".into(),
            ));
        }
        Ok(Self {
            check_id: check_id.into(),
            status: VerificationStatus::Pass,
            evidence_ref: Some(evidence_ref.into()),
            details: details.into(),
        })
    }
    pub fn blocked(check_id: &str, details: &str) -> Self {
        Self {
            check_id: check_id.into(),
            status: VerificationStatus::Blocked,
            evidence_ref: None,
            details: details.into(),
        }
    }

    /// Verify that the canonical engineering dependency graph is acyclic and
    /// therefore has a deterministic dependency-first load/build order.
    pub fn verify_dependency_graph(
        check_id: &str,
        evidence_ref: &str,
        graph: &DependencyGraph,
    ) -> Result<Self> {
        let order = graph.topological_sort().map_err(|remaining| {
            CoreError::InvalidState(format!(
                "dependency graph contains a cycle; unresolved nodes: {}",
                remaining.join(", ")
            ))
        })?;
        if order.len() != graph.node_count() {
            return Err(CoreError::InvalidState(
                "dependency graph is incomplete".into(),
            ));
        }
        Self::pass(
            check_id,
            evidence_ref,
            &format!(
                "dependency graph verified: {} nodes, {} edges, deterministic order",
                graph.node_count(),
                graph.edge_count()
            ),
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct VerificationReport {
    pub results: Vec<VerificationResult>,
}

impl VerificationReport {
    pub fn ready(&self) -> bool {
        !self.results.is_empty() && self.results.iter().all(|r| r.status.is_pass())
    }
    pub fn add(&mut self, result: VerificationResult) {
        self.results.push(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atc_graph::{Node, NodeKind};

    #[test]
    fn pass_requires_evidence() {
        assert!(VerificationResult::pass("build", "", "").is_err());
    }

    #[test]
    fn report_is_fail_closed() {
        let mut r = VerificationReport::default();
        r.add(VerificationResult::blocked("test", "not run"));
        assert!(!r.ready());
    }

    #[test]
    fn dependency_graph_verification_is_explicit() {
        let mut graph = DependencyGraph::new();
        graph
            .add_node(Node {
                id: "app".into(),
                kind: NodeKind::Repository,
            })
            .unwrap();
        graph
            .add_node(Node {
                id: "core".into(),
                kind: NodeKind::Dependency,
            })
            .unwrap();
        graph.add_dependency("app", "core").unwrap();
        let result =
            VerificationResult::verify_dependency_graph("graph", "evidence://graph-test", &graph)
                .unwrap();
        assert!(result.status.is_pass());
    }

    #[test]
    fn dependency_cycle_fails_closed() {
        let mut graph = DependencyGraph::new();
        graph
            .add_node(Node {
                id: "a".into(),
                kind: NodeKind::Module,
            })
            .unwrap();
        graph
            .add_node(Node {
                id: "b".into(),
                kind: NodeKind::Module,
            })
            .unwrap();
        graph.add_dependency("a", "b").unwrap();
        graph.add_dependency("b", "a").unwrap();
        assert!(VerificationResult::verify_dependency_graph(
            "graph",
            "evidence://cycle-test",
            &graph
        )
        .is_err());
    }
}
