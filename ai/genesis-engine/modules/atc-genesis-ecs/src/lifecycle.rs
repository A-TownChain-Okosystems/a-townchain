use atc_genesis_platform::EntityId;
use std::any::TypeId;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleEvent {
    Spawned(EntityId),
    Despawned(EntityId),
    ComponentInserted { entity: EntityId, component: TypeId },
    ComponentRemoved { entity: EntityId, component: TypeId },
}

pub trait LifecycleHook: Send + Sync {
    fn on_event(&mut self, event: LifecycleEvent);
}

#[derive(Default)]
pub struct LifecycleHooks {
    hooks: Vec<Box<dyn LifecycleHook>>,
    events: Vec<LifecycleEvent>,
}

impl LifecycleHooks {
    pub fn register<H: LifecycleHook + 'static>(&mut self, hook: H) {
        self.hooks.push(Box::new(hook));
    }

    pub fn emit(&mut self, event: LifecycleEvent) {
        for hook in &mut self.hooks {
            hook.on_event(event);
        }
        self.events.push(event);
    }

    pub fn events(&self) -> &[LifecycleEvent] {
        &self.events
    }

    pub fn drain_events(&mut self) -> Vec<LifecycleEvent> {
        std::mem::take(&mut self.events)
    }
}

#[derive(Default)]
pub struct ChangeTracker {
    added: HashSet<EntityId>,
    changed: HashSet<EntityId>,
}

impl ChangeTracker {
    pub fn mark_added(&mut self, entity: EntityId) {
        self.added.insert(entity);
        self.changed.insert(entity);
    }

    pub fn mark_changed(&mut self, entity: EntityId) {
        self.changed.insert(entity);
    }

    pub fn is_added(&self, entity: EntityId) -> bool {
        self.added.contains(&entity)
    }

    pub fn is_changed(&self, entity: EntityId) -> bool {
        self.changed.contains(&entity)
    }

    pub fn clear(&mut self) {
        self.added.clear();
        self.changed.clear();
    }

    pub fn clear_entity(&mut self, entity: EntityId) {
        self.added.remove(&entity);
        self.changed.remove(&entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct RecordingHook {
        events: Vec<LifecycleEvent>,
    }

    impl LifecycleHook for RecordingHook {
        fn on_event(&mut self, event: LifecycleEvent) {
            self.events.push(event);
        }
    }

    #[test]
    fn hooks_receive_and_queue_events() {
        let entity = EntityId(7);
        let mut hooks = LifecycleHooks::default();
        hooks.register(RecordingHook { events: Vec::new() });
        hooks.emit(LifecycleEvent::Spawned(entity));
        assert_eq!(hooks.events(), &[LifecycleEvent::Spawned(entity)]);
        assert_eq!(hooks.drain_events(), vec![LifecycleEvent::Spawned(entity)]);
        assert!(hooks.events().is_empty());
    }

    #[test]
    fn change_tracker_distinguishes_added_and_changed() {
        let entity = EntityId(7);
        let mut tracker = ChangeTracker::default();
        tracker.mark_added(entity);
        assert!(tracker.is_added(entity));
        assert!(tracker.is_changed(entity));
        tracker.clear();
        assert!(!tracker.is_added(entity));
        tracker.mark_changed(entity);
        assert!(!tracker.is_added(entity));
        assert!(tracker.is_changed(entity));
    }
}
