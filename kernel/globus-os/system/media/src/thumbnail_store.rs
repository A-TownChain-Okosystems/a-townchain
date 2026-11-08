//! Persistence boundary for thumbnails. Storage is intentionally injected so the media crate
//! does not assume a filesystem format or backend.

use crate::thumbnail_cache::{ThumbnailKey, ThumbnailSize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThumbnailDimensions {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredThumbnail {
    pub key: ThumbnailKey,
    pub dimensions: ThumbnailDimensions,
    pub data: Vec<u8>,
}

pub trait ThumbnailStore {
    fn load(&self, key: &ThumbnailKey) -> Option<StoredThumbnail>;
    fn save(&mut self, thumbnail: StoredThumbnail) -> Result<(), String>;
    fn remove(&mut self, key: &ThumbnailKey) -> Result<(), String>;
    fn clear_uri(&mut self, uri: &str) -> Result<(), String>;
}

#[derive(Debug, Default)]
pub struct MemoryThumbnailStore {
    entries: Vec<StoredThumbnail>,
}

impl MemoryThumbnailStore {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Counterpart to `len`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn contains(&self, uri: &str, size: ThumbnailSize) -> bool {
        self.entries
            .iter()
            .any(|x| x.key.uri == uri && x.key.size == size)
    }
}

impl ThumbnailStore for MemoryThumbnailStore {
    fn load(&self, key: &ThumbnailKey) -> Option<StoredThumbnail> {
        self.entries.iter().find(|x| &x.key == key).cloned()
    }
    fn save(&mut self, thumbnail: StoredThumbnail) -> Result<(), String> {
        self.entries.retain(|x| x.key != thumbnail.key);
        self.entries.push(thumbnail);
        Ok(())
    }
    fn remove(&mut self, key: &ThumbnailKey) -> Result<(), String> {
        self.entries.retain(|x| &x.key != key);
        Ok(())
    }
    fn clear_uri(&mut self, uri: &str) -> Result<(), String> {
        self.entries.retain(|x| x.key.uri != uri);
        Ok(())
    }
}
