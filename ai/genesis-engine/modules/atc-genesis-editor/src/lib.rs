use atc_genesis_platform::{EntityId, Transform};
#[derive(Clone, Debug, PartialEq)]
pub struct SceneNode {
    pub id: EntityId,
    pub parent: Option<EntityId>,
    pub name: String,
    pub transform: Transform,
}
#[derive(Default)]
pub struct SceneDocument {
    nodes: Vec<SceneNode>,
    selection: Option<EntityId>,
    undo: Vec<Vec<SceneNode>>,
}
impl SceneDocument {
    pub fn create_node(&mut self, id: EntityId, name: impl Into<String>, parent: Option<EntityId>) {
        if self.nodes.iter().any(|n| n.id == id) {
            return;
        }
        self.undo.push(self.nodes.clone());
        self.nodes.push(SceneNode {
            id,
            parent,
            name: name.into(),
            transform: Transform::default(),
        });
    }
    pub fn remove_node(&mut self, id: EntityId) -> bool {
        if let Some(pos) = self.nodes.iter().position(|n| n.id == id) {
            self.undo.push(self.nodes.clone());
            self.nodes.remove(pos);
            for n in &mut self.nodes {
                if n.parent == Some(id) {
                    n.parent = None
                }
            }
            if self.selection == Some(id) {
                self.selection = None
            }
            true
        } else {
            false
        }
    }
    pub fn set_transform(&mut self, id: EntityId, transform: Transform) -> bool {
        if let Some(pos) = self.nodes.iter().position(|n| n.id == id) {
            self.undo.push(self.nodes.clone());
            self.nodes[pos].transform = transform;
            true
        } else {
            false
        }
    }
    pub fn select(&mut self, id: Option<EntityId>) {
        self.selection = id
    }
    pub fn selected(&self) -> Option<EntityId> {
        self.selection
    }
    pub fn nodes(&self) -> &[SceneNode] {
        &self.nodes
    }
    pub fn undo(&mut self) -> bool {
        if let Some(state) = self.undo.pop() {
            self.nodes = state;
            true
        } else {
            false
        }
    }
    pub fn apply_to_world(&self, world: &mut atc_genesis_ecs::World) -> Result<(), String> {
        let ids: std::collections::HashSet<_> = self.nodes.iter().map(|n| n.id).collect();
        if self
            .nodes
            .iter()
            .any(|n| n.parent.is_some_and(|p| !ids.contains(&p)))
        {
            return Err("scene contains missing parent".into());
        }
        for n in &self.nodes {
            if world.transform(n.id).is_none() && !world.insert(n.id, n.transform) {
                return Err(format!("failed to create entity {}", n.id.0));
            } else if world.transform(n.id).is_some() && !world.set_transform(n.id, n.transform) {
                return Err(format!("failed to update entity {}", n.id.0));
            }
        }
        for n in &self.nodes {
            if !world.set_parent(n.id, n.parent) {
                return Err(format!("invalid parent for entity {}", n.id.0));
            }
        }
        Ok(())
    }
    pub fn serialize(&self) -> String {
        let mut out = String::from(
            "GENESIS_SCENE 1
",
        );
        let mut nodes = self.nodes.clone();
        nodes.sort_by_key(|n| n.id.0);
        for n in nodes {
            let p = n.parent.map(|x| x.0).unwrap_or(0);
            let t = n.transform;
            out.push_str(&format!(
                "NODE|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}
",
                n.id.0,
                p,
                escape(&n.name),
                t.translation[0],
                t.translation[1],
                t.translation[2],
                t.rotation_xyzw[0],
                t.rotation_xyzw[1],
                t.rotation_xyzw[2],
                t.rotation_xyzw[3],
                t.scale[0],
                t.scale[1],
                t.scale[2]
            ))
        }
        out
    }
    pub fn deserialize(text: &str) -> Result<Self, String> {
        let mut doc = Self::default();
        let mut seen = std::collections::HashSet::new();
        for line in text.lines() {
            if line == "GENESIS_SCENE 1" || line.is_empty() {
                continue;
            }
            let p: Vec<&str> = line.split('|').collect();
            if p.len() != 14 || p[0] != "NODE" {
                return Err("invalid scene row".into());
            }
            let id = EntityId(p[1].parse::<u64>().map_err(|_| "invalid entity id")?);
            if !seen.insert(id) {
                return Err("duplicate entity id".into());
            }
            let parent = match p[2].parse::<u64>().map_err(|_| "invalid parent")? {
                0 => None,
                v => Some(EntityId(v)),
            };
            let tr = [parse(p[4])?, parse(p[5])?, parse(p[6])?];
            let rot = [parse(p[7])?, parse(p[8])?, parse(p[9])?, parse(p[10])?];
            let scale = [parse(p[11])?, parse(p[12])?, parse(p[13])?];
            doc.nodes.push(SceneNode {
                id,
                parent,
                name: unescape(p[3]),
                transform: Transform {
                    translation: tr,
                    rotation_xyzw: rot,
                    scale,
                },
            })
        }
        if doc
            .nodes
            .iter()
            .any(|n| n.parent.is_some_and(|p| !seen.contains(&p)))
        {
            return Err("missing parent".into());
        }
        Ok(doc)
    }
}
fn parse(s: &str) -> Result<f32, String> {
    s.parse().map_err(|_| format!("invalid float: {s}"))
}
fn escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('|', "\\p")
        .replace('\n', "\\n")
}
fn unescape(s: &str) -> String {
    // BUGFIX: Das fruehere erste replace hatte durch Backslash-Zeilenfortsetzung ein
    // LEERES Suchmuster und fuegte "\n" zwischen jedes Zeichen ein; Roundtrip kaputt.
    s.replace("\\n", "\n")
        .replace("\\p", "|")
        .replace("\\\\", "\\")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scene_roundtrip() {
        let mut d = SceneDocument::default();
        d.create_node(EntityId(2), "Root|A", None);
        let r = SceneDocument::deserialize(&d.serialize()).unwrap();
        assert_eq!(r.nodes()[0].name, "Root|A");
        assert_eq!(r.nodes()[0].id, EntityId(2));
    }
    #[test]
    fn scene_applies_to_ecs() {
        let mut d = SceneDocument::default();
        d.create_node(EntityId(7), "Root", None);
        let mut w = atc_genesis_ecs::World::new();
        assert!(d.apply_to_world(&mut w).is_ok());
        assert!(w.transform(EntityId(7)).is_some());
    }
}
