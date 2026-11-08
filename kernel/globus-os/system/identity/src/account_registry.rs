//! Persistent username-to-account index for the GlobusOS identity boundary.
//!
//! This index provides durable, fail-closed username uniqueness. It deliberately stores no
//! password, recovery phrase, wallet secret, or Aurora node key. Account-record persistence
//! must be coordinated by the caller; this index alone is not a transaction across stores.
//!
//! The lock file is acquired with create_new, so concurrent writers cannot both commit. A
//! process crash can leave a stale lock; this backend fails closed in that case and requires
//! explicit recovery rather than silently removing a lock that may belong to a live writer.

use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{AccountUsername, UserId};

const MAGIC: &[u8; 4] = b"GUA1";
const VERSION: u8 = 1;
const MAX_ENTRIES: usize = 1_000_000;
const MAX_FIELD: usize = 128;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccountDirectoryError {
    AlreadyExists,
    NotFound,
    Busy,
    Corrupt,
    InvalidAccountId,
    Io,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountDirectoryEntry {
    pub username: AccountUsername,
    pub user_id: UserId,
}

/// A durable directory backend. Implementations must make insert_if_absent atomic and durable.
pub trait AccountDirectoryStore {
    fn insert_if_absent(
        &mut self,
        username: AccountUsername,
        user_id: UserId,
    ) -> Result<(), AccountDirectoryError>;
    fn lookup(&self, username: &AccountUsername) -> Result<UserId, AccountDirectoryError>;
    fn entries(&self) -> Result<Vec<AccountDirectoryEntry>, AccountDirectoryError>;
}

/// Filesystem-backed username index. Use a path under a trusted, access-controlled data directory.
pub struct FileAccountDirectory {
    path: PathBuf,
    lock_path: PathBuf,
}

impl FileAccountDirectory {
    pub fn open(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let mut lock_name = path.as_os_str().to_os_string();
        lock_name.push(".lock");
        Self {
            path,
            lock_path: PathBuf::from(lock_name),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn acquire_lock(&self) -> Result<DirectoryLock, AccountDirectoryError> {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lock_path)
        {
            Ok(mut file) => {
                // A diagnostic marker only; lock ownership is established by create_new.
                let _ = writeln!(file, "pid={}", std::process::id());
                let _ = file.sync_all();
                Ok(DirectoryLock {
                    path: self.lock_path.clone(),
                })
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                Err(AccountDirectoryError::Busy)
            }
            Err(_) => Err(AccountDirectoryError::Io),
        }
    }

    fn read_entries(&self) -> Result<Vec<AccountDirectoryEntry>, AccountDirectoryError> {
        let mut file = match File::open(&self.path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(_) => return Err(AccountDirectoryError::Io),
        };
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|_| AccountDirectoryError::Io)?;
        decode_entries(&bytes)
    }

    fn write_entries(
        &self,
        entries: &[AccountDirectoryEntry],
    ) -> Result<(), AccountDirectoryError> {
        let parent = self
            .path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        fs::create_dir_all(parent).map_err(|_| AccountDirectoryError::Io)?;
        let bytes = encode_entries(entries)?;
        let mut tmp_name = self.path.as_os_str().to_os_string();
        tmp_name.push(format!(".tmp.{}", std::process::id()));
        let tmp_path = PathBuf::from(tmp_name);

        // Never follow or overwrite a pre-existing temporary path.
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&tmp_path)
            .map_err(|_| AccountDirectoryError::Io)?;
        let write_result = (|| {
            file.write_all(&bytes)
                .map_err(|_| AccountDirectoryError::Io)?;
            file.sync_all().map_err(|_| AccountDirectoryError::Io)?;
            fs::rename(&tmp_path, &self.path).map_err(|_| AccountDirectoryError::Io)?;
            File::open(parent)
                .and_then(|dir| dir.sync_all())
                .map_err(|_| AccountDirectoryError::Io)?;
            Ok(())
        })();
        if write_result.is_err() {
            let _ = fs::remove_file(&tmp_path);
        }
        write_result
    }
}

struct DirectoryLock {
    path: PathBuf,
}
impl Drop for DirectoryLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

impl AccountDirectoryStore for FileAccountDirectory {
    fn insert_if_absent(
        &mut self,
        username: AccountUsername,
        user_id: UserId,
    ) -> Result<(), AccountDirectoryError> {
        if user_id.as_str().is_empty()
            || user_id.as_str().len() > MAX_FIELD
            || user_id.as_str().as_bytes().contains(&0)
        {
            return Err(AccountDirectoryError::InvalidAccountId);
        }
        let _lock = self.acquire_lock()?;
        let mut entries = self.read_entries()?;
        if entries.iter().any(|entry| entry.username == username) {
            return Err(AccountDirectoryError::AlreadyExists);
        }
        if entries.iter().any(|entry| entry.user_id == user_id) {
            return Err(AccountDirectoryError::AlreadyExists);
        }
        entries.push(AccountDirectoryEntry { username, user_id });
        entries.sort_by(|a, b| a.username.as_str().cmp(b.username.as_str()));
        self.write_entries(&entries)
    }

    fn lookup(&self, username: &AccountUsername) -> Result<UserId, AccountDirectoryError> {
        self.read_entries()?
            .into_iter()
            .find(|entry| &entry.username == username)
            .map(|entry| entry.user_id)
            .ok_or(AccountDirectoryError::NotFound)
    }

    fn entries(&self) -> Result<Vec<AccountDirectoryEntry>, AccountDirectoryError> {
        self.read_entries()
    }
}

fn encode_entries(entries: &[AccountDirectoryEntry]) -> Result<Vec<u8>, AccountDirectoryError> {
    if entries.len() > MAX_ENTRIES {
        return Err(AccountDirectoryError::Corrupt);
    }
    let mut bytes = Vec::new();
    bytes.extend_from_slice(MAGIC);
    bytes.push(VERSION);
    bytes.extend_from_slice(&(entries.len() as u32).to_be_bytes());
    for entry in entries {
        let username = entry.username.as_str().as_bytes();
        let user_id = entry.user_id.as_str().as_bytes();
        if username.len() > 32
            || user_id.is_empty()
            || user_id.len() > MAX_FIELD
            || username.len() > u8::MAX as usize
        {
            return Err(AccountDirectoryError::Corrupt);
        }
        bytes.push(username.len() as u8);
        bytes.extend_from_slice(username);
        bytes.extend_from_slice(&(user_id.len() as u16).to_be_bytes());
        bytes.extend_from_slice(user_id);
    }
    Ok(bytes)
}

fn decode_entries(bytes: &[u8]) -> Result<Vec<AccountDirectoryEntry>, AccountDirectoryError> {
    if bytes.len() < 9 || &bytes[..4] != MAGIC || bytes[4] != VERSION {
        return Err(AccountDirectoryError::Corrupt);
    }
    let count = u32::from_be_bytes(
        bytes[5..9]
            .try_into()
            .map_err(|_| AccountDirectoryError::Corrupt)?,
    ) as usize;
    if count > MAX_ENTRIES {
        return Err(AccountDirectoryError::Corrupt);
    }
    let mut cursor = 9usize;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let name_len = *bytes.get(cursor).ok_or(AccountDirectoryError::Corrupt)? as usize;
        cursor += 1;
        let name_end = cursor
            .checked_add(name_len)
            .ok_or(AccountDirectoryError::Corrupt)?;
        let name = std::str::from_utf8(
            bytes
                .get(cursor..name_end)
                .ok_or(AccountDirectoryError::Corrupt)?,
        )
        .map_err(|_| AccountDirectoryError::Corrupt)?;
        cursor = name_end;
        let id_len_end = cursor
            .checked_add(2)
            .ok_or(AccountDirectoryError::Corrupt)?;
        let id_len = u16::from_be_bytes(
            bytes
                .get(cursor..id_len_end)
                .ok_or(AccountDirectoryError::Corrupt)?
                .try_into()
                .map_err(|_| AccountDirectoryError::Corrupt)?,
        ) as usize;
        cursor = id_len_end;
        if id_len == 0 || id_len > MAX_FIELD {
            return Err(AccountDirectoryError::Corrupt);
        }
        let id_end = cursor
            .checked_add(id_len)
            .ok_or(AccountDirectoryError::Corrupt)?;
        let id = std::str::from_utf8(
            bytes
                .get(cursor..id_end)
                .ok_or(AccountDirectoryError::Corrupt)?,
        )
        .map_err(|_| AccountDirectoryError::Corrupt)?;
        cursor = id_end;
        if id.as_bytes().contains(&0) {
            return Err(AccountDirectoryError::Corrupt);
        }
        let username = AccountUsername::parse(name).map_err(|_| AccountDirectoryError::Corrupt)?;
        let user_id = UserId::new(id.to_owned()).map_err(|_| AccountDirectoryError::Corrupt)?;
        if entries
            .iter()
            .any(|e: &AccountDirectoryEntry| e.username == username || e.user_id == user_id)
        {
            return Err(AccountDirectoryError::Corrupt);
        }
        entries.push(AccountDirectoryEntry { username, user_id });
    }
    if cursor != bytes.len() {
        return Err(AccountDirectoryError::Corrupt);
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT: AtomicU64 = AtomicU64::new(0);

    fn temp_path() -> PathBuf {
        std::env::temp_dir().join(format!(
            "globus-account-directory-{}-{}.bin",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ))
    }

    #[test]
    fn registration_is_unique_and_survives_reopen() {
        let path = temp_path();
        let username = AccountUsername::parse("Alice").unwrap();
        let user_id = UserId::new("user-alice").unwrap();
        {
            let mut directory = FileAccountDirectory::open(&path);
            directory
                .insert_if_absent(username.clone(), user_id.clone())
                .unwrap();
            assert_eq!(
                directory.insert_if_absent(username.clone(), UserId::new("other").unwrap()),
                Err(AccountDirectoryError::AlreadyExists)
            );
        }
        let reopened = FileAccountDirectory::open(&path);
        assert_eq!(reopened.lookup(&username).unwrap(), user_id);
        let _ = fs::remove_file(&path);
        let mut lock = path.as_os_str().to_os_string();
        lock.push(".lock");
        let _ = fs::remove_file(PathBuf::from(lock));
    }

    #[test]
    fn concurrent_registration_cannot_create_duplicate_username() {
        use std::sync::{Arc, Barrier};
        use std::{
            thread,
            time::{Duration, Instant},
        };

        let path = temp_path();
        let barrier = Arc::new(Barrier::new(2));
        let mut workers = Vec::new();
        for suffix in ["one", "two"] {
            let worker_path = path.clone();
            let worker_barrier = Arc::clone(&barrier);
            workers.push(thread::spawn(move || {
                let username = AccountUsername::parse("ConcurrentUser").unwrap();
                let user_id = UserId::new(format!("user-{suffix}")).unwrap();
                let mut directory = FileAccountDirectory::open(worker_path);
                worker_barrier.wait();
                // Filesystem sync/rename latency varies substantially across CI runners.
                // Retry for a bounded wall-clock interval instead of a brittle 200 ms budget.
                let deadline = Instant::now() + Duration::from_secs(10);
                loop {
                    match directory.insert_if_absent(username.clone(), user_id.clone()) {
                        Ok(()) => return true,
                        Err(AccountDirectoryError::Busy) if Instant::now() < deadline => {
                            thread::sleep(Duration::from_millis(1));
                        }
                        Err(AccountDirectoryError::Busy) => {
                            panic!("directory lock did not become available within 10 seconds");
                        }
                        Err(AccountDirectoryError::AlreadyExists) => return false,
                        Err(error) => panic!("unexpected directory error: {error:?}"),
                    }
                }
            }));
        }
        let winners = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .filter(|won| *won)
            .count();
        assert_eq!(winners, 1);
        let directory = FileAccountDirectory::open(&path);
        assert_eq!(directory.entries().unwrap().len(), 1);
        let _ = fs::remove_file(&path);
        let mut lock = path.as_os_str().to_os_string();
        lock.push(".lock");
        let _ = fs::remove_file(PathBuf::from(lock));
    }

    #[test]
    fn corrupt_file_fails_closed() {
        let path = temp_path();
        fs::write(&path, b"not-an-account-index").unwrap();
        let directory = FileAccountDirectory::open(&path);
        assert_eq!(directory.entries(), Err(AccountDirectoryError::Corrupt));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn malformed_and_duplicate_entries_are_rejected() {
        assert_eq!(decode_entries(b"GUA1\x01\0\0\0\0"), Ok(Vec::new()));
        assert_eq!(
            decode_entries(b"GUA1\x02\0\0\0\0"),
            Err(AccountDirectoryError::Corrupt)
        );
    }
}
