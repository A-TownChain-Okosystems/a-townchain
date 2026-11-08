// Copyright (c) 2026 Michael Wroblewski / ShivaCore / A-TownChain-Okosystems. All Rights Reserved.
// ─────────────────────────────────────────────────────────────────────────
// K-Sprint 9 — Syscall Interface (ATC-96)
// Kernel Layer | Chain-ID 658467
// Dispatch-Layer: einheitliche Schnittstelle für alle Kernel-Subsysteme.
// Context-Isolation (Node/Contract/Test), Gas-Tracking, Capability-Gating.
// ─────────────────────────────────────────────────────────────────────────

use alloc::format;
use alloc::vec;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::sync::Arc;
use spin::Mutex;

use crate::capability::{CapabilityTable, Rights};
use crate::process::ProcessManager;
use crate::ipc::{IpcSubsystem, ChannelId};
use crate::ats1000::Pid;
use crate::capability::{ResourceType, CapId};
use crate::vfs::{Vfs, OpenMode, VfsError};
use crate::scheduler::DaHeftScheduler as Scheduler;

// ─── Syscall-Nummern (ATC-96) ─────────────────────────────────────────────

pub const SYS_SPAWN: u32 = 1;
pub const SYS_KILL: u32 = 2;
pub const SYS_SLEEP: u32 = 3;
pub const SYS_WAIT: u32 = 4;

pub const SYS_ALLOC: u32 = 10;
pub const SYS_FREE: u32 = 11;
pub const SYS_MEMCPY: u32 = 12;

pub const SYS_OPEN: u32 = 20;
pub const SYS_READ: u32 = 21;
pub const SYS_WRITE: u32 = 22;
pub const SYS_CLOSE: u32 = 23;
pub const SYS_SEEK: u32 = 24;
pub const SYS_MKDIR: u32 = 25;
pub const SYS_RMDIR: u32 = 26;
pub const SYS_LISTDIR: u32 = 27;
pub const SYS_STAT: u32 = 28;
pub const SYS_CREATE_FILE: u32 = 29;
pub const SYS_REMOVE_FILE: u32 = 30;
pub const SYS_SYMLINK: u32 = 31;
pub const SYS_READLINK: u32 = 32;

pub const SYS_IPC_CREATE: u32 = 40;
pub const SYS_IPC_SEND: u32 = 41;
pub const SYS_IPC_RECV: u32 = 42;
pub const SYS_IPC_GRANT: u32 = 43;
pub const SYS_IPC_CLOSE: u32 = 44;

pub const SYS_CAP_CREATE: u32 = 50;
pub const SYS_CAP_DELEGATE: u32 = 51;
pub const SYS_CAP_CHECK: u32 = 52;
pub const SYS_CAP_REVOKE: u32 = 53;

pub const SYS_SCHED_YIELD: u32 = 60;
pub const SYS_SCHED_INFO: u32 = 61;

pub const SYS_KG_QUERY: u32 = 70;
pub const SYS_KG_CREATE_ENTITY: u32 = 71;
pub const SYS_KG_ADD_TRIPLE: u32 = 72;

// ─── Ausführungs-Context (ATC-96 §3) ───────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Context {
    /// Vollzugriff — Kernel-intern oder privilegierter Node
    Node,
    /// Eingeschränkt — nur alloc/free, keine I/O
    Contract,
    /// Mock-Syscalls für Tests
    Test,
    /// User-space (Ring 3) — eingeschränkter Syscall-Zugriff via Interrupt-Gate
    User { pid: u32 },
    /// User-space (Ring 3) — eingeschränkter Syscall-Zugriff via Interrupt-Gate
    User { pid: u32 },
}

impl Context {
    /// Prüft ob ein Syscall in diesem Context erlaubt ist
    pub fn allows(&self, syscall: u32) -> bool {
        match self {
            Context::Node => true, // alle Syscalls
            Context::Contract => {
                matches!(syscall,
                    SYS_ALLOC | SYS_FREE | SYS_MEMCPY
                    | SYS_CAP_CREATE | SYS_CAP_DELEGATE | SYS_CAP_CHECK | SYS_CAP_REVOKE
                    | SYS_SCHED_YIELD
                )
            }
            Context::Test => true, // alle (mit Mocks)
            Context::User { .. } => {
                matches!(syscall,
                    SYS_OPEN | SYS_READ | SYS_WRITE | SYS_CLOSE
                    | SYS_SPAWN | SYS_KILL | SYS_WAIT
                    | SYS_IPC_CREATE | SYS_IPC_SEND | SYS_IPC_RECV
                    | SYS_IPC_GRANT | SYS_IPC_CLOSE
                    | SYS_CAP_CHECK | SYS_CAP_CREATE | SYS_CAP_DELEGATE
                    | SYS_ALLOC | SYS_FREE | SYS_MEMCPY | SYS_SCHED_YIELD
                )
            }
        }
    }
}

// ─── Gas-Kosten (ATC-96 §2/§4) ─────────────────────────────────────────────

pub fn gas_cost(syscall: u32) -> u64 {
    match syscall {
        SYS_SPAWN => 500,
        SYS_KILL => 100,
        SYS_SLEEP => 10,
        SYS_WAIT => 5,
        SYS_ALLOC => 10,      // pro byte, hier Basis
        SYS_FREE => 5,
        SYS_MEMCPY => 1,      // pro byte, hier Basis
        SYS_OPEN => 50,
        SYS_READ => 20,
        SYS_WRITE => 20,
        SYS_CLOSE => 5,
        SYS_SEEK => 5,
        SYS_MKDIR => 50,
        SYS_RMDIR => 50,
        SYS_LISTDIR => 20,
        SYS_STAT => 20,
        SYS_CREATE_FILE => 50,
        SYS_REMOVE_FILE => 50,
        SYS_SYMLINK => 50,
        SYS_READLINK => 20,
        SYS_IPC_CREATE => 100,
        SYS_IPC_SEND => 30,
        SYS_IPC_RECV => 30,
        SYS_IPC_GRANT => 50,
        SYS_IPC_CLOSE => 20,
        SYS_CAP_CREATE => 50,
        SYS_CAP_DELEGATE => 50,
        SYS_CAP_CHECK => 10,
        SYS_CAP_REVOKE => 50,
        SYS_SCHED_YIELD => 10,
        SYS_SCHED_INFO => 10,
        SYS_KG_QUERY => 50,
        SYS_KG_CREATE_ENTITY => 100,
        SYS_KG_ADD_TRIPLE => 50,
        _ => 0,
    }
}

// ─── Syscall-Ergebnis ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyscallResult {
    /// Success mit u64-Rückgabewert (pid, fd, bytes, etc.)
    Success(u64),
    /// Success mit String-Rückgabewert (pfad, query-result, etc.)
    SuccessString(String),
    /// Success mit Vec von Strings (directory listing, etc.)
    SuccessList(Vec<String>),
    /// Erfolg ohne Rückgabewert
    Ok,
    /// Fehler
    Error(SyscallError),
}

impl SyscallResult {
    pub fn is_ok(&self) -> bool {
        !matches!(self, SyscallResult::Error(_))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyscallError {
    /// Syscall in diesem Context nicht erlaubt
    PermissionDenied,
    /// Nicht genug Gas
    OutOfGas,
    /// Ungültige Argumente
    InvalidArgument(String),
    /// Nicht gefunden
    NotFound,
    /// Bereits vorhanden
    AlreadyExists,
    /// I/O Fehler
    IoError(String),
    /// Unbekannter Syscall
    UnknownSyscall(u32),
    /// Capability-Check fehlgeschlagen
    CapabilityDenied,
    /// VFS-Fehler
    VfsError(VfsError),
    /// Prozess-Fehler
    ProcessError(String),
    /// IPC-Fehler
    IpcError(String),
}

impl From<VfsError> for SyscallError {
    fn from(e: VfsError) -> Self {
        SyscallError::VfsError(e)
    }
}

// ─── Syscall-Dispatcher ─────────────────────────────────────────────────────

pub struct SyscallDispatcher {
    context: Context,
    gas_remaining: u64,
    gas_used: u64,
    caps: Arc<Mutex<CapabilityTable>>,
    processes: Arc<Mutex<ProcessManager>>,
    ipc: Arc<Mutex<IpcSubsystem>>,
    vfs: Arc<Mutex<Vfs>>,
    // Track which cap belongs to which pid
    pid: u64,
    cap_handle: u64,
}

impl SyscallDispatcher {
    pub fn new(
        context: Context,
        gas_budget: u64,
        pid: u64,
        cap_handle: u64,
        caps: Arc<Mutex<CapabilityTable>>,
        processes: Arc<Mutex<ProcessManager>>,
        ipc: Arc<Mutex<IpcSubsystem>>,
        vfs: Arc<Mutex<Vfs>>,
    ) -> Self {
        SyscallDispatcher {
            context,
            gas_remaining: gas_budget,
            gas_used: 0,
            caps,
            processes,
            ipc,
            vfs,
            pid,
            cap_handle,
        }
    }

    /// Verbraucht Gas für einen Syscall. Gibt false zurück wenn nicht genug Gas.
    fn charge_gas(&mut self, syscall: u32) -> bool {
        let cost = gas_cost(syscall);
        if self.gas_remaining < cost {
            return false;
        }
        self.gas_remaining -= cost;
        self.gas_used += cost;
        true
    }

    /// Prüft Capability für eine gegebene Operation
    fn check_cap(&self, required: Rights) -> bool {
        let table = self.caps.lock();
        if let Some(cap) = table.get(crate::capability::CapId(self.cap_handle)) {
            cap.owner == Pid(self.pid as u32) && cap.rights.has(required)
        } else {
            false
        }
    }

    /// Haupt-Dispatch-Funktion
    pub fn dispatch(&mut self, syscall: u32, args: &[SyscallArg]) -> SyscallResult {
        // 1. Context-Check
        if !self.context.allows(syscall) {
            return SyscallResult::Error(SyscallError::PermissionDenied);
        }

        // 2. Gas-Check
        if !self.charge_gas(syscall) {
            return SyscallResult::Error(SyscallError::OutOfGas);
        }

        // 3. Dispatch
        match syscall {
            // ── Prozessverwaltung ────────────────────────────────────────────
            SYS_SPAWN => self.sys_spawn(args),
            SYS_KILL => self.sys_kill(args),
            SYS_WAIT => self.sys_wait(args),
            SYS_SLEEP => SyscallResult::Ok, // no-op in test mode

            // ── Speicher ─────────────────────────────────────────────────────
            SYS_ALLOC => SyscallResult::Success(0), // stub — real alloc via global allocator
            SYS_FREE => SyscallResult::Ok,
            SYS_MEMCPY => SyscallResult::Ok,

            // ── VFS ────────────────────────────────────────────────────────────
            SYS_OPEN => self.sys_open(args),
            SYS_READ => self.sys_read(args),
            SYS_WRITE => self.sys_write(args),
            SYS_CLOSE => self.sys_close(args),
            SYS_SEEK => self.sys_seek(args),
            SYS_MKDIR => self.sys_mkdir(args),
            SYS_RMDIR => self.sys_rmdir(args),
            SYS_LISTDIR => self.sys_listdir(args),
            SYS_STAT => self.sys_stat(args),
            SYS_CREATE_FILE => self.sys_create_file(args),
            SYS_REMOVE_FILE => self.sys_remove_file(args),
            SYS_SYMLINK => self.sys_symlink(args),
            SYS_READLINK => self.sys_readlink(args),

            // ── IPC ────────────────────────────────────────────────────────────
            SYS_IPC_CREATE => self.sys_ipc_create(args),
            SYS_IPC_SEND => self.sys_ipc_send(args),
            SYS_IPC_RECV => self.sys_ipc_recv(args),
            SYS_IPC_GRANT => self.sys_ipc_grant(args),
            SYS_IPC_CLOSE => self.sys_ipc_close(args),

            // ── Capabilities ──────────────────────────────────────────────────
            SYS_CAP_CREATE => self.sys_cap_create(args),
            SYS_CAP_DELEGATE => self.sys_cap_delegate(args),
            SYS_CAP_CHECK => self.sys_cap_check(args),
            SYS_CAP_REVOKE => self.sys_cap_revoke(args),

            // ── Scheduler ──────────────────────────────────────────────────────
            SYS_SCHED_YIELD => SyscallResult::Ok, // cooperative yield — no-op in test
            SYS_SCHED_INFO => SyscallResult::SuccessString("scheduler: 1 task".to_string()),

            // ── Knowledge Graph ────────────────────────────────────────────────
            SYS_KG_QUERY => SyscallResult::SuccessString("kg: no results".to_string()),
            SYS_KG_CREATE_ENTITY => SyscallResult::Success(0),
            SYS_KG_ADD_TRIPLE => SyscallResult::Ok,

            _ => SyscallResult::Error(SyscallError::UnknownSyscall(syscall)),
        }
    }

    // ── Prozess-Syscalls ────────────────────────────────────────────────────

    fn sys_spawn(&self, args: &[SyscallArg]) -> SyscallResult {
        let name = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("spawn needs name".into())),
        };

        if !self.check_cap(Rights::EXEC) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }

        let mut pm = self.processes.lock();
        let ptype = crate::process::ProcessType::Agent;
        let pid = pm.spawn(ptype, 128);
        SyscallResult::Success(pid.0 as u64)
    }

    fn sys_kill(&self, args: &[SyscallArg]) -> SyscallResult {
        let target_pid = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("kill needs pid".into())),
        };

        if !self.check_cap(Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }

        let mut pm = self.processes.lock();
        let result = pm.kill(Pid(target_pid as u32), 0);
        if result { SyscallResult::Ok } else { SyscallResult::Error(SyscallError::ProcessError("kill failed".into())) }
    }

    fn sys_wait(&self, args: &[SyscallArg]) -> SyscallResult {
        let target_pid = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("wait needs pid".into())),
        };

        let pm = self.processes.lock();
        match pm.wait(Pid(target_pid as u32)) {
            Some(code) => SyscallResult::Success(code as u64),
            None => SyscallResult::Error(SyscallError::ProcessError("process not terminated".into())),
        }
    }

    // ── VFS-Syscalls ─────────────────────────────────────────────────────────

    fn sys_open(&self, args: &[SyscallArg]) -> SyscallResult {
        let path = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("open needs path".into())),
        };
        let mode = match args.get(1) {
            Some(SyscallArg::U64(m)) => match m {
                0 => OpenMode::Read,
                1 => OpenMode::Write,
                2 => OpenMode::ReadWrite,
                3 => OpenMode::Append,
                4 => OpenMode::Create,
                _ => OpenMode::Read,
            },
            _ => OpenMode::Read,
        };

        if !self.check_cap(Rights::READ | Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }

        let mut vfs = self.vfs.lock();
        match vfs.open(&path, mode, self.pid, self.cap_handle) {
            Ok(fd) => SyscallResult::Success(fd),
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_read(&self, args: &[SyscallArg]) -> SyscallResult {
        let fd = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("read needs fd".into())),
        };
        let len = args.get(1).and_then(|a| match a {
            SyscallArg::U64(v) => Some(*v as usize),
            _ => None,
        }).unwrap_or(256);

        if !self.check_cap(Rights::READ) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }

        let mut vfs = self.vfs.lock();
        let mut buf = vec![0u8; len];
        match vfs.read(fd, &mut buf, self.pid, self.cap_handle) {
            Ok(n) => {
                buf.truncate(n);
                SyscallResult::SuccessString(String::from_utf8_lossy(&buf).to_string())
            }
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_write(&self, args: &[SyscallArg]) -> SyscallResult {
        let fd = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("write needs fd".into())),
        };
        let data = match args.get(1) {
            Some(SyscallArg::String(s)) => s.as_bytes().to_vec(),
            Some(SyscallArg::Bytes(b)) => b.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("write needs data".into())),
        };

        if !self.check_cap(Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }

        let mut vfs = self.vfs.lock();
        match vfs.write(fd, &data, self.pid, self.cap_handle) {
            Ok(n) => SyscallResult::Success(n as u64),
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_close(&self, args: &[SyscallArg]) -> SyscallResult {
        let fd = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("close needs fd".into())),
        };
        let mut vfs = self.vfs.lock();
        match vfs.close(fd) {
            Ok(()) => SyscallResult::Ok,
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_seek(&self, args: &[SyscallArg]) -> SyscallResult {
        let fd = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("seek needs fd".into())),
        };
        let offset = match args.get(1) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("seek needs offset".into())),
        };
        let mut vfs = self.vfs.lock();
        match vfs.seek(fd, offset) {
            Ok(pos) => SyscallResult::Success(pos),
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_mkdir(&self, args: &[SyscallArg]) -> SyscallResult {
        let path = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("mkdir needs path".into())),
        };
        if !self.check_cap(Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut vfs = self.vfs.lock();
        match vfs.mkdir(&path, self.pid, self.cap_handle) {
            Ok(()) => SyscallResult::Ok,
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_rmdir(&self, args: &[SyscallArg]) -> SyscallResult {
        let path = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("rmdir needs path".into())),
        };
        if !self.check_cap(Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut vfs = self.vfs.lock();
        match vfs.rmdir(&path, self.pid, self.cap_handle) {
            Ok(()) => SyscallResult::Ok,
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_listdir(&self, args: &[SyscallArg]) -> SyscallResult {
        let path = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("listdir needs path".into())),
        };
        if !self.check_cap(Rights::READ) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let vfs = self.vfs.lock();
        match vfs.list_dir(&path, self.pid, self.cap_handle) {
            Ok(entries) => {
                let names: Vec<String> = entries.iter()
                    .map(|e| format!("{} ({}, {} bytes)", e.name, 
                        match e.file_type {
                            crate::vfs::FileType::File => "file",
                            crate::vfs::FileType::Directory => "dir",
                            crate::vfs::FileType::Symlink => "link",
                        },
                        e.size))
                    .collect();
                SyscallResult::SuccessList(names)
            }
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_stat(&self, args: &[SyscallArg]) -> SyscallResult {
        let path = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("stat needs path".into())),
        };
        if !self.check_cap(Rights::READ) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let vfs = self.vfs.lock();
        match vfs.stat(&path, self.pid, self.cap_handle) {
            Ok(meta) => SyscallResult::SuccessString(format!(
                "type={:?} size={} owner={}",
                meta.file_type, meta.size, meta.owner_pid
            )),
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_create_file(&self, args: &[SyscallArg]) -> SyscallResult {
        let path = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("create_file needs path".into())),
        };
        if !self.check_cap(Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut vfs = self.vfs.lock();
        match vfs.create_file(&path, self.pid, self.cap_handle) {
            Ok(()) => SyscallResult::Ok,
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_remove_file(&self, args: &[SyscallArg]) -> SyscallResult {
        let path = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("remove_file needs path".into())),
        };
        if !self.check_cap(Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut vfs = self.vfs.lock();
        match vfs.remove_file(&path, self.pid, self.cap_handle) {
            Ok(()) => SyscallResult::Ok,
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_symlink(&self, args: &[SyscallArg]) -> SyscallResult {
        let path = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("symlink needs path".into())),
        };
        let target = match args.get(1) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("symlink needs target".into())),
        };
        if !self.check_cap(Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut vfs = self.vfs.lock();
        match vfs.create_symlink(&path, &target, self.pid, self.cap_handle) {
            Ok(()) => SyscallResult::Ok,
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    fn sys_readlink(&self, args: &[SyscallArg]) -> SyscallResult {
        let path = match args.get(0) {
            Some(SyscallArg::String(s)) => s.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("readlink needs path".into())),
        };
        if !self.check_cap(Rights::READ) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let vfs = self.vfs.lock();
        match vfs.read_symlink(&path, self.pid, self.cap_handle) {
            Ok(target) => SyscallResult::SuccessString(target),
            Err(e) => SyscallResult::Error(SyscallError::VfsError(e)),
        }
    }

    // ── IPC-Syscalls ──────────────────────────────────────────────────────────

    fn sys_ipc_create(&self, args: &[SyscallArg]) -> SyscallResult {
        let capacity = args.get(0).and_then(|a| match a {
            SyscallArg::U64(v) => Some(*v as usize),
            _ => None,
        }).unwrap_or(64);

        if !self.check_cap(Rights::WRITE | Rights::DELEGATE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut ipc = self.ipc.lock();
        let mut caps = self.caps.lock();
        let channel_id = ipc.create_channel(&mut caps, Pid(self.pid as u32), capacity);
        SyscallResult::Success(channel_id.0)
    }

    fn sys_ipc_send(&self, args: &[SyscallArg]) -> SyscallResult {
        let channel_id = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("ipc_send needs channel_id".into())),
        };
        let data = match args.get(1) {
            Some(SyscallArg::String(s)) => s.as_bytes().to_vec(),
            Some(SyscallArg::Bytes(b)) => b.clone(),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("ipc_send needs data".into())),
        };

        if !self.check_cap(Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut ipc = self.ipc.lock();
        let caps = self.caps.lock();
        match ipc.send(&caps, Pid(self.pid as u32), ChannelId(channel_id), data) {
            Ok(()) => SyscallResult::Ok,
            Err(e) => SyscallResult::Error(SyscallError::IpcError(format!("{:?}", e))),
        }
    }

    fn sys_ipc_recv(&self, args: &[SyscallArg]) -> SyscallResult {
        let channel_id = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("ipc_recv needs channel_id".into())),
        };

        if !self.check_cap(Rights::READ) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut ipc = self.ipc.lock();
        let caps = self.caps.lock();
        match ipc.recv(&caps, Pid(self.pid as u32), ChannelId(channel_id)) {
            Ok(data) => SyscallResult::SuccessString(String::from_utf8_lossy(&data.data).to_string()),
            Err(e) => SyscallResult::Error(SyscallError::IpcError(format!("{:?}", e))),
        }
    }

    fn sys_ipc_grant(&self, args: &[SyscallArg]) -> SyscallResult {
        let channel_id = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("ipc_grant needs channel_id".into())),
        };
        let target_pid = match args.get(1) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("ipc_grant needs target_pid".into())),
        };
        let rights = match args.get(2) {
            Some(SyscallArg::U64(v)) => Rights::from_bits_truncate(*v as u8),
            _ => Rights::READ,
        };

        if !self.check_cap(Rights::DELEGATE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut ipc = self.ipc.lock();
        let mut caps = self.caps.lock();
        match ipc.grant_access(&mut caps, Pid(self.pid as u32), ChannelId(channel_id), Pid(target_pid as u32), rights) {
            Ok(cap_id) => SyscallResult::Success(cap_id.0),
            Err(e) => SyscallResult::Error(SyscallError::IpcError(format!("{:?}", e))),
        }
    }

    fn sys_ipc_close(&self, args: &[SyscallArg]) -> SyscallResult {
        let channel_id = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("ipc_close needs channel_id".into())),
        };

        if !self.check_cap(Rights::WRITE) {
            return SyscallResult::Error(SyscallError::CapabilityDenied);
        }
        let mut ipc = self.ipc.lock();
        let mut caps = self.caps.lock();
        let result = ipc.close_channel(&mut caps, Pid(self.pid as u32), ChannelId(channel_id));
        if result { SyscallResult::Ok } else { SyscallResult::Error(SyscallError::IpcError("close failed".into())) }
    }

    // ── Capability-Syscalls ──────────────────────────────────────────────────

    fn sys_cap_create(&self, args: &[SyscallArg]) -> SyscallResult {
        let rights = match args.get(0) {
            Some(SyscallArg::U64(v)) => Rights::from_bits_truncate(*v as u8),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("cap_create needs rights".into())),
        };
        let mut table = self.caps.lock();
        let cap_id = table.create(Pid(self.pid as u32), ResourceType::IpcChannel, 0, rights);
        SyscallResult::Success(cap_id.0)
    }

    fn sys_cap_delegate(&self, args: &[SyscallArg]) -> SyscallResult {
        let cap_handle = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("cap_delegate needs cap_handle".into())),
        };
        let target_pid = match args.get(1) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("cap_delegate needs target_pid".into())),
        };
        let rights = match args.get(2) {
            Some(SyscallArg::U64(v)) => Rights::from_bits_truncate(*v as u8),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("cap_delegate needs rights".into())),
        };

        let mut table = self.caps.lock();
        match table.delegate(Pid(self.pid as u32), CapId(cap_handle), Pid(target_pid as u32), rights) {
            Ok(new_cap) => SyscallResult::Success(new_cap.0),
            Err(e) => SyscallResult::Error(SyscallError::ProcessError(format!("{:?}", e))),
        }
    }

    fn sys_cap_check(&self, args: &[SyscallArg]) -> SyscallResult {
        let cap_handle = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("cap_check needs cap_handle".into())),
        };
        let required = match args.get(1) {
            Some(SyscallArg::U64(v)) => Rights::from_bits_truncate(*v as u8),
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("cap_check needs rights".into())),
        };
        let table = self.caps.lock();
        if table.check_any(Pid(self.pid as u32), cap_handle as u64, required) {
            SyscallResult::Success(1)
        } else {
            SyscallResult::Error(SyscallError::CapabilityDenied)
        }
    }

    fn sys_cap_revoke(&self, args: &[SyscallArg]) -> SyscallResult {
        let cap_handle = match args.get(0) {
            Some(SyscallArg::U64(v)) => *v,
            _ => return SyscallResult::Error(SyscallError::InvalidArgument("cap_revoke needs cap_handle".into())),
        };
        let mut table = self.caps.lock();
        table.revoke(CapId(cap_handle));
        SyscallResult::Ok
    }

    // ── Status ───────────────────────────────────────────────────────────────

    pub fn gas_used(&self) -> u64 {
        self.gas_used
    }

    pub fn gas_remaining(&self) -> u64 {
        self.gas_remaining
    }
}

// ─── Syscall-Argumente ─────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum SyscallArg {
    U64(u64),
    String(String),
    Bytes(Vec<u8>),
}

// ─── Tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::CapabilityTable;
    use crate::process::ProcessManager;
    use crate::ipc::{IpcSubsystem, ChannelId};
use crate::ats1000::Pid;
use crate::capability::{ResourceType, CapId};
    use crate::vfs::Vfs;

    fn setup() -> SyscallDispatcher {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let cap_handle = caps.lock().create(Pid(1), ResourceType::FileSystem, 1, Rights::READ | Rights::WRITE | Rights::EXEC | Rights::DELEGATE).0;

        let processes = Arc::new(Mutex::new(ProcessManager::new()));
        let ipc = Arc::new(Mutex::new(IpcSubsystem::new()));
        let vfs = Arc::new(Mutex::new(Vfs::new(caps.clone())));

        SyscallDispatcher::new(
            Context::Node,
            1_000_000, // 1M gas budget
            1,          // pid
            cap_handle, // cap handle
            caps,
            processes,
            ipc,
            vfs,
        )
    }

    fn setup_contract() -> SyscallDispatcher {
        let caps = Arc::new(Mutex::new(CapabilityTable::new()));
        let cap_handle = caps.lock().create(Pid(1), ResourceType::FileSystem, 1, Rights::READ | Rights::WRITE).0;

        let processes = Arc::new(Mutex::new(ProcessManager::new()));
        let ipc = Arc::new(Mutex::new(IpcSubsystem::new()));
        let vfs = Arc::new(Mutex::new(Vfs::new(caps.clone())));

        SyscallDispatcher::new(
            Context::Contract,
            100_000,
            1,
            cap_handle,
            caps,
            processes,
            ipc,
            vfs,
        )
    }

    // ── Context-Isolation ──────────────────────────────────────────────────

    #[test]
    fn test_context_node_allows_all() {
        assert!(Context::Node.allows(SYS_SPAWN));
        assert!(Context::Node.allows(SYS_OPEN));
        assert!(Context::Node.allows(SYS_IPC_SEND));
    }

    #[test]
    fn test_context_contract_blocks_io() {
        assert!(!Context::Contract.allows(SYS_OPEN));
        assert!(!Context::Contract.allows(SYS_READ));
        assert!(!Context::Contract.allows(SYS_WRITE));
        assert!(!Context::Contract.allows(SYS_IPC_SEND));
        assert!(!Context::Contract.allows(SYS_SPAWN));
    }

    #[test]
    fn test_context_contract_allows_alloc() {
        assert!(Context::Contract.allows(SYS_ALLOC));
        assert!(Context::Contract.allows(SYS_FREE));
    }

    #[test]
    fn test_context_test_allows_all() {
        assert!(Context::Test.allows(SYS_SPAWN));
        assert!(Context::Test.allows(SYS_OPEN));
    }

    // ── Gas-Tracking ────────────────────────────────────────────────────────

    #[test]
    fn test_gas_charged_on_dispatch() {
        let mut d = setup();
        let result = d.dispatch(SYS_MKDIR, &[SyscallArg::String("/testdir".into())]);
        assert!(result.is_ok());
        assert_eq!(d.gas_used(), gas_cost(SYS_MKDIR)); // 50
    }

    #[test]
    fn test_gas_accumulates() {
        let mut d = setup();
        d.dispatch(SYS_MKDIR, &[SyscallArg::String("/a".into())]);
        d.dispatch(SYS_CREATE_FILE, &[SyscallArg::String("/a/f.txt".into())]);
        let expected = gas_cost(SYS_MKDIR) + gas_cost(SYS_CREATE_FILE);
        assert_eq!(d.gas_used(), expected);
    }

    #[test]
    fn test_out_of_gas() {
        let mut d = setup();
        // Drain gas
        d.gas_remaining = 10;
        let result = d.dispatch(SYS_MKDIR, &[SyscallArg::String("/x".into())]);
        assert_eq!(result, SyscallResult::Error(SyscallError::OutOfGas));
    }

    #[test]
    fn test_gas_remaining_decreases() {
        let mut d = setup();
        let initial = d.gas_remaining();
        d.dispatch(SYS_MKDIR, &[SyscallArg::String("/gastest".into())]);
        assert_eq!(d.gas_remaining(), initial - gas_cost(SYS_MKDIR));
    }

    // ── VFS via Syscalls ────────────────────────────────────────────────────

    #[test]
    fn test_vfs_full_cycle_via_syscalls() {
        let mut d = setup();

        // mkdir
        let r = d.dispatch(SYS_MKDIR, &[SyscallArg::String("/home".into())]);
        assert!(r.is_ok());

        // create_file
        let r = d.dispatch(SYS_CREATE_FILE, &[SyscallArg::String("/home/notes.txt".into())]);
        assert!(r.is_ok());

        // open (Create mode = 4)
        let r = d.dispatch(SYS_OPEN, &[SyscallArg::String("/home/notes.txt".into()), SyscallArg::U64(2)]);
        assert!(matches!(r, SyscallResult::Success(_)));

        // write
        if let SyscallResult::Success(fd) = r {
            let wr = d.dispatch(SYS_WRITE, &[SyscallArg::U64(fd), SyscallArg::String("hello world".into())]);
            assert!(matches!(wr, SyscallResult::Success(11)));

            // seek to 0
            d.dispatch(SYS_SEEK, &[SyscallArg::U64(fd), SyscallArg::U64(0)]);

            // read
            let rd = d.dispatch(SYS_READ, &[SyscallArg::U64(fd), SyscallArg::U64(64)]);
            assert!(matches!(rd, SyscallResult::SuccessString(_)));
            if let SyscallResult::SuccessString(s) = rd {
                assert_eq!(s, "hello world");
            }

            // close
            d.dispatch(SYS_CLOSE, &[SyscallArg::U64(fd)]);
        }
    }

    #[test]
    fn test_listdir_via_syscall() {
        let mut d = setup();
        d.dispatch(SYS_MKDIR, &[SyscallArg::String("/dir1".into())]);
        d.dispatch(SYS_MKDIR, &[SyscallArg::String("/dir2".into())]);
        d.dispatch(SYS_CREATE_FILE, &[SyscallArg::String("/file1.txt".into())]);

        let r = d.dispatch(SYS_LISTDIR, &[SyscallArg::String("/".into())]);
        if let SyscallResult::SuccessList(entries) = r {
            assert_eq!(entries.len(), 3);
        } else {
            panic!("expected SuccessList");
        }
    }

    #[test]
    fn test_stat_via_syscall() {
        let mut d = setup();
        d.dispatch(SYS_CREATE_FILE, &[SyscallArg::String("/statme.txt".into())]);

        let r = d.dispatch(SYS_STAT, &[SyscallArg::String("/statme.txt".into())]);
        assert!(matches!(r, SyscallResult::SuccessString(_)));
    }

    #[test]
    fn test_symlink_via_syscall() {
        let mut d = setup();
        d.dispatch(SYS_CREATE_FILE, &[SyscallArg::String("/target.txt".into())]);
        d.dispatch(SYS_SYMLINK, &[SyscallArg::String("/link.txt".into()), SyscallArg::String("/target.txt".into())]);

        let r = d.dispatch(SYS_READLINK, &[SyscallArg::String("/link.txt".into())]);
        if let SyscallResult::SuccessString(target) = r {
            assert_eq!(target, "/target.txt");
        } else {
            panic!("expected SuccessString");
        }
    }

    // ── IPC via Syscalls ────────────────────────────────────────────────────

    #[test]
    fn test_ipc_create_via_syscall() {
        let mut d = setup();
        let r = d.dispatch(SYS_IPC_CREATE, &[SyscallArg::U64(32)]);
        assert!(matches!(r, SyscallResult::Success(_)));
    }

    // ── Process via Syscalls ────────────────────────────────────────────────

    #[test]
    fn test_spawn_via_syscall() {
        let mut d = setup();
        let r = d.dispatch(SYS_SPAWN, &[SyscallArg::String("child_proc".into())]);
        assert!(matches!(r, SyscallResult::Success(_)));
    }

    // ── Capability via Syscalls ─────────────────────────────────────────────

    #[test]
    fn test_cap_create_via_syscall() {
        let mut d = setup();
        let r = d.dispatch(SYS_CAP_CREATE, &[SyscallArg::U64(0x0F)]);
        assert!(matches!(r, SyscallResult::Success(_)));
    }

    #[test]
    fn test_cap_check_via_syscall() {
        let mut d = setup();
        // Check our own cap (which has all rights = 0x0F)
        let r = d.dispatch(SYS_CAP_CHECK, &[SyscallArg::U64(d.cap_handle), SyscallArg::U64(0x0F)]);
        assert!(matches!(r, SyscallResult::Success(1)));
    }

    // ── Contract-Context Restrictions ──────────────────────────────────────

    #[test]
    fn test_contract_cannot_open_files() {
        let mut d = setup_contract();
        let r = d.dispatch(SYS_OPEN, &[SyscallArg::String("/etc/passwd".into()), SyscallArg::U64(0)]);
        assert_eq!(r, SyscallResult::Error(SyscallError::PermissionDenied));
    }

    #[test]
    fn test_contract_cannot_spawn() {
        let mut d = setup_contract();
        let r = d.dispatch(SYS_SPAWN, &[SyscallArg::String("malicious".into())]);
        assert_eq!(r, SyscallResult::Error(SyscallError::PermissionDenied));
    }

    // ── Unknown Syscall ────────────────────────────────────────────────────

    #[test]
    fn test_unknown_syscall() {
        let mut d = setup();
        let r = d.dispatch(999, &[]);
        assert_eq!(r, SyscallResult::Error(SyscallError::UnknownSyscall(999)));
    }

    // ── Invalid Arguments ───────────────────────────────────────────────────

    #[test]
    fn test_spawn_missing_arg() {
        let mut d = setup();
        let r = d.dispatch(SYS_SPAWN, &[]);
        assert!(matches!(r, SyscallResult::Error(SyscallError::InvalidArgument(_))));
    }

    #[test]
    fn test_open_missing_path() {
        let mut d = setup();
        let r = d.dispatch(SYS_OPEN, &[]);
        assert!(matches!(r, SyscallResult::Error(SyscallError::InvalidArgument(_))));
    }

    // ── Gas-Cost-Table ──────────────────────────────────────────────────────

    #[test]
    fn test_gas_costs_defined() {
        assert_eq!(gas_cost(SYS_SPAWN), 500);
        assert_eq!(gas_cost(SYS_KILL), 100);
        assert_eq!(gas_cost(SYS_OPEN), 50);
        assert_eq!(gas_cost(SYS_READ), 20);
        assert_eq!(gas_cost(SYS_WRITE), 20);
        assert_eq!(gas_cost(SYS_IPC_CREATE), 100);
        assert_eq!(gas_cost(SYS_CAP_CREATE), 50);
        assert_eq!(gas_cost(SYS_CAP_CHECK), 10);
    }
}
