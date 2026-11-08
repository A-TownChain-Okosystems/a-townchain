//! Filesystem-neutral media scanning and change-watch contracts.

use crate::library::{MediaMetadata, MimeType, mime_from_extension};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanChange {
    Added,
    Modified,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileRecord {
    pub uri: String,
    pub size_bytes: u64,
    pub modified_unix_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanEvent {
    pub change: ScanChange,
    pub record: FileRecord,
}

pub trait MediaFileSource {
    fn list_files(&mut self, root: &str) -> Result<Vec<FileRecord>, String>;
}
pub trait MediaWatchSource {
    fn poll(&mut self) -> Result<Vec<ScanEvent>, String>;
}

#[derive(Debug, Default)]
pub struct MediaScanner;

impl MediaScanner {
    pub fn new() -> Self {
        Self
    }

    fn stable_id(uri: &str) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for byte in uri.as_bytes() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }

    pub fn metadata_for(&mut self, file: &FileRecord) -> Option<MediaMetadata> {
        let mime = mime_from_extension(&file.uri);
        if mime == MimeType::Unknown {
            return None;
        }
        let title = file.uri.rsplit('/').next().unwrap_or(&file.uri).to_owned();
        Some(MediaMetadata {
            id: Self::stable_id(&file.uri),
            uri: file.uri.clone(),
            title,
            mime,
            size_bytes: file.size_bytes,
            modified_unix_ms: file.modified_unix_ms,
            content_hash: None,
            duration_ms: None,
            width: None,
            height: None,
        })
    }

    pub fn scan<S: MediaFileSource>(
        &mut self,
        source: &mut S,
        root: &str,
    ) -> Result<Vec<MediaMetadata>, String> {
        Ok(source
            .list_files(root)?
            .iter()
            .filter_map(|f| self.metadata_for(f))
            .collect())
    }
}
