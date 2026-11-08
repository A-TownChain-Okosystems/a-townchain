//! Capability-neutral VFS file and inode primitives.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InodeId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileMode {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl FileMode {
    pub const READ_ONLY: Self = Self {
        read: true,
        write: false,
        execute: false,
    };
    pub const READ_WRITE: Self = Self {
        read: true,
        write: true,
        execute: false,
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inode {
    pub id: InodeId,
    pub file_type: FileType,
    pub mode: FileMode,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenFlags {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    Create,
}

impl OpenFlags {
    pub const fn readable(self) -> bool {
        matches!(self, Self::ReadOnly | Self::ReadWrite | Self::Create)
    }
    pub const fn writable(self) -> bool {
        matches!(self, Self::WriteOnly | Self::ReadWrite | Self::Create)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileHandle {
    pub inode: InodeId,
    pub offset: u64,
    pub flags: OpenFlags,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileError {
    PermissionDenied,
    InvalidOffset,
    Overflow,
    NotDirectory,
    NotFile,
    NotFound,
}

impl FileHandle {
    pub fn advance(&mut self, amount: usize) -> Result<u64, FileError> {
        self.offset = self
            .offset
            .checked_add(amount as u64)
            .ok_or(FileError::Overflow)?;
        Ok(self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn handle_advances_deterministically() {
        let mut h = FileHandle {
            inode: InodeId(1),
            offset: 0,
            flags: OpenFlags::ReadOnly,
        };
        assert_eq!(h.advance(8), Ok(8));
    }
    #[test]
    fn flags_expose_access() {
        assert!(OpenFlags::ReadWrite.readable());
        assert!(OpenFlags::ReadWrite.writable());
        assert!(!OpenFlags::ReadOnly.writable());
    }
}
