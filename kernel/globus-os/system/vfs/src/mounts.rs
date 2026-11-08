//! Deterministic mount-table lookup.

use crate::{Mount, path};

#[derive(Debug, Default, Clone)]
pub struct MountTable {
    mounts: Vec<Mount>,
}

impl MountTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, mount: Mount) -> bool {
        let Ok(path) = path::normalize(&mount.mountpoint) else {
            return false;
        };
        if self.mounts.iter().any(|m| m.mountpoint == path) {
            return false;
        }
        let mut mount = mount;
        mount.mountpoint = path;
        self.mounts.push(mount);
        self.mounts.sort_by(|a, b| {
            b.mountpoint
                .len()
                .cmp(&a.mountpoint.len())
                .then_with(|| a.mountpoint.cmp(&b.mountpoint))
        });
        true
    }
    pub fn resolve(&self, path: &str) -> Option<&Mount> {
        let normalized = path::normalize(path).ok()?;
        self.mounts.iter().find(|m| {
            // "/" ist der Root-Mount und matcht jeden absoluten Pfad.
            m.mountpoint == "/"
                || normalized == m.mountpoint
                || normalized.starts_with(&(m.mountpoint.clone() + "/"))
        })
    }
    pub fn mounts(&self) -> &[Mount] {
        &self.mounts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn longest_prefix_wins() {
        let mut t = MountTable::new();
        assert!(t.add(Mount {
            mountpoint: "/".into(),
            filesystem: "root".into(),
            readonly: true
        }));
        assert!(t.add(Mount {
            mountpoint: "/home".into(),
            filesystem: "data".into(),
            readonly: false
        }));
        assert_eq!(t.resolve("/home/user").unwrap().filesystem, "data");
        assert_eq!(t.resolve("/etc").unwrap().filesystem, "root");
    }
}
