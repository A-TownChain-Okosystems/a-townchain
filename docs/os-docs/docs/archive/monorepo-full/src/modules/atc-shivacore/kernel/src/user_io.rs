// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ShivaCore — K-Sprint 34: File Descriptor Table + User I/O
// Copyright (c) Michael Wroblewski. All rights reserved.
//
// Per-Process File Descriptor Table, Stdio (stdin/stdout/stderr),
// Anonymous Pipes für IPC, Poll/Select, User I/O Manager.

use crate::ats1000::Pid;

// ═══════════════════════════════════════════════════════════════════════════════
// File Descriptor Types
// ═══════════════════════════════════════════════════════════════════════════════

/// Maximum FDs per process
pub const MAX_FDS: usize = 256;

/// Standard file descriptors
pub const STDIN_FD:  u32 = 0;
pub const STDOUT_FD: u32 = 1;
pub const STDERR_FD: u32 = 2;

/// File descriptor flags
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FdFlags {
    pub read:     bool,
    pub write:    bool,
    pub append:   bool,
    pub nonblock:  bool,
    pub cloexec:   bool,  // Close on exec
}

impl FdFlags {
    pub const fn read_only()        -> Self { Self { read: true,  write: false, append: false, nonblock: false, cloexec: false } }
    pub const fn write_only()       -> Self { Self { read: false, write: true,  append: false, nonblock: false, cloexec: false } }
    pub const fn read_write()       -> Self { Self { read: true,  write: true,  append: false, nonblock: false, cloexec: false } }
    pub const fn read_write_nb()    -> Self { Self { read: true,  write: true,  append: false, nonblock: true,  cloexec: false } }
}

impl Default for FdFlags {
    fn default() -> Self { Self::read_write() }
}

/// What kind of thing a file descriptor points to
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FdTarget {
    /// A VFS file (path-based)
    File { path: String, inode: u64 },
    /// A pipe (read end or write end)
    PipeRead { pipe_id: u64 },
    PipeWrite { pipe_id: u64 },
    /// Standard I/O (stdin/stdout/stderr)
    Stdin,
    Stdout,
    Stderr,
    /// A socket (future)
    Socket { socket_id: u64 },
    /// Anonymous (e.g. /dev/null)
    Null,
}

/// A single file descriptor entry
#[derive(Clone, Debug)]
pub struct FileDescriptor {
    pub fd:       u32,
    pub target:   FdTarget,
    pub flags:    FdFlags,
    pub offset:   u64,
    pub closed:   bool,
}

impl FileDescriptor {
    pub fn new(fd: u32, target: FdTarget, flags: FdFlags) -> Self {
        Self { fd, target, flags, offset: 0, closed: false }
    }

    pub fn is_readable(&self) -> bool { self.flags.read && !self.closed }
    pub fn is_writable(&self) -> bool { self.flags.write && !self.closed }
    pub fn is_closed(&self) -> bool { self.closed }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Per-Process File Descriptor Table
// ═══════════════════════════════════════════════════════════════════════════════

/// A file descriptor table for one process
#[derive(Clone, Debug)]
pub struct FdTable {
    pub pid:    Pid,
    pub fds:    Vec<Option<FileDescriptor>>,
    pub next_fd: u32,
}

impl FdTable {
    pub fn new(pid: Pid) -> Self {
        let mut fds = vec![None; MAX_FDS];

        // Pre-allocate stdin/stdout/stderr
        fds[0] = Some(FileDescriptor::new(STDIN_FD,  FdTarget::Stdin,  FdFlags::read_only()));
        fds[1] = Some(FileDescriptor::new(STDOUT_FD, FdTarget::Stdout, FdFlags::write_only()));
        fds[2] = Some(FileDescriptor::new(STDERR_FD, FdTarget::Stderr, FdFlags::write_only()));

        Self { pid, fds, next_fd: 3 }
    }

    /// Allocate a new FD
    pub fn alloc(&mut self, target: FdTarget, flags: FdFlags) -> Option<u32> {
        // Find a free slot
        for i in 3..MAX_FDS {
            if self.fds[i].is_none() || self.fds[i].as_ref().unwrap().is_closed() {
                let fd = i as u32;
                self.fds[i] = Some(FileDescriptor::new(fd, target, flags));
                if fd >= self.next_fd { self.next_fd = fd + 1; }
                return Some(fd);
            }
        }
        None
    }

    /// Get an FD entry
    pub fn get(&self, fd: u32) -> Option<&FileDescriptor> {
        let idx = fd as usize;
        if idx < MAX_FDS {
            self.fds[idx].as_ref().filter(|fd| !fd.is_closed())
        } else {
            None
        }
    }

    /// Get a mutable FD entry
    pub fn get_mut(&mut self, fd: u32) -> Option<&mut FileDescriptor> {
        let idx = fd as usize;
        if idx < MAX_FDS {
            self.fds[idx].as_mut().filter(|fd| !fd.is_closed())
        } else {
            None
        }
    }

    /// Close an FD
    pub fn close(&mut self, fd: u32) -> bool {
        let idx = fd as usize;
        if idx < MAX_FDS {
            if let Some(ref mut entry) = self.fds[idx] {
                if entry.is_closed() { return false; }
                entry.closed = true;
                return true;
            }
        }
        false
    }

    /// Close all FDs (on process exit)
    pub fn close_all(&mut self) -> usize {
        let mut count = 0;
        for fd in &mut self.fds {
            if let Some(ref mut entry) = fd {
                if !entry.is_closed() {
                    entry.closed = true;
                    count += 1;
                }
            }
        }
        count
    }

    /// Duplicate an FD (dup/dup2)
    pub fn dup(&mut self, fd: u32) -> Option<u32> {
        let entry = self.get(fd)?.clone();
        self.alloc(entry.target, entry.flags)
    }

    /// Duplicate FD to a specific target (dup2)
    pub fn dup2(&mut self, old_fd: u32, new_fd: u32) -> bool {
        let entry = match self.get(old_fd) {
            Some(e) => e.clone(),
            None => return false,
        };
        let idx = new_fd as usize;
        if idx >= MAX_FDS { return false; }
        // Close existing FD at new_fd if open
        if self.fds[idx].is_some() {
            self.close(new_fd);
        }
        self.fds[idx] = Some(FileDescriptor::new(new_fd, entry.target, entry.flags));
        true
    }

    /// Count open FDs
    pub fn open_count(&self) -> usize {
        self.fds.iter()
            .filter(|f| f.as_ref().map(|fd| !fd.is_closed()).unwrap_or(false))
            .count()
    }

    /// List all open FDs
    pub fn list_fds(&self) -> Vec<u32> {
        self.fds.iter().enumerate()
            .filter(|(_, f)| f.as_ref().map(|fd| !fd.is_closed()).unwrap_or(false))
            .map(|(i, _)| i as u32)
            .collect()
    }

    /// Check if FD is valid and has the right permissions
    pub fn check_access(&self, fd: u32, want_write: bool) -> bool {
        match self.get(fd) {
            Some(entry) => {
                if want_write { entry.is_writable() }
                else { entry.is_readable() }
            }
            None => false,
        }
    }

    /// Update offset (seek)
    pub fn seek(&mut self, fd: u32, offset: u64) -> bool {
        match self.get_mut(fd) {
            Some(entry) => { entry.offset = offset; true }
            None => false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Anonymous Pipe (for IPC between user processes)
// ═══════════════════════════════════════════════════════════════════════════════

const PIPE_BUF_SIZE: usize = 65536; // 64 KiB

/// A pipe for one-directional IPC
pub struct Pipe {
    pub id:         u64,
    pub buffer:     Vec<u8>,
    pub read_end_open:  bool,
    pub write_end_open: bool,
    pub total_read:    u64,
    pub total_written: u64,
}

impl Pipe {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            buffer: Vec::with_capacity(PIPE_BUF_SIZE),
            read_end_open: true,
            write_end_open: true,
            total_read: 0,
            total_written: 0,
        }
    }

    /// Write data to the pipe. Returns bytes written.
    pub fn write(&mut self, data: &[u8]) -> usize {
        if !self.read_end_open {
            return 0; // SIGPIPE scenario
        }
        let space = PIPE_BUF_SIZE.saturating_sub(self.buffer.len());
        let to_write = data.len().min(space);
        self.buffer.extend_from_slice(&data[..to_write]);
        self.total_written += to_write as u64;
        to_write
    }

    /// Read data from the pipe. Returns (bytes_read, eof).
    pub fn read(&mut self, buf: &mut [u8]) -> (usize, bool) {
        if self.buffer.is_empty() {
            return (0, !self.write_end_open); // EOF if write end closed
        }
        let to_read = buf.len().min(self.buffer.len());
        buf[..to_read].copy_from_slice(&self.buffer[..to_read]);
        self.buffer.drain(..to_read);
        self.total_read += to_read as u64;
        (to_read, false)
    }

    pub fn available(&self) -> usize { self.buffer.len() }
    pub fn is_full(&self) -> bool { self.buffer.len() >= PIPE_BUF_SIZE }
    pub fn is_empty(&self) -> bool { self.buffer.is_empty() }

    pub fn close_read(&mut self) { self.read_end_open = false; }
    pub fn close_write(&mut self) { self.write_end_open = false; }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Pipe Manager
// ═══════════════════════════════════════════════════════════════════════════════

pub struct PipeManager {
    pipes:       Vec<Pipe>,
    next_id:     u64,
}

impl Default for PipeManager {
    fn default() -> Self { Self::new() }
}

impl PipeManager {
    pub fn new() -> Self {
        Self { pipes: Vec::new(), next_id: 1 }
    }

    /// Create a new pipe. Returns (pipe_id, read_fd_target, write_fd_target)
    pub fn create_pipe(&mut self) -> (u64, FdTarget, FdTarget) {
        let id = self.next_id;
        self.next_id += 1;
        self.pipes.push(Pipe::new(id));
        (id, FdTarget::PipeRead { pipe_id: id }, FdTarget::PipeWrite { pipe_id: id })
    }

    pub fn get_pipe(&self, id: u64) -> Option<&Pipe> {
        self.pipes.iter().find(|p| p.id == id)
    }

    pub fn get_pipe_mut(&mut self, id: u64) -> Option<&mut Pipe> {
        self.pipes.iter_mut().find(|p| p.id == id)
    }

    /// Write to a pipe by ID
    pub fn write(&mut self, pipe_id: u64, data: &[u8]) -> Option<usize> {
        self.get_pipe_mut(pipe_id).map(|p| p.write(data))
    }

    /// Read from a pipe by ID
    pub fn read(&mut self, pipe_id: u64, buf: &mut [u8]) -> Option<(usize, bool)> {
        self.get_pipe_mut(pipe_id).map(|p| p.read(buf))
    }

    /// Close one end of a pipe
    pub fn close_end(&mut self, pipe_id: u64, is_write: bool) {
        if let Some(pipe) = self.get_pipe_mut(pipe_id) {
            if is_write { pipe.close_write(); }
            else { pipe.close_read(); }
        }
    }

    /// Remove fully closed pipes (cleanup)
    pub fn cleanup(&mut self) -> usize {
        let before = self.pipes.len();
        self.pipes.retain(|p| p.read_end_open || p.write_end_open || !p.is_empty());
        before - self.pipes.len()
    }

    pub fn pipe_count(&self) -> usize { self.pipes.len() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Poll / Select (wait for I/O readiness)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PollEvent {
    Readable,
    Writable,
    Error,
    Hangup,
}

#[derive(Clone, Debug)]
pub struct PollEntry {
    pub fd:     u32,
    pub events: Vec<PollEvent>,
    pub revents: Vec<PollEvent>,
}

impl PollEntry {
    pub fn new(fd: u32, events: Vec<PollEvent>) -> Self {
        Self { fd, events, revents: vec![] }
    }
}

/// Check I/O readiness for a set of FDs
pub fn poll(table: &FdTable, pipe_mgr: &PipeManager, entries: &mut [PollEntry]) -> usize {
    let mut ready_count = 0;

    for entry in entries.iter_mut() {
        entry.revents.clear();
        let fd = match table.get(entry.fd) {
            Some(fd) => fd,
            None => {
                entry.revents.push(PollEvent::Error);
                ready_count += 1;
                continue;
            }
        };

        let want_read = entry.events.contains(&PollEvent::Readable);
        let want_write = entry.events.contains(&PollEvent::Writable);

        match &fd.target {
            FdTarget::PipeRead { pipe_id } => {
                if want_read {
                    if let Some(pipe) = pipe_mgr.get_pipe(*pipe_id) {
                        if !pipe.is_empty() {
                            entry.revents.push(PollEvent::Readable);
                        } else if !pipe.write_end_open {
                            entry.revents.push(PollEvent::Hangup);
                        }
                    }
                }
            }
            FdTarget::PipeWrite { pipe_id } => {
                if want_write {
                    if let Some(pipe) = pipe_mgr.get_pipe(*pipe_id) {
                        if !pipe.is_full() {
                            entry.revents.push(PollEvent::Writable);
                        }
                        if !pipe.read_end_open {
                            entry.revents.push(PollEvent::Hangup);
                        }
                    }
                }
            }
            FdTarget::Stdin => {
                if want_read { entry.revents.push(PollEvent::Readable); } // Always ready
            }
            FdTarget::Stdout | FdTarget::Stderr => {
                if want_write { entry.revents.push(PollEvent::Writable); } // Always ready
            }
            FdTarget::File { .. } => {
                if want_read { entry.revents.push(PollEvent::Readable); }
                if want_write { entry.revents.push(PollEvent::Writable); }
            }
            FdTarget::Null => {
                if want_read { entry.revents.push(PollEvent::Readable); }
                if want_write { entry.revents.push(PollEvent::Writable); }
            }
            FdTarget::Socket { .. } => {} // Not implemented yet
        }

        if !entry.revents.is_empty() {
            ready_count += 1;
        }
    }

    ready_count
}

// ═══════════════════════════════════════════════════════════════════════════════
// User I/O Manager (integrates FdTable + Pipes + VFS)
// ═══════════════════════════════════════════════════════════════════════════════

pub struct UserIoManager {
    pub fd_tables:   Vec<FdTable>,
    pub pipes:       PipeManager,
    pub total_reads:  u64,
    pub total_writes: u64,
    pub total_opens:  u64,
    pub total_closes: u64,
}

impl Default for UserIoManager {
    fn default() -> Self { Self::new() }
}

impl UserIoManager {
    pub fn new() -> Self {
        Self {
            fd_tables: Vec::new(),
            pipes: PipeManager::new(),
            total_reads: 0,
            total_writes: 0,
            total_opens: 0,
            total_closes: 0,
        }
    }

    /// Register a new process (creates FD table with stdio)
    pub fn register(&mut self, pid: Pid) {
        if !self.fd_tables.iter().any(|t| t.pid == pid) {
            self.fd_tables.push(FdTable::new(pid));
        }
    }

    /// Unregister a process (closes all FDs)
    pub fn unregister(&mut self, pid: Pid) -> usize {
        if let Some(idx) = self.fd_tables.iter().position(|t| t.pid == pid) {
            let count = self.fd_tables[idx].close_all();
            self.fd_tables.remove(idx);
            self.total_closes += count as u64;
            count
        } else {
            0
        }
    }

    /// Open a file for a process
    pub fn open(&mut self, pid: Pid, path: &str, flags: FdFlags) -> Option<u32> {
        let table = self.fd_tables.iter_mut().find(|t| t.pid == pid)?;
        let fd = table.alloc(FdTarget::File { path: path.to_string(), inode: 0 }, flags);
        if fd.is_some() { self.total_opens += 1; }
        fd
    }

    /// Close an FD
    pub fn close(&mut self, pid: Pid, fd: u32) -> bool {
        let table = self.fd_tables.iter_mut().find(|t| t.pid == pid)?;
        let success = table.close(fd);
        if success { self.total_closes += 1; }
        success
    }

    /// Read from an FD — returns bytes read (simulated for files, real for pipes)
    pub fn read(&mut self, pid: Pid, fd: u32, buf: &mut [u8]) -> Option<(usize, bool)> {
        let table = self.fd_tables.iter().find(|t| t.pid == pid)?;
        let entry = table.get(fd)?;

        if !entry.is_readable() { return None; }

        self.total_reads += 1;

        match &entry.target {
            FdTarget::PipeRead { pipe_id } => {
                self.pipes.read(*pipe_id, buf)
            }
            FdTarget::Stdin => {
                // Simulated stdin: return a test string
                let data = b"stdin\n";
                let n = buf.len().min(data.len());
                buf[..n].copy_from_slice(&data[..n]);
                Some((n, false))
            }
            FdTarget::File { .. } | FdTarget::Null => {
                // Simulated file read: return zeros
                let n = buf.len().min(4096);
                for b in &mut buf[..n] { *b = 0; }
                Some((n, false))
            }
            _ => None,
        }
    }

    /// Write to an FD — returns bytes written
    pub fn write(&mut self, pid: Pid, fd: u32, data: &[u8]) -> Option<usize> {
        let table = self.fd_tables.iter().find(|t| t.pid == pid)?;
        let entry = table.get(fd)?;

        if !entry.is_writable() { return None; }

        self.total_writes += 1;

        match &entry.target {
            FdTarget::PipeWrite { pipe_id } => {
                self.pipes.write(*pipe_id, data)
            }
            FdTarget::Stdout | FdTarget::Stderr => {
                // Simulated stdout/stderr: accept all writes
                Some(data.len())
            }
            FdTarget::File { .. } | FdTarget::Null => {
                // Simulated file write: accept all writes
                Some(data.len())
            }
            _ => None,
        }
    }

    /// Create a pipe for a process. Returns (read_fd, write_fd).
    pub fn pipe(&mut self, pid: Pid) -> Option<(u32, u32)> {
        let (pipe_id, read_target, write_target) = self.pipes.create_pipe();
        let table = self.fd_tables.iter_mut().find(|t| t.pid == pid)?;
        let read_fd = table.alloc(read_target, FdFlags::read_only())?;
        let write_fd = table.alloc(write_target, FdFlags::write_only())?;
        Some((read_fd, write_fd))
    }

    /// Create a pipe between two processes. Returns (read_fd in proc1, write_fd in proc2).
    pub fn pipe_between(&mut self, pid1: Pid, pid2: Pid) -> Option<(u32, u32)> {
        let (pipe_id, read_target, write_target) = self.pipes.create_pipe();
        let table1 = self.fd_tables.iter_mut().find(|t| t.pid == pid1)?;
        let read_fd = table1.alloc(read_target, FdFlags::read_only())?;
        let table2 = self.fd_tables.iter_mut().find(|t| t.pid == pid2)?;
        let write_fd = table2.alloc(write_target, FdFlags::write_only())?;
        Some((read_fd, write_fd))
    }

    /// Seek (change file offset)
    pub fn seek(&mut self, pid: Pid, fd: u32, offset: u64) -> bool {
        let table = self.fd_tables.iter_mut().find(|t| t.pid == pid)?;
        table.seek(fd, offset)
    }

    /// Duplicate an FD (dup)
    pub fn dup(&mut self, pid: Pid, fd: u32) -> Option<u32> {
        let table = self.fd_tables.iter_mut().find(|t| t.pid == pid)?;
        table.dup(fd)
    }

    /// Duplicate an FD to a specific target (dup2)
    pub fn dup2(&mut self, pid: Pid, old_fd: u32, new_fd: u32) -> bool {
        let table = self.fd_tables.iter_mut().find(|t| t.pid == pid)?;
        table.dup2(old_fd, new_fd)
    }

    /// Get FD table for a process
    pub fn get_table(&self, pid: Pid) -> Option<&FdTable> {
        self.fd_tables.iter().find(|t| t.pid == pid)
    }

    /// Get FD count for a process
    pub fn fd_count(&self, pid: Pid) -> usize {
        self.get_table(pid).map(|t| t.open_count()).unwrap_or(0)
    }

    /// Poll multiple FDs for a process
    pub fn poll(&self, pid: Pid, mut entries: Vec<PollEntry>) -> usize {
        let table = match self.get_table(pid) {
            Some(t) => t,
            None => return 0,
        };
        poll(table, &self.pipes, &mut entries)
    }

    /// Statistics
    pub fn total_reads(&self) -> u64 { self.total_reads }
    pub fn total_writes(&self) -> u64 { self.total_writes }
    pub fn total_opens(&self) -> u64 { self.total_opens }
    pub fn total_closes(&self) -> u64 { self.total_closes }
    pub fn process_count(&self) -> usize { self.fd_tables.len() }
    pub fn pipe_count(&self) -> usize { self.pipes.pipe_count() }
}

// ═══════════════════════════════════════════════════════════════════════════════
// I/O Errors
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IoError {
    BadFd,          // EBADF
    Permission,     // EACCES
    NoSpace,        // ENOSPC
    PipeBroken,     // EPIPE
    NotPipe,        // ESPIPE
    WouldBlock,     // EAGAIN
    NotFound,       // ENOENT
}

impl core::fmt::Display for IoError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            IoError::BadFd       => write!(f, "bad file descriptor"),
            IoError::Permission  => write!(f, "permission denied"),
            IoError::NoSpace     => write!(f, "no space left"),
            IoError::PipeBroken  => write!(f, "broken pipe"),
            IoError::NotPipe     => write!(f, "not a pipe"),
            IoError::WouldBlock  => write!(f, "would block"),
            IoError::NotFound    => write!(f, "not found"),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // --- FdFlags tests ---

    #[test]
    fn test_fd_flags_read_only() {
        let f = FdFlags::read_only();
        assert!(f.read);
        assert!(!f.write);
    }

    #[test]
    fn test_fd_flags_write_only() {
        let f = FdFlags::write_only();
        assert!(!f.read);
        assert!(f.write);
    }

    #[test]
    fn test_fd_flags_read_write() {
        let f = FdFlags::read_write();
        assert!(f.read);
        assert!(f.write);
    }

    #[test]
    fn test_fd_flags_nonblock() {
        let f = FdFlags::read_write_nb();
        assert!(f.nonblock);
        assert!(!FdFlags::read_write().nonblock);
    }

    // --- FdTarget tests ---

    #[test]
    fn test_fd_target_eq() {
        let f1 = FdTarget::File { path: "test".into(), inode: 1 };
        let f2 = FdTarget::File { path: "test".into(), inode: 1 };
        assert_eq!(f1, f2);
        let f3 = FdTarget::File { path: "other".into(), inode: 2 };
        assert_ne!(f1, f3);
    }

    #[test]
    fn test_fd_target_pipe_variants() {
        let r = FdTarget::PipeRead { pipe_id: 1 };
        let w = FdTarget::PipeWrite { pipe_id: 1 };
        assert_ne!(r, w);
    }

    // --- FileDescriptor tests ---

    #[test]
    fn test_fd_new() {
        let fd = FileDescriptor::new(3, FdTarget::Null, FdFlags::read_write());
        assert_eq!(fd.fd, 3);
        assert!(!fd.is_closed());
        assert!(fd.is_readable());
        assert!(fd.is_writable());
    }

    #[test]
    fn test_fd_closed() {
        let mut fd = FileDescriptor::new(3, FdTarget::Null, FdFlags::read_write());
        fd.closed = true;
        assert!(fd.is_closed());
        assert!(!fd.is_readable());
        assert!(!fd.is_writable());
    }

    // --- FdTable tests ---

    #[test]
    fn test_fd_table_new() {
        let table = FdTable::new(Pid(1000));
        assert_eq!(table.pid, Pid(1000));
        // stdin/stdout/stderr should be pre-allocated
        assert!(table.get(STDIN_FD).is_some());
        assert!(table.get(STDOUT_FD).is_some());
        assert!(table.get(STDERR_FD).is_some());
        assert_eq!(table.open_count(), 3);
    }

    #[test]
    fn test_fd_table_alloc() {
        let mut table = FdTable::new(Pid(1000));
        let fd = table.alloc(FdTarget::Null, FdFlags::read_write());
        assert!(fd.is_some());
        assert_eq!(fd.unwrap(), 3); // First free after stdio
        assert_eq!(table.open_count(), 4);
    }

    #[test]
    fn test_fd_table_alloc_multiple() {
        let mut table = FdTable::new(Pid(1000));
        for i in 0..10 {
            let fd = table.alloc(FdTarget::Null, FdFlags::read_write()).unwrap();
            assert_eq!(fd, 3 + i as u32);
        }
        assert_eq!(table.open_count(), 13);
    }

    #[test]
    fn test_fd_table_close() {
        let mut table = FdTable::new(Pid(1000));
        let fd = table.alloc(FdTarget::Null, FdFlags::read_write()).unwrap();
        assert!(table.close(fd));
        assert_eq!(table.open_count(), 3); // Back to stdio only
        assert!(table.get(fd).is_none());
    }

    #[test]
    fn test_fd_table_close_stdin() {
        let mut table = FdTable::new(Pid(1000));
        assert!(table.close(STDIN_FD));
        assert!(table.get(STDIN_FD).is_none());
        assert_eq!(table.open_count(), 2);
    }

    #[test]
    fn test_fd_table_close_already_closed() {
        let mut table = FdTable::new(Pid(1000));
        let fd = table.alloc(FdTarget::Null, FdFlags::read_write()).unwrap();
        assert!(table.close(fd));
        assert!(!table.close(fd)); // Double close fails
    }

    #[test]
    fn test_fd_table_close_all() {
        let mut table = FdTable::new(Pid(1000));
        table.alloc(FdTarget::Null, FdFlags::read_write());
        table.alloc(FdTarget::Null, FdFlags::read_write());
        let count = table.close_all();
        assert_eq!(count, 5); // 3 stdio + 2 allocated
    }

    #[test]
    fn test_fd_table_dup() {
        let mut table = FdTable::new(Pid(1000));
        let fd = table.alloc(FdTarget::File { path: "test".into(), inode: 1 }, FdFlags::read_write()).unwrap();
        let dup_fd = table.dup(fd);
        assert!(dup_fd.is_some());
        assert_ne!(dup_fd.unwrap(), fd);
        assert_eq!(table.open_count(), 5); // 3 stdio + original + dup
    }

    #[test]
    fn test_fd_table_dup2() {
        let mut table = FdTable::new(Pid(1000));
        let fd = table.alloc(FdTarget::File { path: "test".into(), inode: 1 }, FdFlags::read_write()).unwrap();
        assert!(table.dup2(fd, 10));
        let entry = table.get(10).unwrap();
        assert_eq!(entry.target, FdTarget::File { path: "test".into(), inode: 1 });
    }

    #[test]
    fn test_fd_table_dup2_replaces() {
        let mut table = FdTable::new(Pid(1000));
        let fd1 = table.alloc(FdTarget::Null, FdFlags::read_write()).unwrap();
        let fd2 = table.alloc(FdTarget::Null, FdFlags::read_write()).unwrap();
        // dup2 fd1 onto fd2's slot — should close fd2 first
        assert!(table.dup2(fd1, fd2));
        assert_eq!(table.open_count(), 4); // 3 stdio + fd1 (fd2 replaced)
    }

    #[test]
    fn test_fd_table_list_fds() {
        let mut table = FdTable::new(Pid(1000));
        table.alloc(FdTarget::Null, FdFlags::read_write());
        table.alloc(FdTarget::Null, FdFlags::read_write());
        let fds = table.list_fds();
        assert!(fds.contains(&0)); // stdin
        assert!(fds.contains(&1)); // stdout
        assert!(fds.contains(&2)); // stderr
        assert!(fds.contains(&3));
        assert!(fds.contains(&4));
        assert_eq!(fds.len(), 5);
    }

    #[test]
    fn test_fd_table_seek() {
        let mut table = FdTable::new(Pid(1000));
        let fd = table.alloc(FdTarget::File { path: "test".into(), inode: 1 }, FdFlags::read_write()).unwrap();
        assert!(table.seek(fd, 1024));
        let entry = table.get(fd).unwrap();
        assert_eq!(entry.offset, 1024);
    }

    #[test]
    fn test_fd_table_check_access() {
        let mut table = FdTable::new(Pid(1000));
        let fd = table.alloc(FdTarget::Null, FdFlags::read_only()).unwrap();
        assert!(table.check_access(fd, false)); // Can read
        assert!(!table.check_access(fd, true)); // Cannot write
    }

    #[test]
    fn test_fd_table_reuse_closed_slot() {
        let mut table = FdTable::new(Pid(1000));
        let fd1 = table.alloc(FdTarget::Null, FdFlags::read_write()).unwrap();
        assert_eq!(fd1, 3);
        table.close(fd1);
        let fd2 = table.alloc(FdTarget::Null, FdFlags::read_write()).unwrap();
        assert_eq!(fd2, 3); // Reused slot
    }

    // --- Pipe tests ---

    #[test]
    fn test_pipe_new() {
        let pipe = Pipe::new(1);
        assert_eq!(pipe.id, 1);
        assert!(pipe.is_empty());
        assert!(!pipe.is_full());
        assert!(pipe.read_end_open);
        assert!(pipe.write_end_open);
    }

    #[test]
    fn test_pipe_write_read() {
        let mut pipe = Pipe::new(1);
        let written = pipe.write(b"hello world");
        assert_eq!(written, 11);
        assert_eq!(pipe.available(), 11);

        let mut buf = [0u8; 20];
        let (read, eof) = pipe.read(&mut buf);
        assert_eq!(read, 11);
        assert!(!eof);
        assert_eq!(&buf[..11], b"hello world");
        assert!(pipe.is_empty());
    }

    #[test]
    fn test_pipe_eof() {
        let mut pipe = Pipe::new(1);
        pipe.close_write();
        let mut buf = [0u8; 10];
        let (read, eof) = pipe.read(&mut buf);
        assert_eq!(read, 0);
        assert!(eof); // Write end closed → EOF
    }

    #[test]
    fn test_pipe_no_reader() {
        let mut pipe = Pipe::new(1);
        pipe.close_read();
        let written = pipe.write(b"data");
        assert_eq!(written, 0); // No reader → 0 bytes written
    }

    #[test]
    fn test_pipe_partial_read() {
        let mut pipe = Pipe::new(1);
        pipe.write(b"hello world");
        let mut buf = [0u8; 5];
        let (read, _) = pipe.read(&mut buf);
        assert_eq!(read, 5);
        assert_eq!(&buf, b"hello");
        assert_eq!(pipe.available(), 6); // " world" remaining
    }

    #[test]
    fn test_pipe_stats() {
        let mut pipe = Pipe::new(1);
        pipe.write(b"test data");
        let mut buf = [0u8; 4];
        pipe.read(&mut buf);
        assert_eq!(pipe.total_written, 9);
        assert_eq!(pipe.total_read, 4);
    }

    #[test]
    fn test_pipe_full() {
        let mut pipe = Pipe::new(1);
        let big = vec![0xAAu8; PIPE_BUF_SIZE + 100];
        let written = pipe.write(&big);
        assert_eq!(written, PIPE_BUF_SIZE); // Only writes up to buffer size
        assert!(pipe.is_full());
    }

    // --- PipeManager tests ---

    #[test]
    fn test_pipe_manager_create() {
        let mut mgr = PipeManager::new();
        let (id, read_t, write_t) = mgr.create_pipe();
        assert_eq!(id, 1);
        assert_eq!(mgr.pipe_count(), 1);
        assert!(matches!(read_t, FdTarget::PipeRead { pipe_id: 1 }));
        assert!(matches!(write_t, FdTarget::PipeWrite { pipe_id: 1 }));
    }

    #[test]
    fn test_pipe_manager_multiple() {
        let mut mgr = PipeManager::new();
        let (id1, _, _) = mgr.create_pipe();
        let (id2, _, _) = mgr.create_pipe();
        assert_ne!(id1, id2);
        assert_eq!(mgr.pipe_count(), 2);
    }

    #[test]
    fn test_pipe_manager_write_read() {
        let mut mgr = PipeManager::new();
        let (id, _, _) = mgr.create_pipe();
        let written = mgr.write(id, b"hello").unwrap();
        assert_eq!(written, 5);
        let mut buf = [0u8; 10];
        let (read, _) = mgr.read(id, &mut buf).unwrap();
        assert_eq!(read, 5);
    }

    #[test]
    fn test_pipe_manager_close_end() {
        let mut mgr = PipeManager::new();
        let (id, _, _) = mgr.create_pipe();
        mgr.close_end(id, true); // Close write end
        let pipe = mgr.get_pipe(id).unwrap();
        assert!(!pipe.write_end_open);
    }

    #[test]
    fn test_pipe_manager_cleanup() {
        let mut mgr = PipeManager::new();
        let (id1, _, _) = mgr.create_pipe();
        let (id2, _, _) = mgr.create_pipe();
        // Close both ends of pipe 1
        mgr.close_end(id1, true);
        mgr.close_end(id1, false);
        assert_eq!(mgr.pipe_count(), 2);
        mgr.cleanup();
        assert_eq!(mgr.pipe_count(), 1); // Only pipe 2 remains
    }

    // --- Poll tests ---

    #[test]
    fn test_poll_stdin_readable() {
        let table = FdTable::new(Pid(1000));
        let mgr = PipeManager::new();
        let mut entries = vec![PollEntry::new(STDIN_FD, vec![PollEvent::Readable])];
        let ready = poll(&table, &mgr, &mut entries);
        assert_eq!(ready, 1);
        assert!(entries[0].revents.contains(&PollEvent::Readable));
    }

    #[test]
    fn test_poll_stdout_writable() {
        let table = FdTable::new(Pid(1000));
        let mgr = PipeManager::new();
        let mut entries = vec![PollEntry::new(STDOUT_FD, vec![PollEvent::Writable])];
        let ready = poll(&table, &mgr, &mut entries);
        assert_eq!(ready, 1);
        assert!(entries[0].revents.contains(&PollEvent::Writable));
    }

    #[test]
    fn test_poll_pipe_readable() {
        let mut table = FdTable::new(Pid(1000));
        let mut mgr = PipeManager::new();
        let (id, read_t, _) = mgr.create_pipe();
        let fd = table.alloc(read_t, FdFlags::read_only()).unwrap();
        mgr.write(id, b"data").unwrap();

        let mut entries = vec![PollEntry::new(fd, vec![PollEvent::Readable])];
        let ready = poll(&table, &mgr, &mut entries);
        assert_eq!(ready, 1);
        assert!(entries[0].revents.contains(&PollEvent::Readable));
    }

    #[test]
    fn test_poll_pipe_empty_not_readable() {
        let mut table = FdTable::new(Pid(1000));
        let mut mgr = PipeManager::new();
        let (_, read_t, _) = mgr.create_pipe();
        let fd = table.alloc(read_t, FdFlags::read_only()).unwrap();

        let mut entries = vec![PollEntry::new(fd, vec![PollEvent::Readable])];
        let ready = poll(&table, &mgr, &mut entries);
        assert_eq!(ready, 0); // Empty pipe, not ready
    }

    #[test]
    fn test_poll_pipe_hangup() {
        let mut table = FdTable::new(Pid(1000));
        let mut mgr = PipeManager::new();
        let (id, read_t, _) = mgr.create_pipe();
        let fd = table.alloc(read_t, FdFlags::read_only()).unwrap();
        mgr.close_end(id, true); // Close write end → hangup

        let mut entries = vec![PollEntry::new(fd, vec![PollEvent::Readable])];
        let ready = poll(&table, &mgr, &mut entries);
        assert_eq!(ready, 1);
        assert!(entries[0].revents.contains(&PollEvent::Hangup));
    }

    #[test]
    fn test_poll_bad_fd() {
        let table = FdTable::new(Pid(1000));
        let mgr = PipeManager::new();
        let mut entries = vec![PollEntry::new(99, vec![PollEvent::Readable])];
        let ready = poll(&table, &mgr, &mut entries);
        assert_eq!(ready, 1);
        assert!(entries[0].revents.contains(&PollEvent::Error));
    }

    #[test]
    fn test_poll_multiple_fds() {
        let mut table = FdTable::new(Pid(1000));
        let mgr = PipeManager::new();
        let fd = table.alloc(FdTarget::Null, FdFlags::read_write()).unwrap();
        let mut entries = vec![
            PollEntry::new(STDIN_FD, vec![PollEvent::Readable]),
            PollEntry::new(STDOUT_FD, vec![PollEvent::Writable]),
            PollEntry::new(fd, vec![PollEvent::Readable, PollEvent::Writable]),
        ];
        let ready = poll(&table, &mgr, &mut entries);
        assert_eq!(ready, 3); // All ready
    }

    // --- UserIoManager tests ---

    #[test]
    fn test_io_manager_new() {
        let mgr = UserIoManager::new();
        assert_eq!(mgr.process_count(), 0);
        assert_eq!(mgr.pipe_count(), 0);
    }

    #[test]
    fn test_io_manager_register() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        assert_eq!(mgr.process_count(), 1);
        assert_eq!(mgr.fd_count(Pid(1000)), 3); // stdio
    }

    #[test]
    fn test_io_manager_double_register() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        mgr.register(Pid(1000)); // No-op
        assert_eq!(mgr.process_count(), 1);
    }

    #[test]
    fn test_io_manager_unregister() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let closed = mgr.unregister(Pid(1000));
        assert_eq!(closed, 3); // stdio closed
        assert_eq!(mgr.process_count(), 0);
    }

    #[test]
    fn test_io_manager_open() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let fd = mgr.open(Pid(1000), "/test/file", FdFlags::read_write());
        assert!(fd.is_some());
        assert_eq!(mgr.fd_count(Pid(1000)), 4);
        assert_eq!(mgr.total_opens(), 1);
    }

    #[test]
    fn test_io_manager_close() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let fd = mgr.open(Pid(1000), "/test", FdFlags::read_write()).unwrap();
        assert!(mgr.close(Pid(1000), fd));
        assert_eq!(mgr.fd_count(Pid(1000)), 3);
        assert_eq!(mgr.total_closes(), 1);
    }

    #[test]
    fn test_io_manager_write_stdout() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let written = mgr.write(Pid(1000), STDOUT_FD, b"hello").unwrap();
        assert_eq!(written, 5);
        assert_eq!(mgr.total_writes(), 1);
    }

    #[test]
    fn test_io_manager_read_stdin() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let mut buf = [0u8; 10];
        let (read, _) = mgr.read(Pid(1000), STDIN_FD, &mut buf).unwrap();
        assert!(read > 0);
        assert_eq!(mgr.total_reads(), 1);
    }

    #[test]
    fn test_io_manager_pipe_same_process() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let (read_fd, write_fd) = mgr.pipe(Pid(1000)).unwrap();
        assert_ne!(read_fd, write_fd);

        // Write to pipe
        mgr.write(Pid(1000), write_fd, b"pipe data").unwrap();
        // Read from pipe
        let mut buf = [0u8; 20];
        let (read, eof) = mgr.read(Pid(1000), read_fd, &mut buf).unwrap();
        assert_eq!(read, 9);
        assert!(!eof);
        assert_eq!(&buf[..9], b"pipe data");
    }

    #[test]
    fn test_io_manager_pipe_between_processes() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        mgr.register(Pid(1001));

        let (read_fd, write_fd) = mgr.pipe_between(Pid(1000), Pid(1001)).unwrap();

        // P1001 writes, P1000 reads
        mgr.write(Pid(1001), write_fd, b"cross-process").unwrap();
        let mut buf = [0u8; 20];
        let (read, _) = mgr.read(Pid(1000), read_fd, &mut buf).unwrap();
        assert_eq!(read, 15);
        assert_eq!(&buf[..15], b"cross-process");
    }

    #[test]
    fn test_io_manager_dup() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let fd = mgr.open(Pid(1000), "/test", FdFlags::read_write()).unwrap();
        let dup_fd = mgr.dup(Pid(1000), fd);
        assert!(dup_fd.is_some());
        assert_ne!(dup_fd.unwrap(), fd);
    }

    #[test]
    fn test_io_manager_seek() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let fd = mgr.open(Pid(1000), "/test", FdFlags::read_write()).unwrap();
        assert!(mgr.seek(Pid(1000), fd, 2048));
    }

    #[test]
    fn test_io_manager_read_closed_fd() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let fd = mgr.open(Pid(1000), "/test", FdFlags::read_write()).unwrap();
        mgr.close(Pid(1000), fd);
        assert!(mgr.read(Pid(1000), fd, &mut [0u8; 10]).is_none());
    }

    #[test]
    fn test_io_manager_write_readonly_fd() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let fd = mgr.open(Pid(1000), "/test", FdFlags::read_only()).unwrap();
        assert!(mgr.write(Pid(1000), fd, b"data").is_none());
    }

    #[test]
    fn test_io_manager_read_writeonly_fd() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let fd = mgr.open(Pid(1000), "/test", FdFlags::write_only()).unwrap();
        assert!(mgr.read(Pid(1000), fd, &mut [0u8; 10]).is_none());
    }

    #[test]
    fn test_io_manager_poll() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        let entries = vec![
            PollEntry::new(STDIN_FD, vec![PollEvent::Readable]),
            PollEntry::new(STDOUT_FD, vec![PollEvent::Writable]),
        ];
        let ready = mgr.poll(Pid(1000), entries);
        assert_eq!(ready, 2);
    }

    #[test]
    fn test_io_manager_stats() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        mgr.open(Pid(1000), "/a", FdFlags::read_write()).unwrap();
        mgr.open(Pid(1000), "/b", FdFlags::read_write()).unwrap();
        mgr.write(Pid(1000), STDOUT_FD, b"hello").unwrap();
        mgr.read(Pid(1000), STDIN_FD, &mut [0u8; 10]).unwrap();
        mgr.close(Pid(1000), 3).unwrap();
        assert_eq!(mgr.total_opens(), 2);
        assert_eq!(mgr.total_writes(), 1);
        assert_eq!(mgr.total_reads(), 1);
        assert_eq!(mgr.total_closes(), 1);
    }

    #[test]
    fn test_io_manager_full_lifecycle() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));

        // Open files
        let f1 = mgr.open(Pid(1000), "/etc/config", FdFlags::read_write()).unwrap();
        let f2 = mgr.open(Pid(1000), "/var/log", FdFlags::write_only()).unwrap();

        // Create pipe
        let (rfd, wfd) = mgr.pipe(Pid(1000)).unwrap();

        // Write to pipe
        mgr.write(Pid(1000), wfd, b"pipe test").unwrap();

        // Read from pipe
        let mut buf = [0u8; 20];
        let (read, _) = mgr.read(Pid(1000), rfd, &mut buf).unwrap();
        assert_eq!(read, 9);

        // Dup
        let dup = mgr.dup(Pid(1000), f1).unwrap();

        // Seek
        mgr.seek(Pid(1000), f1, 4096);

        // Close
        mgr.close(Pid(1000), f2).unwrap();
        assert_eq!(mgr.fd_count(Pid(1000)), 6); // 3 stdio + f1 + pipe r/w + dup

        // Unregister
        let closed = mgr.unregister(Pid(1000));
        assert_eq!(closed, 6);
        assert_eq!(mgr.process_count(), 0);
    }

    #[test]
    fn test_io_error_display() {
        assert_eq!(format!("{}", IoError::BadFd), "bad file descriptor");
        assert_eq!(format!("{}", IoError::PipeBroken), "broken pipe");
        assert_eq!(format!("{}", IoError::WouldBlock), "would block");
        assert_eq!(format!("{}", IoError::NotFound), "not found");
    }

    #[test]
    fn test_fd_table_max_fds() {
        let mut table = FdTable::new(Pid(1000));
        let mut count = 0;
        while table.alloc(FdTarget::Null, FdFlags::read_write()).is_some() {
            count += 1;
            if count > MAX_FDS { break; }
        }
        assert_eq!(count, MAX_FDS - 3); // MAX_FDS minus stdio
    }

    #[test]
    fn test_multiple_processes_isolated() {
        let mut mgr = UserIoManager::new();
        mgr.register(Pid(1000));
        mgr.register(Pid(1001));

        let fd1 = mgr.open(Pid(1000), "/proc1", FdFlags::read_write()).unwrap();
        let fd2 = mgr.open(Pid(1001), "/proc2", FdFlags::read_write()).unwrap();

        // Each process has its own FD table
        assert_eq!(mgr.fd_count(Pid(1000)), 4);
        assert_eq!(mgr.fd_count(Pid(1001)), 4);
        // Both have fd=3, but different files
        assert!(mgr.get_table(Pid(1000)).unwrap().get(3).is_some());
        assert!(mgr.get_table(Pid(1001)).unwrap().get(3).is_some());
    }
}
