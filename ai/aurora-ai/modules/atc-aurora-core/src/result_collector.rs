// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Validates task/agent/run attribution and provides deterministic aggregation.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentResult {
    pub task_id: String,
    pub agent_id: String,
    pub run_id: String,
    pub payload: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectorError {
    EmptyTaskId,
    EmptyAgentId,
    EmptyRunId,
    EmptyPayload,
    EmptyExpectedAgents,
    DuplicateExpectedAgent(String),
    DuplicateExpectation(String),
    UnexpectedResult {
        task_id: String,
        agent_id: String,
    },
    DuplicateResult {
        task_id: String,
        agent_id: String,
        run_id: String,
    },
}

#[derive(Debug, Default)]
pub struct ResultCollector {
    expected: BTreeMap<String, BTreeSet<String>>,
    results: BTreeMap<(String, String, String), AgentResult>,
}

impl ResultCollector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn expect_agents<I, S>(&mut self, task_id: &str, agent_ids: I) -> Result<(), CollectorError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let task_id = task_id.trim();
        if task_id.is_empty() {
            return Err(CollectorError::EmptyTaskId);
        }
        if self.expected.contains_key(task_id) {
            return Err(CollectorError::DuplicateExpectation(task_id.into()));
        }
        let mut agents = BTreeSet::new();
        for agent_id in agent_ids {
            let agent_id = agent_id.into();
            let agent_id = agent_id.trim().to_owned();
            if agent_id.is_empty() {
                return Err(CollectorError::EmptyAgentId);
            }
            if !agents.insert(agent_id.clone()) {
                return Err(CollectorError::DuplicateExpectedAgent(agent_id));
            }
        }
        if agents.is_empty() {
            return Err(CollectorError::EmptyExpectedAgents);
        }
        self.expected.insert(task_id.into(), agents);
        Ok(())
    }

    pub fn submit(&mut self, result: AgentResult) -> Result<(), CollectorError> {
        if result.task_id.trim().is_empty() {
            return Err(CollectorError::EmptyTaskId);
        }
        if result.agent_id.trim().is_empty() {
            return Err(CollectorError::EmptyAgentId);
        }
        if result.run_id.trim().is_empty() {
            return Err(CollectorError::EmptyRunId);
        }
        if result.payload.trim().is_empty() {
            return Err(CollectorError::EmptyPayload);
        }
        let key = (
            result.task_id.clone(),
            result.agent_id.clone(),
            result.run_id.clone(),
        );
        let allowed = self
            .expected
            .get(&result.task_id)
            .is_some_and(|agents| agents.contains(&result.agent_id));
        if !allowed {
            return Err(CollectorError::UnexpectedResult {
                task_id: result.task_id,
                agent_id: result.agent_id,
            });
        }
        if self.results.contains_key(&key) {
            return Err(CollectorError::DuplicateResult {
                task_id: key.0,
                agent_id: key.1,
                run_id: key.2,
            });
        }
        self.results.insert(key, result);
        Ok(())
    }

    /// Results are ordered lexically by task ID, agent ID, then run ID.
    pub fn aggregate(&self) -> Vec<AgentResult> {
        self.results.values().cloned().collect()
    }

    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    pub fn missing_agents(&self, task_id: &str) -> Vec<String> {
        let Some(expected) = self.expected.get(task_id) else {
            return Vec::new();
        };
        expected
            .iter()
            .filter(|agent| {
                !self
                    .results
                    .keys()
                    .any(|(task, result_agent, _)| task == task_id && result_agent == *agent)
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(task: &str, agent: &str, run: &str, payload: &str) -> AgentResult {
        AgentResult {
            task_id: task.into(),
            agent_id: agent.into(),
            run_id: run.into(),
            payload: payload.into(),
        }
    }

    #[test]
    fn validates_attribution_and_rejects_duplicate_results() {
        let mut collector = ResultCollector::new();
        collector
            .expect_agents("task:1", ["agent:a", "agent:b"])
            .unwrap();
        collector
            .submit(result("task:1", "agent:b", "run:1", "B"))
            .unwrap();
        assert_eq!(
            collector.submit(result("task:1", "agent:b", "run:1", "again")),
            Err(CollectorError::DuplicateResult {
                task_id: "task:1".into(),
                agent_id: "agent:b".into(),
                run_id: "run:1".into(),
            })
        );
        assert_eq!(
            collector.submit(result("task:1", "agent:unknown", "run:2", "X")),
            Err(CollectorError::UnexpectedResult {
                task_id: "task:1".into(),
                agent_id: "agent:unknown".into()
            })
        );
        assert_eq!(collector.missing_agents("task:1"), vec!["agent:a"]);
    }

    #[test]
    fn aggregation_is_deterministic_and_distinguishes_runs() {
        let mut collector = ResultCollector::new();
        collector.expect_agents("task:b", ["agent:z"]).unwrap();
        collector
            .expect_agents("task:a", ["agent:y", "agent:x"])
            .unwrap();
        collector
            .submit(result("task:b", "agent:z", "run:2", "B"))
            .unwrap();
        collector
            .submit(result("task:a", "agent:y", "run:1", "Y"))
            .unwrap();
        collector
            .submit(result("task:a", "agent:x", "run:3", "X"))
            .unwrap();
        collector
            .submit(result("task:a", "agent:x", "run:4", "X retry"))
            .unwrap();
        let aggregate = collector.aggregate();
        assert_eq!(
            aggregate
                .iter()
                .map(|r| (r.task_id.as_str(), r.agent_id.as_str(), r.run_id.as_str()))
                .collect::<Vec<_>>(),
            vec![
                ("task:a", "agent:x", "run:3"),
                ("task:a", "agent:x", "run:4"),
                ("task:a", "agent:y", "run:1"),
                ("task:b", "agent:z", "run:2")
            ]
        );
    }

    #[test]
    fn empty_payload_and_run_id_are_rejected() {
        let mut collector = ResultCollector::new();
        collector.expect_agents("task:1", ["agent:a"]).unwrap();
        assert_eq!(
            collector.submit(result("task:1", "agent:a", " ", "payload")),
            Err(CollectorError::EmptyRunId)
        );
        assert_eq!(
            collector.submit(result("task:1", "agent:a", "run:1", " ")),
            Err(CollectorError::EmptyPayload)
        );
    }
}
