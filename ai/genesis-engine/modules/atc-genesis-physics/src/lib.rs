use atc_genesis_ecs::World;
use atc_genesis_platform::{EntityId, PhysicsWorld};
use atc_genesis_world::{ChunkState, WorldChunkId, WorldStreamer};
mod collision;
pub use collision::{Aabb, Collider, CollisionWorld};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidBody {
    pub entity: EntityId,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub mass: f32,
    pub dynamic: bool,
    pub restitution: f32,
}

impl RigidBody {
    pub fn dynamic(entity: EntityId, position: [f32; 3], mass: f32) -> Self {
        Self {
            entity,
            position,
            velocity: [0.0; 3],
            mass: mass.max(0.0001),
            dynamic: true,
            restitution: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicsConfig {
    pub fixed_dt: f32,
    pub gravity: [f32; 3],
    pub max_substeps: u32,
    pub max_collision_iterations: u32,
    pub floor_y: Option<f32>,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            fixed_dt: 1.0 / 60.0,
            gravity: [0.0, -9.81, 0.0],
            max_substeps: 8,
            max_collision_iterations: 4,
            floor_y: None,
        }
    }
}

#[derive(Default)]
pub struct PhysicsSimulation {
    pub config: PhysicsConfig,
    accumulator: f32,
    bodies: Vec<RigidBody>,
    collisions: CollisionWorld,
}

impl PhysicsSimulation {
    pub fn new(config: PhysicsConfig) -> Self {
        Self {
            config: PhysicsConfig {
                fixed_dt: config.fixed_dt.max(1e-6),
                max_substeps: config.max_substeps.max(1),
                max_collision_iterations: config.max_collision_iterations.max(1),
                ..config
            },
            ..Self::default()
        }
    }

    pub fn add_body(&mut self, body: RigidBody) {
        self.bodies.push(body);
    }
    pub fn remove_body(&mut self, entity: EntityId) -> bool {
        if let Some(i) = self.bodies.iter().position(|b| b.entity == entity) {
            self.bodies.remove(i);
            self.collisions.remove(entity);
            true
        } else {
            false
        }
    }
    pub fn bodies(&self) -> &[RigidBody] {
        &self.bodies
    }
    pub fn body(&self, entity: EntityId) -> Option<&RigidBody> {
        self.bodies.iter().find(|b| b.entity == entity)
    }
    pub fn body_mut(&mut self, entity: EntityId) -> Option<&mut RigidBody> {
        self.bodies.iter_mut().find(|b| b.entity == entity)
    }

    fn resolve_collisions(body: &mut RigidBody, collisions: &CollisionWorld, iterations: u32) {
        for _ in 0..iterations {
            let Some((_entity, normal, penetration)) = collisions.resolve_point(body.position)
            else {
                break;
            };
            let correction = penetration + 1e-5;
            for (pos, n) in body.position.iter_mut().zip(normal) {
                *pos += n * correction;
            }
            let inward_velocity = body.velocity[0] * normal[0]
                + body.velocity[1] * normal[1]
                + body.velocity[2] * normal[2];
            if inward_velocity < 0.0 {
                let impulse = (1.0 + body.restitution.clamp(0.0, 1.0)) * inward_velocity;
                for (v, n) in body.velocity.iter_mut().zip(normal) {
                    *v -= n * impulse;
                }
            }
        }
    }

    pub fn step_fixed(&mut self) {
        let dt = self.config.fixed_dt;
        for body in &mut self.bodies {
            if !body.dynamic {
                continue;
            }
            for i in 0..3 {
                body.velocity[i] += self.config.gravity[i] * dt;
                body.position[i] += body.velocity[i] * dt;
            }
            if let Some(floor) = self.config.floor_y {
                if body.position[1] < floor {
                    body.position[1] = floor;
                    if body.velocity[1] < 0.0 {
                        body.velocity[1] *= -body.restitution.clamp(0.0, 1.0);
                    }
                }
            }
            Self::resolve_collisions(body, &self.collisions, self.config.max_collision_iterations);
        }
    }

    pub fn advance(&mut self, dt: f32) -> u32 {
        self.accumulator += dt.clamp(0.0, 1.0);
        let mut steps = 0;
        while self.accumulator >= self.config.fixed_dt && steps < self.config.max_substeps {
            self.step_fixed();
            self.accumulator -= self.config.fixed_dt;
            steps += 1;
        }
        steps
    }

    pub fn sync_to_world(&self, world: &mut World) -> usize {
        let mut count = 0;
        for body in &self.bodies {
            if world.set_transform(
                body.entity,
                atc_genesis_platform::Transform {
                    translation: body.position,
                    ..world.transform(body.entity).copied().unwrap_or_default()
                },
            ) {
                count += 1;
            }
        }
        count
    }

    pub fn add_collider(&mut self, collider: Collider) {
        self.collisions.add(collider);
    }
    pub fn remove_collider(&mut self, entity: EntityId) -> bool {
        self.collisions.remove(entity)
    }
    pub fn query(&self, bounds: Aabb) -> Vec<EntityId> {
        self.collisions.query_aabb(bounds)
    }
    pub fn interpolation_alpha(&self) -> f32 {
        (self.accumulator / self.config.fixed_dt).clamp(0.0, 1.0)
    }
}

/// Mirrors loaded world chunks into the physics collision world.
/// Chunk entities use the same deterministic reserved ID scheme as the ECS bridge.
#[derive(Default)]
pub struct WorldPhysicsBridge {
    chunk_entities: std::collections::HashMap<WorldChunkId, EntityId>,
}

impl WorldPhysicsBridge {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn entity_for_chunk(&self, chunk: WorldChunkId) -> Option<EntityId> {
        self.chunk_entities.get(&chunk).copied()
    }

    pub fn sync(
        &mut self,
        world: &WorldStreamer,
        physics: &mut PhysicsSimulation,
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
            let bounds = Aabb {
                min: chunk.bounds.min,
                max: chunk.bounds.max,
            };
            physics.remove_collider(entity);
            physics.add_collider(Collider { entity, bounds });
            loaded.push((chunk.id, entity));
        }
        let active: std::collections::HashSet<WorldChunkId> = world
            .chunks()
            .iter()
            .filter(|c| c.state == ChunkState::Loaded)
            .map(|c| c.id)
            .collect();
        let stale: Vec<WorldChunkId> = self
            .chunk_entities
            .keys()
            .copied()
            .filter(|id| !active.contains(id))
            .collect();
        for chunk in stale {
            if let Some(entity) = self.chunk_entities.remove(&chunk) {
                physics.remove_collider(entity);
            }
        }
        loaded.sort_by_key(|(chunk, _)| chunk.0);
        loaded
    }
}

impl PhysicsWorld for PhysicsSimulation {
    fn step(&mut self, dt: f32) {
        let _ = self.advance(dt);
    }
    fn raycast(
        &self,
        origin: [f32; 3],
        direction: [f32; 3],
        max_distance: f32,
    ) -> Option<EntityId> {
        self.collisions.raycast(origin, direction, max_distance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_step_applies_gravity() {
        let mut sim = PhysicsSimulation::new(PhysicsConfig::default());
        sim.add_body(RigidBody::dynamic(EntityId(1), [0.0, 10.0, 0.0], 1.0));
        assert_eq!(sim.advance(1.0 / 60.0), 1);
        assert!(sim.body(EntityId(1)).unwrap().position[1] < 10.0);
    }

    #[test]
    fn floor_stops_body() {
        let c = PhysicsConfig {
            gravity: [0.0, -10.0, 0.0],
            floor_y: Some(0.0),
            ..Default::default()
        };
        let mut s = PhysicsSimulation::new(c);
        s.add_body(RigidBody::dynamic(EntityId(1), [0.0, 0.01, 0.0], 1.0));
        for _ in 0..10 {
            s.advance(1.0 / 60.0);
        }
        assert!(s.body(EntityId(1)).unwrap().position[1] >= 0.0);
    }

    #[test]
    fn static_collider_prevents_penetration() {
        let mut s = PhysicsSimulation::new(PhysicsConfig {
            gravity: [0.0; 3],
            ..Default::default()
        });
        s.add_collider(Collider {
            entity: EntityId(10),
            bounds: Aabb {
                min: [-1.0; 3],
                max: [1.0; 3],
            },
        });
        s.add_body(RigidBody::dynamic(EntityId(1), [0.0, 0.0, 0.0], 1.0));
        s.advance(1.0 / 60.0);
        assert!(s.body(EntityId(1)).unwrap().position[0].abs() >= 0.999);
    }

    #[test]
    fn sync_updates_ecs() {
        let mut w = World::new();
        let id = w.spawn(atc_genesis_platform::Transform::default());
        let mut s = PhysicsSimulation::new(PhysicsConfig::default());
        s.add_body(RigidBody::dynamic(id, [2.0, 3.0, 4.0], 1.0));
        assert_eq!(s.sync_to_world(&mut w), 1);
        assert_eq!(w.transform(id).unwrap().translation, [2.0, 3.0, 4.0]);
    }

    #[test]
    fn loaded_chunks_create_deterministic_colliders() {
        let mut streamer = WorldStreamer::default();
        streamer
            .add_chunk(atc_genesis_world::WorldChunk {
                id: WorldChunkId(7),
                asset: atc_genesis_platform::AssetId(7),
                bounds: atc_genesis_world::WorldBounds {
                    min: [0.0; 3],
                    max: [10.0; 3],
                },
                state: ChunkState::Loaded,
            })
            .unwrap();
        let mut physics = PhysicsSimulation::new(PhysicsConfig::default());
        let mut bridge = WorldPhysicsBridge::new();
        let result = bridge.sync(&streamer, &mut physics);
        assert_eq!(result, vec![(WorldChunkId(7), EntityId(u64::MAX - 7))]);
        assert_eq!(
            physics.query(Aabb {
                min: [1.0; 3],
                max: [2.0; 3]
            }),
            vec![EntityId(u64::MAX - 7)]
        );
        assert_eq!(
            bridge.entity_for_chunk(WorldChunkId(7)),
            Some(EntityId(u64::MAX - 7))
        );
    }
}
