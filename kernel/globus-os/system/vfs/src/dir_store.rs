//! Persistent root-directory store with deterministic entry encoding.

use crate::{DirectoryRecord, Extent, ExtentError, ExtentMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirStoreError {
    InvalidEntry,
    Duplicate,
    NotFound,
    Full,
    Extent(ExtentError),
}
impl From<ExtentError> for DirStoreError {
    fn from(value: ExtentError) -> Self {
        Self::Extent(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryStore {
    entries: Vec<DirectoryRecord>,
    capacity: usize,
}

impl DirectoryStore {
    pub fn new(capacity: usize) -> Result<Self, DirStoreError> {
        if capacity == 0 {
            return Err(DirStoreError::Full);
        }
        Ok(Self {
            entries: Vec::new(),
            capacity,
        })
    }
    pub fn entries(&self) -> &[DirectoryRecord] {
        &self.entries
    }
    pub fn lookup(&self, name: &str) -> Result<u64, DirStoreError> {
        self.entries
            .iter()
            .find(|e| e.name == name)
            .map(|e| e.inode)
            .ok_or(DirStoreError::NotFound)
    }
    pub fn insert(&mut self, record: DirectoryRecord) -> Result<(), DirStoreError> {
        if record.inode == 0 || record.name.is_empty() {
            return Err(DirStoreError::InvalidEntry);
        }
        if self.entries.iter().any(|e| e.name == record.name) {
            return Err(DirStoreError::Duplicate);
        }
        if self.entries.len() == self.capacity {
            return Err(DirStoreError::Full);
        }
        self.entries.push(record);
        self.entries.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(())
    }
    pub fn remove(&mut self, name: &str) -> Result<DirectoryRecord, DirStoreError> {
        let i = self
            .entries
            .iter()
            .position(|e| e.name == name)
            .ok_or(DirStoreError::NotFound)?;
        Ok(self.entries.remove(i))
    }
    pub fn encoded_len(&self) -> Result<usize, DirStoreError> {
        self.entries.iter().try_fold(0usize, |n, e| {
            n.checked_add(e.encoded_len().map_err(|_| DirStoreError::InvalidEntry)?)
                .ok_or(DirStoreError::InvalidEntry)
        })
    }
    pub fn encode(&self, out: &mut [u8]) -> Result<usize, DirStoreError> {
        let need = self.encoded_len()?;
        if out.len() < need {
            return Err(DirStoreError::Full);
        };
        let mut off = 0;
        for e in &self.entries {
            off += e
                .encode(&mut out[off..])
                .map_err(|_| DirStoreError::InvalidEntry)?;
        }
        Ok(off)
    }
    pub fn decode(input: &[u8], capacity: usize) -> Result<Self, DirStoreError> {
        let mut s = Self::new(capacity)?;
        let mut off = 0;
        while off < input.len() {
            if input[off..].iter().all(|b| *b == 0) {
                break;
            }
            let (e, n) =
                DirectoryRecord::decode(&input[off..]).map_err(|_| DirStoreError::InvalidEntry)?;
            s.insert(e)?;
            off += n;
        }
        Ok(s)
    }
    pub fn extent_for_bytes(
        &self,
        physical_start: u64,
        block_size: u64,
    ) -> Result<ExtentMap, DirStoreError> {
        let bytes = self
            .encoded_len()
            .map_err(|_| DirStoreError::InvalidEntry)? as u64;
        let blocks = if bytes == 0 {
            0
        } else {
            bytes
                .checked_add(block_size - 1)
                .ok_or(DirStoreError::InvalidEntry)?
                / block_size
        };
        let mut m = ExtentMap::new();
        if blocks > 0 {
            m.insert(Extent {
                logical: 0,
                physical: physical_start,
                blocks,
            })?;
        }
        Ok(m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DirectoryType;
    #[test]
    fn deterministic_insert_lookup() {
        let mut d = DirectoryStore::new(4).unwrap();
        d.insert(DirectoryRecord {
            inode: 3,
            kind: DirectoryType::File,
            name: "z".into(),
        })
        .unwrap();
        d.insert(DirectoryRecord {
            inode: 2,
            kind: DirectoryType::Directory,
            name: "a".into(),
        })
        .unwrap();
        assert_eq!(d.lookup("a").unwrap(), 2);
        assert_eq!(d.entries()[0].name, "a");
    }
    #[test]
    fn duplicate_rejected() {
        let mut d = DirectoryStore::new(2).unwrap();
        let e = DirectoryRecord {
            inode: 2,
            kind: DirectoryType::File,
            name: "x".into(),
        };
        d.insert(e.clone()).unwrap();
        assert_eq!(d.insert(e), Err(DirStoreError::Duplicate));
    }
    #[test]
    fn round_trip() {
        let mut d = DirectoryStore::new(4).unwrap();
        d.insert(DirectoryRecord {
            inode: 2,
            kind: DirectoryType::File,
            name: "hello".into(),
        })
        .unwrap();
        let mut b = vec![0; 64];
        let n = d.encode(&mut b).unwrap();
        assert_eq!(DirectoryStore::decode(&b[..n], 4).unwrap(), d);
    }
}
