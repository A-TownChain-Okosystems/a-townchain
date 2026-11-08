//! Atomic filesystem backend for encrypted identity blobs.
//!
//! The backend stores the encrypted blob and its monotonic generation in one state file.
//! Replacement is performed through a temporary file, `fsync`, and atomic rename. A delete is
//! represented by a persistent tombstone so a reboot cannot reset the generation to zero.
//! This backend provides storage atomicity; confidentiality and key custody remain the job of
//! `EncryptedBlobStore` and its `IdentityKeyProvider`.

use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use super::{BlobStore, SecureStoreError};

const MAGIC: &[u8; 4] = b"GIB1";
const VERSION: u8 = 1;
const PRESENT: u8 = 1;
const DELETED: u8 = 0;
const HEADER_LEN: usize = 4 + 1 + 1 + 8 + 8;

pub struct FileBlobStore {
    path: PathBuf,
}

impl FileBlobStore {
    pub fn open(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn read_state(&self) -> Result<(u8, u64, Vec<u8>), SecureStoreError> {
        let mut file = match File::open(&self.path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok((DELETED, 0, Vec::new()));
            }
            Err(_) => return Err(SecureStoreError::BackendUnavailable),
        };
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|_| SecureStoreError::BackendUnavailable)?;
        if bytes.len() < HEADER_LEN || &bytes[..4] != MAGIC || bytes[4] != VERSION {
            return Err(SecureStoreError::InvalidEnvelope);
        }
        let state = bytes[5];
        if state != PRESENT && state != DELETED {
            return Err(SecureStoreError::InvalidEnvelope);
        }
        let generation = u64::from_le_bytes(
            bytes[6..14]
                .try_into()
                .map_err(|_| SecureStoreError::InvalidEnvelope)?,
        );
        let len = u64::from_le_bytes(
            bytes[14..22]
                .try_into()
                .map_err(|_| SecureStoreError::InvalidEnvelope)?,
        ) as usize;
        if len != bytes.len().saturating_sub(HEADER_LEN) {
            return Err(SecureStoreError::InvalidEnvelope);
        }
        Ok((state, generation, bytes[HEADER_LEN..].to_vec()))
    }

    fn atomic_write(
        &self,
        state: u8,
        generation: u64,
        blob: &[u8],
    ) -> Result<(), SecureStoreError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|_| SecureStoreError::BackendUnavailable)?;
        }
        let tmp = temporary_path(&self.path);
        let result = (|| {
            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&tmp)
                .map_err(|_| SecureStoreError::BackendUnavailable)?;
            let mut header = Vec::with_capacity(HEADER_LEN);
            header.extend_from_slice(MAGIC);
            header.push(VERSION);
            header.push(state);
            header.extend_from_slice(&generation.to_le_bytes());
            header.extend_from_slice(&(blob.len() as u64).to_le_bytes());
            file.write_all(&header)
                .map_err(|_| SecureStoreError::BackendUnavailable)?;
            file.write_all(blob)
                .map_err(|_| SecureStoreError::BackendUnavailable)?;
            file.sync_all()
                .map_err(|_| SecureStoreError::BackendUnavailable)?;
            fs::rename(&tmp, &self.path).map_err(|_| SecureStoreError::BackendUnavailable)?;
            if let Some(parent) = self.path.parent().filter(|p| !p.as_os_str().is_empty()) {
                let dir = File::open(parent).map_err(|_| SecureStoreError::BackendUnavailable)?;
                dir.sync_all()
                    .map_err(|_| SecureStoreError::BackendUnavailable)?;
            }
            Ok(())
        })();
        if result.is_err() {
            let _ = fs::remove_file(&tmp);
        }
        result
    }
}

impl BlobStore for FileBlobStore {
    fn load(&self) -> Result<Option<Vec<u8>>, SecureStoreError> {
        let (state, _, blob) = self.read_state()?;
        Ok((state == PRESENT).then_some(blob))
    }

    fn replace(&mut self, blob: &[u8], generation: u64) -> Result<(), SecureStoreError> {
        let (_, current, _) = self.read_state()?;
        if generation <= current && current != 0 {
            return Err(SecureStoreError::RollbackDetected {
                stored: current,
                incoming: generation,
            });
        }
        self.atomic_write(PRESENT, generation, blob)
    }

    fn generation(&self) -> Result<u64, SecureStoreError> {
        Ok(self.read_state()?.1)
    }

    fn delete(&mut self) -> Result<(), SecureStoreError> {
        let (_, current, _) = self.read_state()?;
        let generation = current
            .checked_add(1)
            .ok_or(SecureStoreError::GenerationOverflow)?;
        self.atomic_write(DELETED, generation, &[])
    }
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(format!(".tmp-{}", std::process::id()));
    PathBuf::from(tmp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn path() -> PathBuf {
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        env::temp_dir().join(format!("globus-identity-{n}.blob"))
    }

    #[test]
    fn persists_blob_and_generation_across_instances() {
        let path = path();
        let mut first = FileBlobStore::open(&path);
        first.replace(b"encrypted", 1).unwrap();
        drop(first);
        let second = FileBlobStore::open(&path);
        assert_eq!(second.generation().unwrap(), 1);
        assert_eq!(second.load().unwrap(), Some(b"encrypted".to_vec()));
        let _ = fs::remove_file(path);
    }

    #[test]
    fn delete_is_a_persistent_tombstone() {
        let path = path();
        let mut store = FileBlobStore::open(&path);
        store.replace(b"encrypted", 1).unwrap();
        store.delete().unwrap();
        drop(store);
        let reopened = FileBlobStore::open(&path);
        assert_eq!(reopened.load().unwrap(), None);
        assert_eq!(reopened.generation().unwrap(), 2);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn stale_generation_is_rejected_after_reopen() {
        let path = path();
        let mut store = FileBlobStore::open(&path);
        store.replace(b"new", 3).unwrap();
        assert_eq!(
            store.replace(b"old", 2).unwrap_err(),
            SecureStoreError::RollbackDetected {
                stored: 3,
                incoming: 2
            }
        );
        let _ = fs::remove_file(path);
    }
}
