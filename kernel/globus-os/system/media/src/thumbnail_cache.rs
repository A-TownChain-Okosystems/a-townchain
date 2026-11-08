//! Bounded LRU thumbnail cache with file-metadata identity.

use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThumbnailSize {
    Small,
    Medium,
    Large,
}

impl ThumbnailSize {
    pub const fn dimensions(self) -> (u32, u32) {
        match self {
            Self::Small => (128, 128),
            Self::Medium => (256, 256),
            Self::Large => (512, 512),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ThumbnailKey {
    pub uri: String,
    pub modified_ns: u64,
    pub size_bytes: u64,
    pub content_fingerprint: Option<[u8; 32]>,
    pub size: ThumbnailSize,
}

#[derive(Debug)]
pub struct ThumbnailCache {
    capacity: usize,
    byte_limit: usize,
    bytes: usize,
    map: HashMap<ThumbnailKey, Vec<u8>>,
    order: VecDeque<ThumbnailKey>,
}

impl ThumbnailCache {
    pub fn new(capacity: usize) -> Self {
        Self::with_limits(capacity, usize::MAX)
    }
    pub fn with_limits(capacity: usize, byte_limit: usize) -> Self {
        Self {
            capacity,
            byte_limit,
            bytes: 0,
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Counterpart to `len`.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn bytes(&self) -> usize {
        self.bytes
    }
    pub fn insert(&mut self, key: ThumbnailKey, data: Vec<u8>) {
        if self.capacity == 0 || data.len() > self.byte_limit {
            return;
        }
        if let Some(old) = self.map.remove(&key) {
            self.bytes = self.bytes.saturating_sub(old.len());
            self.order.retain(|k| k != &key);
        }
        self.bytes = self.bytes.saturating_add(data.len());
        self.map.insert(key.clone(), data);
        self.order.push_back(key);
        while self.order.len() > self.capacity || self.bytes > self.byte_limit {
            if let Some(old) = self.order.pop_front() {
                if let Some(data) = self.map.remove(&old) {
                    self.bytes = self.bytes.saturating_sub(data.len());
                }
            } else {
                break;
            }
        }
    }
    pub fn get(&mut self, key: &ThumbnailKey) -> Option<&[u8]> {
        if self.map.contains_key(key) {
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
        }
        self.map.get(key).map(Vec::as_slice)
    }
    pub fn invalidate_uri(&mut self, uri: &str) {
        let keys: Vec<_> = self
            .order
            .iter()
            .filter(|k| k.uri == uri)
            .cloned()
            .collect();
        for k in keys {
            if let Some(data) = self.map.remove(&k) {
                self.bytes = self.bytes.saturating_sub(data.len());
            }
        }
        self.order.retain(|k| k.uri != uri);
    }
    pub fn clear(&mut self) {
        self.map.clear();
        self.order.clear();
        self.bytes = 0;
    }
}
