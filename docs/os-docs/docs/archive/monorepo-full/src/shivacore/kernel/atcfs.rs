// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
//! ShivaCore Kernel — ATCFS (Content-Addressed File System) in Rust.
//!
//! Portiert atcfs.py (Python) nach Rust.
//! Content-Adressierung mit SHA3-256, Praefix "atc1".
//! Owner-basierte Zugriffskontrolle + Manifest-Export fuer On-Chain-Anchoring.
//! Implementiert das ats1000 FileSystem-Trait.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::ats1000::FileSystem;
use crate::capability::{CapabilityTable, Pid, ResourceType, Rights};

/// Content-ID: SHA3-256 Hash mit "atc1" Praefix
pub type Cid = String;

/// Datei-Handle (fuer ats1000 FileSystem-Trait)
pub type FileHandle = u64;

/// Datei-/Verzeichnis-Knoten
#[derive(Debug, Clone, PartialEq)]
pub struct AtcNode {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub owner: Pid,
    pub content_cid: Cid,
    pub size: u64,
    pub created: u64,
    pub modified: u64,
    pub children: Vec<String>,
}

/// ATCFS — Content-Addressed File System
pub struct AtcFileSystem {
    nodes: BTreeMap<String, AtcNode>,
    content: BTreeMap<Cid, Vec<u8>>,
    open_files: BTreeMap<FileHandle, OpenFile>,
    next_fh: AtomicU64,
    /// Virtuelle Zeit (simuliert)
    clock: AtomicU64,
}

/// Offene Datei
#[derive(Debug, Clone)]
struct OpenFile {
    path: String,
    mode: u8,  // 0=read, 1=write, 2=read+write
    offset: u64,
}

/// Fehler bei FS-Operationen
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {

    NotFound,
    PermissionDenied,
    AlreadyExists,
    NotADirectory,
    IsADirectory,
    InvalidHandle,
    OutOfSpace,
}

/// Simuliertes SHA3-256 (deterministisch, fuer Tests)
/// In Produktion: echte SHA3-256 Implementierung
fn sha3_256(data: &[u8]) -> String {
    // Vereinfacht: deterministischer Hash basierend auf Byte-Werten
    let mut state: [u64; 4] = [0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a];
    for (i, &b) in data.iter().enumerate() {
        state[i % 4] = state[i % 4]
            .wrapping_mul(0x100000001b3)
            .wrapping_add(b as u64);
    }
    let mut result = String::with_capacity(64);
    for s in &state {
        for byte in s.to_le_bytes() {
            result.push_str(&format!("{:02x}", byte));
        }
    }
    result
}

/// Erzeugt eine Content-ID: "atc1" + SHA3-256("atcfs_v1||" + data)
pub fn atc_content_id(data: &[u8]) -> Cid {
    let mut prefixed = b"atcfs_v1||".to_vec();
    prefixed.extend_from_slice(data);
    format!("atc1{}", sha3_256(&prefixed))
}

impl AtcFileSystem {
    pub fn new() -> Self {
        let mut fs = Self {
            nodes: BTreeMap::new(),
            content: BTreeMap::new(),
            open_files: BTreeMap::new(),
            next_fh: AtomicU64::new(1),
            clock: AtomicU64::new(1),
        };
        fs.init_root();
        fs
    }

    fn now(&self) -> u64 {
        self.clock.fetch_add(1, Ordering::SeqCst)
    }

    fn init_root(&mut self) {
        let now = self.now();
        self.nodes.insert("/".to_string(), AtcNode {
            path: "/".to_string(), name: "/".to_string(),
            is_dir: true, owner: Pid(0), content_cid: "".to_string(),
            size: 0, created: now, modified: now, children: Vec::new(),
        });
        for d in ["atc", "home", "tmp", "bin", "var"] {
            self.mkdir(&format!("/{}", d), Pid(0));
        }
    }

    fn parent_path(path: &str) -> String {
        let trimmed = path.trim_end_matches('/');
        match trimmed.rfind('/') {
            Some(0) | None => "/".to_string(),
            Some(idx) => trimmed[..idx].to_string(),
        }
    }

    fn node_name(path: &str) -> String {
        path.trim_end_matches('/').rsplit('/').next().unwrap_or("").to_string()
    }

    fn mkdir(&mut self, path: &str, owner: Pid) {
        if self.nodes.contains_key(path) { return; }
        let parent = Self::parent_path(path);
        if !self.nodes.contains_key(&parent) {
            self.mkdir(&parent, owner);
        }
        let now = self.now();
        let node = AtcNode {
            path: path.to_string(), name: Self::node_name(path),
            is_dir: true, owner, content_cid: "".to_string(),
            size: 0, created: now, modified: now, children: Vec::new(),
        };
        self.nodes.insert(path.to_string(), node);
        if let Some(p) = self.nodes.get_mut(&parent) {
            p.children.push(path.to_string());
        }
    }

    pub fn exists(&self, path: &str) -> bool {
        self.nodes.contains_key(path.trim_end_matches('/'))
            || path == "/"
    }

    /// Prueft Lesezugriff
    fn check_read(&self, node: &AtcNode, actor: Pid) -> Result<(), FsError> {
        if node.owner == actor || actor == Pid(0) { return Ok(()); }
        // Oeffentliche Pfade
        if node.path.starts_with("/atc/") || node.path.starts_with("/tmp/") {
            return Ok(());
        }
        Err(FsError::PermissionDenied)
    }

    /// Prueft Schreibzugriff
    fn check_write(&self, node: &AtcNode, actor: Pid) -> Result<(), FsError> {
        if node.owner == actor || actor == Pid(0) { return Ok(()); }
        Err(FsError::PermissionDenied)
    }

    /// Schreibt Daten in eine Datei
    pub fn write_file(
        &mut self,
        caps: &CapabilityTable,
        path: &str,
        data: &[u8],
        actor: Pid,
    ) -> Result<AtcNode, FsError> {
        let parent = Self::parent_path(path);
        if !self.nodes.contains_key(&parent) {
            self.mkdir(&parent, actor);
        }

        // Wenn existiert: Schreibrechte pruefen
        if let Some(existing) = self.nodes.get(path) {
            if existing.is_dir { return Err(FsError::IsADirectory); }
            self.check_write(existing, actor)?;
        }

        let cid = atc_content_id(data);
        self.content.insert(cid.clone(), data.to_vec());

        let now = self.now();
        let (created, owner) = if let Some(e) = self.nodes.get(path) {
            (e.created, e.owner)
        } else {
            (now, actor)
        };

        let node = AtcNode {
            path: path.to_string(), name: Self::node_name(path),
            is_dir: false, owner, content_cid: cid.clone(),
            size: data.len() as u64, created, modified: now,
            children: Vec::new(),
        };
        self.nodes.insert(path.to_string(), node.clone());

        // Parent children aktualisieren
        if let Some(p) = self.nodes.get_mut(&parent) {
            if !p.children.contains(&path.to_string()) {
                p.children.push(path.to_string());
            }
        }

        Ok(node)
    }

    /// Liest eine Datei (gibt CID + Node zurueck)
    pub fn read_file(
        &self,
        caps: &CapabilityTable,
        path: &str,
        actor: Pid,
    ) -> Result<(Cid, AtcNode), FsError> {
        let node = self.nodes.get(path).ok_or(FsError::NotFound)?;
        if node.is_dir { return Err(FsError::IsADirectory); }
        self.check_read(node, actor)?;
        Ok((node.content_cid.clone(), node.clone()))
    }

    /// Holt den Inhalt einer Datei ueber CID
    pub fn get_content(&self, cid: &Cid) -> Option<&[u8]> {
        self.content.get(cid).map(|v| v.as_slice())
    }

    /// Listet Verzeichnisinhalte
    pub fn ls(&self, path: &str) -> Vec<AtcNode> {
        let node = match self.nodes.get(path) {
            Some(n) if n.is_dir => n,
            _ => return Vec::new(),
        };
        node.children.iter()
            .filter_map(|c| self.nodes.get(c))
            .cloned()
            .collect()
    }

    /// Exportiert Manifest fuer On-Chain-Anchoring
    pub fn export_manifest(&self) -> Manifest {
        let mut entries: Vec<String> = self.nodes.iter()
            .filter(|(_, n)| !n.is_dir)
            .map(|(p, n)| format!("{}:{}", p, n.content_cid))
            .collect();
        entries.sort();

        let combined = entries.join("|");
        Manifest {
            root_hash: sha3_256(combined.as_bytes()),
            file_count: entries.len() as u64,
            total_size: self.content.values().map(|v| v.len() as u64).sum(),
            generated_at: self.now(),
        }
    }

    /// Loescht eine Datei
    pub fn delete_file(
        &mut self,
        path: &str,
        actor: Pid,
    ) -> Result<(), FsError> {
        let node = self.nodes.get(path).ok_or(FsError::NotFound)?;
        if node.is_dir { return Err(FsError::IsADirectory); }
        self.check_write(node, actor)?;

        let cid = node.content_cid.clone();
        let parent = Self::parent_path(path);

        self.nodes.remove(path);
        self.content.remove(&cid);

        if let Some(p) = self.nodes.get_mut(&parent) {
            p.children.retain(|c| c != path);
        }

        Ok(())
    }

    /// Erzeugt ein Verzeichnis
    pub fn create_dir(&mut self, path: &str, actor: Pid) -> Result<(), FsError> {
        if self.nodes.contains_key(path) {
            return Err(FsError::AlreadyExists);
        }
        self.mkdir(path, actor);
        Ok(())
    }
}

/// Manifest fuer On-Chain-Anchoring
#[derive(Debug, Clone, PartialEq)]
pub struct Manifest {
    pub root_hash: String,
    pub file_count: u64,
    pub total_size: u64,
    pub generated_at: u64,
}

/// ats1000 FileSystem-Trait-Implementierung
impl FileSystem for AtcFileSystem {
    fn open(&mut self, path: &str, mode: u8) -> Option<FileHandle> {
        let node = self.nodes.get(path)?;
        if node.is_dir { return None; }

        let fh = self.next_fh.fetch_add(1, Ordering::SeqCst);
        self.open_files.insert(fh, OpenFile {
            path: path.to_string(),
            mode,
            offset: 0,
        });
        Some(fh)
    }

    fn read(&mut self, fh: FileHandle, buf: &mut [u8]) -> u64 {
        let open = match self.open_files.get(&fh) {
            Some(f) => f.clone(),
            None => return 0,
        };

        let node = match self.nodes.get(&open.path) {
            Some(n) => n,
            None => return 0,
        };

        let content = match self.content.get(&node.content_cid) {
            Some(c) => c,
            None => return 0,
        };

        let offset = open.offset as usize;
        if offset >= content.len() { return 0; }

        let available = content.len() - offset;
        let to_read = buf.len().min(available);
        buf[..to_read].copy_from_slice(&content[offset..offset + to_read]);

        if let Some(f) = self.open_files.get_mut(&fh) {
            f.offset += to_read as u64;
        }

        to_read as u64
    }

    fn write(&mut self, fh: FileHandle, data: &[u8]) -> u64 {
        let open = match self.open_files.get(&fh) {
            Some(f) => f.clone(),
            None => return 0,
        };
        let path = open.path.clone();

        // Vereinfacht: ueberschreibt gesamte Datei
        let cid = atc_content_id(data);
        self.content.insert(cid.clone(), data.to_vec());

        let now = self.now();
        if let Some(node) = self.nodes.get_mut(&path) {
            node.content_cid = cid;
            node.size = data.len() as u64;
            node.modified = now;
        }

        data.len() as u64
    }

    fn close(&mut self, fh: FileHandle) -> bool {
        self.open_files.remove(&fh).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pid(n: u32) -> Pid { Pid(n) }

    #[test]
    fn test_content_id_deterministic() {
        let data = b"hello atcfs";
        let cid1 = atc_content_id(data);
        let cid2 = atc_content_id(data);
        assert_eq!(cid1, cid2);
        assert!(cid1.starts_with("atc1"));
    }

    #[test]
    fn test_content_id_different_data() {
        let cid1 = atc_content_id(b"hello");
        let cid2 = atc_content_id(b"world");
        assert_ne!(cid1, cid2);
    }

    #[test]
    fn test_init_root_directories() {
        let fs = AtcFileSystem::new();
        assert!(fs.exists("/"));
        assert!(fs.exists("/atc"));
        assert!(fs.exists("/home"));
        assert!(fs.exists("/tmp"));
        assert!(fs.exists("/bin"));
        assert!(fs.exists("/var"));
    }

    #[test]
    fn test_write_and_read_file() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        let node = fs.write_file(&caps, "/home/alice/test.txt", b"hello world", pid(1)).unwrap();
        assert_eq!(node.size, 11);
        assert!(node.content_cid.starts_with("atc1"));

        let (cid, node2) = fs.read_file(&caps, "/home/alice/test.txt", pid(1)).unwrap();
        assert_eq!(cid, node.content_cid);
        assert_eq!(node2.size, 11);

        let content = fs.get_content(&cid).unwrap();
        assert_eq!(content, b"hello world");
    }

    #[test]
    fn test_read_nonexistent() {
        let fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();
        assert_eq!(fs.read_file(&caps, "/nonexistent", pid(1)), Err(FsError::NotFound));
    }

    #[test]
    fn test_read_directory_rejected() {
        let fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();
        assert_eq!(fs.read_file(&caps, "/home", pid(1)), Err(FsError::IsADirectory));
    }

    #[test]
    fn test_write_directory_rejected() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();
        assert_eq!(fs.write_file(&caps, "/home", b"data", pid(1)), Err(FsError::IsADirectory));
    }

    #[test]
    fn test_permission_denied() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        // pid(1) schreibt nach /home/alice/secret.txt
        fs.write_file(&caps, "/home/alice/secret.txt", b"secret", pid(1)).unwrap();

        // pid(2) kann nicht lesen (owner-only in /home/)
        assert_eq!(fs.read_file(&caps, "/home/alice/secret.txt", pid(2)), Err(FsError::PermissionDenied));
    }

    #[test]
    fn test_public_paths_readable() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        // pid(1) schreibt nach /atc/public.txt
        fs.write_file(&caps, "/atc/public.txt", b"public data", pid(1)).unwrap();

        // pid(2) kann lesen (oeffentlicher Pfad)
        let (_, node) = fs.read_file(&caps, "/atc/public.txt", pid(2)).unwrap();
        assert_eq!(node.size, 11);
    }

    #[test]
    fn test_ls_directory() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        fs.write_file(&caps, "/atc/file1.txt", b"a", pid(1)).unwrap();
        fs.write_file(&caps, "/atc/file2.txt", b"b", pid(1)).unwrap();

        let entries = fs.ls("/atc");
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_delete_file() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        fs.write_file(&caps, "/tmp/test.txt", b"data", pid(1)).unwrap();
        assert!(fs.exists("/tmp/test.txt"));

        fs.delete_file("/tmp/test.txt", pid(1)).unwrap();
        assert!(!fs.exists("/tmp/test.txt"));
    }

    #[test]
    fn test_delete_without_permission() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        fs.write_file(&caps, "/home/alice/file.txt", b"data", pid(1)).unwrap();
        assert_eq!(fs.delete_file("/home/alice/file.txt", pid(2)), Err(FsError::PermissionDenied));
    }

    #[test]
    fn test_create_dir() {
        let mut fs = AtcFileSystem::new();
        fs.create_dir("/home/alice/docs", pid(1)).unwrap();
        assert!(fs.exists("/home/alice/docs"));

        let entries = fs.ls("/home/alice");
        assert!(entries.iter().any(|e| e.name == "docs"));
    }

    #[test]
    fn test_create_dir_already_exists() {
        let mut fs = AtcFileSystem::new();
        fs.create_dir("/test", pid(1)).unwrap();
        assert_eq!(fs.create_dir("/test", pid(1)), Err(FsError::AlreadyExists));
    }

    #[test]
    fn test_manifest_export() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        fs.write_file(&caps, "/atc/a.txt", b"aaa", pid(1)).unwrap();
        fs.write_file(&caps, "/atc/b.txt", b"bbb", pid(1)).unwrap();

        let m = fs.export_manifest();
        assert_eq!(m.file_count, 2);
        assert_eq!(m.total_size, 6);
        assert!(!m.root_hash.is_empty());
    }

    #[test]
    fn test_manifest_deterministic() {
        let mut fs1 = AtcFileSystem::new();
        let mut fs2 = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        fs1.write_file(&caps, "/atc/a.txt", b"aaa", pid(1)).unwrap();
        fs1.write_file(&caps, "/atc/b.txt", b"bbb", pid(1)).unwrap();

        fs2.write_file(&caps, "/atc/a.txt", b"aaa", pid(1)).unwrap();
        fs2.write_file(&caps, "/atc/b.txt", b"bbb", pid(1)).unwrap();

        let m1 = fs1.export_manifest();
        let m2 = fs2.export_manifest();
        assert_eq!(m1.root_hash, m2.root_hash);
    }

    #[test]
    fn test_ats1000_filesystem_trait() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        // Datei schreiben
        fs.write_file(&caps, "/tmp/test.bin", b"binary data", pid(1)).unwrap();

        // ats1000 open
        let fh = fs.open("/tmp/test.bin", 0).unwrap();
        assert!(fh > 0);

        // ats1000 read
        let mut buf = [0u8; 100];
        let n = fs.read(fh, &mut buf);
        assert_eq!(n, 11);
        assert_eq!(&buf[..11], b"binary data");

        // ats1000 close
        assert!(fs.close(fh));
        // Double-close fails
        assert!(!fs.close(fh));
    }

    #[test]
    fn test_ats1000_open_nonexistent() {
        let mut fs = AtcFileSystem::new();
        assert!(fs.open("/nonexistent", 0).is_none());
    }

    #[test]
    fn test_ats1000_read_after_close() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();
        fs.write_file(&caps, "/tmp/test.txt", b"data", pid(1)).unwrap();

        let fh = fs.open("/tmp/test.txt", 0).unwrap();
        fs.close(fh);

        let mut buf = [0u8; 10];
        assert_eq!(fs.read(fh, &mut buf), 0); // Geschlossen -> 0
    }

    #[test]
    fn test_overwrite_file_preserves_owner() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        fs.write_file(&caps, "/atc/file.txt", b"v1", pid(1)).unwrap();
        // pid(0) ueberschreibt
        fs.write_file(&caps, "/atc/file.txt", b"v2", pid(0)).unwrap();

        let (_, node) = fs.read_file(&caps, "/atc/file.txt", pid(1)).unwrap();
        assert_eq!(node.owner, pid(1)); // Original owner
        assert_eq!(node.size, 2); // Neue Groesse
    }

    #[test]
    fn test_nested_directories() {
        let mut fs = AtcFileSystem::new();
        let caps = CapabilityTable::new();

        fs.write_file(&caps, "/home/alice/docs/research/notes.txt", b"notes", pid(1)).unwrap();
        assert!(fs.exists("/home/alice/docs/research/notes.txt"));
        assert!(fs.exists("/home/alice/docs/research"));
        assert!(fs.exists("/home/alice/docs"));
    }
}
