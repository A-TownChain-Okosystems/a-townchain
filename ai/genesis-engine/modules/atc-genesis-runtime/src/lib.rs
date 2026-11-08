pub mod lifecycle;

use atc_genesis_animation::{AnimationClip, AnimationClipId, AnimationPlayer, BoneId};
use atc_genesis_ecs::{World, WorldEcsBridge};
use atc_genesis_gameplay::{GameplayRuntime, MovementConfig};
use atc_genesis_input::{InputEvent, InputState};
use atc_genesis_network::security::{PacketGuard, PacketRejectReason, SessionId};
use atc_genesis_network::{
    NetworkEntity, ReplicatedState, ReplicationHeader, ReplicationTransport, Replicator, Tick,
};
use atc_genesis_physics::{PhysicsConfig, PhysicsSimulation};
use atc_genesis_platform::{AudioRuntime, EntityId, FrameId, PhysicsWorld, Renderer, Transform};
use atc_genesis_world::{WorldChunk, WorldChunkId, WorldStreamer};
use std::collections::HashMap;

pub struct GenesisRuntime<
    R,
    A = atc_genesis_audio::NullAudioRuntime,
    N: ReplicationTransport = atc_genesis_network::LoopbackTransport,
> {
    pub world: WorldStreamer,
    pub ecs: World,
    pub physics: PhysicsSimulation,
    pub renderer: R,
    pub audio: A,
    pub input: InputState,
    pub gameplay: GameplayRuntime,
    pub network: Replicator<N>,
    network_tick: Tick,
    animations: HashMap<EntityId, AnimationBinding>,
    clips: HashMap<AnimationClipId, AnimationClip>,
    ecs_bridge: WorldEcsBridge,
    frame: u64,
}

struct AnimationBinding {
    player: AnimationPlayer,
    clip: AnimationClipId,
    last_root_translation: [f32; 3],
}

impl<R> GenesisRuntime<R> {
    pub fn new(renderer: R, physics_config: PhysicsConfig) -> Self {
        Self::with_audio_and_network(
            renderer,
            atc_genesis_audio::NullAudioRuntime::default(),
            Replicator::new(atc_genesis_network::LoopbackTransport::default()),
            physics_config,
        )
    }
}

impl<R, A, N: ReplicationTransport> GenesisRuntime<R, A, N> {
    pub fn with_audio_and_network(
        renderer: R,
        audio: A,
        network: Replicator<N>,
        physics_config: PhysicsConfig,
    ) -> Self {
        Self {
            world: WorldStreamer::default(),
            ecs: World::new(),
            physics: PhysicsSimulation::new(physics_config),
            renderer,
            audio,
            input: InputState::default(),
            gameplay: GameplayRuntime::default(),
            network,
            network_tick: Tick(0),
            animations: HashMap::new(),
            clips: HashMap::new(),
            ecs_bridge: WorldEcsBridge::new(),
            frame: 0,
        }
    }

    pub fn frame(&self) -> u64 {
        self.frame
    }
    pub fn network_tick(&self) -> Tick {
        self.network_tick
    }
    pub fn add_chunk(&mut self, chunk: WorldChunk) -> Result<(), &'static str> {
        self.world.add_chunk(chunk)
    }
    pub fn spawn(&mut self, transform: Transform) -> EntityId {
        self.ecs.spawn(transform)
    }
    pub fn chunk_entity(&self, id: WorldChunkId) -> Option<EntityId> {
        self.ecs_bridge.entity_for_chunk(id)
    }
    pub fn stream(
        &mut self,
        position: [f32; 3],
        load_distance: f32,
        unload_distance: f32,
        budget: usize,
    ) -> Vec<WorldChunkId> {
        self.world
            .update(position, load_distance, unload_distance, budget)
    }
    pub fn input_event(&mut self, event: InputEvent) {
        self.input.apply(event)
    }
    pub fn queue_movement(
        &mut self,
        entity: EntityId,
        config: MovementConfig,
    ) -> atc_genesis_gameplay::GameplayCommand {
        self.gameplay.sample_movement(&self.input, entity, config)
    }
    pub fn update_gameplay(&mut self, dt: f32) -> usize {
        self.gameplay.apply(&mut self.ecs, dt)
    }
    pub fn register_animation_clip(&mut self, clip: AnimationClip) -> Option<AnimationClip> {
        self.clips.insert(clip.id, clip)
    }

    pub fn play_animation(
        &mut self,
        entity: EntityId,
        clip: AnimationClipId,
        looping: bool,
        speed: f32,
    ) -> bool {
        if self.ecs.transform(entity).is_none() || !self.clips.contains_key(&clip) {
            return false;
        }
        self.animations.insert(
            entity,
            AnimationBinding {
                player: AnimationPlayer {
                    clip: Some(clip),
                    time_seconds: 0.0,
                    speed,
                    looping,
                },
                clip,
                last_root_translation: [0.0; 3],
            },
        );
        true
    }

    pub fn stop_animation(&mut self, entity: EntityId) -> bool {
        self.animations.remove(&entity).is_some()
    }

    pub fn update_animations(&mut self, dt: f32) -> usize {
        let ids: Vec<_> = self.animations.keys().copied().collect();
        let mut updated = 0;
        for entity in ids {
            let Some(binding) = self.animations.get_mut(&entity) else {
                continue;
            };
            let Some(clip) = self.clips.get(&binding.clip) else {
                continue;
            };
            binding.player.update(dt, clip.duration_seconds);
            let Some((_, pose)) = binding
                .player
                .sample(clip)
                .into_iter()
                .find(|(bone, _)| *bone == BoneId(0))
            else {
                continue;
            };
            let delta = [
                pose.translation[0] - binding.last_root_translation[0],
                pose.translation[1] - binding.last_root_translation[1],
                pose.translation[2] - binding.last_root_translation[2],
            ];
            binding.last_root_translation = pose.translation;
            let Some(mut current) = self.ecs.transform(entity).copied() else {
                continue;
            };
            current.translation[0] += delta[0];
            current.translation[1] += delta[1];
            current.translation[2] += delta[2];
            current.rotation_xyzw = pose.rotation_xyzw;
            current.scale = pose.scale;
            if self.ecs.set_transform(entity, current) {
                updated += 1;
            }
        }
        updated
    }

    pub fn publish_entity(&mut self, entity: EntityId) -> Result<u64, String>
    where
        N: ReplicationTransport,
    {
        let transform = self
            .ecs
            .transform(entity)
            .ok_or_else(|| format!("entity {} does not exist", entity.0))?;
        let position_mm = quantize_mm(transform.translation)?;
        let rotation_xyz_microunits = quantize_rotation(transform.rotation_xyzw)?;
        self.network.publish(&ReplicatedState {
            entity: NetworkEntity(entity.0),
            tick: self.network_tick,
            position_mm,
            rotation_xyz_microunits,
        })
    }

    pub fn apply_network_state(&mut self, state: ReplicatedState) -> Result<(), NetworkApplyError> {
        let entity = EntityId(state.entity.0);
        if self.ecs.transform(entity).is_none() {
            return Err(NetworkApplyError::UnknownEntity(entity));
        }
        if !state.has_valid_rotation() {
            return Err(NetworkApplyError::InvalidRotation);
        }
        let current = self
            .ecs
            .transform(entity)
            .copied()
            .ok_or(NetworkApplyError::UnknownEntity(entity))?;
        let mut transform = current;
        transform.translation = dequantize_mm(state.position_mm);
        transform.rotation_xyzw = dequantize_rotation(state.rotation_xyz_microunits);
        if !transform.rotation_xyzw.iter().all(|v| v.is_finite()) {
            return Err(NetworkApplyError::InvalidRotation);
        }
        self.ecs.set_transform(entity, transform);
        Ok(())
    }

    pub fn receive_network_packet(&mut self, bytes: &[u8]) -> Result<(), NetworkReceiveError>
    where
        N: ReplicationTransport,
    {
        let state = self
            .network
            .accept_packet(bytes)
            .ok_or(NetworkReceiveError::Rejected)?;
        self.apply_network_state(state)
            .map_err(NetworkReceiveError::Apply)
    }

    pub fn receive_secure_network_packet(
        &mut self,
        guard: &mut PacketGuard,
        session: SessionId,
        bytes: &[u8],
    ) -> Result<(), SecureNetworkReceiveError> {
        let validated = guard
            .validate(session, bytes)
            .map_err(SecureNetworkReceiveError::Rejected)?;
        self.apply_network_state(validated.state)
            .map_err(SecureNetworkReceiveError::Apply)
    }

    pub fn receive_secure_network_packet_for_entity(
        &mut self,
        guard: &mut PacketGuard,
        session: SessionId,
        expected: NetworkEntity,
        bytes: &[u8],
    ) -> Result<(), SecureNetworkReceiveError> {
        let validated = guard
            .validate_for_entity(session, expected, bytes)
            .map_err(|reason| match reason {
                PacketRejectReason::EntityMismatch => SecureNetworkReceiveError::EntityMismatch {
                    expected,
                    received: ReplicatedState::decode(&bytes[ReplicationHeader::BYTES..])
                        .map(|s| s.entity)
                        .unwrap_or(expected),
                },
                other => SecureNetworkReceiveError::Rejected(other),
            })?;
        self.apply_network_state(validated.state)
            .map_err(SecureNetworkReceiveError::Apply)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NetworkApplyError {
    UnknownEntity(EntityId),
    InvalidRotation,
}

#[derive(Debug, PartialEq, Eq)]
pub enum NetworkReceiveError {
    Rejected,
    Apply(NetworkApplyError),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SecureNetworkReceiveError {
    Rejected(PacketRejectReason),
    EntityMismatch {
        expected: NetworkEntity,
        received: NetworkEntity,
    },
    Apply(NetworkApplyError),
}

fn quantize_mm(position: [f32; 3]) -> Result<[i32; 3], String> {
    fn to_mm(val: f32) -> Result<i32, String> {
        let mm = (val * 1000.0).round();
        if mm < i32::MIN as f32 || mm > i32::MAX as f32 {
            return Err("position component overflowed i32 mm range".to_string());
        }
        Ok(mm as i32)
    }
    Ok([
        to_mm(position[0])?,
        to_mm(position[1])?,
        to_mm(position[2])?,
    ])
}

fn dequantize_mm(mm: [i32; 3]) -> [f32; 3] {
    [
        mm[0] as f32 / 1000.0,
        mm[1] as f32 / 1000.0,
        mm[2] as f32 / 1000.0,
    ]
}

fn quantize_rotation(rot: [f32; 4]) -> Result<[i32; 3], String> {
    fn to_micro(val: f32) -> Result<i32, String> {
        let micro = (val * 1_000_000.0).round();
        if micro < i32::MIN as f32 || micro > i32::MAX as f32 {
            return Err("rotation component overflowed i32 micro range".to_string());
        }
        Ok(micro as i32)
    }
    Ok([to_micro(rot[0])?, to_micro(rot[1])?, to_micro(rot[2])?])
}

fn dequantize_rotation(rot: [i32; 3]) -> [f32; 4] {
    let x = rot[0] as f32 / 1_000_000.0;
    let y = rot[1] as f32 / 1_000_000.0;
    let z = rot[2] as f32 / 1_000_000.0;
    let sum_sq = x * x + y * y + z * z;
    let w = if sum_sq <= 1.0 {
        (1.0 - sum_sq).sqrt()
    } else {
        0.0
    };
    [x, y, z, w]
}

impl<R: Renderer, A: AudioRuntime, N: ReplicationTransport> GenesisRuntime<R, A, N> {
    pub fn step(&mut self, dt: f32) -> StepSummary {
        let chunks_updated = 0;
        self.physics.step(dt);
        let animations_updated = self.update_animations(dt);
        let commands_processed = self.update_gameplay(dt);
        let frame_id = FrameId(self.frame);
        self.renderer.begin_frame(frame_id);
        self.renderer.end_frame();
        self.audio.update(dt);
        self.network_tick = Tick(self.network_tick.0 + 1);
        self.frame += 1;
        StepSummary {
            frame: self.frame,
            network_tick: self.network_tick,
            chunks_updated,
            animations_updated,
            commands_processed,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct StepSummary {
    pub frame: u64,
    pub network_tick: Tick,
    pub chunks_updated: usize,
    pub animations_updated: usize,
    pub commands_processed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use atc_genesis_platform::{AudioRuntime, FrameId, Renderer, Transform};

    struct NullRenderer;
    impl Renderer for NullRenderer {
        fn begin_frame(&mut self, _frame: FrameId) {}
        fn submit(&mut self, _entity: EntityId, _transform: Transform) {}
        fn end_frame(&mut self) {}
    }

    #[derive(Default)]
    struct MockAudio {
        ticks: u64,
    }

    impl AudioRuntime for MockAudio {
        fn update(&mut self, _dt_seconds: f32) {
            self.ticks += 1;
        }
        fn set_master_gain(&mut self, _gain: f32) {}
    }

    fn runtime() -> GenesisRuntime<NullRenderer> {
        GenesisRuntime::new(
            NullRenderer,
            PhysicsConfig {
                gravity: [0.0, -9.81, 0.0],
                max_substeps: 1,
                ..Default::default()
            },
        )
    }

    #[test]
    fn test_initialization() {
        let r = runtime();
        assert_eq!(r.frame(), 0);
        assert_eq!(r.network_tick(), Tick(0));
    }

    #[test]
    fn test_step_advances_frame_and_tick() {
        let mut r = runtime();
        let summary = r.step(0.016);
        assert_eq!(summary.frame, 1);
        assert_eq!(summary.network_tick, Tick(1));
        assert_eq!(r.frame(), 1);
        assert_eq!(r.network_tick(), Tick(1));
    }

    #[test]
    fn test_add_and_stream_chunks() {
        let mut r = runtime();
        let chunk = WorldChunk {
            id: WorldChunkId(1),
            asset: atc_genesis_platform::AssetId(1),
            bounds: atc_genesis_world::WorldBounds {
                min: [0.0, 0.0, 0.0],
                max: [10.0, 10.0, 10.0],
            },
            state: atc_genesis_world::ChunkState::Unloaded,
        };
        assert!(r.add_chunk(chunk).is_ok());
        let updated = r.stream([0.0, 0.0, 0.0], 5.0, 15.0, 10);
        assert!(!updated.is_empty());
    }

    #[test]
    fn test_publish_and_receive_network_state() {
        let mut r = runtime();
        let entity = r.spawn(Transform {
            translation: [1.0, 2.0, 3.0],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        });

        let result = r.publish_entity(entity);
        assert!(result.is_ok());

        let state = ReplicatedState {
            entity: NetworkEntity(entity.0),
            tick: Tick(1),
            position_mm: [2000, 3000, 4000],
            rotation_xyz_microunits: [0, 0, 0],
        };

        assert!(r.apply_network_state(state).is_ok());
        let t = r.ecs.transform(entity).unwrap();
        assert_eq!(t.translation, [2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_receive_network_packet_fails_on_garbage() {
        let mut r = runtime();
        let err = r.receive_network_packet(&[9, 9, 9]);
        assert_eq!(err, Err(NetworkReceiveError::Rejected));
    }

    #[test]
    fn test_custom_audio_and_network() {
        let mut r = GenesisRuntime::with_audio_and_network(
            NullRenderer,
            MockAudio::default(),
            Replicator::new(atc_genesis_network::LoopbackTransport::default()),
            PhysicsConfig {
                gravity: [0.0, -9.81, 0.0],
                max_substeps: 1,
                ..Default::default()
            },
        );

        let entity = r.spawn(Transform::default());
        let bytes = r.network.publish(&ReplicatedState {
            entity: NetworkEntity(entity.0),
            tick: Tick(0),
            position_mm: [1000, 2000, 3000],
            rotation_xyz_microunits: [0, 0, 0],
        });
        assert!(bytes.is_ok());

        let step = r.step(0.016);
        assert_eq!(step.frame, 1);
        assert_eq!(r.audio.ticks, 1);
    }
}
