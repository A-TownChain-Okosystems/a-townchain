// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Deterministic task lifecycle scheduler. It models orchestration; it does not spawn processes.

use crate::agent_registry::AgentRegistry;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
    TimedOut,
}

impl TaskState {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed | Self::Cancelled | Self::TimedOut
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub swarm_id: Option<String>,
    pub agent_id: Option<String>,
    pub agent_role: Option<String>,
    pub capabilities: Vec<String>,
    /// A fresh UUIDv4 is assigned each time this task is dispatched for execution.
    pub run_id: Option<String>,
    pub name: String,
    pub dependencies: Vec<String>,
    pub state: TaskState,
    pub submitted_at_ms: u64,
    pub started_at_ms: Option<u64>,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchedulerError {
    EmptyId,
    EmptyName,
    EmptySwarmId,
    ZeroTimeout,
    DuplicateTask(String),
    MissingDependency(String),
    DependencyFailed(String),
    TaskNotFound(String),
    AgentNotAuthorized(String),
    InvalidTransition { id: String, state: TaskState },
    InvalidParallelism,
}

#[derive(Debug)]
pub struct TaskScheduler {
    max_parallel: usize,
    tasks: BTreeMap<String, Task>,
}

impl TaskScheduler {
    pub fn new(max_parallel: usize) -> Result<Self, SchedulerError> {
        if max_parallel == 0 {
            return Err(SchedulerError::InvalidParallelism);
        }
        Ok(Self {
            max_parallel,
            tasks: BTreeMap::new(),
        })
    }

    /// Compatibility path for internal/unassigned tasks. Agent-assigned tasks must use
    /// schedule_for_agent and dispatch_ready_authorized so revocation is checked fail-closed.
    pub fn schedule(
        &mut self,
        id: &str,
        name: &str,
        dependencies: Vec<String>,
        now_ms: u64,
        timeout_ms: u64,
    ) -> Result<(), SchedulerError> {
        self.schedule_inner(
            id,
            name,
            None,
            None,
            None,
            Vec::new(),
            dependencies,
            now_ms,
            timeout_ms,
        )
    }

    // Keep assignment identity and scheduling constraints explicit at this API boundary.
    #[allow(clippy::too_many_arguments)]
    pub fn schedule_for_agent(
        &mut self,
        id: &str,
        name: &str,
        swarm_id: &str,
        agent_id: &str,
        dependencies: Vec<String>,
        now_ms: u64,
        timeout_ms: u64,
        registry: &AgentRegistry,
    ) -> Result<(), SchedulerError> {
        let swarm_id = swarm_id.trim();
        if swarm_id.is_empty() {
            return Err(SchedulerError::EmptySwarmId);
        }
        let agent_id = agent_id.trim();
        let agent = registry
            .get_agent(agent_id)
            .filter(|agent| agent.is_active)
            .ok_or_else(|| SchedulerError::AgentNotAuthorized(agent_id.into()))?;
        self.schedule_inner(
            id,
            name,
            Some(swarm_id.into()),
            Some(agent.id.clone()),
            Some(agent.role.clone()),
            agent.capabilities.clone(),
            dependencies,
            now_ms,
            timeout_ms,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn schedule_inner(
        &mut self,
        id: &str,
        name: &str,
        swarm_id: Option<String>,
        agent_id: Option<String>,
        agent_role: Option<String>,
        capabilities: Vec<String>,
        dependencies: Vec<String>,
        now_ms: u64,
        timeout_ms: u64,
    ) -> Result<(), SchedulerError> {
        let id = id.trim();
        if id.is_empty() {
            return Err(SchedulerError::EmptyId);
        }
        if name.trim().is_empty() {
            return Err(SchedulerError::EmptyName);
        }
        if timeout_ms == 0 {
            return Err(SchedulerError::ZeroTimeout);
        }
        if self.tasks.contains_key(id) {
            return Err(SchedulerError::DuplicateTask(id.into()));
        }
        let mut seen = std::collections::HashSet::new();
        for dependency in &dependencies {
            if dependency.as_str() == id || !seen.insert(dependency) {
                return Err(SchedulerError::MissingDependency(dependency.clone()));
            }
            let dep = self
                .tasks
                .get(dependency)
                .ok_or_else(|| SchedulerError::MissingDependency(dependency.clone()))?;
            if matches!(
                dep.state,
                TaskState::Failed | TaskState::Cancelled | TaskState::TimedOut
            ) {
                return Err(SchedulerError::DependencyFailed(dependency.clone()));
            }
        }
        self.tasks.insert(
            id.into(),
            Task {
                id: id.into(),
                swarm_id,
                agent_id,
                agent_role,
                capabilities,
                run_id: None,
                name: name.trim().into(),
                dependencies,
                state: TaskState::Queued,
                submitted_at_ms: now_ms,
                started_at_ms: None,
                timeout_ms,
            },
        );
        Ok(())
    }

    /// Legacy/internal dispatch only runs tasks without an agent assignment.
    /// Assigned tasks fail closed here; use dispatch_ready_authorized with the registry.
    pub fn dispatch_ready(&mut self, now_ms: u64) -> Vec<String> {
        self.dispatch_inner(now_ms, None)
    }

    pub fn dispatch_ready_authorized(
        &mut self,
        now_ms: u64,
        registry: &AgentRegistry,
    ) -> Vec<String> {
        self.dispatch_inner(now_ms, Some(registry))
    }

    fn dispatch_inner(&mut self, now_ms: u64, registry: Option<&AgentRegistry>) -> Vec<String> {
        self.expire(now_ms);
        let blocked: Vec<String> = self
            .tasks
            .values()
            .filter(|task| {
                task.state == TaskState::Queued
                    && (task.dependencies.iter().any(|id| {
                        self.tasks.get(id).is_some_and(|dep| {
                            matches!(
                                dep.state,
                                TaskState::Failed | TaskState::Cancelled | TaskState::TimedOut
                            )
                        })
                    }) || match task.agent_id.as_deref() {
                        Some(agent_id) => registry.is_none_or(|r| !r.is_active(agent_id)),
                        None => false,
                    })
            })
            .map(|task| task.id.clone())
            .collect();
        for id in blocked {
            if let Some(task) = self.tasks.get_mut(&id) {
                task.state = TaskState::Failed;
            }
        }
        let running = self
            .tasks
            .values()
            .filter(|task| task.state == TaskState::Running)
            .count();
        let slots = self.max_parallel.saturating_sub(running);
        let ready: Vec<String> = self
            .tasks
            .values()
            .filter(|task| {
                task.state == TaskState::Queued
                    && task.dependencies.iter().all(|id| {
                        self.tasks
                            .get(id)
                            .is_some_and(|dep| dep.state == TaskState::Completed)
                    })
                    && match task.agent_id.as_deref() {
                        Some(agent_id) => registry.is_some_and(|r| r.is_active(agent_id)),
                        None => true,
                    }
            })
            .map(|task| task.id.clone())
            .take(slots)
            .collect();
        for id in &ready {
            if let Some(task) = self.tasks.get_mut(id) {
                task.state = TaskState::Running;
                task.started_at_ms = Some(now_ms);
                task.run_id = Some(format!("run:{}", Uuid::new_v4()));
            }
        }
        ready
    }

    pub fn complete(&mut self, id: &str) -> Result<(), SchedulerError> {
        self.transition_running(id, TaskState::Completed)
    }

    pub fn fail(&mut self, id: &str) -> Result<(), SchedulerError> {
        self.transition_running(id, TaskState::Failed)
    }

    pub fn cancel(&mut self, id: &str) -> Result<(), SchedulerError> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| SchedulerError::TaskNotFound(id.into()))?;
        if !matches!(task.state, TaskState::Queued | TaskState::Running) {
            return Err(SchedulerError::InvalidTransition {
                id: id.into(),
                state: task.state,
            });
        }
        task.state = TaskState::Cancelled;
        Ok(())
    }

    pub fn expire(&mut self, now_ms: u64) -> Vec<String> {
        let mut expired = Vec::new();
        for task in self.tasks.values_mut() {
            if task.state == TaskState::Running
                && task
                    .started_at_ms
                    .is_some_and(|start| now_ms.saturating_sub(start) >= task.timeout_ms)
            {
                task.state = TaskState::TimedOut;
                expired.push(task.id.clone());
            }
        }
        expired
    }

    pub fn get(&self, id: &str) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn len(&self) -> usize {
        self.tasks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn running_count(&self) -> usize {
        self.tasks
            .values()
            .filter(|task| task.state == TaskState::Running)
            .count()
    }

    fn transition_running(&mut self, id: &str, target: TaskState) -> Result<(), SchedulerError> {
        let task = self
            .tasks
            .get_mut(id)
            .ok_or_else(|| SchedulerError::TaskNotFound(id.into()))?;
        if task.state != TaskState::Running {
            return Err(SchedulerError::InvalidTransition {
                id: id.into(),
                state: task.state,
            });
        }
        task.state = target;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent_registry::AgentRegistry;

    #[test]
    fn dependencies_gate_execution_and_parallelism_is_bounded() {
        let mut scheduler = TaskScheduler::new(1).unwrap();
        scheduler.schedule("task:a", "A", vec![], 0, 100).unwrap();
        scheduler
            .schedule("task:b", "B", vec!["task:a".into()], 0, 100)
            .unwrap();
        assert_eq!(scheduler.dispatch_ready(1), vec!["task:a"]);
        assert!(scheduler.dispatch_ready(2).is_empty());
        scheduler.complete("task:a").unwrap();
        assert_eq!(scheduler.dispatch_ready(3), vec!["task:b"]);
        assert!(scheduler
            .get("task:a")
            .unwrap()
            .run_id
            .as_ref()
            .unwrap()
            .starts_with("run:"));
        assert_ne!(
            scheduler.get("task:a").unwrap().run_id,
            scheduler.get("task:b").unwrap().run_id
        );
    }

    #[test]
    fn timeout_and_cancel_are_terminal() {
        let mut scheduler = TaskScheduler::new(2).unwrap();
        scheduler
            .schedule("task:slow", "slow", vec![], 0, 10)
            .unwrap();
        scheduler
            .schedule("task:cancel", "cancel", vec![], 0, 10)
            .unwrap();
        scheduler.dispatch_ready(5);
        assert_eq!(scheduler.expire(15), vec!["task:cancel", "task:slow"]);
        assert_eq!(
            scheduler.get("task:slow").unwrap().state,
            TaskState::TimedOut
        );
        assert!(scheduler.cancel("task:slow").is_err());

        let mut cancellable = TaskScheduler::new(1).unwrap();
        cancellable
            .schedule("task:cancel", "cancel", vec![], 0, 10)
            .unwrap();
        assert_eq!(cancellable.cancel("task:cancel"), Ok(()));
        assert_eq!(
            cancellable.get("task:cancel").unwrap().state,
            TaskState::Cancelled
        );
        assert!(cancellable.cancel("task:cancel").is_err());
    }

    #[test]
    fn duplicate_ids_and_missing_dependencies_fail_closed() {
        let mut scheduler = TaskScheduler::new(1).unwrap();
        assert_eq!(
            scheduler.schedule("task:a", "A", vec!["missing".into()], 0, 10),
            Err(SchedulerError::MissingDependency("missing".into()))
        );
        scheduler.schedule("task:a", "A", vec![], 0, 10).unwrap();
        assert_eq!(
            scheduler.schedule("task:a", "duplicate", vec![], 0, 10),
            Err(SchedulerError::DuplicateTask("task:a".into()))
        );
    }

    #[test]
    fn failed_dependency_cannot_be_scheduled_as_prerequisite() {
        let mut scheduler = TaskScheduler::new(1).unwrap();
        scheduler.schedule("task:a", "A", vec![], 0, 10).unwrap();
        scheduler.dispatch_ready(0);
        scheduler.fail("task:a").unwrap();
        assert_eq!(
            scheduler.schedule("task:b", "B", vec!["task:a".into()], 1, 10),
            Err(SchedulerError::DependencyFailed("task:a".into()))
        );
    }

    #[test]
    fn already_queued_dependents_fail_if_prerequisite_fails() {
        let mut scheduler = TaskScheduler::new(1).unwrap();
        scheduler.schedule("task:a", "A", vec![], 0, 10).unwrap();
        scheduler
            .schedule("task:b", "B", vec!["task:a".into()], 0, 10)
            .unwrap();
        scheduler.dispatch_ready(0);
        scheduler.fail("task:a").unwrap();
        assert!(scheduler.dispatch_ready(1).is_empty());
        assert_eq!(scheduler.get("task:b").unwrap().state, TaskState::Failed);
    }

    #[test]
    fn revoked_agents_cannot_receive_or_dispatch_new_tasks() {
        let mut registry = AgentRegistry::new();
        let agent_id = registry
            .register_generated("worker", "research", vec!["read".into()])
            .unwrap();
        let mut scheduler = TaskScheduler::new(1).unwrap();
        scheduler
            .schedule_for_agent(
                "task:1",
                "task",
                "swarm:1",
                &agent_id,
                vec![],
                0,
                100,
                &registry,
            )
            .unwrap();
        registry.revoke(&agent_id).unwrap();
        assert!(scheduler.dispatch_ready_authorized(1, &registry).is_empty());
        assert_eq!(scheduler.get("task:1").unwrap().state, TaskState::Failed);
        assert_eq!(
            scheduler.schedule_for_agent(
                "task:2",
                "task",
                "swarm:1",
                &agent_id,
                vec![],
                2,
                100,
                &registry
            ),
            Err(SchedulerError::AgentNotAuthorized(agent_id))
        );
    }

    #[test]
    fn assignment_carries_distinct_swarm_agent_role_capabilities_and_run_ids() {
        let mut registry = AgentRegistry::new();
        let agent_id = registry
            .register_generated("worker", "research", vec!["read".into()])
            .unwrap();
        let mut scheduler = TaskScheduler::new(1).unwrap();
        scheduler
            .schedule_for_agent(
                "task:1",
                "task",
                "swarm:1",
                &agent_id,
                vec![],
                0,
                100,
                &registry,
            )
            .unwrap();
        assert_eq!(
            scheduler.dispatch_ready_authorized(1, &registry),
            vec!["task:1"]
        );
        let task = scheduler.get("task:1").unwrap();
        assert_eq!(task.swarm_id.as_deref(), Some("swarm:1"));
        assert_eq!(task.agent_id.as_deref(), Some(agent_id.as_str()));
        assert_eq!(task.agent_role.as_deref(), Some("research"));
        assert_eq!(task.capabilities, vec!["read"]);
        assert!(task.run_id.as_deref().unwrap().starts_with("run:"));
    }
}
