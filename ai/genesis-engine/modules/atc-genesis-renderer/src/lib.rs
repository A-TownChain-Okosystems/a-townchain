use atc_genesis_platform::{AssetId, EntityId, FrameId, Renderer, Transform};
pub mod animation;
pub mod backend;
pub mod batching;
pub mod ecs;
pub mod render_item;
pub mod resources;
pub use animation::SkinnedPose;
pub use backend::{
    BackendRenderer, CommandBufferBackend, GraphicsBackend, NullBackend, RenderBackend,
    RenderCapabilities,
};
pub use batching::{build_batches, RenderBatch, RenderItem};
pub use render_item::{collect_entity, collect_render_items};
pub use resources::{
    MaterialHandle, MeshHandle, MeshMaterialBinding, RenderResourceBindings, TextureHandle,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub position: [f32; 3],
    pub forward: [f32; 3],
    pub fov_y_radians: f32,
    pub near: f32,
    pub far: f32,
}
impl Default for Camera {
    fn default() -> Self {
        Self {
            position: [0.0; 3],
            forward: [0.0, 0.0, -1.0],
            fov_y_radians: std::f32::consts::FRAC_PI_3,
            near: 0.1,
            far: 1000.0,
        }
    }
}
impl Camera {
    pub fn visible_distance(&self, point: [f32; 3]) -> bool {
        let d = [
            point[0] - self.position[0],
            point[1] - self.position[1],
            point[2] - self.position[2],
        ];
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        len >= self.near && len <= self.far
    }
    pub fn in_front(&self, point: [f32; 3]) -> bool {
        let f = self.forward;
        let n = (f[0] * f[0] + f[1] * f[1] + f[2] * f[2]).sqrt();
        if n <= f32::EPSILON {
            return false;
        }
        let d = [
            point[0] - self.position[0],
            point[1] - self.position[1],
            point[2] - self.position[2],
        ];
        d[0] * f[0] + d[1] * f[1] + d[2] * f[2] >= 0.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawCommand {
    pub entity: EntityId,
    pub transform: Transform,
    pub mesh: Option<AssetId>,
    pub material: Option<AssetId>,
}
#[derive(Default)]
pub struct RenderGraph {
    commands: Vec<DrawCommand>,
}
impl RenderGraph {
    pub fn clear(&mut self) {
        self.commands.clear()
    }
    pub fn push(&mut self, c: DrawCommand) {
        self.commands.push(c)
    }
    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }
    pub fn sort_deterministic(&mut self) {
        self.commands.sort_by(|a, b| {
            a.entity
                .0
                .cmp(&b.entity.0)
                .then_with(|| a.mesh.cmp(&b.mesh))
                .then_with(|| a.material.cmp(&b.material))
        })
    }
    pub fn cull_distance(&mut self, camera: &Camera) {
        self.commands.retain(|c| {
            camera.visible_distance(c.transform.translation)
                && camera.in_front(c.transform.translation)
        })
    }
    pub fn rebuild_from_world(&mut self, world: &atc_genesis_ecs::World) {
        self.clear();
        for (id, transform) in world.query_world_transforms() {
            self.push(DrawCommand {
                entity: id,
                transform,
                mesh: None,
                material: None,
            });
        }
        self.sort_deterministic()
    }
}
#[derive(Default)]
pub struct NullRenderer {
    frame: Option<FrameId>,
    submitted: usize,
    pub graph: RenderGraph,
}
impl NullRenderer {
    pub fn submitted_count(&self) -> usize {
        self.submitted
    }
}
impl Renderer for NullRenderer {
    fn begin_frame(&mut self, frame: FrameId) {
        self.frame = Some(frame);
        self.submitted = 0;
        self.graph.clear()
    }
    fn submit(&mut self, entity: EntityId, transform: Transform) {
        if self.frame.is_some() {
            self.submitted += 1;
            self.graph.push(DrawCommand {
                entity,
                transform,
                mesh: None,
                material: None,
            })
        }
    }
    fn end_frame(&mut self) {
        self.graph.sort_deterministic();
        self.frame = None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn graph_orders_entities() {
        let mut r = NullRenderer::default();
        r.begin_frame(FrameId(1));
        r.submit(EntityId(3), Transform::default());
        r.submit(EntityId(1), Transform::default());
        r.end_frame();
        assert_eq!(r.graph.commands()[0].entity, EntityId(1));
    }
    #[test]
    fn camera_culls_behind() {
        let mut g = RenderGraph::default();
        g.push(DrawCommand {
            entity: EntityId(1),
            transform: Transform {
                translation: [0.0, 0.0, 1.0],
                ..Default::default()
            },
            mesh: None,
            material: None,
        });
        g.cull_distance(&Camera::default());
        assert!(g.commands().is_empty());
    }
    #[test]
    fn world_builds_render_graph() {
        let mut w = atc_genesis_ecs::World::new();
        w.spawn(Transform {
            translation: [0.0, 0.0, -2.0],
            ..Default::default()
        });
        let mut g = RenderGraph::default();
        g.rebuild_from_world(&w);
        assert_eq!(g.commands().len(), 1);
    }
}
