use atc_genesis_platform::{AssetId, AssetKind, AssetStore, WorldAssetResolver};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AssetRecord {
    pub id: AssetId,
    pub kind: AssetKind,
}
#[derive(Default)]
pub struct InMemoryAssetStore {
    assets: Vec<AssetRecord>,
}
impl InMemoryAssetStore {
    pub fn register(&mut self, id: AssetId, kind: AssetKind) {
        if let Some(a) = self.assets.iter_mut().find(|a| a.id == id) {
            a.kind = kind
        } else {
            self.assets.push(AssetRecord { id, kind })
        }
    }
}
impl AssetStore for InMemoryAssetStore {
    fn contains(&self, id: AssetId) -> bool {
        self.assets.iter().any(|a| a.id == id)
    }
    fn kind(&self, id: AssetId) -> Option<AssetKind> {
        self.assets.iter().find(|a| a.id == id).map(|a| a.kind)
    }
}
impl WorldAssetResolver for InMemoryAssetStore {
    fn contains_asset(&self, id: AssetId) -> bool {
        self.contains(id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportRecord {
    pub source: String,
    pub kind: AssetKind,
    pub content_hash: u64,
    pub cooked: bool,
    pub dependencies: Vec<AssetId>,
}
#[derive(Default)]
pub struct AssetPipeline {
    records: HashMap<AssetId, ImportRecord>,
}
impl AssetPipeline {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn import(
        &mut self,
        id: AssetId,
        source: impl Into<String>,
        kind: AssetKind,
        bytes: &[u8],
        mut dependencies: Vec<AssetId>,
    ) -> u64 {
        dependencies.sort_by_key(|x| x.0);
        dependencies.dedup();
        let hash = fnv1a64(bytes);
        self.records.insert(
            id,
            ImportRecord {
                source: source.into(),
                kind,
                content_hash: hash,
                cooked: false,
                dependencies,
            },
        );
        hash
    }
    pub fn cook(&mut self, id: AssetId) -> bool {
        if let Some(r) = self.records.get_mut(&id) {
            r.cooked = true;
            true
        } else {
            false
        }
    }
    pub fn get(&self, id: AssetId) -> Option<&ImportRecord> {
        self.records.get(&id)
    }
    pub fn uncooked(&self) -> Vec<AssetId> {
        let mut v: Vec<_> = self
            .records
            .iter()
            .filter_map(|(id, r)| (!r.cooked).then_some(*id))
            .collect();
        v.sort_by_key(|x| x.0);
        v
    }
    pub fn validate_dependencies(&self, id: AssetId) -> bool {
        if !self.records.contains_key(&id) {
            return false;
        }
        let mut visiting = Vec::new();
        let mut visited = Vec::new();
        self.validate_dependencies_inner(id, &mut visiting, &mut visited)
    }
    fn validate_dependencies_inner(
        &self,
        id: AssetId,
        visiting: &mut Vec<AssetId>,
        visited: &mut Vec<AssetId>,
    ) -> bool {
        if visiting.contains(&id) {
            return false;
        }
        if visited.binary_search_by_key(&id.0, |x| x.0).is_ok() {
            return true;
        }
        let Some(record) = self.records.get(&id) else {
            return false;
        };
        visiting.push(id);
        let mut dependencies = record.dependencies.clone();
        dependencies.sort_by_key(|x| x.0);
        dependencies.dedup();
        for dependency in dependencies {
            if !self.validate_dependencies_inner(dependency, visiting, visited) {
                visiting.pop();
                return false;
            }
        }
        visiting.pop();
        let insert_at = visited
            .binary_search_by_key(&id.0, |x| x.0)
            .unwrap_or_else(|i| i);
        visited.insert(insert_at, id);
        true
    }
    pub fn dependency_order(&self, id: AssetId) -> Option<Vec<AssetId>> {
        if !self.records.contains_key(&id) || !self.validate_dependencies(id) {
            return None;
        }
        let mut order = Vec::new();
        let mut visiting = Vec::new();
        let mut visited = Vec::new();
        self.visit_dependency_order(id, &mut visiting, &mut visited, &mut order);
        Some(order)
    }
    fn visit_dependency_order(
        &self,
        id: AssetId,
        visiting: &mut Vec<AssetId>,
        visited: &mut Vec<AssetId>,
        order: &mut Vec<AssetId>,
    ) {
        if visited.contains(&id) || visiting.contains(&id) {
            return;
        }
        let Some(record) = self.records.get(&id) else {
            return;
        };
        visiting.push(id);
        let mut dependencies = record.dependencies.clone();
        dependencies.sort_by_key(|x| x.0);
        dependencies.dedup();
        for dependency in dependencies {
            self.visit_dependency_order(dependency, visiting, visited, order)
        }
        visiting.pop();
        if !visited.contains(&id) {
            visited.push(id);
            order.push(id)
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MeshAsset {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}
impl MeshAsset {
    pub fn validate(&self) -> Result<(), String> {
        if self.vertices.len() != self.normals.len() {
            return Err("vertex/normal count mismatch".into());
        }
        if self.vertices.len() != self.uvs.len() {
            return Err("vertex/uv count mismatch".into());
        }
        if !self.indices.len().is_multiple_of(3) {
            return Err("index count must be divisible by three".into());
        }
        if self
            .indices
            .iter()
            .any(|i| *i as usize >= self.vertices.len())
        {
            return Err("index out of range".into());
        }
        if self
            .vertices
            .iter()
            .chain(self.normals.iter())
            .any(|v| v.iter().any(|x| !x.is_finite()))
        {
            return Err("non-finite vertex data".into());
        }
        Ok(())
    }
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct MaterialAsset {
    pub base_color: [f32; 4],
    pub metallic: f32,
    pub roughness: f32,
    pub base_color_texture: Option<AssetId>,
}
impl Default for MaterialAsset {
    fn default() -> Self {
        Self {
            base_color: [1.0; 4],
            metallic: 0.0,
            roughness: 1.0,
            base_color_texture: None,
        }
    }
}
impl MaterialAsset {
    pub fn validate(&self) -> bool {
        self.base_color.iter().all(|v| v.is_finite())
            && self.metallic.is_finite()
            && self.roughness.is_finite()
            && self.metallic.clamp(0.0, 1.0) == self.metallic
            && self.roughness.clamp(0.0, 1.0) == self.roughness
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextureAsset {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub pixels: Vec<u8>,
}
impl TextureAsset {
    pub fn validate(&self) -> bool {
        matches!(self.channels, 1..=4)
            && self.width > 0
            && self.height > 0
            && self.pixels.len()
                == self.width as usize * self.height as usize * self.channels as usize
    }
}
pub fn stable_asset_id(source: &str, kind: AssetKind, content_hash: u64) -> AssetId {
    let mut h = fnv1a64(source.as_bytes()) ^ content_hash.rotate_left(17);
    h ^= (kind as u8 as u64).wrapping_mul(0x9e3779b97f4a7c15);
    AssetId(h as u128)
}
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn import_cook_hash() {
        let mut p = AssetPipeline::new();
        let id = AssetId(1);
        let h = p.import(id, "mesh.gltf", AssetKind::Mesh, b"mesh", vec![]);
        assert_ne!(h, 0);
        assert_eq!(p.uncooked(), vec![id]);
        assert!(p.cook(id));
        assert!(p.get(id).unwrap().cooked)
    }
    #[test]
    fn dependency_graph_is_transitive_and_deterministic() {
        let mut p = AssetPipeline::new();
        let root = AssetId(10);
        let a = AssetId(2);
        let b = AssetId(7);
        let leaf = AssetId(1);
        p.import(root, "root", AssetKind::Mesh, b"r", vec![b, a]);
        p.import(a, "a", AssetKind::Mesh, b"a", vec![leaf]);
        p.import(b, "b", AssetKind::Mesh, b"b", vec![leaf]);
        p.import(leaf, "leaf", AssetKind::Mesh, b"l", vec![]);
        assert!(p.validate_dependencies(root));
        assert_eq!(p.dependency_order(root), Some(vec![leaf, a, b, root]))
    }
    #[test]
    fn missing_dependency_is_rejected() {
        let mut p = AssetPipeline::new();
        p.import(AssetId(1), "mesh", AssetKind::Mesh, b"m", vec![AssetId(99)]);
        assert!(!p.validate_dependencies(AssetId(1)));
        assert_eq!(p.dependency_order(AssetId(1)), None)
    }
    #[test]
    fn dependency_cycle_is_rejected() {
        let mut p = AssetPipeline::new();
        p.import(AssetId(1), "a", AssetKind::Mesh, b"a", vec![AssetId(2)]);
        p.import(AssetId(2), "b", AssetKind::Mesh, b"b", vec![AssetId(1)]);
        assert!(!p.validate_dependencies(AssetId(1)));
        assert_eq!(p.dependency_order(AssetId(1)), None)
    }
    #[test]
    fn mesh_validates() {
        let m = MeshAsset {
            vertices: vec![[0.0; 3]; 3],
            normals: vec![[0.0, 1.0, 0.0]; 3],
            uvs: vec![[0.0; 2]; 3],
            indices: vec![0, 1, 2],
        };
        assert!(m.validate().is_ok());
        assert_eq!(m.triangle_count(), 1)
    }
    #[test]
    fn invalid_mesh_index_count_is_rejected() {
        let m = MeshAsset {
            vertices: vec![[0.0; 3]; 3],
            normals: vec![[0.0, 1.0, 0.0]; 3],
            uvs: vec![[0.0; 2]; 3],
            indices: vec![0, 1],
        };
        assert!(m.validate().is_err())
    }
    #[test]
    fn texture_validates() {
        let t = TextureAsset {
            width: 2,
            height: 2,
            channels: 4,
            pixels: vec![0; 16],
        };
        assert!(t.validate())
    }
    #[test]
    fn stable_id_is_deterministic() {
        assert_eq!(
            stable_asset_id("a", AssetKind::Mesh, 7),
            stable_asset_id("a", AssetKind::Mesh, 7)
        )
    }
    #[test]
    fn store_implements_world_resolver() {
        let mut s = InMemoryAssetStore::default();
        s.register(AssetId(7), AssetKind::Scene);
        assert!(s.contains_asset(AssetId(7)));
        assert!(!s.contains_asset(AssetId(8)))
    }
}
