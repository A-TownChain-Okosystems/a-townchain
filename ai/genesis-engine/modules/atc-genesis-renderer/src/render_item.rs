use atc_genesis_ecs::World;
use atc_genesis_platform::EntityId;

use crate::batching::RenderItem;
use crate::resources::RenderResourceBindings;

pub fn collect_render_items(world: &World, resources: &RenderResourceBindings) -> Vec<RenderItem> {
    let mut items = Vec::new();
    for (entity, transform) in world.query_world_transforms() {
        let Some(binding) = resources.binding(entity) else {
            continue;
        };
        items.push(RenderItem {
            entity,
            transform,
            mesh: binding.mesh.0,
            material: binding.material.0,
            texture: None,
            instance_group: None,
        });
    }
    items.sort_by_key(|item| item.entity);
    items
}

pub fn collect_entity(
    world: &World,
    resources: &RenderResourceBindings,
    entity: EntityId,
) -> Option<RenderItem> {
    let transform = world.world_transform(entity)?;
    let binding = resources.binding(entity)?;
    Some(RenderItem {
        entity,
        transform,
        mesh: binding.mesh.0,
        material: binding.material.0,
        texture: None,
        instance_group: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::{MaterialHandle, MeshHandle, MeshMaterialBinding};
    use atc_genesis_platform::{AssetId, Transform};

    #[test]
    fn collect_requires_mesh_material_binding() {
        let mut world = World::new();
        let entity = world.spawn(Transform::default());
        let mut resources = RenderResourceBindings::default();
        assert!(collect_entity(&world, &resources, entity).is_none());
        resources.bind(
            entity,
            MeshMaterialBinding {
                mesh: MeshHandle(AssetId(1)),
                material: MaterialHandle(AssetId(2)),
            },
        );
        assert!(collect_entity(&world, &resources, entity).is_some());
    }
}
