// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// Deterministic in-memory event bus. It performs no I/O and grants no authority.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    RequestCreated,
    StateChanged,
    ApprovalRequired,
    ToolInvoked,
    ExecutionCompleted,
    ExecutionFailed,
    SecurityDenied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuroraEvent {
    pub event_id: String,
    pub request_id: String,
    pub agent_id: Option<String>,
    pub swarm_id: Option<String>,
    pub task_id: Option<String>,
    pub run_id: Option<String>,
    pub kind: EventKind,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventBusError {
    EmptyEventId,
    EmptyRequestId,
    DuplicateEventId,
}

#[derive(Debug, Default)]
pub struct EventBus {
    events: Vec<AuroraEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn publish(&mut self, event: AuroraEvent) -> Result<(), EventBusError> {
        if event.event_id.is_empty() {
            return Err(EventBusError::EmptyEventId);
        }
        if event.request_id.is_empty() {
            return Err(EventBusError::EmptyRequestId);
        }
        if self
            .events
            .iter()
            .any(|existing| existing.event_id == event.event_id)
        {
            return Err(EventBusError::DuplicateEventId);
        }
        self.events.push(event);
        Ok(())
    }

    pub fn events(&self) -> &[AuroraEvent] {
        &self.events
    }

    pub fn events_for_request(&self, request_id: &str) -> Vec<&AuroraEvent> {
        self.events
            .iter()
            .filter(|event| event.request_id == request_id)
            .collect()
    }

    pub fn events_for_agent(&self, agent_id: &str) -> Vec<&AuroraEvent> {
        self.events
            .iter()
            .filter(|event| event.agent_id.as_deref() == Some(agent_id))
            .collect()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(event_id: &str, request_id: &str, agent_id: Option<&str>) -> AuroraEvent {
        AuroraEvent {
            event_id: event_id.into(),
            request_id: request_id.into(),
            agent_id: agent_id.map(str::to_owned),
            swarm_id: Some("swarm:1".into()),
            task_id: Some("task:1".into()),
            run_id: Some("run:1".into()),
            kind: EventKind::RequestCreated,
            detail: "created".into(),
        }
    }

    #[test]
    fn publishes_in_order_and_filters_by_request_and_agent() {
        let mut bus = EventBus::new();
        bus.publish(event("event:1", "request:1", Some("agent:one")))
            .unwrap();
        bus.publish(event("event:2", "request:2", Some("agent:two")))
            .unwrap();
        assert_eq!(bus.len(), 2);
        assert_eq!(bus.events_for_request("request:1").len(), 1);
        assert_eq!(bus.events_for_agent("agent:one").len(), 1);
    }

    #[test]
    fn duplicate_event_id_fails_closed() {
        let mut bus = EventBus::new();
        bus.publish(event("event:1", "request:1", Some("agent:one")))
            .unwrap();
        assert_eq!(
            bus.publish(event("event:1", "request:2", Some("agent:two"))),
            Err(EventBusError::DuplicateEventId)
        );
        assert_eq!(bus.len(), 1);
    }
}
