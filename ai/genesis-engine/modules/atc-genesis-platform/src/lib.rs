//! Stable contracts shared by Genesis Engine subsystems.
//!
//! The contracts deliberately contain no renderer, physics, audio or editor
//! implementation. Concrete backends can evolve independently behind these
//! interfaces.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct EntityId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct AssetId(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameId(pub u64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub translation: [f32; 3],
    pub rotation_xyzw: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: [0.0; 3],
            rotation_xyzw: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0; 3],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AssetKind {
    Mesh,
    Material,
    Texture,
    Shader,
    Audio,
    Animation,
    Scene,
    Prefab,
    Unknown,
}

pub trait AssetStore {
    fn contains(&self, id: AssetId) -> bool;
    fn kind(&self, id: AssetId) -> Option<AssetKind>;
}

pub trait PhysicsWorld {
    fn step(&mut self, dt_seconds: f32);
    fn raycast(&self, origin: [f32; 3], direction: [f32; 3], max_distance: f32)
        -> Option<EntityId>;
}

pub trait AudioRuntime {
    fn update(&mut self, dt_seconds: f32);
    fn set_master_gain(&mut self, gain: f32);
}

pub trait Renderer {
    fn begin_frame(&mut self, frame: FrameId);
    fn submit(&mut self, entity: EntityId, transform: Transform);
    fn end_frame(&mut self);
}

pub trait GameSystem {
    fn name(&self) -> &'static str;
    fn update(&mut self, dt_seconds: f32);
}

/// Asset-backed world streaming capability.
///
/// Implementations must make availability checks deterministic. The world
/// streamer uses this contract to prevent a chunk from entering `Loading`
/// when its backing asset is unavailable.
pub trait WorldAssetResolver {
    fn contains_asset(&self, id: AssetId) -> bool;
}

#[derive(Default)]
pub struct PlatformServices<R, P, A> {
    pub renderer: Option<R>,
    pub physics: Option<P>,
    pub audio: Option<A>,
}

impl<R, P, A> PlatformServices<R, P, A> {
    pub fn new() -> Self {
        Self {
            renderer: None,
            physics: None,
            audio: None,
        }
    }
}
