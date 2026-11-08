use atc_genesis_platform::AssetId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MeshHandle(pub AssetId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MaterialHandle(pub AssetId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TextureHandle(pub AssetId);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MeshMaterialBinding {
    pub mesh: MeshHandle,
    pub material: MaterialHandle,
}

#[derive(Default)]
pub struct RenderResourceBindings {
    bindings: std::collections::HashMap<atc_genesis_platform::EntityId, MeshMaterialBinding>,
}

impl RenderResourceBindings {
    pub fn bind(&mut self, entity: atc_genesis_platform::EntityId, binding: MeshMaterialBinding) {
        self.bindings.insert(entity, binding);
    }

    pub fn unbind(
        &mut self,
        entity: atc_genesis_platform::EntityId,
    ) -> Option<MeshMaterialBinding> {
        self.bindings.remove(&entity)
    }

    pub fn binding(&self, entity: atc_genesis_platform::EntityId) -> Option<MeshMaterialBinding> {
        self.bindings.get(&entity).copied()
    }

    pub fn iter_sorted(&self) -> Vec<(atc_genesis_platform::EntityId, MeshMaterialBinding)> {
        let mut items: Vec<_> = self
            .bindings
            .iter()
            .map(|(id, binding)| (*id, *binding))
            .collect();
        items.sort_by_key(|(id, _)| id.0);
        items
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use atc_genesis_platform::EntityId;

    #[test]
    fn bindings_are_replaced_and_sorted() {
        let mut r = RenderResourceBindings::default();
        let a = EntityId(2);
        let b = EntityId(1);
        let mesh = MeshHandle(AssetId(10));
        let material = MaterialHandle(AssetId(20));
        r.bind(a, MeshMaterialBinding { mesh, material });
        r.bind(b, MeshMaterialBinding { mesh, material });
        assert_eq!(r.binding(a).unwrap().mesh, mesh);
        assert_eq!(r.iter_sorted()[0].0, b);
    }
}
