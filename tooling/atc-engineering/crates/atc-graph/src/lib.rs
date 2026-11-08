//! Canonical deterministic dependency and impact graph for the ATC engineering plane.
//!
//! The graph uses `from -> to` dependency semantics: `A -> B` means A depends on B.
//! Dependency-first topological order is therefore `B, A`.

use atc_core::{CoreError, Result};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum NodeKind {
    Repository,
    File,
    Module,
    Dependency,
    Standard,
    Requirement,
    Test,
    Workflow,
    Schema,
    Documentation,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Relation {
    DependsOn,
    Implements,
    Verifies,
    Governs,
    Documents,
    Affects,
    Requires,
    Supersedes,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Node {
    pub id: String,
    pub kind: NodeKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Edge {
    pub from: String,
    pub relation: Relation,
    pub to: String,
}

/// Canonical deterministic dependency graph.
///
/// Edges use `dependent -> dependency` semantics. All query results are owned
/// and deterministically ordered, so callers never depend on borrowed slices
/// into internal collections.
#[derive(Clone, Debug, Default)]
pub struct DependencyGraph {
    nodes: BTreeMap<String, Node>,
    dependencies: BTreeMap<String, BTreeSet<String>>,
    dependents: BTreeMap<String, BTreeSet<String>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Node) -> Result<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(CoreError::InvalidState(format!(
                "duplicate graph node: {}",
                node.id
            )));
        }
        let id = node.id.clone();
        self.nodes.insert(id.clone(), node);
        self.dependencies.entry(id.clone()).or_default();
        self.dependents.entry(id).or_default();
        Ok(())
    }

    pub fn add_dependency(&mut self, dependent: &str, dependency: &str) -> Result<()> {
        if !self.nodes.contains_key(dependent) || !self.nodes.contains_key(dependency) {
            return Err(CoreError::InvalidState(
                "dependency edge references unknown node".into(),
            ));
        }
        self.dependencies
            .entry(dependent.to_string())
            .or_default()
            .insert(dependency.to_string());
        self.dependents
            .entry(dependency.to_string())
            .or_default()
            .insert(dependent.to_string());
        Ok(())
    }

    pub fn has_node(&self, id: &str) -> bool {
        self.nodes.contains_key(id)
    }

    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Deterministically returns the direct dependencies of `id`.
    pub fn dependencies(&self, id: &str) -> Vec<String> {
        self.dependencies
            .get(id)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Deterministically returns the direct dependents of `id`.
    pub fn dependents(&self, id: &str) -> Vec<String> {
        self.dependents
            .get(id)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Returns all nodes in dependency-first order.
    pub fn topological_sort(&self) -> std::result::Result<Vec<String>, Vec<String>> {
        let mut remaining: BTreeMap<String, usize> = self
            .nodes
            .keys()
            .map(|id| (id.clone(), self.dependencies(id).len()))
            .collect();
        let mut ready: VecDeque<String> = self
            .nodes
            .keys()
            .filter(|id| remaining.get(*id) == Some(&0))
            .cloned()
            .collect();
        let mut result = Vec::with_capacity(self.nodes.len());

        while let Some(dependency) = ready.pop_front() {
            result.push(dependency.clone());
            for dependent in self.dependents(&dependency) {
                let count = remaining.get_mut(&dependent).expect("graph invariant");
                *count -= 1;
                if *count == 0 {
                    let pos = ready
                        .iter()
                        .position(|item| item > &dependent)
                        .unwrap_or(ready.len());
                    ready.insert(pos, dependent);
                }
            }
        }

        if result.len() == self.nodes.len() {
            Ok(result)
        } else {
            Err(self
                .nodes
                .keys()
                .filter(|id| !result.contains(id))
                .cloned()
                .collect())
        }
    }

    pub fn has_cycle(&self) -> bool {
        self.topological_sort().is_err()
    }

    /// Returns the transitive dependency closure, dependency-first.
    pub fn load_order(&self, target: &str) -> std::result::Result<Vec<String>, String> {
        if !self.has_node(target) {
            return Err(format!("unknown graph node: {target}"));
        }
        let mut seen = BTreeSet::new();
        let mut order = Vec::new();
        self.visit_dependencies(target, &mut seen, &mut order)?;
        Ok(order)
    }

    fn visit_dependencies(
        &self,
        node: &str,
        seen: &mut BTreeSet<String>,
        order: &mut Vec<String>,
    ) -> std::result::Result<(), String> {
        if !seen.insert(node.to_string()) {
            return Ok(());
        }
        for dependency in self.dependencies(node) {
            if !self.has_node(&dependency) {
                return Err(format!(
                    "dependency '{dependency}' not found (required by '{node}')"
                ));
            }
            self.visit_dependencies(&dependency, seen, order)?;
        }
        order.push(node.to_string());
        Ok(())
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.dependencies.values().map(BTreeSet::len).sum()
    }
}

/// General ATC impact graph. `Affects` and other relations are retained for
/// evidence/audit traversal, while dependency resolution uses `DependencyGraph`.
#[derive(Clone, Debug, Default)]
pub struct ImpactGraph {
    nodes: BTreeMap<String, Node>,
    edges: BTreeSet<Edge>,
}

impl ImpactGraph {
    pub fn add_node(&mut self, node: Node) -> Result<()> {
        if self.nodes.contains_key(&node.id) {
            return Err(CoreError::InvalidState(format!(
                "duplicate graph node: {}",
                node.id
            )));
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub fn add_edge(&mut self, edge: Edge) -> Result<()> {
        if !self.nodes.contains_key(&edge.from) || !self.nodes.contains_key(&edge.to) {
            return Err(CoreError::InvalidState(
                "graph edge references unknown node".into(),
            ));
        }
        self.edges.insert(edge);
        Ok(())
    }

    pub fn affected_by(&self, root: &str) -> Result<Vec<String>> {
        if !self.nodes.contains_key(root) {
            return Err(CoreError::InvalidId(format!("unknown graph node: {root}")));
        }
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::from([root.to_string()]);
        while let Some(current) = queue.pop_front() {
            for edge in self.edges.iter().filter(|edge| edge.from == current) {
                if seen.insert(edge.to.clone()) {
                    queue.push_back(edge.to.clone());
                }
            }
        }
        Ok(seen.into_iter().collect())
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph() -> DependencyGraph {
        let mut graph = DependencyGraph::new();
        for id in ["a", "b", "c", "d"] {
            graph
                .add_node(Node {
                    id: id.into(),
                    kind: NodeKind::Module,
                })
                .unwrap();
        }
        graph.add_dependency("a", "b").unwrap();
        graph.add_dependency("a", "c").unwrap();
        graph.add_dependency("b", "d").unwrap();
        graph
    }

    #[test]
    fn dependency_queries_are_owned_and_sorted() {
        let graph = graph();
        assert_eq!(graph.dependencies("a"), vec!["b", "c"]);
        assert_eq!(graph.dependents("d"), vec!["b"]);
    }

    #[test]
    fn dependency_first_topological_order() {
        let order = graph().topological_sort().unwrap();
        assert!(order.iter().position(|id| id == "d") < order.iter().position(|id| id == "b"));
        assert!(order.iter().position(|id| id == "b") < order.iter().position(|id| id == "a"));
        assert!(order.iter().position(|id| id == "c") < order.iter().position(|id| id == "a"));
    }

    #[test]
    fn detects_cycles() {
        let mut graph = graph();
        graph.add_dependency("d", "a").unwrap();
        assert!(graph.has_cycle());
    }

    #[test]
    fn load_order_is_dependency_first() {
        assert_eq!(graph().load_order("a").unwrap(), vec!["d", "b", "c", "a"]);
    }

    #[test]
    fn rejects_unknown_edges() {
        let mut graph = DependencyGraph::new();
        graph
            .add_node(Node {
                id: "a".into(),
                kind: NodeKind::Module,
            })
            .unwrap();
        assert!(graph.add_dependency("a", "missing").is_err());
    }

    #[test]
    fn traverses_impact() {
        let mut graph = ImpactGraph::default();
        graph
            .add_node(Node {
                id: "a".into(),
                kind: NodeKind::File,
            })
            .unwrap();
        graph
            .add_node(Node {
                id: "b".into(),
                kind: NodeKind::Test,
            })
            .unwrap();
        graph
            .add_edge(Edge {
                from: "a".into(),
                relation: Relation::Verifies,
                to: "b".into(),
            })
            .unwrap();
        assert_eq!(graph.affected_by("a").unwrap(), vec!["b"]);
    }
}
