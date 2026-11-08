use atc_genesis_platform::{EntityId, FrameId, Renderer, Transform};
use atc_genesis_world::{ChunkState, WorldChunkId, WorldStreamer};
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Parent(pub EntityId);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EntityRecord {
    pub id: EntityId,
}

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

    fn emit(&mut self, event: LifecycleEvent) {
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

trait ComponentTable: Any + Send + Sync {
    fn remove_entity(&mut self, id: EntityId);
    fn contains_entity(&self, id: EntityId) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn clear_changes(&mut self);
    fn is_added(&self, id: EntityId) -> bool;
    fn is_changed(&self, id: EntityId) -> bool;
}

struct TypedComponentTable<T: Any + Send + Sync> {
    values: HashMap<EntityId, T>,
    added: HashSet<EntityId>,
    changed: HashSet<EntityId>,
}

impl<T: Any + Send + Sync> ComponentTable for TypedComponentTable<T> {
    fn remove_entity(&mut self, id: EntityId) {
        self.values.remove(&id);
        self.added.remove(&id);
        self.changed.remove(&id);
    }

    fn contains_entity(&self, id: EntityId) -> bool {
        self.values.contains_key(&id)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn clear_changes(&mut self) {
        self.added.clear();
        self.changed.clear();
    }

    fn is_added(&self, id: EntityId) -> bool {
        self.added.contains(&id)
    }

    fn is_changed(&self, id: EntityId) -> bool {
        self.changed.contains(&id)
    }
}

#[derive(Default)]
pub struct ResourceStore {
    values: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ResourceStore {
    pub fn insert<T: Any + Send + Sync>(&mut self, value: T) -> Option<T> {
        self.values
            .insert(TypeId::of::<T>(), Box::new(value))
            .and_then(|value| value.downcast::<T>().ok().map(|value| *value))
    }

    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.values
            .get(&TypeId::of::<T>())
            .and_then(|value| value.downcast_ref())
    }

    pub fn get_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.values
            .get_mut(&TypeId::of::<T>())
            .and_then(|value| value.downcast_mut())
    }

    pub fn remove<T: Any + Send + Sync>(&mut self) -> Option<T> {
        self.values
            .remove(&TypeId::of::<T>())
            .and_then(|value| value.downcast::<T>().ok().map(|value| *value))
    }

    pub fn contains<T: Any + Send + Sync>(&self) -> bool {
        self.values.contains_key(&TypeId::of::<T>())
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SystemId(pub u32);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SystemAccess {
    read_components: HashSet<TypeId>,
    write_components: HashSet<TypeId>,
    read_resources: HashSet<TypeId>,
    write_resources: HashSet<TypeId>,
}

impl SystemAccess {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_component<T: Any + Send + Sync>(mut self) -> Self {
        self.read_components.insert(TypeId::of::<T>());
        self
    }

    pub fn write_component<T: Any + Send + Sync>(mut self) -> Self {
        self.write_components.insert(TypeId::of::<T>());
        self
    }

    pub fn read_resource<T: Any + Send + Sync>(mut self) -> Self {
        self.read_resources.insert(TypeId::of::<T>());
        self
    }

    pub fn write_resource<T: Any + Send + Sync>(mut self) -> Self {
        self.write_resources.insert(TypeId::of::<T>());
        self
    }

    pub fn conflicts_with(&self, other: &Self) -> bool {
        intersects(&self.write_components, &other.read_components)
            || intersects(&self.write_components, &other.write_components)
            || intersects(&other.write_components, &self.read_components)
            || intersects(&self.write_resources, &other.read_resources)
            || intersects(&self.write_resources, &other.write_resources)
            || intersects(&other.write_resources, &self.read_resources)
    }
}

fn intersects<T: Eq + std::hash::Hash>(a: &HashSet<T>, b: &HashSet<T>) -> bool {
    a.iter().any(|value| b.contains(value))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SystemDescriptor {
    pub id: SystemId,
    pub name: String,
    before: Vec<SystemId>,
    after: Vec<SystemId>,
    access: SystemAccess,
}

impl SystemDescriptor {
    pub fn new(id: SystemId, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            before: Vec::new(),
            after: Vec::new(),
            access: SystemAccess::new(),
        }
    }

    pub fn before(mut self, system: SystemId) -> Self {
        self.before.push(system);
        self
    }

    pub fn after(mut self, system: SystemId) -> Self {
        self.after.push(system);
        self
    }

    pub fn access(mut self, access: SystemAccess) -> Self {
        self.access = access;
        self
    }

    pub fn read_component<T: Any + Send + Sync>(mut self) -> Self {
        self.access = self.access.read_component::<T>();
        self
    }

    pub fn write_component<T: Any + Send + Sync>(mut self) -> Self {
        self.access = self.access.write_component::<T>();
        self
    }

    pub fn read_resource<T: Any + Send + Sync>(mut self) -> Self {
        self.access = self.access.read_resource::<T>();
        self
    }

    pub fn write_resource<T: Any + Send + Sync>(mut self) -> Self {
        self.access = self.access.write_resource::<T>();
        self
    }

    pub fn access_contract(&self) -> &SystemAccess {
        &self.access
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScheduleError {
    DuplicateSystem(SystemId),
    DependencyCycle(Vec<SystemId>),
    MissingDependency {
        system: SystemId,
        dependency: SystemId,
    },
}

#[derive(Default)]
pub struct SystemSchedule {
    systems: HashMap<SystemId, SystemDescriptor>,
}

impl SystemSchedule {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, descriptor: SystemDescriptor) -> Result<(), ScheduleError> {
        if self.systems.contains_key(&descriptor.id) {
            return Err(ScheduleError::DuplicateSystem(descriptor.id));
        }
        self.systems.insert(descriptor.id, descriptor);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.systems.len()
    }

    pub fn is_empty(&self) -> bool {
        self.systems.is_empty()
    }

    pub fn get(&self, id: SystemId) -> Option<&SystemDescriptor> {
        self.systems.get(&id)
    }

    fn dependency_sets(&self) -> Result<HashMap<SystemId, HashSet<SystemId>>, ScheduleError> {
        let mut incoming: HashMap<_, HashSet<_>> = self
            .systems
            .keys()
            .copied()
            .map(|id| (id, HashSet::new()))
            .collect();

        for descriptor in self.systems.values() {
            for dependency in &descriptor.after {
                if !self.systems.contains_key(dependency) {
                    return Err(ScheduleError::MissingDependency {
                        system: descriptor.id,
                        dependency: *dependency,
                    });
                }
                incoming
                    .get_mut(&descriptor.id)
                    .expect("registered system invariant")
                    .insert(*dependency);
            }

            for dependency in &descriptor.before {
                if !self.systems.contains_key(dependency) {
                    return Err(ScheduleError::MissingDependency {
                        system: descriptor.id,
                        dependency: *dependency,
                    });
                }
                incoming
                    .get_mut(dependency)
                    .expect("registered system invariant")
                    .insert(descriptor.id);
            }
        }

        Ok(incoming)
    }

    pub fn ordered_systems(&self) -> Result<Vec<SystemId>, ScheduleError> {
        let mut incoming = self.dependency_sets()?;
        let mut ready: Vec<_> = incoming
            .iter()
            .filter_map(|(id, dependencies)| dependencies.is_empty().then_some(*id))
            .collect();
        ready.sort();

        let mut ordered = Vec::with_capacity(self.systems.len());
        while let Some(id) = ready.first().copied() {
            ready.remove(0);
            ordered.push(id);

            let dependents: Vec<_> = incoming
                .iter()
                .filter_map(|(candidate, dependencies)| {
                    dependencies.contains(&id).then_some(*candidate)
                })
                .collect();

            for dependent in dependents {
                let dependencies = incoming
                    .get_mut(&dependent)
                    .expect("scheduled system invariant");
                dependencies.remove(&id);
                if dependencies.is_empty() {
                    ready.push(dependent);
                }
            }
            ready.sort();
        }

        if ordered.len() != self.systems.len() {
            let mut cycle: Vec<_> = incoming
                .iter()
                .filter_map(|(id, dependencies)| (!dependencies.is_empty()).then_some(*id))
                .collect();
            cycle.sort();
            return Err(ScheduleError::DependencyCycle(cycle));
        }

        Ok(ordered)
    }

    pub fn parallel_batches(&self) -> Result<Vec<Vec<SystemId>>, ScheduleError> {
        let order = self.ordered_systems()?;
        let dependencies = self.dependency_sets()?;
        let mut batches: Vec<Vec<SystemId>> = Vec::new();

        for id in order {
            let descriptor = self.systems.get(&id).expect("scheduled system invariant");
            let mut placed = false;

            for batch in &mut batches {
                let dependency_in_batch = batch.iter().any(|other| {
                    dependencies
                        .get(&id)
                        .expect("dependency map invariant")
                        .contains(other)
                });
                let access_conflict = batch.iter().any(|other| {
                    self.systems
                        .get(other)
                        .expect("scheduled system invariant")
                        .access
                        .conflicts_with(&descriptor.access)
                });

                if !dependency_in_batch && !access_conflict {
                    batch.push(id);
                    batch.sort();
                    placed = true;
                    break;
                }
            }

            if !placed {
                batches.push(vec![id]);
            }
        }

        Ok(batches)
    }
}

pub type SystemFn = Box<dyn FnMut(&mut World) + Send + 'static>;

pub struct ExecutableSystem {
    pub descriptor: SystemDescriptor,
    pub run: SystemFn,
}

#[derive(Default)]
pub struct SystemExecutor {
    systems: HashMap<SystemId, ExecutableSystem>,
}

impl SystemExecutor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<F>(&mut self, descriptor: SystemDescriptor, run: F) -> Result<(), ScheduleError>
    where
        F: FnMut(&mut World) + Send + 'static,
    {
        if self.systems.contains_key(&descriptor.id) {
            return Err(ScheduleError::DuplicateSystem(descriptor.id));
        }
        self.systems.insert(
            descriptor.id,
            ExecutableSystem {
                descriptor,
                run: Box::new(run),
            },
        );
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.systems.len()
    }

    /// Gegenstueck zu `len` (Clippy: `len_without_is_empty`).
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn parallel_batches(&self) -> Result<Vec<Vec<SystemId>>, ScheduleError> {
        let mut schedule = SystemSchedule::new();
        for system in self.systems.values() {
            schedule.register(system.descriptor.clone())?;
        }
        schedule.parallel_batches()
    }

    pub fn run(&mut self, world: &mut World) -> Result<Vec<SystemId>, ScheduleError> {
        let mut schedule = SystemSchedule::new();
        for system in self.systems.values() {
            schedule.register(system.descriptor.clone())?;
        }
        let order = schedule.ordered_systems()?;
        for id in &order {
            self.systems
                .get_mut(id)
                .expect("scheduled system invariant")
                .run
                .as_mut()(world);
        }
        Ok(order)
    }
}

#[derive(Default)]
pub struct World {
    next: u64,
    transforms: HashMap<EntityId, Transform>,
    parents: HashMap<EntityId, Parent>,
    components: HashMap<TypeId, Box<dyn ComponentTable>>,
    resources: ResourceStore,
    lifecycle: LifecycleHooks,
}

impl World {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self, transform: Transform) -> EntityId {
        self.next = self.next.saturating_add(1);
        let id = EntityId(self.next);
        self.transforms.insert(id, transform);
        self.lifecycle.emit(LifecycleEvent::Spawned(id));
        id
    }

    pub fn insert(&mut self, id: EntityId, transform: Transform) -> bool {
        if self.transforms.contains_key(&id) {
            return false;
        }
        self.next = self.next.max(id.0);
        self.transforms.insert(id, transform);
        self.lifecycle.emit(LifecycleEvent::Spawned(id));
        true
    }

    pub fn contains(&self, id: EntityId) -> bool {
        self.transforms.contains_key(&id)
    }

    pub fn despawn(&mut self, id: EntityId) -> bool {
        if self.transforms.remove(&id).is_none() {
            return false;
        }
        self.parents.remove(&id);
        self.parents.retain(|_, parent| parent.0 != id);
        for table in self.components.values_mut() {
            table.remove_entity(id);
        }
        self.lifecycle.emit(LifecycleEvent::Despawned(id));
        true
    }

    pub fn set_transform(&mut self, id: EntityId, transform: Transform) -> bool {
        if let Some(slot) = self.transforms.get_mut(&id) {
            *slot = transform;
            true
        } else {
            false
        }
    }

    pub fn transform(&self, id: EntityId) -> Option<&Transform> {
        self.transforms.get(&id)
    }

    fn component_table<T: Any + Send + Sync>(&mut self) -> &mut TypedComponentTable<T> {
        self.components
            .entry(TypeId::of::<T>())
            .or_insert_with(|| {
                Box::new(TypedComponentTable::<T> {
                    values: HashMap::new(),
                    added: HashSet::new(),
                    changed: HashSet::new(),
                })
            })
            .as_any_mut()
            .downcast_mut::<TypedComponentTable<T>>()
            .expect("component type table invariant")
    }

    pub fn insert_component<T: Any + Send + Sync>(&mut self, id: EntityId, component: T) -> bool {
        if !self.contains(id) {
            return false;
        }
        let type_id = TypeId::of::<T>();
        let inserted = {
            let table = self.component_table::<T>();
            let inserted = table.values.insert(id, component).is_none();
            if inserted {
                table.added.insert(id);
            }
            table.changed.insert(id);
            inserted
        };
        if inserted {
            self.lifecycle.emit(LifecycleEvent::ComponentInserted {
                entity: id,
                component: type_id,
            });
        }
        inserted
    }

    pub fn set_component<T: Any + Send + Sync>(&mut self, id: EntityId, component: T) -> bool {
        if !self.contains(id) {
            return false;
        }
        let type_id = TypeId::of::<T>();
        let replaced = {
            let table = self.component_table::<T>();
            let replaced = table.values.insert(id, component).is_some();
            if !replaced {
                table.added.insert(id);
            }
            table.changed.insert(id);
            replaced
        };
        if !replaced {
            self.lifecycle.emit(LifecycleEvent::ComponentInserted {
                entity: id,
                component: type_id,
            });
        }
        replaced
    }

    pub fn component<T: Any + Send + Sync>(&self, id: EntityId) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())?
            .as_any()
            .downcast_ref::<TypedComponentTable<T>>()?
            .values
            .get(&id)
    }

    pub fn component_mut<T: Any + Send + Sync>(&mut self, id: EntityId) -> Option<&mut T> {
        let table = self
            .components
            .get_mut(&TypeId::of::<T>())?
            .as_any_mut()
            .downcast_mut::<TypedComponentTable<T>>()?;
        if !table.values.contains_key(&id) {
            return None;
        }
        table.changed.insert(id);
        table.values.get_mut(&id)
    }

    pub fn remove_component<T: Any + Send + Sync>(&mut self, id: EntityId) -> Option<T> {
        let type_id = TypeId::of::<T>();
        let removed = {
            let table = self.components.get_mut(&type_id)?;
            let table = table
                .as_any_mut()
                .downcast_mut::<TypedComponentTable<T>>()?;
            let removed = table.values.remove(&id);
            if removed.is_some() {
                table.added.remove(&id);
                table.changed.remove(&id);
            }
            removed
        };
        if removed.is_some() {
            self.lifecycle.emit(LifecycleEvent::ComponentRemoved {
                entity: id,
                component: type_id,
            });
        }
        removed
    }

    pub fn has_component<T: Any + Send + Sync>(&self, id: EntityId) -> bool {
        self.components
            .get(&TypeId::of::<T>())
            .is_some_and(|table| table.contains_entity(id))
    }

    pub fn is_component_added<T: Any + Send + Sync>(&self, id: EntityId) -> bool {
        self.components
            .get(&TypeId::of::<T>())
            .is_some_and(|table| table.is_added(id))
    }

    pub fn is_component_changed<T: Any + Send + Sync>(&self, id: EntityId) -> bool {
        self.components
            .get(&TypeId::of::<T>())
            .is_some_and(|table| table.is_changed(id))
    }

    pub fn clear_change_tracking(&mut self) {
        for table in self.components.values_mut() {
            table.clear_changes();
        }
    }

    pub fn lifecycle_hooks(&mut self) -> &mut LifecycleHooks {
        &mut self.lifecycle
    }

    pub fn lifecycle_events(&self) -> &[LifecycleEvent] {
        self.lifecycle.events()
    }

    pub fn drain_lifecycle_events(&mut self) -> Vec<LifecycleEvent> {
        self.lifecycle.drain_events()
    }

    pub fn insert_resource<T: Any + Send + Sync>(&mut self, resource: T) -> Option<T> {
        self.resources.insert(resource)
    }

    pub fn resource<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.resources.get()
    }

    pub fn resource_mut<T: Any + Send + Sync>(&mut self) -> Option<&mut T> {
        self.resources.get_mut()
    }

    pub fn remove_resource<T: Any + Send + Sync>(&mut self) -> Option<T> {
        self.resources.remove()
    }

    pub fn has_resource<T: Any + Send + Sync>(&self) -> bool {
        self.resources.contains::<T>()
    }

    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }

    pub fn set_parent(&mut self, child: EntityId, parent: Option<EntityId>) -> bool {
        if !self.contains(child)
            || parent == Some(child)
            || parent.is_some_and(|parent| !self.contains(parent))
        {
            return false;
        }

        if let Some(parent) = parent {
            if self.would_cycle(child, parent) {
                return false;
            }
            self.parents.insert(child, Parent(parent));
        } else {
            self.parents.remove(&child);
        }
        true
    }

    fn would_cycle(&self, child: EntityId, proposed_parent: EntityId) -> bool {
        let mut current = proposed_parent;
        for _ in 0..=self.parents.len() {
            if current == child {
                return true;
            }
            match self.parents.get(&current) {
                Some(parent) => current = parent.0,
                None => return false,
            }
        }
        true
    }

    pub fn parent(&self, child: EntityId) -> Option<EntityId> {
        self.parents.get(&child).map(|parent| parent.0)
    }

    pub fn len(&self) -> usize {
        self.transforms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }

    pub fn entities(&self) -> Vec<EntityId> {
        let mut ids: Vec<_> = self.transforms.keys().copied().collect();
        ids.sort_by_key(|id| id.0);
        ids
    }

    pub fn iter_transforms(&self) -> impl Iterator<Item = (EntityId, &Transform)> {
        self.entities()
            .into_iter()
            .filter_map(|id| self.transforms.get(&id).map(|transform| (id, transform)))
    }

    pub fn query1<A: Any + Send + Sync>(&self) -> Vec<(EntityId, &A)> {
        self.entities()
            .into_iter()
            .filter_map(|id| self.component::<A>(id).map(|component| (id, component)))
            .collect()
    }

    pub fn query2<A: Any + Send + Sync, B: Any + Send + Sync>(&self) -> Vec<(EntityId, &A, &B)> {
        self.entities()
            .into_iter()
            .filter_map(|id| Some((id, self.component::<A>(id)?, self.component::<B>(id)?)))
            .collect()
    }

    pub fn query3<A: Any + Send + Sync, B: Any + Send + Sync, C: Any + Send + Sync>(
        &self,
    ) -> Vec<(EntityId, &A, &B, &C)> {
        self.entities()
            .into_iter()
            .filter_map(|id| {
                Some((
                    id,
                    self.component::<A>(id)?,
                    self.component::<B>(id)?,
                    self.component::<C>(id)?,
                ))
            })
            .collect()
    }

    pub fn query_with<A: Any + Send + Sync, F: FnMut(EntityId, &A) -> bool>(
        &self,
        mut filter: F,
    ) -> Vec<(EntityId, &A)> {
        self.query1::<A>()
            .into_iter()
            .filter(|(id, component)| filter(*id, *component))
            .collect()
    }

    pub fn world_transform(&self, id: EntityId) -> Option<Transform> {
        if !self.contains(id) {
            return None;
        }

        let mut chain = Vec::new();
        let mut current = id;
        for _ in 0..=self.parents.len() {
            chain.push(current);
            match self.parents.get(&current) {
                Some(parent) => current = parent.0,
                None => break,
            }
        }

        if chain.len() > self.parents.len() + 1 {
            return None;
        }

        let mut result = *self.transforms.get(chain.last()?)?;
        for entity in chain.iter().rev().skip(1) {
            result = combine(result, *self.transforms.get(entity)?);
        }
        Some(result)
    }

    pub fn query_world_transforms(&self) -> Vec<(EntityId, Transform)> {
        self.entities()
            .into_iter()
            .filter_map(|id| self.world_transform(id).map(|transform| (id, transform)))
            .collect()
    }
}

fn combine(parent: Transform, local: Transform) -> Transform {
    Transform {
        translation: [
            parent.translation[0] + local.translation[0] * parent.scale[0],
            parent.translation[1] + local.translation[1] * parent.scale[1],
            parent.translation[2] + local.translation[2] * parent.scale[2],
        ],
        rotation_xyzw: quat_mul(parent.rotation_xyzw, local.rotation_xyzw),
        scale: [
            parent.scale[0] * local.scale[0],
            parent.scale[1] * local.scale[1],
            parent.scale[2] * local.scale[2],
        ],
    }
}

fn quat_mul(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [
        a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1],
        a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0],
        a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3],
        a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2],
    ]
}

pub struct TransformRenderPipeline<R> {
    pub world: World,
    pub renderer: R,
}

impl<R: Renderer> TransformRenderPipeline<R> {
    pub fn render(&mut self, frame: FrameId) {
        self.renderer.begin_frame(frame);
        for (id, transform) in self.world.query_world_transforms() {
            self.renderer.submit(id, transform);
        }
        self.renderer.end_frame();
    }
}

#[derive(Default)]
pub struct WorldEcsBridge {
    chunk_entities: HashMap<WorldChunkId, EntityId>,
}

impl WorldEcsBridge {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entity_for_chunk(&self, chunk: WorldChunkId) -> Option<EntityId> {
        self.chunk_entities.get(&chunk).copied()
    }

    pub fn sync(
        &mut self,
        world: &WorldStreamer,
        ecs: &mut World,
    ) -> Vec<(WorldChunkId, EntityId)> {
        let mut loaded = Vec::new();

        for chunk in world.chunks() {
            if chunk.state != ChunkState::Loaded {
                continue;
            }

            let entity = *self
                .chunk_entities
                .entry(chunk.id)
                .or_insert_with(|| EntityId(u64::MAX - chunk.id.0));
            let center = [
                (chunk.bounds.min[0] + chunk.bounds.max[0]) * 0.5,
                (chunk.bounds.min[1] + chunk.bounds.max[1]) * 0.5,
                (chunk.bounds.min[2] + chunk.bounds.max[2]) * 0.5,
            ];
            let transform = Transform {
                translation: center,
                ..Default::default()
            };

            if !ecs.set_transform(entity, transform) && !ecs.insert(entity, transform) {
                continue;
            }
            loaded.push((chunk.id, entity));
        }

        let active: HashSet<_> = world
            .chunks()
            .iter()
            .filter(|chunk| chunk.state == ChunkState::Loaded)
            .map(|chunk| chunk.id)
            .collect();
        let stale: Vec<_> = self
            .chunk_entities
            .keys()
            .copied()
            .filter(|id| !active.contains(id))
            .collect();

        for chunk in stale {
            if let Some(entity) = self.chunk_entities.remove(&chunk) {
                ecs.despawn(entity);
            }
        }

        loaded.sort_by_key(|(chunk, _)| chunk.0);
        loaded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Health(u32);

    #[derive(Debug, PartialEq)]
    struct Position(u32);

    #[derive(Debug, PartialEq)]
    struct Velocity(u32);

    #[test]
    fn change_tracking_works() {
        let mut world = World::new();
        let entity = world.spawn(Transform::default());
        assert!(world.insert_component(entity, Health(10)));
        assert!(world.is_component_added::<Health>(entity));
        assert!(world.is_component_changed::<Health>(entity));
        world.clear_change_tracking();
        *world.component_mut::<Health>(entity).unwrap() = Health(5);
        assert!(!world.is_component_added::<Health>(entity));
        assert!(world.is_component_changed::<Health>(entity));
    }

    #[test]
    fn lifecycle_events_are_ordered() {
        let mut world = World::new();
        let entity = world.spawn(Transform::default());
        world.set_component(entity, Health(1));
        assert!(matches!(
            world.lifecycle_events()[0],
            LifecycleEvent::Spawned(id) if id == entity
        ));
        assert!(matches!(
            world.lifecycle_events()[1],
            LifecycleEvent::ComponentInserted { entity: id, .. } if id == entity
        ));
        assert!(world.despawn(entity));
        assert!(matches!(
            world.lifecycle_events()[2],
            LifecycleEvent::Despawned(id) if id == entity
        ));
    }

    #[test]
    fn executor_runs_in_dependency_order() {
        let mut world = World::new();
        let mut executor = SystemExecutor::new();
        executor
            .register(
                SystemDescriptor::new(SystemId(2), "b").after(SystemId(1)),
                |world| {
                    world.insert_resource(2u32);
                },
            )
            .unwrap();
        executor
            .register(SystemDescriptor::new(SystemId(1), "a"), |world| {
                world.insert_resource(1u32);
            })
            .unwrap();
        assert_eq!(
            executor.run(&mut world).unwrap(),
            vec![SystemId(1), SystemId(2)]
        );
        assert_eq!(world.resource::<u32>(), Some(&2));
    }

    #[test]
    fn hierarchy_resolves() {
        let mut world = World::new();
        let parent = world.spawn(Transform {
            translation: [2.0, 0.0, 0.0],
            ..Default::default()
        });
        let child = world.spawn(Transform {
            translation: [1.0, 0.0, 0.0],
            ..Default::default()
        });
        assert!(world.set_parent(child, Some(parent)));
        assert_eq!(world.world_transform(child).unwrap().translation[0], 3.0);
    }

    #[test]
    fn query3_returns_only_entities_with_all_components() {
        let mut world = World::new();
        let complete = world.spawn(Transform::default());
        let partial = world.spawn(Transform::default());
        world.insert_component(complete, Health(1));
        world.insert_component(complete, Position(2));
        world.insert_component(complete, Velocity(3));
        world.insert_component(partial, Health(4));
        world.insert_component(partial, Position(5));

        let result = world.query3::<Health, Position, Velocity>();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, complete);
        assert_eq!(result[0].1, &Health(1));
        assert_eq!(result[0].2, &Position(2));
        assert_eq!(result[0].3, &Velocity(3));
    }

    #[test]
    fn query_with_filters_deterministically() {
        let mut world = World::new();
        for value in [3, 1, 2] {
            let entity = world.spawn(Transform::default());
            world.insert_component(entity, Health(value));
        }
        let result = world.query_with::<Health, _>(|_, health| health.0 >= 2);
        assert_eq!(
            result
                .iter()
                .map(|(_, health)| health.0)
                .collect::<Vec<_>>(),
            vec![3, 2]
        );
    }

    #[test]
    fn access_contracts_detect_conflicts() {
        let write = SystemAccess::new().write_component::<Health>();
        let read = SystemAccess::new().read_component::<Health>();
        let unrelated = SystemAccess::new().write_component::<Position>();
        assert!(write.conflicts_with(&read));
        assert!(read.conflicts_with(&write));
        assert!(!write.conflicts_with(&unrelated));
    }

    #[test]
    fn parallel_batches_are_deterministic_and_dependency_safe() {
        let mut schedule = SystemSchedule::new();
        schedule
            .register(
                SystemDescriptor::new(SystemId(2), "read-position").read_component::<Position>(),
            )
            .unwrap();
        schedule
            .register(SystemDescriptor::new(SystemId(1), "read-health").read_component::<Health>())
            .unwrap();
        schedule
            .register(
                SystemDescriptor::new(SystemId(3), "write-health")
                    .write_component::<Health>()
                    .after(SystemId(1)),
            )
            .unwrap();

        assert_eq!(
            schedule.parallel_batches().unwrap(),
            vec![vec![SystemId(1), SystemId(2)], vec![SystemId(3)]]
        );
    }
}
