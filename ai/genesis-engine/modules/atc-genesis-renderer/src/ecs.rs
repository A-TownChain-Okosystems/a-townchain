use atc_genesis_ecs::World;
use atc_genesis_platform::EntityId;

use crate::resources::{MeshMaterialBinding, RenderResourceBindings};
use crate::{DrawCommand, RenderGraph};

impl RenderGraph {
    pub fn rebuild_from_world_with_resources(
        &mut self,
        world: &World,
        resources: &RenderResourceBindings,
    ) {
        self.clear();
        for (entity, transform) in world.query_world_transforms() {
            let binding = resources.binding(entity);
            self.push(DrawCommand {
                entity,
                transform,
                mesh: binding.map(|b| b.mesh.0),
                material: binding.map(|b| b.material.0),
            });
        }
        self.sort_deterministic();
    }

    pub fn rebuild_entity(
        &mut self,
        world: &World,
        resources: &RenderResourceBindings,
        entity: EntityId,
    ) -> bool {
        let Some(transform) = world.world_transform(entity) else {
            return false;
        };
        let binding = resources.binding(entity);
        self.push(DrawCommand {
            entity,
            transform,
            mesh: binding.map(|b| b.mesh.0),
            material: binding.map(|b| b.material.0),
        });
        true
    }

    pub fn rebuild_binding(&mut self, entity: EntityId, binding: MeshMaterialBinding) {
        self.push(DrawCommand {
            entity,
            transform: Default::default(),
            mesh: Some(binding.mesh.0),
            material: Some(binding.material.0),
        });
    }
}
