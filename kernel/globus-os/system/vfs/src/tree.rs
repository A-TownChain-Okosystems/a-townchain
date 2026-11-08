//! In-memory inode tree used as the filesystem-neutral namespace backend.
//! Persistent filesystems can implement the same operations over a BlockDevice.

use std::collections::BTreeMap;

use crate::{FileError, FileMode, FileType, Inode, InodeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
    InvalidPath,
    NotFound,
    AlreadyExists,
    NotDirectory,
    NotEmpty,
    PermissionDenied,
    Overflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub name: String,
    pub inode: InodeId,
    pub file_type: FileType,
}

#[derive(Debug, Clone)]
struct Node {
    inode: Inode,
    data: Vec<u8>,
    children: BTreeMap<String, InodeId>,
}

#[derive(Debug, Clone)]
pub struct InodeTree {
    next_inode: u64,
    nodes: BTreeMap<InodeId, Node>,
}

impl Default for InodeTree {
    fn default() -> Self {
        Self::new()
    }
}

impl InodeTree {
    pub fn new() -> Self {
        let root = Inode {
            id: InodeId(1),
            file_type: FileType::Directory,
            mode: FileMode {
                read: true,
                write: true,
                execute: true,
            },
            size: 0,
        };
        let mut nodes = BTreeMap::new();
        nodes.insert(
            InodeId(1),
            Node {
                inode: root,
                data: Vec::new(),
                children: BTreeMap::new(),
            },
        );
        Self {
            next_inode: 2,
            nodes,
        }
    }

    pub fn root(&self) -> InodeId {
        InodeId(1)
    }

    fn components(path: &str) -> Result<Vec<String>, FsError> {
        let normalized = crate::normalize(path).map_err(|_| FsError::InvalidPath)?;
        Ok(normalized
            .split('/')
            .filter(|part| !part.is_empty())
            .map(str::to_owned)
            .collect())
    }

    fn lookup(&self, path: &str) -> Result<InodeId, FsError> {
        let mut current = self.root();
        for part in Self::components(path)? {
            let node = self.nodes.get(&current).ok_or(FsError::NotFound)?;
            current = *node.children.get(&part).ok_or(FsError::NotFound)?;
        }
        Ok(current)
    }

    fn parent_and_name(&self, path: &str) -> Result<(InodeId, String), FsError> {
        let normalized = crate::normalize(path).map_err(|_| FsError::InvalidPath)?;
        if normalized == "/" {
            return Err(FsError::InvalidPath);
        }
        let mut parts = normalized.rsplitn(2, '/');
        let name = parts.next().ok_or(FsError::InvalidPath)?.to_owned();
        // Bei Ein-Komponenten-Pfaden ("/home") liefert rsplitn als Rest einen
        // leeren String — das ist der Wurzel-Parent "/", nicht "".
        let parent = match parts.next() {
            None | Some("") => "/",
            Some(remainder) => remainder,
        };
        Ok((self.lookup(parent)?, name))
    }

    pub fn stat(&self, path: &str) -> Result<Inode, FsError> {
        Ok(self
            .nodes
            .get(&self.lookup(path)?)
            .ok_or(FsError::NotFound)?
            .inode
            .clone())
    }

    pub fn create(
        &mut self,
        path: &str,
        file_type: FileType,
        mode: FileMode,
    ) -> Result<InodeId, FsError> {
        let (parent, name) = self.parent_and_name(path)?;
        let parent_node = self.nodes.get(&parent).ok_or(FsError::NotFound)?;
        if parent_node.inode.file_type != FileType::Directory {
            return Err(FsError::NotDirectory);
        }
        if parent_node.children.contains_key(&name) {
            return Err(FsError::AlreadyExists);
        }
        let id = InodeId(self.next_inode);
        self.next_inode = self.next_inode.checked_add(1).ok_or(FsError::Overflow)?;
        let inode = Inode {
            id,
            file_type,
            mode,
            size: 0,
        };
        self.nodes.insert(
            id,
            Node {
                inode,
                data: Vec::new(),
                children: BTreeMap::new(),
            },
        );
        let parent_node = self.nodes.get_mut(&parent).ok_or(FsError::NotFound)?;
        parent_node.children.insert(name, id);
        Ok(id)
    }

    pub fn list(&self, path: &str) -> Result<Vec<DirectoryEntry>, FsError> {
        let id = self.lookup(path)?;
        let node = self.nodes.get(&id).ok_or(FsError::NotFound)?;
        if node.inode.file_type != FileType::Directory {
            return Err(FsError::NotDirectory);
        }
        Ok(node
            .children
            .iter()
            .filter_map(|(name, child)| {
                self.nodes.get(child).map(|n| DirectoryEntry {
                    name: name.clone(),
                    inode: *child,
                    file_type: n.inode.file_type,
                })
            })
            .collect())
    }

    pub fn read(&self, id: InodeId, offset: u64, out: &mut [u8]) -> Result<usize, FileError> {
        let node = self.nodes.get(&id).ok_or(FileError::NotFound)?;
        if node.inode.file_type != FileType::Regular {
            return Err(FileError::NotFile);
        }
        let start = usize::try_from(offset).map_err(|_| FileError::Overflow)?;
        if start >= node.data.len() {
            return Ok(0);
        }
        let count = out.len().min(node.data.len() - start);
        out[..count].copy_from_slice(&node.data[start..start + count]);
        Ok(count)
    }

    pub fn write(&mut self, id: InodeId, offset: u64, input: &[u8]) -> Result<usize, FileError> {
        let node = self.nodes.get_mut(&id).ok_or(FileError::NotFound)?;
        if node.inode.file_type != FileType::Regular {
            return Err(FileError::NotFile);
        }
        let start = usize::try_from(offset).map_err(|_| FileError::Overflow)?;
        let end = start.checked_add(input.len()).ok_or(FileError::Overflow)?;
        if end > node.data.len() {
            node.data.resize(end, 0);
        }
        node.data[start..end].copy_from_slice(input);
        node.inode.size = u64::try_from(node.data.len()).map_err(|_| FileError::Overflow)?;
        Ok(input.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creates_lists_and_round_trips() {
        let mut fs = InodeTree::new();
        fs.create("/home", FileType::Directory, FileMode::READ_WRITE)
            .unwrap();
        let id = fs
            .create("/home/test", FileType::Regular, FileMode::READ_WRITE)
            .unwrap();
        fs.write(id, 0, b"globus").unwrap();
        let mut out = [0; 6];
        fs.read(id, 0, &mut out).unwrap();
        assert_eq!(&out, b"globus");
        assert_eq!(fs.list("/home").unwrap()[0].name, "test");
    }
}
