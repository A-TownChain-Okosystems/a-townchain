//! Virtual filesystem namespace, mount policy, and persistent-disk discovery.

pub mod allocator;
pub mod bitmap;
pub mod dir_store;
pub mod directory;
pub mod extent;
pub mod file;
pub mod file_store;
pub mod gpt;
pub mod inode;
pub mod journal;
pub mod layout;
pub mod mounts;
pub mod path;
pub mod persistent;
pub mod recovery;
pub mod transaction;
pub mod tree;

pub use allocator::{AllocationError, BlockAllocator};
pub use bitmap::{BitmapError, FreeSpaceBitmap};
pub use dir_store::{DirStoreError, DirectoryStore};
pub use directory::{DirectoryError, DirectoryRecord, DirectoryType};
pub use extent::{Extent, ExtentError, ExtentMap};
pub use file::{FileError, FileHandle, FileMode, FileType, Inode, InodeId, OpenFlags};
pub use file_store::{FileStore, FileStoreError};
pub use inode::{DiskInode, INODE_SIZE, InodeAllocator, InodeDiskError};
pub use journal::{JOURNAL_RECORD_SIZE, Journal, JournalError, JournalOp, JournalRecord};
pub use layout::{LayoutError, MetadataLayout};
pub use mounts::MountTable;
pub use path::{PathError, normalize};
pub use persistent::{FsError as PersistentFsError, PersistentFs, Superblock};
pub use recovery::{RecoveryError, ReplayWrite, replay_pending};
pub use transaction::{PendingWrite, Transaction, TransactionError};
pub use tree::{DirectoryEntry, FsError, InodeTree};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mount {
    pub mountpoint: String,
    pub filesystem: String,
    pub readonly: bool,
}

pub fn required_mounts() -> [Mount; 6] {
    [
        Mount {
            mountpoint: "/boot".into(),
            filesystem: "efi".into(),
            readonly: true,
        },
        Mount {
            mountpoint: "/system".into(),
            filesystem: "system".into(),
            readonly: true,
        },
        Mount {
            mountpoint: "/etc".into(),
            filesystem: "config".into(),
            readonly: false,
        },
        Mount {
            mountpoint: "/home".into(),
            filesystem: "data".into(),
            readonly: false,
        },
        Mount {
            mountpoint: "/var".into(),
            filesystem: "state".into(),
            readonly: false,
        },
        Mount {
            mountpoint: "/run".into(),
            filesystem: "runtime".into(),
            readonly: false,
        },
    ]
}
