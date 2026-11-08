// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
use alloc::string::ToString;
// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ─────────────────────────────────────────────────────────────────────────
// K-Sprint 8 — Virtual File System (VFS)
// Kernel Layer | Chain-ID 658467
// Capability-gegates VFS mit In-Memory-Backend, Pfad-Aufloesung, File-Handles.
// ─────────────────────────────────────────────────────────────────────────

use alloc::format;
use alloc::collections::BTreeMap;
use alloc::string::{String, String as Str};
use alloc::vec::Vec;
use alloc::sync::Arc;
use spin::Mutex;

// ─── Capability-Integration ───────────────────────────────────────────────
use crate::capability::{CapabilityTable, Rights};

// ─── Datei-Typen ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

#[derive(Clone, Debug, PartialEq)]
    pub struct FileMetadata {
    pub file_type: FileType,
    pub size: u64,
    pub created_at: u64,
    pub modified_at: u64,
    pub owner_pid: u64,
    pub permissions: Rights,
}

// ─── Inode ────────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Inode {
    id: u64,
    name: String,
    file_type: FileType,
    parent: Option<u64>,
    children: BTreeMap<String, u64>, // nur fuer Directories
    data: Vec<u8>,                   // nur fuer Files
    symlink_target: Option<String>, // nur fuer Symlinks
    metadata: FileMetadata,
}

impl Inode {
    fn new_file(id: u64, name: String, owner_pid: u64) -> Self {
        Inode {
            id,
            name,
            file_type: FileType::File,
            parent: None,
            children: BTreeMap::new(),
            data: Vec::new(),
            symlink_target: None,
            metadata: FileMetadata {
                file_type: FileType::File,
                size: 0,
                created_at: 0,
                modified_at: 0,
                owner_pid,
                permissions: Rights::READ | Rights::WRITE,
            },
        }
    }

    fn new_dir(id: u64, name: String, owner_pid: u64) -> Self {
        Inode {
            id,
            name,
            file_type: FileType::Directory,
            parent: None,
            children: BTreeMap::new(),
            data: Vec::new(),
            symlink_target: None,
            metadata: FileMetadata {
                file_type: FileType::Directory,
                size: 0,
                created_at: 0,
                modified_at: 0,
                owner_pid,
                permissions: Rights::READ | Rights::WRITE | Rights::EXEC,
            },
        }
    }

    fn new_symlink(id: u64, name: String, target: String, owner_pid: u64) -> Self {
        Inode {
            id,
            name,
            file_type: FileType::Symlink,
            parent: None,
            children: BTreeMap::new(),
            data: Vec::new(),
            symlink_target: Some(target),
            metadata: FileMetadata {
                file_type: FileType::Symlink,
                size: 0,
                created_at: 0,
                modified_at: 0,
                owner_pid,
                permissions: Rights::READ,
            },
        }
    }

    fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }

    fn is_file(&self) -> bool {
        self.file_type == FileType::File
    }
}

// ─── File-Handle ──────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct FileHandle {
    inode_id: u64,
    pub position: u64,
    pub mode: OpenMode,
    pub pid: u64,
    pub cap_handle: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenMode {
    Read,
    Write,
    ReadWrite,
    Append,
    Create,
}

impl OpenMode {
    fn allows_read(&self) -> bool {
        matches!(self, OpenMode::Read | OpenMode::ReadWrite)
    }

    fn allows_write(&self) -> bool {
        matches!(self, OpenMode::Write | OpenMode::ReadWrite | OpenMode::Append | OpenMode::Create)
    }
}

// ─── Pfad-Hilfsfunktionen ──────────────────────────────────────────────────

fn normalize_path(path: &str) -> Vec<String> {
    let mut components: Vec<String> = Vec::new();
    for part in path.split('/') {
        match part {
            "" | "." => {} // skip
            ".." => { components.pop(); }
            _ => { components.push(part.to_string()); }
        }
    }
    components
}

fn parent_path(path: &str) -> (String, String) {
    let comps = normalize_path(path);
    if comps.is_empty() {
        return (String::new(), String::new());
    }
    let name = comps.last().unwrap().clone();
    let parent = if comps.len() > 1 {
        format!("/{}/", comps[..comps.len()-1].join("/"))
    } else {
        "/".to_string()
    };
    (parent, name)
}

// ─── VFS ───────────────────────────────────────────────────────────────────

pub struct Vfs {
    inodes: BTreeMap<u64, Inode>,
    next_inode: u64,
    root_inode: u64,
    handles: BTreeMap<u64, FileHandle>, // fd -> handle
    next_fd: u64,
    caps: Arc<Mutex<CapabilityTable>>,
}

impl Vfs {
    pub fn new(caps: Arc<Mutex<CapabilityTable>>) -> Self {
        let mut vfs = Vfs {
            inodes: BTreeMap::new(),
            next_inode: 1,
            root_inode: 0,
            handles: BTreeMap::new(),
            next_fd: 1,
            caps,
        };

        // Root-Verzeichnis erstellen (Inode 1)
        let root = Inode::new_dir(1, "/".to_string(), 0);
        vfs.root_inode = 1;
        vfs.next_inode = 2;
        vfs.inodes.insert(1, root);
        vfs
    }

    // ── Pfad-Auflösung ─────────────────────────────────────────────────────

    fn resolve(&self, path: &str) -> Option<u64> {
        if path == "/" || path.is_empty() {
            return Some(self.root_inode);
        }
        let components = normalize_path(path);
        let mut current = self.root_inode;

        for comp in &components {
            let inode = self.inodes.get(&current)?;
            if !inode.is_dir() {
                return None; // Pfad geht durch eine Datei
            }
            match inode.children.get(comp) {
                Some(&child_id) => { current = child_id; }
                None => return None,
            }
        }
        Some(current)
    }

    // ── Capability-Check ───────────────────────────────────────────────────

    fn check_cap(&self, pid: u64, cap_handle: u64, required: Rights) -> bool {
        let table = self.caps.lock();
        if let Some(cap) = table.get(crate::capability::CapId(cap_handle)) {
            cap.owner == crate::ats1000::Pid(pid as u32) && cap.rights.has(required)
        } else {
            false
        }
    }

    // ── Verzeichnis-Operationen ────────────────────────────────────────────

    pub fn mkdir(&mut self, path: &str, pid: u64, cap_handle: u64) -> Result<(), VfsError> {
        if !self.check_cap(pid, cap_handle, Rights::WRITE) {
            return Err(VfsError::PermissionDenied);
        }

        let (parent_path, name) = parent_path(path);
        let parent_id = self.resolve(&parent_path)
            .ok_or(VfsError::NotFound)?;

        {
            let parent = self.inodes.get(&parent_id).ok_or(VfsError::NotFound)?;
            if !parent.is_dir() {
                return Err(VfsError::NotADirectory);
            }
            if parent.children.contains_key(&name) {
                return Err(VfsError::AlreadyExists);
            }
        }

        let id = self.next_inode;
        self.next_inode += 1;
        let mut dir = Inode::new_dir(id, name.clone(), pid);
        dir.parent = Some(parent_id);
        self.inodes.insert(id, dir);

        self.inodes.get_mut(&parent_id).unwrap().children.insert(name, id);
        Ok(())
    }

    pub fn list_dir(&self, path: &str, _pid: u64, cap_handle: u64) -> Result<Vec<DirEntry>, VfsError> {
        if !self.check_cap(_pid, cap_handle, Rights::READ) {
            return Err(VfsError::PermissionDenied);
        }

        let dir_id = self.resolve(path).ok_or(VfsError::NotFound)?;
        let inode = self.inodes.get(&dir_id).ok_or(VfsError::NotFound)?;
        if !inode.is_dir() {
            return Err(VfsError::NotADirectory);
        }

        let mut entries = Vec::new();
        for (name, &child_id) in &inode.children {
            if let Some(child) = self.inodes.get(&child_id) {
                entries.push(DirEntry {
                    name: name.clone(),
                    file_type: child.file_type,
                    size: child.metadata.size,
                });
            }
        }
        Ok(entries)
    }

    pub fn rmdir(&mut self, path: &str, pid: u64, cap_handle: u64) -> Result<(), VfsError> {
        if !self.check_cap(pid, cap_handle, Rights::WRITE) {
            return Err(VfsError::PermissionDenied);
        }

        let dir_id = self.resolve(path).ok_or(VfsError::NotFound)?;
        if dir_id == self.root_inode {
            return Err(VfsError::CannotRemoveRoot);
        }

        let inode = self.inodes.get(&dir_id).ok_or(VfsError::NotFound)?;
        if !inode.is_dir() {
            return Err(VfsError::NotADirectory);
        }
        if !inode.children.is_empty() {
            return Err(VfsError::DirectoryNotEmpty);
        }

        let parent_id = inode.parent.ok_or(VfsError::NotFound)?;
        let name = inode.name.clone();

        self.inodes.remove(&dir_id);
        self.inodes.get_mut(&parent_id).unwrap().children.remove(&name);
        Ok(())
    }

    // ── Datei-Operationen ──────────────────────────────────────────────────

    pub fn create_file(&mut self, path: &str, pid: u64, cap_handle: u64) -> Result<(), VfsError> {
        if !self.check_cap(pid, cap_handle, Rights::WRITE) {
            return Err(VfsError::PermissionDenied);
        }

        let (parent_path, name) = parent_path(path);
        let parent_id = self.resolve(&parent_path).ok_or(VfsError::NotFound)?;

        {
            let parent = self.inodes.get(&parent_id).ok_or(VfsError::NotFound)?;
            if !parent.is_dir() {
                return Err(VfsError::NotADirectory);
            }
            if parent.children.contains_key(&name) {
                return Err(VfsError::AlreadyExists);
            }
        }

        let id = self.next_inode;
        self.next_inode += 1;
        let mut file = Inode::new_file(id, name.clone(), pid);
        file.parent = Some(parent_id);
        self.inodes.insert(id, file);

        self.inodes.get_mut(&parent_id).unwrap().children.insert(name, id);
        Ok(())
    }

    pub fn open(&mut self, path: &str, mode: OpenMode, pid: u64, cap_handle: u64) -> Result<u64, VfsError> {
        let required = if mode.allows_read() && mode.allows_write() {
            Rights::READ | Rights::WRITE
        } else if mode.allows_read() {
            Rights::READ
        } else {
            Rights::WRITE
        };

        if !self.check_cap(pid, cap_handle, required) {
            return Err(VfsError::PermissionDenied);
        }

        if mode == OpenMode::Create {
            // Datei erstellen falls nicht vorhanden
            if self.resolve(path).is_none() {
                self.create_file(path, pid, cap_handle)?;
            }
        }

        let inode_id = self.resolve(path).ok_or(VfsError::NotFound)?;
        let inode = self.inodes.get(&inode_id).ok_or(VfsError::NotFound)?;
        if !inode.is_file() {
            return Err(VfsError::IsADirectory);
        }

        let fd = self.next_fd;
        self.next_fd += 1;

        let position = match mode {
            OpenMode::Append => inode.data.len() as u64,
            _ => 0,
        };

        self.handles.insert(fd, FileHandle {
            inode_id,
            position,
            mode,
            pid,
            cap_handle,
        });

        Ok(fd)
    }

    pub fn read(&mut self, fd: u64, buf: &mut [u8], _pid: u64, cap_handle: u64) -> Result<usize, VfsError> {
        if !self.check_cap(_pid, cap_handle, Rights::READ) {
            return Err(VfsError::PermissionDenied);
        }

        let handle = self.handles.get(&fd).ok_or(VfsError::BadFileDescriptor)?;
        if !handle.mode.allows_read() {
            return Err(VfsError::InvalidMode);
        }

        let inode = self.inodes.get(&handle.inode_id).ok_or(VfsError::NotFound)?;
        let pos = handle.position as usize;
        let available = inode.data.len().saturating_sub(pos);
        let to_read = buf.len().min(available);

        buf[..to_read].copy_from_slice(&inode.data[pos..pos + to_read]);

        self.handles.get_mut(&fd).unwrap().position += to_read as u64;
        Ok(to_read)
    }

    pub fn write(&mut self, fd: u64, data: &[u8], _pid: u64, cap_handle: u64) -> Result<usize, VfsError> {
        if !self.check_cap(_pid, cap_handle, Rights::WRITE) {
            return Err(VfsError::PermissionDenied);
        }

        let handle = self.handles.get(&fd).ok_or(VfsError::BadFileDescriptor)?;
        if !handle.mode.allows_write() {
            return Err(VfsError::InvalidMode);
        }

        let inode_id = handle.inode_id;
        let pos = handle.position as usize;

        let inode = self.inodes.get_mut(&inode_id).ok_or(VfsError::NotFound)?;
        if pos + data.len() > inode.data.len() {
            inode.data.resize(pos + data.len(), 0);
        }
        inode.data[pos..pos + data.len()].copy_from_slice(data);
        inode.metadata.size = inode.data.len() as u64;
        inode.metadata.modified_at = 1; // vereinfachter Zeitstempel

        self.handles.get_mut(&fd).unwrap().position += data.len() as u64;
        Ok(data.len())
    }

    pub fn close(&mut self, fd: u64) -> Result<(), VfsError> {
        self.handles.remove(&fd).ok_or(VfsError::BadFileDescriptor)?;
        Ok(())
    }

    pub fn seek(&mut self, fd: u64, offset: u64) -> Result<u64, VfsError> {
        let handle = self.handles.get_mut(&fd).ok_or(VfsError::BadFileDescriptor)?;
        handle.position = offset;
        Ok(offset)
    }

    pub fn stat(&self, path: &str, _pid: u64, cap_handle: u64) -> Result<FileMetadata, VfsError> {
        if !self.check_cap(_pid, cap_handle, Rights::READ) {
            return Err(VfsError::PermissionDenied);
        }
        let inode_id = self.resolve(path).ok_or(VfsError::NotFound)?;
        let inode = self.inodes.get(&inode_id).ok_or(VfsError::NotFound)?;
        Ok(inode.metadata.clone())
    }

    // ── Symlink ───────────────────────────────────────────────────────────

    pub fn create_symlink(&mut self, path: &str, target: &str, pid: u64, cap_handle: u64) -> Result<(), VfsError> {
        if !self.check_cap(pid, cap_handle, Rights::WRITE) {
            return Err(VfsError::PermissionDenied);
        }

        let (parent_path, name) = parent_path(path);
        let parent_id = self.resolve(&parent_path).ok_or(VfsError::NotFound)?;

        {
            let parent = self.inodes.get(&parent_id).ok_or(VfsError::NotFound)?;
            if !parent.is_dir() {
                return Err(VfsError::NotADirectory);
            }
            if parent.children.contains_key(&name) {
                return Err(VfsError::AlreadyExists);
            }
        }

        let id = self.next_inode;
        self.next_inode += 1;
        let mut link = Inode::new_symlink(id, name.clone(), target.to_string(), pid);
        link.parent = Some(parent_id);
        self.inodes.insert(id, link);

        self.inodes.get_mut(&parent_id).unwrap().children.insert(name, id);
        Ok(())
    }

    pub fn read_symlink(&self, path: &str, _pid: u64, cap_handle: u64) -> Result<String, VfsError> {
        if !self.check_cap(_pid, cap_handle, Rights::READ) {
            return Err(VfsError::PermissionDenied);
        }
        let inode_id = self.resolve(path).ok_or(VfsError::NotFound)?;
        let inode = self.inodes.get(&inode_id).ok_or(VfsError::NotFound)?;
        if inode.file_type != FileType::Symlink {
            return Err(VfsError::NotASymlink);
        }
        inode.symlink_target.clone().ok_or(VfsError::NotFound)
    }

    // ── Datei löschen ──────────────────────────────────────────────────────

    pub fn remove_file(&mut self, path: &str, pid: u64, cap_handle: u64) -> Result<(), VfsError> {
        if !self.check_cap(pid, cap_handle, Rights::WRITE) {
            return Err(VfsError::PermissionDenied);
        }

        let inode_id = self.resolve(path).ok_or(VfsError::NotFound)?;
        let inode = self.inodes.get(&inode_id).ok_or(VfsError::NotFound)?;
        if !inode.is_file() {
            return Err(VfsError::IsADirectory);
        }

        let parent_id = inode.parent.ok_or(VfsError::NotFound)?;
        let name = inode.name.clone();

        // Offene Handles schliessen
        let fds_to_close: Vec<u64> = self.handles.iter()
            .filter(|(_, h)| h.inode_id == inode_id)
            .map(|(&fd, _)| fd)
            .collect();
        for fd in fds_to_close {
            self.handles.remove(&fd);
        }

        self.inodes.remove(&inode_id);
        self.inodes.get_mut(&parent_id).unwrap().children.remove(&name);
        Ok(())
    }

    // ── Baum-Anzeige (Debug) ───────────────────────────────────────────────

    pub fn tree(&self) -> Vec<String> {
        let mut out = Vec::new();
        self.print_subtree(self.root_inode, "", &mut out);
        out
    }

    fn print_subtree(&self, inode_id: u64, prefix: &str, out: &mut Vec<String>) {
        let inode = match self.inodes.get(&inode_id) {
            Some(i) => i,
            None => return,
        };

        let type_marker = match inode.file_type {
            FileType::Directory => "[DIR] ",
            FileType::File => "      ",
            FileType::Symlink => "[LINK]",
        };

        let size_str = if inode.is_file() {
            format!(" ({} bytes)", inode.data.len())
        } else {
            String::new()
        };

        out.push(format!("{}{}{}{}", prefix, type_marker, inode.name, size_str));

        if inode.is_dir() {
            let child_count = inode.children.len();
            for (idx, (_, &child_id)) in inode.children.iter().enumerate() {
                let is_last = idx == child_count - 1;
                let new_prefix = if is_last {
                    format!("{}    ", prefix)
                } else {
                    format!("{}|   ", prefix)
                };
                self.print_subtree(child_id, &new_prefix, out);
            }
        }
    }
}

// ─── DirEntry ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub file_type: FileType,
    pub size: u64,
}

// ─── Fehler ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VfsError {
    NotFound,
    AlreadyExists,
    NotADirectory,
    IsADirectory,
    NotASymlink,
    DirectoryNotEmpty,
    CannotRemoveRoot,
    PermissionDenied,
    BadFileDescriptor,
    InvalidMode,
    IoError(String),
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ats1000::Pid;
    use crate::capability::ResourceType;

    fn setup_vfs() -> Vfs {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        // Grant full rights to process 1
        {
            let mut table = caps.lock();
            let cap = table.create(Pid(1), ResourceType::FileSystem, 1, Rights::READ | Rights::WRITE | Rights::EXEC | Rights::DELEGATE).0;
            // cap handle = 1 (first cap)
        }
        // We need the cap handle. Let's use a simpler approach:
        // Create caps manually with known handle
        Vfs::new(caps)
    }

    fn grant_full_caps(caps: &Arc<Mutex<CapabilityTable>>, pid: u64) -> u64 {
        let mut table = caps.lock();
        let cap_id = table.create(Pid(pid as u32), ResourceType::FileSystem, 1, Rights::READ | Rights::WRITE | Rights::EXEC | Rights::DELEGATE);
        cap_id.0
    }

    // ── Verzeichnis-Tests ──────────────────────────────────────────────────

    #[test]
    fn test_root_exists() {
        let vfs = setup_vfs();
        let meta = vfs.stat("/", 1, 0).unwrap_or_else(|_| FileMetadata {
            file_type: FileType::Directory,
            size: 0,
            created_at: 0,
            modified_at: 0,
            owner_pid: 0,
            permissions: Rights::READ,
        });
        // Root might fail on caps with handle 0, let's test resolve instead
        assert_eq!(vfs.resolve("/"), Some(1));
        assert_eq!(vfs.resolve(""), Some(1));
    }

    #[test]
    fn test_mkdir_and_list() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.mkdir("/home", 1, cap).unwrap();
        vfs.mkdir("/home/user", 1, cap).unwrap();
        vfs.mkdir("/var", 1, cap).unwrap();

        let entries = vfs.list_dir("/", 1, cap).unwrap();
        assert_eq!(entries.len(), 2);
        let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
        assert!(names.contains(&"home"));
        assert!(names.contains(&"var"));

        let home_entries = vfs.list_dir("/home", 1, cap).unwrap();
        assert_eq!(home_entries.len(), 1);
        assert_eq!(home_entries[0].name, "user");
        assert_eq!(home_entries[0].file_type, FileType::Directory);
    }

    #[test]
    fn test_mkdir_nested_path() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.mkdir("/a", 1, cap).unwrap();
        vfs.mkdir("/a/b", 1, cap).unwrap();
        vfs.mkdir("/a/b/c", 1, cap).unwrap();

        assert!(vfs.resolve("/a/b/c").is_some());
        let entries = vfs.list_dir("/a/b", 1, cap).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, "c");
    }

    #[test]
    fn test_mkdir_already_exists() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.mkdir("/test", 1, cap).unwrap();
        let result = vfs.mkdir("/test", 1, cap);
        assert_eq!(result, Err(VfsError::AlreadyExists));
    }

    #[test]
    fn test_mkdir_parent_not_dir() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.create_file("/file.txt", 1, cap).unwrap();
        let result = vfs.mkdir("/file.txt/subdir", 1, cap);
        assert_eq!(result, Err(VfsError::NotADirectory));
    }

    #[test]
    fn test_mkdir_parent_not_found() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let result = vfs.mkdir("/nonexistent/subdir", 1, cap);
        assert_eq!(result, Err(VfsError::NotFound));
    }

    #[test]
    fn test_rmdir_empty() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.mkdir("/temp", 1, cap).unwrap();
        vfs.rmdir("/temp", 1, cap).unwrap();
        assert!(vfs.resolve("/temp").is_none());
    }

    #[test]
    fn test_rmdir_not_empty() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.mkdir("/parent", 1, cap).unwrap();
        vfs.mkdir("/parent/child", 1, cap).unwrap();
        let result = vfs.rmdir("/parent", 1, cap);
        assert_eq!(result, Err(VfsError::DirectoryNotEmpty));
    }

    #[test]
    fn test_rmdir_root_fails() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let result = vfs.rmdir("/", 1, cap);
        assert_eq!(result, Err(VfsError::CannotRemoveRoot));
    }

    // ── Datei-Tests ────────────────────────────────────────────────────────

    #[test]
    fn test_create_and_stat() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.create_file("/test.txt", 1, cap).unwrap();
        let meta = vfs.stat("/test.txt", 1, cap).unwrap();
        assert_eq!(meta.file_type, FileType::File);
        assert_eq!(meta.size, 0);
        assert_eq!(meta.owner_pid, 1);
    }

    #[test]
    fn test_create_file_already_exists() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.create_file("/file.txt", 1, cap).unwrap();
        let result = vfs.create_file("/file.txt", 1, cap);
        assert_eq!(result, Err(VfsError::AlreadyExists));
    }

    #[test]
    fn test_open_write_read() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.create_file("/data.bin", 1, cap).unwrap();
        let fd = vfs.open("/data.bin", OpenMode::ReadWrite, 1, cap).unwrap();

        let data = b"Hello, ShivaCore VFS!";
        let written = vfs.write(fd, data, 1, cap).unwrap();
        assert_eq!(written, data.len());

        vfs.seek(fd, 0).unwrap();
        let mut buf = [0u8; 64];
        let read = vfs.read(fd, &mut buf, 1, cap).unwrap();
        assert_eq!(read, data.len());
        assert_eq!(&buf[..read], data);
    }

    #[test]
    fn test_open_create_mode() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let fd = vfs.open("/auto_created.txt", OpenMode::Create, 1, cap).unwrap();
        let data = b"auto-created";
        vfs.write(fd, data, 1, cap).unwrap();
        vfs.close(fd).unwrap();

        // Zweites Open im Read-Modus
        let fd2 = vfs.open("/auto_created.txt", OpenMode::Read, 1, cap).unwrap();
        let mut buf = [0u8; 64];
        let read = vfs.read(fd2, &mut buf, 1, cap).unwrap();
        assert_eq!(&buf[..read], data);
    }

    #[test]
    fn test_append_mode() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let fd = vfs.open("/log.txt", OpenMode::Create, 1, cap).unwrap();
        vfs.write(fd, b"line1\n", 1, cap).unwrap();
        vfs.close(fd).unwrap();

        let fd = vfs.open("/log.txt", OpenMode::Append, 1, cap).unwrap();
        vfs.write(fd, b"line2\n", 1, cap).unwrap();
        vfs.close(fd).unwrap();

        let fd = vfs.open("/log.txt", OpenMode::Read, 1, cap).unwrap();
        let mut buf = [0u8; 64];
        let read = vfs.read(fd, &mut buf, 1, cap).unwrap();
        assert_eq!(&buf[..read], b"line1\nline2\n");
    }

    #[test]
    fn test_seek_and_partial_read() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let fd = vfs.open("/seek_test.bin", OpenMode::Create, 1, cap).unwrap();
        vfs.write(fd, b"0123456789", 1, cap).unwrap();
        vfs.close(fd).unwrap();

        let fd = vfs.open("/seek_test.bin", OpenMode::Read, 1, cap).unwrap();
        vfs.seek(fd, 3).unwrap();
        let mut buf = [0u8; 4];
        let read = vfs.read(fd, &mut buf, 1, cap).unwrap();
        assert_eq!(read, 4);
        assert_eq!(&buf, b"3456");
    }

    #[test]
    fn test_read_past_end() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let fd = vfs.open("/short.txt", OpenMode::Create, 1, cap).unwrap();
        vfs.write(fd, b"hi", 1, cap).unwrap();
        vfs.close(fd).unwrap();

        let fd = vfs.open("/short.txt", OpenMode::Read, 1, cap).unwrap();
        let mut buf = [0u8; 100];
        let read = vfs.read(fd, &mut buf, 1, cap).unwrap();
        assert_eq!(read, 2);
        assert_eq!(&buf[..2], b"hi");
    }

    #[test]
    fn test_close_and_reopen() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let fd = vfs.open("/reopen.txt", OpenMode::Create, 1, cap).unwrap();
        vfs.write(fd, b"data", 1, cap).unwrap();
        vfs.close(fd).unwrap();

        // Nach Close: write sollte fehlschlagen
        let result = vfs.write(fd, b"more", 1, cap);
        assert_eq!(result, Err(VfsError::BadFileDescriptor));
    }

    #[test]
    fn test_open_directory_fails() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.mkdir("/mydir", 1, cap).unwrap();
        let result = vfs.open("/mydir", OpenMode::Read, 1, cap);
        assert_eq!(result, Err(VfsError::IsADirectory));
    }

    #[test]
    fn test_remove_file() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.create_file("/delete_me.txt", 1, cap).unwrap();
        vfs.remove_file("/delete_me.txt", 1, cap).unwrap();
        assert!(vfs.resolve("/delete_me.txt").is_none());
    }

    #[test]
    fn test_remove_file_closes_handles() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let fd = vfs.open("/auto.txt", OpenMode::Create, 1, cap).unwrap();
        vfs.write(fd, b"data", 1, cap).unwrap();

        vfs.remove_file("/auto.txt", 1, cap).unwrap();

        // Handle sollte nach Loeschen ungueltig sein
        let result = vfs.read(fd, &mut [0u8; 4], 1, cap);
        assert_eq!(result, Err(VfsError::BadFileDescriptor));
    }

    // ── Symlink-Tests ──────────────────────────────────────────────────────

    #[test]
    fn test_create_and_read_symlink() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.create_file("/target.txt", 1, cap).unwrap();
        vfs.create_symlink("/link.txt", "/target.txt", 1, cap).unwrap();

        let target = vfs.read_symlink("/link.txt", 1, cap).unwrap();
        assert_eq!(target, "/target.txt");

        let meta = vfs.stat("/link.txt", 1, cap).unwrap();
        assert_eq!(meta.file_type, FileType::Symlink);
    }

    #[test]
    fn test_symlink_already_exists() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.create_symlink("/link", "/target", 1, cap).unwrap();
        let result = vfs.create_symlink("/link", "/other", 1, cap);
        assert_eq!(result, Err(VfsError::AlreadyExists));
    }

    // ── Pfad-Normalisierung ─────────────────────────────────────────────────

    #[test]
    fn test_path_normalization() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.mkdir("/a", 1, cap).unwrap();
        vfs.mkdir("/a/b", 1, cap).unwrap();

        // /a/../a/b sollte zu /a/b aufgeloest werden
        assert!(vfs.resolve("/a/../a/b").is_some());
        // /a/./b sollte ebenfalls funktionieren
        assert!(vfs.resolve("/a/./b").is_some());
    }

    #[test]
    fn test_resolve_nonexistent() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let vfs = Vfs::new(caps);
        assert!(vfs.resolve("/does/not/exist").is_none());
    }

    // ── Capability-Gating-Tests ─────────────────────────────────────────────

    #[test]
    fn test_mkdir_without_write_cap() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());

        // Prozess 2 bekommt nur READ (kein WRITE)
        let cap_read = {
            let mut table = caps.lock();
            table.create(Pid(2), ResourceType::FileSystem, 2, Rights::READ).0
        };

        let result = vfs.mkdir("/test", 2, cap_read);
        assert_eq!(result, Err(VfsError::PermissionDenied));
    }

    #[test]
    fn test_read_without_read_cap() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());

        // Prozess 2 bekommt nur WRITE (kein READ)
        let cap_write = {
            let mut table = caps.lock();
            table.create(Pid(2), ResourceType::FileSystem, 2, Rights::WRITE).0
        };

        vfs.create_file("/file.txt", 2, cap_write).unwrap();
        let result = vfs.stat("/file.txt", 2, cap_write);
        assert_eq!(result, Err(VfsError::PermissionDenied));
    }

    #[test]
    fn test_open_read_mode_with_only_write_cap() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());

        let cap_write = {
            let mut table = caps.lock();
            table.create(Pid(2), ResourceType::FileSystem, 2, Rights::WRITE).0
        };

        vfs.create_file("/data.txt", 2, cap_write).unwrap();
        // Versuch Open im Read-Modus mit nur WRITE cap -> Denied
        let result = vfs.open("/data.txt", OpenMode::Read, 2, cap_write);
        assert_eq!(result, Err(VfsError::PermissionDenied));
    }

    // ── Baum-Anzeige ───────────────────────────────────────────────────────

    #[test]
    fn test_tree_display() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.mkdir("/home", 1, cap).unwrap();
        vfs.mkdir("/home/user", 1, cap).unwrap();
        vfs.create_file("/home/user/notes.txt", 1, cap).unwrap();
        vfs.mkdir("/var", 1, cap).unwrap();

        let tree = vfs.tree();
        assert!(!tree.is_empty());
        assert!(tree.iter().any(|l| l.contains("home")));
        assert!(tree.iter().any(|l| l.contains("notes.txt")));
        assert!(tree.iter().any(|l| l.contains("var")));
    }

    #[test]
    fn test_stat_after_write() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let fd = vfs.open("/sized.txt", OpenMode::Create, 1, cap).unwrap();
        vfs.write(fd, b"12345678", 1, cap).unwrap();
        vfs.close(fd).unwrap();

        let meta = vfs.stat("/sized.txt", 1, cap).unwrap();
        assert_eq!(meta.size, 8);
    }

    #[test]
    fn test_multiple_files_in_directory() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        vfs.mkdir("/docs", 1, cap).unwrap();
        vfs.create_file("/docs/a.txt", 1, cap).unwrap();
        vfs.create_file("/docs/b.txt", 1, cap).unwrap();
        vfs.create_file("/docs/c.txt", 1, cap).unwrap();

        let entries = vfs.list_dir("/docs", 1, cap).unwrap();
        assert_eq!(entries.len(), 3);
    }

    #[test]
    fn test_write_expands_file() {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let mut vfs = Vfs::new(caps.clone());
        let cap = grant_full_caps(&caps, 1);

        let fd = vfs.open("/expand.txt", OpenMode::Create, 1, cap).unwrap();
        vfs.write(fd, b"hello", 1, cap).unwrap();
        vfs.seek(fd, 10).unwrap();
        vfs.write(fd, b"world", 1, cap).unwrap();
        vfs.close(fd).unwrap();

        let meta = vfs.stat("/expand.txt", 1, cap).unwrap();
        assert_eq!(meta.size, 15); // 10 offset + 5 bytes

        let fd = vfs.open("/expand.txt", OpenMode::Read, 1, cap).unwrap();
        let mut buf = [0u8; 20];
        let read = vfs.read(fd, &mut buf, 1, cap).unwrap();
        assert_eq!(read, 15);
        assert_eq!(&buf[..5], b"hello");
        assert_eq!(&buf[10..15], b"world");
    }
}
